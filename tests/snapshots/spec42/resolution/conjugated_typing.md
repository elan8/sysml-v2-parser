# META
~~~sexpr
(snapshot (type semantic) (description "Conjugated typing resolution coverage"))
~~~
# SOURCE
~~~sysml
package ConjugatedTypingCoverage {
    port def InputPort;
    port def OutputPort;
    port source : ~InputPort;
    port target : ~OutputPort;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugated_typing.md"
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
    (reference r0 (scope relative) (span (offset 103) (line 4) (column 20) (len 9)) (segments (segment 0 (token "InputPort") (name "InputPort") (separator none) (span (offset 103) (line 4) (column 20) (len 9)))))
    (reference r1 (scope relative) (span (offset 133) (line 5) (column 20) (len 10)) (segments (segment 0 (token "OutputPort") (name "OutputPort") (separator none) (span (offset 133) (line 5) (column 20) (len 10)))))
  )
  (root (package (name "ConjugatedTypingCoverage") (body brace (port-def (name "InputPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "OutputPort") (modifiers) (specializes none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "source") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "target") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
