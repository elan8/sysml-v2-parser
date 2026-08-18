# META
~~~sexpr
(snapshot (type semantic) (description "MetadataBody is a DefinitionMember body, so the bodies of `@`, `#` and `metadata` usages own the whole AnnotatingElement production too. The `#Name` forms are here to keep them distinct from that production: PrefixMetadataMember is a prefix on the declaration that follows it, and a standalone `#Name;` member is an ExtendedUsage -- neither is an AnnotatingElement."))
~~~
# SOURCE
~~~sysml
package AnnotatingMetadataScopes {
    metadata def Approved;
    #Approved part def Prefixed {
        #Approved;
        #Approved {
            doc /* keyword metadata body */
            comment /* keyword metadata aside */
            rep keywordRep language "text" /* keyword metadata rendering */
        }
        @Approved {
            doc /* annotation metadata body */
            comment /* annotation metadata aside */
            rep annotationRep language "text" /* annotation metadata rendering */
        }
        metadata named : Approved {
            doc /* metadata usage body */
            comment /* metadata usage aside */
            rep metadataRep language "text" /* metadata usage rendering */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_metadata_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingMetadataScopes {
    metadata def Approved;
    #Approved
    part def Prefixed {
        #Approved;
        #Approved {
            doc
            /* keyword metadata body */
            comment
            /* keyword metadata aside */
            rep keywordRep language "text"
            /* keyword metadata rendering */
        }
        @Approved {
            doc
            /* annotation metadata body */
            comment
            /* annotation metadata aside */
            rep annotationRep language "text"
            /* annotation metadata rendering */
        }
        metadata named : Approved {
            doc
            /* metadata usage body */
            comment
            /* metadata usage aside */
            rep metadataRep language "text"
            /* metadata usage rendering */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 3) (column 6) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 67) (line 3) (column 6) (len 8)))))
    (reference r1 (scope relative) (span (offset 105) (line 4) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 105) (line 4) (column 10) (len 8)))))
    (reference r2 (scope relative) (span (offset 124) (line 5) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 124) (line 5) (column 10) (len 8)))))
    (reference r3 (scope relative) (span (offset 323) (line 10) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 323) (line 10) (column 10) (len 8)))))
    (reference r4 (scope relative) (span (offset 550) (line 15) (column 26) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 550) (line 15) (column 26) (len 8)))))
  )
  (root (package (name "AnnotatingMetadataScopes") (body brace (metadata-def (name "Approved") (abstract false) (specializes none) (body semicolon)) (metadata-keyword-usage (type (ref r0)) (body none)) (part-def (name "Prefixed") (body brace (metadata-keyword-usage (type (ref r1)) (body semicolon)) (metadata-keyword-usage (type (ref r2)) (body brace (doc) (comment (keyword (span (offset 191) (line 7) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body brace (doc) (comment (keyword (span (offset 393) (line 12) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep))) (metadata-usage (declaration-name "named") (type (ref r4)) (about) (body brace (doc) (comment (keyword (span (offset 615) (line 17) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep))))))))
)
~~~
