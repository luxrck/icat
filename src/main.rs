#[warn(dead_code)]
use std::io;
use std::env;
use std::result::Result;
use std::path::Path;

use image::{ DynamicImage, FilterType, ImageError };
use console::Term;


struct ANSIImage {
    width: u32,
    height: u32,
    bestfit: bool,
    image: DynamicImage,
}

impl ANSIImage {
    fn new(path: &Path) -> Result<Self, ImageError> {
        let img = image::open(path)?;
        Ok(ANSIImage { image: img, width: 0, height: 0, bestfit: false })
    }

    fn with_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width; self.height = height;
        self
    }

    fn with_bestfit(&mut self, bestfit: bool) -> &mut Self {
        self.bestfit = bestfit;
        self
    }
    
    fn write_bestfit(&self, output: &mut io::Write) -> Result<(), ImageError> {


        Ok(())
    }

    fn write(&self, output: &mut io::Write) -> Result<(), ImageError> {
        if self.bestfit { return self.write_bestfit(output) }
        
        let image = self.image
            .resize(self.width, self.height, FilterType::CatmullRom)
            .to_rgb();
        for r in 0..image.height() / 2 {
            for c in 0..image.width() {
                let bg = image.get_pixel(c, 2 * r);
                let fg = image.get_pixel(c, 2 * r + 1);
                write!(output,
                       "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m\u{2584}\x1b[0m",
                       bg[0], bg[1], bg[2], fg[0], fg[1], fg[2])?;
            }
            write!(output, "\n")?;
        }

        Ok(())
    }
}


fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    let (h, w) = Term::stdout().size();
    println!("w:{} h:{}", w, h);
    ANSIImage::new(Path::new(&args[1]))?
        .with_size((w) as u32, ((h-1)*2) as u32)
        .with_bestfit(false)
        .write(&mut io::stdout())?;
    
    Ok(())
}
