use super::*;
use larch::LarchMinimal;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum Cmd {
    Init(String),
    List,
    Search(String, Query),
    Switch(String),
    Last,
    Next,
    Clear,
    Load(String),
    Save(String),
    Mark(String, Event),
}

struct ContextMod;
struct ContextFlags;
struct ContextModel;
pub fn run(cxc: &Cmd) -> Result<(), std::io::Error> {
    Ok(())
}
impl LarchMinimal for ContextMod {
    type Flags = contexts::ContextFlags;
    type Model = contexts::ContextModel;
    type Msg = contexts::Cmd;
    type View = tui::View;

    fn init(flags: Self::Flags) -> Self::Model {
        ContextModel {}
    }
    fn update(
        cxc: Self::Msg,
        model: Self::Model,
    ) -> Result<(Self::Model, Option<Self::Msg>), anyhow::Error> {
        Ok((model, None))
    }
    fn view(model: Self::Model) -> (Self::View, Option<Self::Msg>) {
        (Self::View {}, None)
    }
}
