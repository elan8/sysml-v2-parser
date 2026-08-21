# META
~~~sexpr
(snapshot (type semantic) (description "`CalculationBodyItem = ActionBodyItem | ReturnParameterMember`, so a calculation body owns every action-body member as well as its own `return`. It owned none of them. Worse than a coverage gap: the body's keyword-less `DefaultReferenceUsage` fallback read each action keyword as a feature name, so `first f;` became two invented members -- `'first';` and `f;` -- with no diagnostic, and the document formatted back that way. All of these now parse as one member each and the body is byte-for-byte idempotent."))
~~~
# SOURCE
~~~sysml
package CalculationBodyActionMembers {
    calc def Pipeline {
        in operand;
        first start;
        merge collect;
        decide branch;
        join rejoin;
        fork split;
        action step;
        if operand then step;
        assign operand := step;
        return result;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_body_action_members.md"
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
    (reference r0 (scope relative) (span (offset 97) (line 4) (column 15) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 97) (line 4) (column 15) (len 5)))))
    (reference r1 (scope relative) (span (offset 118) (line 5) (column 15) (len 7)) (segments (segment 0 (token "collect") (name "collect") (separator none) (span (offset 118) (line 5) (column 15) (len 7)))))
    (reference r2 (scope relative) (span (offset 142) (line 6) (column 16) (len 6)) (segments (segment 0 (token "branch") (name "branch") (separator none) (span (offset 142) (line 6) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 163) (line 7) (column 14) (len 6)) (segments (segment 0 (token "rejoin") (name "rejoin") (separator none) (span (offset 163) (line 7) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 184) (line 8) (column 14) (len 5)) (segments (segment 0 (token "split") (name "split") (separator none) (span (offset 184) (line 8) (column 14) (len 5)))))
    (reference r5 (scope relative) (span (offset 223) (line 10) (column 12) (len 7)) (segments (segment 0 (token "operand") (name "operand") (separator none) (span (offset 223) (line 10) (column 12) (len 7)))))
  )
  (root (package (name "CalculationBodyActionMembers") (body brace (calc-def (name "Pipeline") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "operand") (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 71) (line 3) (column 9) (len 11))) (first (source (expression (span (offset 97) (line 4) (column 15) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 102) (line 4) (column 20) (len 1))))) (merge (declaration (named (expression (span (offset 118) (line 5) (column 15) (len 7)) (ref r1)))) (body semicolon (span (span (offset 125) (line 5) (column 22) (len 1))))) (decide (declaration (named (expression (span (offset 142) (line 6) (column 16) (len 6)) (ref r2)))) (body semicolon (span (span (offset 148) (line 6) (column 22) (len 1))))) (join (declaration (named (expression (span (offset 163) (line 7) (column 14) (len 6)) (ref r3)))) (body semicolon (span (span (offset 169) (line 7) (column 20) (len 1))))) (fork (declaration (named (expression (span (offset 184) (line 8) (column 14) (len 5)) (ref r4)))) (body semicolon (span (span (offset 189) (line 8) (column 19) (len 1))))) (action-usage (name "step") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (if (condition (expression (span (offset 223) (line 10) (column 12) (len 7)) (ref r5))) (then (body shorthand (then-action))) (else none)) (assign) (return-declaration (name "result") (short-name none)))))))
)
~~~
