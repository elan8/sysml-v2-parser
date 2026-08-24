# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 42 (Views): Viewpoint Example"))
~~~
# SOURCE
~~~sysml
package 'Viewpoint Example' {	
	part def 'Systems Engineer';
	part def 'IV&V';
	
	concern 'system breakdown' {
		doc /* 
		 * To ensure that a system covers all its required capabilities,
		 * it is necessary to understand how it is broken down into
		 * subsystems and components that provide those capabilities.
		 */
		subject;
		stakeholder se : 'Systems Engineer';
		stakeholder ivv : 'IV&V';
	}
	
	concern 'modularity' {
		doc /*
		 * There should be well defined interfaces between the parts of
		 * a system that allow each part to be understood individually,
		 * as well as being part of the whole system.
		 */		 
        subject;
		stakeholder se : 'Systems Engineer';
	}
	
	viewpoint 'system structure perspective' {		
		frame 'system breakdown';
		frame 'modularity';
		
		require constraint {
			doc /*
			 * A system structure view shall show the hierarchical 
			 * part decomposition of a system, starting with a 
			 * specified root part.
			 */
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "42_viewpoint_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Viewpoint Example' {
    part def 'Systems Engineer';
    part def 'IV&V';
    concern 'system breakdown' {
        doc
        /* 
		 * To ensure that a system covers all its required capabilities,
		 * it is necessary to understand how it is broken down into
		 * subsystems and components that provide those capabilities.
		 */
        subject;
        stakeholder se : 'Systems Engineer';
        stakeholder ivv : 'IV&V';
    }
    concern 'modularity' {
        doc
        /*
		 * There should be well defined interfaces between the parts of
		 * a system that allow each part to be understood individually,
		 * as well as being part of the whole system.
		 */
        subject;
        stakeholder se : 'Systems Engineer';
    }
    viewpoint 'system structure perspective' {
        frame 'system breakdown';
        frame 'modularity';
        require constraint {
            doc
            /*
			 * A system structure view shall show the hierarchical 
			 * part decomposition of a system, starting with a 
			 * specified root part.
			 */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 350) (line 12) (column 20) (len 18)) (segments (segment 0 (token "'Systems Engineer'") (name "Systems Engineer") (separator none) (span (offset 350) (line 12) (column 20) (len 18)))))
    (reference r1 (scope relative) (span (offset 390) (line 13) (column 21) (len 6)) (segments (segment 0 (token "'IV&V'") (name "IV&V") (separator none) (span (offset 390) (line 13) (column 21) (len 6)))))
    (reference r2 (scope relative) (span (offset 661) (line 23) (column 20) (len 18)) (segments (segment 0 (token "'Systems Engineer'") (name "Systems Engineer") (separator none) (span (offset 661) (line 23) (column 20) (len 18)))))
  )
  (root (package (name "Viewpoint Example") (body brace (part-def (name "Systems Engineer") (modifiers) (body semicolon)) (part-def (name "IV&V") (modifiers) (body semicolon)) (concern-usage (name "system breakdown") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 119) (line 6) (column 9) (len 198)) (normalized "To ensure that a system covers all its required capabilities,\nit is necessary to understand how it is broken down into\nsubsystems and components that provide those capabilities.\n"))) (subject-ref) (stakeholder (declaration "se") (target none) (type (ref r0)) (redefinition false)) (stakeholder (declaration "ivv") (target none) (type (ref r1)) (redefinition false)))) (concern-usage (name "modularity") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 435) (line 17) (column 9) (len 184)) (normalized "There should be well defined interfaces between the parts of\na system that allow each part to be understood individually,\nas well as being part of the whole system.\n"))) (subject-ref) (stakeholder (declaration "se") (target none) (type (ref r2)) (redefinition false)))) (viewpoint-usage))))
)
~~~
