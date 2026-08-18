# META
~~~sexpr
(snapshot (type semantic) (description "Coverage: Connector from/to forms, binding connector variants, connector specializations"))
~~~
# SOURCE
~~~sysml
part def A { port p1; port p2; }
part def B { port q1; port q2; }

part def System {
    part a : A;
    part b : B;

    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding of a.p2 = b.q2;

    ref part engine : A;
    individual part myA : A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_connectors.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 122) (line 8) (column 5) (len 139)) (message "unrecognized declaration `connector` in part definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
part def A {
    port p1;
    port p2;
}

part def B {
    port q1;
    port q2;
}

part def System {
    part a : A;
    part b : B;
    connector c1 from a.p1 to b.q1;
    connector c2 :> c1 from a.p2 to b.q2;

    binding b1 of a.p1 = b.q1;
    binding of a.p2 = b.q2;
    ref part engine : A;
    individual part myA : A;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 98) (line 5) (column 14) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 98) (line 5) (column 14) (len 1)))))
    (reference r1 (scope relative) (span (offset 114) (line 6) (column 14) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 114) (line 6) (column 14) (len 1)))))
    (reference r2 (scope relative) (span (offset 279) (line 14) (column 23) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 279) (line 14) (column 23) (len 1)))))
    (reference r3 (scope relative) (span (offset 308) (line 15) (column 27) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 308) (line 15) (column 27) (len 1)))))
  )
  (root (part-def (name "A") (body brace (port-usage (declaration-name "p1") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "p2") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "B") (body brace (port-usage (declaration-name "q1") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "q2") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "System") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "connector c1 from a.p1 to b.q1;") (span (offset 122) (line 8) (column 5) (len 139))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "myA") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
