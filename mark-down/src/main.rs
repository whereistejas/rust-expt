use pulldown_cmark::{html, Parser};
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let mut md_file = File::open("some-markdown.md").expect("Could not open markdown file");

    let mut markdown_content = String::new();
    md_file
        .read_to_string(&mut markdown_content)
        .expect("Could not read markdown file");

    assert!(!markdown_content.is_empty());
    let parser = Parser::new(&markdown_content);

    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    let mut html_file = File::create("some-html.html").expect("Could not create HTML file.");
    html_file
        .write_all(html_content.as_bytes())
        .expect("Could not write HTML file.");
}
