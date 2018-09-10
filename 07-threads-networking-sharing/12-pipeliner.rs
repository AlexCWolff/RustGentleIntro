// pipeliner.rs

/* It's better to find higher-level ways of doing threading, rather than managing the synchronization yourself. An example is when you need to do things in parallel and collect the results. One very cool crate is pipeliner which has a very straightforward API. Here's the 'Hello, World!', an iterator feeds us inputs and we execute up to 'n' of the operations on the values in parallel. */

extern crate pipeliner;
use pipeliner::Pipeline;

fn main() {
    for result in (0..10).with_threads(4).map(|x| x + 1) {
        println!("result: {}", result);
    }
}
// result: 1
// result: 2
// result: 5
// result: 3
// result: 6
// result: 7
// result: 8
// result: 9
// result: 10
// result: 4

/*  Doing network operations in parallel is very useful, because they can take time, and you don't want to wait for them all to finish before starting to do work. We reuse the shell function defined in section 4 to call ping on a range of IP4 addresses. */

extern crate pipeliner;
use pipeliner::Pipeline;

use std::process::Command;

fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("ping -c1 192.168.0.{}",n)).collect();
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| shell(&s)) {
        if result.1 {
            println!("got: {}", result.0);
        }
    }

}
/* 
got: PING 192.168.0.1 (192.168.0.1) 56(84) bytes of data.
64 bytes from 192.168.0.1: icmp_seq=1 ttl=64 time=43.2 ms

--- 192.168.0.1 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 43.284/43.284/43.284/0.000 ms
got: PING 192.168.0.18 (192.168.0.18) 56(84) bytes of data.
64 bytes from 192.168.0.18: icmp_seq=1 ttl=64 time=0.029 ms

--- 192.168.0.18 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 0.029/0.029/0.029/0.000 ms
got: PING 192.168.0.3 (192.168.0.3) 56(84) bytes of data.
64 bytes from 192.168.0.3: icmp_seq=1 ttl=64 time=110 ms

--- 192.168.0.3 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 110.008/110.008/110.008/0.000 ms
got: PING 192.168.0.5 (192.168.0.5) 56(84) bytes of data.
64 bytes from 192.168.0.5: icmp_seq=1 ttl=64 time=207 ms
...

The active addresses come through pretty fast within the first half-second, and we then wait for the negative results to come in. You can now proceed to scrape things like ping times from the output, although this would only work on Linux. ping is universal, but the exact output format is different for each platform. To do better we need to use the cross-platform Rust networking API, and so let's move onto 'Networking'. */