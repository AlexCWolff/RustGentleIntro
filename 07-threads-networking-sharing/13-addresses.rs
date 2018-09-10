// addresses.rs

// If you just want availability and not detailed ping statistics, the 'std::net::ToSocketAddrs' trait will do any DNS resolution for you.
use std::net::*;

fn main() {
    for res in "google.com:80".to_socket_addrs().expect("bad") {
        println!("got {:?}", res);
    }
}
// got V4(216.58.223.14:80)
// got V6([2c0f:fb50:4002:803::200e]:80)

/* It's an iterator because there is often more than one interface associated with a domain; there are both IPV4 and IPV6 interfaces to Google. So, let's naively use this method to rewrite the pipeliner example. Most networking protocols use both an address and a port. */

extern crate pipeliner;
use pipeliner::Pipeline;

use std::net::*;

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0",n)).collect();
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| s.to_socket_addrs()) {
        println!("got: {:?}", result);
    }
}
// got: Ok(IntoIter([V4(192.168.0.1:0)]))
// got: Ok(IntoIter([V4(192.168.0.39:0)]))
// got: Ok(IntoIter([V4(192.168.0.2:0)]))
// got: Ok(IntoIter([V4(192.168.0.3:0)]))
// got: Ok(IntoIter([V4(192.168.0.5:0)]))
// ....

/* This is much faster than the ping example because it's just checking that the IP address is valid; if we fed it a list of actual domain names the DNS lookup could take some time, hence the importance of parallelism. Surprisingly, it sort-of just works. The fact that everything in the standard library implements 'Debug' is great for exploration as well as debugging. The iterator is returning 'Result' and in that 'Result' is an 'IntoIter' into a 'SocketAddr' which is an 'enum' with either a ipv4 or a ipv6 address. Why 'IntoIter'? Because a socket may have multiple addresses, e.g. both ipv4 and ipv6. */

for result in addresses.with_threads(n)
    .map(|s| s.to_socket_addrs().unwrap().next().unwrap())
{
    println!("got: {:?}", result);
}
// got: V4(192.168.0.1:0)
// got: V4(192.168.0.39:0)
// got: V4(192.168.0.3:0)

/* This also works, at least for our simple example. The first 'unwrap' gets rid of the 'Result', and then we explicitly pull the first value out of the iterator. The 'Result' will be bad typically when we give a nonsense address, like an address name without a port. */