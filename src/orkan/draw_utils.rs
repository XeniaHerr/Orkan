

use rusttype::{self, Font, Scale, Point};


pub struct Renderer {
    font : Font<'static>,

    tip : Point<i32>,


    pub cur_search : Vec<char>,

    width : u32,
    height : u32,
    scale : Scale,
}


impl Renderer {

    pub fn new(font : Font<'static>, width : u32, height : u32) -> Self {
        Renderer {
            font : font,
            tip : Point{x: 0, y: 0},
            cur_search : Vec::new(),
            width : width,
            height : height,
            scale : Scale::uniform(18.0),
        }
    }


    pub fn get_width(&self) -> u32 {
        return self.width;
    }
    pub fn get_height(&self) -> u32 {
        return self.height;
    }
    
    pub fn set_width(&mut self, width : u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, width : u32) {
        self.height = width;
    }

pub fn render_length(&mut self, content : Vec<char>, font : &Font, scale : Scale) -> u32 {



    let width = content.iter().fold(0, |acc, c| {
        acc + font.glyph(*c).scaled(scale).h_metrics().advance_width as u32
    });

    return width;
}

pub fn render_full_image(&mut self, canvas : &mut [u8]) {
            canvas.fill(0xff);

            let input_field = self.cur_search.iter().collect::<String>();

            let scale = rusttype::Scale::uniform(18.0);

            let v_metrics = self.font.v_metrics(scale);

            let glyphs = self.font.layout(&input_field, rusttype::Scale::uniform(14.0), rusttype::point(1.0 ,1.0 + v_metrics.ascent));


            for glyph in glyphs {
                if let Some(bb) = glyph.pixel_bounding_box() {
                    glyph.draw(|x,y,v| {
                        let x = x + bb.min.x as u32;
                        let y = y + bb.min.y as u32;
                        let idx = (x + y * self.width) as usize * 4;
                        if idx % 4 != 0 {
                            println!("Idx: {idx}");
                        }
                        canvas[idx..idx+4].copy_from_slice(&[
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        (v * 256.0) as u8]);

                    })
                }
            }




}
}
