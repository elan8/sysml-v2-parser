# META
~~~sexpr
(snapshot (type semantic) (description "Verifies fixed derivation connection and end roles retain marker spans while their targets remain source-backed references."))
~~~
# SOURCE
~~~sysml
package DerivationExample {
    requirement def OriginalReq;
    requirement def DerivedReq;

    #derivation connection {
        end #original ::> Requirements::OriginalReq;
        end #derive ::> Requirements::DerivedReq;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_roles.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DerivationExample {
    requirement def OriginalReq;
    requirement def DerivedReq;
    #derivation connection def {
        end #original ::> Requirements::OriginalReq;
        end #derive ::> Requirements::DerivedReq;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 149) (line 6) (column 27) (len 25)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 149) (line 6) (column 27) (len 12))) (segment 1 (token "OriginalReq") (name "OriginalReq") (separator colon-colon) (span (offset 163) (line 6) (column 41) (len 11)))))
    (reference r1 (scope relative) (span (offset 200) (line 7) (column 25) (len 24)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 200) (line 7) (column 25) (len 12))) (segment 1 (token "DerivedReq") (name "DerivedReq") (separator colon-colon) (span (offset 214) (line 7) (column 39) (len 10)))))
  )
  (root (package (name "DerivationExample") (body brace (requirement-def (name "OriginalReq") (modifiers) (body semicolon)) (requirement-def (name "DerivedReq") (modifiers) (body semicolon)) (connection-def (name none) (modifiers) (role (derivation (span (offset 98) (line 5) (column 5) (len 11)))) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (derivation-role (kind original) (span (offset 135) (line 6) (column 13) (len 9)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (derivation-role (kind derive) (span (offset 188) (line 7) (column 13) (len 7)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))))))
)
~~~
