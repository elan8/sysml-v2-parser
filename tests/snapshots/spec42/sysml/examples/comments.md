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
    comment cmt_cmt
    /* Comment about Comment */
    comment
    /* Documention Comment on Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        comment
        /* Comment in Part Def */
        comment
        /* Comment about Package */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Comments") (body brace (doc) (doc) (comment (keyword (span (offset 94) (line 6) (column 2) (len 7))) (name "cmt") (locale none)) (comment (keyword (span (offset 128) (line 7) (column 2) (len 7))) (name "cmt_cmt") (locale none)) (comment (keyword (span (offset 185) (line 9) (column 2) (len 7))) (name none) (locale none)) (part-def (name "C") (body brace (doc) (comment) (comment))))))
)
~~~
