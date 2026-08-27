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
  (root (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "expected_keyword") (found "in v : SpeedVal") (span (offset 2) (line 1) (column 3) (len 15))))
)
~~~
