# META
~~~sexpr
(snapshot (type semantic) (description "ConstraintDefinition and ConstraintUsage end in CalculationBody, whose CalculationBodyItem = ActionBodyItem | ReturnParameterMember (SysML BNF 1366-1368), so control nodes are grammatical members of a constraint body; validateControlNodeOwningType rejects them semantically, not the parser. A conditional result expression stays a result expression, and a `ref` member keeps its `default` feature value."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyActionMembers {
    constraint def WithNodes {
        fork f;
        join j;
        merge m;
        decide d;
    }
    constraint withNodes {
        decide d;
    }
    constraint def Conditional {
        in x : ScalarValues::Real;
        if x > 0 ? true else false
    }
    constraint def WithDefault {
        ref :>> a, b default c;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_action_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConstraintBodyActionMembers {
    constraint def WithNodes {
        fork f;
        join j;
        merge m;
        decide d;
    }
    constraint withNodes {
        decide d;
    }
    constraint def Conditional {
        in x : ScalarValues::Real;
        if x > 0 ? true else false;
    }
    constraint def WithDefault {
        ref :>> a, b default c;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 14) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 82) (line 3) (column 14) (len 1)))))
    (reference r1 (scope relative) (span (offset 98) (line 4) (column 14) (len 1)) (segments (segment 0 (token "j") (name "j") (separator none) (span (offset 98) (line 4) (column 14) (len 1)))))
    (reference r2 (scope relative) (span (offset 115) (line 5) (column 15) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 115) (line 5) (column 15) (len 1)))))
    (reference r3 (scope relative) (span (offset 133) (line 6) (column 16) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 133) (line 6) (column 16) (len 1)))))
    (reference r4 (scope relative) (span (offset 184) (line 9) (column 16) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 184) (line 9) (column 16) (len 1)))))
    (reference r5 (scope relative) (span (offset 272) (line 13) (column 12) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 272) (line 13) (column 12) (len 1)))))
  )
  (root (package (name "ConstraintBodyActionMembers") (body brace (constraint-def (name "WithNodes") (modifiers) (specializes none) (body brace (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 82) (line 3) (column 14) (len 1)) (ref r0)))) (body semicolon (span (span (offset 83) (line 3) (column 15) (len 1))))) (join (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 98) (line 4) (column 14) (len 1)) (ref r1)))) (body semicolon (span (span (offset 99) (line 4) (column 15) (len 1))))) (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 115) (line 5) (column 15) (len 1)) (ref r2)))) (body semicolon (span (span (offset 116) (line 5) (column 16) (len 1))))) (decide (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 133) (line 6) (column 16) (len 1)) (ref r3)))) (body semicolon (span (span (offset 134) (line 6) (column 17) (len 1))))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "withNodes") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (decide (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 184) (line 9) (column 16) (len 1)) (ref r4)))) (body semicolon (span (span (offset 185) (line 9) (column 17) (len 1))))))) (constraint-def (name "Conditional") (modifiers) (specializes none) (body brace (in-out-declaration) (expression (span (offset 269) (line 13) (column 9) (len 26)) (conditional (test (expression (span (offset 272) (line 13) (column 12) (len 5)) (binary (operator ">") (left (expression (span (offset 272) (line 13) (column 12) (len 1)) (ref r5))) (right (expression (span (offset 276) (line 13) (column 16) (len 1)) (integer 0)))))) (then (expression (span (offset 280) (line 13) (column 20) (len 4)) (boolean true))) (else (expression (span (offset 290) (line 13) (column 30) (len 5)) (boolean false))))))) (constraint-def (name "WithDefault") (modifiers) (specializes none) (body brace (default-reference-usage))))))
)
~~~
