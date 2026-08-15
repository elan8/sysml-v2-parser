# META
~~~sexpr
(snapshot (type semantic) (description "Whether a comment member wrote its `comment` keyword is a grammatical fact, not a formatting choice: KerML 8.2.3.3.2 makes the keyword optional, and a member emitted without it becomes a bare block comment, which reparses as trivia and disappears. All four spellings are byte-identical after formatting -- (stable-idempotent) -- and the projection names the keyword span and locale so the keyword-less form cannot silently acquire one."))
~~~
# SOURCE
~~~sysml
package CommentKeywordProvenance {
    comment /* an anonymous comment member */
    comment named /* a named comment member */
    locale "en_US" /* the keyword-less spelling */
    comment locale "en_US" /* both the keyword and a locale */
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_keyword_provenance.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CommentKeywordProvenance {
    comment
    /* an anonymous comment member */
    comment named
    /* a named comment member */
    locale "en_US"
    /* the keyword-less spelling */
    comment locale "en_US"
    /* both the keyword and a locale */
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "CommentKeywordProvenance") (body brace (comment (keyword (span (offset 39) (line 2) (column 5) (len 7))) (name none) (about) (locale none)) (comment (keyword (span (offset 85) (line 3) (column 5) (len 7))) (name "named") (about) (locale none)) (comment (keyword none) (name none) (about) (locale "en_US")) (comment (keyword (span (offset 183) (line 5) (column 5) (len 7))) (name none) (about) (locale "en_US")))))
)
~~~
