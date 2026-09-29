# META
~~~sexpr
(snapshot (type semantic) (description "A `#derivation connection` is a def-less connection usage whose `OccurrenceUsagePrefix` owns `#derivation` as an extension keyword, and its ends carry `#original` and `#derive` as prefix metadata (`ExtendedUsage`), not fixed roles or names. Each keyword is a qualified reference with its own span; a derivation end is anonymous and its target is a reference subsetting. A `connection def` projects its (extensions) list, empty for an ordinary definition."))
~~~
# SOURCE
~~~sysml
package DerivationConnectionRoles {
    requirement def OriginalReq;
    requirement def DerivedReq;
    #derivation connection {
        end #original ::> OriginalReq;
        end #derive ::> DerivedReq;
    }
    connection def Ordinary {
        end left ::> OriginalReq;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_connection_roles.md"
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
    (reference r0 (scope relative) (span (offset 106) (line 4) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 106) (line 4) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 143) (line 5) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 143) (line 5) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 156) (line 5) (column 27) (len 11)) (segments (segment 0 (token "OriginalReq") (name "OriginalReq") (separator none) (span (offset 156) (line 5) (column 27) (len 11)))))
    (reference r3 (scope relative) (span (offset 182) (line 6) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 182) (line 6) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 193) (line 6) (column 25) (len 10)) (segments (segment 0 (token "DerivedReq") (name "DerivedReq") (separator none) (span (offset 193) (line 6) (column 25) (len 10)))))
    (reference r5 (scope relative) (span (offset 262) (line 9) (column 22) (len 11)) (segments (segment 0 (token "OriginalReq") (name "OriginalReq") (separator none) (span (offset 262) (line 9) (column 22) (len 11)))))
  )
  (root (package (name "DerivationConnectionRoles") (body brace (requirement-def (name "OriginalReq") (modifiers) (body semicolon)) (requirement-def (name "DerivedReq") (modifiers) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r1)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r3)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "Ordinary") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "left") (span (offset 253) (line 9) (column 13) (len 4)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r5)))) (multiplicity none) (redefines none) (crosses none)))))))
)
~~~
