# META
~~~sexpr
(snapshot (type semantic) (description "Regular comments are tokens, notes are trivia"))
~~~
# SOURCE
~~~sysml
x /* comment */ // note
y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comments_and_notes.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 23)) (message "expected a specific keyword or punctuation token"))
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 24) (line 2) (column 1) (len 1)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
x /* comment */ // note

y
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (malformed (code "expected_keyword") (found "x /* comment */ // note") (span (offset 0) (line 1) (column 1) (len 24))) (malformed (code "expected_keyword") (found "y") (span (offset 24) (line 2) (column 1) (len 1))))
)
~~~
