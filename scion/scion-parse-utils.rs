use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use regex::Regex;

fn tokenize(s: &str, re: &Regex) -> Vec<String> {
    re.split(s).filter(|s| !s.is_empty()).map(String::from).collect()
}

fn paddTo4(x: String) -> String {
    format!("{:0>4}", x)
}

fn as_from_dotted_hex(s: &str) -> u64 {
    let re = Regex::new(r"[:]+").unwrap();
    let token: Vec<_> = tokenize(s, &re);

    let hex_str: String = token
        .iter()
        .map(|t| paddTo4(t.to_string()))
        .collect::<Vec<String>>()
        .concat();
        
        println!("{}",hex_str);

    u64::from_str_radix(&hex_str, 16).unwrap()
}

type IA_t = u64;
type AS_t = u64;
type ISD_t = u16;

const IPV6_ADDR_REGEX: &str = r"((([0-9A-Fa-f]{1,4}:){1,6}:)|(([0-9A-Fa-f]{1,4}:){7}))([0-9A-Fa-f]{1,4})";

macro_rules! AS_FROM_IA {
    ($ia:expr) => {
        (($ia << 16) >> 16)
    };
}

macro_rules! ISD_FROM_IA {
    ($ia:expr) => {
        ($ia >> 48)
    };
}

macro_rules! MAKE_BIG_IA {
    ($as:expr, $isd:expr) => {
        (($as << 16) | $isd)
    };
}


fn is_valid_ipv4(ip_address: &str) -> bool {
    ip_address.parse::<Ipv4Addr>().is_ok()
}

fn is_ipv6_address(s: &str) -> bool {
    s.parse::<Ipv6Addr>().is_ok()
}

fn reverse_bytes(input: &[u8], output: &mut [u8; 8]) {
    for i in 0..8 {
        output[i] = input[7 - i];
    }
}

fn parse_scion_impl(host_scion_addr: &str, port_str: &str) -> (IA_t, ISD_t, AS_t, String, u16) {
    let re = regex::Regex::new(
        r"^(?:(\d+)-([\d:A-Fa-f]+)),(?:\[([^\]]+)\]|([^\[\]:]+))(?::(\d+))?$",
    )
    .unwrap();
    let captures = re.captures(host_scion_addr).unwrap();

    println!("{}", captures[0].len());
    println!("{}", captures[1].len());
    println!("{}", captures[2].len());

    let isd: ISD_t = captures[1].parse().unwrap();
    let as_str = &captures[2];
    let as_num = AS_FROM_IA!( as_from_dotted_hex( &captures[2]));

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

    (MAKE_BIG_IA!(as_num, isd as u64), isd, as_num, host, port)
}

fn pad_to_4(x: &str) -> String {
    format!("{:0>4}", x)
}
