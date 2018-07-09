#![warn(rust_2018_idioms, rust_2018_compatibility)]
#![allow(unreachable_pub)] // TODO
#![feature(nll)]

#[macro_use]
extern crate nom;

mod query_planner;
mod sql;
mod table;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

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
            let where_subcommand = WhereSubcommand::new_column_equality(
                &table.schema,
                statement.where_clause.column_name,
                statement.where_clause.value,
            )?;
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Epilogue (pre-α)!");
    let mut schema = TableSchema::new();
    schema.add_column("title".to_owned(), ColumnType::String);
    schema.add_column("year".to_owned(), ColumnType::Integer);
    let books = Table::new(schema);
    let mut db = Database::new();
    db.add_table("books", books);
    println!("There is a table 'books' with string column 'title' \
              and integer column 'year'.");
    let mut input_buffer = String::new();
    loop {
        print!("Epilogue>> ");
        io::stdout()
            .flush()
            .expect("couldn't flush stdout");
        {
            io::stdin()
                .read_line(&mut input_buffer)
                .expect("couldn't read input");
        }

        if input_buffer.trim() == "PRINT" { // demo
            println!("{}", db.tables.get("books").unwrap().display());
            continue;
        }

        match parse_statement(&input_buffer) {
            Ok((_remainder, statement)) => {
                let query_result = execute_statement(&mut db, statement);
                println!("{:?}", query_result);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        input_buffer.truncate(0);
    }
}
