# META
~~~sexpr
(snapshot (type semantic) (description "Direction-prefixed parameter declarations cover trailing and multi-target redefinitions, multiplicity before or after the typing, ordered/nonunique properties, default-expression values (including {expr} body-expression initializers), retained brace bodies, and directed occurrence usages."))
~~~
# SOURCE
~~~sysml
package DirectedParameters {
    action def TransitionAction {
        in transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource;
        inout replacementValues : Anything[0..*] nonunique;
        in occurrence terminatedOccurrence[1] {
            doc /* The occurrence to be terminated. */
        }
    }
    calc def ExcludingOnce {
        in seq[1..*] nonunique ordered;
    }
    state def StateTransition {
        in transitionLinkSource[1]: StateAction :>>
            TransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource;
    }
    action assignmentActions {
        in target : Occurrence[1] default that as Occurrence {
            doc /* The default target. */
        }
    }
    action def WhileLoopAction {
        in whileTest default {true} {
            doc /* A Boolean expression evaluated before each pass. */
        }
        in untilTest default {false};
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "directed_parameter_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DirectedParameters {
    action def TransitionAction {
        in transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource;
        inout replacementValues : Anything[0..*] nonunique;
        in occurrence terminatedOccurrence[1] {
            doc
            /* The occurrence to be terminated. */
        }
    }
    calc def ExcludingOnce {
        in seq[1..*] ordered nonunique;
    }
    state def StateTransition {
        in transitionLinkSource : StateAction[1] :>> TransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource;
    }
    action assignmentActions {
        in target : Occurrence[1] default that as Occurrence {
            doc
            /* The default target. */
        }
    }
    action def WhileLoopAction {
        in whileTest default { true } {
            doc
            /* A Boolean expression evaluated before each pass. */
        }
        in untilTest default { false };
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 97) (line 3) (column 35) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 97) (line 3) (column 35) (len 6)))))
    (reference r1 (scope relative) (span (offset 108) (line 3) (column 46) (len 43)) (segments (segment 0 (token "TransitionPerformance") (name "TransitionPerformance") (separator none) (span (offset 108) (line 3) (column 46) (len 21))) (segment 1 (token "transitionLinkSource") (name "transitionLinkSource") (separator colon-colon) (span (offset 131) (line 3) (column 69) (len 20)))))
    (reference r2 (scope relative) (span (offset 187) (line 4) (column 35) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 187) (line 4) (column 35) (len 8)))))
  )
  (root (package (name "DirectedParameters") (body (action-def (name "TransitionAction") (specializes none) (body (in-out (direction in) (reference false) (declaration "transitionLinkSource") (type (ref r0)) (multiplicity none) (ordered false) (nonunique false) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (value none) (span (offset 71) (line 3) (column 9) (len 81))) (in-out (direction inout) (reference false) (declaration "replacementValues") (type (ref r2)) (multiplicity (lower (expression (span (offset 196) (line 4) (column 44) (len 1)) (integer 0))) (upper unbounded)) (ordered false) (nonunique true) (redefines none) (value none) (span (offset 161) (line 4) (column 9) (len 51))) (occurrence-usage (direction in)))) (calc-def) (state-def (name "StateTransition") (body (inout-declaration))) (action-usage) (action-def (name "WhileLoopAction") (specializes none) (body (in-out (direction in) (reference false) (declaration "whileTest") (type none) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 813) (line 22) (column 30) (len 6)) (body-expr (body (span (offset 813) (line 22) (column 30) (len 6)) (open-brace (span (offset 813) (line 22) (column 30) (len 1))) (parameters) (result (expression (span (offset 814) (line 22) (column 31) (len 4)) (boolean true))) (close-brace (span (offset 818) (line 22) (column 35) (len 1))))))))) (body (doc)) (span (offset 792) (line 22) (column 9) (len 110))) (in-out (direction in) (reference false) (declaration "untilTest") (type none) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 932) (line 25) (column 30) (len 7)) (body-expr (body (span (offset 932) (line 25) (column 30) (len 7)) (open-brace (span (offset 932) (line 25) (column 30) (len 1))) (parameters) (result (expression (span (offset 933) (line 25) (column 31) (len 5)) (boolean false))) (close-brace (span (offset 938) (line 25) (column 36) (len 1))))))))) (span (offset 911) (line 25) (column 9) (len 29))))))))
)
~~~
