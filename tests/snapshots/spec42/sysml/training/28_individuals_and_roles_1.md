# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 28 (Individuals): Individuals and Roles-1"))
~~~
# SOURCE
~~~sysml
package 'Individuals and Roles' {
	private import 'Part Definition Example'::*;
	
	part def Wheel;
	
	individual part def Vehicle_1 :> Vehicle {
		part leftFrontWheel : Wheel;
		part rightFrontWheel : Wheel;
	}
	
	individual part def Wheel_1 :> Wheel;
	
	individual part vehicle_1 : Vehicle_1 {
		snapshot part vehicle_1_t0 {
			snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
		}
		
		then snapshot part vehicle_1_t1 {
			snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_roles_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Individuals and Roles' {
    private import 'Part Definition Example'::*;
    part def Wheel;
    individual part def Vehicle_1 :> Vehicle {
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }
    individual part def Wheel_1 :> Wheel;
    individual part vehicle_1 : Vehicle_1 {
        snapshot part vehicle_1_t0 {
            snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
        }
        then snapshot part vehicle_1_t1 {
            snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'Part Definition Example'") (name "Part Definition Example") (separator none) (span (offset 50) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 169) (line 7) (column 25) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 169) (line 7) (column 25) (len 5)))))
    (reference r2 (scope relative) (span (offset 201) (line 8) (column 26) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 201) (line 8) (column 26) (len 5)))))
    (reference r3 (scope relative) (span (offset 283) (line 13) (column 30) (len 9)) (segments (segment 0 (token "Vehicle_1") (name "Vehicle_1") (separator none) (span (offset 283) (line 13) (column 30) (len 9)))))
  )
  (root (package (name "Individuals and Roles") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 2) (column 42) (len 3))) (separator (span (offset 75) (line 2) (column 42) (len 2))) (marker (span (offset 77) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Wheel") (body semicolon)) (part-def (name "Vehicle_1") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftFrontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightFrontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Wheel_1") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "vehicle_1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "vehicle_1_t0") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "leftFrontWheel_t0") (short-name none) (target none) (body semicolon)))) (part-usage (then true) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "vehicle_1_t1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "rightFrontWheel_t1") (short-name none) (target none) (body semicolon)))))))))
)
~~~
