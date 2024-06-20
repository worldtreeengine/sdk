use regex::Regex;
use libyaml_safer::Parser;
use std::io::{BufReader, Read};
use std::str::FromStr;
use crate::yaml::context::ParsingContext;
use crate::yaml::document::Document;
use crate::yaml::path::{Path};
use crate::yaml::value::Value;
use crate::yaml::result::Result;

pub trait Schema {
    fn resolve(&self, path: &Path, value: Value) -> (String, Value);

    fn parse_string(&self, input: &str) -> Result<Vec<Document>> where Self: Sized {
        let mut parser = Parser::new();
        let mut bytes = input.as_bytes();
        parser.set_input_string(&mut bytes);
        return ParsingContext::new(&mut parser, self).parse();
    }

    fn parse<R: Read>(&self, input: R) -> Result<Vec<Document>> where Self: Sized {
        let mut reader = BufReader::new(input);
        let mut parser = Parser::new();
        parser.set_input(&mut reader);
        return ParsingContext::new(&mut parser, self).parse();
    }
}

pub struct FailsafeSchema {}

impl Schema for FailsafeSchema {
    fn resolve(&self, _: &Path, value: Value) -> (String, Value) {
        match value {
            Value::Scalar(scalar) => (String::from("tag:yaml.org,2002:str"), Value::Scalar(scalar)),
            Value::Sequence(sequence) => (String::from("tag:yaml.org,2002:seq"), Value::Sequence(sequence)),
            Value::Mapping(mapping) => (String::from("tag:yaml.org,2002:map"), Value::Mapping(mapping)),
        }
    }
}

impl FailsafeSchema {
    pub fn parse_string(input: &str) -> Result<Vec<Document>> {
        FailsafeSchema {}.parse_string(input)
    }

    pub fn parse<R: Read>(input: R) -> Result<Vec<Document>> {
        FailsafeSchema {}.parse(input)
    }
}

pub struct JsonSchema {
    integer_regex: Regex,
    float_regex: Regex,
}

// impl JsonSchema {
//     pub fn new() -> JsonSchema {
//         JsonSchema {
//             integer_regex: Regex::new(r"-?(0|[1-9][0-9]*)").unwrap(),
//             float_regex: Regex::new(r"-?(0|[1-9][0-9]*)(\.[0-9]*)?([eE][-+]?[0-9]+)?").unwrap(),
//         }
//     }
// }

impl Schema for JsonSchema {
    fn resolve(&self, _: &Path, value: Value) -> (String, Value) {
        match value {
            Value::Scalar(scalar) => {
                if scalar == "null" {
                    (String::from("tag:yaml.org,2002:null"), Value::Scalar(scalar))
                } else if scalar == "true" || scalar == "false" {
                    (String::from("tag:yaml.org,2002:bool"), Value::Scalar(scalar))
                } else if self.integer_regex.is_match(&scalar) {
                    (String::from("tag:yaml.org,2002:int"), Value::Scalar(scalar))
                } else if self.float_regex.is_match(&scalar) {
                    (String::from("tag:yaml.org,2002:float"), Value::Scalar(format!("{:e}", f64::from_str(&scalar).unwrap())))
                } else {
                    (String::from("tag:yaml.org,2002:str"), Value::Scalar(scalar))
                }
            },
            Value::Sequence(sequence) => (String::from("tag:yaml.org,2002:seq"), Value::Sequence(sequence)),
            Value::Mapping(mapping) => (String::from("tag:yaml.org,2002:map"), Value::Mapping(mapping)),
        }
    }
}

pub struct CoreSchema {
    decimal_integer_regex: Regex,
    octal_integer_regex: Regex,
    hexadecimal_integer_regex: Regex,
    finite_float_regex: Regex,
}

// impl CoreSchema {
//     pub fn new() -> CoreSchema {
//         CoreSchema {
//             decimal_integer_regex: Regex::new(r"[-+]?[0-9]+").unwrap(),
//             octal_integer_regex: Regex::new(r"0o[0-7]+").unwrap(),
//             hexadecimal_integer_regex: Regex::new(r"0x[0-9a-fA-F]+").unwrap(),
//             finite_float_regex: Regex::new(r"[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?").unwrap(),
//         }
//     }
// }

impl Schema for CoreSchema {
    fn resolve(&self, _: &Path, value: Value) -> (String, Value) {
        match value {
            Value::Scalar(scalar) => {
                match scalar.as_str() {
                    "null" | "NULL" | "Null" | "~" | "" => (String::from("tag:yaml.org,2002:null"), Value::Scalar(String::from("null"))),
                    "true" | "TRUE" | "True" | "false" | "FALSE" | "False" => (String::from("tag:yaml.org,2002:bool"), Value::Scalar(String::from(scalar.to_ascii_lowercase()))),
                    ".nan" | ".NaN" | ".NAN" => (String::from("tag:yaml.org,2002:float"), Value::Scalar(String::from(".nan"))),
                    ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => (String::from("tag:yaml.org,2002:float"), Value::Scalar(String::from(".inf"))),
                    "-.inf" | "-.Inf" | "-.INF" => (String::from("tag:yaml.org,2002:float"), Value::Scalar(String::from("-.inf"))),
                    _ => {
                        if self.decimal_integer_regex.is_match(&scalar) {
                            (String::from("tag:yaml.org,2002:int"), Value::Scalar(format!("{}", i64::from_str(&scalar).unwrap())))
                        } else if self.octal_integer_regex.is_match(&scalar) {
                            (String::from("tag:yaml.org,2002:int"), Value::Scalar(format!("{}", i64::from_str_radix(&scalar[2..], 8).unwrap())))
                        } else if self.hexadecimal_integer_regex.is_match(&scalar) {
                            (String::from("tag:yaml.org,2002:int"), Value::Scalar(format!("{}", i64::from_str_radix(&scalar[2..], 16).unwrap())))
                        } else if self.finite_float_regex.is_match(&scalar) {
                            (String::from("tag:yaml.org,2002:int"), Value::Scalar(format!("{:e}", f64::from_str(&scalar).unwrap())))
                        } else {
                            (String::from("tag:yaml.org,2002:str"), Value::Scalar(scalar))
                        }
                    },
                }
            },
            Value::Sequence(sequence) => (String::from("tag:yaml.org,2002:seq"), Value::Sequence(sequence)),
            Value::Mapping(mapping) => (String::from("tag:yaml.org,2002:map"), Value::Mapping(mapping)),
        }
    }
}
