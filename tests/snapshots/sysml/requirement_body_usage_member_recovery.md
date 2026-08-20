# META
~~~sexpr
(snapshot (type semantic) (description "A malformed member of one of the usage families a requirement body inherits from DefinitionBodyItem stays an explicit recovery node with its exact span, and consumes neither the valid member written before it nor the one written after it (spec42 Gap 42)."))
~~~
# SOURCE
~~~sysml
package RequirementBodyUsageMemberRecovery {
    requirement def Recovering {
        part before : Vehicle;
        action weigh : ;
        connect to cargo;
        part later : Vehicle;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_body_usage_member_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_requirement_body_element") (severity error) (category parseerror) (span (offset 117) (line 4) (column 9) (len 25)) (message "unexpected token in requirement body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 117) (line 4) (column 9) (len 25)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 100) (line 3) (column 23) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 100) (line 3) (column 23) (len 7)))))
    (reference r1 (scope relative) (span (offset 181) (line 6) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 181) (line 6) (column 22) (len 7)))))
  )
  (root (package (name "RequirementBodyUsageMemberRecovery") (body brace (requirement-def (name "Recovering") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "before") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_requirement_body_element") (found "action weigh : ;") (span (offset 117) (line 4) (column 9) (len 25))) (malformed (code "recovered_requirement_body_element") (found "connect to cargo;") (span (offset 142) (line 5) (column 9) (len 26))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
