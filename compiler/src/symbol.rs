use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use crate::{ElementTree, Problem};
use crate::element::NameElement;

pub struct SymbolList {
    symbols: Vec<String>,
}

#[allow(dead_code)]
impl SymbolList {
    pub fn new() -> Self {
        SymbolList {
            symbols: Vec::new()
        }
    }

    pub fn builder() -> SymbolListBuilder {
        SymbolListBuilder::new()
    }

    pub fn extract(element_tree: &ElementTree, problems: &mut Vec<Problem>) -> Self {

        let mut name_elements = Vec::new();

        for quality in &element_tree.qualities {
            if let Some(name) = &quality.name {
                name_elements.push(name);
            }

            if let Some(values) = &quality.values {
                for value in &values.elements {
                    if let Some(name) = &value.name {
                        name_elements.push(name);
                    }
                }
            }
        }

        for storylet in &element_tree.storylets {
            if let Some(name) = &storylet.name {
                name_elements.push(name);
            }
        }

        for location in &element_tree.locations {
            if let Some(name) = &location.name {
                name_elements.push(name);
            }

            if let Some(storylets) = &location.storylets {
                for storylet in &storylets.elements {
                    if let Some(name) = &storylet.name {
                        name_elements.push(name);
                    }
                }
            }
        }

        let mut map = HashMap::new();
        let mut symbols = Vec::new();

        for name in name_elements {
            let normalized_name = normalize(&name.name);
            if let Some(existing_attribution) = map.get(&normalized_name) {
                problems.push(Problem::fatal("All names must be unique, but this one isn't", &name.attribution)
                    .with_context("Already defined here", *existing_attribution));
            } else {
                symbols.push(normalized_name.clone());
                map.insert(normalized_name, &name.attribution);
            }
        }

        symbols.sort();
        symbols.reverse();

        Self { symbols }
    }

    pub fn push(&mut self, symbol: &str) {
        let normalized_symbol = normalize(symbol);

        if !self.symbols.contains(&normalized_symbol) {
            self.symbols.push(normalized_symbol);
            self.symbols.sort_unstable();
            self.symbols.reverse();
        }
    }

    pub fn contains(&self, symbol: &str) -> bool {
        let normalized_symbol = normalize(symbol);
        self.symbols.contains(&normalized_symbol)
    }

    pub fn require(&self, name: &NameElement, problems: &mut Vec<Problem>) -> String {
        let normalized_symbol = normalize(&name.name);
        if !self.symbols.contains(&normalized_symbol) {
            problems.push(Problem::fatal("Expected the name of an existing quality, location, or storylet", &name.attribution));
        }
        normalized_symbol
    }

    pub fn starts_with(&self, source: &str) -> Option<(usize, String)> {
        for symbol in &self.symbols {
            let mut source_chars = source.chars();
            let mut symbol_chars = symbol.chars();
            let mut whitespace = false;
            let mut source_length = 0;
            let mut matched = true;
            while let Some(source_char) = source_chars.next() {
                if source_char.is_whitespace() {
                    whitespace = true;
                    source_length += source_char.len_utf8();
                    continue;
                }

                let lowercase_source_char = source_char.to_ascii_lowercase();
                if let Some(symbol_char) = symbol_chars.next() {
                    if whitespace {
                        whitespace = false;
                        if symbol_char.is_whitespace() {
                            if let Some(symbol_char) = symbol_chars.next() {
                                if lowercase_source_char == symbol_char {
                                    source_length += source_char.len_utf8();
                                } else {
                                    break;
                                }
                            } else {
                                return Some((source_length, symbol.clone()));
                            }
                        } else {
                            matched = false;
                            break;
                        }
                    } else if lowercase_source_char == symbol_char {
                        source_length += source_char.len_utf8();
                    } else {
                        matched = false;
                        break;
                    }
                } else {
                    return Some((source_length, symbol.clone()));
                }
            }
            if let None = symbol_chars.next() {
                if matched {
                    return Some((source_length, symbol.clone()));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod symbol_list_tests {
    use super::*;

    #[test]
    fn it_returns_longest() {
        let symbols = SymbolList::builder().push("a river").push("a river in space").build();
        assert_eq!(symbols.starts_with("a riv"), None);
        assert_eq!(symbols.starts_with("a river"), Some((7, String::from("a river"))));
        assert_eq!(symbols.starts_with("a river in space"), Some((16, String::from("a river in space"))));
        assert_eq!(symbols.starts_with("a river in time"), Some((8, String::from("a river"))));
    }

    #[test]
    fn it_is_case_insensitive() {
        let symbols = SymbolList::builder().push("A river").build();
        assert_eq!(symbols.starts_with("a River"), Some((7, String::from("a river"))));
    }

    #[test]
    fn it_collapses_whitespace() {
        let symbols = SymbolList::builder().push("a   river\tin\n\n space   ").build();
        assert_eq!(symbols.starts_with("a\triver    in space"), Some((19, String::from("a river in space"))));
    }

    #[test]
    fn it_counts_utf8_bytes() {
        let symbols = SymbolList::builder().push("r\u{00e9}sum\u{00e9}").build();
        assert_eq!(symbols.starts_with("r\u{00e9}sum\u{00e9}"), Some((8, String::from("r\u{00e9}sum\u{00e9}"))));
    }
}

pub struct SymbolListBuilder {
    symbols: Vec<String>,
}

#[allow(dead_code)]
impl SymbolListBuilder {
    pub fn new() -> Self {
        SymbolListBuilder { symbols: Vec::new() }
    }

    pub fn push(mut self, symbol: &str) -> Self {
        let normalized_symbol = normalize(symbol);
        if !self.symbols.contains(&normalized_symbol) {
            self.symbols.push(normalized_symbol);
        }
        self
    }

    pub fn build(mut self) -> SymbolList {
        self.symbols.sort_unstable();
        self.symbols.reverse();

        SymbolList {
            symbols: self.symbols,
        }
    }
}

lazy_static! {
    static ref WHITESPACE_REGEX: Regex = Regex::new("\\s+").unwrap();
}

pub fn normalize(source: &str) -> String {
    WHITESPACE_REGEX.replace_all(source.trim(), " ").to_lowercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_symbol_list() {
        let symbol_list = SymbolList::builder().push("a river in time").push("a river").build();

        let result = symbol_list.starts_with("a river in time");
        assert_eq!(result, Some((15, "a river in time".to_string())));

        let result = symbol_list.starts_with("a river in space");
        assert_eq!(result, Some((8, "a river".to_string())));

        let result = symbol_list.starts_with("a   River in space");
        assert_eq!(result, Some((10, "a river".to_string())));

        let result = symbol_list.starts_with("a   River in  TIME okay?");
        assert_eq!(result, Some((19, "a river in time".to_string())));

        let result = symbol_list.starts_with("a ritual");
        assert_eq!(result, None);
    }
}
