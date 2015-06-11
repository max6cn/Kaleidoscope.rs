# Kaleidoscope.rs
Kaleidoscope in Rust
## Background
See [Kaleidoscope Tutorial](http://llvm.org/docs/tutorial/LangImpl1.html)

## Build & Run
`cargo run --release`
## Test 
```
RUST_LOG=lib=debug ./target/debug/Kaleidoscope
ready> def foo(x y) x+foo(y, 4.0);
DEBUG:lib::Parser: Parse  definition
DEBUG:lib::Parser: Parse ProtoType
DEBUG:lib::Parser: Got prototype: PrototypeAst { Name: "foo", Args: ["x", "y"] }
DEBUG:lib::Parser: Parse expression
DEBUG:lib::Parser: Parse Primary
DEBUG:lib::Parser: Parse Identifier
DEBUG:lib::Parser: Parse binOpRHS
DEBUG:lib::Parser: Parse Primary
DEBUG:lib::Parser: Parse Identifier
DEBUG:lib::Parser: Parse expression
DEBUG:lib::Parser: Parse Primary
DEBUG:lib::Parser: Parse Identifier
DEBUG:lib::Parser: Parse binOpRHS
DEBUG:lib::Parser: Parse expression
DEBUG:lib::Parser: Parse Primary
DEBUG:lib::Parser: Parse Number exp
DEBUG:lib::Parser: Parse binOpRHS
DEBUG:lib::Parser: Parsed a function Definition
ready>
```
## Status

Parser finished

## License
Kaleidoscope.rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
See LICENSE-APACHE and LICENSE-MIT for details.
