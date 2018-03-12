//! Crate `hcl` is a Rust library for working with [HCL](https://github.com/hashicorp/hcl).

#![deny(missing_debug_implementations, warnings)]

#[cfg_attr(test, macro_use)] extern crate pest;
#[macro_use] extern crate pest_derive;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("hcl.pest");

/// A parser for HCL data contained in a string slice.
#[derive(Debug, Parser)]
#[grammar = "hcl.pest"]
pub struct HclParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_true_lit() {
        parses_to! {
            parser: HclParser,
            input: "true",
            rule: Rule::boolean,
            tokens: [
                boolean(0, 4, [true_lit(0, 4)])
            ]
        }
    }

    #[test]
    fn parse_false_lit() {
        parses_to! {
            parser: HclParser,
            input: "false",
            rule: Rule::boolean,
            tokens: [
                boolean(0, 5, [false_lit(0, 5)])
            ]
        }
    }

    #[test]
    fn parse_digit_0() {
        parses_to! {
            parser: HclParser,
            input: "0",
            rule: Rule::int,
            tokens: [
                int(0, 1)
            ]
        }
    }

    #[test]
    fn parse_digit_01() {
        parses_to! {
            parser: HclParser,
            input: "01",
            rule: Rule::int,
            tokens: [
                int(0, 2)
            ]
        }
    }

    #[test]
    fn parse_digit_1000() {
        parses_to! {
            parser: HclParser,
            input: "1000",
            rule: Rule::int,
            tokens: [
                int(0, 4)
            ]
        }
    }

    #[test]
    fn parse_float_zero_point() {
        parses_to! {
            parser: HclParser,
            input: "0.",
            rule: Rule::float,
            tokens: [
                float(0, 2, [int(0, 1)])
            ]
        }
    }

    #[test]
    fn parse_float_one_exp() {
        parses_to! {
            parser: HclParser,
            input: "1e10",
            rule: Rule::float,
            tokens: [
                float(0, 4, [
                    int(0, 1),
                    exp(1, 4, [
                        int(2, 4)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_pound_comment() {
        parses_to! {
            parser: HclParser,
            input: "# foo",
            rule: Rule::comment,
            tokens: [
                comment(0, 5)
            ]
        }
    }

    #[test]
    fn parse_double_slash_comment() {
        parses_to! {
            parser: HclParser,
            input: "// foo",
            rule: Rule::comment,
            tokens: [
                comment(0, 6)
            ]
        }
    }

    #[test]
    fn parse_block_comment() {
        parses_to! {
            parser: HclParser,
            input: "/* foo\nbar */",
            rule: Rule::comment,
            tokens: [
                comment(0, 13)
            ]
        }
    }

    #[test]
    fn parse_string() {
        parses_to! {
            parser: HclParser,
            input: "\"foo\"",
            rule: Rule::string,
            tokens: [
                string(0, 5)
            ]
        }
    }

    #[test]
    fn parse_list_empty() {
        parses_to! {
            parser: HclParser,
            input: "[]",
            rule: Rule::list,
            tokens: [
                list(0, 2)
            ]
        }
    }

    #[test]
    fn parse_list_single_boolean() {
        parses_to! {
            parser: HclParser,
            input: "[true]",
            rule: Rule::list,
            tokens: [
                list(0, 6, [
                    boolean(1, 5, [
                        true_lit(1, 5)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_list_single_number() {
        parses_to! {
            parser: HclParser,
            input: "[1234]",
            rule: Rule::list,
            tokens: [
                list(0, 6, [
                    number(1, 5, [
                        int(1, 5)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_list_single_string() {
        parses_to! {
            parser: HclParser,
            input: "[\"foo\"]",
            rule: Rule::list,
            tokens: [
                list(0, 7, [
                    string(1, 6)
                ])
            ]
        }
    }

    #[test]
    fn parse_list_trailing_comma() {
        parses_to! {
            parser: HclParser,
            input: "[true,]",
            rule: Rule::list,
            tokens: [
                list(0, 7, [
                    boolean(1, 5, [
                        true_lit(1, 5)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_list_multiple() {
        parses_to! {
            parser: HclParser,
            input: "[true,true]",
            rule: Rule::list,
            tokens: [
                list(0, 11, [
                    boolean(1, 5, [
                        true_lit(1, 5)
                    ]),
                    boolean(6, 10, [
                        true_lit(6, 10)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_list_multiple_space_after_comma() {
        parses_to! {
            parser: HclParser,
            input: "[true, true]",
            rule: Rule::list,
            tokens: [
                list(0, 12, [
                    boolean(1, 5, [
                        true_lit(1, 5)
                    ]),
                    boolean(7, 11, [
                        true_lit(7, 11)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_list_multiple_trailing_comma() {
        parses_to! {
            parser: HclParser,
            input: "[true, true, ]",
            rule: Rule::list,
            tokens: [
                list(0, 14, [
                    boolean(1, 5, [
                        true_lit(1, 5)
                    ]),
                    boolean(7, 11, [
                        true_lit(7, 11)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_assignment() {
        parses_to! {
            parser: HclParser,
            input: "foo = true",
            rule: Rule::assignment,
            tokens: [
                assignment(0, 10, [
                    ident(0, 3),
                    boolean(6, 10, [
                        true_lit(6, 10)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_object() {
        parses_to! {
            parser: HclParser,
            input: "foo { bar = \"baz\" }",
            rule: Rule::object,
            tokens: [
                object(0, 19, [
                    ident(0, 3),
                    assignment(6, 17, [
                        ident(6, 9),
                        string(12, 17)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn parse_nested_object() {
        parses_to! {
            parser: HclParser,
            input: "foo \"bar\" { baz = \"qux\" }",
            rule: Rule::object,
            tokens: [
                object(0, 25, [
                    ident(0, 3),
                    ident(5, 8),
                    assignment(12, 23, [
                        ident(12, 15),
                        string(18, 23)
                    ])
                ])
            ]
        }
    }
}
