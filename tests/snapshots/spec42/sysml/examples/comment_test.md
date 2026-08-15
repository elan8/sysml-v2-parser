# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): CommentTest"))
~~~
# SOURCE
~~~sysml
  /* AAA */
  //a lexical comment ("note") is not a part of model
package CommentTest {
	// inside package
	/*
*AAA
 * BBB*/	
 /*
    *
    *
    * AAA  ***   
    *BBB
    								*/

   /*
 *       AAAA
 *       BBBB           */	
 /* AAAA
 
 
  * BBBB
 *
 * CCCC
 */
 locale "en_US" /*
 * AAAA
 * BBBB
 *    CCC DDD    
 */
	
	/* comment inside a package */
	doc locale "en_US" /* Documentation about Package */
	comment cmt /* Named Comment */	
	comment cmt_cmt about cmt /* Comment about Comment */
	
	comment about C /* Documention Comment about Part Def */
	part def C {
		doc /* Documentation in Part Def */
		comment /* Comment in Part Def */
		comment about CommentTest locale "en_US" /* Comment about Package */
	}
	/* abc */
	part def A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comment_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CommentTest {
    locale "en_US"
    /*
 * AAAA
 * BBBB
 *    CCC DDD    
 */
    doc locale "en_US"
    /* Documentation about Package */
    comment cmt
    /* Named Comment */
    comment cmt_cmt
    /* Comment about Comment */
    comment
    /* Documention Comment about Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        comment
        /* Comment in Part Def */
        comment
        /* Comment about Package */
    }
    part def A;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "CommentTest") (body brace (comment (keyword none) (name none) (locale "en_US")) (doc) (comment (keyword (span (offset 416) (line 33) (column 2) (len 7))) (name "cmt") (locale none)) (comment (keyword (span (offset 450) (line 34) (column 2) (len 7))) (name "cmt_cmt") (locale none)) (comment (keyword (span (offset 507) (line 36) (column 2) (len 7))) (name none) (locale none)) (part-def (name "C") (body brace (doc) (comment) (comment))) (part-def (name "A") (body semicolon)))))
)
~~~
