# META
~~~sexpr
(snapshot (type semantic) (description "Collection-operator BodyExpression parameters own a complete source-backed UsageDeclaration: typed/ref and :> subsetting forms retain their headers, result expression, and canonical formatting."))
~~~
# SOURCE
~~~sysml
package CollectionOperatorParameterSpecialization {
    attribute def A {
        attribute result = values.?{in ref item : Domain::Item; in selected :> ISQ::mass; selected > minimum};
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collection_operator_parameter_specialization.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CollectionOperatorParameterSpecialization {
    attribute def A {
        attribute result = values.?{ in ref item : Domain::Item; in selected :> ISQ::mass; selected > minimum };
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 101) (line 3) (column 28) (len 6)) (segments (segment 0 (token "values") (name "values") (separator none) (span (offset 101) (line 3) (column 28) (len 6)))))
    (reference r1 (scope relative) (span (offset 124) (line 3) (column 51) (len 12)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 124) (line 3) (column 51) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 132) (line 3) (column 59) (len 4)))))
    (reference r2 (scope relative) (span (offset 153) (line 3) (column 80) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 153) (line 3) (column 80) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 158) (line 3) (column 85) (len 4)))))
    (reference r3 (scope relative) (span (offset 164) (line 3) (column 91) (len 8)) (segments (segment 0 (token "selected") (name "selected") (separator none) (span (offset 164) (line 3) (column 91) (len 8)))))
    (reference r4 (scope relative) (span (offset 175) (line 3) (column 102) (len 7)) (segments (segment 0 (token "minimum") (name "minimum") (separator none) (span (offset 175) (line 3) (column 102) (len 7)))))
  )
  (root (package (name "CollectionOperatorParameterSpecialization") (body brace (attribute-def (declaration-name "A") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "result") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 101) (line 3) (column 28) (len 82)) (collection-op (operator "select") (base (expression (span (offset 101) (line 3) (column 28) (len 6)) (ref r0))) (arguments) (brace-body (body (span (offset 109) (line 3) (column 36) (len 74)) (open-brace (span (offset 109) (line 3) (column 36) (len 1))) (parameters (parameter (span (offset 110) (line 3) (column 37) (len 27)) (direction in (span (offset 110) (line 3) (column 37) (len 2))) (reference-keyword (span (offset 113) (line 3) (column 40) (len 3))) (declaration (name "item") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 136) (line 3) (column 63) (len 1))))) (parameter (span (offset 138) (line 3) (column 65) (len 25)) (direction in (span (offset 138) (line 3) (column 65) (len 2))) (reference-keyword none) (declaration (name "selected") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r2))) (value none)) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 162) (line 3) (column 89) (len 1)))))) (result (expression (span (offset 164) (line 3) (column 91) (len 18)) (binary (operator ">") (left (expression (span (offset 164) (line 3) (column 91) (len 8)) (ref r3))) (right (expression (span (offset 175) (line 3) (column 102) (len 7)) (ref r4)))))) (close-brace (span (offset 182) (line 3) (column 109) (len 1)))))))))) (body semicolon)))))))
)
~~~
