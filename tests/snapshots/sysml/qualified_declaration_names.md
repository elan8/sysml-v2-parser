# META
~~~sexpr
(snapshot (type semantic) (description "A package, namespace or library package may be declared with a qualified name. The name is stored in the document's reference arena like any other qualified name, so the references section shows each segment with its own span and the `::` separator that joined it, and the declaration projects as (qualified-declaration (ref rN)) rather than a flattened string."))
~~~
# SOURCE
~~~sysml
package AstronomyReference::Domain;

namespace Mission::Views;

standard library package Kernel::Types;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "qualified_declaration_names.md"
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
    (reference r0 (scope relative) (span (offset 8) (line 1) (column 9) (len 26)) (segments (segment 0 (token "AstronomyReference") (name "AstronomyReference") (separator none) (span (offset 8) (line 1) (column 9) (len 18))) (segment 1 (token "Domain") (name "Domain") (separator colon-colon) (span (offset 28) (line 1) (column 29) (len 6)))))
    (reference r1 (scope relative) (span (offset 47) (line 3) (column 11) (len 14)) (segments (segment 0 (token "Mission") (name "Mission") (separator none) (span (offset 47) (line 3) (column 11) (len 7))) (segment 1 (token "Views") (name "Views") (separator colon-colon) (span (offset 56) (line 3) (column 20) (len 5)))))
    (reference r2 (scope relative) (span (offset 89) (line 5) (column 26) (len 13)) (segments (segment 0 (token "Kernel") (name "Kernel") (separator none) (span (offset 89) (line 5) (column 26) (len 6))) (segment 1 (token "Types") (name "Types") (separator colon-colon) (span (offset 97) (line 5) (column 34) (len 5)))))
  )
  (root (package (name (qualified-declaration (ref r0))) (body semicolon)) (namespace (name (qualified-declaration (ref r1))) (body semicolon)) (library-package (name (qualified-declaration (ref r2))) (standard true) (body semicolon)))
)
~~~
