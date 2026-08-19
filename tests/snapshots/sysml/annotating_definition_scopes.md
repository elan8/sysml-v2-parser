# META
~~~sexpr
(snapshot (type semantic) (description "Every definition body reaches AnnotatingElement through DefinitionBodyItem -> DefinitionMember -> DefinitionElement, so all four alternatives -- doc, comment, rep and the @ metadata spelling -- are legal in each of them, interleaved with ordinary members in authored order. This fixture holds the definition-shaped scopes: part, attribute, port, connection, interface, occurrence and allocation definitions, and the attribute, item and occurrence usage bodies that share their member sets."))
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
    allocation def L {
        doc /* allocation definition */
        comment /* allocation aside */
        rep allocationRep language "text" /* allocation rendering */
        @Approved;
    }
    part def Nested {
        attribute a {
            doc /* attribute usage */
            comment /* attribute usage aside */
            rep attributeUsageRep language "text" /* attribute usage rendering */
            @Approved;
        }
        item i {
            doc /* item usage */
            comment /* item usage aside */
            rep itemUsageRep language "text" /* item usage rendering */
            @Approved;
        }
        occurrence o {
            doc /* occurrence usage */
            comment /* occurrence usage aside */
            rep occurrenceUsageRep language "text" /* occurrence usage rendering */
            @Approved;
        }
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
    allocation def L {
        doc
        /* allocation definition */
        comment
        /* allocation aside */
        rep allocationRep language "text"
        /* allocation rendering */
        @Approved;
    }
    part def Nested {
        attribute a {
            doc
            /* attribute usage */
            comment
            /* attribute usage aside */
            rep attributeUsageRep language "text"
            /* attribute usage rendering */
            @Approved;
        }
        item i {
            doc
            /* item usage */
            comment
            /* item usage aside */
            rep itemUsageRep language "text"
            /* item usage rendering */
            @Approved;
        }
        occurrence o {
            doc
            /* occurrence usage */
            comment
            /* occurrence usage aside */
            rep occurrenceUsageRep language "text"
            /* occurrence usage rendering */
            @Approved;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 187) (line 6) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 187) (line 6) (column 10) (len 8)))))
    (reference r1 (scope relative) (span (offset 402) (line 13) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 402) (line 13) (column 10) (len 8)))))
    (reference r2 (scope relative) (span (offset 568) (line 19) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 568) (line 19) (column 10) (len 8)))))
    (reference r3 (scope relative) (span (offset 792) (line 26) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 792) (line 26) (column 10) (len 8)))))
    (reference r4 (scope relative) (span (offset 983) (line 32) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 983) (line 32) (column 10) (len 8)))))
    (reference r5 (scope relative) (span (offset 1616) (line 51) (column 14) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 1616) (line 51) (column 14) (len 8)))))
    (reference r6 (scope relative) (span (offset 1814) (line 57) (column 14) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 1814) (line 57) (column 14) (len 8)))))
    (reference r7 (scope relative) (span (offset 2042) (line 63) (column 14) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 2042) (line 63) (column 14) (len 8)))))
  )
  (root (package (name "AnnotatingDefinitionScopes") (body brace (part-def (name "P") (body brace (doc) (comment (keyword (span (offset 96) (line 4) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "A") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (comment (keyword (span (offset 296) (line 11) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)))) (port-def (name "Q") (specializes none) (body brace (doc) (comment (keyword (span (offset 477) (line 17) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body semicolon)) (attribute-usage (declaration-name "pressure") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (doc) (comment (keyword (span (offset 683) (line 24) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body semicolon)))) (interface-def (name "I") (modifiers) (specializes none) (body brace (doc) (comment (keyword (span (offset 877) (line 30) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body semicolon)))) (occurrence-def) (allocation-def (name "L") (modifiers)) (part-def (name "Nested") (body brace (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc) (comment (keyword (span (offset 1485) (line 49) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc) (comment (keyword (span (offset 1698) (line 55) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "o") (short-name none) (target none) (body brace (doc) (comment (keyword (span (offset 1908) (line 61) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r7)) (about) (body semicolon)))))))))
)
~~~
