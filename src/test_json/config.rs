/* #region Imports */
// Standard
use std::path::PathBuf;

// 3rd-Party
use serde::{ Deserialize, Serialize };
/* #endregion */

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    name: String,
    size: String,
    path: PathBuf,
    #[serde(rename = "numberOfLetters")]
    number_of_letters: u8,
    depth: u8,
    #[serde(rename = "numberOfChildren")]
    number_of_children: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs(Vec<Config>);
