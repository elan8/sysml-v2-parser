# META
~~~sexpr
(snapshot (type semantic) (description "EndDecl's existing nested ItemUsage and OccurrenceUsage alternatives retain a complete typed inner usage after the outer end identity and multiplicity: both actual corpus forms (`Items.sysml` and `CausationConnections.sysml`) emit and reparse through their structured fields. The pinned kebnf does not admit the Pilot-only leading `end item cart : I` shape through its OccurrenceUsagePrefix (SysML 564-568); Pilot SysML 836-843 does. This fixture covers only the separately parsed post-identity nested form and deliberately does not claim or alter retention of that leading Pilot-only kind spelling."))
~~~
# SOURCE
~~~sysml
package NestedEndUsageEmit {
    connection def Link {
        end touches [0..*] item touched :> things :>> separateSpace;
        end causes [*] occurrence cause :> causes :>> source {
            attribute rationale;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "nested_end_usage_emit.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package NestedEndUsageEmit {
    connection def Link {
        end touches [0..*] item touched :>> separateSpace :> things;
        end causes [*] occurrence cause :> causes :>> source {
            attribute rationale;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 98) (line 3) (column 44) (len 6)) (segments (segment 0 (token "things") (name "things") (separator none) (span (offset 98) (line 3) (column 44) (len 6)))))
    (reference r1 (scope relative) (span (offset 109) (line 3) (column 55) (len 13)) (segments (segment 0 (token "separateSpace") (name "separateSpace") (separator none) (span (offset 109) (line 3) (column 55) (len 13)))))
  )
  (root (package (name "NestedEndUsageEmit") (body brace (connection-def (name "Link") (modifiers) (role ordinary) (specializes none) (body brace (end (short-name none) (identity (declaration (name "touches") (span (offset 67) (line 3) (column 13) (len 7)))) (typing none) (references none) (multiplicity (lower (expression (span (offset 76) (line 3) (column 22) (len 1)) (integer 0))) (upper unbounded)) (redefines none) (crosses none) (nested-usage (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "touched") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r0)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (value none) (body semicolon)))) (end (short-name none) (identity (declaration (name "causes") (span (offset 136) (line 4) (column 13) (len 6)))) (typing none) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cause") (short-name none) (target none) (body brace (attribute-usage (declaration-name "rationale") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))))
)
~~~
