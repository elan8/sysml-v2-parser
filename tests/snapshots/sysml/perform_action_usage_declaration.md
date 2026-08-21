# META
~~~sexpr
(snapshot (type semantic) (description "PerformActionUsageDeclaration retains its distinct declared-action UsageDeclaration and reference-subsetting alternatives, including anonymous action declarations, multiplicity modifiers, relationship clauses, values, and the current source-backed action reference (SysML textual BNF 944-952; pinned Pilot SysML.xtext 1411-1418)."))
~~~
# SOURCE
~~~sysml
package PerformActionUsageDeclaration {
    part host {
        perform action <shot> takePhoto[*] ordered references Camera::takePhoto = selectedPhoto;
        perform action :>> inheritedAction = selectedAction;
        perform source.perform :>> replacementAction;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "perform_action_usage_declaration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package PerformActionUsageDeclaration {
    part host {
        perform action <shot> takePhoto[*] ordered ::> Camera::takePhoto = selectedPhoto;
        perform action :>> inheritedAction = selectedAction;
        perform source.perform :>> replacementAction;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 118) (line 3) (column 63) (len 17)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 118) (line 3) (column 63) (len 6))) (segment 1 (token "takePhoto") (name "takePhoto") (separator colon-colon) (span (offset 126) (line 3) (column 71) (len 9)))))
    (reference r1 (scope relative) (span (offset 138) (line 3) (column 83) (len 13)) (segments (segment 0 (token "selectedPhoto") (name "selectedPhoto") (separator none) (span (offset 138) (line 3) (column 83) (len 13)))))
    (reference r2 (scope relative) (span (offset 180) (line 4) (column 28) (len 15)) (segments (segment 0 (token "inheritedAction") (name "inheritedAction") (separator none) (span (offset 180) (line 4) (column 28) (len 15)))))
    (reference r3 (scope relative) (span (offset 198) (line 4) (column 46) (len 14)) (segments (segment 0 (token "selectedAction") (name "selectedAction") (separator none) (span (offset 198) (line 4) (column 46) (len 14)))))
    (reference r4 (scope relative) (span (offset 230) (line 5) (column 17) (len 14)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 230) (line 5) (column 17) (len 6))) (segment 1 (token "perform") (name "perform") (separator dot) (span (offset 237) (line 5) (column 24) (len 7)))))
    (reference r5 (scope relative) (span (offset 249) (line 5) (column 36) (len 17)) (segments (segment 0 (token "replacementAction") (name "replacementAction") (separator none) (span (offset 249) (line 5) (column 36) (len 17)))))
  )
  (root (package (name "PerformActionUsageDeclaration") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name "takePhoto") (short-name "shot") (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (crosses none) (intersects none))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 138) (line 3) (column 83) (len 13)) (ref r1))))) (body semicolon)) (perform (target (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 198) (line 4) (column 46) (len 14)) (ref r3))))) (body semicolon)) (perform (target (reference (action (ref r4)) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))))) (value none) (body semicolon)))))))
)
~~~
