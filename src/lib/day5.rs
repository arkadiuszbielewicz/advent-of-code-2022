use std::num::TryFromIntError;
use std::ops::Not;
use std::str::Lines;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::{sequence, IResult};

pub fn part_1(input: &str) -> String {
    let mut raw_stack: Vec<Vec<&str>> = parse_raw_stacks(input.lines());
    let mut stack = map_raw_stocks(&mut raw_stack).unwrap();
    let raw_commands = parse_raw_commands(input);
    let commands = map_raw_commands(&raw_commands).unwrap();
    commands.into_iter().for_each(|c| stack.crane_mover_9000(c));
    stack.read_top().into_iter().collect()
}

pub fn part_2(input: &str) -> String {
    let mut raw_stack: Vec<Vec<&str>> = parse_raw_stacks(input.lines());
    let mut stack = map_raw_stocks(&mut raw_stack).unwrap();
    let raw_commands = parse_raw_commands(input);
    let commands = map_raw_commands(&raw_commands).unwrap();
    commands.into_iter().for_each(|c| stack.crane_mover_9001(c));
    stack.read_top().into_iter().collect()
}

fn parse_raw_commands(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .skip_while(|l| l.is_empty().not())
        .filter(|l| l.is_empty().not())
        .map(|l| parse_command_line(l).unwrap().1)
        .collect::<Vec<(u32, u32, u32)>>()
}

fn parse_raw_stacks(lines: Lines) -> Vec<Vec<&str>> {
    lines
        .take_while(|l| l.trim_start().starts_with('1').not())
        .map(|l| parse_stack_line(l).unwrap().1)
        .collect()
}

fn parse_stack(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(complete::char('['), alpha1, complete::char(']')),
        tag("   "),
    ))(input)
}

fn parse_stack_line(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(complete::char(' '), parse_stack)(input)
}

fn parse_command_line(input: &str) -> IResult<&str, (u32, u32, u32)> {
    tuple((
        sequence::preceded(tag("move "), complete::u32),
        sequence::preceded(tag(" from "), complete::u32),
        sequence::preceded(tag(" to "), complete::u32),
    ))(input)
}

fn map_raw_commands(raw_commands: &[(u32, u32, u32)]) -> Result<Vec<Command>, TryFromIntError> {
    raw_commands
        .iter()
        .map(|(count, source, dest)| Command::new(*count, *source, *dest))
        .collect()
}

fn map_raw_stocks(raw_stacks: &mut Vec<Vec<&str>>) -> Option<MultiStack> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); raw_stacks.iter().next()?.len()];
    while let Some(raw_stack) = raw_stacks.pop() {
        for i in 0..raw_stack.len() {
            match raw_stack[i] {
                "   " => {}
                s => {
                    let x = s.chars().next().unwrap();
                    stacks[i].push(x);
                }
            }
        }
    }

    Some(MultiStack { stacks })
}

#[derive(Debug)]
struct Command {
    size: usize,
    source: usize,
    destination: usize,
}

impl Command {
    fn new<R>(size: R, source: R, destination: R) -> Result<Self, R::Error>
    where
        R: TryInto<usize>,
    {
        Ok(Command {
            size: size.try_into()?,
            source: source.try_into()? - 1,
            destination: destination.try_into()? - 1,
        })
    }
}

#[derive(Debug)]
struct MultiStack {
    stacks: Vec<Vec<char>>,
}

impl MultiStack {
    fn crane_mover_9000(&mut self, command: Command) {
        for _i in 0..command.size {
            let v = self.stacks[command.source].pop().unwrap();
            self.stacks[command.destination].push(v);
        }
    }

    fn crane_mover_9001(&mut self, command: Command) {
        let source_vec = &mut self.stacks[command.source];
        let drain_range = (source_vec.len().saturating_sub(command.size))..;
        let mut drain = source_vec.drain(drain_range).collect();
        let destination_vec = &mut self.stacks[command.destination];
        destination_vec.append(&mut drain);
    }

    fn read_top(&mut self) -> Vec<char> {
        self.stacks
            .iter_mut()
            .map(|s| s.pop().unwrap_or(' '))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stock_test() {
        // given
        let input = "[Z]";

        // when
        let result = parse_stack(input).unwrap();

        // then
        assert!(result.0.is_empty());
        assert_eq!(result.1, "Z");

        // given
        let input = "   ";

        // when
        let result = parse_stack(input).unwrap();

        // then
        assert!(result.0.is_empty());
        assert_eq!(result.1, "   ");
    }

    #[test]
    fn parse_stack_line_test() {
        // given
        let input = "[Z]     [a]";

        // when
        let result = parse_stack_line(input).unwrap();

        // then
        assert!(result.0.is_empty());
        assert_eq!(result.1, vec!["Z", "   ", "a"]);
    }

    #[test]
    fn parse_command_line_test() {
        // given
        let input = "move 4 from 2 to 3";

        // when
        let result = parse_command_line(input).unwrap();

        // then
        assert!(result.0.is_empty());
        assert_eq!(result.1, (4, 2, 3));
    }

    #[test]
    fn part_1_test() {
        // given
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        // when
        let result = part_1(input);

        // then
        assert_eq!(result, "CMZ")
    }

    #[test]
    fn part_2_test() {
        // given
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        // when
        let result = part_2(input);

        // then
        assert_eq!(result, "MCD")
    }
}
