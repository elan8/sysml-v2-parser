# META
~~~sexpr
(snapshot (type provenance) (description "Verifies qualified package, namespace, and library declaration names preserve ordered source-backed segments without becoming semantic references."))
~~~
# SOURCE
~~~sysml
package AstronomyReference::Domain;
namespace Mission::Views;
standard library package Kernel::Types;

package Ghost::Broken unexpected;
package Live::Valid;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "qualified_declarations.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 103) (line 5) (column 1) (len 33)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AstronomyReference::Domain;

namespace Mission::Views;

standard library package Kernel::Types;

package Ghost::Broken unexpected;

package Live::Valid;
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 8) (line 1) (column 9) (len 26)) (segments (segment 0 (token "AstronomyReference") (name "AstronomyReference") (separator none) (span (offset 8) (line 1) (column 9) (len 18))) (segment 1 (token "Domain") (name "Domain") (separator colon-colon) (span (offset 28) (line 1) (column 29) (len 6)))))
    (reference r1 (scope relative) (span (offset 46) (line 2) (column 11) (len 14)) (segments (segment 0 (token "Mission") (name "Mission") (separator none) (span (offset 46) (line 2) (column 11) (len 7))) (segment 1 (token "Views") (name "Views") (separator colon-colon) (span (offset 55) (line 2) (column 20) (len 5)))))
    (reference r2 (scope relative) (span (offset 87) (line 3) (column 26) (len 13)) (segments (segment 0 (token "Kernel") (name "Kernel") (separator none) (span (offset 87) (line 3) (column 26) (len 6))) (segment 1 (token "Types") (name "Types") (separator colon-colon) (span (offset 95) (line 3) (column 34) (len 5)))))
    (reference r3 (scope relative) (span (offset 145) (line 6) (column 9) (len 11)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 145) (line 6) (column 9) (len 4))) (segment 1 (token "Valid") (name "Valid") (separator colon-colon) (span (offset 151) (line 6) (column 15) (len 5)))))
  )
  (root (package (name (qualified-declaration (ref r0))) (body semicolon)) (namespace (name (qualified-declaration (ref r1))) (body semicolon)) (library-package (name (qualified-declaration (ref r2))) (standard true) (body semicolon)) (malformed (code "expected_keyword") (found "package Ghost::Broken unexpected;") (span (offset 103) (line 5) (column 1) (len 33))) (package (name (qualified-declaration (ref r3))) (body semicolon)))
)
~~~
