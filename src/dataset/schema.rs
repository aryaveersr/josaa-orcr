pub struct Entry {
    pub institute: String,
    pub quota: String,
    pub seat_type: String,
    pub gender: String,
    pub or: u32,
    pub cr: u32,
}

pub struct InstituteKind {
    pub kind: String,
    pub institutes: Vec<String>,
}
