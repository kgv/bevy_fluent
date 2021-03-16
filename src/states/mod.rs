#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FluentState {
    LoadAssets,
    Snapshot,
    Done,
}
