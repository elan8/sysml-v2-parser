# META
~~~sexpr
(snapshot (type recovery) (description "An unrecognised derivation end marker is retained as recovery syntax rather than guessed at, and the valid end that follows it still parses. Only `#original` and `#derive` are fixed roles; `#mystery` is neither, so the member becomes a malformed node and the next `end #derive ::> Kept;` is unaffected."))
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
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 69) (line 3) (column 9) (len 34)) (message "unexpected token in connection definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package DerivationEndRecovery {
    #derivation connection def {
        end #mystery ::> Missing;
        end #derive ::> Kept;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 119) (line 4) (column 25) (len 4)) (segments (segment 0 (token "Kept") (name "Kept") (separator none) (span (offset 119) (line 4) (column 25) (len 4)))))
  )
  (root (package (name "DerivationEndRecovery") (body brace (connection-def (name none) (role (derivation (span (offset 36) (line 2) (column 5) (len 11)))) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end #mystery ::> Missing;") (span (offset 69) (line 3) (column 9) (len 34))) (end (identity (derivation-role (kind derive) (span (offset 107) (line 4) (column 13) (len 7)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (redefines none) (crosses none)))))))
)
~~~
