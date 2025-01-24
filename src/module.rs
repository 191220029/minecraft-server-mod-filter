use std::hash::Hash;

#[derive(Debug)]
pub struct Module {
    pub(crate) name: String,
    pub(crate) raw: String,
    pub(crate) link: String,
    pub(crate) server_flag: ServerFlag,
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.link.eq(&other.link)
    }
}

impl Eq for Module {}

impl Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.link.hash(state)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ServerFlag {
    ServerNeeded,
    ServerInvalid,
    /// Mod not found on [search.mcmod.cn]
    ModNotFound(String),
    /// Server info not found on [mcmod.cn]
    ServerInfoNotFound(String),
}

impl Module {
    pub fn new(name: &str, raw: &str, link: &str) -> Self {
        Self {
            name: name.to_string(),
            raw: raw.to_string(),
            link: link.to_string(),
            server_flag: ServerFlag::ModNotFound("init".to_string()),
        }
    }
}
