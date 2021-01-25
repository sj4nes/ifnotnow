use std::fmt;
use std::collections::BTreeMap;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub struct Timespan {
	duration_s: u64,
}
impl Timespan {
    pub fn new(duration_s: u64) -> Timespan {
        Timespan{duration_s}
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

#[derive(Debug)]
pub struct List {
    name: String,
    items: Vec<ListItem>,
}
impl List {
    fn new(name: &str) -> List {
        List {name: name.to_string(), items:vec![]}
    }
}


#[derive(Debug)]
pub enum ListItem {
    Heading(String),
    Entry(String),
    Checkbox(String,bool),
    Sublist(List),
}

#[derive(Debug)]
pub struct ListMap {
    lmap: BTreeMap<String, List>,
}
impl ListMap {
    fn new() -> ListMap {
        ListMap { lmap: BTreeMap::new() }
    }
    fn add(&mut self, listname: &str) {
        self.lmap.insert(listname.to_string(), List::new(listname));
    }
    fn drop(&mut self, listname: &str) {
        self.lmap.remove(&listname.to_string());
    }
}


fn main() {
    let t = Timespan::new(60);
    let mut lm = ListMap::new();
    println!("Hello, worldd! {}", t);
    println!("lm {:?}", lm);
    lm.add("test");
    println!("lm {:?}", lm);
    lm.drop("test");
    println!("lm {:?}", lm);
}
