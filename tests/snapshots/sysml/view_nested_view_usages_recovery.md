# META
~~~sexpr
(snapshot (type recovery) (description "A malformed member between nested view usages in view and view definition bodies becomes a recovery node that resumes at the next view keyword, preserving the later valid nested view siblings."))
~~~
# SOURCE
~~~sysml
package NestedViewRecovery {
    view def DocumentType {
        view first;
        view = ;
        view retainedInDefinition;
    }
    view document {
        view first;
        view = ;
        view retainedInUsage {
            view nested;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_nested_view_usages_recovery.md"
    (diagnostics
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 85) (line 4) (column 9) (len 17)) (message "expected ';' or '{' after view declaration header"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 85) (line 4) (column 9) (len 17)) (message "suppressed 1 cascading missing_body_or_semicolon diagnostic after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "NestedViewRecovery") (body brace (view-def (name "DocumentType") (short-name none) (modifiers) (specializes none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "first") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (malformed (code "missing_body_or_semicolon") (found "view = ;") (span (offset 85) (line 4) (column 9) (len 17))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retainedInDefinition") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "document") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "first") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (malformed (code "missing_body_or_semicolon") (found "view = ;") (span (offset 183) (line 9) (column 9) (len 17))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retainedInUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "nested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
