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
        attribute :>> manufacturer = "Michelin";
        attribute :>> hubDiameter = 18.0;
        attribute :>> width = 245;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 57) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 109) (line 4) (column 34) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 109) (line 4) (column 34) (len 4)))))
    (reference r2 (scope relative) (span (offset 335) (line 14) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 335) (line 14) (column 30) (len 6)))))
    (reference r3 (scope relative) (span (offset 374) (line 15) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 374) (line 15) (column 32) (len 11)))))
    (reference r4 (scope relative) (span (offset 412) (line 16) (column 26) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 412) (line 16) (column 26) (len 7)))))
    (reference r5 (scope relative) (span (offset 453) (line 19) (column 22) (len 4)) (segments (segment 0 (token "Tire") (name "Tire") (separator none) (span (offset 453) (line 19) (column 22) (len 4)))))
    (reference r6 (scope relative) (span (offset 479) (line 20) (column 20) (len 12)) (segments (segment 0 (token "manufacturer") (name "manufacturer") (separator none) (span (offset 479) (line 20) (column 20) (len 12)))))
    (reference r7 (scope relative) (span (offset 525) (line 21) (column 20) (len 11)) (segments (segment 0 (token "hubDiameter") (name "hubDiameter") (separator none) (span (offset 525) (line 21) (column 20) (len 11)))))
    (reference r8 (scope relative) (span (offset 564) (line 22) (column 20) (len 5)) (segments (segment 0 (token "width") (name "width") (separator none) (span (offset 564) (line 22) (column 20) (len 5)))))
  )
  (root (package (name "15_02-Basic Value Properties") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 69) (line 2) (column 29) (len 3))) (separator (span (offset 69) (line 2) (column 29) (len 2))) (marker (span (offset 71) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "LengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (part-def (name "Tire") (body brace (attribute-usage (declaration-name "manufacturer") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "hubDiameter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "width") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frenchTire") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 494) (line 20) (column 35) (len 10)) (string "Michelin"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 539) (line 21) (column 34) (len 4)) (real "18.0"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 572) (line 22) (column 28) (len 3)) (integer 245))))) (body semicolon)))))))
)
~~~
