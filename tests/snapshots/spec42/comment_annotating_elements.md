# META
~~~sexpr
(snapshot (type semantic) (description "A bare /* ... */ at a member position is the Comment production's keyword-less spelling, not trivia: it reaches the AST at file level and in every nested body, survives formatting, and keeps the exact span of its body. // and //* notes stay trivia. Every annotating body -- doc, comment, rep -- reports its raw span and the text the pinned processing rules (KerML BNF 214 note 1) make of it (spec42 Gap 55)."))
~~~
# SOURCE
~~~sysml
/* A bare block comment is the whole document's first member. */
// A line note is trivia.
//* A multiline note is trivia too. */
package CommentAnnotatingElements {
    /* Between the brace and the first member. */
    /** The doc-style spelling is a Comment whose body starts with a star. */
    doc
    /*
     * A leading star and one following space come off every subsequent line.
     * This second line proves the rule repeats.
     */
    part def Documented {
        /* Nested one level down. */
        attribute mass : Real;
        /* And after a member. */
    }
    comment named locale "en"
    /* The keyworded spelling keeps its keyword span. */
    rep asText language "text"
    /* A textual representation body normalizes the same way. */
    attribute inline : /* mid-declaration comments stay trivia */ Real;
}
/* And a bare comment can close the file. */
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_annotating_elements.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
/* A bare block comment is the whole document's first member. */

package CommentAnnotatingElements {
    /* Between the brace and the first member. */
    /** The doc-style spelling is a Comment whose body starts with a star. */
    doc
    /*
     * A leading star and one following space come off every subsequent line.
     * This second line proves the rule repeats.
     */
    part def Documented {
        /* Nested one level down. */
        attribute mass : Real;
        /* And after a member. */
    }
    comment named locale "en"
    /* The keyworded spelling keeps its keyword span. */
    rep asText language "text"
    /* A textual representation body normalizes the same way. */
    attribute def inline : Real;
}

/* And a bare comment can close the file. */
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 532) (line 14) (column 26) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 532) (line 14) (column 26) (len 4)))))
    (reference r1 (scope relative) (span (offset 827) (line 21) (column 67) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 827) (line 21) (column 67) (len 4)))))
  )
  (root (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2) (line 1) (column 3) (len 60)) (normalized "A bare block comment is the whole document's first member. "))) (package (name "CommentAnnotatingElements") (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 172) (line 5) (column 7) (len 41)) (normalized "Between the brace and the first member. "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 222) (line 6) (column 7) (len 69)) (normalized "* The doc-style spelling is a Comment whose body starts with a star. "))) (doc (name none) (locale none) (body (span (offset 308) (line 8) (column 7) (len 133)) (normalized "A leading star and one following space come off every subsequent line.\nThis second line proves the rule repeats.\n"))) (part-def (name "Documented") (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 480) (line 13) (column 11) (len 24)) (normalized "Nested one level down. "))) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 548) (line 15) (column 11) (len 21)) (normalized "And after a member. "))))) (comment (keyword (span (offset 582) (line 17) (column 5) (len 7))) (name "named") (about) (locale "en") (body (span (offset 614) (line 18) (column 7) (len 48)) (normalized "The keyworded spelling keeps its keyword span. "))) (textual-rep (name "asText") (language "text") (body (span (offset 702) (line 20) (column 7) (len 56)) (normalized "A textual representation body normalizes the same way. "))) (attribute-def (declaration-name "inline") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 837) (line 23) (column 3) (len 40)) (normalized "And a bare comment can close the file. "))))
)
~~~
