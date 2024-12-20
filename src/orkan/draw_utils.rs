use std::collections::HashMap;
use std::boxed;
use super::search_element::SearchElement;

use rusttype::{self, Font, Scale, Point};

use super::color::Color;
use super::config::Config;






fn draw_rect(canvas : &mut [u8], (c_width, x_height) : (u32, u32), x : u32, y : u32, width : u32, height : u32, color : [u8; 4]) {

    for i in 0..height {
        let start = (x + (y + i) *c_width) as usize *4;
        let end = start + (width * 4) as usize;
        for  chunk in canvas[start..end].chunks_exact_mut(4) {
            chunk.copy_from_slice(&color);
        }


    }


}

/**
 * These structs are unused until further desing changes */
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
    pub fn build_cache(&mut self, chars : Vec<char>, _height: u32) {

        let scale = Scale::uniform((_height as i32 - 4 ) as f32  );
        let v_metrics = self.font.v_metrics(scale);
        for c in chars.iter() {
            if let Some(_glyph) = self.map.get(&c) {
                continue;
            } else {
                let s_glyph = self.font.glyph(*c).scaled(scale);
                let glyph = s_glyph.clone().positioned(Point {x: 0.0, y: 0.0 + v_metrics.ascent});
                let (width, height, minx, miny) =  if let Some(bb) = glyph.pixel_bounding_box() {
                    (s_glyph.h_metrics().advance_width as u32 , bb.height() as u32, bb.min.x, bb.min.y)
                }
                else {
                    (5 as u32, 5 as u32, 0,0)
                };
                let mut content = Box::new(vec![0xff; width as usize * (v_metrics.ascent.abs()as u32 + v_metrics.descent.abs()  as u32) as usize * 4]);
                let glyph = self.font.layout(&c.to_string(), scale, Point {x:0.0, y: 0.0 +v_metrics.ascent}).next().unwrap();
                println!("Height of Glyph: {}, Real Height: {height}", v_metrics.ascent.abs() + v_metrics.descent.abs());
                glyph.draw(|x,y,v| {

                    let x = x + minx as u32;
                    let y = y + miny as u32;
                    let idx = (x + y * width) as usize * 4 ;
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
    pub fn draw_chached_char(&self, c : &char ) -> Option<&RenderedGlyph> {
        self.map.get(c)
    }
}


#[allow(dead_code)]
pub struct Renderer {
    //font : Font<'static>,

    cache : FontCache,

    tip : Point<i32>,


    pub cur_search : Vec<char>,

    width : u32,
    height : u32,
    scale : Scale,

    fg : Color,
    bg : Color,
    hgl : Color,
}


#[allow(dead_code)]
impl Renderer {

    pub fn new(font : Font<'static>, conf : &Config, width : u32, height : u32) -> Self {
        let fg = conf.fontcolor.clone();
        let bg = conf.backgroundcolor.clone();
        let high = conf.highlight.clone();
        Renderer {
            tip : Point{x: 0, y: 0},
            cur_search : Vec::new(),
            width : width,
            height : height,
            scale : Scale::uniform(18.0),
            fg : fg,
            bg : bg,
            hgl : high,

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

    pub fn render_length(&mut self, content : &SearchElement) -> i32 {

        let content = content.search_string.chars();


        let width = content.fold(0, |acc, c| {
            acc + self.cache.font.glyph(c).scaled(self.scale).h_metrics().advance_width as i32
        });

        return width;
    }

    pub fn string_length(&self, content : &SearchElement) -> i32 {
        let string = content.search_string.clone();
        let v_metrics = self.cache.font.v_metrics(self.scale);

        let glyphs : Vec<_> = self.cache.font.layout(&string, self.scale, rusttype::point(0.0, 0.0 + v_metrics.ascent)).collect();
        let width = {
            let min = glyphs.first().map( |g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max = glyphs.last().map( |g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            max - min
        };
        width
    }

    pub fn render_full_image(&mut self, canvas : &mut [u8], results : Vec<SearchElement>, pos : usize) {

        //     self.cache.build_cache(self.cur_search.clone(), self.height);

        draw_rect(canvas, (self.width, self.height), 0, 0, self.width, self.height, self.bg.to_u8());

        let input_field = self.cur_search.iter().collect::<String>();

        let scale = rusttype::Scale::uniform(18.0);

        let v_metrics = self.cache.font.v_metrics(scale);

        let glyphs = self.cache.font.layout(&input_field, rusttype::Scale::uniform(18.0), rusttype::point(1.0 ,1.0 + v_metrics.ascent));


        //Drawin the prompt
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x,y,v| {
                    let x = x + bb.min.x as u32;
                    let y = y + bb.min.y as u32;
                    let idx = (x + y * self.width) as usize * 4;
                    canvas[idx..idx+4].copy_from_slice(&self.fg.interpolate(&self.bg, v));

                })
            }
        }


        let mut offset : i32 = 200; //Offset in Pixels

        let mut index = 0;
        let space = self.width as i32 - offset - 10;
        while (offset < space) && (results.len() > index)  {

            let item_width = self.render_length(&results[index]);

            if index == pos {
                draw_rect(canvas, (self.width, self.height), offset as u32, 0, item_width as u32, self.height, self.hgl.to_u8());
            }

            //println!("Item width: {item_width}");
            //println!("Point = {offset}");


            let glyphs = self.cache.font.layout(results[index].search_string.as_str(),self.scale, Point {x: offset as f32, y: v_metrics.ascent + 1.0 } );

            for glyph in glyphs {
                if let Some(bb) = glyph.pixel_bounding_box() {
                    glyph.draw(|x,y,v| {
                        let x = x + bb.min.x as u32; 
                        let y = y + bb.min.y as u32;
                        let id = (x + y * self.width) as usize * 4;
                        let idx = id;
                        if index == pos {
                    canvas[idx..idx+4].copy_from_slice(&self.fg.interpolate(&self.hgl, v));
                        } else {
                    canvas[idx..idx+4].copy_from_slice(&self.fg.interpolate(&self.bg, v));
                        }

                    })
                }
            }
            offset = offset + item_width + 10;
            index = index + 1;


        }

    }

#[deprecated]
    pub fn draw_full_optimised(&mut self, canvas : &mut [u8]) {

        self.cache.build_cache(self.cur_search.clone(), self.height);

        draw_rect(canvas, (self.width, self.height), 0, 0, self.width, self.height, self.bg.to_u8());

        let mut point : usize = (self.width *2) as usize;

        for c in self.cur_search.iter() {

            if let Some(values) = self.cache.draw_chached_char(c) {

                let content = values.content.as_ref();
                let width = values.stride;

                let rows = content.chunks_exact(4 *width  as usize);

                //assert!(rows.len() == 14);

                for (y,row) in rows.enumerate() {
                    let idx = ( y * self.width  as usize + point) * 4 as usize;
                    let end = idx+ (width*4 as u32) as usize;
                    canvas[idx..end].copy_from_slice(row);
                }
                point += width as usize;
            }
        }
    }
}

