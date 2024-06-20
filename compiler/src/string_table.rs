use std::collections::HashMap;

pub struct StringTable {
    string: String,
    addresses: HashMap<String, StringTableAddress>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringTableAddress {
    pub start: usize,
    pub end: usize,
}

#[allow(dead_code)]
impl StringTable {
    pub fn new() -> Self {
        StringTable {
            string: String::new(),
            addresses: HashMap::new(),
        }
    }

    pub fn put(&mut self, string: &str) -> StringTableAddress {
        self.addresses.get(string).map(|address| *address).unwrap_or_else(|| {
            let address = if let Some(start) = self.string.find(string) {
                let end = start + string.len();
                StringTableAddress { start, end }
            } else {
                let start = self.string.len();
                self.string.push_str(string);
                let end = self.string.len();
                StringTableAddress { start, end }
            };
            self.addresses.insert(String::from(string), address);
            address
        })
    }

    pub fn get_string(&self) -> &str {
        &self.string
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }
}

#[test]
pub fn test_string_table() {
    let mut string_table = StringTable::new();

    let address = string_table.put("a river in space");
    assert_eq!(address.start, 0);
    assert_eq!(address.end, 16);

    let address = string_table.put("a river in time");
    assert_eq!(address.start, 16);
    assert_eq!(address.end, 31);

    let address = string_table.put("a river in space");
    assert_eq!(address.start, 0);
    assert_eq!(address.end, 16);

    let address = string_table.put("a river");
    assert_eq!(address.start, 0);
    assert_eq!(address.end, 7);
}
