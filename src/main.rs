use anyhow::Result;
use crossterm::terminal;
use image::ImageReader;
use std::env;
use std::path::Path;

struct AsciiImg {
    rows: usize,
    cols: usize,
    image: Vec<char>,
}

fn see(file: &Path, term_rows: usize, term_cols: usize) -> Result<AsciiImg> {
    let img = ImageReader::open("myimage.png")?
        .with_guessed_format()?
        .decode()?;
    // open file
}

fn main() {
    let (trows, tcols) = terminal::size().unwrap();
    // read file paths
    for (i, arg) in env::args().enumerate() {
        // ignore progname
        if i == 0 {
            continue;
        }
        println!("{arg}");
        see(Path::new(&arg), trows.into(), tcols.into());
    }
}
