pub mod index_directory {
    pub struct Directory {
        pub path: String,
    }
}


pub mod analyzer {
    pub mod standard_analyzer;
}

pub mod indexer {
    pub mod index_config;
    pub mod index_writer;
}

pub mod document {
    pub mod document;
    pub mod text_field;
}