use nom::{alphanumeric1, digit1, multispace0, multispace1};

use crate::table::Chamber;

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

#[derive(Debug, PartialEq, Eq)]
crate struct InsertStatement {
    table_name: String,
    values: Vec<Chamber>,
}

named!(commaspace <&str, char>,
   delimited!(
       multispace0,
       char!(','),
       multispace0
   )
);

named!(string_literal <&str, Chamber>,
    do_parse!(
        value: delimited!(
            char!('\''),
            take_until!("'"),
            char!('\'')
        ) >>
        (Chamber::String(value.to_owned()))
    )
);

named!(integer_literal <&str, Chamber>,
    do_parse!(
        value: digit1 >>
        (Chamber::Integer(value.parse().unwrap()))
    )
);

named!(literal <&str, Chamber>,
    alt!(integer_literal | string_literal)
);

named!(parse_values <&str, Vec<Chamber>>,
    delimited!(
        char!('('),
        separated_list!(commaspace, literal),
        char!(')')
    )
);

named!(parse_insert_statement<&str, InsertStatement>,
    do_parse!(
        tag!("INSERT") >>
        multispace1 >>
        tag!("INTO") >>
        multispace1 >>
        table_name: alphanumeric1 >>
        multispace1 >>
        tag!("VALUES") >>
        multispace1 >>
        values: parse_values >>
        multispace0 >>
        tag!(";") >>
        (InsertStatement {
            table_name: table_name.to_string(),
            values
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn concerning_the_parsing_of_literals() {
        assert_eq!(
            string_literal("'hello SQL world'"),
            Ok((
                "",
                Chamber::String("hello SQL world".to_owned())
            ))
        );
        assert_eq!(
            literal("'hello SQL world'"),
            Ok((
                "",
                Chamber::String("hello SQL world".to_owned())
            ))
        );
        assert_eq!(
            integer_literal("9001 "),
            Ok((" ", Chamber::Integer(9001)))
        );
        assert_eq!(
            literal("9001 "),
            Ok((" ", Chamber::Integer(9001)))
        )
    }

    #[test]
    fn concerning_the_parsing_of_value_lists() {
        assert_eq!(
            parse_values("(1, 'Structure and Interpretation')"),
            Ok((
                "",
                vec![
                    Chamber::Integer(1),
                    Chamber::String("Structure and Interpretation".to_owned()),
                ]
            ))
        )
    }

    #[test]
    fn concerning_parsing_an_insert_integers_statement() {
        assert_eq!(
            parse_insert_statement("INSERT INTO prices VALUES (120, 8401);"),
            Ok((
                "",
                InsertStatement {
                    table_name: "prices".to_owned(),
                    values: vec![
                        Chamber::Integer(120),
                        Chamber::Integer(8401),
                    ],
                }
            ))
        );
    }

    #[test]
    fn concerning_parsing_an_insert_statement() {
        assert_eq!(
            parse_insert_statement(
                "INSERT INTO books VALUES \
                 ('Mathematical Analysis: A Concise Introduction', 2007);"
            ),
            Ok((
                "",
                InsertStatement {
                    table_name: "books".to_owned(),
                    values: vec![
                        Chamber::String(
                            "Mathematical Analysis: A Concise Introduction"
                                .to_owned(),
                        ),
                        Chamber::Integer(2007),
                    ],
                }
            ))
        );
    }

}
