// ast
// ExprAst -> NumberExprAst | VariableExprAst | BinaryExprAst
//            | CallExprAst | PrototypeAst | FuctionAst
// FuntionAst -> PrototypeAst ExprAst
// PrototypeAst -> name args
// NumberExprAst -> Val
// VariableExprAst -> name
// BinaryExprAst -> lhs op rhs
// struct ExprAst ;
#[derive(Debug)]
pub enum ExprAst {
    NumberExprAst,
    VariableExprAst,
    BinaryExprAst,
    CallExprAst,
}
// trait ExprAst {
//     fn codegen(&self,&mut parser) -> i32 {
//         0
//     }
// }

#[derive(Debug)]
pub struct NumberExprAst {
    pub val: f64,
}
#[derive(Debug)]
pub struct VariableExprAst {
    pub name: String,
}
#[derive(Debug)]
pub struct BinaryExprAst<'a> {
    pub op: char,
    pub lhs: &'a ExprAst,
    pub rhs: &'a ExprAst,
}

#[derive(Debug)]
pub struct CallExprAst<'a> {
    pub callee: String,
    pub args: Vec<&'a ExprAst>,
}

#[derive(Debug)]
pub struct PrototypeAst {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct FunctionAst<'a> {
    pub proto: PrototypeAst,
    pub body: &'a ExprAst,
}
