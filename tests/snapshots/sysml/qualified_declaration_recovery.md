# META
~~~sexpr
(snapshot (type recovery) (description "A qualified declaration name is published to the document's reference arena by the production that owns it, so a production that fails must roll the arena back. Here `package Ghost::Broken unexpected;` fails and the following `package Live::Valid;` parses: the references section must list exactly one reference, Live::Valid, because a Ghost::Broken entry would be a name the document claims to declare while owning no declaration for it."))
~~~
# SOURCE
~~~sysml
package Ghost::Broken unexpected;
package Live::Valid;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "qualified_declaration_recovery.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 33)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Ghost::Broken unexpected;

package Live::Valid;
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 42) (line 2) (column 9) (len 11)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 42) (line 2) (column 9) (len 4))) (segment 1 (token "Valid") (name "Valid") (separator colon-colon) (span (offset 48) (line 2) (column 15) (len 5)))))
  )
  (root (malformed (code "expected_keyword") (found "package Ghost::Broken unexpected;") (span (offset 0) (line 1) (column 1) (len 33))) (package (name (qualified-declaration (ref r0))) (body semicolon)))
)
~~~
