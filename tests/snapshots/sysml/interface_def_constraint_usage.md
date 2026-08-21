# META
~~~sexpr
(snapshot (type semantic) (description "InterfaceBodyItem admits InterfaceOccurrenceUsageMember, whose InterfaceOccurrenceUsageElement includes BehaviorUsageElement and therefore ConstraintUsage (SysML textual BNF 724-750, 374-389, and 1382-1395; pinned Pilot SysML agrees). The interface-definition body owns the existing source-backed ConstraintUsage without reinterpreting its occurrence prefix, declaration, or CalculationBody."))
~~~
# SOURCE
~~~sysml
package InterfaceDefConstraintUsage {
    constraint def Limit;
    interface def PowerInterface {
        constraint limit : Limit;
        abstract constraint bounded : Limit {
            limit == limit;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_def_constraint_usage.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 126) (line 4) (column 28) (len 5)) (segments (segment 0 (token "Limit") (name "Limit") (separator none) (span (offset 126) (line 4) (column 28) (len 5)))))
    (reference r1 (scope relative) (span (offset 171) (line 5) (column 39) (len 5)) (segments (segment 0 (token "Limit") (name "Limit") (separator none) (span (offset 171) (line 5) (column 39) (len 5)))))
    (reference r2 (scope relative) (span (offset 191) (line 6) (column 13) (len 5)) (segments (segment 0 (token "limit") (name "limit") (separator none) (span (offset 191) (line 6) (column 13) (len 5)))))
    (reference r3 (scope relative) (span (offset 200) (line 6) (column 22) (len 5)) (segments (segment 0 (token "limit") (name "limit") (separator none) (span (offset 200) (line 6) (column 22) (len 5)))))
  )
  (root (package (name "InterfaceDefConstraintUsage") (body brace (constraint-def (name "Limit") (modifiers) (specializes none) (body semicolon)) (interface-def (name "PowerInterface") (modifiers) (specializes none) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "limit") (short-name none) (type (ref r0)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bounded") (short-name none) (type (ref r1)) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 191) (line 6) (column 13) (len 14)) (binary (operator "==") (left (expression (span (offset 191) (line 6) (column 13) (len 5)) (ref r2))) (right (expression (span (offset 200) (line 6) (column 22) (len 5)) (ref r3))))))))))))
)
~~~
