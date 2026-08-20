# META
~~~sexpr
(snapshot (type semantic) (description "Recovery in a calc-shaped body resynchronizes on the two member starters the scope gained: a malformed member written before a `flow` member, and another written between a `flow` and a `redefines` member, each recover as their own node with an exact span and neither swallows the valid sibling that follows. Before `flow` and `redefines` were member starters, the recovered slice ran past them, because the keywords were only ever readable as bare feature references (spec42 Gap 61)."))
~~~
# SOURCE
~~~sysml
package Gap61KermlTypeBodyRecovery {
    classifier Recovering {
        feature f : ;
        flow a.y to b.x1;
        feature g : ;
        redefines predecessors [0];
        feature h : Anything unions k;
    }
    calc def RecoveringCalculation {
        feature f : ;
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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 121) (line 5) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 261) (line 10) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 307) (line 12) (column 9) (len 22)) (message "unrecognized declaration `feature` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Gap61KermlTypeBodyRecovery {
    classifier Recovering {
        feature f : ;
        flow from a.y to b.x1;
        feature g : ;
        attribute :>> predecessors[0];
        feature h : Anything unions k;
    }
    calc def RecoveringCalculation {
        feature f : ;
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
    (reference r0 (scope relative) (span (offset 153) (line 6) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 153) (line 6) (column 19) (len 12)))))
    (reference r1 (scope relative) (span (offset 339) (line 13) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 339) (line 13) (column 19) (len 12)))))
  )
  (root (package (name "Gap61KermlTypeBodyRecovery") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Recovering") (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 73) (line 3) (column 9) (len 22))) (flow-usage) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 121) (line 5) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (kerml-feature))) (calc-def (name "RecoveringCalculation") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature f : ;") (span (offset 261) (line 10) (column 9) (len 22))) (flow-usage) (malformed (code "unrecognized_declaration_in_scope") (found "feature g : ;") (span (offset 307) (line 12) (column 9) (len 22))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
