# Failed to create this compiler. [next](https://github.com/rchaser53/rr-llvm-ir).

# rust-monkey-ir
this is a toy compiler emits LLVM IR

# how to play

1. compiles rust-monkey-ir file(xxx.mr) and emits LLVM IR(yyy.ll).

```
cargo run [input] [output]

# default input is input.mr
# default output is output.ll
```

2. executes LLVM IR file by ```lli```.

```
lli [LLVM IR file]

# ex. lli output.ll
```

#### ※ perhaps you need to install LLVM to use lli.

# demo
you can enjoy Fizz Buzz in rust-monkye-ir. like below.

<img alt="" src="https://github.com/rchaser53/rust-monkey-ir/blob/master/FizzBuzz.gif" >
