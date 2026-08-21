# META
~~~sexpr
(snapshot (type semantic) (description "An InterfaceUsage body owns AnnotatingElement members. A regular comment before the closing brace must remain a typed comment rather than being consumed as trivia by the body loop."))
~~~
# SOURCE
~~~sysml
package CommentLoopInterface {
    part context {
        interface source to target {
            /* interface usage body comment */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_loop_interface_usage_body.md"
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
    (reference r0 (scope relative) (span (offset 68) (line 3) (column 19) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 68) (line 3) (column 19) (len 6)))))
    (reference r1 (scope relative) (span (offset 78) (line 3) (column 29) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 78) (line 3) (column 29) (len 6)))))
  )
  (root (package (name "CommentLoopInterface") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form connection) (part (binary (from (interface-end (multiplicity none) (target (ref r0)))) (to (interface-end (multiplicity none) (target (ref r1)))))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 101) (line 4) (column 15) (len 30)) (normalized "interface usage body comment "))))))))))
)
~~~
