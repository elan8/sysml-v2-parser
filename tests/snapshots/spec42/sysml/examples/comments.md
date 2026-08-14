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
    /* Documention Comment on Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        /* Comment in Part Def */
        /* Comment about Package */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Comments") (body (doc) (doc) (comment) (comment) (comment) (part-def (name "C") (body (doc) (comment) (comment))))))
)
~~~
