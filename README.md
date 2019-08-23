- [Kaleidoscope.rs](#kaleidoscopers)
  - [Guide](#guide)
  - [Status](#status)
  - [License](#license)
  - [Why continue?](#why-continue)


# Kaleidoscope.rs
Kaleidoscope in Rust, See [My First Language Frontend with LLVM Tutorial](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html)


## Guide

- [ ] [Chapter 1: Kaleidoscope language and lexer](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl01.html)
- [ ] [Chapter 2: Implementing a parser and AST](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl02.html)
- [ ] [Chapter 3: Code generation to LLVM IR](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl03.html)
- [ ] [Chapter 4: Adding JIT and Optimizer Support](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl04.html)
- [ ] [Chapter 5: Extending the Language: Control Flow](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl05.html)
- [ ] [Chapter 6: Extending the Language: User-defined Operators](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl06.html)
- [ ] [Chapter 7: Extending the Language: Mutable Variables](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl07.html)
- [ ] [Chapter 8: Compiling to Object Files](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl08.html)
- [ ] [Chapter 9: Debug Information](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl09.html)
- [ ] [Chapter 10: Conclusion and other tidbits](http://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl10.html)

## Status

```
export RUST_LOG=main=trace,lib=trace
export LLVM_SYS_80_PREFIX=$HOME/llvm-8.0.1

RUST_LOG=lib=debug ./target/debug/Kaleidoscope
ready> def foo(x y) x+foo(y, 4.0);
DEBUG:lib::parser: Parse  definition
DEBUG:lib::parser: Parse ProtoType
DEBUG:lib::parser: Got prototype: PrototypeAst { name: "foo", args: ["x", "y"] }
DEBUG:lib::parser: Parse expression
DEBUG:lib::parser: Parse Primary
DEBUG:lib::parser: Parse Identifier
DEBUG:lib::parser: Parse binOpRHS
DEBUG:lib::parser: Parse Primary
DEBUG:lib::parser: Parse Identifier
DEBUG:lib::parser: Parse expression
DEBUG:lib::parser: Parse Primary
DEBUG:lib::parser: Parse Identifier
DEBUG:lib::parser: Parse binOpRHS
DEBUG:lib::parser: Parse expression
DEBUG:lib::parser: Parse Primary
DEBUG:lib::parser: Parse Number exp
DEBUG:lib::parser: Parse binOpRHS
DEBUG:lib::parser: Parsed a function Definition
ready>
```
## License
Kaleidoscope.rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
See LICENSE-APACHE and LICENSE-MIT for details.


## Why continue?
Years back when looking into Rust I was thinking making a rust version of LLVM tutorial, but at that time I feel rust was [not ready](https://stackoverflow.com/questions/29885638/cannot-insert-reference-in-hashmap-if-it-is-declared-after-the-data-i-am-inserti) for production, so I stopped with a WIP project here. Since then the [ergonomics](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html) has been improved a lot, maybe its good time to try again.
