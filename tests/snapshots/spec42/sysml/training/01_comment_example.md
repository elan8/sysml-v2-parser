# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 01 (Packages): Comment Example"))
~~~
# SOURCE
~~~sysml
package 'Comment Example' {
	/* This is a comment, which is a part of the model, 
	 * annotating (by default) it's owning namespace. */
	
	comment Comment1 /* This is a named comment. */
	
	comment about Automobile
	/* This is an unnamed comment, annotating an 
	 * explicitly specified element. 
	 */
	 
	part def Automobile;
	
	alias Car for Automobile {
		/*
		 * This is a comment annotating its owning
		 * element.
		 */
	}	                         
	
	// This is a note. It is in the text, but not part 
	// of the model.
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_comment_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Comment Example' {
    comment Comment1
    /* This is a named comment. */
    comment about
    /* This is an unnamed comment, annotating an 
	 * explicitly specified element. 
	 */
    part def Automobile;
    alias Car for Automobile {}
    alias Torque for ISQ::TorqueValue;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 344) (line 14) (column 16) (len 10)) (segments (segment 0 (token "Automobile") (name "Automobile") (separator none) (span (offset 344) (line 14) (column 16) (len 10)))))
    (reference r1 (scope relative) (span (offset 547) (line 23) (column 19) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 547) (line 23) (column 19) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 552) (line 23) (column 24) (len 11)))))
  )
  (root (package (name "Comment Example") (body (comment) (comment) (part-def (name "Automobile") (body semicolon)) (alias (name "Car") (target (ref r0)) (body brace (element-count 0))) (alias (name "Torque") (target (ref r1)) (body semicolon)))))
)
~~~
