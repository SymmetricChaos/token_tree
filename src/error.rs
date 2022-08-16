use std::fmt;
 
#[derive(Clone)]
pub struct TransitionError {
    pub consumed_str: String, 
    pub character: Option<char>,
    pub start: usize,
    pub end: usize,
}
 
impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TransitionError{consumed_str, character, start, end} = self;
        if start == end {
            if let Some(ch) = character {
                if consumed_str.is_empty() {
                    write!(f, "no input path starts with `{ch}`, index: {start}")
                } else {
                    write!(f, "no transition `{consumed_str}` -> `{ch}`, index: {start}")
                }
            } else {
                write!(f, "`{consumed_str}` must transition, index: {start}")
            }
        } else {
            if let Some(ch) = character {
                if consumed_str.is_empty() {
                    write!(f, "no input path starts with `{ch}`, index: {start}")
                } else {
                    write!(f, "no transition `{consumed_str}` -> `{ch}`, range: {start}..{end}")
                }
            } else {
                write!(f, "`{consumed_str}` must transition, range: {start}..{end}")
            }
        }
    }
}
 
impl fmt::Debug for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}