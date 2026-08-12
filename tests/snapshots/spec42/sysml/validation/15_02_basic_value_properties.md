# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_02-Basic Value Properties"))
~~~
# SOURCE
~~~sysml
package '15_02-Basic Value Properties' {
	private import ScalarValues::*;
	
    attribute def LengthValue :> Real {
		doc
		/*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
	}

    part def Tire {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
    }
    
    part frenchTire: Tire {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0;
    	attribute :>> width = 245;
    }
    
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_02_basic_value_properties.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_02-Basic Value Properties' {
    private import ScalarValues::*;
    attribute def LengthValue :> Real {
        doc
        /*
		 * Real world user models would use a quantity type
		 * from the library model. A attribute def is defined
		 * here to show that it is possible.
		 */
    }
    part def Tire {
        attribute manufacturer : String;
        attribute hubDiameter : LengthValue;
        attribute width : Integer;
    }
    part frenchTire : Tire {
        attribute  :>> manufacturer = "Michelin";
        attribute  :>> hubDiameter = 18.0;
        attribute  :>> width = 245;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 57) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 335) (line 14) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 335) (line 14) (column 30) (len 6)))))
    (reference r2 (scope relative) (span (offset 374) (line 15) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 374) (line 15) (column 32) (len 11)))))
    (reference r3 (scope relative) (span (offset 412) (line 16) (column 26) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 412) (line 16) (column 26) (len 7)))))
  )
  (root (package (name "15_02-Basic Value Properties") (body (import (target (span (span (offset 57) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 69) (line 2) (column 29) (len 3))) (separator (span (offset 69) (line 2) (column 29) (len 2))) (marker (span (offset 71) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (part-def (name "Tire") (body (attribute-usage (declaration-name "manufacturer") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "hubDiameter") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "width") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage))))
)
~~~
