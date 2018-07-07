#![allow(dead_code)]

use std::collections::btree_map;
use std::error::Error;

use crate::table::{Chamber, Row, Table, TableSchema};

#[derive(Debug)]
struct WhereClause {
    // XXX TODO: starting out with supporting just a single `column = value`
    // condition, but should eventually expand to conj-/dis-junctions, &c.
    //
    // Use a column offset (don't want to overload the word "index") instead of
    // a name so that we can operate on Row directly rather than looking up the
    // name
    column_offset: usize,
    value: Chamber,
}

impl WhereClause {
    fn new_column_equality(
        schema: &TableSchema,
        column_name: String,
        value: Chamber,
    ) -> Result<Self, Box<dyn Error>> {
        let (i, _column) = schema
            .layout
            .iter()
            .enumerate()
            .find(|i_col| i_col.1.name == column_name)
            .ok_or(format!("no column named {}", column_name))?;
        Ok(Self {
            column_offset: i,
            value,
        })
    }

    fn operationalize(self) -> impl Fn(&Row) -> bool + 'static {
        move |row| {
            let pred = row.0[self.column_offset] == self.value;
            pred
        }
    }
}

struct SelectCommand<'a> {
    view: btree_map::Values<'a, usize, Row>,
    filter: Box<dyn Fn(&Row) -> bool>,
}

impl<'a> SelectCommand<'a> {
    fn new_table_scan(table: &'a Table, where_clause: WhereClause) -> Self {
        Self {
            view: table.rows.values(),
            filter: Box::new(where_clause.operationalize()),
        }
    }

    fn execute(self) -> Vec<&'a Row> {
        // XXX: `for` loops are so pedestrian
        let mut results = Vec::new();
        for row in self.view {
            if (self.filter)(row) {
                results.push(row);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::*;

    fn example_table() -> Table {
        let mut schema = TableSchema::new();
        schema.add_column("title".to_owned(), ColumnType::String);
        schema.add_column("year".to_owned(), ColumnType::Integer);
        let mut table = Table::new(schema);
        table
            .insert(Row(vec![
                Chamber::Key(0),
                Chamber::String("Men Trapped In Men's Bodies".to_owned()),
                Chamber::Integer(2013),
            ]))
            .unwrap();
        table
            .insert(Row(vec![
                Chamber::Key(0),
                Chamber::String("Galileo's Middle Finger".to_owned()),
                Chamber::Integer(2015),
            ]))
            .unwrap();
        table
            .insert(Row(vec![
                Chamber::Key(0),
                Chamber::String("Thing Explainer".to_owned()),
                Chamber::Integer(2015),
            ]))
            .unwrap();
        table
    }

    #[test]
    fn concerning_select_by_primary_key() {
        let table = example_table();
        let where_clause = WhereClause::new_column_equality(
            &table.schema,
            "pk".to_owned(),
            Chamber::Key(2),
        ).unwrap();
        let select_command =
            SelectCommand::new_table_scan(&table, where_clause);
        let result_rows = select_command.execute();
        assert_eq!(result_rows.len(), 1);
        assert_eq!(
            result_rows[0].0[1],
            Chamber::String("Galileo's Middle Finger".to_owned())
        );
    }

    #[test]
    fn concerning_select_by_integer() {
        let table = example_table();
        let where_clause = WhereClause::new_column_equality(
            &table.schema,
            "year".to_owned(),
            Chamber::Integer(2015),
        ).unwrap();
        let select_command =
            SelectCommand::new_table_scan(&table, where_clause);
        let result_rows = select_command.execute();
        assert_eq!(result_rows.len(), 2);
        assert_eq!(
            vec![
                &Chamber::String("Galileo's Middle Finger".to_owned()),
                &Chamber::String("Thing Explainer".to_owned()),
            ],
            result_rows
                .iter()
                .map(|row| &row.0[1])
                .collect::<Vec<_>>(),
        );
    }

}
