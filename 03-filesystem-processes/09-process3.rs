// process3.rs

// Runs a program using the shell
fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    // The name of the shell is different on Windows, but otherwise the same
    let shell = if cfg!(windows) {"cmd.exe"} else {"/bin/sh"};
    let flag = if cfg!(windows) {"/c"} else {"-c"};
    let output = Command::new(shell)
        .arg(flag)
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        // Trim any whitespace from the right so that if you said 'shell("which rustc")' you will get the path without any extra linefeed
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}


fn shell_success(cmd: &str) -> Option<String> {
    let (output,success) = shell(cmd);
    if success {Some(output)} else {None}
}


/* You can control the execution of a program launched by Process by specifying the directory it will run in using the current_dir method and the environment variables it sees using env. */