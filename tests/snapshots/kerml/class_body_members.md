# META
~~~sexpr
(snapshot (type semantic) (description "KerML class/struct/datatype bodies accept feature members with member/composite/portion/var prefixes, step/expr/bool kinds, invariants, connectors with reference ends, nested class definitions, and var-prefixed directed parameters."))
~~~
# SOURCE
~~~sysml
package Classes {
    class A {
        feature innerSpaceDimension : Natural [1];
        member feature x : B;
        composite feature engine subsets carParts;
        portion feature all portions: Occurrence[1..*];
        var t : Integer;
        inv checkIt { x > 1 }
        connector a ::> a.x to b;
        class Inner { feature y : Integer; }
    }
    behavior B2 { out var y1; }
    package Q { connector a2 from x.s to y.t; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "class_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Classes {
    class A {
        feature innerSpaceDimension : Natural[1];
        member feature x : B;
        composite feature engine subsets carParts;
        portion feature all portions : Occurrence[1..*];
        var t : Integer;
        inv checkIt {
            x > 1;
        }
        connector from a references a.x to b;
        class Inner {
            feature y : Integer;
        }
    }
    behavior B2 {
        out var y1;
    }
    package Q {
        connector a2 from x.s to y.t;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 70) (line 3) (column 39) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 70) (line 3) (column 39) (len 7)))))
    (reference r1 (scope relative) (span (offset 110) (line 4) (column 28) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 110) (line 4) (column 28) (len 1)))))
    (reference r2 (scope relative) (span (offset 154) (line 5) (column 42) (len 8)) (segments (segment 0 (token "carParts") (name "carParts") (separator none) (span (offset 154) (line 5) (column 42) (len 8)))))
    (reference r3 (scope relative) (span (offset 202) (line 6) (column 39) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 202) (line 6) (column 39) (len 10)))))
    (reference r4 (scope relative) (span (offset 236) (line 7) (column 17) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 236) (line 7) (column 17) (len 7)))))
    (reference r5 (scope relative) (span (offset 343) (line 10) (column 35) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 343) (line 10) (column 35) (len 7)))))
  )
  (root (package (name "Classes") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "innerSpaceDimension") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity (lower (expression (span (offset 79) (line 3) (column 48) (len 1)) (integer 1))) (upper (expression (span (offset 79) (line 3) (column 48) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "x") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "engine") (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r2))) (value none))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all true) (name "portions") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))) (multiplicity (lower (expression (span (offset 213) (line 6) (column 50) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability var) (metadata)) (kind none) (member false) (all false) (name "t") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (invariant) (connector) (kerml-classifier (keyword class) (abstract false) (name "Inner") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "y") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))) (kerml-classifier (keyword behavior) (abstract false) (name "B2") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction out) (derived false) (abstract false) (portion none) (variability var) (metadata)) (kind none) (member false) (all false) (name "y1") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (package (name "Q") (body brace (kerml-connector))))))
)
~~~
