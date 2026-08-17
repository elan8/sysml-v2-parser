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
  )
  (root (package (name "AnnotatingMetadataScopes") (body brace (metadata-def) (metadata-keyword-usage) (part-def (name "Prefixed") (body brace (metadata-keyword-usage) (metadata-keyword-usage) (metadata-annotation) (metadata-usage))))))
)
~~~
