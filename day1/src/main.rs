fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    for line in include_str!("./input.txt").lines() {
        println!("Line: {line}");
    }

    Ok(())
}
