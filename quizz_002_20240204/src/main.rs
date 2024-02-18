/// What does this program do? Why?
///
/// A. Print "[1, 2, 3] [2, 4, 6]"
///
/// B. Print "[] [2, 4, 6]"
///
/// C. Fail to compile
///
/// D. Print "[2, 4, 6] [2, 4, 6]"
fn main() {
    let a = vec![1, 2, 3];
    let b: Vec<_> = a.into_iter().map(|a| a * 2).collect();

    println!("{a:?} {b:?}");
}
