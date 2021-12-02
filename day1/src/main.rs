use std::io;

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let input: Vec<u32> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid input");

    let first_solution = input
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();

    println!("part 1 solution: {}", first_solution);

    let second_solution = input
        .windows(3)
        .map(|items| items.iter().sum::<u32>())
        .collect::<Vec<_>>();

    let second_solution = second_solution
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count();

    println!("part 2 solution: {}", second_solution);

    Ok(())
}
