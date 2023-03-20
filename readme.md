# Line Diff Count

I created this project to learn more about the Rust. While making this little
cli, I am reading the second edition of 'The Rust Programming Language' as well
as 'Rust for Rustaceons' and making changes as I go along. For this reason I 
will not be documenting very elaborately. If someone somehow finds some genuine
interest, I might do a better job at that. The documentation might not be up-to-date
for aforementioned reasons. I won't publish it to cargo (yet?) so everything is
based on running it with cargo.

This program compares two files line by line and gives you the difference in
a summary. It shows the amount of lines that are in file a that are not in file
b and vice versa. 

Example with a test file of 999999 word files. Running
```zsh
cargo run -- filea fileb
```
produces the following result (truncated for readability).
```text
file a results: 
+	29	wonderful
+	55	political
+	41	baseball
+	25	lying
--- truncated ---
file b results: 
+	49	be
+	28	beneath
+	5	camera
+	10	operation
--- truncated ---
```
This output tells us that file a had 29 more occurences of the word 'wonderful'
than file b. Keep in mind that this is on a **line by line** basis, and not
word by word. These words are just for testing purposes.
