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
  (root (package (name "AnnotatingMetadataScopes") (body brace (metadata-def (name "Approved") (abstract false) (specializes none) (body semicolon)) (metadata-keyword-usage (type (ref r0)) (body none)) (part-def (name "Prefixed") (modifiers) (body brace (metadata-keyword-usage (type (ref r1)) (body semicolon)) (metadata-keyword-usage (type (ref r2)) (body brace (doc (name none) (locale none) (body (span (offset 153) (line 6) (column 19) (len 23)) (normalized "keyword metadata body "))) (comment (keyword (span (offset 191) (line 7) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 201) (line 7) (column 23) (len 24)) (normalized "keyword metadata aside "))) (textual-rep (name "keywordRep") (language "text") (body (span (offset 273) (line 8) (column 46) (len 28)) (normalized "keyword metadata rendering "))))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body brace (doc (name none) (locale none) (body (span (offset 352) (line 11) (column 19) (len 26)) (normalized "annotation metadata body "))) (comment (keyword (span (offset 393) (line 12) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 403) (line 12) (column 23) (len 27)) (normalized "annotation metadata aside "))) (textual-rep (name "annotationRep") (language "text") (body (span (offset 481) (line 13) (column 49) (len 31)) (normalized "annotation metadata rendering "))))) (metadata-usage (declaration-name "named") (type (ref r4)) (about) (body brace (doc (name none) (locale none) (body (span (offset 579) (line 16) (column 19) (len 21)) (normalized "metadata usage body "))) (comment (keyword (span (offset 615) (line 17) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 625) (line 17) (column 23) (len 22)) (normalized "metadata usage aside "))) (textual-rep (name "metadataRep") (language "text") (body (span (offset 696) (line 18) (column 47) (len 26)) (normalized "metadata usage rendering "))))))))))
)
~~~
