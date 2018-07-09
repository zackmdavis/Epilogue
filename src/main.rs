#![warn(rust_2018_idioms, rust_2018_compatibility)]
#![feature(nll)]

#[macro_use]
extern crate nom;

mod query_planner;
mod sql;
mod table;

use std::collections::HashMap;
use std::error::Error;

use crate::query_planner::{
    column_names_to_offsets, SelectCommand, WhereSubcommand,
};
use crate::sql::{ColumnClause, Statement};
use crate::table::{Chamber, ColumnType, Row, Table, TableSchema};

pub struct Database {
    crate tables: HashMap<String, Table>,
}

pub enum QueryOk<'a> {
    Select(Vec<Vec<&'a Chamber>>),
    Insert(usize),
}

fn execute_statement<'db>(
    db: &'db mut Database,
    statement: Statement,
) -> Result<QueryOk<'db>, Box<dyn Error>> {
    match statement {
        Statement::Select(statement) => {
            let table = db.tables
                .get(&statement.table_name)
                .ok_or(format!(
                    "no table named {}",
                    statement.table_name
                ))?;
            let column_names = match statement.column_names {
                ColumnClause::Star => table
                    .schema
                    .layout
                    .iter()
                    .map(|column| &column.name)
                    .cloned()
                    .collect(),
                ColumnClause::Names(names) => names,
            };
            let command = SelectCommand {
                column_offsets: column_names_to_offsets(
                    &table.schema,
                    &column_names,
                )?,
                view: table.rows.values(),
                filter: Box::new(
                    WhereSubcommand {
                        column_offset: column_names_to_offsets(
                            &table.schema,
                            &[statement.where_clause.column_name],
                        )?[0],
                        value: statement.where_clause.value,
                    }.operationalize(),
                ),
            };
            Ok(QueryOk::Select(command.execute()))
        }
        Statement::Insert(statement) => {
            let table = db.tables.get_mut(&statement.table_name).ok_or(
                format!("no table named {}", statement.table_name),
            )?;
            let pk_chamber = Chamber::Key(0);
            let mut chambers = vec![pk_chamber];
            chambers.extend(statement.values);
            table.insert(Row(chambers))?;
            Ok(QueryOk::Insert(1))
        }
    }
}

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
