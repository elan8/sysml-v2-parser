# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 36 (Variability): Variation Usages"))
~~~
# SOURCE
~~~sysml
package 'Variation Usages' {
	private import 'Variation Definitions'::*;
	
	part def Vehicle;
	part def Transmission;
	part manualTransmission;
	part automaticTransmission;
	
	abstract part vehicleFamily : Vehicle {
		part engine : EngineChoices[1];
		
		variation part transmission : Transmission[1] {
			variant manualTransmission;
			variant automaticTransmission;
		}
		
		assert constraint {
			(engine == engine::'4cylEngine' and
			 transmission == transmission::manualTransmission) xor
			(engine == engine::'6cylEngine' and 
			 transmission == transmission::automaticTransmission)
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_usages.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Variation Usages' {
    private import 'Variation Definitions'::*;
    part def Vehicle;
    part def Transmission;
    part manualTransmission;
    part automaticTransmission;
    abstract part vehicleFamily : Vehicle {
        part engine : EngineChoices[1];
        variation part transmission : Transmission[1] {
            variant manualTransmission;
            variant automaticTransmission;
        }
        assert constraint {
            (engine == engine::'4cylEngine' && transmission == transmission::manualTransmission) xor (engine == engine::'6cylEngine' && transmission == transmission::automaticTransmission);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 23)) (segments (segment 0 (token "'Variation Definitions'") (name "Variation Definitions") (separator none) (span (offset 45) (line 2) (column 17) (len 23)))))
    (reference r1 (scope relative) (span (offset 206) (line 9) (column 32) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 206) (line 9) (column 32) (len 7)))))
  )
  (root (package (name "Variation Usages") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 26))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 68) (line 2) (column 40) (len 3))) (separator (span (offset 68) (line 2) (column 40) (len 2))) (marker (span (offset 70) (line 2) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-usage (declaration-name "manualTransmission") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "automaticTransmission") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "vehicleFamily") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (assert-constraint))))))
)
~~~
