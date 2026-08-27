# META
~~~sexpr
(snapshot (type recovery) (description "Occurrence-body recovery synchronizes at the BindingConnectorAsUsage starter after malformed content, retains the typed bind, and continues to a later valid sibling. SysML textual BNF 237-247, 349-353, and 702-707; pinned Pilot SysML agrees."))
~~~
# SOURCE
~~~sysml
package OccurrenceBodyBindRecovery {
    occurrence Transfer {
        nonsense ???;
        bind source = target;
        part later : Later;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_body_bind_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 71) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in occurrence body"))
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
    (reference r0 (scope relative) (span (offset 136) (line 5) (column 22) (len 5)) (segments (segment 0 (token "Later") (name "Later") (separator none) (span (offset 136) (line 5) (column 22) (len 5)))))
  )
  (root (package (name "OccurrenceBodyBindRecovery") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "Transfer") (short-name none) (target none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 71) (line 3) (column 9) (len 22))) (bind) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
