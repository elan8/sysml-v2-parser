# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 03 (Generalization): Generalization Example"))
~~~
# SOURCE
~~~sysml
package 'Generalization Example' {

	abstract part def Vehicle;
	
	part def HumanDrivenVehicle specializes Vehicle {
		ref part driver : Person;
	}
	
	part def PoweredVehicle :> Vehicle {
		part eng : Engine;
	}
	
	part def HumanDrivenPoweredVehicle :> 
		HumanDrivenVehicle, PoweredVehicle;
	
	part def Engine;	
	part def Person;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "03_generalization_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Generalization Example' {
    abstract part def Vehicle;
    part def HumanDrivenVehicle :> Vehicle {
        ref part driver : Person;
    }
    part def PoweredVehicle :> Vehicle {
        part eng : Engine;
    }
    part def HumanDrivenPoweredVehicle :> HumanDrivenVehicle, PoweredVehicle;
    part def Engine;
    part def Person;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Generalization Example") (body (part-def (name "Vehicle") (body semicolon)) (part-def (name "HumanDrivenVehicle") (body (part-usage))) (part-def (name "PoweredVehicle") (body (part-usage))) (part-def (name "HumanDrivenPoweredVehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Person") (body semicolon)))))
)
~~~
