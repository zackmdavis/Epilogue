#![allow(dead_code)]

use std::collections::btree_map;
use std::error::Error;

use crate::table::{Chamber, Row, Table, TableSchema};

#[derive(Debug)]
struct WhereSubcommand {
    // XXX TODO: starting out with supporting just a single `column = value`
    // condition, but should eventually expand to conj-/dis-junctions, &c.
    //
    // Use a column offset (don't want to overload the word "index") instead of
    // a name so that we can operate on Row directly rather than looking up the
    // name
    column_offset: usize,
    value: Chamber,
}

fn column_names_to_offsets(
    schema: &TableSchema,
    column_names: &[String],
) -> Result<Vec<usize>, Box<dyn Error>> {
    let offsets = schema
        .layout
        .iter()
        .enumerate()
        .filter_map(|(i, column)| {
            if column_names.contains(&column.name) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if offsets.len() == column_names.len() {
        Ok(offsets)
    } else {
        // TODO: more precise error message
        Err(From::from("some column names not found"))
    }
}

impl WhereSubcommand {
    fn new_column_equality(
        schema: &TableSchema,
        column_name: String,
        value: Chamber,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            column_offset: column_names_to_offsets(schema, &[column_name])?[0],
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
    column_offsets: Vec<usize>,
    view: btree_map::Values<'a, usize, Row>,
    filter: Box<dyn Fn(&Row) -> bool>,
}

impl<'a> SelectCommand<'a> {
    fn new_table_scan(
        table: &'a Table,
        column_names: Vec<String>,
        where_clause: WhereSubcommand,
    ) -> Self {
        Self {
            column_offsets: column_names_to_offsets(
                &table.schema,
                &column_names,
            ).unwrap(),
            view: table.rows.values(),
            filter: Box::new(where_clause.operationalize()),
        }
    }

    fn execute(self) -> Vec<Vec<&'a Chamber>> {
        // XXX: `for` loops are so pedestrian
        // XXX: overloading the word "result"?
        let mut results = Vec::new();
        for row in self.view {
            if (self.filter)(row) {
                let mut result = Vec::new();
                for (i, chamber) in row.0.iter().enumerate() {
                    if self.column_offsets.contains(&i) {
                        result.push(chamber);
                    }
                }
                results.push(result);
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
    fn concerning_converting_column_names_to_offsets() {
        let table = example_table();
        assert_eq!(
            column_names_to_offsets(
                &table.schema,
                &["title".to_owned(), "year".to_owned()]
            ).unwrap(),
            vec![1, 2]
        );
    }

    #[test]
    fn concerning_select_by_primary_key() {
        let table = example_table();
        let where_clause = WhereSubcommand::new_column_equality(
            &table.schema,
            "pk".to_owned(),
            Chamber::Key(2),
        ).unwrap();
        let select_command = SelectCommand::new_table_scan(
            &table,
            vec!["title".to_owned()],
            where_clause,
        );
        let result_rows = select_command.execute();
        assert_eq!(result_rows.len(), 1);
        assert_eq!(
            &Chamber::String("Galileo's Middle Finger".to_owned()),
            result_rows[0][0],
        );
    }

    #[test]
    fn concerning_select_by_integer() {
        let table = example_table();
        let where_clause = WhereSubcommand::new_column_equality(
            &table.schema,
            "year".to_owned(),
            Chamber::Integer(2015),
        ).unwrap();
        let select_command = SelectCommand::new_table_scan(
            &table,
            vec!["title".to_owned()],
            where_clause,
        );
        let result_rows = select_command.execute();
        assert_eq!(result_rows.len(), 2);
        assert_eq!(
            vec![
                vec![&Chamber::String(
                    "Galileo's Middle Finger".to_owned(),
                )],
                vec![&Chamber::String("Thing Explainer".to_owned())],
            ],
            result_rows
        );
    }

}
