use std::{error::Error, fs};

const INPUT: &str = "day-one-input.txt";

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let input = fs::read(INPUT)?;
    let mut input_str = String::new();
    unsafe {
        input_str = String::from_utf8_unchecked(input);
    }

    let split_str: u32 = input_str
        .split("\n\n")
        .into_iter()
        .filter_map(|value| {
            value
                .split("\n")
                .filter_map(|x| x.parse::<u32>().ok())
                .reduce(|acc, x| acc + x)
        })
        .reduce(|acc, x| acc.max(x))
        .ok_or_else(|| String::from("There is no value"))?;
    println!("{split_str}");
    Ok(())
}
