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

#[derive(Debug)]
pub struct Cleaner {
    config: config::Config
}

impl Cleaner {
    pub fn new(config: config::Config) -> Self {
        Cleaner {
            config
        }
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

    pub fn remove_text_reps(&self, text: &str) -> Option<String> {
        let chars = text.chars().collect::<Vec<_>>();
        let mut pred = Vec::new();

        for idx in 1..chars.len() {
            pred.push(chars[idx - 1]);

            if chars[idx..].starts_with(&pred) {
                for r_idx in (1..chars.len()).rev() {
                    if chars[r_idx..].starts_with(&pred) {
                        return Some(chars[r_idx..].iter().fold(String::new(), |mut acc, ch| { acc.push(*ch); acc}));
                    }
                }
            }
        }

        None
    }

    pub fn clean(&self, text: &str) -> Option<String> {
        match self.config.text_repetitions {
            true => self.replace(text).map(|text| self.remove_text_reps(&text).unwrap_or(text)).or_else(|| self.remove_text_reps(text)),
            false => self.replace(text)
        }
    }
}

#[cfg(test)]
mod tests {
    fn get_cleaner() -> super::Cleaner {
        let config = super::config::Config::from_file(super::std::path::Path::new("../vn-text-trim.toml")).unwrap();
        super::Cleaner::new(config)
    }

    #[test]
    fn clean_nothing() {
        let cleaner = get_cleaner();

        let text = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";
        let result = cleaner.clean(text);
        assert!(result.is_none())
    }

    #[test]
    fn clean_text1() {
        let cleaner = get_cleaner();

        let text = "「甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます」";
        let expected_result = "甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます";

        let result = cleaner.clean(text).expect("To extract!");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn clean_text2() {
        let cleaner = get_cleaner();

        let text = "「 甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます 」";
        let expected_result = "甘いものは別腹と言いますから。私も見なかったこと作戦で食べちゃいます";

        let result = cleaner.clean(text).expect("Extract dialogue with white spaces");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn clean_text3() {
        let cleaner = get_cleaner();

        let text = "「　喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";
        let expected_result = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。";

        let result = cleaner.clean(text).expect("Extract partial dialogue");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn clean_text4() {
        let cleaner = get_cleaner();

        let text = "    \t 喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。  \n]";
        let expected_result = "喫茶店の裏手にある自宅に戻ってきた俺は、食後の一休みをとっていた。]";
        let result = cleaner.clean(text).expect("Should clean whitespace");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn clean_text5() {
        let cleaner = get_cleaner();

        let text = "「　青二「一真、父さんは伝説のコーヒー豆を求めて旅に出る！」";
        let expected_result = "一真、父さんは伝説のコーヒー豆を求めて旅に出る！";
        let result = cleaner.clean(text).expect("Should extract_dialogue");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn clean_text6() {
        let cleaner = get_cleaner();

        //TODO: Find a way to remove repetitions.
        let text = "御館様の想定通り、信濃勢は御館様の想定通り、信濃勢は徹底抗戦の御館様の想定通り、信濃勢は徹底抗戦の構えを見御館様の想定通り、信濃勢は徹底抗戦の構えを見せた。";
        let result = cleaner.clean(text).unwrap();
        println!("{}", result);

        let text = "この麗し<color=#ffffff24>き</color>";
        let expected_result = "この麗しき";
        let result = cleaner.clean(text).unwrap();
        assert_eq!(result, expected_result);
    }
}
