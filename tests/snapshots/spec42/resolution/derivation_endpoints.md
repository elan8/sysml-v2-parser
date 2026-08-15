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
~~~sysml
package DerivationCoverage {
    requirement def ParentRequirement;
    requirement def ChildRequirement;
    #derivation connection def {
        end #original ::> ParentRequirement;
        end #derive ::> ChildRequirement;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 161) (line 5) (column 27) (len 17)) (segments (segment 0 (token "ParentRequirement") (name "ParentRequirement") (separator none) (span (offset 161) (line 5) (column 27) (len 17)))))
    (reference r1 (scope relative) (span (offset 204) (line 6) (column 25) (len 16)) (segments (segment 0 (token "ChildRequirement") (name "ChildRequirement") (separator none) (span (offset 204) (line 6) (column 25) (len 16)))))
  )
  (root (package (name "DerivationCoverage") (body (requirement-def (name "ParentRequirement") (body semicolon)) (requirement-def (name "ChildRequirement") (body semicolon)) (connection-def (name none) (role (derivation (span (offset 110) (line 4) (column 5) (len 11)))) (specializes none) (body (end (identity (derivation-role (kind original) (span (offset 147) (line 5) (column 13) (len 9)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (redefines none) (crosses none)) (end (identity (derivation-role (kind derive) (span (offset 192) (line 6) (column 13) (len 7)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (redefines none) (crosses none)))))))
)
~~~
