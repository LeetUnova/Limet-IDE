use std::collections::BTreeMap;
use crate::Renderer;

pub struct JsRenderer;

impl JsRenderer {
    const STATIC_KEYWORD: &'static str = "\u{1b}[31mstatic\u{1b}[0m";
    const CLASS_KEYWORD: &'static str = "\u{1b}[34mclass\u{1b}[0m";
    const RETURN_KEYWORD: &'static str = "\u{1b}[34mreturn\u{1b}[0m";
    const L_BRACKET: &'static str = "\u{1b}[36m{\u{1b}[0m";
    const R_BRACKET: &'static str = "\u{1b}[36m}\u{1b}[0m";
    const L_PARENTH: &'static str = "\u{1b}[35m(\u{1b}[0m";
    const R_PARENTH: &'static str = "\u{1b}[35m)\u{1b}[0m";
    const LET_KEYWORD: &'static str = "\u{1b}[33mlet\u{1b}[0m";
    const VAR_KEYWORD: &'static str = "\u{1b}[33mvar\u{1b}[0m";
    const ARROW: &'static str = "\u{1b}[32m=>\u{1b}[0m";
}

impl Renderer for JsRenderer {
    fn draw_text(&self, text_map: &BTreeMap<u32, String>) {
        for line in text_map {
            self.draw_line(line);
        }
    }

    fn draw_line(&self, text: (&u32, &String)) {
        println!("{}\t{}", text.0, text.1
            .replace("static", JsRenderer::STATIC_KEYWORD)
            .replace("class", JsRenderer::CLASS_KEYWORD)
            .replace("return", JsRenderer::RETURN_KEYWORD)
            .replace("{", JsRenderer::L_BRACKET)
            .replace("}", JsRenderer::R_BRACKET)
            .replace("(", JsRenderer::L_PARENTH)
            .replace(")", JsRenderer::R_PARENTH)
            .replace("let", JsRenderer::LET_KEYWORD)
            .replace("var", JsRenderer::VAR_KEYWORD)
            .replace("=>", JsRenderer::ARROW)
        );
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
