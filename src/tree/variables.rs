use super::types::Type;

#[derive(Debug)]
pub struct Variable {
    pub public: bool,
    pub mutable: bool,
    pub global: bool,
    pub name: String,
    pub r#type: Type,
}
