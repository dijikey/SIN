use crate::graphics::Color;

impl Color{
    #[inline]
    pub const fn from_rgba(r:u8, g:u8, b:u8, a:u8) -> Self{ Self{ r, g, b, a } }
    #[inline]
    pub const fn from_rgb(r:u8, g:u8, b:u8) -> Self{ Self{ r, g, b, a: 255} }
    #[inline]
    pub const fn from_u8(rgba: &[u8]) -> Self{
        Self{ 
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        } 
    }
    #[inline]
    pub const fn unpack(self) -> [u8; 4]{ [self.r, self.g, self.b, self.a]  }
}

impl Color{
    pub const RED: Color = Color{ r:255, g:0, b:0, a:255};
    pub const BLUE: Color = Color{ r:0, g:0, b:255, a:255};
    pub const GREEN: Color = Color{ r:0, g:255, b:0, a:255};
    pub const WHITE: Color = Color{ r:255, g:255, b:255, a:255};
    pub const BLACK: Color = Color{ r:0, g:0, b:0, a:255};
    pub const TRANSPARENT: Color = Color{ r:0, g:0, b:0, a:0};
}