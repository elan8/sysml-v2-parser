# META
~~~sexpr
(snapshot (type semantic) (description "The pinned grammar makes a direction beside `end` unauthorable -- UnextendedUsagePrefix = EndUsagePrefix | BasicUsagePrefix (SysML BNF 298) and FeaturePrefix = EndFeaturePrefix | BasicFeaturePrefix (KerML BNF 585) are alternatives, and direction lives only in the second. Both orders reach recovery with a stable diagnostic and an exact malformed span, and the valid sibling after each survives (spec42 Gap 59)."))
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
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 76) (line 3) (column 9) (len 37)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 76) (line 3) (column 9) (len 37)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
  (root (package (name "EndPrefixRecovery") (body brace (connection-def (name "DirectionBeforeEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "in end port rejected : Port;") (span (offset 76) (line 3) (column 9) (len 37))) (end (short-name none) (identity (declaration (name "accepted") (span (offset 117) (line 4) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "DirectionAfterEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end in port rejected : Port;") (span (offset 187) (line 7) (column 9) (len 37))) (end (short-name none) (identity (declaration (name "accepted") (span (offset 228) (line 8) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (kerml-classifier (keyword type) (abstract false) (name "KermlDirectionBeforeEnd") (specializes none) (body brace (malformed (code "recovered_calc_body_element") (found "in end feature rejected : T;") (span (offset 294) (line 11) (column 9) (len 37))) (kerml-feature))) (kerml-classifier (keyword type) (abstract false) (name "KermlLegalPrefixes") (specializes none) (body brace (kerml-feature) (kerml-feature))))))
)
~~~
