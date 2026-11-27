#[macro_export]
macro_rules! unit_test_matcher {
    ( $Matcher:ident ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::assert_matcher_case;

            #[test]
            fn null_pattern() {
                assert_matcher_case!($Matcher, "", "", &[0]);
                assert_matcher_case!($Matcher, "", "x", &[0, 1]);
                assert_matcher_case!($Matcher, "", "xy", &[0, 1, 2]);
                assert_matcher_case!($Matcher, "", "xyz", &[0, 1, 2, 3]);
            }

            #[test]
            fn single_char_pattern() {
                assert_matcher_case!($Matcher, "x", "", &[]);
                assert_matcher_case!($Matcher, "x", "x", &[0]);
                assert_matcher_case!($Matcher, "x", "y", &[]);
                assert_matcher_case!($Matcher, "x", "xx", &[0, 1]);
                assert_matcher_case!($Matcher, "x", "xy", &[0]);
                assert_matcher_case!($Matcher, "x", "yx", &[1]);
                assert_matcher_case!($Matcher, "x", "yy", &[]);
                assert_matcher_case!($Matcher, "x", "xxx", &[0, 1, 2]);
                assert_matcher_case!($Matcher, "x", "xxy", &[0, 1]);
                assert_matcher_case!($Matcher, "x", "xyx", &[0, 2]);
                assert_matcher_case!($Matcher, "x", "xyy", &[0]);
                assert_matcher_case!($Matcher, "x", "yxx", &[1, 2]);
                assert_matcher_case!($Matcher, "x", "yxy", &[1]);
                assert_matcher_case!($Matcher, "x", "yyx", &[2]);
                assert_matcher_case!($Matcher, "x", "yyy", &[]);
            }

            #[test]
            fn duplicate_char_pattern() {
                assert_matcher_case!($Matcher, "xx", "", &[]);
                assert_matcher_case!($Matcher, "xx", "x", &[]);
                assert_matcher_case!($Matcher, "xx", "y", &[]);
                assert_matcher_case!($Matcher, "xx", "xx", &[0]);
                assert_matcher_case!($Matcher, "xx", "xy", &[]);
                assert_matcher_case!($Matcher, "xx", "yx", &[]);
                assert_matcher_case!($Matcher, "xx", "yy", &[]);
                assert_matcher_case!($Matcher, "xx", "xxx", &[0, 1]);
                assert_matcher_case!($Matcher, "xx", "xxy", &[0]);
                assert_matcher_case!($Matcher, "xx", "xyx", &[]);
                assert_matcher_case!($Matcher, "xx", "xyy", &[]);
                assert_matcher_case!($Matcher, "xx", "yxx", &[1]);
                assert_matcher_case!($Matcher, "xx", "yxy", &[]);
                assert_matcher_case!($Matcher, "xx", "yyx", &[]);
                assert_matcher_case!($Matcher, "xx", "yyy", &[]);
                assert_matcher_case!($Matcher, "xx", "xxxx", &[0, 1, 2]);
                assert_matcher_case!($Matcher, "xx", "xxxy", &[0, 1]);
                assert_matcher_case!($Matcher, "xx", "xxyx", &[0]);
                assert_matcher_case!($Matcher, "xx", "xxyy", &[0]);
                assert_matcher_case!($Matcher, "xx", "xyxx", &[2]);
                assert_matcher_case!($Matcher, "xx", "xyxy", &[]);
                assert_matcher_case!($Matcher, "xx", "xyyx", &[]);
                assert_matcher_case!($Matcher, "xx", "xyyy", &[]);
                assert_matcher_case!($Matcher, "xx", "yxxx", &[1, 2]);
                assert_matcher_case!($Matcher, "xx", "yxxy", &[1]);
                assert_matcher_case!($Matcher, "xx", "yxyx", &[]);
                assert_matcher_case!($Matcher, "xx", "yxyy", &[]);
                assert_matcher_case!($Matcher, "xx", "yyxx", &[2]);
                assert_matcher_case!($Matcher, "xx", "yyxy", &[]);
                assert_matcher_case!($Matcher, "xx", "yyyx", &[]);
                assert_matcher_case!($Matcher, "xx", "yyyy", &[]);
                assert_matcher_case!($Matcher, "xx", "xxxxx", &[0, 1, 2, 3]);
                assert_matcher_case!($Matcher, "xx", "xxyxx", &[0, 3]);
                assert_matcher_case!($Matcher, "xx", "xyxyx", &[]);
            }

            #[test]
            fn two_char_pattern() {
                assert_matcher_case!($Matcher, "xy", "", &[]);
                assert_matcher_case!($Matcher, "xy", "x", &[]);
                assert_matcher_case!($Matcher, "xy", "y", &[]);
                assert_matcher_case!($Matcher, "xy", "xx", &[]);
                assert_matcher_case!($Matcher, "xy", "xy", &[0]);
                assert_matcher_case!($Matcher, "xy", "yx", &[]);
                assert_matcher_case!($Matcher, "xy", "yy", &[]);
                assert_matcher_case!($Matcher, "xy", "xxx", &[]);
                assert_matcher_case!($Matcher, "xy", "xxy", &[1]);
                assert_matcher_case!($Matcher, "xy", "xyx", &[0]);
                assert_matcher_case!($Matcher, "xy", "xyy", &[0]);
                assert_matcher_case!($Matcher, "xy", "yxx", &[]);
                assert_matcher_case!($Matcher, "xy", "yxy", &[1]);
                assert_matcher_case!($Matcher, "xy", "yyx", &[]);
                assert_matcher_case!($Matcher, "xy", "yyy", &[]);
                assert_matcher_case!($Matcher, "xy", "xxxx", &[]);
                assert_matcher_case!($Matcher, "xy", "xxxy", &[2]);
                assert_matcher_case!($Matcher, "xy", "xxyx", &[1]);
                assert_matcher_case!($Matcher, "xy", "xxyy", &[1]);
                assert_matcher_case!($Matcher, "xy", "xyxx", &[0]);
                assert_matcher_case!($Matcher, "xy", "xyxy", &[0, 2]);
                assert_matcher_case!($Matcher, "xy", "xyyx", &[0]);
                assert_matcher_case!($Matcher, "xy", "xyyy", &[0]);
                assert_matcher_case!($Matcher, "xy", "yxxx", &[]);
                assert_matcher_case!($Matcher, "xy", "yxxy", &[2]);
                assert_matcher_case!($Matcher, "xy", "yxyx", &[1]);
                assert_matcher_case!($Matcher, "xy", "yxyy", &[1]);
                assert_matcher_case!($Matcher, "xy", "yyxx", &[]);
                assert_matcher_case!($Matcher, "xy", "yyxy", &[2]);
                assert_matcher_case!($Matcher, "xy", "yyyx", &[]);
                assert_matcher_case!($Matcher, "xy", "yyyy", &[]);
                assert_matcher_case!($Matcher, "xy", "xz", &[]);
                assert_matcher_case!($Matcher, "xy", "xyz", &[0]);
                assert_matcher_case!($Matcher, "xy", "zxy", &[1]);
            }

            #[test]
            fn control_chars_and_unicode() {
                // ASCII control chars, low non-ASCII chars, some tricky unicode chars, and high unicode chars
                let control_chars_str = {
                    let mut control_chars_str = String::new();
                    for i in 0x00..0x20 {
                        control_chars_str.push(char::from_u32(i).unwrap());
                    }
                    control_chars_str.push('\x20');
                    control_chars_str.push('\x7f');
                    control_chars_str.push('\u{80}');
                    control_chars_str.push('\u{81}');
                    control_chars_str.push('\u{ff}');
                    control_chars_str.push('\u{100}');
                    control_chars_str.push('\u{d7ff}');
                    control_chars_str.push('\u{e000}');
                    control_chars_str.push('\u{feff}');
                    control_chars_str.push('\u{fffd}');
                    control_chars_str.push('\u{fffe}');
                    control_chars_str.push('\u{ffff}');
                    control_chars_str.push('\u{10000}');
                    control_chars_str.push('\u{10fffe}');
                    control_chars_str.push('\u{10ffff}');
                    control_chars_str
                };

                for (i, ch) in control_chars_str.chars().enumerate() {
                    assert_matcher_case!($Matcher, ch.to_string().as_str(), control_chars_str, &[i]);
                }
            }
        }
    }
}