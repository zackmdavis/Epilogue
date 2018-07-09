# Epilogue

I want to write a database!!

I want to call my database _Epilogue_ (like a "sequel", but smaller). Querying the Overmind in its gopher aspect for _epilogue database_, it looks like [dchester/epilogue](https://github.com/dchester/epilogue) may have scooped me on this naming idea by five years. That's probably OK.

## Demo!

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/epilogue`
Welcome to Epilogue (pre-α)!
There is a table 'books' with string column 'title' and integer column 'year'.
Epilogue>> INSERT INTO books VALUES ('Diaspora', 1997);
Ok(Insert(1))
Epilogue>> INSERT INTO books VALUES ('Into Thin Air', 1997);                  
Ok(Insert(1))
Epilogue>> INSERT INTO books VALUES ('Structure and Interpretation of Computer Programs', 1979);
Ok(Insert(1))
Epilogue>> SELECT title FROM books WHERE year = 1997;
Ok(Select([[String("Diaspora")], [String("Into Thin Air")]]))
Epilogue>> SELECT * FROM books WHERE year = 1979;
Ok(Select([[Key(3), String("Structure and Interpretation of Computer Programs"), Integer(1979)]]))
Epilogue>> PRINT
+----+---------------------------------------------------+------+
| pk | title                                             | year |
+----+---------------------------------------------------+------+
| 1  | Diaspora                                          | 1997 |
| 2  | Into Thin Air                                     | 1997 |
| 3  | Structure and Interpretation of Computer Programs | 1979 |
+----+---------------------------------------------------+------+

Epilogue>>
```
