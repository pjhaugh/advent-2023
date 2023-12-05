use anyhow::Result;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::preceded;

mod utils;
use utils::utils::ws;

#[derive(Debug)]
struct Game {
    _id: u32,
    winning_nums: Vec<u32>,
    my_nums: Vec<u32>,
}


fn parse_game(input: &'static str) -> IResult<&str, Game> {
    let (input, id) = map_res(
        preceded(preceded(tag("Card "), space0), digit1),
        str::parse::<u32>,
    )(input)?;
    let (input, winning_nums) = preceded(
        ws(tag(":")),
        separated_list1(space1, map_res(digit1, str::parse::<u32>)),
    )(input)?;

    let (input, my_nums) = preceded(
        ws(tag("|")),
        separated_list1(space1, map_res(digit1, str::parse::<u32>)),
    )(input)?;
    Ok((
        input,
        Game {
            _id: id,
            winning_nums,
            my_nums,
        },
    ))
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-04-2023.txt");
    // let input = include_str!("../../inputs/test-04.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn part_one(input: &'static str) -> Result<u32> {
    let mut res: u32 = 0;
    let games: Vec<Game> = input
        .lines()
        .map(parse_game)
        .map(|r| r.unwrap().1)
        .collect();
    for game in games {
        let wins = game
            .my_nums
            .iter()
            .filter(|n| game.winning_nums.contains(n))
            .count();
        if wins > 0 {
            res += 2_u32.pow((wins - 1) as u32);
        }
    }
    Ok(res)
}

fn part_two(input: &'static str) -> Result<usize> {
    let games: Vec<Game> = input
        .lines()
        .map(parse_game)
        .map(|r| r.unwrap().1)
        .collect();
    let mut counts = vec![1; games.len()];
    for (index, game) in games.iter().enumerate() {
        let wins = game
            .my_nums
            .iter()
            .filter(|n| game.winning_nums.contains(n))
            .count();
        let num = counts[index];
        for i in index + 1..=index + wins {
            counts[i] += num;
        }
    }
    Ok(counts.iter().sum())
}
