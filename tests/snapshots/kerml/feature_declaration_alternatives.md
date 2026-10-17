# META
~~~sexpr
(snapshot (type semantic) (description "KerML FeatureDeclaration alternatives that have typed parsers: a FeatureIdentification short name, a ConjugationPart instead of a FeatureSpecializationPart, feature-chain operands of type relationship parts, an Invariant's true/false keyword with a FeatureDeclaration and a semicolon body, and a package-level Succession whose first is optional.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package P {
    classifier A { feature f; feature g; }
    feature b : A;
    feature h1;
    feature <sn> shortNamed : A;
    feature <anon> : A;
    feature conjugatedByOperator ~ A::f;
    feature conjugatedByKeyword conjugates A::f;
    feature h2 differences b.f, b.g intersects b.f disjoint from h1;
    inv false negated { false }
    inv i :> h1;
    inv true asserted { true }
    step s1;
    step s2;
    succession [1] s1 then [*] s2;
    succession all [*] s1 then [1] s2;
    classifier C {
        feature nested ~ A::g;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_declaration_alternatives.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    classifier A {
        feature f;
        feature g;
    }
    feature b : A;
    feature h1;
    feature <sn> shortNamed : A;
    feature <anon> : A;
    feature conjugatedByOperator ~ A::f;
    feature conjugatedByKeyword conjugates A::f;
    feature h2 differences b.f, b.g intersects b.f disjoint from h1;
    inv false negated {
        false
    }
    inv i :> h1;
    inv true asserted {
        true
    }
    step s1;
    step s2;
    succession [1] s1 then [*] s2;
    succession all [*] s1 then [1] s2;
    classifier C {
        feature nested ~ A::g;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 71) (line 3) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 71) (line 3) (column 17) (len 1)))))
    (reference r1 (scope relative) (span (offset 120) (line 5) (column 31) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 120) (line 5) (column 31) (len 1)))))
    (reference r2 (scope relative) (span (offset 144) (line 6) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 144) (line 6) (column 22) (len 1)))))
    (reference r3 (scope relative) (span (offset 182) (line 7) (column 36) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 182) (line 7) (column 36) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 185) (line 7) (column 39) (len 1)))))
    (reference r4 (scope relative) (span (offset 231) (line 8) (column 44) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 231) (line 8) (column 44) (len 1))) (segment 1 (token "f") (name "f") (separator colon-colon) (span (offset 234) (line 8) (column 47) (len 1)))))
    (reference r5 (scope relative) (span (offset 264) (line 9) (column 28) (len 3)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 264) (line 9) (column 28) (len 1))) (segment 1 (token "f") (name "f") (separator dot) (span (offset 266) (line 9) (column 30) (len 1)))))
    (reference r6 (scope relative) (span (offset 269) (line 9) (column 33) (len 3)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 269) (line 9) (column 33) (len 1))) (segment 1 (token "g") (name "g") (separator dot) (span (offset 271) (line 9) (column 35) (len 1)))))
    (reference r7 (scope relative) (span (offset 284) (line 9) (column 48) (len 3)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 284) (line 9) (column 48) (len 1))) (segment 1 (token "f") (name "f") (separator dot) (span (offset 286) (line 9) (column 50) (len 1)))))
    (reference r8 (scope relative) (span (offset 302) (line 9) (column 66) (len 2)) (segments (segment 0 (token "h1") (name "h1") (separator none) (span (offset 302) (line 9) (column 66) (len 2)))))
    (reference r9 (scope relative) (span (offset 351) (line 11) (column 14) (len 2)) (segments (segment 0 (token "h1") (name "h1") (separator none) (span (offset 351) (line 11) (column 14) (len 2)))))
    (reference r10 (scope relative) (span (offset 431) (line 15) (column 20) (len 2)) (segments (segment 0 (token "s1") (name "s1") (separator none) (span (offset 431) (line 15) (column 20) (len 2)))))
    (reference r11 (scope relative) (span (offset 443) (line 15) (column 32) (len 2)) (segments (segment 0 (token "s2") (name "s2") (separator none) (span (offset 443) (line 15) (column 32) (len 2)))))
    (reference r12 (scope relative) (span (offset 470) (line 16) (column 24) (len 2)) (segments (segment 0 (token "s1") (name "s1") (separator none) (span (offset 470) (line 16) (column 24) (len 2)))))
    (reference r13 (scope relative) (span (offset 482) (line 16) (column 36) (len 2)) (segments (segment 0 (token "s2") (name "s2") (separator none) (span (offset 482) (line 16) (column 36) (len 2)))))
    (reference r14 (scope relative) (span (offset 530) (line 18) (column 26) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 530) (line 18) (column 26) (len 1))) (segment 1 (token "g") (name "g") (separator colon-colon) (span (offset 533) (line 18) (column 29) (len 1)))))
  )
  (root (package (name "P") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "A") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "g") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "b") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "h1") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "shortNamed") (short-name "sn") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (short-name "anon") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "conjugatedByOperator") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (conjugates (operator (ref r3))) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "conjugatedByKeyword") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (conjugates (keyword (ref r4))) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "h2") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships (type-relationship (keyword differences) (targets (ref r5) (ref r6))) (type-relationship (keyword intersects) (targets (ref r7))) (type-relationship (keyword disjoint from) (targets (ref r8)))) (value none) (body semicolon)) (kerml-invariant (negated true) (name "negated")) (kerml-invariant (negated false) (name "i") (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r9))) (value none)))) (kerml-invariant (negated false) (name "asserted")) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "s1") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "s2") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-succession (visibility none) (all false) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword false) (first (connector-end (multiplicity (lower none) (upper (expression (span (offset 428) (line 15) (column 17) (len 1)) (integer 1)))) (target (ref r10)) (references none))) (then (connector-end (multiplicity (lower none) (upper (expression (span (offset 440) (line 15) (column 29) (len 1)) (infinity)))) (target (ref r11)) (references none)))) (kerml-succession (visibility none) (all true) (name none) (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (first-keyword false) (first (connector-end (multiplicity (lower none) (upper (expression (span (offset 467) (line 16) (column 21) (len 1)) (infinity)))) (target (ref r12)) (references none))) (then (connector-end (multiplicity (lower none) (upper (expression (span (offset 479) (line 16) (column 33) (len 1)) (integer 1)))) (target (ref r13)) (references none)))) (kerml-classifier (keyword classifier) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "nested") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (conjugates (operator (ref r14))) (relationships) (value none) (body semicolon)))))))
)
~~~
