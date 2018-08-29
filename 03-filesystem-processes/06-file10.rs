// file10.rs

use std::env;
use std::path::Path;

fn main() {
    let file = env::args().skip(1).next().unwrap_or("file10.rs".to_string());
    let path = Path::new(&file);
    // The details about a file (size, type, etc) are called its metadata.
    match path.metadata() {
        Ok(data) => {
            println!("type {:?}", data.file_type());
            println!("len {}", data.len());
            println!("perm {:?}", data.permissions());
            println!("modified {:?}", data.modified());
        },
        // There may be an error, not just 'not found' but also if we don't have permission to read this file
        Err(e) => println!("error {:?}", e)
    }
}
// type FileType(FileType { mode: 33204 })
// len 488
// perm Permissions(FilePermissions { mode: 436 })
// modified Ok(SystemTime { tv_sec: 1483866529, tv_nsec: 600495644 })

/* The length of the file (in bytes) and modified time are straightforward to interpret. Note that we may not be able to get this time. The file type has methods 'is_dir', 'is_file' and 'is_symlink'. "permissions" is complicated, Rust strives to be cross-platform, and so it's a case of the "lowest common denominator". In general, all you can query is whether the file is read-only; the "permissions" concept is extended in Unix and encodes read/write/executable for user/group/others. If you are not interested in Windows then bringing in a platform-specific trait will give us at least the permission mode bits. As usual, a trait only kicks in when it is visible. */

// Applying the program to its own executable 
use std::os::unix::fs::PermissionsExt;
...
println!("perm {:o}",data.permissions().mode());
// perm 755
(Note '{:o}' for printing out in octal)

/* Whether a file is executable on Windows is determined by its extension. The executable extensions are found in the 'PATHEXT' environment variable; '.exe','.bat', etc. 'std::fs' contains a number of useful functions for working with files, such as copying or moving files, making symbolic links and creating directories. To find the contents of a directory, 'std::fs::read_dir' provides an iterator. */

// All files with extension ".rs" and size greater than 1024 bytes:
fn dump_dir(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();
        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "rs" && data.len() > 1024 {
                    println!("{} length {}", path.display(),data.len());
                }
            }
        }
    }
    Ok(())
}
// ./enum4.rs length 2401
// ./struct7.rs length 1151
// ./sexpr.rs length 7483
// ./struct6.rs length 1359
// ./new-sexpr.rs length 7719

/* Obviously 'read_dir' might fail (usually "not found" or "no permission"), but also getting each new entry might fail, like the lines iterator over a buffered reader's contents. Also, we might not be able to get the metadata corresponding to the entry. A file might have no extension, so we have to check for that as well. Why not use an iterator over paths? On Unix this is the way the 'opendir' system call works, but on Windows you cannot iterate over a directory's contents without getting the metadata. So this is a compromise that allows cross-platform code to be as efficient as possible.

Any operating system call may fail, but note that these errors always existed, it's not that Rust is just making it impossible for you to ignore them. Languages like Java and Python throw exceptions; languages like Go and Lua return two values, where the first is the result and the second is the error: like Rust it is considered bad manners for library functions to raise errors so there is a lot of error checking and early-returns from functions. Rust uses 'Result' because it's either-or: you cannot get both a result and an error. And the question-mark operator makes handling errors much cleaner. */