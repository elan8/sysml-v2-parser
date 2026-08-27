# META
~~~sexpr
(snapshot (type semantic) (description "A `return` member of a constraint body that declares neither a ReturnParameterMember nor a return expression stays an explicit recovery node with its exact span and its own scope's diagnostic code, and consumes neither the constraint written before it nor the one written after it. `return result : ;` announces a typing it never gives, so the shared named-return guard rejects it before the declaration parser can invent one; `return ;` and `return = 3;` declare nothing at all and reach the shared return recovery, which now reports them as constraint body members rather than as calc body members."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyReturnMemberRecovery {
    constraint def Recovering {
        constraint before;
        return result : ;
        return ;
        return = 3;
        constraint later;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_return_member_recovery.md"
    (diagnostics
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 112) (line 4) (column 9) (len 26)) (message "expected return type after ':'"))
      (diagnostic (code "recovered_constraint_body_element") (severity error) (category parseerror) (span (offset 138) (line 5) (column 9) (len 8)) (message "unexpected token in constraint body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 138) (line 5) (column 9) (len 8)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
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
  )
  (root (package (name "ConstraintBodyReturnMemberRecovery") (body brace (constraint-def (name "Recovering") (modifiers) (specializes none) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "before") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (malformed (code "missing_type_reference") (found "return result : ;") (span (offset 112) (line 4) (column 9) (len 26))) (malformed (code "recovered_constraint_body_element") (found "return ;") (span (offset 138) (line 5) (column 9) (len 8))) (malformed (code "recovered_constraint_body_element") (found "return = 3;") (span (offset 155) (line 6) (column 9) (len 11))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
