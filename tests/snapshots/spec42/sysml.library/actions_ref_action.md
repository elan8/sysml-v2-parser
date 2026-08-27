# META
~~~sexpr
(snapshot (type semantic) (description "Direct retained context from Systems Library/Actions.sysml: the base Action definition declares typed, redefining `ref action self` and `ref action incomingTransfers` usages. They now retain ActionUsage identity rather than routing through generic RefDecl."))
~~~
# SOURCE
~~~sysml
standard library package Actions {
    abstract action def Action :> Performance {
        ref action self: Action :>> Performance::self;
        ref action incomingTransfers :>> Performance::incomingTransfers;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "actions_ref_action.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Actions {
    abstract action def Action :> Performance {
        ref action self : Action :>> Performance::self;
        ref action incomingTransfers :>> Performance::incomingTransfers;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 69) (line 2) (column 35) (len 11)) (segments (segment 0 (token "Performance") (name "Performance") (separator none) (span (offset 69) (line 2) (column 35) (len 11)))))
    (reference r1 (scope relative) (span (offset 108) (line 3) (column 26) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 108) (line 3) (column 26) (len 6)))))
    (reference r2 (scope relative) (span (offset 119) (line 3) (column 37) (len 17)) (segments (segment 0 (token "Performance") (name "Performance") (separator none) (span (offset 119) (line 3) (column 37) (len 11))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 132) (line 3) (column 50) (len 4)))))
    (reference r3 (scope relative) (span (offset 179) (line 4) (column 42) (len 30)) (segments (segment 0 (token "Performance") (name "Performance") (separator none) (span (offset 179) (line 4) (column 42) (len 11))) (segment 1 (token "incomingTransfers") (name "incomingTransfers") (separator colon-colon) (span (offset 192) (line 4) (column 55) (len 17)))))
  )
  (root (library-package (name "Actions") (standard true) (body brace (action-def (name "Action") (modifiers (abstract (span (offset 39) (line 2) (column 5) (len 8)))) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace (action-usage (keyword action) (name "self") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (body semicolon)) (action-usage (keyword action) (name "incomingTransfers") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (body semicolon)))))))
)
~~~
