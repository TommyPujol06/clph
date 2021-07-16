use std::cell::RefCell;
use std::rc::Rc;

extern crate raster;

pub struct Pixel {
    value: u16,
    pos: (i32, i32),
}

pub struct Image {
    src: Rc<RefCell<raster::Image>>,
}

impl Image {
    pub fn open(filepath: &str) -> Self {
        let src = raster::open(filepath).unwrap_or_else(|e| {
            panic!(
                "There was an error while opening the image file.\nError: {:#?}",
                e
            )
        });

        let mut cur_x = 0;
        let mut cur_y = 0;

        for pixel in src.bytes.chunks(4).into_iter() {
            if cur_x % (src.width - 1) == 0 && cur_x != 0 {
                cur_y += 1;
                cur_x = 0;
            } else {
                cur_x += 1;
            }

            if let [r, g, b, _] = pixel {
                let pxl = Pixel {
                    value: *r as u16 + *g as u16 + *b as u16,
                    pos: (cur_x, cur_y),
                };
            }
        }

        Self {
            src: Rc::new(RefCell::new(src)),
        }
    }

    // pub fn draw_chunk(&self, chunk: &Chunk>, colour: &raster::Color) {
    //     let mut img_ref = self.src.borrow_mut();

    //     for pxl in blob.iter() {
    //         img_ref
    //             .set_pixel(pxl.pos.0, pxl.pos.1, colour.to_owned())
    //             .unwrap();
    //     }
    // }

    pub fn save(&self, outfile: &str) {
        let img_ref = self.src.borrow_mut();
        raster::save(&img_ref, outfile).unwrap();
    }
}

fn main() {
    let _img = Image::open(&"sample.jpeg");

    // let colour = raster::Color::rgb(0, 0, 0);
    // let blobs = img.blobs.to_owned().unwrap();
    // for i in 0..20 {
    //     img.draw_blob(&blobs[i], &colour);
    // }

    // img.save(&"out.jpeg");
}
