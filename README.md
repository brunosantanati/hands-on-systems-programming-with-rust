# [Hands-On Systems Programming with Rust](https://www.udemy.com/course/hands-on-systems-programming-with-rust/)

## Useful Links
[Installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)  

## Intallation on Linux
```
curl https://sh.rustup.rs -sSf | sh
```

## Commands
```
check Rust version:
rustc -V

check Cargo version:
cargo version

see all possible usages for cargo:
cargo

show help info:
cargo -h
cargo new -h

create a new cargo package (in this case a new binary application project):
cargo new myproject
cargo new pipeviewer

list the content of myproject folder:
tree --noreport myproject
or if you are inside the desired folder, simply run:
tree --noreport

compile and run the project in one step:
cargo run

compile the binary in release mode, resulting in a faster and smaller binary:
cargo run --release

run the automatically created binary directly:
target/debug/myproject
or if we want to run the release binary:
target/release/myproject

create a library package:
cargo new --lib mylib

run the tests:
cargo test

run only the Doc-tests
cargo test --doc --package mylib -- four --nocapture

generate documentation and open it in the browser:
cargo doc --no-deps --open
the doc will be generated at target/doc/mylib/index.html

build the project (and the entire dependency chain):
cargo build
```

## Pipeviewer commands
```
run the code:
cargo run

create a new file:
dd if=/dev/urandom bs=1024 count=128 of=myfile

show files size:
ls -lh
ls -la

build the binary:
cargo build

run the program again using a file as an input:
cat myfile | target/debug/pipeviewer > myfile2

see there is no difference between the 2 files:
diff myfile myfile2
```