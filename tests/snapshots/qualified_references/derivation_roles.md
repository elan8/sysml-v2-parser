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
    #derivation connection {
        end #original ::> Requirements::OriginalReq;
        end #derive ::> Requirements::DerivedReq;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 99) (line 5) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 99) (line 5) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 136) (line 6) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 136) (line 6) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 149) (line 6) (column 27) (len 25)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 149) (line 6) (column 27) (len 12))) (segment 1 (token "OriginalReq") (name "OriginalReq") (separator colon-colon) (span (offset 163) (line 6) (column 41) (len 11)))))
    (reference r3 (scope relative) (span (offset 189) (line 7) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 189) (line 7) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 200) (line 7) (column 25) (len 24)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 200) (line 7) (column 25) (len 12))) (segment 1 (token "DerivedReq") (name "DerivedReq") (separator colon-colon) (span (offset 214) (line 7) (column 39) (len 10)))))
  )
  (root (package (name "DerivationExample") (body brace (requirement-def (name "OriginalReq") (modifiers) (body semicolon)) (requirement-def (name "DerivedReq") (modifiers) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r1)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r3)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (multiplicity none) (redefines none) (crosses none)))))))
)
~~~
