# META
~~~sexpr
(snapshot (type semantic) (description "KerML Connector declarations take the full FeatureSpecializationPart (KerML BNF Connector/FeatureDeclaration 573-576, Pilot KerML.xtext 824-830): typing, subsetting, reference subsetting, redefinition, ordered/nonunique, and multiplicity in either position are retained as ordered typed specializations, both with body-owned ends and with from/to ends."))
~~~
# SOURCE
~~~sysml
package Connectors {
    classifier Holder {
        feature a;
        feature b;
        feature c;
        connector pair : Links::BinaryLink { end feature e1 :>> a; end feature e2 :>> b; }
        connector tern subsets links [1] { end feature e1 :>> a; end feature e2 :>> b; end feature e3 :>> c; }
        connector sub :> pair, tern redefines pair ordered nonunique from a to b;
        connector [0..1] typed by Links::BinaryLink references pair;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connector_feature_specialization_part.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Connectors {
    classifier Holder {
        feature a;
        feature b;
        feature c;
        connector pair : Links::BinaryLink {
            end feature e1 :>> a;
            end feature e2 :>> b;
        }
        connector tern subsets links[1] {
            end feature e1 :>> a;
            end feature e2 :>> b;
            end feature e3 :>> c;
        }
        connector sub :> pair, tern redefines pair ordered nonunique from a to b;
        connector typed by Links::BinaryLink references pair[0..1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 127) (line 6) (column 26) (len 17)) (segments (segment 0 (token "Links") (name "Links") (separator none) (span (offset 127) (line 6) (column 26) (len 5))) (segment 1 (token "BinaryLink") (name "BinaryLink") (separator colon-colon) (span (offset 134) (line 6) (column 33) (len 10)))))
    (reference r1 (scope relative) (span (offset 166) (line 6) (column 65) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 166) (line 6) (column 65) (len 1)))))
    (reference r2 (scope relative) (span (offset 188) (line 6) (column 87) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 188) (line 6) (column 87) (len 1)))))
    (reference r3 (scope relative) (span (offset 224) (line 7) (column 32) (len 5)) (segments (segment 0 (token "links") (name "links") (separator none) (span (offset 224) (line 7) (column 32) (len 5)))))
    (reference r4 (scope relative) (span (offset 255) (line 7) (column 63) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 255) (line 7) (column 63) (len 1)))))
    (reference r5 (scope relative) (span (offset 277) (line 7) (column 85) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 277) (line 7) (column 85) (len 1)))))
    (reference r6 (scope relative) (span (offset 299) (line 7) (column 107) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 299) (line 7) (column 107) (len 1)))))
    (reference r7 (scope relative) (span (offset 329) (line 8) (column 26) (len 4)) (segments (segment 0 (token "pair") (name "pair") (separator none) (span (offset 329) (line 8) (column 26) (len 4)))))
    (reference r8 (scope relative) (span (offset 335) (line 8) (column 32) (len 4)) (segments (segment 0 (token "tern") (name "tern") (separator none) (span (offset 335) (line 8) (column 32) (len 4)))))
    (reference r9 (scope relative) (span (offset 350) (line 8) (column 47) (len 4)) (segments (segment 0 (token "pair") (name "pair") (separator none) (span (offset 350) (line 8) (column 47) (len 4)))))
    (reference r10 (scope relative) (span (offset 378) (line 8) (column 75) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 378) (line 8) (column 75) (len 1)))))
    (reference r11 (scope relative) (span (offset 383) (line 8) (column 80) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 383) (line 8) (column 80) (len 1)))))
    (reference r12 (scope relative) (span (offset 420) (line 9) (column 35) (len 17)) (segments (segment 0 (token "Links") (name "Links") (separator none) (span (offset 420) (line 9) (column 35) (len 5))) (segment 1 (token "BinaryLink") (name "BinaryLink") (separator colon-colon) (span (offset 427) (line 9) (column 42) (len 10)))))
    (reference r13 (scope relative) (span (offset 449) (line 9) (column 64) (len 4)) (segments (segment 0 (token "pair") (name "pair") (separator none) (span (offset 449) (line 9) (column 64) (len 4)))))
  )
  (root (package (name "Connectors") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Holder") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "a") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "b") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "c") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-connector (all false) (name "pair") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from none) (to none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e1") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e2") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r2))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-connector (all false) (name "tern") (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (multiplicity (lower none) (upper (expression (span (offset 231) (line 7) (column 39) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (from none) (to none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e1") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r4))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e2") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r5))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "e3") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r6))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-connector (all false) (name "sub") (specializations (subsetting (relationship (kind subsets) (implied false) (targets (ref r7) (ref r8))) (value none)) (redefinition (relationship (kind redefines) (implied false) (targets (ref r9))))) (multiplicity none) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (from (connector-end (multiplicity none) (target (ref r10)) (references none))) (to (connector-end (multiplicity none) (target (ref r11)) (references none))) (body semicolon)) (kerml-connector (all false) (name none) (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r13))))) (multiplicity (lower (expression (span (offset 405) (line 9) (column 20) (len 1)) (integer 0))) (upper (expression (span (offset 408) (line 9) (column 23) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (from none) (to none) (body semicolon)))))))
)
~~~
