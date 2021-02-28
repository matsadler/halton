# halton

[![Build](https://github.com/matsadler/halton/actions/workflows/test.yml/badge.svg)](https://github.com/matsadler/halton/actions/workflows/test.yml)
[![Version](https://img.shields.io/crates/v/halton.svg)](https://crates.io/crates/halton)
[![Docs](https://docs.rs/halton/badge.svg)](https://docs.rs/halton)

A module for generating Halton sequences, a deterministic low discrepancy
sequence that appears to be random. The uniform distribution and
repeatability makes the sequence ideal for choosing sample points or
placing objects in 2D or 3D space.

## Examples

``` rust
use halton::Sequence;

let mut grid = [["."; 10]; 10];
let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").skip(1).take(26);
let seq = Sequence::new(2).zip(Sequence::new(3)).zip(alpha);
for ((x, y), c) in seq {
    grid[(y * 10.0) as usize][(x * 10.0) as usize] = c;
}
for row in grid.iter() {
    println!("{}", row.join(" "));
}
```

Outputs:

``` text
. . R . . I . . . .
. L . . . . U C . .
X . . F . . . . . O
. . . J . A . . . .
. D . . . . M S . .
P . . . V . . . G .
. . B . . Y . . . .
. T . . . . E . K .
H . . . N . . . . W
. . . Z . Q . . . .
```

## License

This project is licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)

at your option.
