/// Fluent state component
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FluentState {
    LoadAssets,
    TakeSnapshot,
    Done,
}
