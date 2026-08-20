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
/* AAA */

package CommentTest {
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
    locale "en_US"
    /*
 * AAAA
 * BBBB
 *    CCC DDD    
 */
    /* comment inside a package */
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
    /* abc */
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
  (root (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4) (line 1) (column 5) (len 5)) (normalized "AAA "))) (package (name "CommentTest") (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 110) (line 5) (column 4) (len 12)) (normalized "AAA\nBBB"))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 129) (line 8) (column 4) (len 52)) (normalized "\n\nAAA  ***   \nBBB\n"))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 190) (line 15) (column 6) (len 39)) (normalized "      AAAA\n      BBBB           "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 236) (line 18) (column 4) (len 31)) (normalized "AAAA\n\n\nBBBB\n\nCCCC\n"))) (comment (keyword none) (name none) (about) (locale "en_US") (body (span (offset 288) (line 25) (column 19) (len 36)) (normalized "AAAA\nBBBB\n   CCC DDD    \n"))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 332) (line 31) (column 4) (len 26)) (normalized "comment inside a package "))) (doc (name none) (locale "en_US") (body (span (offset 383) (line 32) (column 23) (len 29)) (normalized "Documentation about Package "))) (comment (keyword (span (offset 416) (line 33) (column 2) (len 7))) (name "cmt") (about) (locale none) (body (span (offset 430) (line 33) (column 16) (len 15)) (normalized "Named Comment "))) (comment (keyword (span (offset 450) (line 34) (column 2) (len 7))) (name "cmt_cmt") (about (ref r0)) (locale none) (body (span (offset 478) (line 34) (column 30) (len 23)) (normalized "Comment about Comment "))) (comment (keyword (span (offset 507) (line 36) (column 2) (len 7))) (name none) (about (ref r1)) (locale none) (body (span (offset 525) (line 36) (column 20) (len 36)) (normalized "Documention Comment about Part Def "))) (part-def (name "C") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 586) (line 38) (column 9) (len 27)) (normalized "Documentation in Part Def "))) (comment (keyword (span (offset 618) (line 39) (column 3) (len 7))) (name none) (about) (locale none) (body (span (offset 628) (line 39) (column 13) (len 21)) (normalized "Comment in Part Def "))) (comment (keyword (span (offset 654) (line 40) (column 3) (len 7))) (name none) (about (ref r2)) (locale "en_US") (body (span (offset 697) (line 40) (column 46) (len 23)) (normalized "Comment about Package "))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 729) (line 42) (column 4) (len 5)) (normalized "abc "))) (part-def (name "A") (modifiers) (body semicolon)))))
)
~~~
