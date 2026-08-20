# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_04-Logical Expressions"))
~~~
# SOURCE
~~~sysml
package '15_04-Logical Expressions' {
	private import ScalarValues::*;
	
	part def Engine;
	part def '4CylEngine' :> Engine;
	part def '6CylEngine' :> Engine;
	
	part def Transmission;
	part def ManualTransmission :> Transmission;
	part def AutomaticTransmission :> Transmission;
	
	part def Vehicle {
		attribute isHighPerformance: Boolean;
		
		part engine: Engine[1];
		part transmission: Transmission[1];
		
		assert constraint {
			if isHighPerformance? engine istype '6CylEngine'
			else engine istype '4CylEngine'
		}
		
		assert constraint {
			(engine istype '4CylEngine' and 
			 transmission istype ManualTransmission) xor
			(engine istype '6CylEngine' and
			 transmission istype AutomaticTransmission)
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_04_logical_expressions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_04-Logical Expressions' {
    private import ScalarValues::*;
    part def Engine;
    part def '4CylEngine' :> Engine;
    part def '6CylEngine' :> Engine;
    part def Transmission;
    part def ManualTransmission :> Transmission;
    part def AutomaticTransmission :> Transmission;
    part def Vehicle {
        attribute isHighPerformance : Boolean;
        part engine : Engine[1];
        part transmission : Transmission[1];
        assert constraint {
            if isHighPerformance ? engine istype '6CylEngine' else engine istype '4CylEngine';
        }
        assert constraint {
            (engine istype '4CylEngine' && transmission istype ManualTransmission) xor (engine istype '6CylEngine' && transmission istype AutomaticTransmission);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 333) (line 13) (column 32) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 333) (line 13) (column 32) (len 7)))))
    (reference r2 (scope relative) (span (offset 360) (line 15) (column 16) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 360) (line 15) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 392) (line 16) (column 22) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 392) (line 16) (column 22) (len 12)))))
  )
  (root (package (name "15_04-Logical Expressions") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (modifiers) (body semicolon)) (part-def (name "4CylEngine") (modifiers) (body semicolon)) (part-def (name "6CylEngine") (modifiers) (body semicolon)) (part-def (name "Transmission") (modifiers) (body semicolon)) (part-def (name "ManualTransmission") (modifiers) (body semicolon)) (part-def (name "AutomaticTransmission") (modifiers) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "isHighPerformance") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 367) (line 15) (column 23) (len 1)) (integer 1))) (upper (expression (span (offset 367) (line 15) (column 23) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 405) (line 16) (column 35) (len 1)) (integer 1))) (upper (expression (span (offset 405) (line 16) (column 35) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (assert-constraint) (assert-constraint))))))
)
~~~
