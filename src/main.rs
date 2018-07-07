#![warn(rust_2018_idioms, rust_2018_compatibility)]
#![feature(nll)]

#[macro_use]
extern crate nom;

mod query_planner;
mod sql;
mod table;

use std::error::Error;

use crate::table::{Chamber, ColumnType, Row, Table, TableSchema};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello Epilogue world!");
    let mut schema = TableSchema::new();
    schema.add_column("title".to_owned(), ColumnType::String);
    schema.add_column("year".to_owned(), ColumnType::Integer);
    let mut books = Table::new(schema);
    let the_art_of_rationality = Row(vec![
        Chamber::Key(0),
        Chamber::String("Rationality: From AI to Zombies".to_owned()),
        Chamber::Integer(2015),
    ]);
    books.insert(the_art_of_rationality)?;
    println!("{}", books.display());
    Ok(())
}
