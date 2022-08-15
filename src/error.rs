use std::fmt;

#[derive(Clone)]
pub struct TransitionError(pub String, pub Option<char>);

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            Some(c) => {
                if self.0 == "" {
                    write!(f, "invalid initial symbol `{}`", c)
                } else {
                    write!(f, "no transition `{}` -> `{}`", self.0, c)
                }
            }
            None => write!(f, "`{}` must transition", self.0),
        }
    }
}

impl fmt::Debug for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
