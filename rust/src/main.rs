use std::io::Result;
mod day1;
mod day2;

fn main() -> Result<()> {
    println!("Day 1");
    let day1 = day1::main()?;
    println!("Calibration value: {}", day1);

    println!("\nDay 2");
    day2::main()?;

    Ok(())
}


