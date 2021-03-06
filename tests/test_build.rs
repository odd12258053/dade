use paste::paste;

macro_rules! fail_pattern {
    ($ty: ident, $base_dir:literal, $suffix:ident) => {
        paste! {
            #[test]
            fn [<test_fail_ $ty _in_ $suffix>]() {
                let t = trybuild::TestCases::new();
                t.compile_fail(format!("{}/{}/fail/*.rs", $base_dir, stringify!($ty)));
            }
        }
    };
    ( ($( $ty:ident ),*), $base_dir:literal, $suffix:ident ) => { $( fail_pattern!($ty, $base_dir, $suffix); )* };
}

macro_rules! pass_pattern {
    ($ty: ident, $base_dir:literal, $suffix:ident) => {
        paste! {
            #[test]
            fn [<test_pass_ $ty _in_ $suffix>]() {
                let t = trybuild::TestCases::new();
                t.pass(format!("{}/{}/pass/*.rs", $base_dir, stringify!($ty)));
            }
        }
    };
    ( ($( $ty:ident ),*), $base_dir:literal, $suffix:ident ) => { $( pass_pattern!($ty, $base_dir, $suffix); )* };
}

fail_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/struct/named",
    struct_named
);

fail_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/struct/unnamed",
    struct_unnamed
);

fail_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/enum/unnamed",
    enum_unnamed
);

fail_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/enum/named",
    enum_named
);

fail_pattern!(unit, "./tests/models/enum", enum_unit);

pass_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/struct/named",
    struct_named
);

pass_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/struct/unnamed",
    struct_unnamed
);

pass_pattern!(unit, "./tests/models/struct/", struct_unit);

pass_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/enum/unnamed",
    enum_unnamed
);

pass_pattern!(
    (
        bool, f32, f64, i128, i16, i32, i64, i8, isize, string, u128, u16, u32, u64, u8, unit,
        usize, vec
    ),
    "./tests/models/enum/named",
    enum_named
);

pass_pattern!(unit, "./tests/models/enum", enum_unit);
