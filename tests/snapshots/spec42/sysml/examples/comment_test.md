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
    comment cmt_cmt about cmt
    /* Comment about Comment */
    comment about C
    /* Documention Comment about Part Def */
    part def C {
        doc
        /* Documentation in Part Def */
        comment
        /* Comment in Part Def */
        comment about CommentTest locale "en_US"
        /* Comment about Package */
    }
    part def A;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 472) (line 34) (column 24) (len 3)) (segments (segment 0 (token "cmt") (name "cmt") (separator none) (span (offset 472) (line 34) (column 24) (len 3)))))
    (reference r1 (scope relative) (span (offset 521) (line 36) (column 16) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 521) (line 36) (column 16) (len 1)))))
    (reference r2 (scope relative) (span (offset 668) (line 40) (column 17) (len 11)) (segments (segment 0 (token "CommentTest") (name "CommentTest") (separator none) (span (offset 668) (line 40) (column 17) (len 11)))))
  )
  (root (package (name "CommentTest") (body brace (comment (keyword none) (name none) (about) (locale "en_US")) (doc) (comment (keyword (span (offset 416) (line 33) (column 2) (len 7))) (name "cmt") (about) (locale none)) (comment (keyword (span (offset 450) (line 34) (column 2) (len 7))) (name "cmt_cmt") (about (ref r0)) (locale none)) (comment (keyword (span (offset 507) (line 36) (column 2) (len 7))) (name none) (about (ref r1)) (locale none)) (part-def (name "C") (body brace (doc) (comment (keyword (span (offset 618) (line 39) (column 3) (len 7))) (name none) (about) (locale none)) (comment (keyword (span (offset 654) (line 40) (column 3) (len 7))) (name none) (about (ref r2)) (locale "en_US")))) (part-def (name "A") (body semicolon)))))
)
~~~
