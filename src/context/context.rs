use super::call_stack::CallStack;
use super::in_stack::InStack;
use super::out_map::OutMap;
use crate::store::FileStore;

pub struct Context {
    pub in_stack: InStack,
    pub call_stack: CallStack,
    pub out_map: OutMap,
    pub file_store: FileStore,
}
