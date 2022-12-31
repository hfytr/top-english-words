use include_lines::{count_lines, include_lines};

/// The total number of words that can be retrieved from the library.
pub const NUM_WORDS: usize = count_lines!("words.txt");

pub static WORD_LIST: [&str; NUM_WORDS] = include_lines!("words.txt");
