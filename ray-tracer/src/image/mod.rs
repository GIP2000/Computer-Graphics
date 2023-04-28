use anyhow::Result;
use rayon::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::atomic::AtomicU32,
};

use crate::Color;

pub struct PPMImageWriter {
    file: File,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub aspect_ratio: f64,
}

impl PPMImageWriter {
    pub fn new(
        file_name: &str,
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
    ) -> Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)?;

        return Ok(Self {
            file,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            image_width,
            aspect_ratio,
            samples_per_pixel,
        });
    }

    fn write_color(&mut self, color: Color) -> Result<()> {
        let scale = 1. / self.samples_per_pixel as f64;
        let r = (256. * ((scale * color.x).sqrt().clamp(0., 0.999))) as u32;
        let g = (256. * ((scale * color.y).sqrt().clamp(0., 0.999))) as u32;
        let b = (256. * ((scale * color.z).sqrt().clamp(0., 0.999))) as u32;

        writeln!(self.file, "{} {} {}", r, g, b)?;
        Ok(())
    }

    pub fn write<F>(mut self, closure: F) -> Result<()>
    where
        F: Fn(u32, u32, &Self) -> Color + Send + Sync,
    {
        writeln!(
            self.file,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;

        let counter: AtomicU32 = AtomicU32::new(0);
        eprintln!(
            "width: {}, height: {}, total: {}",
            self.image_width,
            self.image_height,
            self.image_height * self.image_width
        );

        let colors: Vec<Color> = (0..(self.image_width * self.image_height))
            .into_par_iter()
            .map(|idx| {
                let j = self.image_height - (idx / self.image_width);
                let i = idx % self.image_width;
                let res = closure(j, i, &self);
                let prev = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                eprint!(
                    "\r{:.2}%",
                    (((prev + 1) as f64 / (self.image_width * self.image_height) as f64) * 100.)
                );
                std::io::stderr().flush().unwrap();
                res
            })
            .collect();

        for color in colors.into_iter() {
            self.write_color(color)?;
        }
        // for j in (0..self.image_height).rev() {
        //     for i in 0..self.image_width {
        //         eprint!("\rScanlines remaing {} ", j);
        //         std::io::stderr().flush()?;
        //         self.write_color(closure(j, i, &self))?;
        //     }
        // }
        eprintln!("\nDone. ");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use cgmath::vec3;

    use super::*;

    #[test]
    fn test_writer() {
        let writer = PPMImageWriter::new("./image.ppm", 256, 1., 100).unwrap();
        assert!(writer
            .write(|j, i, writer| {
                return vec3(
                    (i as f64) / (writer.image_width - 1) as f64,
                    (j as f64) / (writer.image_height - 1) as f64,
                    0.25,
                );
            })
            .map(|_| true)
            .unwrap_or(false));
    }
}
