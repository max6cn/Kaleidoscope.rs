use ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FloatValue, PointerValue};
use inkwell::FloatPredicate;
use std::collections::HashMap;
use {Parser, Token};

//trait AnyValue {}
pub struct LLVM {
    context: Context,
    build: Builder,
    module: Module,
    named_values: HashMap<String, PointerValue>,
}

pub trait CodeGen {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str>;
}
impl CodeGen for ExprAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        match self {
            ExprAst::BinaryExpr(bexp) => bexp.codegen(llvm),
            ExprAst::CallExpr(cexp) => cexp.codegen(llvm),
            ExprAst::NumberExpr(nexp) => nexp.codegen(llvm),
            ExprAst::VariableExpr(vexp) => vexp.codegen(llvm),
        }
    }
}
impl CodeGen for VariableExprAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        if let Some(addr) = llvm.named_values.get(self.name.clone().as_str()) {
            let fvalue = llvm
                .build
                .build_load(*addr, self.name.as_ref())
                .into_float_value();
            Ok(fvalue)
        } else {
            Err("Unknown variable name")
        }
    }
}
impl CodeGen for NumberExprAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        let fv = llvm.context.f64_type().const_float(self.val);
        Ok(fv)
    }
}
impl CodeGen for BinaryExprAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        let l = self.lhs.codegen(llvm)?;
        let r = self.lhs.codegen(llvm)?;
        match self.op {
            '+' => {
                let f = llvm.build.build_float_add(l, r, "addtmp0");
                Ok(f)
            }
            '-' => {
                let f = llvm.build.build_float_add(l, r, "addtmp0");
                Ok(f)
            }
            '*' => {
                let f = llvm.build.build_float_add(l, r, "addtmp0");
                Ok(f)
            }
            //            '<' => {
            //                let fcmp = llvm
            //                    .build
            //                    .build_float_compare(FloatPredicate::ULT, l, r, "cmptmp0");
            //                Ok(llvm.build.build_unsigned_int_to_float(
            //                    fcmp,
            //                    llvm.context.f64_type(),
            //                    "tmp0bool",
            //                ))
            //            }
            _ => Err("Undefined binary operator."),
        }
    }
}
impl CodeGen for CallExprAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        Err("unimplemented!")
    }
}

impl CodeGen for FunctionAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        Err("unimplemented!")
    }
}
impl CodeGen for PrototypeAst {
    fn codegen(&self, llvm: &LLVM) -> Result<FloatValue, &'static str> {
        Err("unimplemented!")
    }
}
#[cfg(test)]
mod test {
    use ast::*;
    use codegen::CodeGen;
    use codegen::LLVM;
    use inkwell::context::Context;
    use std::sync::mpsc::channel;
    use std::thread;
    use Lexer;
    use {Parser, Token};

    #[test]
    pub fn compile() {
        let prog3 = r"
         def foo(x)
             x + 1
         ";
        debug!("input :\n{}", prog3);
        let expected = "ast : Some(FAst(FunctionAst { proto: PrototypeAst { name: \"foo\", args: [\"x\"] }, body: BinaryExpr(BinaryExprAst { op: \'+\', lhs: VariableExpr(VariableExprAst { name: \"x\" }), rhs: NumberExpr(NumberExprAst { val: 1.0 }) }) }))";
        let (tx_char_stream, rx_char_stream) = channel();
        let (tx_token_stream, rx_token_stream) = channel();
        let lexer_guard = thread::spawn(move || {
            for c in prog3.chars() {
                let _ = tx_char_stream.send(c).expect("Send Error");
            }
            let _ = tx_char_stream.send('$').expect("Send Error");
            let mut lexer = Lexer::new(rx_char_stream, tx_token_stream);
            lexer.get_token();
        });
        let parser_guard = thread::spawn(move || {
            let mut parser = Parser::new(rx_token_stream);
            let ast = *parser.parse_input().unwrap();
            let res = format!("ast : {:?}", ast);
            assert_eq!(expected, res);
            let context = Context::create();
            let module = context.create_module("repl");
            let builder = context.create_builder();
            let mut llvm = LLVM {
                context: context,
                build: builder,
                module: module,
                named_values: Default::default(),
            };
            match ast {
                Ast::FAst(f) => {
                    let code = f.codegen(&llvm);
                    println!("{:?}", code);
                }
                _ => {}
            };
        });

        let _ = lexer_guard.join();
        let _ = parser_guard.join();
    }
}
