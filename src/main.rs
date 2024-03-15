use std::{collections::BTreeMap, env, fmt::Error, fs::File, io::{stdin, Read, Stdin, Write}, str::Split};
use crate::{java_renderer::JavaRenderer, js_renderer::JsRenderer, python_renderer::PythonRenderer, txt_renderer::TxtRenderer};
mod java_renderer;
mod python_renderer;
mod js_renderer;
mod txt_renderer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let text: String = read_file(&filename);
    let split: Split<'_, &str> = text.split("\n");
    let ext = filename.split(".").last()
        .or_else(|| Some("txt"))
        .expect("This should absolutely work.");

    let mut text_map: BTreeMap<u32, String> = BTreeMap::new();

    let mut line_num: u32 = 0;
    for line in split {
        if !is_whitespace(line) {
            text_map.insert(line_num, line.to_string());
        }
        line_num += 1;
    }

    let renderer: &dyn Renderer = if ext == "py" {
        &PythonRenderer
    }
    else if ext == "java" {
        &JavaRenderer
    } else if ext == "js" {
        &JsRenderer
    } else {
        &TxtRenderer
    };

    let bar: &str = "------------------------------";

    println!("{}", bar);
    renderer.draw_text(&text_map);
    println!("{}", bar);

    let input: Stdin = stdin();

    loop {
        let mut input_text: String = String::new();

        input.read_line(&mut input_text)
            .expect("Yo");

        input_text = input_text.replace("\t", "    ");

        if input_text.starts_with("quit") {
            break;
        }

        let rewind: usize = text_map.len() + 3;
        println!("\u{1b}[{}A", rewind);
        renderer.blank_text(&text_map);

        if input_text.starts_with("save") {
            save(filename, &text_map);
        }
        else {
            modify_text(input_text, &mut text_map);

            text_map.retain(|_: &u32, v: &mut String| !is_whitespace(v));
        }

        println!("\u{1b}[{}A{}", rewind, bar);
        renderer.draw_text(&text_map);
        println!("{}", bar);
    }
}

fn save(filename: &String, text_map: &BTreeMap<u32, String>) {
    let mut file: File = File::create(filename)
        .expect("come on work");

    let mut line_num = 0;

    for line in text_map {
        while line_num < *line.0 {
            file.write_all("\n".as_bytes())
                .expect("work dang it");
            line_num += 1;
        }

        file.write_all(line.1.as_bytes())
            .expect("Lets a go");
    }
}

fn is_whitespace(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }

    for l in text.chars() {
        if !l.is_whitespace() {
            return false;
        }
    }

    true
}

fn modify_text(input_text: String, text_map: &mut BTreeMap<u32, String>) {
    let binding: &str = input_text.trim_start();
    let text: Split<'_, &str> = binding.split(" ");
    let first: &str = text.into_iter().next().expect("msg");

    let num: u32 = first.parse::<u32>()
        .expect("shz");

    text_map.insert(num, binding
        .split((first.to_string() + " ").as_str())
        .into_iter()
        .nth(1)
        .expect("yo")
        .replace("\n", "")
        .to_string()
    );
}

fn read_file(filename: &String) -> String {
    let mut file: File = File::open(filename)
        .or_else(|_| File::create(filename))
        .expect("yo");

    let mut text: String = "".to_string();

    file.read_to_string(&mut text)
        .or_else(|_| Result::<usize, Error>::Ok(0))
        .expect("ok");

    text
}

trait Renderer {
    fn draw_text(&self, text_map: &BTreeMap<u32, String>);
    fn draw_line(&self, text: (&u32, &String));
    fn blank_text(&self, text_map: &BTreeMap<u32, String>);
    fn blank_line(&self);
}
