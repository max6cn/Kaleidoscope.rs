use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::mpsc::Receiver;

//use super::token::{TokChar,TokNumber,TokIdentifier,TokDef,TokEof,TokExtern};
use ast::*;
use token::*;
#[derive(Debug)]
pub struct Parser {
    token_input: Receiver<Token>,
    current_token: Token,
    binop_precedence: HashMap<char, i8>,
}

impl<'a> Parser {
    pub fn new(token_input: Receiver<Token>) -> Parser {
        let mut binop_precedence = HashMap::new();
        // Install standard binary operators.
        // 1 is lowest precedence.
        binop_precedence.insert('<', 10);
        binop_precedence.insert('+', 20);
        binop_precedence.insert('-', 20);
        binop_precedence.insert('*', 40);
        // highest
        Parser {
            token_input,
            current_token: Token::TokChar(' '),
            binop_precedence,
        }
    }
    fn get_tok_precedence(&mut self) -> i8 {
        match self.current_token {
            Token::TokChar(ch) => {
                if self.binop_precedence.contains_key(&ch) {
                    self.binop_precedence[&ch]
                } else {
                    -1
                }
            }
            _ => -1,
        }
    }
    fn get_next_token(&mut self) -> Token {
        self.current_token = match self.token_input.recv() {
            Ok(tok) => {
                //debug!("Got token :{:?}",&tok);
                tok
            }
            Err(_) => Token::TokEof,
        };
        self.current_token.clone()
    }
    ///
    /// identifierexpr
    ///   ::= identifier
    ///   ::= identifier '(' expression* ')'
    ///
    fn parse_identifier_expr(&mut self) -> Option<ExprAst> {
        debug!("Parse Identifier: {:?} ", self.current_token);
        if let Token::TokIdentifier(id_name) = self.current_token.clone() {
            self.get_next_token();
            // simple variable ref
            if self.current_token != Token::TokChar('(') {
                let exp = VariableExprAst { name: id_name };
                return Some(ExprAst::VariableExpr(exp));
            }
            // call.
            self.get_next_token(); //eat (
            let mut args: Vec<ExprAst> = vec![];
            if self.current_token != Token::TokChar(')') {
                loop {
                    let e = self.parse_expression();
                    match e {
                        Some(arg) => {
                            args.push(*arg);
                        }
                        None => {
                            return None;
                        }
                    }

                    if self.current_token == Token::TokChar(')') {
                        break;
                    }
                    if self.current_token != Token::TokChar(',') {
                        debug!("Error, Expect ')' or ',' in argument list");
                        return None;
                    }
                    self.get_next_token();
                }
            }
            // eat the ')'
            self.get_next_token();
            let call_expr_ast = CallExprAst {
                callee: id_name,
                args,
            };
            return Some(ExprAst::CallExpr(call_expr_ast));
        }
        None
    }
    ///
    /// numberexpr ::= number
    ///
    fn parse_number_expr(&mut self) -> Option<ExprAst> {
        debug!("Parse Number exp : {:?} ", self.current_token);
        let result = NumberExprAst {
            val: match self.current_token.clone() {
                Token::TokNumber(v) => v,
                _ => {
                    debug!("Invalid numebr");
                    0.0
                }
            },
        };
        self.get_next_token(); // consume number
        Some(ExprAst::NumberExpr(result))
    }
    ///
    /// parenexpr ::= '(' expression ')'
    ///
    fn parse_paren_expr(&mut self) -> Option<ExprAst> {
        debug!("parse Paren Expr: {:?} ", self.current_token);
        self.get_next_token(); // eat (.
        if let Some(v) = self.parse_expression() {
            if self.current_token != Token::TokChar(')') {
                debug!("Error: Expected ')'");
                return None;
            }
            self.get_next_token(); // eat )
            return Some(*v);
        }
        None
    }
    ///
    /// primary
    ///   ::= identifierexpr
    ///   ::= numberexpr
    ///   ::= parenexpr
    ///
    fn parse_primary(&mut self) -> Option<ExprAst> {
        debug!("Parse Primary: {:?} ", self.current_token);
        match self.current_token.clone() {
            Token::TokIdentifier(_id) => self.parse_identifier_expr(),
            Token::TokNumber(_val) => self.parse_number_expr(),
            Token::TokChar('(') => self.parse_paren_expr(),
            _ => {
                debug!("unknow token when expecting an expression");
                None
            }
        }
    }
    ///
    /// binoprhs
    ///   ::= ('+' primary)*
    ///
    fn parse_bin_op_rhs(&mut self, expr_prec: i8, lhs: &ExprAst) -> Option<Box<ExprAst>> {
        let mut lhs_local = lhs.clone();
        loop {
            let tok_prec = self.get_tok_precedence();
            debug!(
                "---> Parse binOpRHS: lhs = {:? }, {:?}  ",
                lhs_local, self.current_token
            );
            debug!(
                "---> tok_precedence {:?}, expression prec {} ",
                tok_prec, expr_prec
            );

            // If this is a binop that binds at least as tightly as the current binop,
            // consume it, otherwise we are done.
            if tok_prec < expr_prec {
                let r = Box::new(lhs_local);
                debug!("parse_bin_op_rhs: Got {:?}", r);
                return Some(r);
            }
            // Okay, we know this is a binop.
            let bin_op = match self.current_token {
                Token::TokChar(op) => op,
                _ => unreachable!(),
            };
            self.get_next_token(); //eat binop
                                   // Parse the primary expression after the binary operator.
            match self.parse_primary() {
                Some(rhs) => {
                    // If BinOp binds less tightly with rhs than the operator after rhs, let
                    // the pending operator take rhs as its lhs.
                    debug!("------> {:?} : {} : {:?}", lhs_local, bin_op, rhs);
                    let next_prec = self.get_tok_precedence();
                    if tok_prec < next_prec {
                        let rhs = self.parse_bin_op_rhs(tok_prec + 1, &rhs);
                        rhs.as_ref()?;
                    }
                    // Merge LHS/RHS.
                    let nlhs = BinaryExprAst {
                        op: bin_op,
                        lhs: Box::new(lhs_local.clone()),
                        rhs: Box::new(rhs),
                    };
                    lhs_local = ExprAst::BinaryExpr(nlhs);
                    debug!("Got Expr: {:?}", &lhs_local);
                }
                None => {
                    debug!("Got Nothing in BinOP");
                    return None;
                }
            }
        }
    }
    ///
    /// expression
    ///   ::= primary binoprhs
    ///
    ///
    fn parse_expression(&mut self) -> Option<Box<ExprAst>> {
        debug!("Parse expression: {:?} ", self.current_token);
        if let Some(lhs) = self.parse_primary() {
            match self.parse_bin_op_rhs(0, &lhs) {
                // return Some(Box::new(*exp));
                Some(bin_expr) => return Some(Box::new(*bin_expr)),
                None => {
                    return None;
                }
            }
        }
        None
    }
    ///
    /// prototype
    ///   ::= id '(' id* ')'
    ///
    fn parse_proto_type(&mut self) -> Option<PrototypeAst> {
        debug!("Parse ProtoType : {:?} ", self.current_token);
        match self.current_token.clone() {
            Token::TokIdentifier(id) => {
                let fn_name = id;
                // debug!("wait ( ");
                self.get_next_token();
                match self.current_token.clone() {
                    Token::TokChar('(') => {
                        let mut argname: Vec<String> = vec![];
                        // debug!("Got (");
                        loop {
                            // matching an identifier
                            self.get_next_token();
                            match self.current_token.clone() {
                                Token::TokIdentifier(arg) => {
                                    argname.push(arg);
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                        // debug!("argnames :{:?}",&argname);
                        match self.current_token {
                            Token::TokChar(')') => {
                                // success
                                self.get_next_token(); //eat ')'
                                let proto = PrototypeAst {
                                    name: fn_name,
                                    args: argname,
                                };
                                debug!("Got prototype: {:?} ", &proto);
                                Some(proto)
                            }
                            _ => {
                                debug!("Expecting ( in prototype");
                                None
                            }
                        }
                    }
                    _ => {
                        debug!("Expecting ( in prototype");
                        None
                    }
                }
            }
            _ => {
                debug!("Expecting function name in prototype");
                None
            }
        }
    }
    ///
    /// definition ::= 'def' prototype expression
    ///
    fn parse_definition(&mut self) -> Option<Box<FunctionAst>> {
        debug!("Parse  definition: {:?} ", self.current_token);
        self.get_next_token();
        match self.parse_proto_type() {
            Some(proto) => match self.parse_expression() {
                Some(expr) => Some(Box::new(FunctionAst {
                    proto,
                    body: Box::clone(&expr),
                })),
                None => None,
            },
            None => None,
        }
    }
    ///
    /// toplevelexpr ::= expression
    ///
    fn parse_top_level_expr(&mut self) -> Option<Box<FunctionAst>> {
        if let Some(expr) = self.parse_expression() {
            // make an anonymous proto.
            debug!("parse_top_level_expr: {:?} ", self.current_token);
            let prot1 = Box::new(PrototypeAst {
                name: "".to_string(),
                args: vec![],
            });
            //let proto  :&'static PrototypeAst =&  prot1;
            return Some(Box::new(FunctionAst {
                proto: *prot1,
                body: Box::new(*expr),
            }));
        }
        None
    }
    /// external ::= 'extern' prototype
    fn parse_extern(&mut self) -> Option<PrototypeAst> {
        self.get_next_token();
        self.parse_proto_type()
    }
    //===----------------------------------------------------------------------===//
    // Top-Level parsing
    //===----------------------------------------------------------------------===//
    fn handle_definition(&mut self) {
        match self.parse_definition() {
            Some(ast) => {
                debug!("Parsed a function Definition, {:?}", ast);
            }
            None => {
                self.get_next_token();
            }
        }
    }
    fn handle_extern(&mut self) {
        match self.parse_extern() {
            Some(_) => {
                debug!("Parsed an extern \n");
            }
            None => {
                self.get_next_token();
            }
        }
    }
    fn handle_top_level_expr(&mut self) {
        match self.parse_top_level_expr() {
            Some(_) => {
                debug!("Parsed an top-level expr");
            }
            None => {
                self.get_next_token();
            }
        };
    }
    pub fn run(&mut self) {
        print!("ready> ");
        stdout().flush().unwrap();
        self.get_next_token();
        loop {
            let token = self.current_token.clone();
            match token {
                Token::TokEof => {
                    return;
                }
                Token::TokChar(';') => {
                    self.get_next_token();
                    continue;
                }
                Token::TokDef => self.handle_definition(),
                Token::TokExtern => self.handle_extern(),
                ref _s => self.handle_top_level_expr(),
            }
            print!("ready> ");
            stdout().flush().unwrap();
            self.get_next_token();
        }
    }
    pub fn parse_input(&mut self) {
        self.get_next_token();
        let token = self.current_token.clone();
        match token {
            Token::TokEof => {}
            Token::TokChar(';') => {
                self.get_next_token();
            }
            Token::TokDef => self.handle_definition(),
            Token::TokExtern => self.handle_extern(),
            ref _s => self.handle_top_level_expr(),
        }
        self.get_next_token();
    }
}
#[cfg(test)]
mod test {
    //#[macro_use]
    extern crate env_logger;
    extern crate log;
    use std::sync::mpsc::channel;
    use std::thread;

    use super::super::lexer::*;
    use super::super::token::*;
    use super::Parser;
    use log::LevelFilter;

    fn init() {
        let _ = env_logger::builder()
            .filter(None, LevelFilter::Info)
            .is_test(true)
            .try_init();
    }

    #[test]
    fn test_binop_pre() {
        let (tx, rx) = channel();
        let mut parser = Parser::new(rx);
        let mut check = |ch, expect| {
            tx.send(Token::TokChar(ch)).expect("Cant Send");
            parser.get_next_token();
            let res = parser.get_tok_precedence();
            assert_eq!(res, expect);
        };
        check('+', 20);
        check('-', 20);
        check('/', -1);
        check('*', 40);
    }
    #[test]
    fn test_def() {
        let (tx, rx) = channel();
        thread::spawn(move || {
            // tx.send(token::TokDef);
            tx.send(Token::TokIdentifier("a".to_string()))
                .expect("Send Error");
            tx.send(Token::TokChar('(')).expect("Send Error");
            tx.send(Token::TokIdentifier("b".to_string()))
                .expect("Send Error");
            tx.send(Token::TokChar(')')).expect("Send Error");
            tx.send(Token::TokNumber(123.1)).expect("Send Error");
            tx.send(Token::TokEof).expect("Send Error");
        });
        let mut parser = Parser::new(rx);
        let res = parser.parse_definition();
        let res_str = format!("{:?}", res);
        // dbg!(res);
        assert_eq!(res_str, "Some(FunctionAst { proto: PrototypeAst { name: \"a\", args: [\"b\"] }, body: NumberExpr(NumberExprAst { val: 123.1 }) })"); // test case wrong
    }
    /// rx_char_stream ----> test_stub  ----> tx_char_stream
    /// rx_char_stream --> lexer --> tx_token_stream
    /// rx_token_stream --> parser --> AstJson;
    ///
    #[test]
    fn test_parse_input() {
        init();
        let _prog = r"
# Compute the x'th fibonacci number.
def fib(x)
  if x < 3 then
    1
  else
    fib(x-1)+fib(x-2)

# This expression will compute the 40th number.
fib(40)
";
        let _prog2 = r"
# Compute the x'th fibonacci number.
def sum(x)
  a = x - 1
  b = x + sum(a)
  b

# This expression will compute the 40th number.
fib(40)
";
        let prog3 = r"
         def foo(x)
             x + 1
         ";
        debug!("input :\n{}", prog3);
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
            let _res = parser.parse_input();
            println!("tokens : {:?}", parser);
        });
        let _ = lexer_guard.join();
        let _ = parser_guard.join();
    }

}
