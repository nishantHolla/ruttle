use super::scope::{Fingerprint, Scope, ScopeDef};
use crate::ast::Literal;
use crate::store::FileId;
use serde_json::Value;

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

    pub fn get_current_scope(&self) -> Option<&Scope> {
        if self.scopes.len() == 0 {
            None
        } else {
            self.scopes.last()
        }
    }

    pub fn get_mut_current_scope(&mut self) -> Option<&mut Scope> {
        if self.scopes.len() == 0 {
            None
        } else {
            self.scopes.last_mut()
        }
    }

    pub fn resolve_to_value(&self, key: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.resolve_to_value(key) {
                return Some(val);
            }
        }

        None
    }

    pub fn get_open_file_id(&self, key: &str) -> Option<FileId> {
        for scope in self.scopes.iter().rev() {
            if let Some(id) = scope.get_open_file_id(key) {
                return Some(id);
            }
        }

        return None;
    }

    pub fn resolve_to_lit(&self, key: &str) -> Option<Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(lit) = scope.resolve_to_lit(key) {
                return Some(lit);
            }
        }

        None
    }

    pub fn resolve(&self, key: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(lit) = scope.resolve(key) {
                return Some(lit);
            }
        }

        None
    }

    // pub fn set_definition(&mut self, key: &str, lit: Literal) {
    //     if self.scopes.len() == 0 {
    //         return;
    //     }
    //
    //     self.scopes.last_mut().unwrap().set(key, lit);
    // }
    //
    // pub fn get_definition(&self, key: &str) -> Option<&Literal> {
    //     for scope in self.scopes.iter().rev() {
    //         if let Some(s) = scope.get(key) {
    //             return Some(s);
    //         }
    //     }
    //
    //     None
    // }
    //
    // pub fn open_file(
    //     &mut self,
    //     identifier: &str,
    //     path: impl AsRef<Path>,
    //     file_id: FileId,
    // ) -> Result<(), ContextError> {
    //     if self.scopes.len() == 0 {
    //         let s = format!("No scope to open file in");
    //         return Err(ContextError::NoScopeError(s));
    //     }
    //
    //     self.scopes
    //         .last_mut()
    //         .unwrap()
    //         .open(identifier, path, file_id)
    // }

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
