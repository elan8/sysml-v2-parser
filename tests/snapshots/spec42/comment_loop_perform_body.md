# META
~~~sexpr
(snapshot (type semantic) (description "A PerformActionUsage body owns AnnotatingElement members before and between in/out bindings. The member-loop trivia boundary must leave a regular comment for the typed annotating parser."))
~~~
# SOURCE
~~~sysml
package CommentLoopPerform {
    part context {
        perform action work {
            /* perform body comment */
            in input = source;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_loop_perform_body.md"
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
    (reference r0 (scope relative) (span (offset 132) (line 5) (column 16) (len 5)) (segments (segment 0 (token "input") (name "input") (separator none) (span (offset 132) (line 5) (column 16) (len 5)))))
    (reference r1 (scope relative) (span (offset 140) (line 5) (column 24) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 140) (line 5) (column 24) (len 6)))))
  )
  (root (package (name "CommentLoopPerform") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name "work") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 92) (line 4) (column 15) (len 22)) (normalized "perform body comment "))) (binding (direction in) (target (ref r0)) (value (expression (span (offset 140) (line 5) (column 24) (len 6)) (ref r1)))))))))))
)
~~~
