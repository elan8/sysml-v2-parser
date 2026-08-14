# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): CalculationTest"))
~~~
# SOURCE
~~~sysml
package CalculationExample {
	private import ISQ::*;
	private import NumericalFunctions::*;
	
	part def VehiclePart {
		attribute m : MassValue;
	}
	
	part def Vehicle :> VehiclePart;
	
	part vehicle : Vehicle {		
		part eng : VehiclePart;		
		part trans : VehiclePart;
		attribute ::> m = ms.totalMass;
	}
	
	calc def MassSum {
		in partMasses : MassValue[0..*];
		return totalMass : MassValue = sum(partMasses);
	}
	
	calc ms: MassSum {
		in partMasses = (vehicle.eng.m, vehicle.trans.m);
		return totalMass;
	}
	
	part vehicles[*] = (vehicle, vehicle);
	attribute masses1[*] = (vehicles as VehiclePart).m;
	attribute masses2[*] = (vehicles as vehicle).m;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CalculationExample {
    private import ISQ::*;
    private import NumericalFunctions::*;
    part def VehiclePart {
        attribute m : MassValue;
    }
    part def Vehicle :> VehiclePart;
    part vehicle : Vehicle {
        part eng : VehiclePart;
        part trans : VehiclePart;
        attribute ::> m = ms.totalMass;
    }
    calc def MassSum {
        in partMasses : MassValue[0..*];
        return totalMass : MassValue = sum(partMasses);
    }
    calc def ms : MassSum {
        in partMasses = (vehicle.eng.m, vehicle.trans.m);
        return totalMass;
    }
    part vehicles[*] = (vehicle, vehicle);
    attribute def masses1 = (vehicles as VehiclePart).m;
    attribute def masses2 = (vehicles as vehicle).m;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 45) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 69) (line 3) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 69) (line 3) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 134) (line 6) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 134) (line 6) (column 17) (len 9)))))
  )
  (root (package (name "CalculationExample") (body (import (target (span (span (offset 45) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 48) (line 2) (column 20) (len 3))) (separator (span (offset 48) (line 2) (column 20) (len 2))) (marker (span (offset 50) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 69) (line 3) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 87) (line 3) (column 35) (len 3))) (separator (span (offset 87) (line 3) (column 35) (len 2))) (marker (span (offset 89) (line 3) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "VehiclePart") (body (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle") (body semicolon)) (part-usage) (calc-def) (calc-def) (part-usage) (attribute-def) (attribute-def))))
)
~~~
