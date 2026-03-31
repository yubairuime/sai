#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Function {
        params: Vec<Type>,
        return_ty: Box<Type>,
    },
    Error
}
