use jjay::*;

#[allow(unused)]
macro_rules! make_test {
    ($(#[$meta:meta])* $name:ident, $file:literal, $expected:ident) => {
        ::paste::item! {
            make_test!(@meta $expected
                #[test]
                #[allow(non_snake_case)]
                $(#[$meta])*
                fn [< json_ $name >]() {
                    $crate::common::run_test(include_bytes!(concat!("json_data/", $file)), $crate::common::TestResult::$expected);
                }
            );
        }
    };

    (@meta y $($tt:tt)*) => { $($tt)* };
    (@meta n $($tt:tt)*) => { #[cfg_attr(not(feature = "n-tests"), ignore)] $($tt)* };
    (@meta i $($tt:tt)*) => { #[cfg_attr(not(feature = "i-tests"), ignore)] $($tt)* };
}

#[derive(Copy, Clone, Debug)]
pub enum TestResult {
    Success,
    Failure,
    Indeterminate,
}

#[allow(non_upper_case_globals, dead_code)]
impl TestResult {
    pub const y: TestResult = TestResult::Success;
    pub const n: TestResult = TestResult::Failure;
    pub const i: TestResult = TestResult::Indeterminate;
}

impl TestResult {
    fn success_or_indeterminate(self) -> bool {
        matches!(self, TestResult::Success | TestResult::Indeterminate)
    }
}

pub fn run_test(data: &[u8], expected: TestResult) {
    if expected.success_or_indeterminate() {
        match run_test_internal(data) {
            Ok(value) => {
                let expected: serde_json::Value = serde_json::from_slice(data).unwrap();
                if !compare(&value, &expected) {
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
        match run_test_internal(data) {
            Ok(_) => panic!("expected failure"),
            Err(_) => (/* OK */),
        }
    }
}

fn run_test_internal(data: &[u8]) -> ScriptResult<Value> {
    let source = String::from_utf8(data.to_vec()).unwrap();
    let value = jjay::run_script(source)?;
    Ok(value)
}

fn compare(actual: &Value, expected: &serde_json::Value) -> bool {
    use std::collections::HashSet;

    match (actual, expected) {
        (Value::Object(actual_map), serde_json::Value::Object(expected_map)) => {
            let actual_keys: HashSet<&String> = actual_map.keys().collect();
            let expected_keys: HashSet<&String> = expected_map.keys().collect();
            if actual_keys != expected_keys {
                return false;
            }

            for key in actual_keys {
                if !compare(&actual_map[key], &expected_map[key]) {
                    return false;
                }
            }

            return true;
        }

        (Value::Array(actual_items), serde_json::Value::Array(expected_items)) => {
            actual_items.len() == expected_items.len()
                && actual_items
                    .iter()
                    .zip(expected_items.iter())
                    .all(|(actual, expected)| compare(actual, expected))
        }

        (Value::String(actual_value), serde_json::Value::String(expected_value)) => {
            actual_value == expected_value
        }

        (Value::Number(actual_value), serde_json::Value::Number(expected_value)) => {
            let expected_value = match expected_value.as_f64() {
                Some(value) => value,
                None => return false,
            };

            return float_cmp::ApproxEq::approx_eq(
                *actual_value,
                expected_value,
                float_cmp::F64Margin::zero().ulps(8),
            );
        }

        (Value::Boolean(actual_value), serde_json::Value::Bool(expected_value)) => {
            actual_value == expected_value
        }

        (Value::Null, serde_json::Value::Null) => true,

        _ => false,
    }
}
