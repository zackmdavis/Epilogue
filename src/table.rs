use std::collections::BTreeMap;
use std::error::Error;

use prettytable;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
crate enum ColumnType {
    Key,
    Integer,
    String,
}

// TODO: `Option`-ize to represent NULL?
#[derive(Clone, Debug, PartialEq, Eq)]
crate enum Chamber {
    Key(usize),
    Integer(isize),
    String(String),
}

impl Chamber {
    crate fn column_type(&self) -> ColumnType {
        match *self {
            Chamber::Key(_) => ColumnType::Key,
            Chamber::Integer(_) => ColumnType::Integer,
            Chamber::String(_) => ColumnType::String,
        }
    }

    crate fn display(&self) -> String {
        match self {
            Chamber::Key(k) => format!("{}", k),
            Chamber::Integer(i) => format!("{}", i),
            Chamber::String(s) => format!("{}", s),
        }
    }
}

#[derive(Debug)]
crate struct Column {
    name: String,
    column_type: ColumnType,
}

crate struct TableSchema {
    layout: Vec<Column>,
}

impl TableSchema {
    crate fn new() -> Self {
        Self {
            layout: vec![Column {
                name: "pk".to_owned(),
                column_type: ColumnType::Key,
            }],
        }
    }

    crate fn add_column(&mut self, name: String, column_type: ColumnType) {
        self.layout.push(Column {
            name,
            column_type,
        });
    }

    crate fn validate_row(
        &self,
        &Row(ref chambers): &Row,
    ) -> Result<(), Box<dyn Error>> {
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

crate struct Row(crate Vec<Chamber>);

crate struct Table {
    crate schema: TableSchema,
    crate rows: BTreeMap<usize, Row>,
    // TODO indices
}

impl Table {
    crate fn new(schema: TableSchema) -> Self {
        Self {
            schema,
            rows: BTreeMap::new(),
        }
    }

    // TODO: use `failure` crate
    crate fn insert(&mut self, mut row: Row) -> Result<usize, Box<dyn Error>> {
        self.schema.validate_row(&row)?;
        let pk = self.rows.len() + 1;
        row.0[0] = Chamber::Key(pk);
        let p = self.rows.insert(pk, row);
        assert!(p.is_none());
        Ok(pk)
    }

    crate fn display(&self) -> String {
        let mut buf = Vec::new();
        // TODO don't use such absolute paths (but I want to avoid collisions
        // on `Row`, `Cell`, &c.)
        let mut display_table = prettytable::Table::new();
        display_table.set_format(
            *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        );
        let mut headers = prettytable::row::Row::empty();
        for column in &self.schema.layout {
            headers.add_cell(prettytable::cell::Cell::new(&column.name));
        }
        display_table.set_titles(headers);
        for (_, Row(ref chambers)) in &self.rows {
            let mut display_row = prettytable::row::Row::empty();
            for chamber in chambers {
                display_row.add_cell(
                    prettytable::cell::Cell::new(&chamber.display()),
                );
            }
            display_table.add_row(display_row);
        }
        display_table
            .print(&mut buf)
            .expect("should print to buffer");
        String::from_utf8(buf).expect("pretty table should be valid UTF-8")
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

    #[test]
    fn concerning_table_display() {
        let mut books = example_table();
        let permutation_city = Row(vec![
            Chamber::Key(0),
            Chamber::String("Permutation City".to_owned()),
            Chamber::Integer(1994),
        ]);
        books.insert(permutation_city).unwrap();
        assert_eq!(
            books.display(),
            "\
+----+------------------+------+
| pk | title            | year |
+----+------------------+------+
| 1  | Permutation City | 1994 |
+----+------------------+------+
"
        );
    }

}
