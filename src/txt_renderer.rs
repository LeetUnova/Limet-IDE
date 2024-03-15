use std::collections::BTreeMap;
use crate::Renderer;

pub struct TxtRenderer;

impl Renderer for TxtRenderer {
    fn draw_text(&self, text_map: &BTreeMap<u32, String>) {
        for line in text_map {
            self.draw_line(line);
        }
    }

    fn draw_line(&self, text: (&u32, &String)) {
        println!("{}\t{}", text.0, text.1);
    }
    
    fn blank_text(&self, text_map: &BTreeMap<u32, String>) {
        for _ in 0..(text_map.len() + 2) {
            self.blank_line();
        }
    }
    
    fn blank_line(&self) {
        println!("\u{1b}[2K");
    }
}
