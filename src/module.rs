pub struct Module {
    name: String,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
