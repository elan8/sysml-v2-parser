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
  )
  (root (package (name "CalculationBodyActionMembers") (body brace (calc-def (name "Pipeline") (modifiers) (body brace (in-out-declaration) (first (source (expression (span (offset 97) (line 4) (column 15) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 102) (line 4) (column 20) (len 1))))) (merge) (decision) (join) (fork) (action-usage (declaration "step") (type none)) (if) (assign) (return-declaration (name "result") (short-name none)))))))
)
~~~
