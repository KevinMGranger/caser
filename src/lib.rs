use std::iter::FromIterator;

pub enum Case {
    // snake_case
    SnakeCase,
    // PascalCase
    PascalCase,
    // camelCase
    CamelCase,
    // Sentence case. First letter of first word, separated by `.`, '?' or `!`
    Sentence,
}

#[derive(Clone, Copy)]
enum SentenceState {
    StartOfSentence,
    StartOfWord,
    Other,
}

impl Case {
    pub fn transform(&self, string: &str) -> String {
        let mut chars: Vec<char> = vec![];
        return match self {
            Self::SnakeCase => {
                for ch in string.chars() {
                    if ch.is_uppercase() {
                        chars.push('_');
                        chars.extend(ch.to_lowercase());
                    } else {
                        chars.push(ch);
                    }
                }

                let first_is_underscore = chars.first().map_or(false, |ch| *ch == '_');
                if first_is_underscore {
                    String::from_iter(chars.into_iter().skip(1))
                } else {
                    String::from_iter(chars)
                }
            }
            Self::PascalCase => {
                let mut uppercase_next = false;
                for (i, ch) in string.chars().enumerate() {
                    if i == 0 {
                        chars.extend(ch.to_uppercase())
                    } else if ch == '_' {
                        uppercase_next = true;
                    } else if uppercase_next {
                        chars.extend(ch.to_uppercase());
                        uppercase_next = false;
                    } else {
                        chars.push(ch);
                    }
                }

                String::from_iter(chars)
            }
            Self::CamelCase => {
                let mut uppercase_next = false;
                for (i, ch) in string.chars().enumerate() {
                    if i == 0 {
                        chars.extend(ch.to_lowercase())
                    } else if ch == '_' {
                        uppercase_next = true;
                    } else if uppercase_next {
                        chars.extend(ch.to_uppercase());
                        uppercase_next = false;
                    } else {
                        chars.push(ch);
                    }
                }

                String::from_iter(chars)
            }
            Self::Sentence => {
                let mut state = SentenceState::StartOfSentence;
                for ch in string.chars() {
                    match (state, ch.is_whitespace()) {
                        (SentenceState::Other, true) => {
                            state = SentenceState::StartOfWord;
                            chars.push(ch);
                        }
                        (SentenceState::Other, false) => {
                            chars.push(ch);
                            if let '.' | '!' | '?' = ch {
                                state = SentenceState::StartOfSentence;
                            }
                        }
                        (SentenceState::StartOfSentence, true) => chars.push(ch),
                        (SentenceState::StartOfSentence, false) => {
                            chars.extend(ch.to_uppercase());
                            state = if let '.' | '!' | '?' = ch {
                                SentenceState::StartOfSentence
                            } else {
                                SentenceState::Other
                            };
                        }
                        (SentenceState::StartOfWord, true) => chars.push(ch),
                        (SentenceState::StartOfWord, false) => {
                            chars.extend(ch.to_lowercase());
                            state = if let '.' | '!' | '?' = ch {
                                SentenceState::StartOfSentence
                            } else {
                                SentenceState::Other
                            };
                        }
                    }
                }

                String::from_iter(chars)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Case::*;

    #[test]
    fn camel_to_snake() {
        let input = "camelToSnake";
        let expected = "camel_to_snake";
        assert_eq!(expected, SnakeCase.transform(input))
    }

    #[test]
    fn pascal_to_snake() {
        let input = "PascalToSnake";
        let expected = "pascal_to_snake";
        assert_eq!(expected, SnakeCase.transform(input))
    }

    #[test]
    fn snake_to_camel() {
        let input = "snake_to_camel";
        let expected = "snakeToCamel";
        assert_eq!(expected, CamelCase.transform(input))
    }

    #[test]
    fn pascal_to_camel() {
        let input = "PascalToCamel";
        let expected = "pascalToCamel";
        assert_eq!(expected, CamelCase.transform(input))
    }

    #[test]
    fn snake_to_pascal() {
        let input = "snake_to_pascal";
        let expected = "SnakeToPascal";
        assert_eq!(expected, PascalCase.transform(input))
    }

    #[test]
    fn camel_to_pascal() {
        let input = "camelToPascal";
        let expected = "CamelToPascal";
        assert_eq!(expected, PascalCase.transform(input))
    }

    #[test]
    fn sentence_fix() {
        let input = "What's The Deal With Airline Food? it's so plane.";
        let expected = "What's the deal with airline food? It's so plane.";
        assert_eq!(expected, Sentence.transform(input))
    }
}
