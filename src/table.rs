#![allow(dead_code)]

use std::collections::BTreeMap;
use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ColumnType {
    Key,
    Integer,
    String,
}

// TODO: `Option`-ize to represent NULL?
#[derive(Clone, Debug, PartialEq, Eq)]
enum Chamber {
    Key(usize),
    Integer(isize),
    String(String),
}

impl Chamber {
    fn column_type(&self) -> ColumnType {
        match *self {
            Chamber::Key(_) => ColumnType::Key,
            Chamber::Integer(_) => ColumnType::Integer,
            Chamber::String(_) => ColumnType::String,
        }
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    column_type: ColumnType,
}

struct TableSchema {
    layout: Vec<Column>,
}

impl TableSchema {
    fn new() -> Self {
        Self {
            layout: vec![Column {
                name: "pk".to_owned(),
                column_type: ColumnType::Key,
            }],
        }
    }

    fn add_column(&mut self, name: String, column_type: ColumnType) {
        self.layout.push(Column {
            name,
            column_type,
        });
    }

    fn validate_row(
        &self,
        &Row(ref chambers): &Row,
    ) -> Result<(), Box<Error>> {
        for (i, (ref chamber, ref column_def)) in
            chambers.iter().zip(&self.layout).enumerate()
        {
            let column_type = chamber.column_type();
            let expected_type = column_def.column_type;
            if column_type != expected_type {
                return Err(From::from(format!(
                    "type mismatch at {}: expected {:?}, got {:?}",
                    i, expected_type, column_type
                )));
            }
        }
        Ok(())
    }
}

struct Row(Vec<Chamber>);

struct Table {
    schema: TableSchema,
    rows: BTreeMap<usize, Row>,
    // TODO indices
}

impl Table {
    fn new(schema: TableSchema) -> Self {
        Self {
            schema,
            rows: BTreeMap::new(),
        }
    }

    // TODO: use `failure` crate
    fn insert(&mut self, mut row: Row) -> Result<usize, Box<Error>> {
        self.schema.validate_row(&row)?;
        let pk = self.rows.len() + 1;
        row.0[0] = Chamber::Key(pk);
        let p = self.rows.insert(pk, row);
        assert!(p.is_none());
        Ok(pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_table() -> Table {
        let mut schema = TableSchema::new();
        schema.add_column("title".to_owned(), ColumnType::String);
        schema.add_column("year".to_owned(), ColumnType::Integer);
        let table = Table::new(schema);
        table
    }

    #[test]
    fn concerning_insertion() {
        let mut books = example_table();
        let the_fountainhead = Row(vec![
            Chamber::Key(0),
            Chamber::String("The Fountainhead".to_owned()),
            Chamber::Integer(1943),
        ]);
        books.insert(the_fountainhead).unwrap();
    }

    #[test]
    fn concerning_type_mismatch_on_insertion() {
        let mut books = example_table();
        let causality = Row(vec![
            Chamber::Key(0),
            Chamber::Integer(2000),
            Chamber::String("Causality".to_owned()),
        ]);
        assert!(books.insert(causality).is_err());
    }

}
