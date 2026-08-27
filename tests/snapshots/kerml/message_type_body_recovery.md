# META
~~~sexpr
(snapshot (type recovery) (description "SysML-only Message declarations in a KerML type body are recovered as whole invalid members, including the MemberPrefix spelling, without swallowing valid following Flow and Feature members."))
~~~
# SOURCE
~~~sysml
package MessageTypeBodyRecovery {
    classifier C {
        message m of T;
        private message hidden of T;
        flow a.y to b.x1;
        redefines predecessors [0];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "message_type_body_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 61) (line 3) (column 9) (len 24)) (message "unexpected keyword `message` in calc body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 85) (line 4) (column 9) (len 37)) (message "unexpected keyword `private` in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MessageTypeBodyRecovery {
    classifier C {
        message m of T;
        private message hidden of T;
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 127) (line 5) (column 14) (len 3)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 127) (line 5) (column 14) (len 1))) (segment 1 (token "y") (name "y") (separator dot) (span (offset 129) (line 5) (column 16) (len 1)))))
    (reference r1 (scope relative) (span (offset 134) (line 5) (column 21) (len 4)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 134) (line 5) (column 21) (len 1))) (segment 1 (token "x1") (name "x1") (separator dot) (span (offset 136) (line 5) (column 23) (len 2)))))
    (reference r2 (scope relative) (span (offset 158) (line 6) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 158) (line 6) (column 19) (len 12)))))
  )
  (root (package (name "MessageTypeBodyRecovery") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C") (specializes none) (conjugates none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "message m of T;") (span (offset 61) (line 3) (column 9) (len 24))) (malformed (code "unexpected_keyword_in_scope") (found "private message hidden of T;") (span (offset 85) (line 4) (column 9) (len 37))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r0)) (references none))) (to (connector-end (multiplicity none) (target (ref r1)) (references none))))) (body (body semicolon))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
