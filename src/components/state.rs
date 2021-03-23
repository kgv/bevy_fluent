/// Fluent state component
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FluentState {
    LoadAssets,
    TakeSnapshot,
    Done,
}
