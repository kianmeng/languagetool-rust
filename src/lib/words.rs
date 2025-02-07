//! Structures for `words` requests and responses.

#[cfg(feature = "cli")]
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Check if `v` is a valid word.
///
/// A valid word is any string slice that does not contain any whitespace
///
/// # Examples
///
/// ```
/// # use languagetool_rust::words::is_word;
/// assert!(is_word("word").is_ok());
///
/// assert!(is_word("some words").is_err());
/// ```
pub fn is_word(v: &str) -> Result<(), String> {
    if !v.contains(' ') {
        return Ok(());
    }
    Err(String::from(
        "The value should be a word that does not contain any whitespace",
    ))
}

#[cfg_attr(feature = "cli", derive(Parser))]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
/// Login arguments required by the API.
pub struct LoginArgs {
    #[cfg_attr(feature = "cli", clap(short = 'u', long, required = true))]
    /// Your username as used to log in at languagetool.org.
    pub username: String,
    #[cfg_attr(feature = "cli", clap(short = 'k', long, required = true))]
    /// [Your API key](https://languagetool.org/editor/settings/api)
    pub api_key: String,
}

#[cfg_attr(feature = "cli", derive(Parser))]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool GET words request.
///
/// List words in the user's personal dictionaries.
pub struct WordsRequest {
    #[cfg_attr(feature = "cli", clap(long, default_value = "0"))]
    /// Offset of where to start in the list of words.
    offset: isize,
    #[cfg_attr(feature = "cli", clap(long, default_value = "10"))]
    /// Maximum number of words to return.
    pub limit: isize,
    #[cfg_attr(feature = "cli", clap(flatten))]
    /// Login arguments
    pub login: LoginArgs,
    #[cfg_attr(feature = "cli", clap(long))]
    /// Comma-separated list of dictionaries to include words from; uses special default dictionary if this is unset
    pub dicts: Option<Vec<String>>,
}

#[cfg_attr(feature = "cli", derive(Parser))]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool POST words add request.
///
/// Add a word to one of the user's personal dictionaries. Please note that this feature is considered to be used for personal dictionaries which must not contain more than 500 words. If this is an issue for you, please contact us.
pub struct WordsAddRequest {
    #[cfg_attr(feature = "cli", clap(required = true, validator = is_word))]
    /// The word to be added. Must not be a phrase, i.e. cannot contain white space. The word is added to a global dictionary that applies to all languages.
    pub word: String,
    #[cfg_attr(feature = "cli", clap(flatten))]
    /// Login arguments
    pub login: LoginArgs,
    #[cfg_attr(feature = "cli", clap(long))]
    /// Name of the dictionary to add the word to; non-existent dictionaries are created after
    /// calling this; if unset, adds to special default dictionary
    dict: Option<String>,
}

#[cfg_attr(feature = "cli", derive(Parser))]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool POST words delete request.
///
/// Remove a word from one of the user's personal dictionaries.
pub struct WordsDeleteRequest {
    #[cfg_attr(feature = "cli", clap(required = true, validator = is_word))]
    /// The word to be removed.
    pub word: String,
    #[cfg_attr(feature = "cli", clap(flatten))]
    /// Login arguments
    pub login: LoginArgs,
    #[cfg_attr(feature = "cli", clap(long))]
    /// Name of the dictionary to add the word to; non-existent dictionaries are created after
    /// calling this; if unset, adds to special default dictionary
    pub dict: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool GET words reponse.
pub struct WordsResponse {
    /// List of words
    words: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool POST word add reponse.
pub struct WordsAddResponse {
    /// `true` if word was corretly added
    added: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
/// LanguageTool POST word delete reponse.
pub struct WordsDeleteResponse {
    /// `true` if word was correctly removed
    deleted: bool,
}
