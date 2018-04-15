extern crate halton;

use halton::Sequence;

fn main() {
    let mut grid = [["."; 10]; 10];
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").skip(1).take(26);
    let seq = Sequence::new(2).zip(Sequence::new(3)).zip(alpha);
    for ((x, y), c) in seq {
        grid[(y * 10.0) as usize][(x * 10.0) as usize] = c;
    }
    for row in grid.iter() {
        println!("{}", row.join(" "));
    }
}
