use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use serde_yaml;

const IFNOTNOW_EXTENSION: &str = ".inn.yaml";

pub enum Cmd {
    InitializeTimeline(String),
    SwitchTimeline(String),
    LoadTimeline(String),
    SaveTimeline(String),
    MarkTimeline(Event),
    SearchTimeline,
}

#[derive(Eq, Debug, PartialEq, PartialOrd, Deserialize, Serialize, Ord)]
pub struct Timespan {
    duration_s: u64,
}
impl Timespan {
    pub fn new(duration_s: u64) -> Timespan {
        Timespan { duration_s }
    }
}
impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.duration_s)
    }
}

pub enum Horizon {
    Day,
    Week,
    Month,
    Year,
    Lifetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct List {
    kind: String,
    pub name: String,
    pub items: Vec<ListItem>,
}
impl List {
    fn new(name: &str) -> List {
        List {
            kind: "list/v1".to_string(),
            name: name.to_string(),
            items: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkbox {
    kind: String,
    pub label: String,
    pub done: Option<DateTime<Utc>>,
    pub active: Option<DateTime<Utc>>,
    pub started: Option<DateTime<Utc>>,
    pub accrued: Timespan,
    created: DateTime<Utc>,
}
impl Checkbox {
    fn new(label: String, done: Option<DateTime<Utc>>) -> Checkbox {
        Checkbox {
            kind: "checkbox/v1".to_string(),
            label,
            done,
            active: None,
            started: None,
            accrued: Timespan::new(0),
            created: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListItem {
    Heading(String),
    Entry(String),
    Checkbox(Checkbox),
    Sublist(List),
    Note(String),
}

#[derive(Debug)]
pub struct ListMap {
    lmap: BTreeMap<String, List>,
}
impl ListMap {
    fn new() -> ListMap {
        ListMap {
            lmap: BTreeMap::new(),
        }
    }
    fn add(&mut self, listname: &str) {
        self.lmap.insert(listname.to_string(), List::new(listname));
    }
    fn drop(&mut self, listname: &str) {
        self.lmap.remove(&listname.to_string());
    }
}

#[derive(Serialize)]
pub struct Event {
    list: List,
    created_ts: DateTime<Utc>,
    begins: Option<DateTime<Utc>>,
    ends: Option<DateTime<Utc>>,
    span: Option<Timespan>,
}
impl Event {
    fn new(list: List, span: Timespan) -> Event {
        Event {
            list,
            span: Some(span),
            begins: None,
            ends: None,
            created_ts: Utc::now(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("ifnotnow")
        .version("1.0")
        .author("Simon Janes <spjanes@protonmail.com>")
        .subcommand(
            SubCommand::with_name("init").arg(
                Arg::with_name("NAME")
                    .help("Sets the name of the new timeline")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("init") {
        let name = matches.value_of("NAME").unwrap();
        let mut timeline = List::new(&name);
        let timeline_yaml = serde_yaml::to_string(&timeline).unwrap();
        let filename = format!("{}{}", &name, IFNOTNOW_EXTENSION);
        if std::path::Path::new(&filename).exists() {
            eprintln!("ERROR: {} exists, not overwriting", filename);
        } else {
            let mut buf = File::create(&filename)?;
            buf.write_all(&timeline_yaml.as_bytes())?;
        }
    }

    Ok(())
}

fn starter_timeline() -> List {
    let mut timeline = List::new(&"Your Starter Timeline");
    timeline.items.push(ListItem::Heading(String::from(
        "Welcome to Your Starter Timeline",
    )));
    timeline.items.push(ListItem::Note(String::from(
        "This is an example timeline that shows the kinds of items you can capture in them.",
    )));
    timeline.items.push(ListItem::Checkbox(Checkbox::new(
        "A TODO Item".to_string(),
        None,
    )));
    timeline.items.push(ListItem::Checkbox(Checkbox::new(
        "A Second TODO Item".to_string(),
        Some(Utc::now()),
    )));
    timeline
        .items
        .push(ListItem::Sublist(List::new("nested list")));
    timeline
}
