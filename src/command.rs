#[derive(Debug)]
pub(super) enum Command {
    Pause,
    Unpause,
    ListDirectory(String),
    PlayPath(String),
}
