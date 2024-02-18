/// Why doesn't this program run computations in parallel?
///
/// A. Iterator operations are lazy
///
/// B. Only one thread can run at all times
fn main() {
    let values = [1, 2, 3]
        .into_iter()
        .map(|value| {
            std::thread::spawn(move || {
                println!("COMPUTING {value} * 2...");
                std::thread::sleep(std::time::Duration::from_secs(1));

                value * 2
            })
        })
        .map(|handle| handle.join())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("{values:?}");
}
