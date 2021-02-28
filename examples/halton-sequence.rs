extern crate halton;

use std::env;
use std::io::{self, Write};
use std::process::exit;

mod sysexit {
    pub const USAGE: i32 = 64;
    pub const SIGPIPE: i32 = 141;
    pub const IOERR: i32 = 74;
}

fn main() {
    let mut args = env::args();
    args.next(); // skip progname
    let base = match args.next().map(|s| s.parse()).unwrap_or(Ok(2)) {
        Ok(i) if i > 1 => i,
        _ => {
            eprintln!("Bad value for BASE");
            exit(sysexit::USAGE)
        }
    };
    let skip = match args.next().map(|s| s.parse()).unwrap_or(Ok(0)) {
        Ok(i) => i,
        _ => {
            eprintln!("Bad value for SKIP");
            exit(sysexit::USAGE)
        }
    };
    if args.count() > 0 {
        eprintln!("Usage: halton-sequence [BASE] [SKIP]");
        exit(sysexit::USAGE)
    }

    match print_sequence(base, skip) {
        Ok(()) => (),
        Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => exit(sysexit::SIGPIPE),
        Err(e) => {
            eprintln!("{}", e);
            exit(sysexit::IOERR)
        }
    }
}

fn print_sequence(base: u16, skip: usize) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for f in halton::Sequence::new(base).skip(skip) {
        handle.write(f.to_string().as_bytes())?;
        handle.write(b"\n")?;
    }

    Ok(())
}
