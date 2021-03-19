/// State
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum State {
    InitResources,
    LoadAssets,
    TakeSnapshot,
    Done,
}
