use dade::{json_dump, json_load};
use std::fs::File;
use std::io::{BufReader, Read};

macro_rules! test_success {
    ($test_name: ident, $file: literal) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_ok(), "{:?} {}", ret.err(), contents.trim());
            assert_eq!(json_dump(&ret.as_ref().unwrap(), true), contents.trim());
        }
    };
}

macro_rules! test_success_with_correct {
    ($test_name: ident, $file: literal, $correct: literal) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_ok());
            assert_eq!(json_dump(&ret.as_ref().unwrap(), true), $correct);
        }
    };
}

macro_rules! test_fail {
    ($test_name: ident, $file: literal) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let file = File::open($file).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let ret = json_load(contents.as_str());
            assert!(ret.is_err());
        }
    };
}

test_fail!(
    n_structure_uescaped_LF_before_string,
    "./tests/data/parsing/n_structure_uescaped_LF_before_string.json"
);
test_fail!(
    n_object_trailing_comment_open,
    "./tests/data/parsing/n_object_trailing_comment_open.json"
);
test_success!(
    y_array_ending_with_newline,
    "./tests/data/parsing/y_array_ending_with_newline.json"
);
test_fail!(
    i_structure_UTF_8_BOM_empty_object,
    "./tests/data/parsing/i_structure_UTF-8_BOM_empty_object.json"
);
test_fail!(
    n_structure_Up2060_word_joined,
    "./tests/data/parsing/n_structure_U+2060_word_joined.json"
);
test_fail!(
    n_structure_angle_bracket_null,
    "./tests/data/parsing/n_structure_angle_bracket_null.json"
);
test_fail!(
    n_number_real_without_fractional_part,
    "./tests/data/parsing/n_number_real_without_fractional_part.json"
);
test_success_with_correct!(
    y_string_uescaped_newline,
    "./tests/data/parsing/y_string_uescaped_newline.json",
    "[\"new\\nline\"]"
);
test_fail!(
    n_number_real_garbage_after_e,
    "./tests/data/parsing/n_number_real_garbage_after_e.json"
);
test_fail!(
    n_string_unescaped_newline,
    "./tests/data/parsing/n_string_unescaped_newline.json"
);
test_success!(
    y_number_negative_one,
    "./tests/data/parsing/y_number_negative_one.json"
);
test_success!(y_string_space, "./tests/data/parsing/y_string_space.json");
test_fail!(
    i_string_incomplete_surrogate_and_escape_valid,
    "./tests/data/parsing/i_string_incomplete_surrogate_and_escape_valid.json"
);
test_success!(
    i_number_huge_exp,
    "./tests/data/parsing/i_number_huge_exp.json"
);
test_fail!(
    n_structure_object_with_trailing_garbage,
    "./tests/data/parsing/n_structure_object_with_trailing_garbage.json"
);
test_success_with_correct!(
    y_string_surrogates_Up1D11E_MUSICAL_SYMBOL_G_CLEF,
    "./tests/data/parsing/y_string_surrogates_U+1D11E_MUSICAL_SYMBOL_G_CLEF.json",
    "[\"\\ud834\\udd1e\"]"
);
test_success!(
    y_structure_lonely_negative_real,
    "./tests/data/parsing/y_structure_lonely_negative_real.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_overlong_sequence_6_bytes_null,
//     "./tests/data/parsing/i_string_overlong_sequence_6_bytes_null.json"
// );
test_fail!(
    n_structure_number_with_trailing_garbage,
    "./tests/data/parsing/n_structure_number_with_trailing_garbage.json"
);
test_success!(y_array_empty, "./tests/data/parsing/y_array_empty.json");
test_success!(
    y_number_real_capital_e,
    "./tests/data/parsing/y_number_real_capital_e.json"
);
test_success!(
    y_string_unicode_Up2064_invisible_plus,
    "./tests/data/parsing/y_string_unicode_U+2064_invisible_plus.json"
);
test_fail!(
    n_structure_unclosed_array_unfinished_true,
    "./tests/data/parsing/n_structure_unclosed_array_unfinished_true.json"
);
test_success!(
    y_string_simple_ascii,
    "./tests/data/parsing/y_string_simple_ascii.json"
);
test_fail!(
    n_object_two_commas_in_a_row,
    "./tests/data/parsing/n_object_two_commas_in_a_row.json"
);
test_success!(y_array_false, "./tests/data/parsing/y_array_false.json");
test_success_with_correct!(
    y_string_in_array_with_leading_space,
    "./tests/data/parsing/y_string_in_array_with_leading_space.json",
    "[\"asd\"]"
);
test_success!(
    i_number_double_huge_neg_exp,
    "./tests/data/parsing/i_number_double_huge_neg_exp.json"
);
test_success!(y_object_simple, "./tests/data/parsing/y_object_simple.json");
test_fail!(
    n_array_colon_instead_of_comma,
    "./tests/data/parsing/n_array_colon_instead_of_comma.json"
);
test_fail!(
    n_object_trailing_comma,
    "./tests/data/parsing/n_object_trailing_comma.json"
);
test_success_with_correct!(
    y_string_1_2_3_bytes_UTF_8_sequences,
    "./tests/data/parsing/y_string_1_2_3_bytes_UTF-8_sequences.json",
    "[\"`\\u012a\\u12ab\"]"
);
test_fail!(
    n_string_1_surrogate_then_escape_u1,
    "./tests/data/parsing/n_string_1_surrogate_then_escape_u1.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_number_real_with_invalid_utf8_after_e,
//     "./tests/data/parsing/n_number_real_with_invalid_utf8_after_e.json"
// );
test_success!(
    y_array_with_several_null,
    "./tests/data/parsing/y_array_with_several_null.json"
);
test_fail!(
    n_array_star_inside,
    "./tests/data/parsing/n_array_star_inside.json"
);
test_success_with_correct!(
    y_string_unicode_Up10FFFE_nonchar,
    "./tests/data/parsing/y_string_unicode_U+10FFFE_nonchar.json",
    "[\"\\udbff\\udffe\"]"
);
test_fail!(
    n_structure_open_object,
    "./tests/data/parsing/n_structure_open_object.json"
);
test_fail!(
    n_object_double_colon,
    "./tests/data/parsing/n_object_double_colon.json"
);
test_fail!(
    n_array_unclosed_trailing_comma,
    "./tests/data/parsing/n_array_unclosed_trailing_comma.json"
);
test_success!(
    y_number_minus_zero,
    "./tests/data/parsing/y_number_minus_zero.json"
);
test_fail!(
    n_string_escaped_emoji,
    "./tests/data/parsing/n_string_escaped_emoji.json"
);
test_fail!(
    n_string_with_trailing_garbage,
    "./tests/data/parsing/n_string_with_trailing_garbage.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_string_invalid_utf_8_in_escape,
//     "./tests/data/parsing/n_string_invalid-utf-8-in-escape.json"
// );
test_success_with_correct!(
    y_string_pi,
    "./tests/data/parsing/y_string_pi.json",
    "[\"\\u03c0\"]"
);
test_fail!(
    n_structure_open_object_string_with_apostrophes,
    "./tests/data/parsing/n_structure_open_object_string_with_apostrophes.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_number_invalid_utf_8_in_exponent,
//     "./tests/data/parsing/n_number_invalid-utf-8-in-exponent.json"
// );
test_fail!(
    n_structure_end_array,
    "./tests/data/parsing/n_structure_end_array.json"
);
test_fail!(
    n_string_1_surrogate_then_escape_u,
    "./tests/data/parsing/n_string_1_surrogate_then_escape_u.json"
);
test_success!(
    y_number_double_close_to_zero,
    "./tests/data/parsing/y_number_double_close_to_zero.json"
);
test_fail!(
    n_number_invalidp_,
    "./tests/data/parsing/n_number_invalid+-.json"
);
test_success_with_correct!(
    y_string_unicode_Up1FFFE_nonchar,
    "./tests/data/parsing/y_string_unicode_U+1FFFE_nonchar.json",
    "[\"\\ud83f\\udffe\"]"
);
test_success!(
    i_number_too_big_pos_int,
    "./tests/data/parsing/i_number_too_big_pos_int.json"
);
test_success!(
    y_string_comments,
    "./tests/data/parsing/y_string_comments.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_UTF_16LE_with_BOM,
//     "./tests/data/parsing/i_string_UTF-16LE_with_BOM.json"
// );
test_success_with_correct!(
    y_string_unicode,
    "./tests/data/parsing/y_string_unicode.json",
    "[\"\\ua66d\"]"
);
test_success_with_correct!(
    y_string_unicode_Up200B_ZERO_WIDTH_SPACE,
    "./tests/data/parsing/y_string_unicode_U+200B_ZERO_WIDTH_SPACE.json",
    "[\"\\u200b\"]"
);
test_fail!(
    n_structure_lone_open_bracket,
    "./tests/data/parsing/n_structure_lone-open-bracket.json"
);
test_fail!(
    n_object_bracket_key,
    "./tests/data/parsing/n_object_bracket_key.json"
);
test_fail!(
    y_object_duplicated_key,
    "./tests/data/parsing/y_object_duplicated_key.json"
);
test_fail!(
    n_object_missing_semicolon,
    "./tests/data/parsing/n_object_missing_semicolon.json"
);
test_fail!(
    i_string_incomplete_surrogates_escape_valid,
    "./tests/data/parsing/i_string_incomplete_surrogates_escape_valid.json"
);
test_fail!(
    i_string_1st_surrogate_but_2nd_missing,
    "./tests/data/parsing/i_string_1st_surrogate_but_2nd_missing.json"
);
test_fail!(n_number_0e, "./tests/data/parsing/n_number_0e.json");
test_fail!(n_number_2de3, "./tests/data/parsing/n_number_2.e3.json");
test_fail!(n_number__1d0d, "./tests/data/parsing/n_number_-1.0..json");
test_success!(
    y_number_real_pos_exponent,
    "./tests/data/parsing/y_number_real_pos_exponent.json"
);
test_fail!(
    n_array_extra_close,
    "./tests/data/parsing/n_array_extra_close.json"
);
test_success!(
    y_number_negative_zero,
    "./tests/data/parsing/y_number_negative_zero.json"
);
test_success_with_correct!(
    y_number_after_space,
    "./tests/data/parsing/y_number_after_space.json",
    "[4]"
);
test_fail!(
    n_number_invalid_negative_real,
    "./tests/data/parsing/n_number_invalid-negative-real.json"
);
test_success_with_correct!(
    y_string_up2028_line_sep,
    "./tests/data/parsing/y_string_u+2028_line_sep.json",
    "[\"\\u2028\"]"
);
test_fail!(
    n_string_single_doublequote,
    "./tests/data/parsing/n_string_single_doublequote.json"
);
test_fail!(n_number_Inf, "./tests/data/parsing/n_number_Inf.json");
test_success!(
    y_string_escaped_control_character,
    "./tests/data/parsing/y_string_escaped_control_character.json"
);
test_success_with_correct!(
    y_object_string_unicode,
    "./tests/data/parsing/y_object_string_unicode.json",
    "{\"title\":\"\\u041f\\u043e\\u043b\\u0442\\u043e\\u0440\\u0430 \\u0417\\u0435\\u043c\\u043b\\u0435\\u043a\\u043e\\u043f\\u0430\"}"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_lone_utf8_continuation_byte,
//     "./tests/data/parsing/i_string_lone_utf8_continuation_byte.json"
// );
test_success!(
    y_array_empty_string,
    "./tests/data/parsing/y_array_empty-string.json"
);
test_fail!(
    n_array_inner_array_no_comma,
    "./tests/data/parsing/n_array_inner_array_no_comma.json"
);
test_success_with_correct!(
    y_string_one_byte_utf_8,
    "./tests/data/parsing/y_string_one-byte-utf-8.json",
    "[\",\"]"
);
test_fail!(
    n_object_no_colon,
    "./tests/data/parsing/n_object_no-colon.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_array_a_invalid_utf8,
//     "./tests/data/parsing/n_array_a_invalid_utf8.json"
// );
test_fail!(
    n_object_comma_instead_of_colon,
    "./tests/data/parsing/n_object_comma_instead_of_colon.json"
);
test_success!(
    y_number_real_exponent,
    "./tests/data/parsing/y_number_real_exponent.json"
);
test_fail!(
    n_number_minus_space_1,
    "./tests/data/parsing/n_number_minus_space_1.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_UTF8_surrogate_UpD800,
//     "./tests/data/parsing/i_string_UTF8_surrogate_U+D800.json"
// );
test_fail!(
    i_string_inverted_surrogates_Up1D11E,
    "./tests/data/parsing/i_string_inverted_surrogates_U+1D11E.json"
);
test_success!(
    y_string_two_byte_utf_8,
    "./tests/data/parsing/y_string_two-byte-utf-8.json"
);
test_fail!(n_number_2dep3, "./tests/data/parsing/n_number_2.e+3.json");
// invalid UTF-8 code
// test_fail!(
//     i_string_not_in_unicode_range,
//     "./tests/data/parsing/i_string_not_in_unicode_range.json"
// );
test_fail!(
    n_object_unterminated_value,
    "./tests/data/parsing/n_object_unterminated-value.json"
);
test_fail!(
    n_object_missing_value,
    "./tests/data/parsing/n_object_missing_value.json"
);
test_fail!(
    n_structure_open_array_apostrophe,
    "./tests/data/parsing/n_structure_open_array_apostrophe.json"
);
test_fail!(
    n_structure_comma_instead_of_closing_brace,
    "./tests/data/parsing/n_structure_comma_instead_of_closing_brace.json"
);
test_fail!(
    n_structure_single_star,
    "./tests/data/parsing/n_structure_single_star.json"
);
test_fail!(
    n_structure_unclosed_array,
    "./tests/data/parsing/n_structure_unclosed_array.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_object_lone_continuation_byte_in_key_and_trailing_comma,
//     "./tests/data/parsing/n_object_lone_continuation_byte_in_key_and_trailing_comma.json"
// );
test_success!(y_number_0ep1, "./tests/data/parsing/y_number_0e+1.json");
test_success!(
    y_structure_lonely_int,
    "./tests/data/parsing/y_structure_lonely_int.json"
);
test_fail!(
    n_structure_UTF8_BOM_no_data,
    "./tests/data/parsing/n_structure_UTF8_BOM_no_data.json"
);
test_success!(
    y_number_real_neg_exp,
    "./tests/data/parsing/y_number_real_neg_exp.json"
);
test_fail!(
    n_structure_open_object_open_array,
    "./tests/data/parsing/n_structure_open_object_open_array.json"
);
test_fail!(n_number_pInf, "./tests/data/parsing/n_number_+Inf.json");
test_fail!(n_number_0ep, "./tests/data/parsing/n_number_0e+.json");
test_fail!(n_number_d_1, "./tests/data/parsing/n_number_.-1.json");
test_fail!(
    n_number_minus_infinity,
    "./tests/data/parsing/n_number_minus_infinity.json"
);
test_fail!(
    n_object_non_string_key,
    "./tests/data/parsing/n_object_non_string_key.json"
);
test_fail!(n_object_emoji, "./tests/data/parsing/n_object_emoji.json");
test_fail!(
    i_string_1st_valid_surrogate_2nd_invalid,
    "./tests/data/parsing/i_string_1st_valid_surrogate_2nd_invalid.json"
);
test_success_with_correct!(
    y_object_with_newlines,
    "./tests/data/parsing/y_object_with_newlines.json",
    "{\"a\":\"b\"}"
);
test_fail!(
    n_string_1_surrogate_then_escape,
    "./tests/data/parsing/n_string_1_surrogate_then_escape.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_structure_lone_invalid_utf_8,
//     "./tests/data/parsing/n_structure_lone-invalid-utf-8.json"
// );
// invalid UTF-8 code
// test_fail!(
//     i_string_iso_latin_1,
//     "./tests/data/parsing/i_string_iso_latin_1.json"
// );
test_fail!(
    n_string_single_quote,
    "./tests/data/parsing/n_string_single_quote.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_structure_incomplete_UTF8_BOM,
//     "./tests/data/parsing/n_structure_incomplete_UTF8_BOM.json"
// );
test_fail!(
    i_string_invalid_lonely_surrogate,
    "./tests/data/parsing/i_string_invalid_lonely_surrogate.json"
);
test_fail!(
    n_object_repeated_null_null,
    "./tests/data/parsing/n_object_repeated_null_null.json"
);
test_success_with_correct!(
    y_string_unicode_UpFFFE_nonchar,
    "./tests/data/parsing/y_string_unicode_U+FFFE_nonchar.json",
    "[\"\\ufffe\"]"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_utf16BE_no_BOM,
//     "./tests/data/parsing/i_string_utf16BE_no_BOM.json"
// );
test_success!(
    y_string_double_escape_a,
    "./tests/data/parsing/y_string_double_escape_a.json"
);
test_fail!(n_number_9dep, "./tests/data/parsing/n_number_9.e+.json");
test_fail!(
    n_number_expression,
    "./tests/data/parsing/n_number_expression.json"
);
test_success_with_correct!(
    y_object_long_strings,
    "./tests/data/parsing/y_object_long_strings.json",
    "{\"x\":[{\"id\":\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"}],\"id\":\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"}"
);
test_success_with_correct!(
    y_string_last_surrogates_1_and_2,
    "./tests/data/parsing/y_string_last_surrogates_1_and_2.json",
    "[\"\\udbff\\udfff\"]"
);
test_fail!(
    n_object_missing_key,
    "./tests/data/parsing/n_object_missing_key.json"
);
test_success!(
    y_string_backslash_doublequotes,
    "./tests/data/parsing/y_string_backslash_doublequotes.json"
);
test_fail!(
    n_array_just_comma,
    "./tests/data/parsing/n_array_just_comma.json"
);
test_success!(
    y_number_simple_real,
    "./tests/data/parsing/y_number_simple_real.json"
);
test_fail!(n_number_1_000, "./tests/data/parsing/n_number_1_000.json");
test_success!(
    y_number_negative_int,
    "./tests/data/parsing/y_number_negative_int.json"
);
test_fail!(
    n_number_0_capital_Ep,
    "./tests/data/parsing/n_number_0_capital_E+.json"
);
test_fail!(
    n_string_incomplete_surrogate,
    "./tests/data/parsing/n_string_incomplete_surrogate.json"
);
test_success!(
    y_array_with_leading_space,
    "./tests/data/parsing/y_array_with_leading_space.json"
);
test_fail!(
    n_string_unescaped_crtl_char,
    "./tests/data/parsing/n_string_unescaped_crtl_char.json"
);
test_fail!(
    n_string_leading_uescaped_thinspace,
    "./tests/data/parsing/n_string_leading_uescaped_thinspace.json"
);
test_fail!(
    n_structure_angle_bracket_d,
    "./tests/data/parsing/n_structure_angle_bracket_..json"
);
test_success!(
    y_number_int_with_exp,
    "./tests/data/parsing/y_number_int_with_exp.json"
);
test_fail!(n_number__01, "./tests/data/parsing/n_number_-01.json");
test_fail!(
    n_structure_open_array_open_string,
    "./tests/data/parsing/n_structure_open_array_open_string.json"
);
test_fail!(
    n_string_escape_x,
    "./tests/data/parsing/n_string_escape_x.json"
);
test_fail!(
    n_string_unicode_CapitalU,
    "./tests/data/parsing/n_string_unicode_CapitalU.json"
);
test_success!(y_array_null, "./tests/data/parsing/y_array_null.json");
test_fail!(n_number_1d0e, "./tests/data/parsing/n_number_1.0e.json");
test_success!(y_number_0e1, "./tests/data/parsing/y_number_0e1.json");
test_success_with_correct!(
    y_string_up2029_par_sep,
    "./tests/data/parsing/y_string_u+2029_par_sep.json",
    "[\"\\u2029\"]"
);
test_fail!(
    i_object_key_lone_2nd_surrogate,
    "./tests/data/parsing/i_object_key_lone_2nd_surrogate.json"
);
test_success_with_correct!(
    y_string_unicodeEscapedBackslash,
    "./tests/data/parsing/y_string_unicodeEscapedBackslash.json",
    "[\"\\\\\"]"
);
test_success!(
    y_structure_true_in_array,
    "./tests/data/parsing/y_structure_true_in_array.json"
);
test_success_with_correct!(
    y_string_unicode_2,
    "./tests/data/parsing/y_string_unicode_2.json",
    "[\"\\u2342\\u3234\\u2342\"]"
);
test_fail!(
    n_object_missing_colon,
    "./tests/data/parsing/n_object_missing_colon.json"
);
test_success!(
    y_structure_lonely_false,
    "./tests/data/parsing/y_structure_lonely_false.json"
);
test_success_with_correct!(
    y_string_nonCharacterInUTF_8_Up10FFFF,
    "./tests/data/parsing/y_string_nonCharacterInUTF-8_U+10FFFF.json",
    "[\"\\udbff\\udfff\"]"
);
test_fail!(
    n_object_several_trailing_commas,
    "./tests/data/parsing/n_object_several_trailing_commas.json"
);
test_success_with_correct!(
    y_string_reservedCharacterInUTF_8_Up1BFFF,
    "./tests/data/parsing/y_string_reservedCharacterInUTF-8_U+1BFFF.json",
    "[\"\\ud82f\\udfff\"]"
);
test_fail!(
    y_object_duplicated_key_and_value,
    "./tests/data/parsing/y_object_duplicated_key_and_value.json"
);
test_fail!(
    n_structure_double_array,
    "./tests/data/parsing/n_structure_double_array.json"
);
test_fail!(
    n_string_accentuated_char_no_quotes,
    "./tests/data/parsing/n_string_accentuated_char_no_quotes.json"
);
test_fail!(n_number_1d0e_, "./tests/data/parsing/n_number_1.0e-.json");
test_fail!(
    n_object_bad_value,
    "./tests/data/parsing/n_object_bad_value.json"
);
test_success!(
    y_structure_lonely_null,
    "./tests/data/parsing/y_structure_lonely_null.json"
);
test_fail!(
    n_number_0_capital_E,
    "./tests/data/parsing/n_number_0_capital_E.json"
);
test_fail!(n_number_0de1, "./tests/data/parsing/n_number_0.e1.json");
test_success!(
    i_number_real_pos_overflow,
    "./tests/data/parsing/i_number_real_pos_overflow.json"
);
test_success_with_correct!(
    y_array_heterogeneous,
    "./tests/data/parsing/y_array_heterogeneous.json",
    "[null,1,\"1\",{}]"
);
test_fail!(
    n_number_starting_with_dot,
    "./tests/data/parsing/n_number_starting_with_dot.json"
);
test_fail!(
    n_number_minus_sign_with_trailing_garbage,
    "./tests/data/parsing/n_number_minus_sign_with_trailing_garbage.json"
);
test_fail!(
    n_string_1_surrogate_then_escape_u1x,
    "./tests/data/parsing/n_string_1_surrogate_then_escape_u1x.json"
);
test_fail!(
    n_array_items_separated_by_semicolon,
    "./tests/data/parsing/n_array_items_separated_by_semicolon.json"
);
test_fail!(
    n_string_escaped_ctrl_char_tab,
    "./tests/data/parsing/n_string_escaped_ctrl_char_tab.json"
);
test_fail!(
    n_structure_array_trailing_garbage,
    "./tests/data/parsing/n_structure_array_trailing_garbage.json"
);
test_fail!(
    n_structure_whitespace_formfeed,
    "./tests/data/parsing/n_structure_whitespace_formfeed.json"
);
test_fail!(n_number_1d0ep, "./tests/data/parsing/n_number_1.0e+.json");
test_fail!(
    n_string_invalid_backslash_esc,
    "./tests/data/parsing/n_string_invalid_backslash_esc.json"
);
test_fail!(
    n_structure_open_array_comma,
    "./tests/data/parsing/n_structure_open_array_comma.json"
);
test_fail!(
    n_structure_unicode_identifier,
    "./tests/data/parsing/n_structure_unicode-identifier.json"
);
test_fail!(
    n_number_hex_1_digit,
    "./tests/data/parsing/n_number_hex_1_digit.json"
);
test_fail!(
    n_array_extra_comma,
    "./tests/data/parsing/n_array_extra_comma.json"
);
test_fail!(
    n_array_incomplete,
    "./tests/data/parsing/n_array_incomplete.json"
);
test_fail!(
    n_structure_no_data,
    "./tests/data/parsing/n_structure_no_data.json"
);
test_success_with_correct!(
    y_string_escaped_noncharacter,
    "./tests/data/parsing/y_string_escaped_noncharacter.json",
    "[\"\\uffff\"]"
);
test_fail!(
    n_array_number_and_several_commas,
    "./tests/data/parsing/n_array_number_and_several_commas.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_invalid_utf_8,
//     "./tests/data/parsing/i_string_invalid_utf-8.json"
// );
test_fail!(
    n_structure_object_followed_by_closing_object,
    "./tests/data/parsing/n_structure_object_followed_by_closing_object.json"
);
test_fail!(
    n_incomplete_true,
    "./tests/data/parsing/n_incomplete_true.json"
);
test_success!(
    y_string_null_escape,
    "./tests/data/parsing/y_string_null_escape.json"
);
test_success_with_correct!(
    y_string_unicode_escaped_double_quote,
    "./tests/data/parsing/y_string_unicode_escaped_double_quote.json",
    "[\"\\\"\"]"
);
test_success_with_correct!(
    y_string_nonCharacterInUTF_8_UpFFFF,
    "./tests/data/parsing/y_string_nonCharacterInUTF-8_U+FFFF.json",
    "[\"\\uffff\"]"
);
test_fail!(
    n_string_unescaped_tab,
    "./tests/data/parsing/n_string_unescaped_tab.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_overlong_sequence_2_bytes,
//     "./tests/data/parsing/i_string_overlong_sequence_2_bytes.json"
// );
test_fail!(
    n_object_trailing_comment_slash_open,
    "./tests/data/parsing/n_object_trailing_comment_slash_open.json"
);
test_fail!(
    n_structure_trailing_sharp,
    "./tests/data/parsing/n_structure_trailing_#.json"
);
test_success!(
    y_object_empty_key,
    "./tests/data/parsing/y_object_empty_key.json"
);
test_fail!(
    n_structure_unclosed_object,
    "./tests/data/parsing/n_structure_unclosed_object.json"
);
test_fail!(
    n_number_neg_real_without_int_part,
    "./tests/data/parsing/n_number_neg_real_without_int_part.json"
);
test_fail!(n_number_0d3e, "./tests/data/parsing/n_number_0.3e.json");
test_fail!(
    n_structure_null_byte_outside_string,
    "./tests/data/parsing/n_structure_null-byte-outside-string.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_truncated_utf_8,
//     "./tests/data/parsing/i_string_truncated-utf-8.json"
// );
test_success!(y_object_empty, "./tests/data/parsing/y_object_empty.json");
test_fail!(
    n_string_single_string_no_double_quotes,
    "./tests/data/parsing/n_string_single_string_no_double_quotes.json"
);
test_fail!(
    n_incomplete_false,
    "./tests/data/parsing/n_incomplete_false.json"
);
test_success_with_correct!(
    y_string_nbsp_uescaped,
    "./tests/data/parsing/y_string_nbsp_uescaped.json",
    "[\"new\\u00a0line\"]"
);
test_fail!(n_number_p1, "./tests/data/parsing/n_number_+1.json");
test_fail!(
    n_incomplete_null,
    "./tests/data/parsing/n_incomplete_null.json"
);
test_fail!(
    n_array_unclosed_with_new_lines,
    "./tests/data/parsing/n_array_unclosed_with_new_lines.json"
);
test_success!(
    y_structure_lonely_true,
    "./tests/data/parsing/y_structure_lonely_true.json"
);
test_fail!(
    n_array_spaces_vertical_tab_formfeed,
    "./tests/data/parsing/n_array_spaces_vertical_tab_formfeed.json"
);
test_success!(
    i_number_real_underflow,
    "./tests/data/parsing/i_number_real_underflow.json"
);
test_success_with_correct!(
    y_object_escaped_null_in_key,
    "./tests/data/parsing/y_object_escaped_null_in_key.json",
    "{\"foo\\u0000bar\":42}"
);
test_fail!(
    n_structure_open_open,
    "./tests/data/parsing/n_structure_open_open.json"
);
test_success!(y_number, "./tests/data/parsing/y_number.json");
// invalid UTF-8 code
// test_fail!(
//     n_structure_single_eacute,
//     "./tests/data/parsing/n_structure_single_eacute.json"
// );
test_success_with_correct!(
    y_object_extreme_numbers,
    "./tests/data/parsing/y_object_extreme_numbers.json",
    "{\"min\":-1.0e+28,\"max\":1.0e+28}"
);
test_success_with_correct!(
    y_array_with_1_and_newline,
    "./tests/data/parsing/y_array_with_1_and_newline.json",
    "[1]"
);
test_fail!(n_number_0d1d2, "./tests/data/parsing/n_number_0.1.2.json");
test_success!(
    y_structure_whitespace_array,
    "./tests/data/parsing/y_structure_whitespace_array.json"
);
test_fail!(
    n_structure_open_object_open_string,
    "./tests/data/parsing/n_structure_open_object_open_string.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_string_invalid_utf8_after_escape,
//     "./tests/data/parsing/n_string_invalid_utf8_after_escape.json"
// );
test_success!(
    y_string_double_escape_n,
    "./tests/data/parsing/y_string_double_escape_n.json"
);
test_fail!(
    n_structure_ascii_unicode_identifier,
    "./tests/data/parsing/n_structure_ascii-unicode-identifier.json"
);
test_fail!(
    n_array_incomplete_invalid_value,
    "./tests/data/parsing/n_array_incomplete_invalid_value.json"
);
test_fail!(
    n_string_invalid_unicode_escape,
    "./tests/data/parsing/n_string_invalid_unicode_escape.json"
);
test_fail!(
    n_array_just_minus,
    "./tests/data/parsing/n_array_just_minus.json"
);
test_success_with_correct!(
    y_string_uEscape,
    "./tests/data/parsing/y_string_uEscape.json",
    "[\"a\\u30af\\u30ea\\u30b9\"]"
);
test_fail!(n_number_d2e_3, "./tests/data/parsing/n_number_.2e-3.json");
test_fail!(
    n_string_escaped_backslash_bad,
    "./tests/data/parsing/n_string_escaped_backslash_bad.json"
);
test_fail!(
    n_array_double_comma,
    "./tests/data/parsing/n_array_double_comma.json"
);
test_fail!(
    n_string_start_escape_unclosed,
    "./tests/data/parsing/n_string_start_escape_unclosed.json"
);
test_success!(
    y_number_real_capital_e_pos_exp,
    "./tests/data/parsing/y_number_real_capital_e_pos_exp.json"
);
test_fail!(
    n_string_incomplete_escaped_character,
    "./tests/data/parsing/n_string_incomplete_escaped_character.json"
);
test_success!(
    i_number_too_big_neg_int,
    "./tests/data/parsing/i_number_too_big_neg_int.json"
);
test_success!(
    y_structure_trailing_newline,
    "./tests/data/parsing/y_structure_trailing_newline.json"
);
test_fail!(
    n_array_missing_value,
    "./tests/data/parsing/n_array_missing_value.json"
);
test_fail!(
    n_structure_object_with_comment,
    "./tests/data/parsing/n_structure_object_with_comment.json"
);
test_fail!(
    n_number_neg_with_garbage_at_end,
    "./tests/data/parsing/n_number_neg_with_garbage_at_end.json"
);
test_fail!(
    n_multidigit_number_then_00,
    "./tests/data/parsing/n_multidigit_number_then_00.json"
);
test_fail!(
    n_array_newlines_unclosed,
    "./tests/data/parsing/n_array_newlines_unclosed.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_array_invalid_utf8,
//     "./tests/data/parsing/n_array_invalid_utf8.json"
// );
test_fail!(
    n_number_with_alpha_char,
    "./tests/data/parsing/n_number_with_alpha_char.json"
);
test_fail!(
    n_number_infinity,
    "./tests/data/parsing/n_number_infinity.json"
);
test_fail!(n_number__2d, "./tests/data/parsing/n_number_-2..json");
test_fail!(
    n_structure_object_unclosed_no_value,
    "./tests/data/parsing/n_structure_object_unclosed_no_value.json"
);
test_fail!(
    n_string_incomplete_escape,
    "./tests/data/parsing/n_string_incomplete_escape.json"
);
test_fail!(
    n_object_non_string_key_but_huge_number_instead,
    "./tests/data/parsing/n_object_non_string_key_but_huge_number_instead.json"
);
test_success_with_correct!(
    y_string_unicode_UpFDD0_nonchar,
    "./tests/data/parsing/y_string_unicode_U+FDD0_nonchar.json",
    "[\"\\ufdd0\"]"
);
test_success!(
    i_number_very_big_negative_int,
    "./tests/data/parsing/i_number_very_big_negative_int.json"
);
test_fail!(
    n_structure_capitalized_True,
    "./tests/data/parsing/n_structure_capitalized_True.json"
);
test_fail!(
    n_array_double_extra_comma,
    "./tests/data/parsing/n_array_double_extra_comma.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_utf16LE_no_BOM,
//     "./tests/data/parsing/i_string_utf16LE_no_BOM.json"
// );
test_fail!(
    n_array_unclosed_with_object_inside,
    "./tests/data/parsing/n_array_unclosed_with_object_inside.json"
);
test_success_with_correct!(
    y_string_with_del_character,
    "./tests/data/parsing/y_string_with_del_character.json",
    "[\"a\\u007fa\"]"
);
test_success_with_correct!(
    y_string_utf8,
    "./tests/data/parsing/y_string_utf8.json",
    "[\"\\u20ac\\ud834\\udd1e\"]"
);
test_fail!(n_number_1eE2, "./tests/data/parsing/n_number_1eE2.json");
test_fail!(
    n_object_with_trailing_garbage,
    "./tests/data/parsing/n_object_with_trailing_garbage.json"
);
test_fail!(n_number_pp, "./tests/data/parsing/n_number_++.json");
test_success!(y_object_basic, "./tests/data/parsing/y_object_basic.json");
test_fail!(
    n_structure_array_with_unclosed_string,
    "./tests/data/parsing/n_structure_array_with_unclosed_string.json"
);
test_success!(
    y_number_real_capital_e_neg_exp,
    "./tests/data/parsing/y_number_real_capital_e_neg_exp.json"
);
test_fail!(
    n_string_incomplete_surrogate_escape_invalid,
    "./tests/data/parsing/n_string_incomplete_surrogate_escape_invalid.json"
);
test_fail!(
    n_object_trailing_comment,
    "./tests/data/parsing/n_object_trailing_comment.json"
);
test_success!(
    y_array_with_trailing_space,
    "./tests/data/parsing/y_array_with_trailing_space.json"
);
test_fail!(
    n_object_unquoted_key,
    "./tests/data/parsing/n_object_unquoted_key.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_overlong_sequence_6_bytes,
//     "./tests/data/parsing/i_string_overlong_sequence_6_bytes.json"
// );
test_fail!(
    n_object_with_single_string,
    "./tests/data/parsing/n_object_with_single_string.json"
);
test_fail!(
    n_array_unclosed,
    "./tests/data/parsing/n_array_unclosed.json"
);
test_fail!(n_number__NaN, "./tests/data/parsing/n_number_-NaN.json");
test_fail!(n_number_2de_3, "./tests/data/parsing/n_number_2.e-3.json");
test_success!(
    i_number_pos_double_huge_exp,
    "./tests/data/parsing/i_number_pos_double_huge_exp.json"
);
test_success_with_correct!(
    y_string_accepted_surrogate_pairs,
    "./tests/data/parsing/y_string_accepted_surrogate_pairs.json",
    "[\"\\ud83d\\ude39\\ud83d\\udc8d\"]"
);
test_fail!(
    n_number_neg_int_starting_with_zero,
    "./tests/data/parsing/n_number_neg_int_starting_with_zero.json"
);
test_fail!(n_number_NaN, "./tests/data/parsing/n_number_NaN.json");
test_success_with_correct!(
    y_string_allowed_escapes,
    "./tests/data/parsing/y_string_allowed_escapes.json",
    "[\"\\\"\\\\/\\b\\f\\n\\r\\t\"]"
);
test_fail!(
    n_number_UpFF11_fullwidth_digit_one,
    "./tests/data/parsing/n_number_U+FF11_fullwidth_digit_one.json"
);
test_success!(
    y_string_in_array,
    "./tests/data/parsing/y_string_in_array.json"
);
test_fail!(
    n_structure_array_with_extra_array_close,
    "./tests/data/parsing/n_structure_array_with_extra_array_close.json"
);
test_fail!(
    n_array_comma_and_number,
    "./tests/data/parsing/n_array_comma_and_number.json"
);
test_fail!(
    i_string_lone_second_surrogate,
    "./tests/data/parsing/i_string_lone_second_surrogate.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_number_invalid_utf_8_in_int,
//     "./tests/data/parsing/n_number_invalid-utf-8-in-int.json"
// );
test_fail!(
    n_object_key_with_single_quotes,
    "./tests/data/parsing/n_object_key_with_single_quotes.json"
);
test_success_with_correct!(
    y_array_arraysWithSpaces,
    "./tests/data/parsing/y_array_arraysWithSpaces.json",
    "[[]]"
);
test_fail!(n_single_space, "./tests/data/parsing/n_single_space.json");
test_success!(
    y_string_backslash_and_u_escaped_zero,
    "./tests/data/parsing/y_string_backslash_and_u_escaped_zero.json"
);
test_success!(
    y_structure_lonely_string,
    "./tests/data/parsing/y_structure_lonely_string.json"
);
test_success!(
    y_string_three_byte_utf_8,
    "./tests/data/parsing/y_string_three-byte-utf-8.json"
);
test_fail!(
    n_structure_open_array_string,
    "./tests/data/parsing/n_structure_open_array_string.json"
);
test_fail!(
    n_number_hex_2_digits,
    "./tests/data/parsing/n_number_hex_2_digits.json"
);
test_success!(
    y_number_simple_int,
    "./tests/data/parsing/y_number_simple_int.json"
);
test_success!(
    y_structure_string_empty,
    "./tests/data/parsing/y_structure_string_empty.json"
);
test_fail!(
    i_string_incomplete_surrogate_pair,
    "./tests/data/parsing/i_string_incomplete_surrogate_pair.json"
);
test_fail!(
    n_structure_unclosed_array_unfinished_false,
    "./tests/data/parsing/n_structure_unclosed_array_unfinished_false.json"
);
test_fail!(
    n_structure_open_object_comma,
    "./tests/data/parsing/n_structure_open_object_comma.json"
);
test_fail!(
    n_structure_unclosed_array_partial_null,
    "./tests/data/parsing/n_structure_unclosed_array_partial_null.json"
);
test_fail!(
    n_structure_open_object_close_array,
    "./tests/data/parsing/n_structure_open_object_close_array.json"
);
test_success!(
    i_number_neg_int_huge_exp,
    "./tests/data/parsing/i_number_neg_int_huge_exp.json"
);
test_success_with_correct!(
    y_string_unescaped_char_delete,
    "./tests/data/parsing/y_string_unescaped_char_delete.json",
    "[\"\\u007f\"]"
);
test_fail!(
    n_object_trailing_comment_slash_open_incomplete,
    "./tests/data/parsing/n_object_trailing_comment_slash_open_incomplete.json"
);
test_fail!(
    n_array_comma_after_close,
    "./tests/data/parsing/n_array_comma_after_close.json"
);
test_success!(
    i_number_real_neg_overflow,
    "./tests/data/parsing/i_number_real_neg_overflow.json"
);
test_fail!(
    n_object_single_quote,
    "./tests/data/parsing/n_object_single_quote.json"
);
test_fail!(
    n_number_with_leading_zero,
    "./tests/data/parsing/n_number_with_leading_zero.json"
);
test_fail!(n_number_0d3ep, "./tests/data/parsing/n_number_0.3e+.json");
test_fail!(
    n_structure_whitespace_Up2060_word_joiner,
    "./tests/data/parsing/n_structure_whitespace_U+2060_word_joiner.json"
);
// invalid UTF-8 code
// test_fail!(
//     n_number_invalid_utf_8_in_bigger_int,
//     "./tests/data/parsing/n_number_invalid-utf-8-in-bigger-int.json"
// );
test_success_with_correct!(
    y_object,
    "./tests/data/parsing/y_object.json",
    "{\"asd\":\"sdf\",\"dfg\":\"fgh\"}"
);
test_fail!(
    n_string_no_quotes_with_bad_escape,
    "./tests/data/parsing/n_string_no_quotes_with_bad_escape.json"
);
test_success_with_correct!(
    y_string_accepted_surrogate_pair,
    "./tests/data/parsing/y_string_accepted_surrogate_pair.json",
    "[\"\\ud801\\udc37\"]"
);
test_success!(
    y_number_real_fraction_exponent,
    "./tests/data/parsing/y_number_real_fraction_exponent.json"
);
test_fail!(
    n_structure_open_array_open_object,
    "./tests/data/parsing/n_structure_open_array_open_object.json"
);
test_fail!(
    n_structure_close_unopened_array,
    "./tests/data/parsing/n_structure_close_unopened_array.json"
);
test_fail!(
    n_number_with_alpha,
    "./tests/data/parsing/n_number_with_alpha.json"
);
// invalid UTF-8 code
// test_fail!(
//     i_string_UTF_8_invalid_sequence,
//     "./tests/data/parsing/i_string_UTF-8_invalid_sequence.json"
// );
test_fail!(
    i_string_invalid_surrogate,
    "./tests/data/parsing/i_string_invalid_surrogate.json"
);
test_fail!(
    n_string_backslash_00,
    "./tests/data/parsing/n_string_backslash_00.json"
);
test_fail!(
    n_array_1_true_without_comma,
    "./tests/data/parsing/n_array_1_true_without_comma.json"
);
test_fail!(
    n_array_number_and_comma,
    "./tests/data/parsing/n_array_number_and_comma.json"
);
test_fail!(
    n_object_garbage_at_end,
    "./tests/data/parsing/n_object_garbage_at_end.json"
);
