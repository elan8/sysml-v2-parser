# META
~~~sexpr
(snapshot (type semantic) (description "Any `#Name` before an end is prefix metadata (`ExtendedUsage = UnextendedUsagePrefix UsageExtensionKeyword+ Usage`), not a fixed derivation role: `end #mystery ::> Missing;` is valid syntax and keeps `mystery` as a qualified reference, exactly like `end #derive ::> Kept;`. Whether the metadata resolves is a semantic question, so the parser reports nothing."))
~~~
# SOURCE
~~~sysml
package DerivationEndRecovery {
    #derivation connection {
        end #mystery ::> Missing;
        end #derive ::> Kept;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_end_role_recovery.md"
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
    (reference r0 (scope relative) (span (offset 37) (line 2) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 37) (line 2) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 74) (line 3) (column 14) (len 7)) (segments (segment 0 (token "mystery") (name "mystery") (separator none) (span (offset 74) (line 3) (column 14) (len 7)))))
    (reference r2 (scope relative) (span (offset 86) (line 3) (column 26) (len 7)) (segments (segment 0 (token "Missing") (name "Missing") (separator none) (span (offset 86) (line 3) (column 26) (len 7)))))
    (reference r3 (scope relative) (span (offset 108) (line 4) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 108) (line 4) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 119) (line 4) (column 25) (len 4)) (segments (segment 0 (token "Kept") (name "Kept") (separator none) (span (offset 119) (line 4) (column 25) (len 4)))))
  )
  (root (package (name "DerivationEndRecovery") (body brace (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (extended-usage (visibility none) (prefix (end (cross none))) (extensions (ref r1)) (declaration (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (end (cross none))) (extensions (ref r3)) (declaration (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
