# META
~~~sexpr
(snapshot (type recovery) (description "An incomplete pinned `end ref` ReferenceUsage becomes an explicit connection-body recovery node, while the later complete `end ref` sibling remains typed. The recovery is not widened to Pilot-only occurrence-end or bare DefaultReferenceUsage forms."))
~~~
# SOURCE
~~~sysml
package EndDeclIntroducerRecovery {
    connection def C {
        end ref ;
        end ref later : Thing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_decl_introducer_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 67) (line 3) (column 9) (len 18)) (message "unexpected token in connection definition body"))
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
    (reference r0 (scope relative) (span (offset 101) (line 4) (column 25) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 101) (line 4) (column 25) (len 5)))))
  )
  (root (package (name "EndDeclIntroducerRecovery") (body brace (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end ref ;") (span (offset 67) (line 3) (column 9) (len 18))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 89) (line 4) (column 13) (len 3)))) (short-name none) (identity (declaration (name "later") (span (offset 93) (line 4) (column 17) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))))))
)
~~~
