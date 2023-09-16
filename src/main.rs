mod args;
mod elements;
mod helpers;
mod properties;
mod executors;

extern crate slugify;
use args::sprints::SprintCommand;
use elements::{issue::Issue, elem::WriteAll};
use elements::sprint::Sprint;
use elements::elem::ElemBase;
use executors::close::Closeable;
use executors::commit::Commitable;
use executors::create::Createable;
use executors::delete::Deleteable;
use helpers::id_from_input;
use properties::{tags::{Tag, TagTrait}, statuses::{Status, StatusTrait}};

use crate::args::{
    Cli,
    EntityType,
    issues::IssueCommand,
};

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.entity_type {
        EntityType::Sprint(SprintCommand::Create(cmd)) => {
            Sprint::create(cmd)?;
        }
        EntityType::Issue(IssueCommand::Create(cmd)) => {
            Issue::create(cmd)?;
        },
        EntityType::Issue(IssueCommand::Commit(cmd)) => {
            Issue::commit(&cmd)?;
        }
        EntityType::Issue(IssueCommand::Close(cmd)) => {
            Issue::close(&cmd)?;
        },
        EntityType::Issue(IssueCommand::Delete(cmd)) => {
            Issue::delete(&cmd)?;
        },
        EntityType::Issue(IssueCommand::List(cmd)) => {
            // let mut issues = Elems::new(ElemType::Issue);
            // issues.update()?;
            // let stdout = stdout();
            // let mut writer = BufWriter::new(stdout);
            // writeln!(writer,"{} #{} ({})",
            // Issue::elem().to_uppercase(),
            // &id,
            // issue.display()
            // )?;
            // let issues = get_all_elems::<Issue>()?;
            // println!("{:#?}", issues);
        }
    }

    Ok(())
}
