# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 04 (Subsetting): Subsetting Example"))
~~~
# SOURCE
~~~sysml
package 'Subsetting Example' {
	
	part def Vehicle {
		part parts : VehiclePart[*];
		
		part eng : Engine subsets parts;
		part trans : Transmission subsets parts;
		part wheels : Wheel[4] :> parts;
	}
	
	abstract part def VehiclePart;
	part def Engine :> VehiclePart;
	part def Transmission :> VehiclePart;
	part def Wheel :> VehiclePart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "04_subsetting_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Subsetting Example' {
    part def Vehicle {
        part parts : VehiclePart[*];
        part eng : Engine :> parts;
        part trans : Transmission :> parts;
        part wheels : Wheel[4] :> parts;
    }
    abstract part def VehiclePart;
    part def Engine :> VehiclePart;
    part def Transmission :> VehiclePart;
    part def Wheel :> VehiclePart;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Subsetting Example") (body brace (part-def (name "Vehicle") (body brace (part-usage) (part-usage) (part-usage) (part-usage))) (part-def (name "VehiclePart") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-def (name "Wheel") (body semicolon)))))
)
~~~
