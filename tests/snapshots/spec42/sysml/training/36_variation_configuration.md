# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 36 (Variability): Variation Configuration"))
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_configuration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Variation Configuration' {
    private import 'Variation Usages'::*;
    part vehicle4Cyl :> vehicleFamily {
        part  :>> engine = engine::'4cylEngine';
        part  :>> transmission = transmission::manualTransmission;
    }
    part vehicle6Cyl :> vehicleFamily {
        part  :>> engine = engine::'6cylEngine';
        part  :>> transmission = transmission::manualTransmission;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 18)) (segments (segment 0 (token "'Variation Usages'") (name "Variation Usages") (separator none) (span (offset 52) (line 2) (column 17) (len 18)))))
  )
  (root (package (name "Variation Configuration") (body (import (target (span (span (offset 52) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 70) (line 2) (column 35) (len 3))) (separator (span (offset 70) (line 2) (column 35) (len 2))) (marker (span (offset 72) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage))))
)
~~~
