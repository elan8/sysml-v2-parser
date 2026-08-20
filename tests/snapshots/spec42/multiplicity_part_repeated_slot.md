# META
~~~sexpr
(snapshot (type semantic) (description "MultiplicityPart admits at most one ordering keyword and at most one uniqueness keyword (SysML BNF 495-496, KerML BNF 639-640), so a repeat, a same-slot contradiction, or a third keyword is not this production's. The parser leaves the excess unconsumed instead of folding it into the slot that already has a span: the member reaches the enclosing scope's recovery with a stable code, an exact span covering exactly the authored declaration, and the valid sibling immediately after each one still parses."))
~~~
# SOURCE
~~~sysml
package MultiplicityPartRepeatedSlot {
    part def RepeatedOrdering {
        attribute repeated : Real[0..*] ordered ordered;
        attribute sibling : Real[0..*] ordered;
    }
    part def RepeatedUniqueness {
        attribute repeated : Real[0..*] nonunique nonunique;
        attribute sibling : Real[0..*] nonunique;
    }
    part def ContradictoryOrdering {
        attribute contradictory : Real[0..*] ordered nonordered;
        attribute sibling : Real[0..*] nonordered;
    }
    part def ContradictoryUniqueness {
        attribute contradictory : Real[0..*] unique nonunique;
        attribute sibling : Real[0..*] unique;
    }
    part def ThirdKeyword {
        attribute third : Real[0..*] ordered nonunique ordered;
        attribute sibling : Real[0..*] ordered nonunique;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_part_repeated_slot.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 79) (line 3) (column 9) (len 57)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 79) (line 3) (column 9) (len 57)) (message "suppressed 4 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 156) (line 4) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 156) (line 4) (column 29) (len 4)))))
    (reference r1 (scope relative) (span (offset 305) (line 8) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 305) (line 8) (column 29) (len 4)))))
    (reference r2 (scope relative) (span (offset 463) (line 12) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 463) (line 12) (column 29) (len 4)))))
    (reference r3 (scope relative) (span (offset 622) (line 16) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 622) (line 16) (column 29) (len 4)))))
    (reference r4 (scope relative) (span (offset 767) (line 20) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 767) (line 20) (column 29) (len 4)))))
  )
  (root (package (name "MultiplicityPartRepeatedSlot") (body brace (part-def (name "RepeatedOrdering") (body brace (malformed (code "recovered_part_def_body_element") (found "attribute repeated : Real[0..*] ordered ordered;") (span (offset 79) (line 3) (column 9) (len 57))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "RepeatedUniqueness") (body brace (malformed (code "recovered_part_def_body_element") (found "attribute repeated : Real[0..*] nonunique nonunique;") (span (offset 224) (line 7) (column 9) (len 61))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "ContradictoryOrdering") (body brace (malformed (code "recovered_part_def_body_element") (found "attribute contradictory : Real[0..*] ordered nonordered;") (span (offset 378) (line 11) (column 9) (len 65))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "ContradictoryUniqueness") (body brace (malformed (code "recovered_part_def_body_element") (found "attribute contradictory : Real[0..*] unique nonunique;") (span (offset 539) (line 15) (column 9) (len 63))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "ThirdKeyword") (body brace (malformed (code "recovered_part_def_body_element") (found "attribute third : Real[0..*] ordered nonunique ordered;") (span (offset 683) (line 19) (column 9) (len 64))) (attribute-usage (declaration-name "sibling") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
