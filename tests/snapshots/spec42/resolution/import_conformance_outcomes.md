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
    import Source::*;
    import Source::Item::*;
    import Missing::*;
    import Source [ 1 ];
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
    import Source::*;
    import Source::Item::*;
    import Missing::*;
    import Source [1];
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 85) (line 6) (column 12) (len 6)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 85) (line 6) (column 12) (len 6)))))
    (reference r1 (scope relative) (span (offset 107) (line 7) (column 12) (len 12)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 107) (line 7) (column 12) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 115) (line 7) (column 20) (len 4)))))
    (reference r2 (scope relative) (span (offset 135) (line 8) (column 12) (len 7)) (segments (segment 0 (token "Missing") (name "Missing") (separator none) (span (offset 135) (line 8) (column 12) (len 7)))))
    (reference r3 (scope relative) (span (offset 158) (line 9) (column 12) (len 6)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 158) (line 9) (column 12) (len 6)))))
  )
  (root (package (name "Source") (body brace (package (name "Inner") (body semicolon)) (part-def (name "Item") (modifiers) (body semicolon)))) (package (name "Client") (body brace (import (target (span (span (offset 85) (line 6) (column 12) (len 9))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 91) (line 6) (column 18) (len 3))) (separator (span (offset 91) (line 6) (column 18) (len 2))) (marker (span (offset 93) (line 6) (column 20) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 107) (line 7) (column 12) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 119) (line 7) (column 24) (len 3))) (separator (span (offset 119) (line 7) (column 24) (len 2))) (marker (span (offset 121) (line 7) (column 26) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 135) (line 8) (column 12) (len 10))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 142) (line 8) (column 19) (len 3))) (separator (span (offset 142) (line 8) (column 19) (len 2))) (marker (span (offset 144) (line 8) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 158) (line 9) (column 12) (len 12))) (all none) (ref r3) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 165) (line 9) (column 19) (len 5))) (open (span (offset 165) (line 9) (column 19) (len 1))) (expression (expression (span (offset 167) (line 9) (column 21) (len 1)) (integer 1))) (close (span (offset 169) (line 9) (column 23) (len 1))))))))))))
)
~~~
