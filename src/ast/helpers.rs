use pest::iterators::{Pair, Pairs};
use std::cell::RefCell;

use super::{Node, Rule};
use crate::error::*;

pub fn check_rule(pair: &Pair<Rule>, rule: &Rule) -> ParseResult<()> {
    if &pair.as_rule() == rule {
        Ok(())
    } else {
        unreachable!("rule {:?}, expected {:?}", pair.as_rule(), rule);
    }
}

pub fn into_single(mut pairs: Pairs<Rule>) -> ParseResult<Pair<Rule>> {
    let single = match pairs.next() {
        Some(inner) => inner,
        None => unreachable!("end of token group"),
    };
    check_end(pairs)?;
    Ok(single)
}

pub fn check_end(mut pairs: Pairs<Rule>) -> ParseResult<()> {
    match pairs.next() {
        Some(pair) => unreachable!("rule {:?}, expected end", pair.as_rule()),
        None => Ok(()),
    }
}
