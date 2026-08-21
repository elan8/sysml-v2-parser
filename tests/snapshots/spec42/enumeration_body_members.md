# META
~~~sexpr
(snapshot (type semantic) (description "An enumeration body carries the complete AnnotatingElement production -- doc, comment, rep and @metadata -- interleaved with its enumerated values in source order, and each value keeps its own body and its `= expr` initializer (spec42 Gap 56)."))
~~~
# SOURCE
~~~sysml
enum def Color {
    doc
    /* The colours this enumeration defines. */
    enum red = 1;
    comment
    /* between two values */
    enum green {
        doc
        /* green is the second literal */
    }
    rep asText language "text"
    /* Color rendering */
    @Palette;
    enum blue = 3 {
        doc
        /* blue carries both a body and an initializer */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumeration_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 271) (line 13) (column 6) (len 7)) (segments (segment 0 (token "Palette") (name "Palette") (separator none) (span (offset 271) (line 13) (column 6) (len 7)))))
  )
  (root (enum-def (name "Color") (body brace (doc (name none) (locale none) (body (span (offset 31) (line 3) (column 7) (len 39)) (normalized "The colours this enumeration defines. "))) (enum-value (enum-keyword present) (visibility none) (name "red") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 88) (line 4) (column 16) (len 1)) (integer 1))))) (body semicolon) (span (offset 77) (line 4) (column 5) (len 13))) (comment (keyword (span (offset 95) (line 5) (column 5) (len 7))) (name none) (about) (locale none) (body (span (offset 109) (line 6) (column 7) (len 20)) (normalized "between two values "))) (enum-value (enum-keyword present) (visibility none) (name "green") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 171) (line 9) (column 11) (len 29)) (normalized "green is the second literal ")))) (span (offset 136) (line 7) (column 5) (len 72))) (textual-rep (name "asText") (language "text") (body (span (offset 246) (line 12) (column 7) (len 17)) (normalized "Color rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (enum-value (enum-keyword present) (visibility none) (name "blue") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 296) (line 14) (column 17) (len 1)) (integer 3))))) (body brace (doc (name none) (locale none) (body (span (offset 322) (line 16) (column 11) (len 45)) (normalized "blue carries both a body and an initializer ")))) (span (offset 284) (line 14) (column 5) (len 91))))))
)
~~~
