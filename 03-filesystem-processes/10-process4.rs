// process4.rs (aka process5.rs)

/* Up to now, our program simply waits for the child process to finish. If you use the spawn method then we return immediately and must explicitly wait for it to finish, or go off and do something else in the meantime. */ 

use std::process::{Command,Stdio};

fn main() {
    let mut child = Command::new("rustc")
        // Suppress standard out and standard error
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("no rustc?");

    let res = child.wait();
    println!("res {:?}", res);
}

/* By default, the child 'inherits' the standard input and output of the parent. In this case, we redirect the child's output handles into 'nowhere'. It's possible to do these things using the shell ('sh' or 'cmd') in Rust, but this way you get full programmatic control of process creation.

For example, if we just had '.stdout(Stdio::piped())' then the child's standard output is redirected to a pipe. Then 'child.stdout' is something you can use to directly read the output (i.e. implements 'Read'). Likewise, you can use the '.stdout(Stdio::piped())' method so you can write to 'child.stdin'. If we used 'wait_with_output' instead of 'wait' then it returns a 'Result<Output>' and the child's output is captured into the 'stdout' field of that 'Output' as a 'Vec<u8>' just as before. The 'Child' struct also gives you an explicit 'kill' method. */