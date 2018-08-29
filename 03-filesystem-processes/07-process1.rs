// process1.rs

// Programs need to be able to run programs or processes. Your program can spawn as many child processes it likes, and they have a special relationship with their parent.

// To run a program, use the Command struct.
use std::process::Command;

fn main() {
    // Build up arguments to pass to the program
    // 'new' receives the name of the program (it will be looked up on PATH if not an absolute filename)
    let status = Command::new("rustc")
        // 'arg' adds a new argument
        .arg("-V")
        // 'status' causes it to be run
        .status()
        .expect("no rustc?");

    println!("cool {} code {}", status.success(), status.code().unwrap());
}
// rustc 1.15.0-nightly (8f02c429a 2016-12-15)
// cool true code 0

/* This returns a 'Result', which is 'Ok' if the program actually runs, containing an 'ExitStatus'. In this case the program succeeded and returned an exit code 0. The 'unwrap' is because we can't always get the code if the program was killed by a signal. If we change the '-V' to '-v' then 'rustc' fails.

error: no input filename given

cool false code 101

So there are three possibilities:
program didn't exist, was bad, or we were not allowed to run it
program ran, but was not successful - non-zero exit code
program ran, but was not successful - zero exit code. */