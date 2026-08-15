# META
~~~sexpr
(snapshot (type semantic) (description "Every definition body reaches AnnotatingElement through DefinitionBodyItem -> DefinitionMember -> DefinitionElement, so all four alternatives -- doc, comment, rep and the @ metadata spelling -- are legal in each of them, interleaved with ordinary members in authored order. This fixture holds the definition-shaped scopes: part, attribute, port, connection, interface and occurrence definitions."))
~~~
# SOURCE
~~~sysml
package AnnotatingDefinitionScopes {
    part def P {
        doc /* part definition */
        comment /* part aside */
        rep partRep language "text" /* part rendering */
        @Approved;
        attribute mass;
    }
    attribute def A {
        doc /* attribute definition */
        comment /* attribute aside */
        rep attributeRep language "text" /* attribute rendering */
        @Approved;
    }
    port def Q {
        doc /* port definition */
        comment /* port aside */
        rep portRep language "text" /* port rendering */
        @Approved;
        attribute pressure;
    }
    connection def C {
        doc /* connection definition */
        comment /* connection aside */
        rep connectionRep language "text" /* connection rendering */
        @Approved;
    }
    interface def I {
        doc /* interface definition */
        comment /* interface aside */
        rep interfaceRep language "text" /* interface rendering */
        @Approved;
    }
    occurrence def O {
        doc /* occurrence definition */
        comment /* occurrence aside */
        rep occurrenceRep language "text" /* occurrence rendering */
        @Approved;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_definition_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingDefinitionScopes {
    part def P {
        doc
        /* part definition */
        comment
        /* part aside */
        rep partRep language "text"
        /* part rendering */
        @Approved;
        attribute mass;
    }
    attribute def A {
        doc
        /* attribute definition */
        comment
        /* attribute aside */
        rep attributeRep language "text"
        /* attribute rendering */
        @Approved;
    }
    port def Q {
        doc
        /* port definition */
        comment
        /* port aside */
        rep portRep language "text"
        /* port rendering */
        @Approved;
        attribute pressure;
    }
    connection def C {
        doc
        /* connection definition */
        comment
        /* connection aside */
        rep connectionRep language "text"
        /* connection rendering */
        @Approved;
    }
    interface def I {
        doc
        /* interface definition */
        comment
        /* interface aside */
        rep interfaceRep language "text"
        /* interface rendering */
        @Approved;
    }
    occurrence def O {
        doc
        /* occurrence definition */
        comment
        /* occurrence aside */
        rep occurrenceRep language "text"
        /* occurrence rendering */
        @Approved;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingDefinitionScopes") (body brace (part-def (name "P") (body brace (doc) (comment (keyword (span (offset 96) (line 4) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def) (port-def (name "Q") (specializes none) (body brace (doc) (comment (keyword (span (offset 477) (line 17) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation) (attribute-usage (declaration-name "pressure") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "C") (role ordinary) (specializes none) (body brace (doc) (comment (keyword (span (offset 683) (line 24) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation))) (interface-def (name "I") (specializes none) (body brace (doc) (comment (keyword (span (offset 877) (line 30) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation))) (occurrence-def))))
)
~~~
