use jjay::*;

#[allow(unused)]
macro_rules! make_test {
    ($name:ident, $file:literal, $expected:ident) => {
        ::paste::item! {
            #[test]
            #[allow(non_snake_case)]
            fn [< json_ $name >]() {
                $crate::common::run_test(include_bytes!(concat!("json_data/", $file)), $crate::common::TestResult::$expected);
            }
        }
    };
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
        match run_test_internal(data).and_then(|value| value.to_json()) {
            Ok(value) => {
                let actual: serde_json::Value = serde_json::from_slice(data).unwrap();
                assert_eq!(actual, value);
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
