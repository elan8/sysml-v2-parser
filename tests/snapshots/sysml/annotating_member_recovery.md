# META
~~~sexpr
(snapshot (type recovery) (description "Malformed annotating syntax stays malformed once every alternative of the production parses. Documentation with no comment body, a `rep` with no language string, and a `comment about` with no annotated element each become a source-backed recovery node at the position they were authored -- before a valid sibling, between two valid siblings, and before a later declaration -- and every later member and declaration still parses. Editor parsing returns this whole document; strict parsing rejects it, which is what the diagnostics record."))
~~~
# SOURCE
~~~sysml
package AnnotatingMemberRecovery {
    part def Leading {
        doc locale
        doc /* the sibling after malformed leading documentation */
    }
    part def Between {
        doc /* before */
        rep language
        comment /* after */
    }
    part def Trailing {
        comment about
        attribute mass;
    }
    part def Annotated {
        comment about Leading, Between /* an annotated element list is part of the production */
    }
    part def Later {
        rep laterRep language "text" /* the declaration after every recovery */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_member_recovery.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 66) (line 3) (column 9) (len 19)) (message "missing semicolon before next declaration"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 207) (line 8) (column 9) (len 21)) (message "unexpected keyword `rep` in part definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingMemberRecovery {
    part def Leading {
        doc locale
        doc
        /* the sibling after malformed leading documentation */
    }
    part def Between {
        doc
        /* before */
        rep language
        comment
        /* after */
    }
    part def Trailing {
        comment
        /* an annotated element list is part of the production */
    }
    part def Later {
        rep laterRep language "text"
        /* the declaration after every recovery */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingMemberRecovery") (body brace (part-def (name "Leading") (body brace (malformed (code "missing_semicolon") (found "doc locale") (span (offset 66) (line 3) (column 9) (len 19))) (doc))) (part-def (name "Between") (body brace (doc) (malformed (code "unexpected_keyword_in_scope") (found "rep language") (span (offset 207) (line 8) (column 9) (len 21))) (comment (keyword (span (offset 228) (line 9) (column 9) (len 7))) (name none) (locale none)))) (part-def (name "Trailing") (body brace (comment (keyword (span (offset 286) (line 12) (column 9) (len 7))) (name none) (locale none)))) (part-def (name "Later") (body brace (textual-rep))))))
)
~~~
