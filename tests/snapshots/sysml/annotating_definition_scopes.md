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
  (root (package (name "AnnotatingDefinitionScopes") (body brace (part-def (name "P") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 68) (line 3) (column 15) (len 17)) (normalized "part definition "))) (comment (keyword (span (offset 96) (line 4) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 106) (line 4) (column 19) (len 12)) (normalized "part aside "))) (textual-rep (name "partRep") (language "text") (body (span (offset 159) (line 5) (column 39) (len 16)) (normalized "part rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "A") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 263) (line 10) (column 15) (len 22)) (normalized "attribute definition "))) (comment (keyword (span (offset 296) (line 11) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 306) (line 11) (column 19) (len 17)) (normalized "attribute aside "))) (textual-rep (name "attributeRep") (language "text") (body (span (offset 369) (line 12) (column 44) (len 21)) (normalized "attribute rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)))) (port-def (name "Q") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 449) (line 16) (column 15) (len 17)) (normalized "port definition "))) (comment (keyword (span (offset 477) (line 17) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 487) (line 17) (column 19) (len 12)) (normalized "port aside "))) (textual-rep (name "portRep") (language "text") (body (span (offset 540) (line 18) (column 39) (len 16)) (normalized "port rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body semicolon)) (attribute-usage (declaration-name "pressure") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 649) (line 23) (column 15) (len 23)) (normalized "connection definition "))) (comment (keyword (span (offset 683) (line 24) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 693) (line 24) (column 19) (len 18)) (normalized "connection aside "))) (textual-rep (name "connectionRep") (language "text") (body (span (offset 758) (line 25) (column 45) (len 22)) (normalized "connection rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body semicolon)))) (interface-def (name "I") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 844) (line 29) (column 15) (len 22)) (normalized "interface definition "))) (comment (keyword (span (offset 877) (line 30) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 887) (line 30) (column 19) (len 17)) (normalized "interface aside "))) (textual-rep (name "interfaceRep") (language "text") (body (span (offset 950) (line 31) (column 44) (len 21)) (normalized "interface rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body semicolon)))) (occurrence-def (modifiers)) (allocation-def (name "L") (modifiers)) (part-def (name "Nested") (modifiers) (body brace (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1453) (line 48) (column 19) (len 17)) (normalized "attribute usage "))) (comment (keyword (span (offset 1485) (line 49) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 1495) (line 49) (column 23) (len 23)) (normalized "attribute usage aside "))) (textual-rep (name "attributeUsageRep") (language "text") (body (span (offset 1573) (line 50) (column 53) (len 27)) (normalized "attribute usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1671) (line 54) (column 19) (len 12)) (normalized "item usage "))) (comment (keyword (span (offset 1698) (line 55) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 1708) (line 55) (column 23) (len 18)) (normalized "item usage aside "))) (textual-rep (name "itemUsageRep") (language "text") (body (span (offset 1776) (line 56) (column 48) (len 22)) (normalized "item usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "o") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 1875) (line 60) (column 19) (len 18)) (normalized "occurrence usage "))) (comment (keyword (span (offset 1908) (line 61) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 1918) (line 61) (column 23) (len 24)) (normalized "occurrence usage aside "))) (textual-rep (name "occurrenceUsageRep") (language "text") (body (span (offset 1998) (line 62) (column 54) (len 28)) (normalized "occurrence usage rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r7)) (about) (body semicolon)))))))))
)
~~~
