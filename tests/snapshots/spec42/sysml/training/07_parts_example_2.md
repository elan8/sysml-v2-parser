# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 07 (Parts): Parts Example-2"))
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Parts Example-2' {
    part def Vehicle;
    part def Engine;
    part def Cylinder;
    part vehicle : Vehicle {
        part eng : Engine {
            part cyl : Cylinder[4..6];
        }
    }
    part smallVehicle :> vehicle {
        part  :>> eng {
            part  :>> cyl[4];
        }
    }
    part bigVehicle :> vehicle {
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
    (reference r0 (scope relative) (span (offset 138) (line 11) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 138) (line 11) (column 17) (len 7)))))
  )
  (root (package (name "Parts Example-2") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Cylinder") (body semicolon)) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body brace (part-usage))) (part-usage (declaration-name "smallVehicle") (typing none) (body brace (part-usage))) (part-usage (declaration-name "bigVehicle") (typing none) (body brace (part-usage))))))
)
~~~
