/// Why doesn't this program compile?
///
/// A. We need to overload the == operator for A
///
/// B. A needs to implement the Eq trait
///
/// C. We cannot compare structs in Rust
///
/// D. A needs to implement the PartialEq trait
struct A;

fn main() {
    println!("A == A ? {}", A == A);
}
