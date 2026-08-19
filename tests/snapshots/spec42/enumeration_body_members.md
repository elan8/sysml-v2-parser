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
~~~sysml
enum def Color {
    doc
    /* The colours this enumeration defines. */
    red = 1;
    comment
    /* between two values */
    green {
        doc
        /* green is the second literal */
    }
    rep asText language "text"
    /* Color rendering */
    @Palette;
    blue = 3 {
        doc
        /* blue carries both a body and an initializer */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 271) (line 13) (column 6) (len 7)) (segments (segment 0 (token "Palette") (name "Palette") (separator none) (span (offset 271) (line 13) (column 6) (len 7)))))
  )
  (root (enum-def (name "Color") (body brace (doc) (enum-value (name "red") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 88) (line 4) (column 16) (len 1)) (integer 1))))) (body semicolon) (span (offset 77) (line 4) (column 5) (len 13))) (comment (keyword (span (offset 95) (line 5) (column 5) (len 7))) (name none) (about) (locale none)) (enum-value (name "green") (short-name none) (value none) (body brace (doc)) (span (offset 136) (line 7) (column 5) (len 72))) (textual-rep) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (enum-value (name "blue") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 296) (line 14) (column 17) (len 1)) (integer 3))))) (body brace (doc)) (span (offset 284) (line 14) (column 5) (len 91))))))
)
~~~
