use std::path::PathBuf;

use clap::Parser;
use bibi_db::{DataSchema, Database};
mod args;
mod commands;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match args::Cli::parse().command {
        args::Commands::CreateDB(args) => commands::create_db(args)?,
        args::Commands::AddSchema(args) => commands::add_schema_to_database(args)?,
        args::Commands::RemoveSchema(args) => commands::remove_schema_from_database(args)?,
        args::Commands::GetSchema(args) => commands::get_schema(args)?,
        args::Commands::InsertData(args) => commands::add_data_to_schema(args)?,
        args::Commands::GetData(args) => commands::get_data_from_schema(args)?,
        args::Commands::DeleteData(args) => commands::remove_data_from_schema(args)?,
        args::Commands::UpdateData(args) => commands::update_data(args)?,
    }

    Ok(())
}
