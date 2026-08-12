# META
~~~sexpr
(snapshot (type semantic) (description "Coverage: Standalone relationship declarations (disjoining, typing, subsetting, redefinition)"))
~~~
# SOURCE
~~~sysml
package RelationshipCoverage {
    type A;
    type B;
    type C;
    type D;
    feature f;
    feature g;
    feature parent;
    feature child;

    disjoining d1 disjoint A from B;
    disjoint C from D;

    typing t1 typing f typed by B;
    typing g : A;

    subset parent subsets f;

    redefinition child :>> parent;
    redefinition f redefines g;

    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_relationships.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 35) (line 2) (column 5) (len 431)) (message "unrecognized declaration `type` in package body"))
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
  (root (package (name "RelationshipCoverage") (body (malformed (code "unrecognized_declaration_in_scope") (found "type A;") (span (offset 35) (line 2) (column 5) (len 431))))))
)
~~~
