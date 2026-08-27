# META
~~~sexpr
(snapshot (type semantic) (description "A then succession retains the complete pinned IfNode alternative, including its condition plus braced and shorthand branches. SysML textual BNF 954-965 and 1123-1141; Pilot SysML.xtext 1438-1439 and 1596-1612."))
~~~
# SOURCE
~~~sysml
package ThenIf {
    action def DefinitionOwner {
        then if ready {
            action yes;
        }
    }

    action UsageOwner {
        then if active then join;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "then_if.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ThenIf {
    action def DefinitionOwner {
        then if ready {
            action yes;
        }
    }
    action UsageOwner {
        then if active then join;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 17) (len 5)) (segments (segment 0 (token "ready") (name "ready") (separator none) (span (offset 66) (line 3) (column 17) (len 5)))))
    (reference r1 (scope relative) (span (offset 155) (line 9) (column 17) (len 6)) (segments (segment 0 (token "active") (name "active") (separator none) (span (offset 155) (line 9) (column 17) (len 6)))))
  )
  (root (package (name "ThenIf") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (then-if (if (condition (expression (span (offset 66) (line 3) (column 17) (len 5)) (ref r0))) (then (body brace (action-usage (keyword action) (name "yes") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (else none))))) (action-usage (keyword action) (name "UsageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (then-if (if (condition (expression (span (offset 155) (line 9) (column 17) (len 6)) (ref r1))) (then (body shorthand (then-control (join (declaration anonymous) (body semicolon (span (span (offset 171) (line 9) (column 33) (len 1)))))))) (else none))))))))
)
~~~
