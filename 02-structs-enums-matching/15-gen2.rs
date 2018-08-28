// gen2.rs

// We need to constrain the type even further.
fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}

/* That finally works. LListening to the compiler will often get you closer to compiling cleanly. This is a bit simpler in C++:

template <typename T>
T sqr(x: T) {
    return x * x;
}

However, C++ template errors are famously bad because all the compiler knows is that some operator or method is not defined. The C++ committee knows this is a problem and so they are working toward concepts, which are like trait-constrained type parameters in Rust. Rust generic functions may overwhelming but being explicit means you know exactly what kind of values you can safely feed it just by looking at the definition.

These functions are called monomorphic, in constrast to polymorphic, because the body of the function is compiled separately for each unique type. With polymorphic functions the same machine code works with each matching type, dynamically dispatching the correct method.

Monomorphic functions produce faster code specialized for the particular type, and can often be inlined. When sqr(x) is seen, it's effectively replaced with x*x. The downside is that large generic functions produce a lot of code for each type used, which can result in code bloat. */