// ast
// ExprAst -> NumberExprAst | VariableExprAst | BinaryExprAst
//            | CallExprAst | PrototypeAst | FuctionAst
// FuntionAst -> PrototypeAst ExprAst
// PrototypeAst -> name args
// NumberExprAst -> Val
// VariableExprAst -> name
// BinaryExprAst -> lhs op rhs
// struct ExprAst ;

// trait ExprAst {
//     fn codegen(&self,&mut parser) -> i32 {
//         0
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct NumberExprAst {
    pub val: f64,
}
#[derive(Debug, Clone)]
pub struct VariableExprAst {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct BinaryExprAst {
    pub op: char,
    pub lhs: Box<ExprAst>,
    pub rhs: Box<ExprAst>,
}

#[derive(Debug, Clone)]
pub struct CallExprAst {
    pub callee: String,
    pub args: Vec<ExprAst>,
}
#[derive(Debug, Clone)]
pub enum ExprAst {
    NumberExpr(NumberExprAst),
    VariableExpr(VariableExprAst),
    BinaryExpr(BinaryExprAst),
    CallExpr(CallExprAst),
}
#[derive(Debug)]
pub struct PrototypeAst {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct FunctionAst {
    pub proto: PrototypeAst,
    pub body: Box<ExprAst>,
}
