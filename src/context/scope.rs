use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type ScopeDef = BTreeMap<String, String>;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fingerprint(u64);

pub struct Scope {
    definitions: ScopeDef,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            definitions: ScopeDef::new(),
        }
    }

    pub fn from(definitions: ScopeDef) -> Self {
        Self { definitions }
    }

    pub fn fingerprint(&self, extra: Option<impl Hash>) -> Fingerprint {
        let mut hasher = DefaultHasher::new();

        for (k, v) in &self.definitions {
            k.hash(&mut hasher);
            v.hash(&mut hasher);
        }

        if let Some(extra) = extra {
            extra.hash(&mut hasher);
        }

        Fingerprint(hasher.finish())
    }
}
