use super::{block::Block, expr::Expr};

#[derive(Debug, Clone)]
pub struct IfStruct {
    pub branches: Vec<Branch>,
    pub else_branch: Option<Box<Branch>>,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub condition: Box<Expr>,
    pub block: Block,
}
