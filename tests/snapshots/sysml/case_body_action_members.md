# META
~~~sexpr
(snapshot (type semantic) (description "`CaseBodyItem : ActionBodyItem`, so control nodes, `item` members and structured action nodes are members of a case body. Each keyword used to reach the expression arm and split into stray feature references. The shape comes from Annex A's SimpleVehicleModel. A trailing result expression keeps its own arm."))
~~~
# SOURCE
~~~sysml
use case def Scenario {
    action a;
    action b;
    item def Signal;
    item carried : Signal;
    join join1;
    merge merge1;
    decide decide1;
    fork fork1;
    then a;
    then b;
    first a then join1;
    send carried to b;
    accept received : Signal;
    if ready {
        action nested;
    }
    while running {
        action step;
    }
    loop {
        action again;
    }
    terminate;
}
analysis def Result {
    fork split;
    then branch;
    action branch;
    value
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_body_action_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
use case def Scenario {
    action a;
    action b;
    item def Signal;
    item carried : Signal;
    join join1;
    merge merge1;
    decide decide1;
    fork fork1;
    then a;
    then b;
    first a then join1;
    send carried to b;
    accept received : Signal;
    if ready {
        action nested;
    }
    while running {
        action step;
    }
    loop {
        action again;
    }
    terminate;
}

analysis def Result {
    fork split;
    then branch;
    action branch;
    value
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 92) (line 5) (column 20) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 92) (line 5) (column 20) (len 6)))))
    (reference r1 (scope relative) (span (offset 109) (line 6) (column 10) (len 5)) (segments (segment 0 (token "join1") (name "join1") (separator none) (span (offset 109) (line 6) (column 10) (len 5)))))
    (reference r2 (scope relative) (span (offset 126) (line 7) (column 11) (len 6)) (segments (segment 0 (token "merge1") (name "merge1") (separator none) (span (offset 126) (line 7) (column 11) (len 6)))))
    (reference r3 (scope relative) (span (offset 145) (line 8) (column 12) (len 7)) (segments (segment 0 (token "decide1") (name "decide1") (separator none) (span (offset 145) (line 8) (column 12) (len 7)))))
    (reference r4 (scope relative) (span (offset 163) (line 9) (column 10) (len 5)) (segments (segment 0 (token "fork1") (name "fork1") (separator none) (span (offset 163) (line 9) (column 10) (len 5)))))
    (reference r5 (scope relative) (span (offset 204) (line 12) (column 11) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 204) (line 12) (column 11) (len 1)))))
    (reference r6 (scope relative) (span (offset 211) (line 12) (column 18) (len 5)) (segments (segment 0 (token "join1") (name "join1") (separator none) (span (offset 211) (line 12) (column 18) (len 5)))))
    (reference r7 (scope relative) (span (offset 263) (line 14) (column 23) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 263) (line 14) (column 23) (len 6)))))
    (reference r8 (scope relative) (span (offset 278) (line 15) (column 8) (len 5)) (segments (segment 0 (token "ready") (name "ready") (separator none) (span (offset 278) (line 15) (column 8) (len 5)))))
    (reference r9 (scope relative) (span (offset 325) (line 18) (column 11) (len 7)) (segments (segment 0 (token "running") (name "running") (separator none) (span (offset 325) (line 18) (column 11) (len 7)))))
    (reference r10 (scope relative) (span (offset 449) (line 27) (column 10) (len 5)) (segments (segment 0 (token "split") (name "split") (separator none) (span (offset 449) (line 27) (column 10) (len 5)))))
    (reference r11 (scope relative) (span (offset 496) (line 30) (column 5) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 496) (line 30) (column 5) (len 5)))))
  )
  (root (use-case-def (name "Scenario") (modifiers) (body brace (action-usage) (action-usage) (item-def (name "Signal") (modifiers) (individual false) (specializes none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "carried") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (join (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 109) (line 6) (column 10) (len 5)) (ref r1)))) (body semicolon (span (span (offset 114) (line 6) (column 15) (len 1))))) (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 126) (line 7) (column 11) (len 6)) (ref r2)))) (body semicolon (span (span (offset 132) (line 7) (column 17) (len 1))))) (decide (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 145) (line 8) (column 12) (len 7)) (ref r3)))) (body semicolon (span (span (offset 152) (line 8) (column 19) (len 1))))) (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 163) (line 9) (column 10) (len 5)) (ref r4)))) (body semicolon (span (span (offset 168) (line 9) (column 15) (len 1))))) (then-action) (then-action) (first (source (expression (span (offset 204) (line 12) (column 11) (len 1)) (ref r5))) (target (expression (span (offset 211) (line 12) (column 18) (len 5)) (ref r6))) (body semicolon (span (span (offset 216) (line 12) (column 23) (len 1))))) (action-usage (keyword send) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword accept) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "received") (type (ref r7)) (via none))) (body semicolon)) (if (condition (expression (span (offset 278) (line 15) (column 8) (len 5)) (ref r8))) (then (body brace (action-usage (keyword action) (name "nested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (else none)) (while-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition (expression (span (offset 325) (line 18) (column 11) (len 7)) (ref r9))) (body-parameter (action-declaration none) (body brace (action-usage (keyword action) (name "step") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until none)) (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition none) (body-parameter (action-declaration none) (body brace (action-usage (keyword action) (name "again") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until none)) (terminate (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target none)))) (analysis-case-def (name "Result") (modifiers) (body brace (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 449) (line 27) (column 10) (len 5)) (ref r10)))) (body semicolon (span (span (offset 454) (line 27) (column 15) (len 1))))) (then-action) (action-usage) (expression (span (offset 496) (line 30) (column 5) (len 5)) (ref r11)))))
)
~~~
