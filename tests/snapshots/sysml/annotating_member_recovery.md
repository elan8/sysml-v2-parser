# META
~~~sexpr
(snapshot (type recovery) (description "Malformed annotating syntax stays malformed once every alternative of the production parses. Documentation with no comment body, a `rep` with no language string, and a `comment about` with no annotated element each become a source-backed recovery node at the position they were authored -- before a valid sibling, between two valid siblings, and before a later declaration -- and every later member and declaration still parses. The enumeration body is here because it had no recovery representation at all: an unparseable member used to send it to the closing brace, discarding everything after it with no node and no diagnostic. Editor parsing returns this whole document; strict parsing rejects it, which is what the diagnostics record."))
~~~
# SOURCE
~~~sysml
package AnnotatingMemberRecovery {
    part def Leading {
        doc locale
        doc /* the sibling after malformed leading documentation */
    }
    part def Between {
        doc /* before */
        rep language
        comment /* after */
    }
    part def Trailing {
        comment about
        attribute mass;
    }
    part def Annotated {
        comment about Leading, Between /* an annotated element list is part of the production */
    }
    part def Later {
        rep laterRep language "text" /* the declaration after every recovery */
    }
    enum def Level {
        doc /* before */
        !!not a member;
        high;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_member_recovery.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 66) (line 3) (column 9) (len 19)) (message "missing semicolon before next declaration"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 207) (line 8) (column 9) (len 21)) (message "unexpected keyword `rep` in part definition body"))
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 286) (line 12) (column 9) (len 22)) (message "missing semicolon before next declaration"))
      (diagnostic (code "recovered_enumeration_body_element") (severity error) (category parseerror) (span (offset 619) (line 23) (column 9) (len 24)) (message "unexpected token in enumeration body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingMemberRecovery {
    part def Leading {
        doc locale
        doc
        /* the sibling after malformed leading documentation */
    }
    part def Between {
        doc
        /* before */
        rep language
        comment
        /* after */
    }
    part def Trailing {
        comment about
        attribute mass;
    }
    part def Annotated {
        comment about Leading, Between
        /* an annotated element list is part of the production */
    }
    part def Later {
        rep laterRep language "text"
        /* the declaration after every recovery */
    }
    enum def Level {
        doc
        /* before */
        !!not a member;
        high;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 377) (line 16) (column 23) (len 7)) (segments (segment 0 (token "Leading") (name "Leading") (separator none) (span (offset 377) (line 16) (column 23) (len 7)))))
    (reference r1 (scope relative) (span (offset 386) (line 16) (column 32) (len 7)) (segments (segment 0 (token "Between") (name "Between") (separator none) (span (offset 386) (line 16) (column 32) (len 7)))))
  )
  (root (package (name "AnnotatingMemberRecovery") (body brace (part-def (name "Leading") (modifiers) (body brace (malformed (code "missing_semicolon") (found "doc locale") (span (offset 66) (line 3) (column 9) (len 19))) (doc (name none) (locale none) (body (span (offset 91) (line 4) (column 15) (len 51)) (normalized "the sibling after malformed leading documentation "))))) (part-def (name "Between") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 188) (line 7) (column 15) (len 8)) (normalized "before "))) (malformed (code "unexpected_keyword_in_scope") (found "rep language") (span (offset 207) (line 8) (column 9) (len 21))) (comment (keyword (span (offset 228) (line 9) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 238) (line 9) (column 19) (len 7)) (normalized "after "))))) (part-def (name "Trailing") (modifiers) (body brace (malformed (code "missing_semicolon") (found "comment about") (span (offset 286) (line 12) (column 9) (len 22))) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Annotated") (modifiers) (body brace (comment (keyword (span (offset 363) (line 16) (column 9) (len 7))) (name none) (about (ref r0) (ref r1)) (locale none) (body (span (offset 396) (line 16) (column 42) (len 53)) (normalized "an annotated element list is part of the production "))))) (part-def (name "Later") (modifiers) (body brace (textual-rep (name "laterRep") (language "text") (body (span (offset 518) (line 19) (column 40) (len 38)) (normalized "the declaration after every recovery "))))) (enum-def (name "Level") (body brace (doc (name none) (locale none) (body (span (offset 600) (line 22) (column 15) (len 8)) (normalized "before "))) (malformed (code "recovered_enumeration_body_element") (found "!!not a member;") (span (offset 619) (line 23) (column 9) (len 24))) (enum-value (extensions) (enum-keyword none) (visibility none) (name "high") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 643) (line 24) (column 9) (len 5))))))))
)
~~~
