use super::ast_map::AstMap;
use super::call_stack::CallStack;
use super::error::ContextError;
use super::in_stack::InStack;
use super::out_map::OutMap;
use crate::Args;
use crate::ast;
use crate::store::{FileId, FileStore, NodeStore, error::FileStoreError};

pub struct Context {
    pub ast_map: AstMap,
    pub in_stack: InStack,
    pub call_stack: CallStack,
    pub out_map: OutMap,
    pub file_store: FileStore,
    pub node_store: NodeStore,
}

impl Context {
    pub fn new(args: &Args) -> Result<Self, ContextError> {
        let mut ctx = Context {
            ast_map: AstMap::new(),
            in_stack: InStack::new(),
            call_stack: CallStack::new(),
            out_map: OutMap::new(&args.output),
            file_store: FileStore::new(),
            node_store: NodeStore::new(),
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
        let result = String::new();

        let path = self
            .file_store
            .get_by_id(file_id)
            .ok_or_else(|| {
                let s = format!("Could not find the stored file with id {:?}", file_id);
                ContextError::GenerationError(s)
            })?
            .to_path_buf();

        if !self.ast_map.has_ast_for(file_id) {
            let root_id = ast::from_file(file_id, self).map_err(|e| {
                let s = format!(
                    "Failed to generate AST from context for path {}\n{}",
                    path.display(),
                    e.to_string()
                );

                ContextError::GenerationError(s)
            })?;

            self.ast_map.insert(file_id, root_id);
        }

        // TODO: Traverse ast

        Ok(result)
    }

    pub fn finalize(&self) -> Result<(), ContextError> {
        self.out_map.save(&self.file_store).map_err(|e| {
            let s = format!("Failed to finalize context\n{}", e.to_string());
            ContextError::FinalizationError(s)
        })
    }

    pub fn debug(&self) {
        self.file_store.debug();
        self.node_store.debug();
        self.in_stack.debug();
        self.call_stack.debug();
        self.out_map.debug();
        self.ast_map.debug();
    }

    pub fn debug_with_ast(&self) {
        self.debug();
        self.ast_map.debug_all_ast(&self.node_store);
    }
}
