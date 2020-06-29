#[macro_use]
mod common;

make_test!(var: r"let x = 0; x" => "0");
make_test!(func: r"let f(x) = x + 1; f(2)" => "3");
make_test!(func2: r"let f(x)(y) = x * y + x + y; f(3)(4)" => "19");
make_test!(pipe: r"let f(x)(y) = x + y; 1 | f(2)" => "3");
make_test!(pipe_twice: r"let f(x) = x + 1; let g(x) = x * 2; 1 | f | g" => "4");
make_test!(pipe_nested: r"let f(x)(y) = x + y; 1 | (2 | f)" => "3");
make_test!(func_var: r"let f(x) = x * x; let g = f; g(3)" => "9");
make_test!(arithmetic: "1 + 2 * 2 * 3 / 4 * 5 - 6 + 7" => "17");
make_test!(func_noargs1: r"let f() = 3; f()" => "3");
make_test!(func_noargs2: r"let f() = 3; f(1)" => "3");
make_test!(func_noargs3: r"let f(x) = x; f()" => "null");

make_fail_test!(empty_script: "");
make_fail_test!(no_overload: "let x = 3; let x = 3; let f(x) = x; let f(x) = x; null");
make_test!(overload_in_block: "let x = 3; (let x = 4; x) + x" => "7");
make_test!(overload_in_func: "let x = 3; let f(y) = (let x = 4; x + y); f(1)" => "5");
make_test!(func_in_func: "let f(x) = (let g(y) = [x, y]; g); f(1)(2)" => "[1, 2]");
make_test!(func_defsite_scope: "let f = (let y = 3; let f() = y; f); f()" => "3");
make_fail_test!(func_no_callsite_scope: "let f(x) = y; let y = 3; f()");
