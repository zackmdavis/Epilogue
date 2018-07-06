#![allow(dead_code)]

use std::collections::btree_map;

use crate::table::{Table, Row, Chamber};


#[derive(Debug)]
struct WhereClause {
    // XXX TODO: starting out with supporting just a single `column = value`
    // condition, but should eventually expand to conj-/dis-junctions, &c.
    //
    // Use a column offset (don't want to overload the word "index") instead of
    // a name so that we can operate on Row directly rather than looking up the
    // name
    column_offset: usize,
    value: Chamber
}

impl WhereClause {
    fn filter(self) -> impl Fn(&Row) -> bool + 'static {
        move |row| {
            let pred = row.0[self.column_offset] == self.value;
            pred
        }
    }
}


struct SelectCommand<'a> {
    view: btree_map::Values<'a, usize, Row>,
    filter: Box<dyn Fn(&Row) -> bool>
}

impl<'a> SelectCommand<'a> {
    fn table_scan(table: &'a Table, where_clause: WhereClause) -> Self {
        Self {
            view: table.rows.values(),
            filter: Box::new(where_clause.filter())
        }
    }

    fn execute(self) -> Vec<&'a Row> {
        // XXX: `for` loops are so pedestrian
        let mut results = Vec::new();
        for row in self.view {
            results.push(row);
        }
        results
    }
}
