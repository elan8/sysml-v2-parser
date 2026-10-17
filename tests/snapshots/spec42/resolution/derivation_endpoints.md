# META
~~~sexpr
(snapshot (type semantic) (description "Derivation endpoint resolution coverage"))
~~~
# SOURCE
~~~sysml
package DerivationCoverage {
    requirement def ParentRequirement;
    requirement def ChildRequirement;
    #derivation connection {
        end #original ::> ParentRequirement;
        end #derive ::> ChildRequirement;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_endpoints.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 111) (line 4) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 111) (line 4) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 148) (line 5) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 148) (line 5) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 161) (line 5) (column 27) (len 17)) (segments (segment 0 (token "ParentRequirement") (name "ParentRequirement") (separator none) (span (offset 161) (line 5) (column 27) (len 17)))))
    (reference r3 (scope relative) (span (offset 193) (line 6) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 193) (line 6) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 204) (line 6) (column 25) (len 16)) (segments (segment 0 (token "ChildRequirement") (name "ChildRequirement") (separator none) (span (offset 204) (line 6) (column 25) (len 16)))))
  )
  (root (package (name "DerivationCoverage") (body brace (requirement-def (name "ParentRequirement") (modifiers) (body semicolon)) (requirement-def (name "ChildRequirement") (modifiers) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (extended-usage (visibility none) (prefix (end (cross none))) (extensions (ref r1)) (declaration (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (end (cross none))) (extensions (ref r3)) (declaration (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
