use crate::index_directory::Directory;
use super::index_config::IndexWriterConfig;

pub struct IndexWriter{
    pub config: IndexWriterConfig,
    pub directory: Directory,
}
impl IndexWriter {
    pub fn addDocument(&self, document: crate::document::document::Document) {
        todo!()
    }

    pub fn commit(&self) {
        todo!()
    }

    pub fn close(&self) {
        todo!()
    }
}