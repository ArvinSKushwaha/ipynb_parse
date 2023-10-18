use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Notebook<'a> {
    #[serde(borrow)]
    pub metadata: NotebookMetadata<'a>,

    #[serde(borrow)]
    pub cells: Vec<NotebookCell<'a>>,

    pub nbformat: u16,
    pub nbformat_minor: u16,
}

#[derive(Deserialize, Serialize)]
pub struct NotebookMetadata<'a> {
    #[serde(borrow)]
    #[serde(rename = "kernelspec")]
    pub kernel_spec: KernelSpecification<'a>,

    #[serde(borrow)]
    #[serde(skip_deserializing)]
    pub language_info: Option<LanguageInfo<'a>>,

    pub authors: Vec<Author>,
}

#[derive(Deserialize, Serialize)]
pub struct KernelSpecification<'a> {
    #[serde(borrow)]
    pub argv: Option<Vec<&'a str>>,

    #[serde(borrow)]
    pub display_name: &'a str,

    #[serde(borrow)]
    pub language: &'a str,

    #[serde(borrow)]
    pub env: Option<HashMap<&'a str, &'a str>>,

    pub interrupt_mode: Option<InterruptMode>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InterruptMode {
    Signal,
    Message,
}

#[derive(Deserialize, Serialize)]
pub struct LanguageInfo<'a> {
    pub file_extension: &'a str,
    pub mimetype: &'a str,
    pub name: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct Author {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NotebookCell<'a> {
    #[serde(borrow)]
    pub metadata: Metadata<'a>,

    pub source: String,
    pub cell_type: CellType,
    // pub outputs: Option<CellOutputs>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    Markdown,
    Code,
}

#[derive(Debug, Error)]
#[error("Attempted to parse {attempted}, but expected 'markdown' or 'code'")]
pub struct CellParseError {
    attempted: String,
}

impl FromStr for CellType {
    type Err = CellParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "markdown" => Ok(CellType::Markdown),
            "code" => Ok(CellType::Code),
            _ => Err(CellParseError {
                attempted: s.to_string(),
            }),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Metadata<'a> {
    #[serde(borrow)]
    pub name: &'a str,

    #[serde(borrow)]
    pub tags: Vec<&'a str>,

    #[serde(borrow)]
    pub format: &'a str,

    pub collapsed: bool,
    pub scrolled: bool,
    pub deletable: bool,
    pub editable: bool,
}

impl Default for Metadata<'_> {
    fn default() -> Self {
        Self {
            name: "",
            tags: vec![],
            collapsed: false,
            scrolled: false,
            deletable: true,
            editable: true,
            format: "",
        }
    }
}
