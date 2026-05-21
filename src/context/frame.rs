use super::scope::{Fingerprint, Scope, ScopeDef};
use crate::ast::Literal;
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

    pub fn set_definition(&mut self, key: &str, lit: Literal) {
        if self.scopes.len() == 0 {
            return;
        }

        self.scopes.last_mut().unwrap().set(key, lit);
    }

    pub fn to_string(&self) -> String {
        format!("Frame({:?}, {:?})", self.file_id, self.fingerprint)
    }
}
