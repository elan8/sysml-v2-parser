# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_05-Unification of Expression and Constraint Definition"))
~~~
# SOURCE
~~~sysml
package '15_05-Unification of Expression and Constraint Definition' {
	private import '15_03-Value Expression'::*;
	private import ControlFunctions::forAll;
	private import SI::*;
	
	constraint def DiscBrakeConstraint {
		in wheelAssy : WheelAssy[4];
		
		wheelAssy->forAll {in ref w: WheelAssy; 
			2 * w.discBrakeAssy.radius < w.wheel.outerDiameter
		}
	}
	
	constraint def DiscBrakeFitConstraint_Alt {
		in discBrakeAssy : DiscBrakeAssy[1];
		in wheel : Wheel[1];	
			
		2 * discBrakeAssy.radius < wheel.outerDiameter
	}
	
	part def Vehicle_2 {
		attribute mass : MassValue[1] = 1200 [kg];
		attribute length : LengthValue[1] = 4.82 [m];
		
		part wheelAssy : WheelAssy[4];
		
		constraint discBrakeConstraint : DiscBrakeConstraint {
			doc
			/*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
			in wheelAssy = Vehicle_2::wheelAssy;
		}
	}
	
	part def WheelAssy {
		part wheel : Wheel[1];
		part discBrakeAssy : DiscBrakeAssy[1];
		
		assert constraint discBrakeFitConstraint_Alt: DiscBrakeFitConstraint_Alt {
			doc
			/*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */
		
			in discBrakeAssy = WheelAssy::discBrakeAssy;
			in wheel = WheelAssy::wheel;
		}
	}
	
	part def DiscBrakeAssy {
		attribute radius : LengthValue[1] = 95 [mm];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_05_unification_of_expression_and_constraint_definition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_05-Unification of Expression and Constraint Definition' {
    private import '15_03-Value Expression'::*;
    private import ControlFunctions::forAll;
    private import SI::*;
    constraint def DiscBrakeConstraint {
        in wheelAssy : WheelAssy[4];
        wheelAssy->forAll { in ref w : WheelAssy; 2 * w.discBrakeAssy.radius < w.wheel.outerDiameter };
    }
    constraint def DiscBrakeFitConstraint_Alt {
        in discBrakeAssy : DiscBrakeAssy[1];
        in wheel : Wheel[1];
        2 * discBrakeAssy.radius < wheel.outerDiameter;
    }
    part def Vehicle_2 {
        attribute mass : MassValue[1] = 1200 [kg];
        attribute length : LengthValue[1] = 4.82 [m];
        part wheelAssy : WheelAssy[4];
        constraint discBrakeConstraint : DiscBrakeConstraint {
            doc
            /*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
            in wheelAssy = Vehicle_2::wheelAssy;
        }
    }
    part def WheelAssy {
        part wheel : Wheel[1];
        part discBrakeAssy : DiscBrakeAssy[1];
        assert constraint discBrakeFitConstraint_Alt : DiscBrakeFitConstraint_Alt {
            doc
            /*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */
            in discBrakeAssy = WheelAssy::discBrakeAssy;
            in wheel = WheelAssy::wheel;
        }
    }
    part def DiscBrakeAssy {
        attribute radius : LengthValue[1] = 95 [mm];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 86) (line 2) (column 17) (len 24)) (segments (segment 0 (token "'15_03-Value Expression'") (name "15_03-Value Expression") (separator none) (span (offset 86) (line 2) (column 17) (len 24)))))
    (reference r1 (scope relative) (span (offset 131) (line 3) (column 17) (len 24)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 131) (line 3) (column 17) (len 16))) (segment 1 (token "forAll") (name "forAll") (separator colon-colon) (span (offset 149) (line 3) (column 35) (len 6)))))
    (reference r2 (scope relative) (span (offset 173) (line 4) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 173) (line 4) (column 17) (len 2)))))
    (reference r3 (scope relative) (span (offset 567) (line 22) (column 20) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 567) (line 22) (column 20) (len 9)))))
    (reference r4 (scope relative) (span (offset 614) (line 23) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 614) (line 23) (column 22) (len 11)))))
    (reference r5 (scope relative) (span (offset 1411) (line 54) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 1411) (line 54) (column 22) (len 11)))))
  )
  (root (package (name "15_05-Unification of Expression and Constraint Definition") (body brace (import (target (span (span (offset 86) (line 2) (column 17) (len 27))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 110) (line 2) (column 41) (len 3))) (separator (span (offset 110) (line 2) (column 41) (len 2))) (marker (span (offset 112) (line 2) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 131) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 173) (line 4) (column 17) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 175) (line 4) (column 19) (len 3))) (separator (span (offset 175) (line 4) (column 19) (len 2))) (marker (span (offset 177) (line 4) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (constraint-def) (constraint-def) (part-def (name "Vehicle_2") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 582) (line 22) (column 35) (len 9)) (literal-with-unit (value (expression (span (offset 582) (line 22) (column 35) (len 4)) (integer 1200))) (unit (expression (span (offset 588) (line 22) (column 41) (len 2)) (bracket (expression (span (offset 588) (line 22) (column 41) (len 2)) (unit "kg")))))))))) (body semicolon)) (attribute-usage (declaration-name "length") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 631) (line 23) (column 39) (len 8)) (literal-with-unit (value (expression (span (offset 631) (line 23) (column 39) (len 4)) (real "4.82"))) (unit (expression (span (offset 637) (line 23) (column 45) (len 1)) (bracket (expression (span (offset 637) (line 23) (column 45) (len 1)) (unit "m")))))))))) (body semicolon)) (part-usage) (constraint-usage))) (part-def (name "WheelAssy") (body brace (part-usage) (part-usage) (assert-constraint))) (part-def (name "DiscBrakeAssy") (body brace (attribute-usage (declaration-name "radius") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1428) (line 54) (column 39) (len 7)) (literal-with-unit (value (expression (span (offset 1428) (line 54) (column 39) (len 2)) (integer 95))) (unit (expression (span (offset 1432) (line 54) (column 43) (len 2)) (bracket (expression (span (offset 1432) (line 54) (column 43) (len 2)) (unit "mm")))))))))) (body semicolon)))))))
)
~~~
