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

struct Generator {
    rule: u8,
    steps: u32,
    current: BitVec<u32>,
    next: BitVec<u32>,
}

impl Generator {
    fn new(rule: u8, steps: u32) -> Self {
        let current = BitVec::from_elem((2 * steps + 1) as usize, false);
        let next = BitVec::from_elem((2 * steps + 1) as usize, false);

        Self { rule, steps, current, next }
    }

    fn init(&mut self) {
        self.current.set(self.steps as usize, true);
    }

    fn next(&mut self) {
        let bv = &self.current;
        let nbv = &mut self.next;
        let end = bv.len() - 1;
    
        // handles edges
        nbv.set(0, (self.rule >> index(false, bv[0], bv[1])) & 1 == 1);
        nbv.set(end, (self.rule >> index(bv[end - 1], bv[end], false)) & 1 == 1);
    
        for i in 1..end {
            nbv.set(i, (self.rule >> index(bv[i - 1], bv[i], bv[i + 1])) & 1 == 1);
        }

        std::mem::swap(&mut self.current, &mut self.next);
    }

    #[inline]
    fn write_png<W: Write>(&mut self, w: &mut W) -> std::io::Result<()> {
        let data = self.current.to_bytes();
        w.write_all(&data)?;
        Ok(())
    }

    fn generate_png<W: Write>(&mut self, w: &mut W) -> std::io::Result<()> {
        let mut encoder = png::Encoder::new(w, 2 * self.steps + 1, self.steps + 1);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::One);
    
        let mut writer = encoder.write_header()?;
        let mut writer = writer.stream_writer();

        self.write_png(&mut writer)?;

        for _ in 1..(self.steps + 1) {
            self.next();
            self.write_png(&mut writer)?;
        }
    
        writer.finish()?;
        Ok(())
    }

    fn write_ascii<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        for bit in self.current.iter() {
            if bit {
                w.write_all("█".as_bytes())?;
            } else {
                w.write_all(" ".as_bytes())?;
            }
        }
        w.write_all("\n".as_bytes())?;
        w.flush()?;
        Ok(())
    }

    fn generate_ascii<W: Write>(&mut self, mut w: &mut W) -> std::io::Result<()> {
        self.write_ascii(&mut w)?;

        for _ in 1..(self.steps + 1) {
            self.next();
            self.write_ascii(&mut w)?;
        }
        Ok(())
    }
}

fn main() {
    let matches = clap_app!(conus => 
        (version: "0.2")
        (author: "Vinicius H. P. Giroto <viniciusgiroto@usp.br>")
        (about: "Generates cellular automata diagrams")
        (@arg RULE: +required "Sets the rule number")
        (@arg ITER: +required "Number of iterations")
        (@arg FILE: -o +takes_value "Output file")
        (@arg FORMAT: -f --output-format +takes_value "Output format (ascii,png)")
    ).get_matches();

    let rule: u8 = matches.value_of("RULE").unwrap().parse().expect("Rule must be a number between 0 and 255.");
    let steps: u32 = matches.value_of("ITER").unwrap().parse().expect("Iter must be a number bigger than or equal to zero.");
    let format = matches.value_of("FORMAT").or_else(|| Some("ascii")).unwrap();
    let path = matches.value_of_os("FILE");

    /* Initial state */
    let mut generator = Generator::new(rule, steps);
    generator.init();

    /**/
    if let Some(value) = path {
        let mut write = {
            let path = Path::new(value);
            let file = File::create(path).unwrap();
            BufWriter::new(file)
        };

        match format {
            "ascii" => { generator.generate_ascii(&mut write); }
            "png" => { generator.generate_png(&mut write); }
            _ => {}
        }
    } else {
        let mut write = BufWriter::new(std::io::stdout());

        match format {
            "ascii" => { generator.generate_ascii(&mut write); }
            "png" => { generator.generate_png(&mut write); }
            _ => {}
        }
    }
}
