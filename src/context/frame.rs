use super::scope::{Fingerprint, Scope, ScopeDef};
use crate::store::FileId;

pub struct Frame {
    scopes: Vec<Scope>,
    fingerprint: Fingerprint,
    file_id: FileId,
}

impl Frame {
    pub fn new(file_id: FileId, scope: Option<ScopeDef>) -> Self {
        let scope = match scope {
            Some(scope) => Scope::from(scope),
            None => Scope::new(),
        };

        let fingerprint = scope.fingerprint(Some(file_id));

        Self {
            scopes: vec![scope],
            fingerprint,
            file_id,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Frame({:?}, {:?})", self.file_id, self.fingerprint)
    }
}
