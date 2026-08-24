# META
~~~sexpr
(snapshot (type recovery) (description "A malformed directed keyword-less KerML Feature recovers as one exact TypeBody member and does not consume the following valid directed Feature. This exercises the same FeaturePrefix-first route as named and anonymous declarations without permitting a SysML action parameter to leak into a KerML type body."))
~~~
# SOURCE
~~~sysml
package DirectedKeywordlessFeatureRecovery {
    behavior Recovering {
        in broken : ;
        out retained : Result;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "directed_keywordless_feature_recovery.md"
    (diagnostics
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 79) (line 3) (column 9) (len 22)) (message "expected input type after ':'"))
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
    (reference r0 (scope relative) (span (offset 116) (line 4) (column 24) (len 6)) (segments (segment 0 (token "Result") (name "Result") (separator none) (span (offset 116) (line 4) (column 24) (len 6)))))
  )
  (root (package (name "DirectedKeywordlessFeatureRecovery") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "Recovering") (specializes none) (conjugates none) (body brace (malformed (code "missing_type_reference") (found "in broken : ;") (span (offset 79) (line 3) (column 9) (len 22))) (kerml-feature (prefix (head basic) (direction out) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "retained") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))))))
)
~~~
