use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    file_path: Option<String>,

    schema: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    CreateDB(CreateDB),
    AddSchema(AddSchema),
    RemoveSchema(DeleteSchema),
    GetSchema(GetSchema),
    GetData(GetData),
    DeleteData(DeleteData),
    InsertData(InsertData),
    UpdateData(UpdateData),
}

#[derive(Parser)]
pub struct CreateDB {
    pub path: String,
}

#[derive(Parser)]
pub struct AddSchema {
    pub path: String,
    pub key: String,
}

#[derive(Parser)]
pub struct DeleteSchema {
    pub path: String,
    pub key: String,
}

#[derive(Parser)]
pub struct GetSchema {
    pub path: String,
    pub key: String,
}

#[derive(Parser)]
pub struct CreateSchema {
    pub key: String,
}

#[derive(Parser)]
pub struct GetData {
    pub path: String,
    pub schema_key: String,
    pub key: String,
}

#[derive(Parser)]
pub struct DeleteData {
    pub path: String,
    pub schema_key: String,
    pub key: String,
}

#[derive(Parser)]
pub struct InsertData {
    pub path: String,
    pub schema_key: String,
    pub key: String,
    pub value: String,
}

#[derive(Parser)]
pub struct UpdateData {
    pub path: String,
    pub schema_key: String,
    pub key: String,
    pub value: String,
}
