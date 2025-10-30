use anyhow::Result;
use crossterm::terminal;
use image::{imageops::FilterType, DynamicImage, ImageReader};
use std::{env, fmt, path::Path, process::exit};

const ASCII_GRADIENT: &[u8] = b"@%#*+=-:. "; // darkest → lightest

struct AsciiImg {
    width: u32,
    image: Vec<char>,
}

impl fmt::Display for AsciiImg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, p) in self.image.iter().enumerate() {
            if i > 0 && i % self.width as usize == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{p}")?;
        }
        Ok(())
    }
}

impl From<DynamicImage> for AsciiImg {
    fn from(img: DynamicImage) -> Self {
        AsciiImg {
            width: img.width(),
            image: img.as_bytes().iter().map(|b| pixel_to_ascii(*b)).collect(),
        }
    }
}

fn pixel_to_ascii(byte: u8) -> char {
    let index = (byte as f32 / 255.0 * (ASCII_GRADIENT.len() - 1) as f32) as usize;
    ASCII_GRADIENT[index] as char
}

fn see(file: &Path, term_rows: u32, term_cols: u32) -> Result<()> {
    let dyn_img = ImageReader::open(file)?
        .with_guessed_format()?
        .decode()?
        .resize(term_cols, term_rows, FilterType::Gaussian)
        .grayscale();
    let img = AsciiImg::from(dyn_img);
    println!("{img}");
    // TODO: print the image
    Ok(())
}

fn main() -> Result<()> {
    if env::args().len() != 2 {
        println!("usage: cargo run -- PATH");
        exit(1);
    }
    let (tcols, trows) = terminal::size().unwrap();
    // read file paths
    for (i, arg) in env::args().enumerate() {
        // ignore progname
        if i == 0 {
            continue;
        }
        println!("{arg}");
        see(Path::new(&arg), trows.into(), tcols.into())?;
    }
    Ok(())
}
