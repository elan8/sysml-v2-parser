# META
~~~sexpr
(snapshot (type semantic) (description "Direct upstream KerML Simple Tests/Inverses.kerml: featured-by remains ordered before the following inverse-of relationship."))
~~~
# SOURCE
~~~sysml
package Inverses {
    class A {
        feature f : B inverse of B::g disjoint from h;
        feature h : B;
    }
    class B {
        feature g : A;
    }
    inverse B::g of A::f;
    inverting Invert inverse B::g.f of A::h;
    feature gg : A featured by B inverse of A::f;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_inverses_type_featuring.md"
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
    (reference r0 (scope relative) (span (offset 53) (line 3) (column 21) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 53) (line 3) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 66) (line 3) (column 34) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 66) (line 3) (column 34) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 69) (line 3) (column 37) (len 1)))))
    (reference r2 (scope relative) (span (offset 85) (line 3) (column 53) (len 1)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 85) (line 3) (column 53) (len 1)))))
    (reference r3 (scope relative) (span (offset 108) (line 4) (column 21) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 108) (line 4) (column 21) (len 1)))))
    (reference r4 (scope relative) (span (offset 151) (line 7) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 151) (line 7) (column 21) (len 1)))))
    (reference r5 (scope relative) (span (offset 172) (line 9) (column 13) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 172) (line 9) (column 13) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 175) (line 9) (column 16) (len 1)))))
    (reference r6 (scope relative) (span (offset 180) (line 9) (column 21) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 180) (line 9) (column 21) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 183) (line 9) (column 24) (len 1)))))
    (reference r7 (scope relative) (span (offset 215) (line 10) (column 30) (len 6)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 215) (line 10) (column 30) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 218) (line 10) (column 33) (len 1))) (segment 2 (token "f") (name "f") (separator dot) (span (offset 220) (line 10) (column 35) (len 1)))))
    (reference r8 (scope relative) (span (offset 225) (line 10) (column 40) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 225) (line 10) (column 40) (len 1))) (segment 1 (token "h") (name "h") (separator colon-colon) (span (offset 228) (line 10) (column 43) (len 1)))))
    (reference r9 (scope relative) (span (offset 248) (line 11) (column 18) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 248) (line 11) (column 18) (len 1)))))
    (reference r10 (scope relative) (span (offset 262) (line 11) (column 32) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 262) (line 11) (column 32) (len 1)))))
    (reference r11 (scope relative) (span (offset 275) (line 11) (column 45) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 275) (line 11) (column 45) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 278) (line 11) (column 48) (len 1)))))
  )
  (root (package (name "Inverses") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships (inverse-of (ref r1)) (type-relationship (keyword disjoint from) (targets (ref r2)))) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "h") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract false) (name "B") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "g") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-relationship (keyword inverse) (declaration-keyword false) (source (ref r5)) (target (ref r6))) (kerml-relationship (keyword inverse) (declaration-keyword true) (source (ref r7)) (target (ref r8))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "gg") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships (featured-by (ref r10)) (inverse-of (ref r11))) (value none) (body semicolon)))))
)
~~~
