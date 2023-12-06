use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-07-2023.txt");
    // let input = include_str!("../../inputs/test-07.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}


fn part_one(input: &'static str) -> Result<u64> {
    let mut res = 0;
    Ok(res)
}


fn part_two(input: &'static str) -> Result<u64> {
    let mut res = 0;
    Ok(res)
}
