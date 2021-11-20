use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::satisfy,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
enum Commands {
    SwapPositions(u32, u32),
    SwapLetter(char, char),
    RotateLeft(u32),
    RotateRight(u32),
    RotatePosition(char),
    Reverse(u32, u32),
    Move(u32, u32),
}

fn str_to_u32(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(take_while(is_digit), str_to_u32)(input)
}

fn parse_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn parse_swap_positions(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("swap position ")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(" with position ")(input)?;
    let (input, y) = parse_u32(input)?;

    Ok((input, Commands::SwapPositions(x, y)))
}

fn parse_swap_letter(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("swap letter ")(input)?;
    let (input, x) = satisfy(parse_alpha)(input)?;
    let (input, _) = tag(" with letter ")(input)?;
    let (input, y) = satisfy(parse_alpha)(input)?;

    Ok((input, Commands::SwapLetter(x, y)))
}

fn parse_rotate_left(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("rotate left ")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(" step")(input)?;
    let (input, _) = alt((tag("s"), tag("")))(input)?;

    Ok((input, Commands::RotateLeft(x)))
}

fn parse_rotate_right(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("rotate right ")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(" step")(input)?;
    let (input, _) = alt((tag("s"), tag("")))(input)?;

    Ok((input, Commands::RotateRight(x)))
}

fn parse_rotate_position(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("rotate based on position of letter ")(input)?;
    let (input, x) = satisfy(parse_alpha)(input)?;

    Ok((input, Commands::RotatePosition(x)))
}

fn parse_reverse(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("reverse positions ")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, y) = parse_u32(input)?;

    Ok((input, Commands::Reverse(x, y)))
}

fn parse_move(input: &str) -> IResult<&str, Commands> {
    let (input, _) = tag("move position ")(input)?;
    let (input, x) = parse_u32(input)?;
    let (input, _) = tag(" to position ")(input)?;
    let (input, y) = parse_u32(input)?;

    Ok((input, Commands::Move(x, y)))
}

fn parse_line(input: &str) -> IResult<&str, Commands> {
    alt((
        parse_swap_positions,
        parse_swap_letter,
        parse_rotate_left,
        parse_rotate_right,
        parse_rotate_position,
        parse_reverse,
        parse_move,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Commands>> {
    separated_list1(tag("\n"), parse_line)(input)
}

fn do_swap_positions(mut input: Vec<char>, x: u32, y: u32) -> Vec<char> {
    let x_value = input[x as usize];
    input[x as usize] = input[y as usize];
    input[y as usize] = x_value;
    println!("do_swap_positions {:?}", input);
    input
}

fn do_swap_letter(input: Vec<char>, x: char, y: char) -> Vec<char> {
    let input = input
        .iter()
        .map(|c| match c {
            c if *c == x => y,
            c if *c == y => x,
            c => *c,
        })
        .collect();
    println!("do_swap_letter {:?}", input);
    input
}

fn do_rotate_left(mut input: Vec<char>, x: u32) -> Vec<char> {
    input.rotate_left(x as usize);
    println!("do_rotate_left {:?}", input);
    input
}

fn do_rotate_right(mut input: Vec<char>, x: u32) -> Vec<char> {
    input.rotate_right(x as usize);
    println!("do_rotate_right {:?}", input);
    input
}

fn do_rotate_position(mut input: Vec<char>, x: char) -> Vec<char> {
    let x = input.iter().position(|c| *c == x).unwrap();
    let len = input.len();
    if x >= 4 {
        input.rotate_right((x + 2) % len);
    } else {
        input.rotate_right((x + 1) % len);
    }
    println!("do_rotate_position {:?}", input);
    input
}

fn do_reverse_rotate_position(mut input: Vec<char>, x: char) -> Vec<char> {
    let x = input.iter().position(|c| *c == x).unwrap();
    match x {
        0 | 1 => input.rotate_left(1),
        2 => input.rotate_right(2),
        3 => input.rotate_left(2),
        4 => input.rotate_right(1),
        5 => input.rotate_left(3),
        6 => {}
        7 => input.rotate_left(4),
        _ => unreachable!(),
    }
    println!("do_reverse_rotate_position {:?}", input);
    input
}

fn do_reverse(mut input: Vec<char>, x: u32, y: u32) -> Vec<char> {
    let up_bound = (y - x) / 2;
    for i in x..x + up_bound + 1 {
        println!(
            "x: {}, up_bound: {}, i: {}, y - i: {}",
            x,
            up_bound,
            i,
            y - i
        );
        let a = input[i as usize];
        input[i as usize] = input[(y - (i - x)) as usize];
        input[(y - (i - x)) as usize] = a;
    }
    println!("do_reverse {:?}, x: {}, y: {}", input, x, y);
    input
}

fn do_move(mut input: Vec<char>, x: u32, y: u32) -> Vec<char> {
    let a = input[x as usize];
    input.remove(x as usize);
    input.insert(y as usize, a);
    println!("do_move {:?}", input);
    input
}

fn do_cmds(mut input: Vec<char>, cmds: Vec<Commands>) -> Vec<char> {
    for cmd in cmds {
        input = match cmd {
            Commands::SwapPositions(x, y) => do_swap_positions(input, x, y),
            Commands::SwapLetter(x, y) => do_swap_letter(input, x, y),
            Commands::RotateLeft(x) => do_rotate_left(input, x),
            Commands::RotateRight(x) => do_rotate_right(input, x),
            Commands::RotatePosition(x) => do_rotate_position(input, x),
            Commands::Reverse(x, y) => do_reverse(input, x, y),
            Commands::Move(x, y) => do_move(input, x, y),
        }
    }
    input
}

fn do_reverse_cmds(mut input: Vec<char>, cmds: Vec<Commands>) -> Vec<char> {
    for cmd in cmds {
        input = match cmd {
            Commands::SwapPositions(x, y) => do_swap_positions(input, y, x),
            Commands::SwapLetter(x, y) => do_swap_letter(input, y, x),
            Commands::RotateLeft(x) => do_rotate_right(input, x),
            Commands::RotateRight(x) => do_rotate_left(input, x),
            Commands::RotatePosition(x) => do_reverse_rotate_position(input, x),
            Commands::Reverse(x, y) => do_reverse(input, x, y),
            Commands::Move(x, y) => do_move(input, y, x),
        }
    }
    input
}

fn main() {
    let (_, cmds) = parse_input(include_str!("input.txt")).unwrap_or(("", vec![]));
    // let result = do_cmds(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'], cmds);
    let result = do_reverse_cmds(
        "fbgdceah".chars().into_iter().collect(),
        cmds.into_iter().rev().collect(),
    );
    println!("{:?}", result);
    println!("{}", result.iter().collect::<String>());
}

#[cfg(test)]
mod test {
    use crate::Commands::{Move, Reverse};
    use crate::{
        do_reverse, parse_input, parse_line, parse_move, parse_reverse, parse_rotate_left,
        parse_rotate_position, parse_rotate_right, parse_swap_letter, parse_swap_positions,
        Commands,
    };

    #[test]
    fn test_swap_positions_parser() {
        let result = parse_swap_positions("swap position 1 with position 2");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::SwapPositions(1u32, 2u32)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_swap_letter_parser() {
        let result = parse_swap_letter("swap letter a with letter B");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::SwapLetter('a', 'B')),
            _ => panic!(),
        }
    }

    #[test]
    fn test_rotate_left_parser() {
        let result = parse_rotate_left("rotate left 123 steps");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotateLeft(123)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_rotate_right_parser() {
        let result = parse_rotate_right("rotate right 123 steps");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotateRight(123)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_rotate_position_parser() {
        let result = parse_rotate_position("rotate based on position of letter a");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotatePosition('a')),
            _ => panic!(),
        }
    }

    #[test]
    fn test_reverse_position_parser() {
        let result = parse_reverse("reverse positions 23 through 33");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::Reverse(23, 33)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_move_position_parser() {
        let result = parse_move("move position 11 to position 33");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::Move(11, 33)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_swap_positions_parser() {
        let result = parse_line("swap position 1 with position 2");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::SwapPositions(1u32, 2u32)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_swap_letter_parser() {
        let result = parse_line("swap letter a with letter B");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::SwapLetter('a', 'B')),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_rotate_left_parser() {
        let result = parse_line("rotate left 123 steps");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotateLeft(123)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_rotate_right_parser() {
        let result = parse_line("rotate right 123 steps");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotateRight(123)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_rotate_position_parser() {
        let result = parse_line("rotate based on position of letter a");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::RotatePosition('a')),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_reverse_position_parser() {
        let result = parse_line("reverse positions 23 through 33");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::Reverse(23, 33)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_line_move_position_parser() {
        let result = parse_line("move position 11 to position 33");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, Commands::Move(11, 33)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_multiline() {
        let result =
            parse_input("move position 11 to position 33\nreverse positions 23 through 33");
        match result {
            Ok((_, cmd)) => assert_eq!(cmd, vec![Move(11, 33), Reverse(23, 33)]),
            _ => panic!(),
        }
    }

    #[test]
    fn test_do_reverse() {
        let result = do_reverse("test string".chars().collect(), 5, 10);
        assert_eq!(result, "test gnirts".chars().collect::<Vec<_>>());
    }
}
