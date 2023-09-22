use std::{
    env::current_dir,
    fs::{create_dir_all, File},
    io::{stdout, BufWriter, Stdout, Write},
    iter::Flatten,
    iter::IntoIterator,
    iter::Iterator,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use git2::{IndexAddOption, Repository};
use slugify::slugify;
use walkdir::WalkDir;

// pub fn get_all_repos_from_parent() -> Vec<PathBuf> {
//     let mut curr = current_dir()?;
//     vec![]
// }

pub fn check_if_dir_is_repo(d: &Path) -> Result<()> {
    d.join(".git");
    if !d.is_dir() {
        bail!("Current dir is not a git repository (or the root of the repo)");
    }
    Ok(())
}

pub fn get_group_dir() -> Result<PathBuf> {
    let mut dir = current_dir()?;
    dir.pop();
    Ok(dir)
}

pub fn get_valid_repo(repo_name: &str) -> Result<PathBuf> {
    let mut dir = get_group_dir()?;
    dir.push(repo_name);
    check_if_dir_is_repo(&dir)?;
    Ok(dir)
}

pub fn get_valid_issue(repo: &Path, issue_id: &str) -> Result<PathBuf> {
    let issue = repo.join(issue_id);
    if !issue.is_dir() {
        bail!("Dir {} is not a valid issue", issue.display());
    }
    Ok(issue)
}

// pub fn type_to_str<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

pub fn wstdout() -> BufWriter<Stdout> {
    let stdout = stdout();
    BufWriter::new(stdout)
}

pub fn walkdir_into_iter(path: &PathBuf) -> Flatten<walkdir::IntoIter> {
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .flatten()
}

pub fn traverse_files(path: &PathBuf) -> Vec<PathBuf> {
    let walk_iter = walkdir_into_iter(path);
    walk_iter.map(|e| e.into_path()).collect()
}

pub fn traverse_dirs(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut vec = vec![];
    for path in paths {
        let walk_iter = walkdir_into_iter(path);
        vec.extend(
            walk_iter
                .filter(|e| e.file_type().is_dir())
                .map(|e| e.into_path())
                .collect::<Vec<PathBuf>>(),
        );
    }
    vec
}

pub fn sys_base_path() -> PathBuf {
    PathBuf::from_str("ripi").unwrap()
}

pub fn base_path(stype: &str) -> PathBuf {
    let mut base_path = sys_base_path();
    base_path.push(stype);
    base_path
}

pub fn base_path_closed(stype: &str) -> PathBuf {
    let mut closed = get_closed_dir();
    closed.push(stype);
    closed
}

pub fn base_path_all(stype: &str) -> Vec<PathBuf> {
    vec![base_path(stype), base_path_closed(stype)]
}

pub fn get_closed_dir() -> PathBuf {
    let mut closed = sys_base_path();
    closed.push(".closed");
    closed
}

pub fn write_file(dir: &PathBuf, file: &str, content: Option<&str>) -> Result<()> {
    create_dir_all(dir).with_context(|| format!("Could not create {}", dir.display()))?;
    let mut file_path = dir.clone();
    file_path.push(file);
    let mut file = File::create(&file_path)
        .with_context(|| format!("Could not create file {}", &file_path.display()))?;
    if let Some(c) = content {
        file.write_all(c.as_bytes())
            .with_context(|| format!("Could not write content to file {}", file_path.display()))?;
    }
    Ok(())
}

pub fn is_not_empty(arg: &str) -> Result<String> {
    if arg.is_empty() {
        bail!("issue create: name cannot be empty");
    }
    Ok(arg.to_string())
}

pub fn is_valid_iso_date(arg: &str) -> Result<String> {
    NaiveDate::parse_from_str(arg, "%Y-%m-%d")?;
    Ok(arg.to_owned())
}

pub fn slug(s: &str) -> String {
    slugify!(&s.to_lowercase(), separator = "_")
}

pub fn slug_tag(s: &str) -> String {
    slugify!(&s.to_lowercase(), separator = "-")
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_owned()
}

pub fn git_commit(files_to_add: Option<&[String]>, msg: &str) -> Result<()> {
    let repo = Repository::open(".").with_context(|| "failed to open repository")?;
    let signature = repo.signature()?;
    let mut index = repo.index()?;
    if let Some(files_to_add) = files_to_add {
        index.add_all(files_to_add.iter(), IndexAddOption::DEFAULT, None)?;
    }
    index.write()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let head = repo.head()?;
    let ref_name = head.name();
    let parent_commit_res = head.peel_to_commit();
    let parent_commit = if parent_commit_res.is_ok() {
        vec![parent_commit_res.as_ref().unwrap()]
    } else {
        vec![]
    };

    repo.commit(ref_name, &signature, &signature, msg, &tree, &parent_commit)?;
    Ok(())
}
