# snet - scion-std-net

This crate contains the std::net::SocketAddr struct extended to include a SCION variant in addition to IP- V4/V6.
Compared to std::net it comes with two new classes: SocketAddrScion and ScionAddr - that represent the L4 and L3 address of a host,
which is reachable via the SCION-Next Generation Internet Architecture.
The snet IP address structs can be used interchangeably with the std::net ones, as they implement the respective From/Into traits.