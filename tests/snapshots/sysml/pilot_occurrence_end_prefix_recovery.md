# META
~~~sexpr
(snapshot (type recovery) (description "The 2026-04 conformance pin defines OccurrenceUsagePrefix from BasicUsagePrefix, then optional individual/portion slots and UsageExtensionKeyword* (SysML-textual-bnf.kebnf 564-570). Sibling Pilot SysML.xtext 820-826 instead admits EndUsagePrefix there. Pilot-only leading `end port`, `end part`, `end item`, and `end occurrence` forms must recover as exact members rather than pass through the legacy EndDecl parser, which cannot retain their kind. An ordinary valid end sibling remains typed after recovery."))
~~~
# SOURCE
~~~sysml
package PilotOccurrenceEndPrefixRecovery {
    connection def C {
        end port p : P;
        end [1] part q : Q;
        end item r : R;
        end occurrence s : S;
        end valid : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "pilot_occurrence_end_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 74) (line 3) (column 9) (len 24)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 74) (line 3) (column 9) (len 24)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 192) (line 7) (column 21) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 192) (line 7) (column 21) (len 1)))))
  )
  (root (package (name "PilotOccurrenceEndPrefixRecovery") (body brace (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end port p : P;") (span (offset 74) (line 3) (column 9) (len 24))) (malformed (code "recovered_connection_def_body_element") (found "end [1] part q : Q;") (span (offset 98) (line 4) (column 9) (len 28))) (malformed (code "recovered_connection_def_body_element") (found "end item r : R;") (span (offset 126) (line 5) (column 9) (len 24))) (malformed (code "recovered_connection_def_body_element") (found "end occurrence s : S;") (span (offset 150) (line 6) (column 9) (len 30))) (end (short-name none) (identity (declaration (name "valid") (span (offset 184) (line 7) (column 13) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))))))
)
~~~
