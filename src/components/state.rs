/// State
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    LoadAssets,
    TakeSnapshot,
    Done,
}
