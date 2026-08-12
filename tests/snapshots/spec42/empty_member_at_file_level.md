# META
~~~sexpr
(snapshot (type semantic) (description "Empty member (bare semicolon) at file level"))
~~~
# SOURCE
~~~sysml
; in v : SpeedVal
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_at_file_level.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 17)) (message "expected a specific keyword or punctuation token"))
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 2) (line 1) (column 3) (len 15)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
;

in v : SpeedVal
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (malformed (code "expected_keyword") (found "; in v : SpeedVal") (span (offset 0) (line 1) (column 1) (len 1))) (malformed (code "expected_keyword") (found "in v : SpeedVal") (span (offset 2) (line 1) (column 3) (len 15))))
)
~~~
