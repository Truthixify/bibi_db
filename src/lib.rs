use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::BTreeMap,
    error::Error,
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
// use crc::Crc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    path: PathBuf,
    pub index: BTreeMap<String, DataSchema>,
}

impl Database {
    pub fn open(path: &PathBuf) -> Result<Database, Box<dyn Error>> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Database {
            path: path.to_path_buf(),
            index: BTreeMap::new(),
        })
    }

    pub fn load_db_from_file(&mut self, path: &PathBuf) -> Result<Database, Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut file = BufReader::new(&mut file);

        let mut json_str = String::new();

        file.read_to_string(&mut json_str)?;

        let db: Database = serde_json::from_str(&json_str)?;

        Ok(db)
    }

    pub fn save_db_data_to_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut file = BufWriter::new(&mut file);

        let json_data = serde_json::to_string(self)?;

        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    pub fn add_schema_to_database(&mut self, key: String, schema: DataSchema) {
        self.index.insert(key, schema);
    }

    pub fn remove_schema_from_database(&mut self, key: String) -> Option<(String, DataSchema)> {
        let (key, value) = self.index.remove_entry(&key)?;
        Some((key, value))
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn contains_schema(&self, key: String) -> bool {
        self.index.contains_key(&key)
    }

    pub fn get_schema(&self, key: String) -> Option<&DataSchema> {
        self.index.get(&key)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    _id: u32,
    data: BTreeMap<String, Data>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Data {
    Int(i32),
    Float(f32),
    Text(String),
    Bool(bool),
}

impl DataSchema {
    pub fn new() -> DataSchema {
        DataSchema {
            _id: uid::IdU32::<u32>::new().get(),
            data: BTreeMap::new(),
        }
    }

    pub fn get_data_from_schema(&self, key: String) -> Option<&Data> {
        self.data.get(&key)
    }

    pub fn add_data_to_schema(&mut self, key: String, value: Data) {
        self.data.insert(key, value);
    }

    pub fn remove_data_from_schema(&mut self, key: String) -> Option<(String, Data)> {
        let (key, value) = self.data.remove_entry(&key)?;
        Some((key, value))
    }

    pub fn update_data_in_schema(&mut self, key: String, value: Data) {
        self.data.insert(key, value);
    }

    pub fn contains_data(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_database_empty() {
        let db = Database::open(&PathBuf::from("db")).unwrap();

        assert!(db.is_empty(), "The database is not empty");
    }

    #[test]
    fn test_is_database_not_empty() {
        let mut db = Database::open(&PathBuf::from("db")).unwrap();
        let schema = DataSchema {
            _id: 1,
            data: BTreeMap::new(),
        };
        db.add_schema_to_database("Test".to_string(), schema);

        assert!(!db.is_empty(), "The database is empty");
    }

    #[test]
    fn test_database_contains_schema() {
        let mut db = Database::open(&PathBuf::from("db")).unwrap();
        let schema = DataSchema {
            _id: 1,
            data: BTreeMap::new(),
        };
        db.add_schema_to_database("Test".to_string(), schema);

        assert!(
            db.contains_schema("Test".to_string()),
            "The database does not contain the key"
        );
    }

    #[test]
    fn test_get_schema_from_database() {
        let mut db = Database::open(&PathBuf::from("db")).unwrap();
        let expected_schema = DataSchema {
            _id: 1,
            data: BTreeMap::new(),
        };
        db.add_schema_to_database("Test".to_string(), expected_schema.clone());
        let schema = db.get_schema("Test".to_string()).unwrap().clone();

        assert_eq!(
            schema, expected_schema,
            "The database does not contain the key"
        );
    }

    #[test]
    fn test_schema_contains_data() {
        let mut data_schema = DataSchema::new();
        data_schema.add_data_to_schema(String::from("Age"), Data::Int(18));

        assert!(
            data_schema.contains_data(String::from("Age")),
            "The dataschema does not contain the key"
        );
    }
}
