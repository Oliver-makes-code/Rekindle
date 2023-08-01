use super::{functions::Function, traits::TraitImpl, variables::Variable};

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub children: Vec<Variable>,
    pub global_variables: Vec<Variable>,
    pub local_variables: Vec<Variable>,
    pub functions: Vec<Function>,
    pub trait_impls: Vec<TraitImpl>,
}

pub struct Object {
    pub name: String,
    pub variables: Vec<Variable>,
}
