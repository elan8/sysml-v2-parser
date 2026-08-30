# META
~~~sexpr
(snapshot (type semantic) (description "Bare and explicitly introduced SuccessionAsUsage members in occurrence and item definition bodies."))
~~~
# SOURCE
~~~sysml
package SuccessionDefinitionBodies {
    occurrence def OccurrenceSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
        succession namedSequence first firstEvent then secondEvent;
    }

    item def ItemSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
        succession namedSequence first firstEvent then secondEvent;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_as_usage_definition_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SuccessionDefinitionBodies {
    occurrence def OccurrenceSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
        succession namedSequence first firstEvent then secondEvent;
    }
    item def ItemSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
        succession namedSequence first firstEvent then secondEvent;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "SuccessionDefinitionBodies") (body brace (occurrence-def (modifiers)) (item-def (name "ItemSequence") (modifiers) (individual false) (specializes none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "firstEvent") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "secondEvent") (short-name none) (target none) (body semicolon)) (succession-usage (keyword false) (name none)) (succession-usage (keyword true) (name "namedSequence")))))))
)
~~~
