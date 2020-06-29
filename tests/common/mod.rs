#![allow(unused, dead_code)]

use jjay::*;

macro_rules! make_test {
    ($(#[$meta:meta])* $name:ident: $script:literal => $expected:literal) => {
        #[test]
        $(#[$meta])*
        fn $name() {
            $crate::common::run_script_test($script, $expected)
        }
    }
}

macro_rules! make_fail_test {
    ($(#[$meta:meta])* $name:ident: $script:literal) => {
        #[test]
        $(#[$meta])*
        fn $name() {
            $crate::common::run_script_fail_test($script)
        }
    }
}

macro_rules! make_json_test {
    ($(#[$meta:meta])* $name:ident, $file:literal, $expected:ident) => {
        ::paste::item! {
            make_json_test!(@meta $expected
                #[test]
                #[allow(non_snake_case, non_upper_case_globals, dead_code)]
                $(#[$meta])*
                fn [< json_ $name >]() {
                    pub const y: Option<bool> = Some(true);
                    pub const n: Option<bool> = Some(false);
                    pub const i: Option<bool> = None;

                    $crate::common::run_json_test(include_bytes!(concat!("json_data/", $file)), $expected);
                }
            );
        }
    };

    (@meta y $($tt:tt)*) => { $($tt)* };
    (@meta n $($tt:tt)*) => { #[cfg_attr(not(feature = "n-tests"), ignore)] $($tt)* };
    (@meta i $($tt:tt)*) => { #[cfg_attr(not(feature = "i-tests"), ignore)] $($tt)* };
}

#[allow(unused)]

pub fn run_json_test(data: &[u8], expected: Option<bool>) {
    let source = String::from_utf8(data.to_vec()).unwrap();
    if expected.unwrap_or(true) {
        match run_test(&source) {
            Ok(value) => {
                let expected: serde_json::Value = serde_json::from_slice(data).unwrap();
                if !compare_json_value(&value, &expected) {
                    eprintln!("expected:");
                    eprintln!("{:#?}", expected);
                    eprintln!("got:");
                    eprintln!("{:#?}", value);
                    panic!("assertion failed");
                }
            }

            Err(err) => {
                eprintln!("{}", err);
                panic!("test failure");
            }
        }
    } else {
        match run_test(&source) {
            Ok(_) => panic!("expected failure"),
            Err(_) => (/* OK */),
        }
    }
}

pub fn run_script_test(source: &str, expected: &str) {
    match run_test(source) {
        Ok(value) => {
            let expected: serde_json::Value = serde_json::from_str(expected).unwrap();
            if !compare_json_value(&value, &expected) {
                eprintln!("expected:");
                eprintln!("{:#?}", expected);
                eprintln!("got:");
                eprintln!("{:#?}", value);
                panic!("assertion failed");
            }
        }

        Err(err) => {
            eprintln!("{}", err);
            panic!("test failure");
        }
    }
}

pub fn run_script_fail_test(source: &str) {
    match run_test(&source) {
        Ok(_) => panic!("expected failure"),
        Err(_) => (/* OK */),
    }
}

fn run_test(source: &str) -> ScriptResult<serde_json::Value> {
    jjay::run_script(source).and_then(|value| value.to_json())
}

fn compare_json_value(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    use serde_json::Value;
    use std::collections::HashSet;

    match (actual, expected) {
        (Value::Object(actual_map), Value::Object(expected_map)) => {
            let actual_keys: HashSet<&String> = actual_map.keys().collect();
            let expected_keys: HashSet<&String> = expected_map.keys().collect();
            if actual_keys != expected_keys {
                return false;
            }

            for key in actual_keys {
                if !compare_json_value(&actual_map[key], &expected_map[key]) {
                    return false;
                }
            }

            return true;
        }

        (Value::Array(actual_items), Value::Array(expected_items)) => {
            actual_items.len() == expected_items.len()
                && actual_items
                    .iter()
                    .zip(expected_items.iter())
                    .all(|(actual, expected)| compare_json_value(actual, expected))
        }

        (Value::String(actual_value), Value::String(expected_value)) => {
            actual_value == expected_value
        }

        (Value::Number(actual_value), Value::Number(expected_value)) => {
            let actual_value = match expected_value.as_f64() {
                Some(value) => value,
                None => return false,
            };
            let expected_value = match expected_value.as_f64() {
                Some(value) => value,
                None => return false,
            };

            return float_cmp::ApproxEq::approx_eq(
                actual_value,
                expected_value,
                float_cmp::F64Margin::zero(),
            );
        }

        (Value::Bool(actual_value), Value::Bool(expected_value)) => actual_value == expected_value,

        (Value::Null, Value::Null) => true,

        _ => false,
    }
}
