#![warn(rust_2018_idioms, rust_2018_compatibility)]
#![feature(nll)]

#[macro_use]
extern crate nom;

mod query_planner;
mod sql;
mod table;

use std::collections::HashMap;
use std::error::Error;

use rustyline::{self, error::ReadlineError};

use crate::query_planner::{SelectCommand, WhereSubcommand};
use crate::sql::{parse_statement, ColumnClause, Statement};
use crate::table::{Chamber, ColumnType, Row, Table, TableSchema};

pub struct Database {
    crate tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, name: &str, table: Table) {
        self.tables.insert(name.to_owned(), table);
    }
}

#[derive(Debug)]
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

            let where_subcommand = match statement.where_clause {
                Some(where_clause) => WhereSubcommand::new_column_equality(
                    &table.schema,
                    where_clause.column_name,
                    where_clause.value,
                )?,
                None => WhereSubcommand::new_unconditional(),
            };

            let command = SelectCommand::new_table_scan(
                &table,
                column_names,
                where_subcommand,
            );
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

fn main() {
    println!("Welcome to Epilogue (pre-α)!");
    let mut schema = TableSchema::new();
    schema.add_column("title".to_owned(), ColumnType::String);
    schema.add_column("year".to_owned(), ColumnType::Integer);
    let books = Table::new(schema);
    let mut db = Database::new();
    db.add_table("books", books);
    println!(
        "There is a table 'books' with string column 'title' \
         and integer column 'year'."
    );
    // TODO: completion
    let mut line_reader = rustyline::Editor::<()>::new();
    loop {
        let read = line_reader.readline("Epilogue>> ");
        match read {
            Ok(line) => {
                line_reader.add_history_entry(line.as_ref());
                match parse_statement(&line) {
                    Ok((_remainder, statement)) => {
                        let query_result =
                            execute_statement(&mut db, statement);
                        if let Ok(QueryOk::Select(selectrows)) = query_result {
                            // TODO: use prettytable
                            for selectrow in selectrows {
                                println!("{:?}", selectrow);
                            }
                        } else {
                            println!("{:?}", query_result);
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exited!");
                break;
            }
            Err(err) => {
                println!(
                    "Readline error not otherwise specified?!—{:?}",
                    err
                );
                break;
            }
        }
    }
}
