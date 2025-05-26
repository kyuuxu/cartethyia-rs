use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum PoolCategory {
    Permanent,
    Event(Duration),
    Special(Duration),
}