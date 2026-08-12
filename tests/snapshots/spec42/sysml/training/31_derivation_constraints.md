# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 31 (Constraints): Derivation Constraints"))
~~~
# SOURCE
~~~sysml
package 'Derivation Constraints' {
	private import SI::*;
	private import 'Constraints Example-1'::*;
	
	part vehicle1 : Vehicle {
		attribute totalMass : MassValue;			
		assert constraint {totalMass == chassisMass + engine.mass + transmission.mass}	
	}
	
	part vehicle2 : Vehicle {
		attribute totalMass : MassValue = chassisMass + engine.mass + transmission.mass;
	}
	
	constraint def Dynamics {
		in mass: MassValue;
		in initialSpeed : SpeedValue;
		in finalSpeed : SpeedValue;
		in deltaT : TimeValue;
		in force : ForceValue;

		force * deltaT == mass * (finalSpeed - initialSpeed) and
		mass > 0[kg]
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_derivation_constraints.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Derivation Constraints' {
    private import SI::*;
    private import 'Constraints Example-1'::*;
    part vehicle1 : Vehicle {
        attribute totalMass : MassValue;
        assert constraint  {
            totalMass == chassisMass + engine.mass + transmission.mass;
        }
    }
    part vehicle2 : Vehicle {
        attribute totalMass : MassValue = chassisMass + engine.mass + transmission.mass;
    }
    constraint def Dynamics {
        in mass : MassValue;
        in initialSpeed : SpeedValue;
        in finalSpeed : SpeedValue;
        in deltaT : TimeValue;
        in force : ForceValue;
        force * deltaT == mass * (finalSpeed - initialSpeed) && mass > 0 [kg];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 51) (line 2) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 51) (line 2) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 74) (line 3) (column 17) (len 23)) (segments (segment 0 (token "'Constraints Example-1'") (name "Constraints Example-1") (separator none) (span (offset 74) (line 3) (column 17) (len 23)))))
  )
  (root (package (name "Derivation Constraints") (body (import (target (span (span (offset 51) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 19) (len 3))) (separator (span (offset 53) (line 2) (column 19) (len 2))) (marker (span (offset 55) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 26))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 97) (line 3) (column 40) (len 3))) (separator (span (offset 97) (line 3) (column 40) (len 2))) (marker (span (offset 99) (line 3) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage) (constraint-def))))
)
~~~
