use crate::parser::Parser;

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl BinaryOperator {
    pub fn precedence(&self) -> i32 {
        match self {
            &BinaryOperator::Divide | &BinaryOperator::Multiply | &BinaryOperator::Mod => 10,
            &BinaryOperator::Add | &BinaryOperator::Subtract => 9,
            &BinaryOperator::LessThan | &BinaryOperator::LessThanEqual | &BinaryOperator::GreaterThan | &BinaryOperator::GreaterThanEqual => 8,
            &BinaryOperator::Equal | &BinaryOperator::NotEqual => 7,
            &BinaryOperator::And => 6,
            &BinaryOperator::Or => 5,
        }
    }
}

#[derive(Debug)]

pub enum ExprNode {
    Integer(i32),
    Unary{unary_op: UnaryOperator, expr: Box<ExprNode>},
    Binary{binary_op: BinaryOperator, lhs: Box<ExprNode>, rhs: Box<ExprNode>},
}

impl ExprNode {
    pub fn parse(parser: &mut Parser) -> Result<Box<ExprNode>, String> {
        parser.parse_expr(0)
    }
}

#[derive(Debug)]
pub enum FactorNode {
    Integer(i32),
    Unary{unary_op: UnaryOperator, expr: Box<ExprNode>},
    Expr(Box<ExprNode>),
}

impl FactorNode {
    pub fn parse(parser: &mut Parser) -> Result<FactorNode, String> {
        parser.parse_factor()
    }
}

#[derive(Debug)]
pub enum StatementNode {
    Return(ExprNode)
}

impl StatementNode {
    pub fn parse(parser: &mut Parser) -> Result<StatementNode, String> {
        parser.parse_return_statement()
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    identifier: String,
    body: StatementNode,
}

impl FunctionDefinition {
    pub fn parse(parser: &mut Parser) -> Result<FunctionDefinition, String> {
        parser.parse_function_definition()
    }
}

impl FunctionDefinition {
    pub fn new(identifier: String, body: StatementNode) -> Self {
        FunctionDefinition { identifier, body }
    }
}

pub struct ProgramNode {
    function_main: FunctionDefinition,
}