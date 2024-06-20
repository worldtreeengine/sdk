use std::rc::Rc;
use crate::yaml::Mark;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribution {
    pub source: Rc<String>,
    pub path: String,
    pub start_mark: Mark,
    pub end_mark: Mark,
}

impl Attribution {
    pub fn new(source: &str, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: Rc::new(String::from(source)),
            path: String::from(""),
            start_mark,
            end_mark,
        }
    }

    pub fn new_at_key(source: &str, key: &str, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: Rc::new(String::from(source)),
            path: format!(".{}", key),
            start_mark,
            end_mark,
        }
    }

    pub fn new_at_index(source: &str, index: usize, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: Rc::new(String::from(source)),
            path: format!("[{}]", index),
            start_mark,
            end_mark,
        }
    }

    pub fn at_key(&self, key: &str, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: self.source.clone(),
            path: format!("{}.{}", self.path, key),
            start_mark,
            end_mark,
        }
    }

    pub fn at_index(&self, index: usize, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: self.source.clone(),
            path: format!("{}[{}]", self.path, index),
            start_mark,
            end_mark,
        }
    }

    pub fn at_marks(&self, start_mark: Mark, end_mark: Mark) -> Attribution {
        Attribution {
            source: self.source.clone(),
            path: self.path.clone(),
            start_mark: Mark {
                line: self.start_mark.line + start_mark.line,
                column: if start_mark.line == 0 { self.start_mark.column + start_mark.column } else { start_mark.column },
            },
            end_mark: Mark {
                line: self.start_mark.line + end_mark.line,
                column: if end_mark.line == 0 { self.start_mark.column + end_mark.column } else { end_mark.column },
            },
        }
    }

    pub fn at_mark(&self, mark: Mark) -> Attribution {
        self.at_marks(mark, mark)
    }
}
