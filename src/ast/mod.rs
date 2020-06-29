#[macro_use]
mod node_macro;

mod helpers;
mod nodes;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use crate::error::*;

pub use self::nodes::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JJayParser;

pub fn parse_str(s: &str) -> ParseResult<Script> {
    let mut pairs = JJayParser::parse(Rule::script, s)?;
    let node = Script::parse_many(&mut pairs)?;
    helpers::check_end(pairs)?;
    Ok(node)
}

pub trait Node: Sized + Clone + std::fmt::Debug {
    fn can_parse(rule: &Rule) -> bool;
    fn parse(pair: Pair<Rule>) -> ParseResult<Self>;

    fn parse_many(pairs: &mut Pairs<Rule>) -> ParseResult<Self> {
        helpers::log_call::<Self, _, _>("parse_many", move || match pairs.next() {
            Some(pair) => Self::parse(pair),
            None => unreachable!("end of tokens"),
        })
    }
}

impl Node for String {
    fn can_parse(_: &Rule) -> bool {
        true
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<String> {
        Ok(pair.as_str().to_string())
    }
}

impl<T: Node> Node for Box<T> {
    fn can_parse(_: &Rule) -> bool {
        true
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<Box<T>> {
        T::parse(pair).map(Box::new)
    }

    fn parse_many(pairs: &mut Pairs<Rule>) -> ParseResult<Box<T>> {
        T::parse_many(pairs).map(Box::new)
    }
}

impl<T: Node> Node for Option<T> {
    fn can_parse(_: &Rule) -> bool {
        true
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<Option<T>> {
        if T::can_parse(&pair.as_rule()) {
            T::parse(pair).map(Some)
        } else {
            Ok(None)
        }
    }

    fn parse_many(pairs: &mut Pairs<Rule>) -> ParseResult<Option<T>> {
        if let Some(_) = pairs
            .peek()
            .as_ref()
            .map(Pair::as_rule)
            .filter(T::can_parse)
        {
            T::parse_many(pairs).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<T: Node> Node for Vec<T> {
    fn can_parse(_: &Rule) -> bool {
        true
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<Vec<T>> {
        if T::can_parse(&pair.as_rule()) {
            T::parse(pair).map(|item| vec![item])
        } else {
            Ok(vec![])
        }
    }

    fn parse_many(pairs: &mut Pairs<Rule>) -> ParseResult<Vec<T>> {
        let mut items = Vec::new();
        while let Some(_) = pairs
            .peek()
            .as_ref()
            .map(Pair::as_rule)
            .filter(T::can_parse)
        {
            let pair = pairs.next().unwrap();
            items.push(T::parse(pair)?)
        }
        Ok(items)
    }
}
