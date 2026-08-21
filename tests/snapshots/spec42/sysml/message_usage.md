# META
~~~sexpr
(snapshot (type semantic) (description "SysML message usage (isAbstract flow) in a package body, named and anonymous forms"))
~~~
# SOURCE
~~~sysml
package MessageUsageExample {
    message msg1 of Payload from a to b;
    message msg2 from a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "message_usage.md"
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
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 21) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 50) (line 2) (column 21) (len 7)))))
    (reference r1 (scope relative) (span (offset 63) (line 2) (column 34) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 63) (line 2) (column 34) (len 1)))))
    (reference r2 (scope relative) (span (offset 68) (line 2) (column 39) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 68) (line 2) (column 39) (len 1)))))
    (reference r3 (scope relative) (span (offset 93) (line 3) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 93) (line 3) (column 23) (len 1)))))
    (reference r4 (scope relative) (span (offset 98) (line 3) (column 28) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 98) (line 3) (column 28) (len 1)))))
  )
  (root (package (name "MessageUsageExample") (body brace (flow-usage (kind message) (declaration (declared (name "msg1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r0)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r1)) (references none))) (to (connector-end (multiplicity none) (target (ref r2)) (references none)))))) (body (body semicolon))) (flow-usage (kind message) (declaration (declared (name "msg2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r3)) (references none))) (to (connector-end (multiplicity none) (target (ref r4)) (references none)))))) (body (body semicolon))))))
)
~~~
