use super::ast_map::AstMap;
use super::call_stack::CallStack;
use super::error::ContextError;
use super::in_stack::InStack;
use super::out_map::OutMap;
use crate::Args;
use crate::store::{FileId, FileStore, error::FileStoreError};

pub struct Context {
    pub ast_map: AstMap,
    pub in_stack: InStack,
    pub call_stack: CallStack,
    pub out_map: OutMap,
    pub file_store: FileStore,
}

impl Context {
    pub fn new(args: &Args) -> Result<Self, ContextError> {
        let mut ctx = Context {
            ast_map: AstMap::new(),
            in_stack: InStack::new(),
            call_stack: CallStack::new(),
            out_map: OutMap::new(&args.output),
            file_store: FileStore::new(),
        };

        // Add all the input files to the file store
        for path in &args.inputs {
            match ctx.file_store.add(path) {
                Ok(id) => ctx.in_stack.push(id),

                Err(FileStoreError::InvalidPath(s)) => {
                    let s = format!("Failed to initialize context\n{}", s);
                    return Err(ContextError::InitializationError(s));
                }

                Err(FileStoreError::DuplicatePath(_)) => {
                    /* Do nothing if the path already exists */
                }
            }
        }

        Ok(ctx)
    }

    pub fn complete(&mut self) -> Result<(), ContextError> {
        while !self.in_stack.empty() {
            let current = self.in_stack.pop().unwrap();
            self.call_stack.push(current, None);

            let result = self.generate(current)?;
            self.out_map.insert(current, result);
        }

        Ok(())
    }

    fn generate(&mut self, file_id: FileId) -> Result<String, ContextError> {
        // TODO: Complete this
        Ok(String::from("Hello"))
    }

    pub fn finalize(&self) -> Result<(), ContextError> {
        self.out_map.save(&self.file_store).map_err(|e| {
            let s = format!("Failed to finalize context\n{}", e.to_string());
            ContextError::FinalizationError(s)
        })
    }
}
