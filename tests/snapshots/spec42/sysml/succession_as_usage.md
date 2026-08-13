# META
~~~sexpr
(snapshot (type semantic) (description "SysML SuccessionAsUsage (first/then connector) in a package body, named and bare forms"))
~~~
# SOURCE
~~~sysml
package SuccessionAsUsageExample {
    succession s1 : AB first a then b;
    first a then b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_as_usage.md"
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
    (reference r0 (scope relative) (span (offset 64) (line 2) (column 30) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 64) (line 2) (column 30) (len 1)))))
    (reference r1 (scope relative) (span (offset 71) (line 2) (column 37) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 71) (line 2) (column 37) (len 1)))))
    (reference r2 (scope relative) (span (offset 84) (line 3) (column 11) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 84) (line 3) (column 11) (len 1)))))
    (reference r3 (scope relative) (span (offset 91) (line 3) (column 18) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 91) (line 3) (column 18) (len 1)))))
  )
  (root (package (name "SuccessionAsUsageExample") (body (first (source (expression (span (offset 64) (line 2) (column 30) (len 1)) (ref r0))) (target (expression (span (offset 71) (line 2) (column 37) (len 1)) (ref r1))) (body semicolon)) (first (source (expression (span (offset 84) (line 3) (column 11) (len 1)) (ref r2))) (target (expression (span (offset 91) (line 3) (column 18) (len 1)) (ref r3))) (body semicolon)))))
)
~~~
