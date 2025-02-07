use super::Language;

pub trait Translation {
    fn t(&self, key: &str) -> String;
    fn t_with_lang(&self, key: &str, lang: &Language) -> String;
}
