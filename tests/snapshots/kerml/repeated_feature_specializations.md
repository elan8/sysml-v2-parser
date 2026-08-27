# META
~~~sexpr
(snapshot (type provenance) (description "KerML feature specialization alternatives retain authored order and clause identity: two references/crosses clauses stay distinct from one comma-target clause, and keyword/operator spellings remain typed. KerML textual BNF FeatureSpecialization, References, and Crosses; Pilot KerML.xtext FeatureSpecialization, ReferencesKeyword, and Crossings."))
~~~
# SOURCE
~~~sysml
package FeatureSpecializationClauses {
    feature repeatedReferences references first references second;
    feature combinedReferences references first, second;
    feature repeatedCrosses crosses first crosses second;
    feature combinedCrosses crosses first, second;
    feature interleaved references r1 => c1 ::> r2 crosses c2;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "repeated_feature_specializations.md"
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
    (reference r0 (scope relative) (span (offset 81) (line 2) (column 43) (len 5)) (segments (segment 0 (token "first") (name "first") (separator none) (span (offset 81) (line 2) (column 43) (len 5)))))
    (reference r1 (scope relative) (span (offset 98) (line 2) (column 60) (len 6)) (segments (segment 0 (token "second") (name "second") (separator none) (span (offset 98) (line 2) (column 60) (len 6)))))
    (reference r2 (scope relative) (span (offset 148) (line 3) (column 43) (len 5)) (segments (segment 0 (token "first") (name "first") (separator none) (span (offset 148) (line 3) (column 43) (len 5)))))
    (reference r3 (scope relative) (span (offset 155) (line 3) (column 50) (len 6)) (segments (segment 0 (token "second") (name "second") (separator none) (span (offset 155) (line 3) (column 50) (len 6)))))
    (reference r4 (scope relative) (span (offset 199) (line 4) (column 37) (len 5)) (segments (segment 0 (token "first") (name "first") (separator none) (span (offset 199) (line 4) (column 37) (len 5)))))
    (reference r5 (scope relative) (span (offset 213) (line 4) (column 51) (len 6)) (segments (segment 0 (token "second") (name "second") (separator none) (span (offset 213) (line 4) (column 51) (len 6)))))
    (reference r6 (scope relative) (span (offset 257) (line 5) (column 37) (len 5)) (segments (segment 0 (token "first") (name "first") (separator none) (span (offset 257) (line 5) (column 37) (len 5)))))
    (reference r7 (scope relative) (span (offset 264) (line 5) (column 44) (len 6)) (segments (segment 0 (token "second") (name "second") (separator none) (span (offset 264) (line 5) (column 44) (len 6)))))
    (reference r8 (scope relative) (span (offset 307) (line 6) (column 36) (len 2)) (segments (segment 0 (token "r1") (name "r1") (separator none) (span (offset 307) (line 6) (column 36) (len 2)))))
    (reference r9 (scope relative) (span (offset 313) (line 6) (column 42) (len 2)) (segments (segment 0 (token "c1") (name "c1") (separator none) (span (offset 313) (line 6) (column 42) (len 2)))))
    (reference r10 (scope relative) (span (offset 320) (line 6) (column 49) (len 2)) (segments (segment 0 (token "r2") (name "r2") (separator none) (span (offset 320) (line 6) (column 49) (len 2)))))
    (reference r11 (scope relative) (span (offset 331) (line 6) (column 60) (len 2)) (segments (segment 0 (token "c2") (name "c2") (separator none) (span (offset 331) (line 6) (column 60) (len 2)))))
  )
  (root (package (name "FeatureSpecializationClauses") (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "repeatedReferences") (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r0)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "combinedReferences") (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r2) (ref r3))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "repeatedCrosses") (specializations (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r4)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r5))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "combinedCrosses") (specializations (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r6) (ref r7))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "interleaved") (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r8)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r9)))) (reference-subsetting (relationship (kind references) (implied false) (targets (ref r10)))) (cross-subsetting (relationship (kind crosses) (implied false) (targets (ref r11))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
