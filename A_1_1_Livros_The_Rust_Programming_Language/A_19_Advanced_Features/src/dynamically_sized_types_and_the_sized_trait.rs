// Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. 
// This trait is automatically implemented for everything whose size is known at compile time. 
// In addition, Rust implicitly adds a bound on Sized to every generic function. 
// That is, a generic function definition like
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

// is actually treated as though we had written this:
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

// By default, generic functions will work only on types that have a known size at compile time. 
// However, you can use the following special syntax to relax this restriction:
// A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types 
// must have a known size at compile time. The ?Trait syntax with this meaning is only available for Sized, not any other traits.
// Also note that we switched the type of the t parameter from T to &T. 
// Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

pub fn sample(){
    
}