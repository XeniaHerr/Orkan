use core::panic;
use std::fmt::Display;



#[derive(Debug)]
pub struct Color {

    r : u8,
    g : u8,
    b: u8,
    a: u8,
}


impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a}
    }

    pub fn interpolate( &self, background : &Color, exposure : f32) -> [u8; 4] {


    let ret =  [ (self.r as f32 * exposure + background.r as f32 * (1.0 - exposure)) as u8,
                            (self.g as f32 * exposure + background.g as f32 * (1.0 - exposure)) as u8,
                            (self.b as f32 * exposure + background.b as f32 * (1.0 - exposure)) as u8,
                            self.a as u8];
    return ret;
    }


    pub fn to_u8(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn to_u8_mut(&mut self) -> [ &mut u8; 4] {
        [& mut self.r, &mut self.g, &mut self.b, &mut self.a]
    }

    pub fn from_hex_string(s : &str) -> Result<Color, std::num::ParseIntError> {
        if s.len() != 6 {
            panic!("Hex String not valid");

        }

        Ok(Color {
            r : u8::from_str_radix(&s[4..6], 16).unwrap(),
            g : u8::from_str_radix(&s[2..4], 16).unwrap(),
            b : u8::from_str_radix(&s[0..2], 16).unwrap(),
            a : 0xff
        })
    }

}


impl Clone for Color {

    fn clone(&self) -> Self {
        Color {
            r : self.r.clone(),
            g : self.g.clone(),
            b : self.b.clone(),
            a : self.a.clone()
        }
    }
}


impl Display for Color {


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
