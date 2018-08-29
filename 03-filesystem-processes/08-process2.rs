// process2.rs

// By default, the program's standard output and standard error streams go to the terminal.
// Often we want to capture that output, so there's the output method.
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("no rustc?");

    if output.status.success() {
        println!("ok!");
    }
    println!("len stdout {} stderr {}", output.stdout.len(), output.stderr.len());
}
// ok!
// len stdout 44 stderr 0

/* As with status our program blocks until the child process is finished, and we get back three things; the status, the contents of stdout, and the contents of stderr. The captured output is simply 'Vec<u8>', just bytes. Recall that we have no guarantee that data we receive from the operating system is a properly encoded UTF-8 string. In fact, we have no guarantee that it even is a string; programs may return arbitrary binary data.

If we are sure the output is UTF-8, then 'String::from_utf8' will convert those vectors or bytes. It returns a 'Result' because this conversion may not succeed. A more sloppy function is 'String::from_utf8_lossy' which will make a good attempt at conversion and insert the invalid Unicode mark � where it failed. */