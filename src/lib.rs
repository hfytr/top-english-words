//! # top-english-words
//!
//! Rust crate for retrieving the most frequently used words in the English language.
//! Useful for training AI, text compression, or testing word processing applications.
//!
//! Read the [documentation](https://docs.rs/top-english-words/) for more information.

mod functions;
mod word_list;

pub use functions::*;
pub use word_list::NUM_WORDS;

#[cfg(test)]
mod tests {
    use crate::{word_list::WORD_LIST, *};

    #[test]
    fn test_get_words() {
        for (i, s) in get_words::<&str>().iter().enumerate() {
            assert_eq!(*s, WORD_LIST[i]);
        }
    }

    #[test]
    fn test_get_words_range() {
        let words = get_words_range::<String>(..10).unwrap();
        assert_eq!(words.len(), 10);
        let words = get_words_range::<String>(..).unwrap();
        assert_eq!(words.len(), NUM_WORDS);

        // Check the function returns None on an invalid range
        let words_fail = get_words_range::<String>(..NUM_WORDS + 2);
        assert_eq!(words_fail, None);
        let words_fail = get_words_range::<String>(20..10);
        assert_eq!(words_fail, None);
        let words_fail = get_words_range::<String>(NUM_WORDS + 1..NUM_WORDS + 2);
        assert_eq!(words_fail, None);

        // Check that they're in alphabetical order for `get_words_range_a`
        let words_alpha = get_words_range_a::<String>(..).unwrap();
        assert!(words_alpha.windows(2).all(|w| w[1] >= w[0]));
    }

    #[test]
    fn test_get_word() {
        let word0 = get_word::<&str>(0).unwrap();
        let word999 = get_word::<&str>(NUM_WORDS - 1).unwrap();
        let word_fail = get_word::<String>(NUM_WORDS);

        assert_eq!(word0, crate::word_list::WORD_LIST[0]);
        assert_eq!(word999, crate::word_list::WORD_LIST[NUM_WORDS - 1]);
        assert_eq!(word_fail, None);
    }

    #[test]
    fn test_is_top_word() {
        assert!(is_top_word(WORD_LIST[0]));
        assert!(is_top_word(WORD_LIST[5]));
        assert!(!is_top_word("crepuscular"));
    }

    #[test]
    fn test_get_word_index() {
        assert_eq!(get_word_index(WORD_LIST[0]).unwrap(), 0);
        assert_eq!(get_word_index(WORD_LIST[5]).unwrap(), 5);
        assert_eq!(get_word_index("crepuscular"), None);
    }

    #[test]
    fn test_mutability() {
        let mut _word = get_word::<&str>(0).unwrap();
        _word = "this is bad";

        assert_ne!(get_word::<&str>(0).unwrap(), "this is bad");
    }
}
