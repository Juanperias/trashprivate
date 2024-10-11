use super::crypto::Crypto;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smartcore::{
    linalg::basic::matrix::DenseMatrix, tree::decision_tree_classifier::DecisionTreeClassifier,
};
use std::{fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Db {
    pub train_data: Vec<Crypto>,
    pub model: Option<DecisionTreeClassifier<i64, i64, DenseMatrix<i64>, Vec<i64>>>,
    workdir: PathBuf,
}

impl Db {
    pub fn create(workdir: PathBuf, train_data: Vec<Crypto>) -> Result<Self> {
        let file_path = workdir.join("train.toml");
        let _file = File::create(&file_path)?;

        Ok(Self {
            train_data,
            model: None,
            workdir: file_path,
        })
    }
    pub fn push_data(data: Crypto) {}
}
