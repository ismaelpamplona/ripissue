use std::path::PathBuf;
use std::str::FromStr;

use clap::ValueEnum;
use anyhow::{Result, bail, Context};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumString, EnumIter};

use crate::helpers::{traverse_files, get_file_name};

#[derive(AsRefStr, EnumString, EnumIter, Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Status {
    /// Issue must be done and is waiting to begin
    Todo,
    /// Issue is in execution
    Doing,
}

impl Status {

    pub fn status_from_files(path: &PathBuf) -> Result<Option<Self>> {
        let statuses: Vec<PathBuf> = traverse_files(path);
        let status = match statuses.len() {
            0 => None,
            1 => {
                let status_full_path = statuses.get(0).unwrap();
                let status_str = get_file_name(&status_full_path);
                let status = FromStr::from_str(&status_str)
                    .with_context(|| {
                        let statuses_available = Status::iter()
                            .fold(vec![], |mut v, s| {
                                v.push(s.as_ref().to_owned());
                                v
                            });
                        let statuses = statuses_available.join(", ");
                        format!(
                            "Input Status \"{}\" is incorrect. Possible values are {}.",
                            &status_str,
                            &statuses
                            )
                    })?;
                Some(status)
            },
            _ => {
                let msg: Vec<String> = statuses.into_iter()
                    .map(|e| e.to_str().unwrap().to_owned())
                    .collect();
                bail!("Status can't be more than one. Found {}",
                      &msg.join(", "));
            },
        };
        Ok(status)
    }

}
