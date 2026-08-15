# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_12-Compound Value Type"))
~~~
# SOURCE
~~~sysml
package '15_12-Compound Value Type' {
	private import ScalarValues::*;
	private import USCustomaryUnits::'in';
	
	/*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */

    attribute def PositionVector {
        attribute x: Real[1];
        attribute y: Real[1];
        attribute z: Real[1];
    }
    
    attribute def LengthValue :> Real;

    attribute def TireInfo {
    	attribute manufacturer: String;
        attribute hubDiameter: LengthValue;
        attribute width: Integer;
        attribute placement: PositionVector[0..1];
    }
    
    attribute frenchTireInfo: TireInfo {
    	attribute :>> manufacturer = "Michelin";
    	attribute :>> hubDiameter = 18.0['in'];
    	attribute :>> width = 245;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_12_compound_value_type.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_12-Compound Value Type' {
    private import ScalarValues::*;
    private import USCustomaryUnits::'in';
    attribute def PositionVector {
        attribute x : Real[1];
        attribute y : Real[1];
        attribute z : Real[1];
    }
    attribute def LengthValue :> Real;
    attribute def TireInfo {
        attribute manufacturer : String;
        attribute hubDiameter : LengthValue;
        attribute width : Integer;
        attribute placement : PositionVector[0..1];
    }
    attribute def frenchTireInfo : TireInfo {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0 ['in'];
        attribute :>> width = 245;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 22)) (segments (segment 0 (token "USCustomaryUnits") (name "USCustomaryUnits") (separator none) (span (offset 87) (line 3) (column 17) (len 16))) (segment 1 (token "'in'") (name "in") (separator colon-colon) (span (offset 105) (line 3) (column 35) (len 4)))))
  )
  (root (package (name "15_12-Compound Value Type") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 22))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def))))
)
~~~
