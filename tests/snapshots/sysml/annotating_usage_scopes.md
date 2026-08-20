# META
~~~sexpr
(snapshot (type semantic) (description "UsageBody = DefinitionBody, so a usage body admits the whole AnnotatingElement production exactly as a definition body does. This fixture holds the usage-shaped scopes whose member sets are their own: port, interface, perform, rendering and view usages."))
~~~
# SOURCE
~~~sysml
package AnnotatingUsageScopes {
    part p {
        port q {
            doc /* port usage */
            comment /* port usage aside */
            rep portUsageRep language "text" /* port usage rendering */
            @Approved;
        }
        interface i : I {
            doc /* interface usage */
            comment /* interface usage aside */
            rep interfaceUsageRep language "text" /* interface usage rendering */
            @Approved;
        }
        perform action a {
            doc /* perform */
            comment /* perform aside */
            rep performRep language "text" /* perform rendering */
            @Approved;
        }
    }
    view v {
        doc /* view usage */
        comment /* view usage aside */
        rep viewRep language "text" /* view usage rendering */
        @Approved;
        render r {
            doc /* rendering usage */
            comment /* rendering usage aside */
            rep renderingRep language "text" /* rendering usage rendering */
            @Approved;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_usage_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingUsageScopes {
    part p {
        port q {
            doc
            /* port usage */
            comment
            /* port usage aside */
            rep portUsageRep language "text"
            /* port usage rendering */
            @Approved;
        }
        interface i : I {
            doc
            /* interface usage */
            comment
            /* interface usage aside */
            rep interfaceUsageRep language "text"
            /* interface usage rendering */
            @Approved;
        }
        perform action a {
            doc
            /* perform */
            comment
            /* perform aside */
            rep performRep language "text"
            /* perform rendering */
            @Approved;
        }
    }
    view v {
        doc
        /* view usage */
        comment
        /* view usage aside */
        rep viewRep language "text"
        /* view usage rendering */
        @Approved;
        render r {
            doc
            /* rendering usage */
            comment
            /* rendering usage aside */
            rep renderingRep language "text"
            /* rendering usage rendering */
            @Approved;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 223) (line 7) (column 14) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 223) (line 7) (column 14) (len 8)))))
    (reference r1 (scope relative) (span (offset 826) (line 26) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 826) (line 26) (column 10) (len 8)))))
  )
  (root (package (name "AnnotatingUsageScopes") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "q") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 80) (line 4) (column 19) (len 12)) (normalized "port usage "))) (comment (keyword (span (offset 107) (line 5) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 117) (line 5) (column 23) (len 18)) (normalized "port usage aside "))) (textual-rep (name "portUsageRep") (language "text") (body (span (offset 185) (line 6) (column 48) (len 22)) (normalized "port usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)))) (interface-usage) (perform))) (view (name "v") (short-name none) (type none) (body brace (doc (name none) (locale none) (body (span (offset 700) (line 23) (column 15) (len 12)) (normalized "view usage "))) (comment (keyword (span (offset 723) (line 24) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 733) (line 24) (column 19) (len 18)) (normalized "view usage aside "))) (textual-rep (name "viewRep") (language "text") (body (span (offset 792) (line 25) (column 39) (len 22)) (normalized "view usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)) (view-rendering))))))
)
~~~
