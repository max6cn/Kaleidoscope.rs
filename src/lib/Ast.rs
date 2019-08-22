// ast
// ExprAst -> NumberExprAst | VariableExprAst | BinaryExprAst
//            | CallExprAst | PrototypeAst | FuctionAst
// FuntionAst -> PrototypeAst ExprAst
// PrototypeAst -> Name Args
// NumberExprAst -> Val
// VariableExprAst -> name
// BinaryExprAst -> LHS op RHS
// struct ExprAst ;
#[derive(Debug)]
pub enum ExprAst {
    NumberExprAst,
    VariableExprAst,
    BinaryExprAst,
    CallExprAst,
}
// trait ExprAst {
//     fn codegen(&self,&mut Parser) -> i32 {
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
    pub Op: char,
    pub LHS: &'a ExprAst,
    pub RHS: &'a ExprAst,
}

#[derive(Debug)]
pub struct CallExprAst<'a> {
    pub Callee: String,
    pub Args: Vec<&'a ExprAst>,
}

#[derive(Debug)]
pub struct PrototypeAst {
    pub Name: String,
    pub Args: Vec<String>,
}

#[derive(Debug)]
pub struct FunctionAst<'a> {
    pub Proto: PrototypeAst,
    pub Body: &'a ExprAst,
}
