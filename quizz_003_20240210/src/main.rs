/// What does this program do?
///
/// A. Print "5"
///
/// B. Print "(10, 7)"
///
/// C. Print "17"
///
/// D. Print "2"
fn main() {
    let result = Some(5)
        .zip(None)
        .or_else(|| Some((10, 7)))
        .map(|(a, b)| a + b)
        .unwrap_or(2);

    println!("{result}");
}
