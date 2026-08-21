# META
~~~sexpr
(snapshot (type semantic) (description "DefaultReferenceUsage owns the pinned RefPrefix Usage header once in package and attribute bodies: declaration short name, prefix slots, typing, multiplicity modifiers, every FeatureSpecialization relationship, value expressions and nested attribute-shaped usage bodies remain source-backed and emit without inventing an attribute or feature keyword."))
~~~
# SOURCE
~~~sysml
package DefaultReferenceUsageScopes {
    private derived constant <measure> measurement : Units::Length[1] ordered nonunique :> base :>> parent ::> referenced => crossed intersects shared = 1 [SI::mm];
    attribute def Holder {
        derived local : Units::Length :>> measurement;
        nested : Units::Length {
            inner = 1 [SI::mm];
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DefaultReferenceUsageScopes {
    private derived constant <measure> measurement : Units::Length[1] ordered nonunique :> base :>> parent ::> referenced => crossed intersects shared = 1[SI::mm];
    attribute def Holder {
        derived local : Units::Length :>> measurement;
        nested : Units::Length {
            inner = 1[SI::mm];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 91) (line 2) (column 54) (len 13)) (segments (segment 0 (token "Units") (name "Units") (separator none) (span (offset 91) (line 2) (column 54) (len 5))) (segment 1 (token "Length") (name "Length") (separator colon-colon) (span (offset 98) (line 2) (column 61) (len 6)))))
    (reference r1 (scope relative) (span (offset 129) (line 2) (column 92) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 129) (line 2) (column 92) (len 4)))))
    (reference r2 (scope relative) (span (offset 138) (line 2) (column 101) (len 6)) (segments (segment 0 (token "parent") (name "parent") (separator none) (span (offset 138) (line 2) (column 101) (len 6)))))
    (reference r3 (scope relative) (span (offset 149) (line 2) (column 112) (len 10)) (segments (segment 0 (token "referenced") (name "referenced") (separator none) (span (offset 149) (line 2) (column 112) (len 10)))))
    (reference r4 (scope relative) (span (offset 163) (line 2) (column 126) (len 7)) (segments (segment 0 (token "crossed") (name "crossed") (separator none) (span (offset 163) (line 2) (column 126) (len 7)))))
    (reference r5 (scope relative) (span (offset 182) (line 2) (column 145) (len 6)) (segments (segment 0 (token "shared") (name "shared") (separator none) (span (offset 182) (line 2) (column 145) (len 6)))))
    (reference r6 (scope relative) (span (offset 194) (line 2) (column 157) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 194) (line 2) (column 157) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 198) (line 2) (column 161) (len 2)))))
    (reference r7 (scope relative) (span (offset 254) (line 4) (column 25) (len 13)) (segments (segment 0 (token "Units") (name "Units") (separator none) (span (offset 254) (line 4) (column 25) (len 5))) (segment 1 (token "Length") (name "Length") (separator colon-colon) (span (offset 261) (line 4) (column 32) (len 6)))))
    (reference r8 (scope relative) (span (offset 272) (line 4) (column 43) (len 11)) (segments (segment 0 (token "measurement") (name "measurement") (separator none) (span (offset 272) (line 4) (column 43) (len 11)))))
    (reference r9 (scope relative) (span (offset 302) (line 5) (column 18) (len 13)) (segments (segment 0 (token "Units") (name "Units") (separator none) (span (offset 302) (line 5) (column 18) (len 5))) (segment 1 (token "Length") (name "Length") (separator colon-colon) (span (offset 309) (line 5) (column 25) (len 6)))))
    (reference r10 (scope relative) (span (offset 341) (line 6) (column 24) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 341) (line 6) (column 24) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 345) (line 6) (column 28) (len 2)))))
  )
  (root (package (name "DefaultReferenceUsageScopes") (body brace (default-reference-usage (prefix (direction none) (derived true) (variance none) (constant true)) (declaration-name "measurement") (short-name "measure") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 105) (line 2) (column 68) (len 1)) (integer 1))) (upper (expression (span (offset 105) (line 2) (column 68) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references (relationship (kind references) (implied false) (targets (ref r3)))) (crosses (relationship (kind crosses) (implied false) (targets (ref r4)))) (intersects (relationship (kind intersects) (implied false) (targets (ref r5)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 191) (line 2) (column 154) (len 10)) (bracket (base (expression (span (offset 191) (line 2) (column 154) (len 1)) (integer 1))) (operands (sequence-list (element first (expression (span (offset 194) (line 2) (column 157) (len 6)) (ref r6)))))))))) (body semicolon)) (attribute-def (declaration-name "Holder") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived true) (variance none) (constant false)) (declaration-name "local") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "nested") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "inner") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 338) (line 6) (column 21) (len 10)) (bracket (base (expression (span (offset 338) (line 6) (column 21) (len 1)) (integer 1))) (operands (sequence-list (element first (expression (span (offset 341) (line 6) (column 24) (len 6)) (ref r10)))))))))) (body semicolon)))))))))
)
~~~
