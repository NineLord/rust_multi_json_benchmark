/* #region Imports */
// Standard
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf, sync::Arc,
};

// 3rd-Party
use serde::{Deserialize};
/* #endregion */

/* #region Config */
#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: Arc<String>,
    pub size: String,
    pub path: PathBuf,
    #[serde(rename = "numberOfLetters")]
    pub number_of_letters: u8,
    pub depth: u8,
    #[serde(rename = "numberOfChildren")]
    pub number_of_children: u8,
    #[serde(skip)]
    pub raw: Option<Arc<String>>,
}

/* #endregion */

/* #region Configs */
#[derive(Debug, Deserialize)]
pub struct Configs(Vec<Config>);

impl IntoIterator for Configs {
    type Item = Config;
    type IntoIter = <Vec<Config> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Configs {
    type Target = [Config];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl DerefMut for Configs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}
/* #endregion */
