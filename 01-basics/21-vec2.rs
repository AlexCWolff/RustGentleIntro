// vec2.rs
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    // The borrow operator is coercing the vector in to a slice
    let slice = &v[1..];
    println!("slice is {:?}", slice);
}

/* Stack - faster, but limited (often megabytes). 
Heap - slower, but large memory (can be gigabytes).
Garbage Collection - a form of automatic memory management.
Garbage Collector (GC) - the utility that performs garbage collection. 

These used to be actual hardware constructs, but the OS abstracts these away. 
Every program (technically every process and thread, but I think threads from 
the same process share access to their stacks) have their own stack and heap.
Messing with the stack is dangerous since the program cannot handle errors on 
the stack, the OS kills them. 

To get around these issues, many programming languages abstract away memory 
management and use a GC. The GC is convenient for general programming, but there
are some cases where it is not desireable. GC is wasteful of memory (this 
matters in small/embedded systems and some server workloads, not on standard 
desktops and servers), and it will decide automatically that a clean up will 
happen with little consideration for what your program is doing.

Rust panics are memory safe because they happen before any illegal 
access to memory. The stack is unwound as with exceptions, all allocated 
objects are dropped, and a backtrace is generated.

When a vector is modified or created, it allocates from the heap and becomes
the owner of that memory. The slice borrows the memory from the vector, and 
when the vector dies or drops, it lets the memory go. */