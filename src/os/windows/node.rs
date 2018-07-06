//use cocoa::base::id;
use winapi;

pub(crate) type NodeHandle = id;

#[doc(hidden)]
pub trait Node {
    fn handle(&self) -> NodeHandle;
}
