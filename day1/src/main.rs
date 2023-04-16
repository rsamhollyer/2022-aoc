fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut max = 0;

    for group in include_str!("./problem.txt")
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
        max = max.max(sum);
        println!("Sum: {sum}");
    }

    println!("Max: {max}");
    Ok(())
}
