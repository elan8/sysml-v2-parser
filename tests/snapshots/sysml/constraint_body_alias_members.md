# META
~~~sexpr
(snapshot (type semantic) (description "Constraint definitions and usages share CalculationBody, whose ActionBodyItem branch retains AliasMember with visibility, a source-backed qualified target, and an annotation-only relationship body."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyAliasMembers {
    constraint def AliasDefinition {
        private alias Torque for ISQ::TorqueValue {
            doc /* A private alias in a constraint definition. */
        }
    }
    constraint AliasUsage {
        alias DriveTorque for Torque;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_alias_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConstraintBodyAliasMembers {
    constraint def AliasDefinition {
        private alias Torque for ISQ::TorqueValue {
            doc
            /* A private alias in a constraint definition. */
        }
    }
    constraint AliasUsage {
        alias DriveTorque for Torque;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 107) (line 3) (column 34) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 107) (line 3) (column 34) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 112) (line 3) (column 39) (len 11)))))
    (reference r1 (scope relative) (span (offset 266) (line 8) (column 31) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 266) (line 8) (column 31) (len 6)))))
  )
  (root (package (name "ConstraintBodyAliasMembers") (body brace (constraint-def (name "AliasDefinition") (modifiers) (specializes none) (body brace (alias (name "Torque") (target (ref r0)) (body brace (element-count 1))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "AliasUsage") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (alias (name "DriveTorque") (target (ref r1)) (body semicolon)))))))
)
~~~
