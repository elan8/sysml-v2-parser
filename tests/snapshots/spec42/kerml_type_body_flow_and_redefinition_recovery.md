# META
~~~sexpr
(snapshot (type recovery) (description "Recovery in a calc-shaped body resynchronizes on all three member starters the scope owns: malformed members before `alias`, `flow`, and `redefines` each retain an exact recovered span without swallowing the valid sibling. Alias is a direct KerML TypeBodyElement member and reaches SysML calculation bodies through ActionBodyItem -> NonBehaviorBodyItem; before its starter was listed, recovery could consume the alias following malformed content (spec42 Gap 61 / RC7)."))
~~~
# SOURCE
~~~sysml
package Gap61KermlTypeBodyRecovery {
    classifier Recovering {
        feature f : ;
        alias recoveredClassifier for Target::feature;
        flow a.y to b.x1;
        feature g : ;
        redefines predecessors [0];
        feature h : Anything unions k;
    }
    calc def RecoveringCalculation {
        feature f : ;
        alias recoveredCalculation for Target::feature;
        message m of T;
        feature g : ;
        redefines predecessors [0];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_type_body_flow_and_redefinition_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 73) (line 3) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 176) (line 6) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 316) (line 11) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 418) (line 14) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Gap61KermlTypeBodyRecovery {
    classifier Recovering {
        feature f : ;
        alias recoveredClassifier for Target::feature;
        flow from a.y to b.x1;
        feature g : ;
        attribute :>> predecessors[0];
        feature h : Anything unions k;
    }
    calc def RecoveringCalculation {
        feature f : ;
        alias recoveredCalculation for Target::feature;
        message m of T;
        feature g : ;
        attribute :>> predecessors[0];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 125) (line 4) (column 39) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 125) (line 4) (column 39) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 133) (line 4) (column 47) (len 7)))))
    (reference r1 (scope relative) (span (offset 155) (line 5) (column 14) (len 3)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 155) (line 5) (column 14) (len 1))) (segment 1 (token "y") (name "y") (separator dot) (span (offset 157) (line 5) (column 16) (len 1)))))
    (reference r2 (scope relative) (span (offset 162) (line 5) (column 21) (len 4)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 162) (line 5) (column 21) (len 1))) (segment 1 (token "x1") (name "x1") (separator dot) (span (offset 164) (line 5) (column 23) (len 2)))))
    (reference r3 (scope relative) (span (offset 208) (line 7) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 208) (line 7) (column 19) (len 12)))))
    (reference r4 (scope relative) (span (offset 246) (line 8) (column 21) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 246) (line 8) (column 21) (len 8)))))
    (reference r5 (scope relative) (span (offset 262) (line 8) (column 37) (len 1)) (segments (segment 0 (token "k") (name "k") (separator none) (span (offset 262) (line 8) (column 37) (len 1)))))
    (reference r6 (scope relative) (span (offset 369) (line 12) (column 40) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 369) (line 12) (column 40) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 377) (line 12) (column 48) (len 7)))))
    (reference r7 (scope relative) (span (offset 407) (line 13) (column 22) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 407) (line 13) (column 22) (len 1)))))
    (reference r8 (scope relative) (span (offset 450) (line 15) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 450) (line 15) (column 19) (len 12)))))
  )
  (root (package (name "Gap61KermlTypeBodyRecovery") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Recovering") (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 73) (line 3) (column 9) (len 22))) (alias (name "recoveredClassifier") (target (ref r0)) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r1)) (references none))) (to (connector-end (multiplicity none) (target (ref r2)) (references none))))) (body (body semicolon))) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 176) (line 6) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "h") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (type-relationship (keyword unions) (targets (ref r5)))) (value none) (body semicolon)))) (calc-def (name "RecoveringCalculation") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 316) (line 11) (column 9) (len 22))) (alias (name "recoveredCalculation") (target (ref r6)) (body semicolon)) (flow-usage (kind message) (visibility none) (declaration (declared (name "m") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r7)) (conjugated false) (multiplicity none)) (endpoints none))) (body (body semicolon))) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 418) (line 14) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
