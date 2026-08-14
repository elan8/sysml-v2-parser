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
    comment locale "en_US"
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
    comment 'about'
    /* Documention Comment about Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        comment 'comment'
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
  (root (package (name "CommentTest") (body (comment) (doc) (comment) (comment) (comment) (part-def (name "C") (body (doc) (comment))) (part-def (name "A") (body semicolon)))))
)
~~~
