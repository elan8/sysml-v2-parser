# META
~~~sexpr
(snapshot (type semantic) (description "EnumerationBody is the one production that names AnnotatingMember directly: `';' | '{' ( ownedRelationship += AnnotatingMember | ownedRelationship += EnumerationUsageMember )* '}'`. The body parser recognised `doc` and `comment` only to discard them -- no node, no span, no diagnostic -- and dropped everything after an unparseable member. Annotating members are now retained in authored order beside the enumerated values."))
~~~
# SOURCE
~~~sysml
package AnnotatingEnumerationBody {
    enum def Level {
        doc /* enumeration definition */
        low;
        comment /* between two values */
        medium;
        rep levelRep language "text" /* enumeration rendering */
        @Approved;
        high;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_enumeration_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingEnumerationBody {
    enum def Level {
        doc
        /* enumeration definition */
        low;
        comment
        /* between two values */
        medium;
        rep levelRep language "text"
        /* enumeration rendering */
        @Approved;
        high;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 242) (line 8) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 242) (line 8) (column 10) (len 8)))))
  )
  (root (package (name "AnnotatingEnumerationBody") (body brace (enum-def (name "Level") (body brace (doc) (enum-value (name "low") (span (offset 106) (line 4) (column 9) (len 3))) (comment (keyword (span (offset 119) (line 5) (column 9) (len 7))) (name none) (about) (locale none)) (enum-value (name "medium") (span (offset 160) (line 6) (column 9) (len 6))) (textual-rep) (metadata-annotation (declared-name none) (type (ref r0)) (about) (body semicolon)) (enum-value (name "high") (span (offset 260) (line 9) (column 9) (len 4))))))))
)
~~~
