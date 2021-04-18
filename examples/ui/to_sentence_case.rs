/// To sentence case
pub trait ToSentenceCase: ToOwned {
    fn to_sentence_case(&self) -> Self::Owned;
}

impl ToSentenceCase for str {
    fn to_sentence_case(&self) -> Self::Owned {
        let mut chars = self.chars();
        chars
            .next()
            .map(char::to_uppercase)
            .into_iter()
            .flatten()
            .chain(chars)
            .collect()
    }
}
