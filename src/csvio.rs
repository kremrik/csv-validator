use csv::{
    Reader,
    StringRecord,
    StringRecordsIter,
};
use std::collections::HashMap;
use std::fs::File;


pub struct Handler {
    pub reader: Reader<File>
}

impl Handler {
    pub fn new(file: File) -> Handler {
        Handler { reader: Reader::from_reader(file) }
    }
}


// type Row = HashMap<String, String>;
pub struct Row {
    number: usize,
    fields: HashMap<String, String>,  // TODO: use ref
}


pub struct CSVIterator<'f> {
    current_row: usize,
    headers: StringRecord,
    records: StringRecordsIter<'f, File>,
}

impl<'f> CSVIterator<'f> {
    pub fn new(handler: &'f mut Handler) -> CSVIterator<'f> {
        CSVIterator {
            current_row: 0,
            headers: handler.reader.headers().unwrap().clone(),
            records: handler.reader.records(),
        }
    }
}

impl<'f> Iterator for CSVIterator<'f> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let next_record = self.records.next();

        if next_record.is_none() {
            return None;
        }

        // TODO: maybe we should handle any weird parsing errors
        let record = next_record.unwrap().unwrap();
        let fields = make_fields(&record, &self.headers);
        let row = Row {
            number: self.current_row,
            fields: fields,
        };

        return Some(row)
    }
}


fn make_fields<'r>(
    record: &'r StringRecord, header: &'r StringRecord
) -> HashMap<String, String> {
    let mut row: HashMap<String, String> = HashMap::new();
    for (colname, value) in header.iter().zip(record) {
        row.insert(colname.to_string(), value.to_string());
    }
    row
}
