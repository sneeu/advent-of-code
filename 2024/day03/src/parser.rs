use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Mul {
    left: i64,
    right: i64,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Do,
    Dont,
    Mul(Mul),
}

impl Mul {
    pub fn apply(&self) -> i64 {
        self.left * self.right
    }
}

fn num(i: &str) -> IResult<&str, i64> {
    map_res(cc::digit1, str::parse)(i)
}

fn parse_mul(i: &str) -> IResult<&str, Operation> {
    let (i, (_, left, _, right, _)) = tuple((tag("mul("), num, tag(","), num, tag(")")))(i)?;
    Ok((i, Operation::Mul(Mul { left, right })))
}

fn parse_do(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("do()")(i)?;
    Ok((i, Operation::Do))
}

fn parse_dont(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("don't()")(i)?;
    Ok((i, Operation::Dont))
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    alt((parse_mul, parse_do, parse_dont))(i)
}

pub fn find_operations(i: &str) -> Vec<Operation> {
    let mut results = vec![];
    let mut remaining = i;

    while !remaining.is_empty() {
        match parse_operation(remaining) {
            Ok((r, op)) => {
                results.push(op);
                remaining = r;
            }
            Err(_) => remaining = &remaining[1..],
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mul() {
        assert_eq!(
            parse_mul("mul(3,4)"),
            Ok(("", Operation::Mul(Mul { left: 3, right: 4 })))
        );
        assert!(parse_mul("mul(56,78phon)").is_err());
        assert!(parse_mul("abcdefgh").is_err());
        assert!(parse_mul("mul(12, 32)").is_err());
        assert!(parse_mul("blah(f,6)").is_err());
    }
}
