use std::io::Result;
mod day1;

fn main() -> Result<()> {
    let day1 = day1::main()?;
    println!("Calibration value: {}", day1);

    Ok(())
}


