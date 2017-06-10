#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use errors::*;

fn main() {
    if let Err(ref e) = run() {
      use std::io::Write;
      let stderr = &mut ::std::io::stderr();
      let errmsg = "Error writing to stderr";

      writeln!(stderr, "error: {}", e).expect(errmsg);

      for e in e.iter().skip(1) {
          writeln!(stderr, "caused by: {}", e).expect(errmsg);
      }

      if let Some(backtrace) = e.backtrace() {
          writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
      }

      ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).chain_err(|| format!("cannot open file {}", filename))?;
    let read_file = BufReader::new(&f);

    let mut total_bytes = 0;
    let mut z_bytes = 0;
    for b in read_file.bytes() {
      if b.unwrap() == 0 { z_bytes += 1; }
      total_bytes += 1;
    }

    println!("{}: {:.2}% empty", filename, (z_bytes as f64 / total_bytes as f64) * 100.00);

    Ok(())
}
