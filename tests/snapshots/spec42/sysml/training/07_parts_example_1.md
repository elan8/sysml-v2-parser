# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 07 (Parts): Parts Example-1"))
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {
    part def Vehicle {
        part eng : Engine;
    }
    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def Cylinder;
    part smallVehicle : Vehicle {
        part  :>> eng {
            part  :>> cyl[4];
        }
    }
    part bigVehicle : Vehicle {
        part  :>> eng {
            part  :>> cyl[6];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Parts Example-1") (body brace (part-def (name "Vehicle") (body brace (part-usage))) (part-def (name "Engine") (body brace (part-usage))) (part-def (name "Cylinder") (body semicolon)) (part-usage) (part-usage))))
)
~~~
