# META
~~~sexpr
(snapshot (type semantic) (description "EnumerationBody is the one production that names AnnotatingMember directly: `';' | '{' ( ownedRelationship += AnnotatingMember | ownedRelationship += EnumerationUsageMember )* '}'`. The body parser recognised `doc` and `comment` only to discard them -- no node, no span, no diagnostic -- and dropped everything after an unparseable member. Annotating members are now retained in authored order beside the enumerated values."))
~~~
# SOURCE
~~~sysml
package AnnotatingEnumerationBody {
    enum def Level {
        doc /* enumeration definition */
        <lo> low = 1;
        comment /* between two values */
        medium;
        rep levelRep language "text" /* enumeration rendering */
        @Approved;
        high {
            doc /* per-value documentation */
        }
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
        <lo> low = 1;
        comment
        /* between two values */
        medium;
        rep levelRep language "text"
        /* enumeration rendering */
        @Approved;
        high {
            doc
            /* per-value documentation */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 251) (line 8) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 251) (line 8) (column 10) (len 8)))))
  )
  (root (package (name "AnnotatingEnumerationBody") (body brace (enum-def (name "Level") (body brace (doc (name none) (locale none) (body (span (offset 71) (line 3) (column 15) (len 24)) (normalized "enumeration definition "))) (enum-value (enum-keyword none) (visibility none) (name "low") (short-name "lo") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 117) (line 4) (column 20) (len 1)) (integer 1))))) (body semicolon) (span (offset 106) (line 4) (column 9) (len 13))) (comment (keyword (span (offset 128) (line 5) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 138) (line 5) (column 19) (len 20)) (normalized "between two values "))) (enum-value (enum-keyword none) (visibility none) (name "medium") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 169) (line 6) (column 9) (len 7))) (textual-rep (name "levelRep") (language "text") (body (span (offset 216) (line 7) (column 40) (len 23)) (normalized "enumeration rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (enum-value (enum-keyword none) (visibility none) (name "high") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 294) (line 10) (column 19) (len 25)) (normalized "per-value documentation ")))) (span (offset 269) (line 9) (column 9) (len 62))))))))
)
~~~
