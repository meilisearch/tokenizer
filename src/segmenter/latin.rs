use unicode_segmentation::UnicodeSegmentation;

use super::Segmenter;

/// Latin specialized [`Segmenter`].
///
/// This Segmenter uses [`UnicodeSegmentation`] internally to segment the provided text.
pub struct LatinSegmenter;

impl Segmenter for LatinSegmenter {
    fn segment_str<'a>(&self, s: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(s.split_word_bounds())
    }
}

#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    const TEXT: &str = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
    const SEGMENTED: &[&str] = &[
        "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can't", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it's",
        " ", "29.3", "°", "F", "!",
    ];
    const TOKENIZED: &[&str] = &[
        "the", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can't", " ",
        "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "brr", ",", " ", "it's",
        " ", "29.3", "°", "f", "!",
    ];

    test_segmenter!(LatinSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Latin, Language::Other);
}