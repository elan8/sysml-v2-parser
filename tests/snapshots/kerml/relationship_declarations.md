# META
~~~sexpr
(snapshot (type semantic) (description "KerML explicit relationship declarations (specialization/subclassifier/typing/subset/redefinition/disjoining/inverse/featuring), general type declarations with unions/intersects/differences, bare classifier forward declarations as typed nodes, plain feature members, and keyword-less implicit-feature package members."))
~~~
# SOURCE
~~~sysml
package RelationshipCoverage {
    type A; type B;
    feature f : Integer;
    feature g;
    specialization S subclassifier A specializes B;
    subclassifier A specializes B;
    typing t1 typing f typed by B;
    typing g : A;
    subset parent subsets f;
    redefinition child :>> parent;
    disjoining d1 disjoint A from B;
    disjoint C from D;
    inverse a.b of c.d;
    featuring F of y by C;
    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
    classifier X;
    causeA;
    y = f;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "relationship_declarations.md"
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
    feature f : Integer;
    feature g;
    specialization S subclassifier A specializes B;
    subclassifier A specializes B;
    typing t1 typing f typed by B;
    typing g typed by A;
    subset parent subsets f;
    redefinition child redefines parent;
    disjoining d1 disjoint A from B;
    disjoint C from D;
    inverse a.b of c.d;
    featuring F of y by C;
    type UnionType unions A, B;
    type InterType intersects A, B;
    type DiffType differences A, B;
    classifier X;
    causeA;
    y = f;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 3) (column 17) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 67) (line 3) (column 17) (len 7)))))
    (reference r1 (scope relative) (span (offset 126) (line 5) (column 36) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 126) (line 5) (column 36) (len 1)))))
    (reference r2 (scope relative) (span (offset 140) (line 5) (column 50) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 140) (line 5) (column 50) (len 1)))))
    (reference r3 (scope relative) (span (offset 161) (line 6) (column 19) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 161) (line 6) (column 19) (len 1)))))
    (reference r4 (scope relative) (span (offset 175) (line 6) (column 33) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 175) (line 6) (column 33) (len 1)))))
    (reference r5 (scope relative) (span (offset 199) (line 7) (column 22) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 199) (line 7) (column 22) (len 1)))))
    (reference r6 (scope relative) (span (offset 210) (line 7) (column 33) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 210) (line 7) (column 33) (len 1)))))
    (reference r7 (scope relative) (span (offset 224) (line 8) (column 12) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 224) (line 8) (column 12) (len 1)))))
    (reference r8 (scope relative) (span (offset 228) (line 8) (column 16) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 228) (line 8) (column 16) (len 1)))))
    (reference r9 (scope relative) (span (offset 242) (line 9) (column 12) (len 6)) (segments (segment 0 (token "parent") (name "parent") (separator none) (span (offset 242) (line 9) (column 12) (len 6)))))
    (reference r10 (scope relative) (span (offset 257) (line 9) (column 27) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 257) (line 9) (column 27) (len 1)))))
    (reference r11 (scope relative) (span (offset 277) (line 10) (column 18) (len 5)) (segments (segment 0 (token "child") (name "child") (separator none) (span (offset 277) (line 10) (column 18) (len 5)))))
    (reference r12 (scope relative) (span (offset 287) (line 10) (column 28) (len 6)) (segments (segment 0 (token "parent") (name "parent") (separator none) (span (offset 287) (line 10) (column 28) (len 6)))))
    (reference r13 (scope relative) (span (offset 322) (line 11) (column 28) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 322) (line 11) (column 28) (len 1)))))
    (reference r14 (scope relative) (span (offset 329) (line 11) (column 35) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 329) (line 11) (column 35) (len 1)))))
    (reference r15 (scope relative) (span (offset 345) (line 12) (column 14) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 345) (line 12) (column 14) (len 1)))))
    (reference r16 (scope relative) (span (offset 352) (line 12) (column 21) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 352) (line 12) (column 21) (len 1)))))
    (reference r17 (scope relative) (span (offset 367) (line 13) (column 13) (len 3)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 367) (line 13) (column 13) (len 1))) (segment 1 (token "b") (name "b") (separator dot) (span (offset 369) (line 13) (column 15) (len 1)))))
    (reference r18 (scope relative) (span (offset 374) (line 13) (column 20) (len 3)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 374) (line 13) (column 20) (len 1))) (segment 1 (token "d") (name "d") (separator dot) (span (offset 376) (line 13) (column 22) (len 1)))))
    (reference r19 (scope relative) (span (offset 398) (line 14) (column 20) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 398) (line 14) (column 20) (len 1)))))
    (reference r20 (scope relative) (span (offset 403) (line 14) (column 25) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 403) (line 14) (column 25) (len 1)))))
    (reference r21 (scope relative) (span (offset 548) (line 20) (column 9) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 548) (line 20) (column 9) (len 1)))))
  )
  (root (package (name "RelationshipCoverage") (body brace (kerml-classifier (keyword type) (abstract false) (name "A") (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "B") (specializes none) (conjugates none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "g") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-relationship (keyword subclassifier) (declaration-keyword true) (source (ref r1)) (target (ref r2))) (kerml-relationship (keyword subclassifier) (declaration-keyword false) (source (ref r3)) (target (ref r4))) (kerml-relationship (keyword typing) (declaration-keyword false) (source (ref r5)) (target (ref r6))) (kerml-relationship (keyword typing) (declaration-keyword false) (source (ref r7)) (target (ref r8))) (kerml-relationship (keyword subset) (declaration-keyword false) (source (ref r9)) (target (ref r10))) (kerml-relationship (keyword redefinition) (declaration-keyword false) (source (ref r11)) (target (ref r12))) (kerml-relationship (keyword disjoint) (declaration-keyword true) (source (ref r13)) (target (ref r14))) (kerml-relationship (keyword disjoint) (declaration-keyword false) (source (ref r15)) (target (ref r16))) (kerml-relationship (keyword inverse) (declaration-keyword false) (source (ref r17)) (target (ref r18))) (kerml-relationship (keyword featuring) (declaration-keyword false) (source (ref r19)) (target (ref r20))) (kerml-classifier (keyword type) (abstract false) (name "UnionType") (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "InterType") (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "DiffType") (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword classifier) (abstract false) (name "X") (specializes none) (conjugates none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "causeA") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "y") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 548) (line 20) (column 9) (len 1)) (ref r21))))) (body semicolon)))))
)
~~~
