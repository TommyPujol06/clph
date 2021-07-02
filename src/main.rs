use std::cmp::{max, min};
use std::collections::HashMap;

extern crate raster;

pub const MAX_PIXEL_DIFFERENCE: usize = 5;
pub const MAX_FIELDS_DIFFERENT: u8 = 2;
pub const SKIP_N_BYTES: usize = 4;
pub const MIN_BLOB_LEN: usize = 80_000;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    pos: (i32, i32),
}

// blobs = {
//      colour1: [
//              (colour1, (x, y)),
//              (colour2, (x, y)),
//              (colourN, (x, y)), -- similar colours to parent colour.
//          ]
//      ,
//
//      colourN: [...] -- same as first exmaple.
// }

pub type Blob<'a> = Vec<(&'a Pixel, (i32, i32))>;
pub type Blobs<'a> = HashMap<&'a Pixel, Blob<'a>>;

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, pos: (i32, i32)) -> Self {
        Self { r, g, b, pos }
    }

    pub fn is_similar(&self, other: &Pixel) -> bool {
        let mut different_fields: u8 = 0;
        if ((self.r as isize - other.r as isize) as isize).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        if ((self.g as isize - other.g as isize) as isize).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        if ((self.b as isize - other.b as isize) as isize).unsigned_abs() > MAX_PIXEL_DIFFERENCE {
            different_fields += 1;
        }

        different_fields < MAX_FIELDS_DIFFERENT
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
        assert!(bytes.len() % 4 == 0);

        let mut pixels: Vec<Pixel> = Vec::new();
        let (mut cur_x, mut cur_y): (i32, i32) = (-1, 0);
        for (i, chunk) in bytes.chunks(4).into_iter().enumerate() {
            if cur_x % img.width == 0 && cur_x != 0 && cur_x != -1 {
                cur_y += 1;
                cur_x = 0;
            } else {
                cur_x += 1;
            }

            if i % SKIP_N_BYTES == 0 && i != 0 {
                continue;
            }

            let pos = (cur_x, cur_y);
            if let [r, g, b, _] = chunk {
                // FIXME: This is very a bad way of skiping colours.
                if *r >= 140 && *g >= 140 && *b >= 140 {
                    // Skip white-grayish colours.
                    continue;
                }
                pixels.push(Pixel::new(*r, *g, *b, pos));
            } else {
                unreachable!("Bad formatted image.");
            }
        }

        Self { pixels }
    }

    pub fn find_blobs(&self) -> Blobs {
        let mut blobs: Blobs = Blobs::new();
        for pixel in self.pixels.iter() {
            let mut has_found_similar_pixel = false;
            for tup in blobs.iter_mut() {
                let (pxl, similar) = tup;
                if pxl.is_similar(pixel) {
                    similar.push((pixel, pixel.pos));
                    has_found_similar_pixel = true;
                    break;
                }
            }

            if !has_found_similar_pixel {
                blobs.insert(pixel, vec![(pixel, pixel.pos)]);
            }
        }

        blobs
    }
}

pub fn filter_blobs(blobs: Blobs) -> Blobs {
    let mut filtered: Blobs = Blobs::new();
    for tup in blobs.iter() {
        let (key, blob) = tup;
        if blob.len() >= MIN_BLOB_LEN {
            filtered.insert(key, blob.clone());
        }
    }

    filtered
}

pub fn find_largest_blob(blobs: Blobs) -> Blob {
    let mut biggest_key: &Pixel = &Pixel::new(0, 0, 0, (0, 0));
    let mut biggest_len = 0;

    for tup in blobs.iter() {
        let (pxl, blob) = tup;
        let blob_len = blob.len();
        biggest_len = max(biggest_len, blob_len);
        if biggest_len == blob_len {
            biggest_key = *pxl;
        }
    }

    let blob = blobs.get(biggest_key).unwrap().clone();
    println!("Biggest blob size: {}", blob.len());
    // println!("Biggest blob: {:#?}", blobs.get(biggest_key));
    blob
}

pub fn analyse_blob(blob: Blob) {
    let mut biggest_x = 0;
    let mut biggest_y = 0;
    let mut smallest_x = 10_000_000; // This number needs to be bigger than any possible x
    let mut smallest_y = 10_000_000;

    for tup in blob.iter() {
        let (pxl, _) = tup;
        biggest_x = max(biggest_x, pxl.pos.0);
        biggest_y = max(biggest_y, pxl.pos.1);
        smallest_x = min(smallest_x, pxl.pos.0);
        smallest_y = min(smallest_y, pxl.pos.1);
    }

    println!("Biggest x: {}", biggest_x);
    println!("Biggest y: {}", biggest_y);
    println!("Smallest x: {}", smallest_x);
    println!("Smallest y: {}", smallest_y);
}

fn main() {
    println!("Opening image...");
    let img = Image::open(&"sample.jpeg");
    println!("Parsing image...");
    let img = Image::from(img);
    println!("Finding blobs in image...");
    let blobs = img.find_blobs();
    let blobs = filter_blobs(blobs);
    println!("Found {} blobs.", blobs.len());
    let biggest_blob = find_largest_blob(blobs);
    analyse_blob(biggest_blob);
    println!("Image has {} pixels.", img.pixels.len());
}
