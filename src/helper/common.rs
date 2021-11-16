use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Language {
    Czech,
    Danish,
    Dutch,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Italian,
    Norwegian,
    Polish,
    Portuguese,
    Slovenian,
    Spanish,
    Swedish,
    Turkish
}

impl FromStr for Language {

    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "Czech"      => Ok(Language::Czech),
            "Danish"     => Ok(Language::Danish),
            "Dutch"      => Ok(Language::Dutch),
            "English"    => Ok(Language::English),
            "Estonian"   => Ok(Language::Estonian),
            "Finnish"    => Ok(Language::Finnish),
            "French"     => Ok(Language::French),
            "German"     => Ok(Language::German),
            "Greek"      => Ok(Language::Greek),
            "Italian"    => Ok(Language::Italian),
            "Norwegian"  => Ok(Language::Norwegian),
            "Polish"     => Ok(Language::Polish),
            "Portuguese" => Ok(Language::Portuguese),
            "Slovenian"  => Ok(Language::Slovenian),
            "Spanish"    => Ok(Language::Spanish),
            "Swedish"    => Ok(Language::Swedish),
            "Turkish"    => Ok(Language::Turkish),
            _ => Err(())
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", {
            let input_string = format!("{:?}", self);
            let mut output: Vec<&str> = Vec::new();
            let mut start: usize = 0;

            for (c_pos, c) in input_string.char_indices() {
                if c.is_uppercase() {
                    if start < c_pos {
                        output.push(&input_string[start..c_pos]);
                    }
                    start = c_pos;
                }
            }
            if start < input_string.len() {
                output.push(&input_string[start..]);
            }
            output.join(" ")
        })
    }
}

impl Language {
    pub fn get_iso_639_1_code(&self) -> &'static str {
        match self {
            Language::Czech => "cs",
            Language::Danish => "da",
            Language::Dutch => "nl",
            Language::English => "en",
            Language::Estonian => "et",
            Language::Finnish => "fi",
            Language::French => "fr",
            Language::German => "de",
            Language::Greek => "el",
            Language::Italian => "it",
            Language::Norwegian => "no",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Slovenian => "sl",
            Language::Spanish => "es",
            Language::Swedish => "sv",
            Language::Turkish => "tr"
        }
    }
}