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
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 297) (line 14) (column 3) (len 199)) (message "unexpected token in part usage body"))
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
    (reference r1 (scope relative) (span (offset 283) (line 13) (column 30) (len 9)) (segments (segment 0 (token "Vehicle_1") (name "Vehicle_1") (separator none) (span (offset 283) (line 13) (column 30) (len 9)))))
  )
  (root (package (name "Individuals and Roles") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 2) (column 42) (len 3))) (separator (span (offset 75) (line 2) (column 42) (len 2))) (marker (span (offset 77) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Wheel") (body semicolon)) (part-def (name "Vehicle_1") (body brace (part-usage) (part-usage))) (part-def (name "Wheel_1") (body semicolon)) (part-usage (declaration-name "vehicle_1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (malformed (code "recovered_part_usage_body_element") (found "snapshot part vehicle_1_t0 {") (span (offset 297) (line 14) (column 3) (len 199))))))))
)
~~~
