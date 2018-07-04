// I'm imagining we want data structures something like this?!—

// struct Database {
//     tables: HashMap<String, Table>
// }

// struct TableSchema {
//     name: String,
//     layout: Vec<ColumnType>
// }

// struct PrimaryKey(usize);

// struct Table {
//     schema: TableSchema,
//     rows: BTreeMap<PrimaryKey, Row>,
//     indices: Vec<BTreeMap<Chamber, Vec<PrimaryKey>>>
// }

// struct Row {
//     cells: Vec<Chamber>
// }

// enum Chamber {
//     Integer(isize),
//     String(String),
//     ForeignKey(usize)
// }

fn main() {
    println!("Hello Epilogue world!");
}
