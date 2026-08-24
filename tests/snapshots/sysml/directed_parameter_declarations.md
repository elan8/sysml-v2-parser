# META
~~~sexpr
(snapshot (type semantic) (description "Direction-prefixed parameter declarations cover trailing and multi-target redefinitions, multiplicity before or after the typing, ordered/nonunique properties, default-expression values (including {expr} body-expression initializers), retained brace bodies, and directed occurrence usages. A part usage body carries the keyword-less directed redefinition as an in-out-declaration member, while the kinded in item / out item forms still reach the item usage parser and keep their direction. An occurrence body carries directed occurrence usages (including the event spelling) and connection usages (the flow definition this shape comes from cannot be emitted yet, so the fixture uses an occurrence definition); a requirement body carries an abstract concern usage with its multiplicity and directed or abstract calc usages -- CalcUsage::direction was a field nothing populated, and the abstract keyword was consumed and dropped."))
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
    occurrence def Message {
        in event occurrence sourceEvent [1] default that.sourceEvent;
        connection :HappensDuring connect sourceEvent to [1] self;
    }
    requirement def RequirementCheck {
        abstract concern concerns[0..*] :> concernChecks;
        in calc eval : EvaluationFunction {
            doc /* The evaluation function for this objective. */
        }
        abstract calc subcalculations : Calculation :> calculations;
    }
    part def MessageActions {
        ref sentMessage :>> sentTransfer : MessageTransfer {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
        part pump : FuelPump {
            in item pumpIn : Fuel;
            out item pumpOut : Fuel;
        }
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
        in seq[1..*] nonunique ordered;
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
    occurrence def Message {
        in event occurrence sourceEvent[1] default that.sourceEvent;
        connection  : HappensDuring connect sourceEvent to [1] self;
    }
    requirement def RequirementCheck {
        abstract concern concerns[0..*] :> concernChecks;
        in calc eval : EvaluationFunction {
            doc
            /* The evaluation function for this objective. */
        }
        abstract calc subcalculations : Calculation :> calculations;
    }
    part def MessageActions {
        ref sentMessage : MessageTransfer :>> sentTransfer {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
        part pump : FuelPump {
            in item pumpIn : Fuel;
            out item pumpOut : Fuel;
        }
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
    (reference r3 (scope relative) (span (offset 1201) (line 32) (column 44) (len 13)) (segments (segment 0 (token "concernChecks") (name "concernChecks") (separator none) (span (offset 1201) (line 32) (column 44) (len 13)))))
    (reference r4 (scope relative) (span (offset 1484) (line 39) (column 44) (len 15)) (segments (segment 0 (token "MessageTransfer") (name "MessageTransfer") (separator none) (span (offset 1484) (line 39) (column 44) (len 15)))))
    (reference r5 (scope relative) (span (offset 1469) (line 39) (column 29) (len 12)) (segments (segment 0 (token "sentTransfer") (name "sentTransfer") (separator none) (span (offset 1469) (line 39) (column 29) (len 12)))))
    (reference r6 (scope relative) (span (offset 1601) (line 42) (column 21) (len 8)) (segments (segment 0 (token "FuelPump") (name "FuelPump") (separator none) (span (offset 1601) (line 42) (column 21) (len 8)))))
    (reference r7 (scope relative) (span (offset 1641) (line 43) (column 30) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 1641) (line 43) (column 30) (len 4)))))
    (reference r8 (scope relative) (span (offset 1678) (line 44) (column 32) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 1678) (line 44) (column 32) (len 4)))))
  )
  (root (package (name "DirectedParameters") (body brace (action-def (name "TransitionAction") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "transitionLinkSource") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (value none) (span (offset 71) (line 3) (column 9) (len 81))) (in-out (direction inout) (reference false) (declaration "replacementValues") (subsets none) (type (ref r2)) (multiplicity (lower (expression (span (offset 196) (line 4) (column 44) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (redefines none) (value none) (span (offset 161) (line 4) (column 9) (len 51))) (occurrence-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions))))) (calc-def (name "ExcludingOnce") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "seq") (subsets none) (type none) (multiplicity (lower (expression (span (offset 376) (line 10) (column 16) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (redefines none) (value none) (span (offset 369) (line 10) (column 9) (len 31))))) (state-def (name "StateTransition") (modifiers) (body brace (inout-declaration))) (action-usage (keyword action) (name "assignmentActions") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (action-def (name "WhileLoopAction") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "whileTest") (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 813) (line 22) (column 30) (len 6)) (body-expr (body (span (offset 813) (line 22) (column 30) (len 6)) (open-brace (span (offset 813) (line 22) (column 30) (len 1))) (parameters) (result (expression (span (offset 814) (line 22) (column 31) (len 4)) (boolean true))) (close-brace (span (offset 818) (line 22) (column 35) (len 1))))))))) (body brace (doc (name none) (locale none) (body (span (offset 840) (line 23) (column 19) (len 50)) (normalized "A Boolean expression evaluated before each pass. ")))) (span (offset 792) (line 22) (column 9) (len 110))) (in-out (direction in) (reference false) (declaration "untilTest") (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 932) (line 25) (column 30) (len 7)) (body-expr (body (span (offset 932) (line 25) (column 30) (len 7)) (open-brace (span (offset 932) (line 25) (column 30) (len 1))) (parameters) (result (expression (span (offset 933) (line 25) (column 31) (len 5)) (boolean false))) (close-brace (span (offset 938) (line 25) (column 36) (len 1))))))))) (span (offset 911) (line 25) (column 9) (len 29))))) (occurrence-def (modifiers)) (requirement-def (name "RequirementCheck") (modifiers) (body brace (concern-usage (name "concerns") (visibility none) (abstract true) (individual false) (definition false) (type none) (multiplicity (lower (expression (span (offset 1192) (line 32) (column 35) (len 1)) (integer 0))) (upper unbounded)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (body semicolon)) (calc-usage (name "eval") (multiplicity none)) (calc-usage (name "subcalculations") (multiplicity none)))) (part-def (name "MessageActions") (modifiers) (body brace (ref (name "sentMessage") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (subsets none) (body brace (in-out-declaration))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpIn") (short-name none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpOut") (short-name none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
