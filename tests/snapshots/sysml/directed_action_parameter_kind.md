# META
~~~sexpr
(snapshot (type provenance) (description "Direction-prefixed parameters distinguish the pinned ActionBodyParameter action-kind keyword from the keyword-less declaration and retain its exact authored span; formatting and reparsing preserve that distinction. SysML textual BNF 1133-1140."))
~~~
# SOURCE
~~~sysml
action def DirectedKinds {
    in action body {
        action nested;
    }
    in body {
        action plainNested;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "directed_action_parameter_kind.md"
    (diagnostics
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
  (root (action-def (name "DirectedKinds") (modifiers) (specializes none) (body brace (in-out (direction in) (kind (action (span (offset 34) (line 2) (column 8) (len 6)))) (reference false) (declaration "body") (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (body brace (action-usage (keyword action) (name "nested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (span (offset 31) (line 2) (column 5) (len 45))) (in-out (direction in) (kind none) (reference false) (declaration "body") (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (body brace (action-usage (keyword action) (name "plainNested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (span (offset 81) (line 5) (column 5) (len 43))))))
)
~~~
