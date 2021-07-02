use std::collections::HashMap;

extern crate raster;

pub const MAX_PIXEL_DIFFERENCE: u8 = 5;
pub const MAX_FIELDS_DIFFERENT: u8 = 2;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

// blobs = {
//      colour1: [
//          (x, y), -- where we found colour
//          {
//              colour1: (x, y),
//              colour2: (x, y),
//              colourN: (x, y), -- similar colours to parent colour.
//          }
//      ],
//
//      colourN: [...] -- same as first exmaple.
// }

pub enum Blob {
    Pxl(Box<Pixel>),
    Similar(HashMap<Pixel, (u64, u64)>),
}

pub type Blobs = HashMap<Pixel, Vec<Blob>>;

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn is_similar(&self, other: &Pixel) -> bool {
        let mut different_fields: u8 = 0;
        if ((self.r - other.r) as i8).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        if ((self.g - other.g) as i8).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        if ((self.b - other.b) as i8).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        different_fields <= MAX_FIELDS_DIFFERENT
    }
}

pub struct Image {
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn open(filepath: &str) -> raster::Image {
        raster::open(filepath).unwrap_or_else(|e| {
            panic!(
                "There was an error while opening the image file.\nError: {:#?}",
                e
            )
        })
    }

    pub fn from(img: raster::Image) -> Self {
        let bytes = img.bytes;
        assert!(bytes.len() % 3 == 0);

        let mut pixels: Vec<Pixel> = Vec::new();
        for chunk in bytes.chunks(3).into_iter() {
            if let [r, g, b] = chunk {
                pixels.push(Pixel::new(*r, *g, *b));
            } else {
                unreachable!("Bad formatted image.");
            }
        }

        Self { pixels }
    }

    pub fn find_blobs(&self) -> Blobs {
        let /*mut*/ blobs: Blobs = Blobs::new(); // This will need to be mutable.
        let mut previous: Pixel = Pixel::new(0, 0, 0);
        for pixel in self.pixels.iter() {
            if previous.is_similar(&pixel) {
                // Check if any pixel with a similar colour is in blobs. If so add this pixel
                // there.
                //
                // Otherwise insert it into the HashMap.
                todo!();
            }

            previous = *pixel;
        }

        blobs
    }
}

fn main() {
    println!("Opening image...");
    let img = Image::open(&"sample.jpeg");
    println!("Parsing image...");
    let img = Image::from(img);
    println!("Finding blobs in image...");
    let blobs = img.find_blobs();
    println!("Found {} blobs.", blobs.len());
}
