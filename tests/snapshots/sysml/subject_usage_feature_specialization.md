# META
~~~sexpr
(snapshot (type semantic) (description "SubjectUsage retains the complete FeatureSpecializationPart: typing, subsetting, reference-subsetting, crossing, and redefinition relationships, plus multiplicity on both grammar-legal sides of specialization clauses."))
~~~
# SOURCE
~~~sysml
package SubjectUsageFeatureSpecialization {
    requirement def R {
        subject declared : Type;
        subject subset :> subset;
        subject referenced ::> reference;
        subject crossed => cross;
        subject redefined :>> redefine;
        subject later :> beforeMultiplicity [2] : AfterType;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subject_usage_feature_specialization.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SubjectUsageFeatureSpecialization {
    requirement def R {
        subject declared : Type;
        subject subset :> subset;
        subject referenced ::> reference;
        subject crossed => cross;
        subject redefined :>> redefine;
        subject later : AfterType[2] :> beforeMultiplicity;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 95) (line 3) (column 28) (len 4)) (segments (segment 0 (token "Type") (name "Type") (separator none) (span (offset 95) (line 3) (column 28) (len 4)))))
    (reference r1 (scope relative) (span (offset 127) (line 4) (column 27) (len 6)) (segments (segment 0 (token "subset") (name "subset") (separator none) (span (offset 127) (line 4) (column 27) (len 6)))))
    (reference r2 (scope relative) (span (offset 166) (line 5) (column 32) (len 9)) (segments (segment 0 (token "reference") (name "reference") (separator none) (span (offset 166) (line 5) (column 32) (len 9)))))
    (reference r3 (scope relative) (span (offset 204) (line 6) (column 28) (len 5)) (segments (segment 0 (token "cross") (name "cross") (separator none) (span (offset 204) (line 6) (column 28) (len 5)))))
    (reference r4 (scope relative) (span (offset 241) (line 7) (column 31) (len 8)) (segments (segment 0 (token "redefine") (name "redefine") (separator none) (span (offset 241) (line 7) (column 31) (len 8)))))
    (reference r5 (scope relative) (span (offset 301) (line 8) (column 51) (len 9)) (segments (segment 0 (token "AfterType") (name "AfterType") (separator none) (span (offset 301) (line 8) (column 51) (len 9)))))
    (reference r6 (scope relative) (span (offset 276) (line 8) (column 26) (len 18)) (segments (segment 0 (token "beforeMultiplicity") (name "beforeMultiplicity") (separator none) (span (offset 276) (line 8) (column 26) (len 18)))))
  )
  (root (package (name "SubjectUsageFeatureSpecialization") (body brace (requirement-def (name "R") (modifiers) (body brace (subject (name "declared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (subject (name "subset") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (subject (name "referenced") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none) (value none) (body semicolon)) (subject (name "crossed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses (relationship (kind crosses) (implied false) (targets (ref r3)))) (intersects none) (value none) (body semicolon)) (subject (name "redefined") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (subject (name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 296) (line 8) (column 46) (len 1)) (integer 2))) (upper (expression (span (offset 296) (line 8) (column 46) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r6)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
