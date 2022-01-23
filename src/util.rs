use alloc::vec::Vec;
use alloc::string::String;
use super::ffi::text_width;


pub fn split_text_lines(text: &str, width: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in text.split_ascii_whitespace() {
        let mut line_test = current_line.clone();

        line_test.push_str(" ");
        line_test.push_str(word);

        if text_width(&line_test) > width {
            result.push(current_line);

            current_line = String::from(word);
        } else {
            current_line = line_test;
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}
