fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    for group in include_str!("./input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        println!("In a group");

        let mut sum = 0;

        for line in group.lines() {
            let value = line.parse::<u64>()?;
            println!(" - {value}");
            sum += value;
        }
        println!("Sum: {sum}");
    }

    Ok(())
}
