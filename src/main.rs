//! ifnotnow (inn) is a Timestripe inspired CLI/TUI application
//!
//! - local first software guidelines
//! - use YAML documents as the "data store" for human-readable/GIT trackable changes
//! - these documents are called "Contexts". They can be added and removed from the view.
//! - Views have one or more axes that structure the items inside.
//! - A time-based view is common
//! - Alphabetic views for topics, tags, titles, people, places, etc.
//! - Geoviews locate places on a projection onto space. These spaces could be the globe or a 2D space
//!   such as the interior of a building.
//!
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use regex;
use serde::{Deserialize, Serialize};
use serde_yaml;

pub type DTUtc = DateTime<Utc>;

const IFNOTNOW_EXTENSION: &str = ".inn.yaml";

/// Patterns for searching contexts
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Pattern {
    Keyword(String),
    Regex(String),
}
impl Pattern {
    fn check_errors(&self) -> Option<PatternErr> {
        use Pattern::*;
        match self {
            Keyword(_) => None,
            Regex(rx) => match regex::Regex::new(rx) {
                Ok(_) => None,
                _ => Some(PatternErr::InvalidRegex),
            },
        }
    }
}

/// Some patterns may have syntax errors.
pub enum PatternErr {
    InvalidRegex,
}

/// Trait for how anything could be matched against a pattern.
pub trait Matchable {
    /// Returns the number of matches of pattern in traited data.
    fn matches(&self, pattern: Pattern) -> Result<usize, PatternErr> {
        Ok(0)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Query {
    ContextNames(Pattern),
    ContextItems(Pattern),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ViewCmd {
    Switch(String),
    Last,
    Next,
    Clear,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ContextCmd {
    Init(String),
    Search(String, Query),
    Switch(String),
    Last,
    Next,
    Clear,
    Load(String),
    Save(String),
    Mark(String, Event),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Commands trigger fun things
pub enum Cmd {
    Noop,
    Help,
    Context(ContextCmd),
    View(ViewCmd),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    now_context: Option<String>,
    contexts: ListMap,
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

#[derive(Debug, Deserialize, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct ListV1 {
    pub name: String,
    pub items: Vec<ListItem>,
}
impl ListV1 {
    fn new(name: &str) -> ListV1 {
        ListV1 {
            name: name.to_string(),
            items: vec![],
        }
    }
    fn filename(name: &str) -> String {
        format!("{}{}", &name, IFNOTNOW_EXTENSION)
    }
    fn load(name: &str) -> Result<ListV1, INNError> {
        match std::fs::File::open(ListV1::filename(&name)) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match serde_yaml::from_reader(reader) {
                    Ok(l) => Ok(l),
                    Err(e) => Err(INNError::Yaml(e)),
                }
            }
            Err(e) => Err(INNError::File(e)),
        }
    }
}

#[derive(Debug)]
pub enum INNError {
    Yaml(serde_yaml::Error),
    File(std::io::Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub enum AttentionEvent {
    Created(DTUtc),
    Started(DTUtc),
    Paused(DTUtc),
    WaitingFor(DTUtc, String),
    Abandoned(DTUtc),
    Finished(DTUtc),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Goal {
    pub label: String,
    pub done: bool,
}
impl Goal {
    fn new(label: String, done: bool) -> Goal {
        Goal { label, done }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct CheckTimebox {
    pub label: String,
    pub done: Option<DTUtc>,
    pub history: Vec<AttentionEvent>,
    pub accrued: Timespan,
    pub budget: Timespan,
}
impl CheckTimebox {
    fn new(label: String, done: Option<DTUtc>) -> CheckTimebox {
        CheckTimebox {
            label,
            done,
            accrued: Timespan::new(0),
            budget: Timespan::new(3600),
            history: vec![AttentionEvent::Created(Utc::now())],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Ord, Eq, PartialOrd, PartialEq)]
pub enum ListItem {
    Heading(String),
    Entry(String),
    Goal(Goal),
    Timebox(CheckTimebox),
    Sublist(ListV1),
    Note(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMap {
    lmap: BTreeMap<String, ListV1>,
}
impl ListMap {
    fn new() -> ListMap {
        ListMap {
            lmap: BTreeMap::new(),
        }
    }
    fn add(&mut self, listname: &str) {
        self.lmap
            .insert(listname.to_string(), ListV1::new(listname));
    }
    fn drop(&mut self, listname: &str) {
        self.lmap.remove(&listname.to_string());
    }
}

#[derive(Debug, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Event {
    list: ListV1,
    created_ts: DTUtc,
    begins: Option<DTUtc>,
    ends: Option<DTUtc>,
    span: Option<Timespan>,
}
impl Event {
    fn new(list: ListV1, span: Timespan) -> Event {
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
        .subcommand(
            SubCommand::with_name("add")
                .arg(
                    Arg::with_name("NAME")
                        .help("Sets the name of the timeline to modify")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("goal")
                        .short("g")
                        .long("goal")
                        .help("Short description of the goal")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("now").arg(
                Arg::with_name("NAME")
                    .help("Sets the name of the timeline to display for now")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();
    let cmd_queue: Vec<Cmd> = vec![];
    match matches.subcommand() {
        ("init", Some(args)) => {
            let name = args.value_of("NAME").unwrap();
            Cmd::Context(ContextCmd::Init(name.to_string()))
        }
        ("add", Some(args)) => {
            let name = args.value_of("NAME").unwrap();
            let event = Event::new(ListV1::new(&name), Timespan::new(3600));
            Cmd::Context(ContextCmd::Mark(name.to_string(), event))
        }
        ("help", Some(args)) => Cmd::Help,
        ("now", Some(args)) => Cmd::Noop,
        _ => Cmd::Noop,
    };
    for cmd in cmd_queue.iter() {
        match cmd {
            Cmd::Context(ContextCmd::Init(name)) => {
                init_timeline(name)?;
            }
            _ => println!("{:?} not implelmented", cmd),
        }
    }

    Ok(())
}

fn init_timeline(name: &str) -> std::io::Result<()> {
    let timeline = match name {
        "starter" => starter_timeline(),
        _ => ListV1::new(&name),
    };
    let timeline_yaml = serde_yaml::to_string(&timeline).unwrap();
    let filename = format!("{}{}", &name, IFNOTNOW_EXTENSION);

    if std::path::Path::new(&filename).exists() {
        eprintln!("ERROR: {} exists, not overwriting", filename);
    } else {
        let mut buf = File::create(&filename)?;
        buf.write_all(&timeline_yaml.as_bytes())?;
    }
    Ok(())
}

fn run_add(matches: &ArgMatches) -> std::io::Result<()> {
    let name = matches.value_of("NAME").unwrap();
    let timeline = ListV1::load(&name);
    match timeline {
        Ok(mut timeline) => {
            if let Some(goal) = matches.value_of("goal") {
                timeline
                    .items
                    .push(ListItem::Goal(Goal::new(goal.to_string(), false)));
            }
            let timeline_yaml = serde_yaml::to_string(&timeline).unwrap();
            let mut buf = File::create(ListV1::filename(&name))?;
            buf.write_all(&timeline_yaml.as_bytes())?;
            Ok(())
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn render_list(list: &ListV1, indent: &str) -> String {
    let mut out = String::from("");
    for x in list.items.iter() {
        out = match x {
            ListItem::Heading(txt) => format!("{}{}## {}\n", out, indent, txt),
            ListItem::Note(txt) => format!("{}{}> {}\n", out, indent, txt),
            ListItem::Goal(cb) => {
                format!(
                    "{}{} - [{}] {}\n",
                    out,
                    indent,
                    if cb.done { "x" } else { " " },
                    if cb.done {
                        format!("~~{}~~", &cb.label)
                    } else {
                        String::from(&cb.label)
                    }
                )
            }
            ListItem::Timebox(tb) => {
                format!(
                    "{}{} - [{}] {} (..{} <={})\n",
                    out, indent, "?", tb.label, tb.accrued, tb.budget
                )
            }
            ListItem::Entry(ent) => {
                format!("{}{} - {}\n", out, indent, ent)
            }
            ListItem::Sublist(sub) => {
                format!("{}{}", out, render_list(sub, &"   "))
            }
        }
    }
    out
}

fn run_now(matches: &ArgMatches) -> std::io::Result<()> {
    let name = matches.value_of("NAME").unwrap();
    let timeline = ListV1::load(&name);
    match timeline {
        Ok(timeline) => {
            println!("# {}", timeline.name);
            println!("{}", render_list(&timeline, ""));
            Ok(())
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn starter_timeline() -> ListV1 {
    let mut timeline = ListV1::new(&"Your Starter Timeline");
    timeline.items.push(ListItem::Heading(String::from(
        "Welcome to Your Starter Timeline",
    )));
    timeline.items.push(ListItem::Note(String::from(
        "This is an example timeline that shows the kinds of items you can capture in them.",
    )));
    timeline
        .items
        .push(ListItem::Goal(Goal::new("A TODO Item".to_string(), false)));
    timeline.items.push(ListItem::Goal(Goal::new(
        "A done TODO Item".to_string(),
        true,
    )));
    timeline
        .items
        .push(ListItem::Goal(Goal::new("A TODO Item".to_string(), false)));
    timeline.items.push(ListItem::Timebox(CheckTimebox::new(
        "A Second TODO Item".to_string(),
        Some(Utc::now()),
    )));
    timeline
        .items
        .push(ListItem::Sublist(ListV1::new("nested list")));
    timeline
}
