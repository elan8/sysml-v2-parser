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
    concern modularity {
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
        frame modularity;
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
  )
  (root (package (name "Viewpoint Example") (body (part-def (name "Systems Engineer") (body semicolon)) (part-def (name "IV&V") (body semicolon)) (concern-usage) (concern-usage) (viewpoint-usage))))
)
~~~
