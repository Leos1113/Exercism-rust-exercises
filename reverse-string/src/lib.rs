use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reverse: String = input.graphemes(true).rev().collect();
    reverse
}
