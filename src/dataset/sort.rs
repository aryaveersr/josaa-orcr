use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Sort {
    OpeningAscending,
    OpeningDescending,
    ClosingAscending,
    ClosingDescending,
}

impl Sort {
    pub fn as_vec() -> Vec<Self> {
        vec![
            Sort::OpeningAscending,
            Sort::OpeningDescending,
            Sort::ClosingAscending,
            Sort::ClosingDescending,
        ]
    }
}

impl Default for Sort {
    fn default() -> Self {
        Self::ClosingAscending
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::OpeningAscending => write!(f, "Ascending (OR)"),
            Sort::OpeningDescending => write!(f, "Descending (OR)"),
            Sort::ClosingAscending => write!(f, "Ascending (CR)"),
            Sort::ClosingDescending => write!(f, "Descending (CR)"),
        }
    }
}
