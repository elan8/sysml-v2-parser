# META
~~~sexpr
(snapshot (type semantic) (description "MultiplicityPart admits one OwnedMultiplicity and it precedes the keyword slots (SysML BNF 492-496, KerML BNF 636-640), so a second range, or a range written after a keyword, is not this production's. Each is left unconsumed rather than consumed and ignored, so the member reaches the enclosing scope's recovery with a stable code and an exact span, and the valid sibling immediately after each one still parses."))
~~~
# SOURCE
~~~sysml
package MultiplicityPartRangeCardinality {
    part def InterleavedRange {
        attribute interleaved [1] ordered [2] nonunique : Real;
        attribute sibling [1] ordered nonunique : Real;
    }
    part def RepeatedRange {
        attribute repeated [1] [2] : Real;
        attribute sibling [1] : Real;
    }
    part def RangeAfterKeyword {
        attribute trailing [1] ordered [2] : Real;
        attribute sibling [1] ordered : Real;
    }
    type KermlRepeatedRange {
        feature repeated [1] [2] : Real;
        feature sibling [1] : Real;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_part_range_cardinality.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 83) (line 3) (column 9) (len 64)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 83) (line 3) (column 9) (len 64)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 491) (line 15) (column 9) (len 41)) (message "unrecognized declaration `feature` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MultiplicityPartRangeCardinality {
    part def InterleavedRange {
        attribute interleaved [1] ordered [2] nonunique : Real;
        attribute sibling : Real[1] ordered nonunique;
    }
    part def RepeatedRange {
        attribute repeated [1] [2] : Real;
        attribute sibling : Real[1];
    }
    part def RangeAfterKeyword {
        attribute trailing [1] ordered [2] : Real;
        attribute sibling : Real[1] ordered;
    }
    type KermlRepeatedRange {
        feature repeated [1] [2] : Real;
        feature sibling : Real[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 189) (line 4) (column 51) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 189) (line 4) (column 51) (len 4)))))
    (reference r1 (scope relative) (span (offset 305) (line 8) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 305) (line 8) (column 33) (len 4)))))
    (reference r2 (scope relative) (span (offset 441) (line 12) (column 41) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 441) (line 12) (column 41) (len 4)))))
  )
  (root (package (name "MultiplicityPartRangeCardinality") (body brace (part-def (name "InterleavedRange") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "attribute interleaved [1] ordered [2] nonunique : Real;") (span (offset 83) (line 3) (column 9) (len 64))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "RepeatedRange") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "attribute repeated [1] [2] : Real;") (span (offset 238) (line 7) (column 9) (len 43))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "RangeAfterKeyword") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "attribute trailing [1] ordered [2] : Real;") (span (offset 358) (line 11) (column 9) (len 51))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlRepeatedRange") (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature repeated [1] [2] : Real;") (span (offset 491) (line 15) (column 9) (len 41))) (kerml-feature))))))
)
~~~
