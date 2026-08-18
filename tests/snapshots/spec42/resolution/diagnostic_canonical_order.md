# META
~~~sexpr
(snapshot (type semantic) (description "Diagnostics preserve canonical ordering for multiple unresolved type references"))
~~~
# SOURCE
~~~sysml
package P {
    part bad_first : MissingFirst;
    part bad_second : MissingSecond;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "diagnostic_canonical_order.md"
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
    (reference r0 (scope relative) (span (offset 33) (line 2) (column 22) (len 12)) (segments (segment 0 (token "MissingFirst") (name "MissingFirst") (separator none) (span (offset 33) (line 2) (column 22) (len 12)))))
    (reference r1 (scope relative) (span (offset 69) (line 3) (column 23) (len 13)) (segments (segment 0 (token "MissingSecond") (name "MissingSecond") (separator none) (span (offset 69) (line 3) (column 23) (len 13)))))
  )
  (root (package (name "P") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bad_first") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bad_second") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
