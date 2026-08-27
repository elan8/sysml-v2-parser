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
    /*
	 * Real world user models would use quantity and vector types
	 * from library models. They are included here for the purpose
	 * of showing how such attribute defs can be defined.
	 */
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
    attribute frenchTireInfo : TireInfo {
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0['in'];
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
    (reference r2 (scope relative) (span (offset 361) (line 12) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 361) (line 12) (column 22) (len 4)))))
    (reference r3 (scope relative) (span (offset 391) (line 13) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 391) (line 13) (column 22) (len 4)))))
    (reference r4 (scope relative) (span (offset 421) (line 14) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 421) (line 14) (column 22) (len 4)))))
    (reference r5 (scope relative) (span (offset 474) (line 17) (column 34) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 474) (line 17) (column 34) (len 4)))))
    (reference r6 (scope relative) (span (offset 539) (line 20) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 539) (line 20) (column 30) (len 6)))))
    (reference r7 (scope relative) (span (offset 578) (line 21) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 578) (line 21) (column 32) (len 11)))))
    (reference r8 (scope relative) (span (offset 616) (line 22) (column 26) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 616) (line 22) (column 26) (len 7)))))
    (reference r9 (scope relative) (span (offset 654) (line 23) (column 30) (len 14)) (segments (segment 0 (token "PositionVector") (name "PositionVector") (separator none) (span (offset 654) (line 23) (column 30) (len 14)))))
  )
  (root (package (name "15_12-Compound Value Type") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 22))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 116) (line 5) (column 4) (len 185)) (normalized "Real world user models would use quantity and vector types\nfrom library models. They are included here for the purpose\nof showing how such attribute defs can be defined.\n"))) (attribute-def (declaration-name "PositionVector") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "y") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "z") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "LengthValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "TireInfo") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "manufacturer") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "hubDiameter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "width") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "placement") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage))))
)
~~~
