# giza_example

## Error replication
```
git clone git@github.com:lambdaclass/giza_example.git
cd giza_example
cargo run
```

## Explanation 
We start with the following cairo program:

```
%builtins output
from starkware.cairo.common.serialize import serialize_word

func main{output_ptr: felt*}() {
    tempvar x = 10;
    tempvar y = x + x;
    tempvar z = y * y + x;
    serialize_word(x);
    serialize_word(y);
    serialize_word(z);
    return ();
}

``` 

This program is compiled with
```
cairo-compile program.cairo --output program.json
```

then, the memory and trace files are generated with

```
cairo-run --program program.json --trace_file trace.bin --layout all --memory_file memory.bin
```

The code panics in this line from `main.rs`
```rust
let (_proof, _pub_inputs) = giza_prover::prove_trace(trace, &proof_options).unwrap();
```

with the following error
```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 0, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 93, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31]`,
 right: `[31, 31, 31, 31, 31, 31, 31, 31, 31, 0, 0, 31, 31, 31, 31, 0, 31, 62, 62, 62, 31, 62, 0, 62, 0, 0, 62, 62, 62, 62, 62, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31]`: transition constraint degrees didn't match
expected: [ 31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,   0,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  93,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31]
actual:   [ 31,  31,  31,  31,  31,  31,  31,  31,  31,   0,   0,  31,  31,  31,  31,   0,  31,  62,  62,  62,  31,  62,   0,  62,   0,   0,  62,  62,  62,  62,  62,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31,  31]', /Users/marian/.cargo/git/checkouts/winterfell-948b882ce897e2c0/79f83b3/prover/src/constraints/evaluation_table.rs:245:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```