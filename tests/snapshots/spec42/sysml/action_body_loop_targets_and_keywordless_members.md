# META
~~~sexpr
(snapshot (type semantic) (description "Action-body members next to `perform`: `while`, `loop` and `for` nodes are ActionNodes and so valid `then` succession targets; a keyword-less member led by a specialization operator needs no value; `perform x redefines y;` is the keyword spelling of `:>>`; and an occurrence usage body owns `perform` members."))
~~~
# SOURCE
~~~sysml
package Control {
    action def Run {
        attribute i : ScalarValues::Integer;
        attribute stateSpace;
        action step;
        first start;
        then while i > 0 {
            perform step;
        }
        then loop {
            perform step;
        } until i > 3;
        then for n in (1, 2, 3) {
            perform step;
        }
        then done;
    }
    action run : Run {
        :>> stateSpace : ScalarValues::Real;
        :> i;
    }
    part def Rig {
        perform action providePower;
    }
    part rig : Rig {
        perform run redefines providePower;
        timeslice warmUp {
            perform action :>> providePower;
            perform run;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_loop_targets_and_keywordless_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Control {
    action def Run {
        attribute i : ScalarValues::Integer;
        attribute stateSpace;
        action step;
        first start;
        then while i > 0 {
            perform step;
        }
        then loop {
            perform step;
        } until i > 3;
        then for n in (1, 2, 3) {
            perform step;
        }
        then done;
    }
    action run : Run {
         : ScalarValues::Real :>> stateSpace;
         :> i;
    }
    part def Rig {
        perform action providePower;
    }
    part rig : Rig {
        perform run :>> providePower;
        timeslice warmUp {
            perform action :>> providePower;
            perform run;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 149) (line 6) (column 15) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 149) (line 6) (column 15) (len 5)))))
    (reference r1 (scope relative) (span (offset 175) (line 7) (column 20) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 175) (line 7) (column 20) (len 1)))))
    (reference r2 (scope relative) (span (offset 203) (line 8) (column 21) (len 4)) (segments (segment 0 (token "step") (name "step") (separator none) (span (offset 203) (line 8) (column 21) (len 4)))))
    (reference r3 (scope relative) (span (offset 259) (line 11) (column 21) (len 4)) (segments (segment 0 (token "step") (name "step") (separator none) (span (offset 259) (line 11) (column 21) (len 4)))))
    (reference r4 (scope relative) (span (offset 281) (line 12) (column 17) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 281) (line 12) (column 17) (len 1)))))
    (reference r5 (scope relative) (span (offset 342) (line 14) (column 21) (len 4)) (segments (segment 0 (token "step") (name "step") (separator none) (span (offset 342) (line 14) (column 21) (len 4)))))
    (reference r6 (scope relative) (span (offset 400) (line 18) (column 18) (len 3)) (segments (segment 0 (token "Run") (name "Run") (separator none) (span (offset 400) (line 18) (column 18) (len 3)))))
    (reference r7 (scope relative) (span (offset 431) (line 19) (column 26) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 431) (line 19) (column 26) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 445) (line 19) (column 40) (len 4)))))
    (reference r8 (scope relative) (span (offset 418) (line 19) (column 13) (len 10)) (segments (segment 0 (token "stateSpace") (name "stateSpace") (separator none) (span (offset 418) (line 19) (column 13) (len 10)))))
    (reference r9 (scope relative) (span (offset 462) (line 20) (column 12) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 462) (line 20) (column 12) (len 1)))))
    (reference r10 (scope relative) (span (offset 548) (line 25) (column 16) (len 3)) (segments (segment 0 (token "Rig") (name "Rig") (separator none) (span (offset 548) (line 25) (column 16) (len 3)))))
    (reference r11 (scope relative) (span (offset 570) (line 26) (column 17) (len 3)) (segments (segment 0 (token "run") (name "run") (separator none) (span (offset 570) (line 26) (column 17) (len 3)))))
    (reference r12 (scope relative) (span (offset 584) (line 26) (column 31) (len 12)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 584) (line 26) (column 31) (len 12)))))
    (reference r13 (scope relative) (span (offset 656) (line 28) (column 32) (len 12)) (segments (segment 0 (token "providePower") (name "providePower") (separator none) (span (offset 656) (line 28) (column 32) (len 12)))))
    (reference r14 (scope relative) (span (offset 690) (line 29) (column 21) (len 3)) (segments (segment 0 (token "run") (name "run") (separator none) (span (offset 690) (line 29) (column 21) (len 3)))))
  )
  (root (package (name "Control") (body brace (action-def (name "Run") (modifiers) (specializes none) (body brace (attribute-usage) (attribute-usage) (action-usage (keyword action) (name "step") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (first (source (expression (span (offset 149) (line 6) (column 15) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 154) (line 6) (column 20) (len 1))))) (then-while (while-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition (expression (span (offset 175) (line 7) (column 20) (len 5)) (binary (operator ">") (left (expression (span (offset 175) (line 7) (column 20) (len 1)) (ref r1))) (right (expression (span (offset 179) (line 7) (column 24) (len 1)) (integer 0)))))) (body-parameter (action-declaration none) (body brace (perform (target (reference (action (ref r2)) (redefines none))) (value none) (body semicolon)))) (until none))) (then-loop (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition none) (body-parameter (action-declaration none) (body brace (perform (target (reference (action (ref r3)) (redefines none))) (value none) (body semicolon)))) (until (expression (span (offset 281) (line 12) (column 17) (len 5)) (binary (operator ">") (left (expression (span (offset 281) (line 12) (column 17) (len 1)) (ref r4))) (right (expression (span (offset 285) (line 12) (column 21) (len 1)) (integer 3)))))))) (then-for (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "n") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 310) (line 13) (column 23) (len 9)) (sequence (sequence-list (element first (expression (span (offset 311) (line 13) (column 24) (len 1)) (integer 1))) (element comma (expression (span (offset 314) (line 13) (column 27) (len 1)) (integer 2))) (element comma (expression (span (offset 317) (line 13) (column 30) (len 1)) (integer 3))))))) (body-parameter (action-declaration none) (body brace (perform (target (reference (action (ref r5)) (redefines none))) (value none) (body semicolon)))))) (then-action))) (action-usage (keyword action) (name "run") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r9)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Rig") (modifiers) (body brace (perform (target (action (name "providePower") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rig") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (reference (action (ref r11)) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))))) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "warmUp") (short-name none) (target none) (body brace (perform (target (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (perform (target (reference (action (ref r14)) (redefines none))) (value none) (body semicolon)))))))))
)
~~~
