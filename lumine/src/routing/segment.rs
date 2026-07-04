#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
    Static(&'static str),
    Param(&'static str),
    Wildcard,
}

impl From<&'static str> for Segment {
    fn from(s: &'static str) -> Self {
        if s.starts_with(':') {
            Segment::Param(s.trim_start_matches(':'))
        } else if s == "*" {
            Segment::Wildcard
        } else {
            Segment::Static(s)
        }
    }
}
