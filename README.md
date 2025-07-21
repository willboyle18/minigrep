Mini version of the grep command line program written in Rust.

To use specify a query you are searching for along with a text file (poem.txt is included here for testing)

ex: cargo run query examplefile.txt

To search with case insensitivity, set IGNORE_CASE=1

ex: IGNORE_CASE=1 cargo run query examplefile.txt
