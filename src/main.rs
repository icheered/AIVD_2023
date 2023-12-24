use color_eyre::Report;
use std::time::Instant;

// Importing the part1 and part2 modules
mod solver;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let words = include_str!("words.txt");

    let start = Instant::now();
    let result = solver::solve(&input, &words);

    let duration = start.elapsed();
    println!("\nResult: {:?}", result);
    println!("Time taken: {:?}", duration.as_micros());

    Ok(())
}
