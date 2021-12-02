use std::io;

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut position = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: u32 = amount.parse().unwrap();
        match direction {
            "forward" => position += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => unreachable!(),
        }
    }

    println!("part 1 solution: {}", position * depth);

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        let amount: u32 = amount.parse().unwrap();
        match direction {
            "forward" => {
                position += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => unreachable!(),
        }
    }

    println!("part 2 solution: {}", position * depth);

    Ok(())
}
