use fancy_regex::*;
use std::ops::{BitOr};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

static _ORTHO_BEG_UC: usize = 1 << 1;
static _ORTHO_MID_UC: usize = 1 << 2;
static _ORTHO_UNK_UC: usize = 1 << 3;
static _ORTHO_BEG_LC: usize = 1 << 4;
static _ORTHO_MID_LC: usize = 1 << 5;
static _ORTHO_UNK_LC: usize = 1 << 6;

static _ORTHO_UC: usize = _ORTHO_BEG_UC + _ORTHO_MID_UC + _ORTHO_UNK_UC;
static _ORTHO_LC: usize = _ORTHO_BEG_LC + _ORTHO_MID_LC + _ORTHO_UNK_LC;


#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Collocations (
    String,
    String,
);

impl Eq for Collocations {}

impl PartialEq for Collocations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PunktParameters {
    pub(crate) abbrev_types: HashSet<String>,
    pub(crate) collocations: HashSet<Collocations>,
    pub(crate) sent_starters: HashSet<String>,
    pub(crate) ortho_context: HashMap<String, usize>,
}

impl PunktParameters {

    fn new() -> Self {
        PunktParameters {
            abbrev_types: HashSet::new(),
            collocations: HashSet::new(),
            sent_starters: HashSet::new(),
            ortho_context: HashMap::new(),
        }
    }

    fn clear_abbrevs(&mut self){
        self.abbrev_types.clear()
    }

    fn clear_collocations(&mut self){
        self.collocations.clear()
    }

    fn clear_sent_starters(&mut self){
        self.sent_starters.clear()
    }

    fn clear_ortho_context(&mut self){
        self.ortho_context.clear()
    }

    fn add_ortho_context(&mut self, typ: &str, flag: usize)  {
        self.ortho_context.insert(
            typ.to_string(),
            flag.bitor(self.ortho_context.get(typ).unwrap_or(&0))
        );
    }

    fn get_ortho_context(&self, key: &str) -> usize {
        *self.ortho_context.get(key).unwrap_or(&0)
    }
}

#[derive(Debug, Clone)]
pub struct PunktLanguagePros {
    _word_tokenizer_re: Regex,
    period_context_re: Regex,
}

impl PunktLanguagePros {
    fn new(
        word_tokenize_fmt: &str,
        re_non_word_chars: &str,
        re_multi_char_punct: &str,
        re_word_start: &str,
        period_context_fmt: &str,
        re_sent_end_chars: &str,
    ) -> Self {
        PunktLanguagePros {
            _word_tokenizer_re: Self::build_word_tokenizer_re(word_tokenize_fmt, re_non_word_chars, re_multi_char_punct, re_word_start),
            period_context_re: Self::build_period_context_re(period_context_fmt, re_sent_end_chars, re_non_word_chars),
        }
    }

    fn build_word_tokenizer_re(
        word_tokenize_fmt: &str,
        re_non_word_chars: &str,
        re_multi_char_punct: &str,
        re_word_start: &str
    ) -> Regex {
        fancy_regex::Regex::new(
         &word_tokenize_fmt
            .replace("{NonWord}", re_non_word_chars)
            .replace("{MultiChar}", re_multi_char_punct)
            .replace("{WordStart}", re_word_start)
        ).unwrap()
    }

    fn build_period_context_re(
        period_context_fmt: &str,
        re_sent_end_chars: &str,
        re_non_word_chars: &str,
    ) -> fancy_regex::Regex {
        fancy_regex::Regex::new(
        &period_context_fmt
            .replace("{SentEndChars}", re_sent_end_chars)
            .replace("{NonWord}", re_non_word_chars)
        ).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PunktLanguageStaticVars<'a> {
    _re_sent_end_chars: String,
    _re_non_word_chars: String,
    sent_end_chars: HashSet<&'a str>,
    internal_punctuation: &'static str,
    re_boundary_realignment: Regex,
    _re_word_start: &'static str,
    _re_multi_char_punct: &'static str,
    _word_tokenize_fmt: &'static str,
    _period_context_fmt: &'static str,
}

impl <'a>PunktLanguageStaticVars<'a> {
    fn new(sent_end_chars: &HashSet<&'a str>) -> Self {
        PunktLanguageStaticVars {
            _re_sent_end_chars: Self::build_re_sent_end_chars(sent_end_chars),
            _re_non_word_chars: Self::build_re_non_word_chars(sent_end_chars),
            sent_end_chars: sent_end_chars.clone(),
            internal_punctuation: ",:;",
            re_boundary_realignment: Regex::new(r#"^["\')\]}]+?(?:\s+|(?=--)|$)"#).unwrap(),
            _re_word_start: r#"[^\(\"\`{\[:;&\#\*@\)}\]\-,]"#,
            _re_multi_char_punct: r#"(?:\-{2,}|\.{2,}|(?:\.\s){2,}\.)"#,
            _word_tokenize_fmt: r#"(?x) ({MultiChar} | (?={WordStart})\S+?(?=\s|$| {NonWord} | {MultiChar} | ,(?=$|\s|{NonWord}| {MultiChar})) | \S)"#,
            _period_context_fmt: r#"(?x) \S* {SentEndChars} (?=(?P<after_tok> {NonWord} | \s+ (?P<next_tok> \S+ )))"#,
        }
    }

    fn build_re_sent_end_chars(sent_end_chars: &HashSet<&str>) -> String {
        format!(
            "[{escaped_string}]",
            escaped_string=fancy_regex::escape(&sent_end_chars.into_iter().map(|x|*x).collect::<Vec<&str>>().join(""))
        )
    }

    fn build_re_non_word_chars(sent_end_chars: &HashSet<&str>) -> String {
        String::from(r#"(?:[;)}"\]*:@'({\[{string}])"#).replace(
            "{string}",
            &fancy_regex::escape(&sent_end_chars.into_iter().map(|x|*x).collect::<Vec<&str>>().join("").replace(".", ""))
        )
    }
}

#[derive(Debug, Clone)]
pub struct PunktLanguageVars<'a> {
    punkt_language_static_vars: PunktLanguageStaticVars<'a>,
    punkt_language_pros: PunktLanguagePros,
}

impl <'a>PunktLanguageVars<'a> {
    fn new() -> Self {
        let sec = HashSet::from([".", "?", "!"]);
        let punkt_language_static_vars = PunktLanguageStaticVars::new(&sec);
        let punkt_language_pros = PunktLanguagePros::new(
                &punkt_language_static_vars._word_tokenize_fmt,
                &punkt_language_static_vars._re_non_word_chars,
                &punkt_language_static_vars._re_multi_char_punct,
                &punkt_language_static_vars._re_word_start,
                &punkt_language_static_vars._period_context_fmt,
                &punkt_language_static_vars._re_sent_end_chars,
        );
        Self {
            punkt_language_static_vars,
            punkt_language_pros
        }
    }

    fn word_tokenize<'r, 't>(&self, string: String) -> Vec<String> {
        self.punkt_language_pros._word_tokenizer_re.find_iter(&string).into_iter().flatten().map(|m|String::from(m.as_str())).collect::<Vec<String>>()
    }
}


#[derive(Debug, Clone)]
pub struct PunktToken {
    token: String,
    type_: String,
    period_final: bool,
    para_start: Option<bool>,
    line_start: Option<bool>,
    sent_break: Option<bool>,
    abbr: Option<bool>,
    ellipsis: Option<bool>
}

impl PunktToken {
    fn new(token: &str, para_start: bool, line_start: bool) -> Self {
        Self {
            token: token.to_string(),
            type_: Self::_get_type(token),
            period_final: token.ends_with("."),
            para_start: Some(para_start),
            line_start: Some(line_start),
            sent_break: None,
            abbr: None,
            ellipsis: None
        }
    }

    fn _get_type(tok: &str) -> String {
        fancy_regex::Regex::new(r"(?xm) ^-?[\.,]?\d[\d,\.-]*\.?$")
            .unwrap()
            .replace(&tok.to_lowercase(), "##number##")
            .to_string()
    }

    fn type_no_period(&self) -> String {
        if self.type_.len() > 1 && self.type_.chars().last().unwrap() == '.' {
            return self.type_[0..self.type_.len()-1].to_string()
        }
        return self.type_.clone()
    }

    fn type_no_sent_period(&self) -> String {
        return match self.sent_break {
            Some(true) => self.type_no_period(),
            _ => self.type_.clone()
        }
    }

    fn first_upper(&self) -> bool {
        self.tok.chars().nth(0).unwrap().is_uppercase()
    }

    fn first_lower(&self) -> bool {
        self.tok.chars().nth(0).unwrap().is_lowercase()
    }

    fn first_case(&self) -> &'static str {
        if self.first_lower() {
            return "lower"
        } else if self.first_upper() {
            return "lower"
        }
        return "none"
    }

    fn is_ellipsis(&self) -> bool {
        fancy_regex::Regex::new(r"^(\.\.+$)")
            .unwrap()
            .find(&self.tok).unwrap().is_some()
    }

    fn is_number(&self) -> bool {
        self.tok.starts_with("##number##")
    }

    fn is_initial(&self) -> bool {
        fancy_regex::Regex::new(r"^([^\W\d]\.$)")
            .unwrap()
            .find(&self.tok).unwrap().is_some()
    }
}

#[cfg(test)]
mod punkt_parameters_tests {

    use std::collections::{HashMap, HashSet};
    use crate::tokenize::punkt::{PunktParameters, Collocations, PunktLanguageVars};

    pub fn get_static_data() -> PunktParameters {
        PunktParameters {
            abbrev_types: HashSet::from([String::from("ok"), String::from("a.g"), String::from("a.m")]),
            collocations: HashSet::from([Collocations(String::from("b"), String::from("wigton")), Collocations(String::from("o"), String::from("ludcke"))]),
            sent_starters: HashSet::from([String::from("since"), String::from("among"), String::from("they")]),
            ortho_context: HashMap::from([(String::from("a"), 126), (String::from("a&m"), 4), (String::from("a-%"), 32)])
        }
    }

    static WEIGHT: &str  = r#"
        {
            "abbrev_types": ["ok", "a.g", "a.m"],
            "collocations": [["b", "wigton"], ["o", "ludcke"]],
            "sent_starters": ["since", "among", "they"],
            "ortho_context": { "a": 126, "a&m": 4, "a-%": 32 }
        }
    "#;

    #[test]
    fn test() {
        let string = String::from("A last thing to note about key sentences is that academic readers expect them to be at the beginning of the paragraph");
        let sec = PunktLanguageVars::new();
        let data = sec.word_tokenize(string);
        println!("{:?}", data);
    }

    #[test]
    fn test_load_obj() {
        assert_eq!(get_static_data(), serde_json::from_str(WEIGHT).expect("Can't load from string"));
    }

    #[test]
    fn test_new_obj() {

        let pp = PunktParameters::new();

        assert_eq!(pp.abbrev_types.len(), 0);
        assert_eq!(pp.collocations.len(), 0);
        assert_eq!(pp.sent_starters.len(), 0);
        assert_eq!(pp.ortho_context.len(), 0);
    }

    #[test]
    fn test_clear_abbrevs() {

        let mut from_struct: PunktParameters = get_static_data();
        from_struct.clear_abbrevs();

        assert_eq!(from_struct.abbrev_types.len(), 0);
    }

    #[test]
    fn test_clear_collocations() {

        let mut from_struct: PunktParameters = get_static_data();
        from_struct.clear_collocations();

        assert_eq!(from_struct.collocations.len(), 0);
    }

    #[test]
    fn test_clear_sent_starters() {

        let mut from_struct: PunktParameters = get_static_data();
        from_struct.clear_sent_starters();

        assert_eq!(from_struct.sent_starters.len(), 0);
    }

    #[test]
    fn test_clear_ortho_context() {

        let mut from_struct: PunktParameters = get_static_data();
        from_struct.clear_ortho_context();

        assert_eq!(from_struct.ortho_context.len(), 0);
    }

    #[test]
    fn test_ortho_context() {

        let mut from_struct: PunktParameters = get_static_data();
        let value = from_struct.get_ortho_context("a");

        assert_eq!(value, 126);
        assert_eq!(from_struct.ortho_context.len(), 3);

        from_struct.add_ortho_context("a", 250);
        from_struct.add_ortho_context("hello", 100);

        assert_eq!(from_struct.get_ortho_context("a"), 254);
        assert_eq!(from_struct.ortho_context.len(), 4);
    }
}