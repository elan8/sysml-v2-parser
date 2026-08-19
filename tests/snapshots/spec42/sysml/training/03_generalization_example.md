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
    part def HumanDrivenVehicle specializes Vehicle {
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
    (reference r0 (scope relative) (span (offset 137) (line 6) (column 21) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 137) (line 6) (column 21) (len 6)))))
    (reference r1 (scope relative) (span (offset 201) (line 10) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 201) (line 10) (column 14) (len 6)))))
  )
  (root (package (name "Generalization Example") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "HumanDrivenVehicle") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "driver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "PoweredVehicle") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "HumanDrivenPoweredVehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Person") (body semicolon)))))
)
~~~
