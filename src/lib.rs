//! top-english-words
//! 
//! Rust crate for retrieving the most frequently used words in the English language
//! from a list of 1000 words. Useful for training AI, text compression, or testing
//! word processing applications.

mod functions;
mod word_list;

pub use functions::*;

pub const NUM_WORDS: usize = 1000;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_words_range() {
        let words = get_words_range::<String>(..10).unwrap();
        assert_eq!(words.len(), 10);
        let words = get_words_range::<String>(..).unwrap();
        assert_eq!(words.len(), NUM_WORDS);

        // Check the function returns None on an invalid range
        let words_fail = get_words_range::<String>(..1002);
        assert_eq!(words_fail, None);
        let words_fail = get_words_range::<String>(20..10);
        assert_eq!(words_fail, None);
        let words_fail = get_words_range::<String>(1000..1001);
        assert_eq!(words_fail, None);

        // Check that they're in alphabetical order for `get_words_range_a`
        let words_alpha = get_words_range_a::<String>(..).unwrap();
        assert!(words_alpha.windows(2).all(|w| w[1] >= w[0]));
    }

    #[test]
    fn test_get_word() {
        let word0 = get_word::<&str>(0).unwrap();
        let word999 = get_word::<&str>(999).unwrap();
        let word_fail = get_word::<String>(1000);

        assert_eq!(word0, crate::word_list::WORD_LIST[0]);
        assert_eq!(word999, crate::word_list::WORD_LIST[999]);
        assert_eq!(word_fail, None);
    }

    #[test]
    fn test_is_top_word() {
        assert!(is_top_word("the"));
        assert!(is_top_word("and"));
        assert!(!is_top_word("crepuscular"));
    }

    #[test]
    fn test_get_word_index() {
        assert_eq!(get_word_index("the").unwrap(), 0);
        assert_eq!(get_word_index("crepuscular"), None);
    }
}
