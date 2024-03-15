use std::collections::BTreeMap;
use crate::Renderer;

pub struct JavaRenderer;

impl JavaRenderer {
    const PUBLIC_KEYWORD: &'static str = "\u{1b}[31mpublic\u{1b}[0m";
    const PRIVATE_KEYWORD: &'static str = "\u{1b}[31mprivate\u{1b}[0m";
    const PROTECTED_KEYWORD: &'static str = "\u{1b}[31mprotected\u{1b}[0m";
    const STATIC_KEYWORD: &'static str = "\u{1b}[31mstatic\u{1b}[0m";
    const CLASS_KEYWORD: &'static str = "\u{1b}[34mclass\u{1b}[0m";
    const RETURN_KEYWORD: &'static str = "\u{1b}[34mreturn\u{1b}[0m";
    const L_BRACKET: &'static str = "\u{1b}[36m{\u{1b}[0m";
    const R_BRACKET: &'static str = "\u{1b}[36m}\u{1b}[0m";
    const L_PARENTH: &'static str = "\u{1b}[35m(\u{1b}[0m";
    const R_PARENTH: &'static str = "\u{1b}[35m)\u{1b}[0m";
    const INT_PRIMITIVE: &'static str = "\u{1b}[33mint\u{1b}[0m";
    const CHAR_PRIMITIVE: &'static str = "\u{1b}[33mchar\u{1b}[0m";
    const FLOAT_PRIMITIVE: &'static str = "\u{1b}[33mfloat\u{1b}[0m";
    const BOOLEAN_PRIMITIVE: &'static str = "\u{1b}[33mboolean\u{1b}[0m";
    const DOUBLE_PRIMITIVE: &'static str = "\u{1b}[33mdouble\u{1b}[0m";
    const SHORT_PRIMITIVE: &'static str = "\u{1b}[33mshort\u{1b}[0m";
    const LONG_PRIMITIVE: &'static str = "\u{1b}[33mlong\u{1b}[0m";
    const BYTE_PRIMITIVE: &'static str = "\u{1b}[33mbyte\u{1b}[0m";
    const VOID_KEYWORD: &'static str = "\u{1b}[33mvoid\u{1b}[0m";
    const ARROW: &'static str = "\u{1b}[32m->\u{1b}[0m";
}

impl Renderer for JavaRenderer {
    fn draw_text(&self, text_map: &BTreeMap<u32, String>) {
        for line in text_map {
            self.draw_line(line);
        }
    }

    fn draw_line(&self, text: (&u32, &String)) {
        println!("{}\t{}", text.0, text.1
            .replace("public", JavaRenderer::PUBLIC_KEYWORD)
            .replace("private", JavaRenderer::PRIVATE_KEYWORD)
            .replace("protected", JavaRenderer::PROTECTED_KEYWORD)
            .replace("static", JavaRenderer::STATIC_KEYWORD)
            .replace("class", JavaRenderer::CLASS_KEYWORD)
            .replace("return", JavaRenderer::RETURN_KEYWORD)
            .replace("{", JavaRenderer::L_BRACKET)
            .replace("}", JavaRenderer::R_BRACKET)
            .replace("(", JavaRenderer::L_PARENTH)
            .replace(")", JavaRenderer::R_PARENTH)
            .replace("int", JavaRenderer::INT_PRIMITIVE)
            .replace("char", JavaRenderer::CHAR_PRIMITIVE)
            .replace("boolean", JavaRenderer::BOOLEAN_PRIMITIVE)
            .replace("float", JavaRenderer::FLOAT_PRIMITIVE)
            .replace("double", JavaRenderer::DOUBLE_PRIMITIVE)
            .replace("long", JavaRenderer::LONG_PRIMITIVE)
            .replace("short", JavaRenderer::SHORT_PRIMITIVE)
            .replace("byte", JavaRenderer::BYTE_PRIMITIVE)
            .replace("void", JavaRenderer::VOID_KEYWORD)
            .replace("->", JavaRenderer::ARROW)
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
