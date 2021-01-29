/// Every subcommand may be a minimal Larch application. Larch being
/// my flavor of the Elm Architecture for CLI/TUI
pub trait LarchMinimal {
    /// The initial configuration for the application, perhaps to load the persisted state.
    type Flags;
    /// The state of the application
    type Model;
    /// Something that causes an update to the model
    type Msg;
    /// How the model is translated into a view
    type View;

    fn init(flags: Self::Flags) -> Self::Model;
    fn update(
        msg: Self::Msg,
        model: Self::Model,
    ) -> Result<(Self::Model, Option<Self::Msg>), anyhow::Error>;
    fn view(model: Self::Model) -> (Self::View, Option<Self::Msg>);
}
