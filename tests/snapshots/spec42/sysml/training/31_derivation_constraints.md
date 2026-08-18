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
        assert constraint {
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
    (reference r2 (scope relative) (span (offset 121) (line 5) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 121) (line 5) (column 18) (len 7)))))
    (reference r3 (scope relative) (span (offset 155) (line 6) (column 25) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 155) (line 6) (column 25) (len 9)))))
    (reference r4 (scope relative) (span (offset 273) (line 10) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 273) (line 10) (column 18) (len 7)))))
    (reference r5 (scope relative) (span (offset 307) (line 11) (column 25) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 307) (line 11) (column 25) (len 9)))))
    (reference r6 (scope relative) (span (offset 319) (line 11) (column 37) (len 11)) (segments (segment 0 (token "chassisMass") (name "chassisMass") (separator none) (span (offset 319) (line 11) (column 37) (len 11)))))
    (reference r7 (scope relative) (span (offset 333) (line 11) (column 51) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 333) (line 11) (column 51) (len 6)))))
    (reference r8 (scope relative) (span (offset 340) (line 11) (column 58) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 340) (line 11) (column 58) (len 4)))))
    (reference r9 (scope relative) (span (offset 347) (line 11) (column 65) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 347) (line 11) (column 65) (len 12)))))
    (reference r10 (scope relative) (span (offset 360) (line 11) (column 78) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 360) (line 11) (column 78) (len 4)))))
    (reference r11 (scope relative) (span (offset 535) (line 21) (column 3) (len 5)) (segments (segment 0 (token "force") (name "force") (separator none) (span (offset 535) (line 21) (column 3) (len 5)))))
    (reference r12 (scope relative) (span (offset 543) (line 21) (column 11) (len 6)) (segments (segment 0 (token "deltaT") (name "deltaT") (separator none) (span (offset 543) (line 21) (column 11) (len 6)))))
    (reference r13 (scope relative) (span (offset 553) (line 21) (column 21) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 553) (line 21) (column 21) (len 4)))))
    (reference r14 (scope relative) (span (offset 561) (line 21) (column 29) (len 10)) (segments (segment 0 (token "finalSpeed") (name "finalSpeed") (separator none) (span (offset 561) (line 21) (column 29) (len 10)))))
    (reference r15 (scope relative) (span (offset 574) (line 21) (column 42) (len 12)) (segments (segment 0 (token "initialSpeed") (name "initialSpeed") (separator none) (span (offset 574) (line 21) (column 42) (len 12)))))
    (reference r16 (scope relative) (span (offset 594) (line 22) (column 3) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 594) (line 22) (column 3) (len 4)))))
  )
  (root (package (name "Derivation Constraints") (body brace (import (target (span (span (offset 51) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 19) (len 3))) (separator (span (offset 53) (line 2) (column 19) (len 2))) (marker (span (offset 55) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 26))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 97) (line 3) (column 40) (len 3))) (separator (span (offset 97) (line 3) (column 40) (len 2))) (marker (span (offset 99) (line 3) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 319) (line 11) (column 37) (len 45)) (binary (operator "+") (left (expression (span (offset 319) (line 11) (column 37) (len 25)) (binary (operator "+") (left (expression (span (offset 319) (line 11) (column 37) (len 11)) (ref r6))) (right (expression (span (offset 333) (line 11) (column 51) (len 11)) (member-access (base (expression (span (offset 333) (line 11) (column 51) (len 6)) (ref r7))) (separator dot) (member (ref r8)))))))) (right (expression (span (offset 347) (line 11) (column 65) (len 17)) (member-access (base (expression (span (offset 347) (line 11) (column 65) (len 12)) (ref r9))) (separator dot) (member (ref r10)))))))))) (body semicolon)))) (constraint-def (name "Dynamics") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration) (expression (span (offset 535) (line 21) (column 3) (len 71)) (binary (operator "&&") (left (expression (span (offset 535) (line 21) (column 3) (len 52)) (binary (operator "==") (left (expression (span (offset 535) (line 21) (column 3) (len 14)) (binary (operator "*") (left (expression (span (offset 535) (line 21) (column 3) (len 5)) (ref r11))) (right (expression (span (offset 543) (line 21) (column 11) (len 6)) (ref r12)))))) (right (expression (span (offset 553) (line 21) (column 21) (len 34)) (binary (operator "*") (left (expression (span (offset 553) (line 21) (column 21) (len 4)) (ref r13))) (right (expression (span (offset 560) (line 21) (column 28) (len 27)) (parenthesized (expression (span (offset 561) (line 21) (column 29) (len 25)) (binary (operator "-") (left (expression (span (offset 561) (line 21) (column 29) (len 10)) (ref r14))) (right (expression (span (offset 574) (line 21) (column 42) (len 12)) (ref r15)))))))))))))) (right (expression (span (offset 594) (line 22) (column 3) (len 12)) (binary (operator ">") (left (expression (span (offset 594) (line 22) (column 3) (len 4)) (ref r16))) (right (expression (span (offset 601) (line 22) (column 10) (len 5)) (literal-with-unit (value (expression (span (offset 601) (line 22) (column 10) (len 1)) (integer 0))) (unit (expression (span (offset 603) (line 22) (column 12) (len 2)) (bracket (expression (span (offset 603) (line 22) (column 12) (len 2)) (unit "kg"))))))))))))))))))
)
~~~
