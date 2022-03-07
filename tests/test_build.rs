#[test]
fn test_fail_u8_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/u8/fail/*.rs");
}


#[test]
fn test_fail_u16_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/u16/fail/*.rs");
}


#[test]
fn test_fail_u32_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/u32/fail/*.rs");
}


#[test]
fn test_fail_u64_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/u64/fail/*.rs");
}


#[test]
fn test_fail_u128_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/u128/fail/*.rs");
}


#[test]
fn test_fail_usize_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/usize/fail/*.rs");
}


#[test]
fn test_fail_i8_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/i8/fail/*.rs");
}


#[test]
fn test_fail_i16_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/i16/fail/*.rs");
}


#[test]
fn test_fail_i32_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/i32/fail/*.rs");
}


#[test]
fn test_fail_i64_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/i64/fail/*.rs");
}


#[test]
fn test_fail_i128_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/i128/fail/*.rs");
}


#[test]
fn test_fail_isize_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/isize/fail/*.rs");
}


#[test]
fn test_fail_f32_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/f32/fail/*.rs");
}


#[test]
fn test_fail_f64_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/f64/fail/*.rs");
}


#[test]
fn test_fail_string_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/String/fail/*.rs");
}


#[test]
fn test_fail_bool_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/bool/fail/*.rs");
}


#[test]
fn test_fail_unit_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/unit/fail/*.rs");
}


#[test]
fn test_fail_vec_models(){
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/models/vec/fail/*.rs");
}


#[test]
fn test_pass_u8_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/u8/pass/*.rs");
}


#[test]
fn test_pass_u16_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/u16/pass/*.rs");
}


#[test]
fn test_pass_u32_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/u32/pass/*.rs");
}


#[test]
fn test_pass_u64_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/u64/pass/*.rs");
}


#[test]
fn test_pass_u128_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/u128/pass/*.rs");
}


#[test]
fn test_pass_usize_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/usize/pass/*.rs");
}


#[test]
fn test_pass_i8_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/i8/pass/*.rs");
}


#[test]
fn test_pass_i16_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/i16/pass/*.rs");
}


#[test]
fn test_pass_i32_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/i32/pass/*.rs");
}


#[test]
fn test_pass_i64_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/i64/pass/*.rs");
}


#[test]
fn test_pass_i128_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/i128/pass/*.rs");
}


#[test]
fn test_pass_isize_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/isize/pass/*.rs");
}


#[test]
fn test_pass_f32_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/f32/pass/*.rs");
}


#[test]
fn test_pass_f64_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/f64/pass/*.rs");
}


#[test]
fn test_pass_string_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/String/pass/*.rs");
}


#[test]
fn test_pass_bool_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/bool/pass/*.rs");
}


#[test]
fn test_pass_unit_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/unit/pass/*.rs");
}

#[test]
fn test_pass_vec_models(){
    let t = trybuild::TestCases::new();
    t.pass("tests/models/vec/pass/*.rs");
}
