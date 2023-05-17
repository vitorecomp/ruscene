use ruscene::analyzer::standard_analyzer::StandardAnalyzer;
use ruscene::document::document::Document;
use ruscene::document::text_field::TextField;
use ruscene::index_directory;
use ruscene::indexer::index_config::IndexWriterConfig;
use ruscene::indexer::index_writer::IndexWriter;


fn main() {

    // Create the index directory
    let indexDirectory = index_directory::Directory{path: String::from("index")};

    // Create the analyzer and index writer configuration
    let analyzer = StandardAnalyzer{};
    let index_config = IndexWriterConfig{analyzer};

    // Create the index writer
    let indexWriter = IndexWriter{config: index_config, directory: indexDirectory};

    // Create a document to be indexed
    let mut document = Document::new();
    document.add(TextField{title:String::from("content"), content:String::from("This is the content of the document")});

    // Add the document to the index
    indexWriter.addDocument(document);

    // Commit the changes and close the index writer
    indexWriter.commit();
    indexWriter.close();

    println!("Document indexed successfully.");
}
