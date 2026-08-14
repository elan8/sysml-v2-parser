# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 31 (Constraints): Constraints Example-1"))
~~~
# SOURCE
~~~sysml
package 'Constraints Example-1' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		constraint massConstraint : MassConstraint {
			in partMasses = (chassisMass, engine.mass, transmission.mass);
			in massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_constraints_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Constraints Example-1' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;
    part def Engine;
    part def Transmission;
    constraint def MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;
        sum(partMasses) <= massLimit;
    }
    part def Vehicle {
        constraint massConstraint : MassConstraint {
            in partMasses = (chassisMass, engine.mass, transmission.mass);
            in massLimit = 2500 [kg];
        }
        attribute chassisMass : MassValue;
        part engine : Engine {
            attribute mass : MassValue;
        }
        part transmission : Engine {
            attribute mass : MassValue;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 50) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 74) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 74) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 97) (line 4) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 97) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 496) (line 22) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 496) (line 22) (column 27) (len 9)))))
  )
  (root (package (name "Constraints Example-1") (body (import (target (span (span (offset 50) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 20) (len 3))) (separator (span (offset 53) (line 2) (column 20) (len 2))) (marker (span (offset 55) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 76) (line 3) (column 19) (len 3))) (separator (span (offset 76) (line 3) (column 19) (len 2))) (marker (span (offset 78) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 97) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 115) (line 4) (column 35) (len 3))) (separator (span (offset 115) (line 4) (column 35) (len 2))) (marker (span (offset 117) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (constraint-def) (part-def (name "Vehicle") (body (constraint-usage) (attribute-usage (declaration-name "chassisMass") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage))))))
)
~~~
