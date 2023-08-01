use super::{types::Type, variables::Variable};

#[derive(Debug)]
pub struct Function {
    pub public: bool,
    pub name: String,
    pub type_parameters: Vec<Variable>,
    pub value_parameters: Vec<Variable>,
    pub this_type: Option<Type>,
    pub return_type: Type,
    // TODO: tree
}
