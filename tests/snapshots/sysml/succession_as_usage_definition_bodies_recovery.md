# META
~~~sexpr
(snapshot (type semantic) (description "Malformed bare SuccessionAsUsage members recover without consuming later valid definition-body siblings."))
~~~
# SOURCE
~~~sysml
package SuccessionDefinitionBodyRecovery {
    occurrence def OccurrenceSequence {
        event occurrence firstEvent;
        first firstEvent then;
        event occurrence afterRecovery;
    }

    item def ItemSequence {
        event occurrence firstEvent;
        first firstEvent then;
        event occurrence afterRecovery;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_as_usage_definition_bodies_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_occurrence_def_body_element") (severity error) (category parseerror) (span (offset 128) (line 4) (column 9) (len 31)) (message "unexpected token in occurrence definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 271) (line 10) (column 9) (len 31)) (message "unexpected keyword `first` in attribute body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package SuccessionDefinitionBodyRecovery {
    occurrence def OccurrenceSequence {
        event occurrence firstEvent;
        first firstEvent then;
        event occurrence afterRecovery;
    }
    item def ItemSequence {
        event occurrence firstEvent;
        first firstEvent then;
        event occurrence afterRecovery;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "SuccessionDefinitionBodyRecovery") (body brace (occurrence-def (modifiers)) (item-def (name "ItemSequence") (modifiers) (individual false) (specializes none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "firstEvent") (short-name none) (target none) (body semicolon)) (malformed (code "unexpected_keyword_in_scope") (found "first firstEvent then;") (span (offset 271) (line 10) (column 9) (len 31))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "afterRecovery") (short-name none) (target none) (body semicolon)))))))
)
~~~
