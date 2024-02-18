/// What design pattern is used in the following code?
///
/// A. Builder
///
/// B. Strategy
///
/// C. Visitor
///
/// D. Newtype
#[derive(Debug)]
struct Even(u64);

impl TryFrom<u64> for Even {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(Self(value))
        } else {
            Err("not even")
        }
    }
}

fn main() {
    let a: Result<Even, _> = 4u64.try_into();
    let b: Result<Even, _> = 5u64.try_into();

    println!("{a:?}\n{b:?}");
}
