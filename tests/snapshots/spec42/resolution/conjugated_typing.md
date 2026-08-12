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
~~~sysml
package ConjugatedTypingCoverage {
    port def InputPort;
    port def OutputPort;
    port def source : ~InputPort;
    port def target : ~OutputPort;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 103) (line 4) (column 20) (len 9)) (segments (segment 0 (token "InputPort") (name "InputPort") (separator none) (span (offset 103) (line 4) (column 20) (len 9)))))
    (reference r1 (scope relative) (span (offset 133) (line 5) (column 20) (len 10)) (segments (segment 0 (token "OutputPort") (name "OutputPort") (separator none) (span (offset 133) (line 5) (column 20) (len 10)))))
  )
  (root (package (name "ConjugatedTypingCoverage") (body (port-def (name "InputPort") (specializes none) (body semicolon)) (port-def (name "OutputPort") (specializes none) (body semicolon)) (port-def (name "source") (specializes (typing (kind typing) (conjugated true) (implied false) (targets (ref r0)))) (body semicolon)) (port-def (name "target") (specializes (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (body semicolon)))))
)
~~~
