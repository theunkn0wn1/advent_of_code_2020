use day_1::{numbers2, numbers3, do_read};

fn main() -> anyhow::Result<()> {
    let numbers = do_read()?;

    println!("=== part one ===");
    numbers2(numbers.clone());
    println!("--- part two ---");
    numbers3(numbers);
    println!("Done.");
    Ok(())
}