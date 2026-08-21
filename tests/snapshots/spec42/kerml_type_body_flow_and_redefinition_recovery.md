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
    (reference r1 (scope relative) (span (offset 208) (line 7) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 208) (line 7) (column 19) (len 12)))))
    (reference r2 (scope relative) (span (offset 369) (line 12) (column 40) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 369) (line 12) (column 40) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 377) (line 12) (column 48) (len 7)))))
    (reference r3 (scope relative) (span (offset 450) (line 15) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 450) (line 15) (column 19) (len 12)))))
  )
  (root (package (name "Gap61KermlTypeBodyRecovery") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Recovering") (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 73) (line 3) (column 9) (len 22))) (alias (name "recoveredClassifier") (target (ref r0)) (body semicolon)) (flow-usage) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 176) (line 6) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (kerml-feature))) (calc-def (name "RecoveringCalculation") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 316) (line 11) (column 9) (len 22))) (alias (name "recoveredCalculation") (target (ref r2)) (body semicolon)) (flow-usage) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 418) (line 14) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
