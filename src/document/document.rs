use super::text_field::TextField;

pub struct Document{
    fields: Vec<TextField>,
}


impl Document {
    pub fn new() -> Document {
        Document{fields: Vec::new()}
    }

    pub fn add(&mut self, field: TextField) {
        self.fields.push(field);
    }
}

