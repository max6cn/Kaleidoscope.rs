use ast::*;

trait AnyValue {}
trait CodeGen {
    fn codegen(&self) -> Box<AnyValue>;
}
impl CodeGen for VariableExprAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}
impl CodeGen for NumberExprAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}
impl CodeGen for BinaryExprAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}
impl CodeGen for CallExprAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}

impl CodeGen for FunctionAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}
impl CodeGen for PrototypeAst {
    fn codegen(&self) -> Box<AnyValue> {
        unimplemented!()
    }
}
