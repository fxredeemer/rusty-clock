use embedded_graphics::fonts::{Font8x16, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyle;
use epd_waveshare::epd2in9bc::Display2in9bc;

const MARGIN: i32 = 0;
const FONT_HEIGHT: i32 = 16;
const FONT_WIDTH: i32 = 8;
const BOTTOM_Y: i32 = 128 - MARGIN - FONT_HEIGHT;

pub struct Header<'a> {
    display: &'a mut Display2in9bc,
}

impl<'a> Header<'a> {
    pub fn new(display: &'a mut Display2in9bc) -> Self {
        Self { display }
    }

    pub fn top_left(&mut self, s: &str) {
        self.render(s, MARGIN, MARGIN);
    }

    pub fn _top_center(&mut self, s: &str) {
        let len = s.chars().count() as i32;
        self.render(s, 296 / 2 - FONT_WIDTH * len / 2, MARGIN);
    }

    pub fn top_right(&mut self, s: &str) {
        let len = s.chars().count() as i32;
        self.render(s, 295 - MARGIN - FONT_WIDTH * len, MARGIN);
    }

    pub fn bottom_left(&mut self, s: &str) {
        self.render(s, MARGIN, BOTTOM_Y);
    }

    pub fn _bottom_center(&mut self, s: &str) {
        let len = s.chars().count() as i32;
        self.render(s, 296 / 2 - FONT_WIDTH * len / 2, BOTTOM_Y);
    }

    pub fn bottom_right(&mut self, s: &str) {
        let len = s.chars().count() as i32;
        self.render(s, 295 - MARGIN - FONT_WIDTH * len, BOTTOM_Y);
    }

    fn render(&mut self, s: &str, x: i32, y: i32) {
        Text::new(&s, Point::new(x, y))
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On))
            .draw(self.display)
            .unwrap();
    }
}
