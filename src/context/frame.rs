use super::error::ContextError;
use super::scope::{Fingerprint, Scope, ScopeDef};
use crate::ast::Literal;
use crate::store::FileId;
use std::path::Path;

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

    pub fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    pub fn set_definition(&mut self, key: &str, lit: Literal) {
        if self.scopes.len() == 0 {
            return;
        }

        self.scopes.last_mut().unwrap().set(key, lit);
    }

    pub fn get_definition(&self, key: &str) -> Option<&Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(s) = scope.get(key) {
                return Some(s);
            }
        }

        None
    }

    pub fn open_file(
        &mut self,
        identifier: &str,
        path: impl AsRef<Path>,
        file_id: FileId,
    ) -> Result<(), ContextError> {
        if self.scopes.len() == 0 {
            let s = format!("No scope to open file in");
            return Err(ContextError::NoScopeError(s));
        }

        self.scopes
            .last_mut()
            .unwrap()
            .open(identifier, path, file_id)
    }

    pub fn enter_new_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_current_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn to_string(&self) -> String {
        format!("Frame({:?}, {:?})", self.file_id, self.fingerprint)
    }
}
