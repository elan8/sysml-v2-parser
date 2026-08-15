# META
~~~sexpr
(snapshot (type semantic) (description "UsageBody = DefinitionBody, so a usage body admits the whole AnnotatingElement production exactly as a definition body does. This fixture holds the usage-shaped scopes whose member sets are their own: port, interface, perform, rendering and view usages, plus a return ref body and a nested feature body."))
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
        interface i {
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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 107) (line 5) (column 13) (len 134)) (message "unexpected keyword `comment` in port body"))
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 251) (line 9) (column 9) (len 223)) (message "unexpected token in part usage body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 251) (line 9) (column 9) (len 223)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 719) (line 24) (column 9) (len 121)) (message "unexpected keyword `comment` in view body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 901) (line 29) (column 13) (len 144)) (message "unexpected keyword `comment` in rendering usage body"))
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
            comment /* port usage aside */
            rep portUsageRep language "text" /* port usage rendering */
            @Approved;
        }
        interface i {
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
        doc
        /* view usage */
        comment /* view usage aside */
        rep viewRep language "text" /* view usage rendering */
        @Approved;
        render r {
            doc
            /* rendering usage */
            comment /* rendering usage aside */
            rep renderingRep language "text" /* rendering usage rendering */
            @Approved;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingUsageScopes") (body brace (part-usage) (view (name "v") (type none) (body brace (doc) (malformed (code "unexpected_keyword_in_scope") (found "comment /* view usage aside */") (span (offset 719) (line 24) (column 9) (len 121))) (view-rendering))))))
)
~~~
