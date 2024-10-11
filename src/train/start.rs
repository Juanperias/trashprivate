use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use smartcore::tree::decision_tree_classifier::DecisionTreeClassifier;

pub fn train(folder: PathBuf) -> Result<()> {
    println!("[{}] Training model", "INFO".blue());

    Ok(())
}
