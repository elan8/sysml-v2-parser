# META
~~~sexpr
(snapshot (type malformed) (description "A documentation member missing its body remains malformed after note trivia and recovery preserves later documentation and feature members."))
~~~
# SOURCE
~~~sysml
class C {
    doc <missing> // missing body
        unexpected;
    doc <valid> // note
        name /* later documentation */
    feature afterDoc;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "documentation_note_trivia_recovery.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 14) (line 2) (column 5) (len 54)) (message "missing semicolon before next declaration"))
    )
  )
)
~~~
# FORMAT
~~~sysml
class C {
    doc <missing> // missing body
        unexpected;
    doc <valid> name
    /* later documentation */
    feature afterDoc;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (kerml-classifier (keyword class) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "missing_semicolon") (found "doc <missing> // missing body") (span (offset 14) (line 2) (column 5) (len 54))) (doc (name "name") (short-name "valid") (locale none) (body (span (offset 103) (line 5) (column 16) (len 21)) (normalized "later documentation "))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "afterDoc") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
