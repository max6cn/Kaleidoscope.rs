clang hello.c -o hello-clang
clang hello.c -o hello-clang.ll -emit-llvm
llc hello.ll -o hello.s
as hello.s -o hello.o
ld  -macosx_version_min 10.14 hello.o -lSystem -o hello
./hello
