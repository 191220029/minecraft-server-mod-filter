pub struct Module {
    pub(crate) name: String,
    pub(crate) raw: String,
    pub(crate) server_flag: ServerFlag,
}

#[derive(Debug, PartialEq)]
pub enum ServerFlag {
    ServerNeeded,
    ServerInvalid,
    /// Mod not found on [search.mcmod.cn]
    ModNotFound(String),
    /// Server info not found on [mcmod.cn]
    ServerInfoNotFound(String),
}

impl Module {
    pub fn new(name: &str, raw: &str) -> Self {
        Self {
            name: name.to_string(),
            raw: raw.to_string(),
            server_flag: ServerFlag::ModNotFound("init".to_string()),
        }
    }
}
