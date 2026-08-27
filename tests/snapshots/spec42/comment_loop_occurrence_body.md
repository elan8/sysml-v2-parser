# META
~~~sexpr
(snapshot (type semantic) (description "An occurrence usage body owns AnnotatingElement members. A regular comment immediately before a later typed sibling must survive the body-loop boundary and preserve that sibling."))
~~~
# SOURCE
~~~sysml
package CommentLoopOccurrence {
    occurrence subject : Subject {
        /* occurrence body comment */
        snapshot instant : Instant;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_loop_occurrence_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "CommentLoopOccurrence") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "subject") (short-name none) (target none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 77) (line 3) (column 11) (len 25)) (normalized "occurrence body comment "))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "instant") (short-name none) (target none) (body semicolon)))))))
)
~~~
