# META
~~~sexpr
(snapshot (type semantic) (description "Typed import conformance outcomes"))
~~~
# SOURCE
~~~sysml
package Source {
    package Inner;
    part def Item;
}
package Client {
    private import Source::*;
    private import Source::Item::*;
    private import Missing::*;
    private import Source [ 1 ];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_conformance_outcomes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Source {
    package Inner;
    part def Item;
}

package Client {
    private import Source::*;
    private import Source::Item::*;
    private import Missing::*;
    private import Source [1];
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 93) (line 6) (column 20) (len 6)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 93) (line 6) (column 20) (len 6)))))
    (reference r1 (scope relative) (span (offset 123) (line 7) (column 20) (len 12)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 123) (line 7) (column 20) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 131) (line 7) (column 28) (len 4)))))
    (reference r2 (scope relative) (span (offset 159) (line 8) (column 20) (len 7)) (segments (segment 0 (token "Missing") (name "Missing") (separator none) (span (offset 159) (line 8) (column 20) (len 7)))))
    (reference r3 (scope relative) (span (offset 190) (line 9) (column 20) (len 6)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 190) (line 9) (column 20) (len 6)))))
  )
  (root (package (name "Source") (body brace (package (name "Inner") (body semicolon)) (part-def (name "Item") (modifiers) (body semicolon)))) (package (name "Client") (body brace (import (target (span (span (offset 93) (line 6) (column 20) (len 9))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 99) (line 6) (column 26) (len 3))) (separator (span (offset 99) (line 6) (column 26) (len 2))) (marker (span (offset 101) (line 6) (column 28) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 123) (line 7) (column 20) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 135) (line 7) (column 32) (len 3))) (separator (span (offset 135) (line 7) (column 32) (len 2))) (marker (span (offset 137) (line 7) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 159) (line 8) (column 20) (len 10))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 166) (line 8) (column 27) (len 3))) (separator (span (offset 166) (line 8) (column 27) (len 2))) (marker (span (offset 168) (line 8) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 190) (line 9) (column 20) (len 12))) (all none) (ref r3) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 197) (line 9) (column 27) (len 5))) (open (span (offset 197) (line 9) (column 27) (len 1))) (expression (expression (span (offset 199) (line 9) (column 29) (len 1)) (integer 1))) (close (span (offset 201) (line 9) (column 31) (len 1))))))))))))
)
~~~
