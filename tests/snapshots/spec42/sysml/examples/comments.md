# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Comment): Comments"))
~~~
# SOURCE
~~~sysml
package Comments {
	doc /* Documentation Comment */

	doc /* Documentation about Package */

	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Comment about Comment */
	
	comment about C /* Documention Comment on Part Def */
	part def C {
		doc /* Documentation in Part Def */
		comment /* Comment in Part Def */
		comment about Comments /* Comment about Package */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comments.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Comments {
    doc
    /* Documentation Comment */
    doc
    /* Documentation about Package */
    comment cmt
    /* Named Comment */
    comment cmt_cmt about cmt
    /* Comment about Comment */
    comment about C
    /* Documention Comment on Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        comment
        /* Comment in Part Def */
        comment about Comments
        /* Comment about Package */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 150) (line 7) (column 24) (len 3)) (segments (segment 0 (token "cmt") (name "cmt") (separator none) (span (offset 150) (line 7) (column 24) (len 3)))))
    (reference r1 (scope relative) (span (offset 199) (line 9) (column 16) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 199) (line 9) (column 16) (len 1)))))
    (reference r2 (scope relative) (span (offset 343) (line 13) (column 17) (len 8)) (segments (segment 0 (token "Comments") (name "Comments") (separator none) (span (offset 343) (line 13) (column 17) (len 8)))))
  )
  (root (package (name "Comments") (body brace (doc) (doc) (comment (keyword (span (offset 94) (line 6) (column 2) (len 7))) (name "cmt") (about) (locale none)) (comment (keyword (span (offset 128) (line 7) (column 2) (len 7))) (name "cmt_cmt") (about (ref r0)) (locale none)) (comment (keyword (span (offset 185) (line 9) (column 2) (len 7))) (name none) (about (ref r1)) (locale none)) (part-def (name "C") (body brace (doc) (comment (keyword (span (offset 293) (line 12) (column 3) (len 7))) (name none) (about) (locale none)) (comment (keyword (span (offset 329) (line 13) (column 3) (len 7))) (name none) (about (ref r2)) (locale none)))))))
)
~~~
