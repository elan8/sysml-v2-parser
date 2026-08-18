# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 31 (Constraints): Constraint Assertions-2"))
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
	}
	
	constraint massConstraint : MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert massConstraint {
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
  (document "31_constraint_assertions_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Constraint Assertions-2' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;
    part def Engine;
    part def Transmission;
    constraint def MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;
    }
    constraint massConstraint : MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;
        sum(partMasses) <= massLimit;
    }
    part def Vehicle {
        assert massConstraint {
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
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 52) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 76) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 76) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 99) (line 4) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 99) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 591) (line 27) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 591) (line 27) (column 27) (len 9)))))
    (reference r4 (scope relative) (span (offset 621) (line 29) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 621) (line 29) (column 17) (len 6)))))
    (reference r5 (scope relative) (span (offset 650) (line 30) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 650) (line 30) (column 21) (len 9)))))
    (reference r6 (scope relative) (span (offset 690) (line 33) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 690) (line 33) (column 23) (len 6)))))
    (reference r7 (scope relative) (span (offset 719) (line 34) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 719) (line 34) (column 21) (len 9)))))
  )
  (root (package (name "Constraint Assertions-2") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 20) (len 3))) (separator (span (offset 55) (line 2) (column 20) (len 2))) (marker (span (offset 57) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 3) (column 19) (len 3))) (separator (span (offset 78) (line 3) (column 19) (len 2))) (marker (span (offset 80) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 99) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 117) (line 4) (column 35) (len 3))) (separator (span (offset 117) (line 4) (column 35) (len 2))) (marker (span (offset 119) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (constraint-def) (constraint-usage (name "massConstraint") (short-name none) (multiplicity none)) (part-def (name "Vehicle") (body brace (assert-constraint) (attribute-usage (declaration-name "chassisMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
