# META
~~~sexpr
(snapshot (type semantic) (description "StatePerformActionUsage keeps a referenced action's FeatureSpecializationPart (typing, subsetting, multiplicity), and StateAccept/Send/AssignmentActionUsage keep their optional ActionNodeUsageDeclaration. A perform member shares the same PerformActionUsageDeclaration."))
~~~
# SOURCE
~~~sysml
package P {
    action def A;
    item def S;
    part def Q {
        action ops : A;
        attribute x;
        perform ops : A;
        perform ops :> ops [1];
        state def SD {
            entry ops : A;
            do ops :> ops;
            exit ops [1];
        }
        state def Declared {
            entry action accept S;
            do action sender send S() to self;
            exit action setter assign x := 1;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_declaration_forms.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    action def A;
    item def S;
    part def Q {
        action ops : A;
        attribute x;
        perform ops : A;
        perform ops[1] :> ops;
        state def SD {
            entry ops : A;
            do ops :> ops;
            exit ops[1];
        }
        state def Declared {
            entry action accept S;
            do action sender send S() to self;
            exit action setter assign x := 1;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 124) (line 7) (column 17) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 124) (line 7) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 130) (line 7) (column 23) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 130) (line 7) (column 23) (len 1)))))
    (reference r2 (scope relative) (span (offset 149) (line 8) (column 17) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 149) (line 8) (column 17) (len 3)))))
    (reference r3 (scope relative) (span (offset 156) (line 8) (column 24) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 156) (line 8) (column 24) (len 3)))))
    (reference r4 (scope relative) (span (offset 206) (line 10) (column 19) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 206) (line 10) (column 19) (len 3)))))
    (reference r5 (scope relative) (span (offset 212) (line 10) (column 25) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 212) (line 10) (column 25) (len 1)))))
    (reference r6 (scope relative) (span (offset 230) (line 11) (column 16) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 230) (line 11) (column 16) (len 3)))))
    (reference r7 (scope relative) (span (offset 237) (line 11) (column 23) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 237) (line 11) (column 23) (len 3)))))
    (reference r8 (scope relative) (span (offset 259) (line 12) (column 18) (len 3)) (segments (segment 0 (token "ops") (name "ops") (separator none) (span (offset 259) (line 12) (column 18) (len 3)))))
    (reference r9 (scope relative) (span (offset 339) (line 15) (column 33) (len 1)) (segments (segment 0 (token "S") (name "S") (separator none) (span (offset 339) (line 15) (column 33) (len 1)))))
    (reference r10 (scope relative) (span (offset 376) (line 16) (column 35) (len 1)) (segments (segment 0 (token "S") (name "S") (separator none) (span (offset 376) (line 16) (column 35) (len 1)))))
    (reference r11 (scope relative) (span (offset 383) (line 16) (column 42) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 383) (line 16) (column 42) (len 4)))))
    (reference r12 (scope relative) (span (offset 427) (line 17) (column 39) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 427) (line 17) (column 39) (len 1)))))
  )
  (root (package (name "P") (body brace (action-def (name "A") (modifiers) (specializes none) (body semicolon)) (item-def (name "S") (modifiers) (individual false) (specializes none) (body semicolon)) (part-def (name "Q") (modifiers) (body brace (action-usage) (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r0)) (specialization (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (value none) (body semicolon)) (perform (target (reference (action (ref r2)) (specialization (name none) (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 161) (line 8) (column 29) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3))) (value none)) (redefines none) (references none) (crosses none) (intersects none)))) (value none) (body semicolon)) (state-def (name "SD") (modifiers) (body brace (entry (action (reference (action (ref r4)) (specialization (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none)) (body semicolon)) (do (action (reference (action (ref r6)) (specialization (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r7))) (value none)) (redefines none) (references none) (crosses none) (intersects none))) (value none)) (body semicolon)) (exit (action (reference (action (ref r8)) (specialization (name none) (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 264) (line 12) (column 23) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none)) (body semicolon)))) (state-def (name "Declared") (modifiers) (body brace (entry (action (declared-effect (action-declaration bare) (accept (payload (expression (span (offset 339) (line 15) (column 33) (len 1)) (ref r9))) (type none) (via none) (body none)))) (body semicolon)) (do (action (declared-effect (action-declaration (name "sender") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (send (payload (expression (span (offset 376) (line 16) (column 35) (len 3)) (invocation (callee (expression (span (offset 376) (line 16) (column 35) (len 1)) (ref r10))) (arguments)))) (type none) (via none) (to (expression (span (offset 383) (line 16) (column 42) (len 4)) (ref r11))) (body none)))) (body semicolon)) (exit (action (declared-effect (action-declaration (name "setter") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (assign (lhs (expression (span (offset 427) (line 17) (column 39) (len 1)) (ref r12))) (rhs (expression (span (offset 432) (line 17) (column 44) (len 1)) (integer 1))) (body none)))) (body semicolon)))))))))
)
~~~
