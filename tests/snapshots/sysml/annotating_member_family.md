# META
~~~sexpr
(snapshot (type semantic) (description "AnnotatingElement = Comment | Documentation | TextualRepresentation | MetadataFeature is one production, so a scope that owns annotating members owns all of it. Every scope here parses the same family through the same parser and emits it through the same emitter: before the family, three emitter copies disagreed, and a rep member emitted from an import body while failing as unsupported from a dependency, alias or connect body -- whether a document could be formatted depended on which construct owned the body."))
~~~
# SOURCE
~~~sysml
package AnnotatingMemberFamily {
    part def A;
    dependency d from a to b {
        doc /* why */
        comment /* aside */
        rep inline language "text" /* hello */
    }
    alias B for A {
        rep inline language "text" /* hello */
    }
    import ISQ::* {
        rep inline language "text" /* hello */
    }
    connection def C {
        port a;
        port b;
        connect a to b {
            rep inline language "text" /* hello */
        }
        ref b {
            doc /* the ref body owns the same production */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_member_family.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingMemberFamily {
    part def A;
    dependency d from a to b {
        doc
        /* why */
        comment
        /* aside */
        rep inline language "text"
        /* hello */
    }
    alias B for A {
        rep inline language "text"
        /* hello */
    }
    import ISQ::* {
        rep inline language "text"
        /* hello */
    }
    connection def C {
        port a;
        port b;
        connect a to b {
            rep inline language "text"
            /* hello */
        }
        ref b {
            doc
            /* the ref body owns the same production */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 71) (line 3) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 71) (line 3) (column 23) (len 1)))))
    (reference r1 (scope relative) (span (offset 76) (line 3) (column 28) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 76) (line 3) (column 28) (len 1)))))
    (reference r2 (scope relative) (span (offset 199) (line 8) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 199) (line 8) (column 17) (len 1)))))
    (reference r3 (scope relative) (span (offset 267) (line 11) (column 12) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 267) (line 11) (column 12) (len 3)))))
  )
  (root (package (name "AnnotatingMemberFamily") (body (part-def (name "A") (body semicolon)) (dependency (clients (ref r0)) (suppliers (ref r1)) (body (doc) (comment (keyword (span (offset 110) (line 5) (column 9) (len 7))) (name none) (locale none)) (textual-rep))) (alias (name "B") (target (ref r2)) (body brace (element-count 1))) (import (target (span (span (offset 267) (line 11) (column 12) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 270) (line 11) (column 15) (len 3))) (separator (span (offset 270) (line 11) (column 15) (len 2))) (marker (span (offset 272) (line 11) (column 17) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (connection-def (name "C") (role ordinary) (specializes none) (body (port-usage) (port-usage) (connect) (ref (name "b") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body (doc))))))))
)
~~~
