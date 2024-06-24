use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::tokenizer::stemmer::Language;
use crate::tokenizer::tokenizer::TextAnalyzer;
use crate::tokenizer::{
    LowerCaser, RawTokenizer, RemoveLongFilter, SimpleTokenizer, Stemmer, WhitespaceTokenizer,
};

/// The tokenizer manager serves as a store for
/// all of the pre-configured tokenizer pipelines.
///
/// By default, it is populated with the following managers.
///
///  * `raw` : does not process nor tokenize the text.
///  * `default` : Chops the text on according to whitespace and
///  punctuation, removes tokens that are too long, and lowercases
///  tokens
///  * `en_stem` : Like `default`, but also applies stemming on the
///  resulting tokens. Stemming can improve the recall of your
///  search engine.
/// * `whitespace` : Splits the text on whitespaces.
#[derive(Clone)]
pub struct TokenizerManager {
    tokenizers: Arc<RwLock<HashMap<String, TextAnalyzer>>>,
}

impl TokenizerManager {
    /// Creates an empty tokenizer manager.
    pub fn new() -> Self {
        Self {
            tokenizers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a new tokenizer associated with a given name.
    pub fn register<T>(&self, tokenizer_name: &str, tokenizer: T)
    where
        TextAnalyzer: From<T>,
    {
        let boxed_tokenizer: TextAnalyzer = TextAnalyzer::from(tokenizer);
        self.tokenizers
            .write()
            .expect("Acquiring the lock should never fail")
            .insert(tokenizer_name.to_string(), boxed_tokenizer);
    }

    /// Accessing a tokenizer given its name.
    pub fn get(&self, tokenizer_name: &str) -> Option<TextAnalyzer> {
        self.tokenizers
            .read()
            .expect("Acquiring the lock should never fail")
            .get(tokenizer_name)
            .cloned()
    }
}

impl Default for TokenizerManager {
    /// Creates an `TokenizerManager` prepopulated with
    /// the default pre-configured tokenizers of `tantivy`.
    fn default() -> TokenizerManager {
        let manager = TokenizerManager::new();
        manager.register("raw", RawTokenizer::default());
        manager.register(
            "default",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .build(),
        );
        manager.register(
            "de_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::German))
                .build(),
        );
        manager.register(
            "en_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::English))
                .build(),
        );
        manager.register(
            "es_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Spanish))
                .build(),
        );
        manager.register(
            "fr_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::French))
                .build(),
        );
        manager.register(
            "hu_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Hungarian))
                .build(),
        );
        manager.register(
            "it_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Italian))
                .build(),
        );
        manager.register(
            "nl_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Dutch))
                .build(),
        );
        manager.register(
            "pt_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Portuguese))
                .build(),
        );
        manager.register(
            "ro_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Romanian))
                .build(),
        );
        manager.register(
            "ru_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Russian))
                .build(),
        );
        manager.register(
            "ta_stem",
            TextAnalyzer::builder(SimpleTokenizer::default())
                .filter(RemoveLongFilter::limit(40))
                .filter(LowerCaser)
                .filter(Stemmer::new(Language::Tamil))
                .build(),
        );
        manager.register("whitespace", WhitespaceTokenizer::default());
        manager
    }
}
