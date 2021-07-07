use std::cell::RefCell;
use std::rc::Rc;

extern crate raster;

pub const MAX_PIXEL_DIFFERENCE: usize = 5;
pub const MAX_FIELDS_DIFFERENT: u8 = 1;

#[derive(Hash, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    pos: (i32, i32),
}

pub type Blobs<'a> = Vec<Vec<&'a Pixel>>;

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

pub struct Image<'a> {
    src: Rc<RefCell<raster::Image>>,
    pixels: Vec<Pixel>,
    blobs: Option<Blobs<'a>>,
}

impl<'a> Image<'a> {
    pub fn open(filepath: &str) -> Self {
        println!("Opening image...");
        let src = raster::open(filepath).unwrap_or_else(|e| {
            panic!(
                "There was an error while opening the image file.\nError: {:#?}",
                e
            )
        });

        let bytes = src.bytes.clone();
        assert!(bytes.len() % 4 == 0);

        println!("Image is {}x{}", src.width, src.height);

        let mut pixels: Vec<Pixel> = Vec::new();
        let (mut cur_x, mut cur_y): (i32, i32) = (0, 0);
        for (i, chunk) in bytes.chunks(4).into_iter().enumerate() {
            let pos = (cur_x, cur_y);
            if cur_x % (src.width - 1) == 0 && cur_x != 0 {
                cur_y += 1;
                cur_x = 0;
            } else {
                cur_x += 1;
            }

            if i % 4 == 0 {
                // Only parse 1 byte for every 4 bytes for better performance.
                continue;
            }

            if let [r, g, b, _] = chunk {
                let luminance = 0.2126 * *r as f64 + 0.7152 * *g as f64 + 0.0722 * *b as f64;
                if luminance >= 190.0 {
                    continue; // Skip bright colours.
                }
                let pxl = Pixel::new(*r, *g, *b, pos);
                pixels.push(pxl);
            } else {
                unreachable!("Bad formatted image.");
            }
        }

        Self {
            src: Rc::new(RefCell::new(src)),
            pixels,
            blobs: None,
        }
    }

    pub fn find_blobs(&'a mut self) {
        println!("Finding blobs in image...");
        let mut blobs: Blobs = Blobs::new();

        // FIXME: O(n^3) -- Very inefficient.
        // FIXME: There is probably a logic error.
        // FIXME: This takes **very** long.
        for pixel in self.pixels.iter() {
            let mut found_match = false;
            for blob in blobs.iter_mut() {
                for pxl in blob.to_owned().iter() {
                    if pxl.is_similar(pixel) {
                        blob.push(pixel);
                        found_match = true;
                        break;
                    }
                }
            }

            if !found_match {
                blobs.push(vec![pixel]);
            }
        }

        println!("Found {} blobs.", blobs.len());
        self.blobs = Some(blobs);
    }

    pub fn sorted_blobs(&mut self) {
        if self.blobs.is_some() {
            self.blobs
                .to_owned()
                .unwrap()
                .sort_by(|this, other| this.len().cmp(&other.len()));
        }
    }

    pub fn draw_blob(&self, blob: &Vec<&Pixel>, colour: &raster::Color) {
        let mut img_ref = self.src.borrow_mut();
        for pxl in blob.iter() {
            img_ref
                .set_pixel(pxl.pos.0, pxl.pos.1, colour.to_owned())
                .unwrap();
        }
    }

    pub fn save(&self, outfile: &str) {
        println!("Saving image...");
        let img_ref = self.src.borrow_mut();
        raster::save(&img_ref, outfile).unwrap();
    }
}

fn main() {
    let mut img = Image::open(&"sample.jpeg");

    img.find_blobs();
    // img.sorted_blobs();

    // let colour = raster::Color::rgb(0, 0, 0);
    // let blobs = img.blobs.to_owned().unwrap();
    // for i in 0..20 {
    //     img.draw_blob(&blobs[i], &colour);
    // }

    // img.save(&"out.jpeg");
}
