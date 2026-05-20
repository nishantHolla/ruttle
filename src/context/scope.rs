use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fingerprint(u64);

pub struct Scope {
    definitions: BTreeMap<String, String>,
}
