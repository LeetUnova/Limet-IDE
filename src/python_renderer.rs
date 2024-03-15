use std::collections::BTreeMap;
use crate::Renderer;

pub struct PythonRenderer;

impl PythonRenderer {
    const DEF_KEYWORD: &'static str = "\u{1b}[31mdef\u{1b}[0m";
    const SELF_KEYWORD: &'static str = "\u{1b}[34mself\u{1b}[0m";
    const COLON: &'static str = "\u{1b}[32m:\u{1b}[0m";
    const STATIC_KEYWORD: &'static str = "\u{1b}[31mstatic\u{1b}[0m";
    const CLASS_KEYWORD: &'static str = "\u{1b}[34mclass\u{1b}[0m";
    const RETURN_KEYWORD: &'static str = "\u{1b}[34mreturn\u{1b}[0m";
    const L_BRACKET: &'static str = "\u{1b}[36m{\u{1b}[0m";
    const R_BRACKET: &'static str = "\u{1b}[36m}\u{1b}[0m";
    const L_PARENTH: &'static str = "\u{1b}[35m(\u{1b}[0m";
    const R_PARENTH: &'static str = "\u{1b}[35m)\u{1b}[0m";
    const INT_PRIMITIVE: &'static str = "\u{1b}[33mint\u{1b}[0m";
    const STRING_PRIMITIVE: &'static str = "\u{1b}[33mstr\u{1b}[0m";
    const FLOAT_PRIMITIVE: &'static str = "\u{1b}[33mfloat\u{1b}[0m";
    const BOOLEAN_PRIMITIVE: &'static str = "\u{1b}[33mbool\u{1b}[0m";
    const NONE_KEYWORD: &'static str = "\u{1b}[33mnone\u{1b}[0m";
    const LAMBDA_KEYWORD: &'static str = "\u{1b}[32mlamda\u{1b}[0m";
}

impl Renderer for PythonRenderer {
    fn draw_text(&self, text_map: &BTreeMap<u32, String>) {
        for line in text_map {
            self.draw_line(line);
        }
    }

    fn draw_line(&self, text: (&u32, &String)) {
        println!("{}\t{}", text.0, text.1
            .replace("def", PythonRenderer::DEF_KEYWORD)
            .replace("self", PythonRenderer::SELF_KEYWORD)
            .replace(":", PythonRenderer::COLON)
            .replace("static", PythonRenderer::STATIC_KEYWORD)
            .replace("class", PythonRenderer::CLASS_KEYWORD)
            .replace("return", PythonRenderer::RETURN_KEYWORD)
            .replace("{", PythonRenderer::L_BRACKET)
            .replace("}", PythonRenderer::R_BRACKET)
            .replace("(", PythonRenderer::L_PARENTH)
            .replace(")", PythonRenderer::R_PARENTH)
            .replace("int", PythonRenderer::INT_PRIMITIVE)
            .replace("str", PythonRenderer::STRING_PRIMITIVE)
            .replace("bool", PythonRenderer::BOOLEAN_PRIMITIVE)
            .replace("float", PythonRenderer::FLOAT_PRIMITIVE)
            .replace("none", PythonRenderer::NONE_KEYWORD)
            .replace("lambda", PythonRenderer::LAMBDA_KEYWORD)
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
