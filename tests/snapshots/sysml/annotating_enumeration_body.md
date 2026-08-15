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
        low;
        medium;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingEnumerationBody") (body brace (enum-def))))
)
~~~
