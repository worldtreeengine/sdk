#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct Mark {
    pub line: u64,
    pub column: u64,
}

impl From<libyaml_safer::Mark> for Mark {
    fn from(value: libyaml_safer::Mark) -> Self {
        Self {
            line: value.line,
            column: value.column,
        }
    }
}
