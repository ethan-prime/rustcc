pub enum StatementNode {
    Return(i32)
}

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