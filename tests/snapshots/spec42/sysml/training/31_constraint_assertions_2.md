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
    (reference r3 (scope relative) (span (offset 298) (line 14) (column 30) (len 14)) (segments (segment 0 (token "MassConstraint") (name "MassConstraint") (separator none) (span (offset 298) (line 14) (column 30) (len 14)))))
    (reference r4 (scope relative) (span (offset 384) (line 18) (column 3) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 384) (line 18) (column 3) (len 3)))))
    (reference r5 (scope relative) (span (offset 388) (line 18) (column 7) (len 10)) (segments (segment 0 (token "partMasses") (name "partMasses") (separator none) (span (offset 388) (line 18) (column 7) (len 10)))))
    (reference r6 (scope relative) (span (offset 403) (line 18) (column 22) (len 9)) (segments (segment 0 (token "massLimit") (name "massLimit") (separator none) (span (offset 403) (line 18) (column 22) (len 9)))))
    (reference r7 (scope relative) (span (offset 591) (line 27) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 591) (line 27) (column 27) (len 9)))))
    (reference r8 (scope relative) (span (offset 621) (line 29) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 621) (line 29) (column 17) (len 6)))))
    (reference r9 (scope relative) (span (offset 650) (line 30) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 650) (line 30) (column 21) (len 9)))))
    (reference r10 (scope relative) (span (offset 690) (line 33) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 690) (line 33) (column 23) (len 6)))))
    (reference r11 (scope relative) (span (offset 719) (line 34) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 719) (line 34) (column 21) (len 9)))))
  )
  (root (package (name "Constraint Assertions-2") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 20) (len 3))) (separator (span (offset 55) (line 2) (column 20) (len 2))) (marker (span (offset 57) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 3) (column 19) (len 3))) (separator (span (offset 78) (line 3) (column 19) (len 2))) (marker (span (offset 80) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 99) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 117) (line 4) (column 35) (len 3))) (separator (span (offset 117) (line 4) (column 35) (len 2))) (marker (span (offset 119) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (constraint-def (name "MassConstraint") (specializes none) (body brace (in-out-declaration) (in-out-declaration))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "massConstraint") (short-name none) (type (ref r3)) (multiplicity none) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 384) (line 18) (column 3) (len 28)) (binary (operator "<=") (left (expression (span (offset 384) (line 18) (column 3) (len 15)) (invocation (callee (expression (span (offset 384) (line 18) (column 3) (len 3)) (ref r4))) (arguments (argument (parameter none) (value (expression (span (offset 388) (line 18) (column 7) (len 10)) (ref r5)))))))) (right (expression (span (offset 403) (line 18) (column 22) (len 9)) (ref r6))))))) (part-def (name "Vehicle") (body brace (assert-constraint) (attribute-usage (declaration-name "chassisMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
