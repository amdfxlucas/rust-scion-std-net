use crate::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::{convert::TryInto, str::FromStr};

extern crate regex;
use self::regex::Regex;

fn tokenize(s: &str, re: &Regex) -> Vec<String> {
    re.split(s)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub fn as_from_dotted_hex(s: &str) -> u64 {
    let re = Regex::new(r"[:]+").unwrap();
    let token: Vec<_> = tokenize(s, &re);
    // println!("{:?}",token);

    let hex_str: String = token
        .iter()
        .map(|t| pad_to_4(&t.to_string()))
        .collect::<Vec<String>>()
        .concat();

    //println!("{}",hex_str);

    u64::from_str_radix(&hex_str, 16).unwrap()
}

type IA_t = u64;
type AS_t = u64;
type ISD_t = u16;

pub fn as_to_dotted_hex(as_num: AS_t) -> String {
    let hex_str = format!("{:x}", as_num);
    let mut result = String::new();
    let mut begin = true;
    let mut encountered_zeros_in_row = 0;

    for (pos, s) in hex_str.chars().enumerate() {
        if pos != 0 && pos % 4 == 0 && !begin {
            result.push(':');
            encountered_zeros_in_row = 0;
            begin = true;
        }

        if begin {
            if s == '0' {
                encountered_zeros_in_row += 1;
                if encountered_zeros_in_row == 4 {
                    result.push('0');
                    result.push(':');
                    begin = true;
                    encountered_zeros_in_row = 0;
                }
                continue;
            } else {
                result.push(s);
                encountered_zeros_in_row = 0;
                begin = false;
            }
        } else {
            result.push(s);
        }
    }

    result
}

const IPV6_ADDR_REGEX: &str =
    r"((([0-9A-Fa-f]{1,4}:){1,6}:)|(([0-9A-Fa-f]{1,4}:){7}))([0-9A-Fa-f]{1,4})";

pub fn as_from_ia(ia: u64) -> u64 {
    (ia << 16) >> 16
}

pub fn isd_from_ia(ia: u64) -> u16 {
    (ia >> 48).try_into().unwrap()
}

/*
#[macro_export]
macro_rules! AS_FROM_IA {
    ($ia:expr) => {
        (($ia << 16) >> 16)
    };
}
#[macro_export]
 macro_rules! ISD_FROM_IA {
    ($ia:expr) => {
        ($ia >> 48)
    };
}

#[macro_export]
macro_rules! MAKE_BIG_IA {
    ($as:expr, $isd:expr) => {
        (($as << 16) | $isd)
    };
} */

pub fn make_ia(isd: u16, as_: u64) -> u64 {
    ((isd as u64) << 48) | as_
}

fn is_valid_ipv4(ip_address: &str) -> bool {
    ip_address.parse::<Ipv4Addr>().is_ok()
}

fn is_ipv6_address(s: &str) -> bool {
    s.parse::<Ipv6Addr>().is_ok()
}

pub fn parse_scion_impl(host_scion_addr: &str, port_str: &str) -> (IA_t, ISD_t, AS_t, String, u16) {
    let re =
        regex::Regex::new(r"^(?:(\d+)-([\d:A-Fa-f]+)),(?:\[([^\]]+)\]|([^\[\]:]+))(?::(\d+))?$")
            .unwrap();
    let captures = re.captures(host_scion_addr).unwrap();

    /*println!("{}", captures[0].len());
    println!("{}", captures[1].len());
    println!("{}", captures[2].len());*/

    let isd: ISD_t = captures[1].parse().unwrap();
    let as_str = &captures[2];
    let as_num = as_from_ia(as_from_dotted_hex(&captures[2]));

    let host = if let Some(ipv6) = captures.get(3) {
        ipv6.as_str().to_string()
    } else {
        captures[4].to_string()
    };

    let port: u16 = if let Some(port_match) = captures.get(5) {
        port_match.as_str().parse().unwrap()
    } else {
        port_str.parse().unwrap()
    };

    (make_ia(isd, as_num), isd, as_num, host, port)
}

fn pad_to_4(x: &str) -> String {
    format!("{:0>4}", x)
}
