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
    (reference r0 (scope relative) (span (offset 203) (line 17) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 203) (line 17) (column 22) (len 7)))))
    (reference r1 (scope relative) (span (offset 290) (line 23) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 290) (line 23) (column 20) (len 7)))))
  )
  (root (package (name "Parts Example-1") (body brace (part-def (name "Vehicle") (body brace (part-usage))) (part-def (name "Engine") (body brace (part-usage))) (part-def (name "Cylinder") (body semicolon)) (part-usage (declaration-name "smallVehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage))) (part-usage (declaration-name "bigVehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage))))))
)
~~~
