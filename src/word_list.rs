use include_lines::{count_lines, pub_static_include_lines};

/// The total number of words that can be retrieved from the library.
pub const NUM_WORDS: usize = count_lines!("words.txt");

pub_static_include_lines!(WORD_LIST, "words.txt");
