# META
~~~sexpr
(snapshot (type semantic) (description "interface usage subsetting and redefinition clauses (:>/:>>)"))
~~~
# SOURCE
~~~sysml
package InterfaceUsageSubsettingExample {
    part a;
    part b;
    interface i : I :> baseI connect a to b;
    interface :>> redefinedI connect a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_usage_subsetting.md"
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
    (reference r0 (scope relative) (span (offset 84) (line 4) (column 19) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 84) (line 4) (column 19) (len 1)))))
    (reference r1 (scope relative) (span (offset 103) (line 4) (column 38) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 103) (line 4) (column 38) (len 1)))))
    (reference r2 (scope relative) (span (offset 108) (line 4) (column 43) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 108) (line 4) (column 43) (len 1)))))
    (reference r3 (scope relative) (span (offset 148) (line 5) (column 38) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 148) (line 5) (column 38) (len 1)))))
    (reference r4 (scope relative) (span (offset 153) (line 5) (column 43) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 153) (line 5) (column 43) (len 1)))))
  )
  (root (package (name "InterfaceUsageSubsettingExample") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r1)))) (to (interface-end (multiplicity none) (target (ref r2)))))) (body semicolon)) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r3)))) (to (interface-end (multiplicity none) (target (ref r4)))))) (body semicolon)))))
)
~~~
