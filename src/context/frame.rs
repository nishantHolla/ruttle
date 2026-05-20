use super::scope::{Fingerprint, Scope};
use crate::store::FileId;

pub struct Frame {
    scopes: Vec<Scope>,
    fingerprint: Fingerprint,
    file_id: FileId,
}
