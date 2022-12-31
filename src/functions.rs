use std::ops::{Bound, RangeBounds};

use crate::word_list::WORD_LIST;
use crate::NUM_WORDS;

/// Get all the words from the list of top English words.
///
/// The number of words returned in the list is [NUM_WORDS].
pub fn get_words<T>() -> Vec<T>
where
    T: From<&'static str>,
{
    let mut word_vec = vec![];
    word_vec.reserve_exact(NUM_WORDS);

    for word in WORD_LIST {
        word_vec.push(T::from(word));
    }

    word_vec
}

/// Get words from the list of top English words that fall into the given [range](https://doc.rust-lang.org/reference/expressions/range-expr.html).
///
/// The words will be ordered by their rank in the list. If the range is invalid,
/// this function returns [None].
///
/// # Example
///
/// ```
/// use top_english_words::get_words_range;
///
/// let first_5_words = get_words_range::<String>(..5).unwrap();
/// ```
pub fn get_words_range<T>(range: impl RangeBounds<usize>) -> Option<Vec<T>>
where
    T: From<&'static str>,
{
    let start_index = match range.start_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(_) => 0,
        Bound::Unbounded => 0,
    };

    let end_index = match range.end_bound() {
        Bound::Included(i) => *i,
        Bound::Excluded(i) => {
            if *i > 0 {
                i - 1
            } else {
                return None;
            }
        }
        Bound::Unbounded => NUM_WORDS - 1,
    };

    if start_index > end_index || end_index >= NUM_WORDS {
        return None;
    }

    let mut word_vec = vec![];
    word_vec.reserve_exact(end_index - start_index + 1);

    for word in WORD_LIST.iter().take(end_index + 1).skip(start_index) {
        word_vec.push(T::from(word));
    }

    Some(word_vec)
}

/// Get words from the list of top English words that fall into the given [range](https://doc.rust-lang.org/reference/expressions/range-expr.html).
///
/// The words will be ordered alphabetically. If the range is invalid,
/// this function returns [None].
///
/// # Example
///
/// ```
/// use top_english_words::get_words_range_a;
///
/// let first_5_words = get_words_range_a::<String>(..5).unwrap();
/// ```
pub fn get_words_range_a<T>(range: impl RangeBounds<usize>) -> Option<Vec<T>>
where
    T: From<&'static str> + Ord,
{
    let mut words_vec = get_words_range::<T>(range)?;
    words_vec.sort();

    Some(words_vec)
}

/// Get a word from the list of top English words.
///
/// If the index is invalid, this function returns [None].
pub fn get_word<T>(position: usize) -> Option<T>
where
    T: From<&'static str>,
{
    Some(T::from(WORD_LIST.get(position)?))
}

/// Check if the given word is in the list of top English words.
pub fn is_top_word(word: &str) -> bool {
    WORD_LIST.contains(&word)
}

/// Given a word, get its index in the list.
///
/// Note that the list is sorted by how frequently their used. Lower indices
/// mean that a word is used more frequently than another.
///
/// If the word is not present in the list, this function returns [None].
pub fn get_word_index(word: &str) -> Option<usize> {
    WORD_LIST.iter().position(|w| *w == word)
}
