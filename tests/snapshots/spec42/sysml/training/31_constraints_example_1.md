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
    (reference r3 (scope relative) (span (offset 268) (line 13) (column 3) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 268) (line 13) (column 3) (len 3)))))
    (reference r4 (scope relative) (span (offset 272) (line 13) (column 7) (len 10)) (segments (segment 0 (token "partMasses") (name "partMasses") (separator none) (span (offset 272) (line 13) (column 7) (len 10)))))
    (reference r5 (scope relative) (span (offset 287) (line 13) (column 22) (len 9)) (segments (segment 0 (token "massLimit") (name "massLimit") (separator none) (span (offset 287) (line 13) (column 22) (len 9)))))
    (reference r6 (scope relative) (span (offset 352) (line 17) (column 31) (len 14)) (segments (segment 0 (token "MassConstraint") (name "MassConstraint") (separator none) (span (offset 352) (line 17) (column 31) (len 14)))))
    (reference r7 (scope relative) (span (offset 496) (line 22) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 496) (line 22) (column 27) (len 9)))))
    (reference r8 (scope relative) (span (offset 526) (line 24) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 526) (line 24) (column 17) (len 6)))))
    (reference r9 (scope relative) (span (offset 555) (line 25) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 555) (line 25) (column 21) (len 9)))))
    (reference r10 (scope relative) (span (offset 595) (line 28) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 595) (line 28) (column 23) (len 6)))))
    (reference r11 (scope relative) (span (offset 624) (line 29) (column 21) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 624) (line 29) (column 21) (len 9)))))
  )
  (root (package (name "Constraints Example-1") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 20) (len 3))) (separator (span (offset 53) (line 2) (column 20) (len 2))) (marker (span (offset 55) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 76) (line 3) (column 19) (len 3))) (separator (span (offset 76) (line 3) (column 19) (len 2))) (marker (span (offset 78) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 97) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 115) (line 4) (column 35) (len 3))) (separator (span (offset 115) (line 4) (column 35) (len 2))) (marker (span (offset 117) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (constraint-def (name "MassConstraint") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 268) (line 13) (column 3) (len 28)) (binary (operator "<=") (left (expression (span (offset 268) (line 13) (column 3) (len 15)) (invocation (callee (expression (span (offset 268) (line 13) (column 3) (len 3)) (ref r3))) (arguments (argument (parameter none) (value (expression (span (offset 272) (line 13) (column 7) (len 10)) (ref r4)))))))) (right (expression (span (offset 287) (line 13) (column 22) (len 9)) (ref r5))))))) (part-def (name "Vehicle") (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "massConstraint") (short-name none) (type (ref r6)) (multiplicity none) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (attribute-usage (declaration-name "chassisMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
