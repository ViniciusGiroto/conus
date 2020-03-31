#[macro_use]
extern crate clap;

use bit_vec::BitVec;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Converts three booleans into an `u8` of value `[0, 8)`
///
/// # Arguments
/// * `l` the third bit
/// * `c` the second bit
/// * `r` the first bit
#[inline]
fn index(l: bool, c: bool, r: bool) -> u8 {
    (l as u8) << 2 | (c as u8) << 1 | (r as u8)
}

/// Processes the current state and producess the next one
///
/// # Arguments
///
/// * `nbv` the new state
/// * `nb` the current state
fn step(nbv: &mut BitVec<u32>, bv: &BitVec<u32>, rule: u8) {
    let end = bv.len() - 1;

    // handles edges
    nbv.set(0, (rule >> index(false, bv[0], bv[1])) & 1 == 1);
    nbv.set(end, (rule >> index(bv[end - 1], bv[end], false)) & 1 == 1);

    for i in 1..end {
        nbv.set(i, (rule >> index(bv[i - 1], bv[i], bv[i + 1])) & 1 == 1);
    }
}

/// Generates the states and calls a callback function each step
///
/// # Arguments
/// * `rule` the rule
/// * `steps` the number of steps
/// * `cb` the callback function, receives the current iteration (starts at 0) and the current state
fn generate<F>(rule: u8, steps: u32, mut cb: F) where F: FnMut(u32, &BitVec<u32>) {
    let n = steps as usize;
    let len = 2 * n + 1;

    let mut nbv = BitVec::from_elem(len as usize, false);
    let mut bv = BitVec::from_elem(len as usize, false);

    nbv.set(n, true);
    cb(0, &nbv);

    for i in 1..(steps + 1) {
        std::mem::swap(&mut bv, &mut nbv);
        step(&mut nbv, &bv, rule);
        cb(i, &nbv);
    }
}

/// Same as `generate` but writes png output to a `Write`
///
/// # Arguments
/// * `w` the `Write` implementation
/// * `rule` the rule
/// * `steps` the number of steps
fn generate_to_writer<W>(ref mut w: W, rule: u8, steps: u32) where W: Write {
    let mut encoder = png::Encoder::new(w, 2 * steps + 1, steps + 1);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);

    let mut writer = encoder.write_header().unwrap();
    let mut writer = writer.stream_writer();

    generate(rule, steps, |_, bv| {
        // TODO: avoid a copy
        let data = bv.to_bytes();
        writer.write(&data).unwrap();
    });

    writer.finish().unwrap();
}

fn main() {
    let matches = clap_app!(conus => 
        (version: "1.0")
        (author: "Vinicius H. P. Giroto <viniciusgiroto@usp.br>")
        (about: "Generates cellular automata diagrams")
        (@arg RULE: +required "Sets the rule number")
        (@arg ITER: +required "Number of iterations")
        (@arg FILE: -o +takes_value "Output file")
        (@arg ASCII: -a "ASCII mode")
    ).get_matches();

    let rule: u8 = matches.value_of("RULE").unwrap().parse().expect("Rule must be a number between 0 and 255.");
    let steps: u32 = matches.value_of("ITER").unwrap().parse().expect("Iter must be a number bigger than or equal to zero.");

    if matches.is_present("ASCII") {
        generate(rule, steps, |_, bv| {
            for bit in bv.iter() {
                if bit {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!("");
        });
    } else {
        if let Some(value) = matches.value_of_os("FILE") {
            generate_to_writer({
                let path = Path::new(value);
                let file = File::create(path).unwrap();
                BufWriter::new(file)
            }, rule, steps);
        } else {
            generate_to_writer(BufWriter::new(std::io::stdout()), rule, steps);
        }
    } 
}
