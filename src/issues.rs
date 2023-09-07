use std::collections::HashMap;
use std::fs::{create_dir_all, create_dir, File};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::{prelude::*, stdout, BufWriter, Write};

use anyhow::{Context, Result, bail, Ok};
use walkdir::{WalkDir, DirEntry};

use crate::helpers::{slug, get_file_name, get_parent_dir};

#[derive(Debug, Clone)]
pub struct Issue {
    pub name: String,
    pub path: PathBuf,
}

impl Default for Issue {
    fn default() -> Self {
        Self {
            name: String::default(),
            path: PathBuf::default(),
        }
    }
}

impl Issue {

    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
        }
    }

    pub fn get_from_str(s: &str) -> Result<Self> {
        let issues = Issues::get_all()?;
        let path = PathBuf::from_str(&s).unwrap();
        if let Some(i) = issues.0.get(s) {
        // s: issue_name
            return Ok(i.clone());
        } else if let Some(i) = issues.0.get(&get_file_name(&path)) {
        // s: kanban/issue_name
            let kanban_dirs = KanbanDirs::new();
            kanban_dirs.is_kanban(&get_parent_dir(&path))?;
            if path == i.path {
                return Ok(i.clone());
            }
        }
        bail!(format!("Input \"{}\" doesn't match with any issue", s));
    }

    pub fn write(&self) -> Result<()> {
        create_dir_all(&self.path)
            .with_context(|| format!("could not create issue_dir {}", &self.path.display()) )?;

        let mut desc_file_path = self.path.clone();
        desc_file_path.push("description.md");
        let mut desc_file = File::create(&desc_file_path)
            .with_context(|| "could not create issue description.md")?;
        desc_file.write_all(format!("# {}", self.name).as_bytes())
            .with_context(|| format!("could not write description title at file: {}", desc_file_path.display()))?;
        Ok(())
    }

}

#[derive(Debug)]
pub struct Issues(pub HashMap<String,Issue>);

impl Issues {

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn already_exists(&self, issue: &Issue) -> Result<()> {
        if self.0.contains_key(&issue.name) {
            bail!(format!(
                    "Issue {} ({}) already exists, rename it before continue",
                    &issue.name,
                    &issue.path.display()));

        }
        Ok(())
    }

    pub fn add(&mut self, issue: Issue) -> Result<()> {
        self.already_exists(&issue)?;
        self.0.insert(issue.name.clone(), issue);
        Ok(())
    }

    pub fn get_all() -> Result<Issues> {
        let kanban_dirs = KanbanDirs::new();
        let mut issues = Issues::new();

        for kanban_dir in kanban_dirs.as_vec() {
            let issues_in_kanban_dir = WalkDir::new(kanban_dir)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir());
            for issue_path in issues_in_kanban_dir {
                let issue_path_buf = issue_path.path().to_path_buf();
                let name = get_file_name(&issue_path_buf);
                let issue = Issue::new(name.clone(), issue_path_buf.clone());
                issues.add(issue)?;
            }
        }

        Ok(issues)
    }

}

#[derive(Debug)]
pub struct KanbanDirs {
    pub backlog: PathBuf,
    pub todo: PathBuf,
    pub doing: PathBuf,
    pub staging: PathBuf,
    pub closed: PathBuf,
}

impl KanbanDirs {

    pub fn new() -> Self {
        KanbanDirs {
            backlog: PathBuf::from_str("_0_backlog").unwrap(),
            todo: PathBuf::from_str("_1_todo").unwrap(),
            doing: PathBuf::from_str("_2_doing").unwrap(),
            staging: PathBuf::from_str("_3_staging").unwrap(),
            closed: PathBuf::from_str("_4_closed").unwrap(),
        }
    }

    fn is_kanban(&self, path: &PathBuf) -> Result<()> {
        let kanban_dirs = KanbanDirs::new();
        if !kanban_dirs.as_vec().contains(path) {
            bail!(format!("Issue \"{}\" isn't in a correct kanban dir", path.display()));
        }
        Ok(())
    }

    fn as_vec(&self) -> Vec<PathBuf> {
        vec![
            self.backlog.clone(),
            self.todo.clone(),
            self.doing.clone(),
            self.staging.clone(),
            self.closed.clone(),
        ]
    }

    pub fn write(&self) -> Result<()> {
        let dirs = self.as_vec();
        for dir in dirs {
            if !dir.is_dir() {
                create_dir(&dir)
                    .with_context(|| format!("could not create dir {}", dir.display()) )?;
                let mut empty_file = dir;
                empty_file.push(".kanban");
                File::create(&empty_file)
                    .with_context(|| "could not create empty file")?;
            }
        }
        Ok(())
    }

}
