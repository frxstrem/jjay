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

pub fn log_call<T: Node, F, R>(func_name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    const LOG: bool = true;

    thread_local! {
        static INDENT: RefCell<usize> = RefCell::new(0);
    }

    fn indent() {
        let n = INDENT.with(|n| *n.borrow());
        for _ in 0..n {
            eprint!("  ");
        }
    }

    if !LOG {
        return f();
    }

    indent();
    eprintln!(
        "<\u{1b}[33m{}\u{1b}[0m>::{} \u{1b}[32m{{\u{1b}[0m",
        std::any::type_name::<T>(),
        func_name
    );

    INDENT.with(|n| *n.borrow_mut() += 1);

    let result = f();

    INDENT.with(|n| *n.borrow_mut() -= 1);

    indent();
    eprintln!("\u{1b}[32m}}\u{1b}[0m");
    result
}
