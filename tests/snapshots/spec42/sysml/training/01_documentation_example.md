# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 01 (Packages): Documentation Example"))
~~~
# SOURCE
~~~sysml
package 'Documentation Example' {
	doc /* This is documentation of the owning 
	     * package.
	     */
	
	part def Automobile {
		doc Document1 /* This documentation of Automobile. */
	}
	
	alias Car for Automobile {
		doc /* This is documentation of the alias. */
	}
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_documentation_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Documentation Example' {
    doc
    /* This is documentation of the owning 
	     * package.
	     */
    part def Automobile {
        doc Document1
        /* This documentation of Automobile. */
    }
    alias Car for Automobile {
        doc
        /* This is documentation of the alias. */
    }
    alias Torque for ISQ::TorqueValue;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 206) (line 10) (column 16) (len 10)) (segments (segment 0 (token "Automobile") (name "Automobile") (separator none) (span (offset 206) (line 10) (column 16) (len 10)))))
    (reference r1 (scope relative) (span (offset 288) (line 13) (column 19) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 288) (line 13) (column 19) (len 3))) (segment 1 (token "TorqueValue") (name "TorqueValue") (separator colon-colon) (span (offset 293) (line 13) (column 24) (len 11)))))
  )
  (root (package (name "Documentation Example") (body brace (doc (name none) (locale none) (body (span (offset 41) (line 2) (column 8) (len 61)) (normalized "This is documentation of the owning \npackage.\n"))) (part-def (name "Automobile") (modifiers) (body brace (doc (name "Document1") (locale none) (body (span (offset 148) (line 7) (column 19) (len 35)) (normalized "This documentation of Automobile. "))))) (alias (name "Car") (target (ref r0)) (body brace (element-count 1))) (alias (name "Torque") (target (ref r1)) (body semicolon)))))
)
~~~
