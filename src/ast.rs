
#[derive(Debug)]

pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
}

impl BinaryOperator {
    pub fn precedence(&self) -> i32 {
        match self {
            &BinaryOperator::Multiply => 10,
            &BinaryOperator::Divide => 10,
            &BinaryOperator::Mod => 10,
            &BinaryOperator::Add => 9,
            &BinaryOperator::Subtract => 9, 
        }
    }
}

#[derive(Debug)]

pub enum ExprNode {
    Integer(i32),
    Unary{unary_op: UnaryOperator, expr: Box<ExprNode>},
    Binary{lhs: Box<ExprNode>, rhs: Box<ExprNode>, binary_op: BinaryOperator},
}

#[derive(Debug)]
pub enum FactorNode {
    Integer(i32),
    Unary{unary_op: UnaryOperator, expr: Box<ExprNode>},
    Expr(Box<ExprNode>),
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