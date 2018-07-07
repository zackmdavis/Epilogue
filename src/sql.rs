use nom::{alphanumeric1, digit1, multispace0, multispace1};

#[derive(Debug, PartialEq, Eq)]
crate enum ColumnClause {
    Star,
    #[allow(dead_code)]
    Names(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
crate struct WhereClause {
    column_name: String,
    // XXX TODO: this actually needs to be a `Chamber`
    // and how will we detect Integer vs. Key?!
    value: isize,
}

#[derive(Debug, PartialEq, Eq)]
crate struct SelectStatement {
    column_names: ColumnClause,
    table_name: String,
    where_clause: WhereClause,
}

named!(parse_where_clause<&str, WhereClause>,
    do_parse!(
        tag!("WHERE") >>
        multispace1 >>
        column_name: alphanumeric1 >>
        multispace0 >>
        tag!("=") >>
        multispace0 >>
        value: digit1 >>
        (WhereClause { column_name: column_name.to_owned(),
                       value: value.parse().unwrap() })
    )
);

named!(parse_select_statement<&str, SelectStatement>,
   do_parse!(
       tag!("SELECT") >>
       multispace1 >>
       tag!("*") >>
       multispace1 >>
       tag!("FROM") >>
       multispace1 >>
       table_name: alphanumeric1 >>
       multispace1 >>
       where_clause: parse_where_clause >>
       multispace0 >>
       tag!(";") >>
       (SelectStatement { column_names: ColumnClause::Star,
                          table_name: table_name.to_string(),
                          where_clause })
   )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::table::Chamber;

    #[test]
    fn concerning_parsing_a_where_clause_for_an_integer_column() {
        assert_eq!(
            parse_where_clause("WHERE year = 2018 "),
            Ok((
                " ",
                WhereClause {
                    column_name: "year".to_owned(),
                    value: 2018isize
                }
            ))
        );
    }

    #[test]
    fn concerning_parsing_a_select_star_statement() {
        assert_eq!(
            parse_select_statement("SELECT * FROM books WHERE year = 2018;"),
            Ok((
                "",
                SelectStatement {
                    column_names: ColumnClause::Star,
                    table_name: "books".to_owned(),
                    where_clause: WhereClause {
                        column_name: "year".to_owned(),
                        value: 2018isize
                    },
                }
            ))
        );
    }

}
