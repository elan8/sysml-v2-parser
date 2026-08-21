# META
~~~sexpr
(snapshot (type recovery) (description "A malformed anonymous binding in a timeslice body recovers before the next typed anonymous binding, while the nested snapshot decimal binding remains intact (SysML textual BNF 305-315, 572-586; pinned Pilot SysML.xtext 845-858)."))
~~~
# SOURCE
~~~sysml
package TimesliceSnapshotDecimalBindingRecovery {
    timeslice transportPeriod {
        snapshot :>> done {
            :>> elapseTime = 1.5 [s];
        }
        :>> broken = ;
        :>> retained = 2 [s];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "timeslice_snapshot_decimal_binding_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_occurrence_body_element") (severity error) (category parseerror) (span (offset 166) (line 6) (column 9) (len 23)) (message "unexpected token in occurrence body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TimesliceSnapshotDecimalBindingRecovery {
    timeslice transportPeriod {
        snapshot :>> done {
            attribute :>> elapseTime = 1.5[s];
        }
        :>> broken = ;
        attribute :>> retained = 2[s];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 126) (line 4) (column 17) (len 10)) (segments (segment 0 (token "elapseTime") (name "elapseTime") (separator none) (span (offset 126) (line 4) (column 17) (len 10)))))
    (reference r1 (scope relative) (span (offset 144) (line 4) (column 35) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 144) (line 4) (column 35) (len 1)))))
    (reference r2 (scope relative) (span (offset 193) (line 7) (column 13) (len 8)) (segments (segment 0 (token "retained") (name "retained") (separator none) (span (offset 193) (line 7) (column 13) (len 8)))))
    (reference r3 (scope relative) (span (offset 207) (line 7) (column 27) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 207) (line 7) (column 27) (len 1)))))
  )
  (root (package (name "TimesliceSnapshotDecimalBindingRecovery") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "transportPeriod") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139) (line 4) (column 30) (len 7)) (bracket (base (expression (span (offset 139) (line 4) (column 30) (len 3)) (real "1.5"))) (operands (sequence-list (element first (expression (span (offset 144) (line 4) (column 35) (len 1)) (ref r1)))))))))) (body semicolon)))) (malformed (code "recovered_occurrence_body_element") (found ":>> broken = ;") (span (offset 166) (line 6) (column 9) (len 23))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 204) (line 7) (column 24) (len 5)) (bracket (base (expression (span (offset 204) (line 7) (column 24) (len 1)) (integer 2))) (operands (sequence-list (element first (expression (span (offset 207) (line 7) (column 27) (len 1)) (ref r3)))))))))) (body semicolon)))))))
)
~~~
