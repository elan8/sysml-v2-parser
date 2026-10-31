# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 2: CalculationBodyItem admits case, analysis, verification and use case usages through the shared action-body grammar, preserving a final result."))
~~~
# SOURCE
~~~sysml
package CasesInCalculations {
    calc def F {
        case c;
        analysis a;
        verification v;
        use case u;
        calc nested;
        1 + 2
    }
    calc f {
        case c;
        analysis a;
        verification v;
        use case u;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_family_in_calculation_bodies.md"
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
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "CasesInCalculations") (body brace (calc-def (name "F") (modifiers) (body brace (case-usage (declaration (name "c") (short-name none) (abstract false) (type none) (subsets none)) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "a") (short-name none) (type none) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (declaration (name "v") (short-name none) (abstract false) (type none) (subsets none)) (body semicolon)) (use-case-usage (declaration (name "u") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body semicolon)) (calc-usage (name "nested") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (expression (expression (span (offset 156) (line 8) (column 9) (len 5)) (binary (operator "+") (left (expression (span (offset 156) (line 8) (column 9) (len 1)) (integer 1))) (right (expression (span (offset 160) (line 8) (column 13) (len 1)) (integer 2)))))))) (calc-usage (name "f") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (case-usage (declaration (name "c") (short-name none) (abstract false) (type none) (subsets none)) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "a") (short-name none) (type none) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (declaration (name "v") (short-name none) (abstract false) (type none) (subsets none)) (body semicolon)) (use-case-usage (declaration (name "u") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body semicolon)))))))
)
~~~
