use pulldown_cmark::{html, Options, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn serialize(html_input: &str) -> String {
  unimplemented!()
}

#[wasm_bindgen]
pub fn deserialize(markdown_input: &str) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(markdown_input, options);

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  html_output
}
