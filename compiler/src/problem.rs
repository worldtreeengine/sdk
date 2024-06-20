use crate::Attribution;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Warning,
    Fatal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub level: Level,
    pub message: &'static str,
    pub attribution: Attribution,
    pub context: Option<Context>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    pub message: &'static str,
    pub attribution: Attribution,
}

impl Problem {
    pub fn warning(message: &'static str, attribution: &Attribution) -> Problem {
        Problem {
            level: Level::Warning,
            message,
            attribution: attribution.clone(),
            context: None,
        }
    }

    pub fn fatal(message: &'static str, attribution: &Attribution) -> Problem {
        Problem {
            level: Level::Fatal,
            message,
            attribution: attribution.clone(),
            context: None,
        }
    }

    pub fn with_context(self, message: &'static str, attribution: &Attribution) -> Problem {
        Problem {
            level: self.level,
            message: self.message,
            attribution: self.attribution,
            context: Some(Context {
                message,
                attribution: attribution.clone(),
            }),
        }
    }
}
