# META
~~~sexpr
(snapshot (type semantic) (description "Spec42 285: ActionBodyItem admits ViewUsage through NonBehaviorBodyItem and StructureUsageMember in definitions, usages, and nested action bodies; typed views retain their full bodies."))
~~~
# SOURCE
~~~sysml
package V {
    action def A {
        action x;
        /* The comment remains a member before its view. */
        view v : StandardViewDefinitions::ActionFlowView;
        view plain;
        #Diagram view tagged;
        view body { expose x; }
        private ref view hidden;
        action nested { view inside; }
    }
    action a : A { view usageView; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "views_in_action_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package V {
    action def A {
        action x;
        /* The comment remains a member before its view. */
        view v : StandardViewDefinitions::ActionFlowView;
        view plain;
        #Diagram view tagged;
        view body {
            expose x;
        }
        private ref view hidden;
        action nested {
            view inside;
        }
    }
    action a : A {
        view usageView;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 126) (line 5) (column 18) (len 39)) (segments (segment 0 (token "StandardViewDefinitions") (name "StandardViewDefinitions") (separator none) (span (offset 126) (line 5) (column 18) (len 23))) (segment 1 (token "ActionFlowView") (name "ActionFlowView") (separator colon-colon) (span (offset 151) (line 5) (column 43) (len 14)))))
    (reference r1 (scope relative) (span (offset 196) (line 7) (column 10) (len 7)) (segments (segment 0 (token "Diagram") (name "Diagram") (separator none) (span (offset 196) (line 7) (column 10) (len 7)))))
    (reference r2 (scope relative) (span (offset 244) (line 8) (column 28) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 244) (line 8) (column 28) (len 1)))))
    (reference r3 (scope relative) (span (offset 342) (line 12) (column 16) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 342) (line 12) (column 16) (len 1)))))
  )
  (root (package (visibility none) (name "V") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "x") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 59) (line 4) (column 11) (len 47)) (normalized "The comment remains a member before its view. "))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "v") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "plain") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1))) (name "tagged") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (expose (target (span (span (offset 244) (line 8) (column 28) (len 1))) (all none) (ref r2) (shape (membership (recursive-suffix none)))) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "hidden") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (action-usage (keyword action) (name "nested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "inside") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))) (action-usage (keyword action) (name "a") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "usageView") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))
)
~~~
