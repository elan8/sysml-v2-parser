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
    )
  )
)
~~~
# FORMAT
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
    specialization t1 typing f typed by B;
    typing g typed by A;
    subset parent subsets f;
    redefinition child redefines parent;
    redefinition f redefines g;
    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 176) (line 11) (column 28) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 176) (line 11) (column 28) (len 1)))))
    (reference r1 (scope relative) (span (offset 183) (line 11) (column 35) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 183) (line 11) (column 35) (len 1)))))
    (reference r2 (scope relative) (span (offset 199) (line 12) (column 14) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 199) (line 12) (column 14) (len 1)))))
    (reference r3 (scope relative) (span (offset 206) (line 12) (column 21) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 206) (line 12) (column 21) (len 1)))))
    (reference r4 (scope relative) (span (offset 231) (line 14) (column 22) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 231) (line 14) (column 22) (len 1)))))
    (reference r5 (scope relative) (span (offset 242) (line 14) (column 33) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 242) (line 14) (column 33) (len 1)))))
    (reference r6 (scope relative) (span (offset 256) (line 15) (column 12) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 256) (line 15) (column 12) (len 1)))))
    (reference r7 (scope relative) (span (offset 260) (line 15) (column 16) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 260) (line 15) (column 16) (len 1)))))
    (reference r8 (scope relative) (span (offset 275) (line 17) (column 12) (len 6)) (segments (segment 0 (token "parent") (name "parent") (separator none) (span (offset 275) (line 17) (column 12) (len 6)))))
    (reference r9 (scope relative) (span (offset 290) (line 17) (column 27) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 290) (line 17) (column 27) (len 1)))))
    (reference r10 (scope relative) (span (offset 311) (line 19) (column 18) (len 5)) (segments (segment 0 (token "child") (name "child") (separator none) (span (offset 311) (line 19) (column 18) (len 5)))))
    (reference r11 (scope relative) (span (offset 321) (line 19) (column 28) (len 6)) (segments (segment 0 (token "parent") (name "parent") (separator none) (span (offset 321) (line 19) (column 28) (len 6)))))
    (reference r12 (scope relative) (span (offset 346) (line 20) (column 18) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 346) (line 20) (column 18) (len 1)))))
    (reference r13 (scope relative) (span (offset 358) (line 20) (column 30) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 358) (line 20) (column 30) (len 1)))))
  )
  (root (package (name "RelationshipCoverage") (body (kerml-classifier (keyword type) (abstract false) (name "A") (specializes none)) (kerml-classifier (keyword type) (abstract false) (name "B") (specializes none)) (kerml-classifier (keyword type) (abstract false) (name "C") (specializes none)) (kerml-classifier (keyword type) (abstract false) (name "D") (specializes none)) (kerml-feature (name "f")) (kerml-feature (name "g")) (kerml-feature (name "parent")) (kerml-feature (name "child")) (kerml-relationship (keyword disjoint) (source (ref r0)) (target (ref r1))) (kerml-relationship (keyword disjoint) (source (ref r2)) (target (ref r3))) (kerml-relationship (keyword typing) (source (ref r4)) (target (ref r5))) (kerml-relationship (keyword typing) (source (ref r6)) (target (ref r7))) (kerml-relationship (keyword subset) (source (ref r8)) (target (ref r9))) (kerml-relationship (keyword redefinition) (source (ref r10)) (target (ref r11))) (kerml-relationship (keyword redefinition) (source (ref r12)) (target (ref r13))) (kerml-classifier (keyword type) (abstract false) (name "UnionType") (specializes none)) (kerml-classifier (keyword type) (abstract false) (name "InterType") (specializes none)) (kerml-classifier (keyword type) (abstract false) (name "DiffType") (specializes none)))))
)
~~~
