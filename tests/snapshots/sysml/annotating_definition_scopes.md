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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 129) (line 5) (column 9) (len 57)) (message "unexpected keyword `rep` in part definition body"))
      (diagnostic (code "recovered_attribute_body_element") (severity error) (category parseerror) (span (offset 296) (line 11) (column 9) (len 120)) (message "unexpected token in attribute body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 477) (line 17) (column 9) (len 109)) (message "unexpected keyword `comment` in port definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 683) (line 24) (column 9) (len 123)) (message "unexpected keyword `comment` in connection definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 877) (line 30) (column 9) (len 120)) (message "unexpected keyword `comment` in interface definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 1070) (line 36) (column 9) (len 123)) (message "unexpected keyword `comment` in occurrence definition body"))
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
        rep partRep language "text" /* part rendering */
        @Approved;
        attribute mass;
    }
    attribute def A {
        doc
        /* attribute definition */
        comment /* attribute aside */
        rep attributeRep language "text" /* attribute rendering */
        @Approved;
    }
    port def Q {
        doc
        /* port definition */
        comment /* port aside */
        rep portRep language "text" /* port rendering */
        @Approved;
        attribute pressure;
    }
    connection def C {
        doc
        /* connection definition */
        comment /* connection aside */
        rep connectionRep language "text" /* connection rendering */
        @Approved;
    }
    interface def I {
        doc
        /* interface definition */
        comment /* interface aside */
        rep interfaceRep language "text" /* interface rendering */
        @Approved;
    }
    occurrence def O {
        doc
        /* occurrence definition */
        comment /* occurrence aside */
        rep occurrenceRep language "text" /* occurrence rendering */
        @Approved;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "AnnotatingDefinitionScopes") (body brace (part-def (name "P") (body brace (doc) (comment) (malformed (code "unexpected_keyword_in_scope") (found "rep partRep language \"text\" /* part rendering */") (span (offset 129) (line 5) (column 9) (len 57))) (metadata-annotation) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def) (port-def (name "Q") (specializes none) (body brace (doc) (malformed (code "unexpected_keyword_in_scope") (found "comment /* port aside */") (span (offset 477) (line 17) (column 9) (len 109))) (attribute-usage (declaration-name "pressure") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "C") (role ordinary) (specializes none) (body brace (doc) (malformed (code "unexpected_keyword_in_scope") (found "comment /* connection aside */") (span (offset 683) (line 24) (column 9) (len 123))))) (interface-def (name "I") (specializes none) (body brace (doc) (malformed (code "unexpected_keyword_in_scope") (found "comment /* interface aside */") (span (offset 877) (line 30) (column 9) (len 120))))) (occurrence-def))))
)
~~~
