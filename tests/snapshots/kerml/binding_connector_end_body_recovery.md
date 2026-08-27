# META
~~~sexpr
(snapshot (type recovery) (description "A malformed inline KerML binding end pair is recovered atomically without leaking its speculative endpoint reference or consuming a later declaration-only binding whose body-owned end feature remains typed."))
~~~
# SOURCE
~~~sysml
package BindingConnectorEndRecovery {
    classifier Holder {
        binding broken of leaked = ;
        binding later {
            end feature retained :>> target;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "binding_connector_end_body_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 70) (line 3) (column 9) (len 37)) (message "unexpected keyword `binding` in calc body"))
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
    (reference r0 (scope relative) (span (offset 160) (line 5) (column 38) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 160) (line 5) (column 38) (len 6)))))
  )
  (root (package (name "BindingConnectorEndRecovery") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Holder") (specializes none) (conjugates none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "binding broken of leaked = ;") (span (offset 70) (line 3) (column 9) (len 37))) (binding (all false) (name "later") (multiplicity none) (inline-ends none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "retained") (specializations (redefinition (relationship (kind redefines) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))))))
)
~~~
