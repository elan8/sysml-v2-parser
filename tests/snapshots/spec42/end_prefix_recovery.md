# META
~~~sexpr
(snapshot (type semantic) (description "The pin makes a direction or BasicFeaturePrefix restriction beside `end` unauthorable: FeaturePrefix = EndFeaturePrefix | BasicFeaturePrefix (KerML BNF 573-588; Pilot KerML.xtext 510-526) is a choice, where EndFeaturePrefix spells only `const`? `end` and the direction/derived/abstract/composite/portion/var slots live only in BasicFeaturePrefix. Both orders reach recovery with exact malformed spans; `const end`, later feature siblings, and an `abstract struct` classifier remain typed (spec42 Gaps 59/67)."))
~~~
# SOURCE
~~~sysml
package EndPrefixRecovery {
    connection def DirectionBeforeEnd {
        in end port rejected : Port;
        end accepted : Port;
    }
    connection def DirectionAfterEnd {
        end in port rejected : Port;
        end accepted : Port;
    }
    type KermlDirectionBeforeEnd {
        in end feature rejected : T;
        end feature accepted : T;
    }
    type KermlLegalPrefixes {
        const end feature constEnd;
        in feature directed : T;
    }
    type KermlRestrictionBeforeEnd {
        derived end feature derivedRejected : T;
        end derived feature derivedRejectedAfterEnd : T;
        abstract end feature abstractRejected : T;
        end abstract feature abstractRejectedAfterEnd : T;
        composite end feature compositeRejected : T;
        end composite feature compositeRejectedAfterEnd : T;
        portion end feature portionRejected : T;
        end portion feature portionRejectedAfterEnd : T;
        var end feature varRejected : T;
        end var feature varRejectedAfterEnd : T;
        const end feature constEnd;
        feature later : T;
        abstract struct NestedClassifier {
            feature nested;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 76) (line 3) (column 9) (len 37)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 76) (line 3) (column 9) (len 37)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 513) (line 19) (column 9) (len 49)) (message "unexpected keyword `derived` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 562) (line 20) (column 9) (len 57)) (message "unexpected keyword `end` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 619) (line 21) (column 9) (len 51)) (message "unexpected keyword `abstract` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 670) (line 22) (column 9) (len 59)) (message "unexpected keyword `end` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 729) (line 23) (column 9) (len 53)) (message "unrecognized declaration `composite` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 782) (line 24) (column 9) (len 61)) (message "unexpected keyword `end` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 843) (line 25) (column 9) (len 49)) (message "unrecognized declaration `portion` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 892) (line 26) (column 9) (len 57)) (message "unexpected keyword `end` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 949) (line 27) (column 9) (len 41)) (message "unrecognized declaration `var` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 990) (line 28) (column 9) (len 49)) (message "unexpected keyword `end` in calc body"))
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
    (reference r0 (scope relative) (span (offset 128) (line 4) (column 24) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 128) (line 4) (column 24) (len 4)))))
    (reference r1 (scope relative) (span (offset 239) (line 8) (column 24) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 239) (line 8) (column 24) (len 4)))))
  )
  (root (package (name "EndPrefixRecovery") (body brace (connection-def (name "DirectionBeforeEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "in end port rejected : Port;") (span (offset 76) (line 3) (column 9) (len 37))) (end (introducer bare) (short-name none) (identity (declaration (name "accepted") (span (offset 117) (line 4) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "DirectionAfterEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end in port rejected : Port;") (span (offset 187) (line 7) (column 9) (len 37))) (end (introducer bare) (short-name none) (identity (declaration (name "accepted") (span (offset 228) (line 8) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (kerml-classifier (keyword type) (abstract false) (name "KermlDirectionBeforeEnd") (specializes none) (body brace (malformed (code "recovered_calc_body_element") (found "in end feature rejected : T;") (span (offset 294) (line 11) (column 9) (len 37))) (kerml-feature (name "accepted") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlLegalPrefixes") (specializes none) (body brace (kerml-feature (name "constEnd") (relationships) (value none) (body semicolon)) (kerml-feature (name "directed") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlRestrictionBeforeEnd") (specializes none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "derived end feature derivedRejected : T;") (span (offset 513) (line 19) (column 9) (len 49))) (malformed (code "unexpected_keyword_in_scope") (found "end derived feature derivedRejectedAfterEnd : T;") (span (offset 562) (line 20) (column 9) (len 57))) (malformed (code "unexpected_keyword_in_scope") (found "abstract end feature abstractRejected : T;") (span (offset 619) (line 21) (column 9) (len 51))) (malformed (code "unexpected_keyword_in_scope") (found "end abstract feature abstractRejectedAfterEnd : T;") (span (offset 670) (line 22) (column 9) (len 59))) (malformed (code "unrecognized_declaration_in_scope") (found "composite end feature compositeRejected : T;") (span (offset 729) (line 23) (column 9) (len 53))) (malformed (code "unexpected_keyword_in_scope") (found "end composite feature compositeRejectedAfterEnd : T;") (span (offset 782) (line 24) (column 9) (len 61))) (malformed (code "unrecognized_declaration_in_scope") (found "portion end feature portionRejected : T;") (span (offset 843) (line 25) (column 9) (len 49))) (malformed (code "unexpected_keyword_in_scope") (found "end portion feature portionRejectedAfterEnd : T;") (span (offset 892) (line 26) (column 9) (len 57))) (malformed (code "unrecognized_declaration_in_scope") (found "var end feature varRejected : T;") (span (offset 949) (line 27) (column 9) (len 41))) (malformed (code "unexpected_keyword_in_scope") (found "end var feature varRejectedAfterEnd : T;") (span (offset 990) (line 28) (column 9) (len 49))) (kerml-feature (name "constEnd") (relationships) (value none) (body semicolon)) (kerml-feature (name "later") (relationships) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract true) (name "NestedClassifier") (specializes none) (body brace (kerml-feature (name "nested") (relationships) (value none) (body semicolon)))))))))
)
~~~
