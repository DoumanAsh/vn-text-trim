extern crate regex;

extern crate config;

#[inline(always)]
///Determines whether text contains japanese
pub fn is_jp<T: AsRef<str>>(text: T) -> bool {
    let text = text.as_ref();
    text.chars().any(|elem_char| match elem_char { '\u{3000}'...'\u{303f}'| //punctuation
                                                   '\u{3040}'...'\u{309f}'| //hiragana
                                                   '\u{30a0}'...'\u{30ff}'| //katakana
                                                   '\u{ff00}'...'\u{ffef}'| //roman characters
                                                   '\u{4e00}'...'\u{9faf}'| //common kanji
                                                   '\u{3400}'...'\u{4dbf}'  //rare kanji
                                                      => true,
                                                   _  => false,
    })
}

pub struct Cleaner {
    config: config::Config
}

impl Cleaner {
    pub fn new(config: config::Config) -> Self {
        Cleaner {
            config
        }
    }

    pub fn extract_dialogue<'a>(&self, text: &'a str) -> Option<&'a str> {
        self.config.dialogue.re.captures(&text)
                               .and_then(|caps| caps.get(1))
                               .map(|cap| cap.as_str())
    }

    ///Replacer function to use, if there are replacement patterns.
    fn replace(&self, text: &str) -> Option<String> {
        let replaces = match self.config.replace.as_ref() {
            Some(replaces) => replaces,
            None => return None,
        };

        let original_len = text.len();
        let mut text = text.to_string();
        for replace in replaces {
            text = replace.pattern.replacen(&text, replace.limit, replace.replacement.as_str()).to_string();
        }

        if original_len == text.len() {
            None
        }
        else {
            Some(text.to_string())
        }
    }

    pub fn clean(&self, text: &str) -> Option<String> {
        self.replace(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_dialogue() {
        let config = super::config::Config::from_file(super::std::path::Path::new("../vn-text-trim.toml")).unwrap();
        let cleaner = super::Cleaner::new(config);

        let text = "「甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます」";
        let expected_result = "甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます";

        let result = cleaner.extract_dialogue(text).expect("To extract!");
        assert_eq!(result, expected_result);

        let text = "「 甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます 」";
        let expected_result = "甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます";

        let result = cleaner.extract_dialogue(text).expect("Extract dialogue with  white spaces");
        assert_eq!(result, expected_result);

        let text = "「　喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";
        let expected_result = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";

        let result = cleaner.extract_dialogue(text).expect("Extract partial dialogue");
        assert_eq!(result, expected_result);

        let text = "    \t 喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。  \n]";
        let expected_result = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。]";
        let result = cleaner.clean(text).expect("Should clean whitespace");
        assert_eq!(result, expected_result);

        let text = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";
        let result = cleaner.clean(text);
        assert!(result.is_none())
    }
}
