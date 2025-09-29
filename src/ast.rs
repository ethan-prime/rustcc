
#[derive(Debug)]

pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum ExprNode {
    Integer(i32),
    Unary{unary_op: UnaryOperator, expr: Box<ExprNode>}
}

#[derive(Debug)]
pub enum StatementNode {
    Return(ExprNode)
}

#[derive(Debug)]
pub struct FunctionDefinition {
    identifier: String,
    body: StatementNode,
}

impl FunctionDefinition {
    pub fn new(identifier: String, body: StatementNode) -> Self {
        FunctionDefinition { identifier, body }
    }
}

pub struct ProgramNode {
    function_main: FunctionDefinition,
}