# META
~~~sexpr
(snapshot (type recovery) (description "Malformed full enum-value headers recover at their exact semicolon without consuming following pin-valid EnumerationUsageMember siblings. The later visible and optional-enum values prove the enumeration-body recovery starters include MemberPrefix while Pilot-only metadata syntax remains a separate unsupported form."))
~~~
# SOURCE
~~~sysml
package EnumeratedValueFullUsageRecovery {
    enum def Level {
        broken : = 0;
        private enum <ok> valid : Level = 1;
        following : Level = 2;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumerated_value_full_usage_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 72) (line 3) (column 9) (len 22)) (message "unrecognized declaration `broken` in enumeration body"))
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
    (reference r0 (scope relative) (span (offset 120) (line 4) (column 35) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 120) (line 4) (column 35) (len 5)))))
    (reference r1 (scope relative) (span (offset 151) (line 5) (column 21) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 151) (line 5) (column 21) (len 5)))))
  )
  (root (package (name "EnumeratedValueFullUsageRecovery") (body brace (enum-def (name "Level") (body brace (malformed (code "unrecognized_declaration_in_scope") (found "broken : = 0;") (span (offset 72) (line 3) (column 9) (len 22))) (enum-value (extensions) (enum-keyword present) (visibility private) (name "valid") (short-name "ok") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 128) (line 4) (column 43) (len 1)) (integer 1))))) (body semicolon) (span (offset 94) (line 4) (column 9) (len 36))) (enum-value (extensions) (enum-keyword none) (visibility none) (name "following") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 159) (line 5) (column 29) (len 1)) (integer 2))))) (body semicolon) (span (offset 139) (line 5) (column 9) (len 22))))))))
)
~~~
