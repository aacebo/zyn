use crate::SourceFile;

#[derive(Debug, Default)]
pub struct SourceMap(Vec<SourceFile>);

impl SourceMap {
    pub fn files(&self) -> &[SourceFile] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
