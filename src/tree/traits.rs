use super::{functions::Function, types::Type, variables::Variable};

#[derive(Debug)]
pub struct Trait {
    pub public: bool,
    pub name: String,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub trait_type: Type,
    pub class_type: Type,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}
