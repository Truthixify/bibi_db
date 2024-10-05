use crate::*;
use args::{
    AddSchema, CreateDB, DeleteData, DeleteSchema, GetData, GetSchema, InsertData, UpdateData,
};
use bibi_db::Data;
use std::error::Error;

pub fn create_db(args: CreateDB) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);

    let mut db = Database::open(&path)?;

    db.save_db_data_to_file(&path)?;

    Ok(())
}

pub fn add_schema_to_database(args: AddSchema) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;
    let schema = DataSchema::new();

    let mut db = db.load_db_from_file(&path)?;

    db.add_schema_to_database(args.key, schema);

    db.save_db_data_to_file(&path)?;

    Ok(())
}

pub fn remove_schema_from_database(args: DeleteSchema) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let mut db = db.load_db_from_file(&path)?;

    db.remove_schema_from_database(args.key);

    db.save_db_data_to_file(&path)?;

    Ok(())
}

pub fn get_schema(args: GetSchema) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let db = db.load_db_from_file(&path)?;

    let schema = db.get_schema(args.key.clone());

    println!("schema: {:?}", schema);

    Ok(())
}

pub fn add_data_to_schema(args: InsertData) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let mut db = db.load_db_from_file(&path)?;

    let mut schema = db.get_schema(args.schema_key.clone()).unwrap().to_owned();

    if args.value.contains("Bool") {
        let v: bool = args
            .value
            .replace("(", "")
            .replace("Bool", "")
            .replace(")", "")
            .parse()
            .expect("Not a bool");
        schema.add_data_to_schema(args.key, Data::Bool(v));
    } else if args.value.contains("Int") {
        let v: i32 = args
            .value
            .replace("(", "")
            .replace("Int", "")
            .replace(")", "")
            .parse()
            .expect("Not an i32");
        schema.add_data_to_schema(args.key, Data::Int(v));
    } else if args.value.contains("Float") {
        let v: f32 = args
            .value
            .replace("(", "")
            .replace("Float", "")
            .replace(")", "")
            .parse()
            .expect("Not a f32");
        schema.add_data_to_schema(args.key, Data::Float(v));
    } else if args.value.contains("Text") {
        let v: String = args
            .value
            .replace("(", "")
            .replace("Text", "")
            .replace(")", "")
            .parse()
            .expect("Not a String");
        schema.add_data_to_schema(args.key, Data::Text(v));
    } else {
        return Err("Invalid type for data.".into());
    }

    db.add_schema_to_database(args.schema_key, schema);

    db.save_db_data_to_file(&path)?;

    Ok(())
}

pub fn get_data_from_schema(args: GetData) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let db = db.load_db_from_file(&path)?;

    let schema = db.get_schema(args.schema_key).unwrap();

    let data = schema.get_data_from_schema(args.key).unwrap();

    println!("{:?}", data);

    Ok(())
}

pub fn remove_data_from_schema(args: DeleteData) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let mut db = db.load_db_from_file(&path)?;

    let mut schema = db.get_schema(args.schema_key.clone()).unwrap().clone();

    println!("{schema:?}");

    schema.remove_data_from_schema(args.key);

    println!("{schema:?}");

    println!("{db:?}");

    db.add_schema_to_database(args.schema_key, schema);

    println!("{db:?}");

    db.save_db_data_to_file(&path)?;

    Ok(())
}

pub fn update_data(args: UpdateData) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(args.path);
    let mut db = Database::open(&path)?;

    let mut db = db.load_db_from_file(&path)?;

    let mut schema = db.get_schema(args.schema_key.clone()).unwrap().to_owned();

    if args.value.contains("Bool") {
        let v: bool = args
            .value
            .replace("(", "")
            .replace("Bool", "")
            .replace(")", "")
            .parse()
            .expect("Not a bool");
        schema.add_data_to_schema(args.key, Data::Bool(v));
    } else if args.value.contains("Int") {
        let v: i32 = args
            .value
            .replace("(", "")
            .replace("Int", "")
            .replace(")", "")
            .parse()
            .expect("Not an i32");
        schema.add_data_to_schema(args.key, Data::Int(v));
    } else if args.value.contains("Float") {
        let v: f32 = args
            .value
            .replace("(", "")
            .replace("Float", "")
            .replace(")", "")
            .parse()
            .expect("Not a f32");
        schema.add_data_to_schema(args.key, Data::Float(v));
    } else if args.value.contains("Text") {
        let v: String = args
            .value
            .replace("(", "")
            .replace("Text", "")
            .replace(")", "")
            .parse()
            .expect("Not a String");
        schema.add_data_to_schema(args.key, Data::Text(v));
    } else {
        return Err("Invalid type for data.".into());
    }

    db.add_schema_to_database(args.schema_key, schema);

    db.save_db_data_to_file(&path)?;

    Ok(())
}
