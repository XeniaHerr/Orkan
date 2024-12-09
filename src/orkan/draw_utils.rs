use std::collections::HashMap;
use std::boxed;

use rusttype::{self, Font, Scale, Point};


struct RenderedGlyph {
    stride : u32,

    content : Box<Vec<u8>>
}

struct FontCache {

    map : HashMap<char, RenderedGlyph>,

    pub font : Font<'static>,


}


impl FontCache {
    pub fn new(font : Font<'static>) -> Self {
        FontCache {
            map : HashMap::new(),
            font : font,
        }
    }

    /**
     * Iteate trough all chars given and make shure they are cached*/
    pub fn build_cache(&mut self, chars : Vec<char>) {

        for c in chars.iter() {
        if let Some(glyph) = self.map.get(&c) {
            continue;
        } else {
            let glyph = self.font.glyph(*c).scaled(Scale::uniform(18.0)).positioned(Point {x: 0.0, y: 0.0});
            //THis causes a panic, because <space> doesn't have a bounding box
            let width =  if let Some(bb) = glyph.pixel_bounding_box() {
                bb.width() as u32
            }
            else {
                    10 as u32
            };
                let mut content = Box::new(vec![0xff; width as usize * 18 * 4]);
                glyph.draw(|x,y,v| {

                        //let x = x + bb.min.x as u32;
                        //let y = y + bb.min.y as u32;
                        let idx = (x + y * width) as usize * 4;
                        if idx % 4 != 0 {
                            println!("Idx: {idx}");
                        }
                        content[idx..idx+4].copy_from_slice(&[
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        (0x00 as f32 * v + 0xff as f32 * (1.0 - v)) as u8,
                        0xff as u8]);
                });
                let rg = RenderedGlyph {
                    stride : width, 
                    content : content,
                };

                self.map.insert(*c, rg);
        }
    }
    }


    /**
     * Return cached Glyph if available */
    pub fn draw_chached_char(&self, c : char ) -> Option<&RenderedGlyph> {
        if let Some(glyph) = self.map.get(&c) {
            Some(glyph)
        } else {
            None
        }
    }
}


pub struct Renderer {
    //font : Font<'static>,

    cache : FontCache,

    tip : Point<i32>,


    pub cur_search : Vec<char>,

    width : u32,
    height : u32,
    scale : Scale,
}


impl Renderer {

    pub fn new(font : Font<'static>, width : u32, height : u32) -> Self {
        Renderer {
            tip : Point{x: 0, y: 0},
            cur_search : Vec::new(),
            width : width,
            height : height,
            scale : Scale::uniform(18.0),

            cache : FontCache::new(font),
        }
    }


    pub fn get_width(&self) -> u32 {
        return self.width;
    }
    pub fn get_height(&self) -> u32 {
        return self.height;
    }
    
    pub fn get_width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    pub fn get_height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }

pub fn render_length(&mut self, content : Vec<char>, font : &Font, scale : Scale) -> u32 {



    let width = content.iter().fold(0, |acc, c| {
        acc + font.glyph(*c).scaled(scale).h_metrics().advance_width as u32
    });

    return width;
}

pub fn render_full_image(&mut self, canvas : &mut [u8]) {

            self.cache.build_cache(self.cur_search.clone());
            canvas.fill(0xff);

            let input_field = self.cur_search.iter().collect::<String>();

            let scale = rusttype::Scale::uniform(18.0);

            let v_metrics = self.cache.font.v_metrics(scale);

            let glyphs = self.cache.font.layout(&input_field, rusttype::Scale::uniform(14.0), rusttype::point(1.0 ,1.0 + v_metrics.ascent));


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
                        0xff as u8]);

                    })
                }
            }




}
}
