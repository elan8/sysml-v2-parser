# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Mass Roll-up): MassConstraintExample"))
~~~
# SOURCE
~~~sysml
package MassConstraintExample {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine {
		attribute m :> mass;
	}
	
	part def Transmission {
		attribute m :> mass;
	}
	
	part def Vehicle1 {
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	part def Vehicle2 {
		assert constraint { m == eng.m + trans.m }
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassConstraint3 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
			
		totalMass == sum(partMasses)
	}
	
	part def Vehicle3 {
		assert constraint massConstraint : MassConstraint3 {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng {
			attribute m : MassValue;
		}
		
		part trans {
			attribute m : MassValue;
		}
	}
	
	constraint def MassConstraint4 {
		in totalMass : MassValue;
		in partMasses : MassValue[0..*];
	}
	
	constraint mc : MassConstraint4 {
		in totalMass : MassValue; 
		in partMasses : MassValue[0..*];
		
		totalMass == sum(partMasses)
	}
	
	part def Vehicle4 {
		assert mc {
			in totalMass = m;
			in partMasses = (eng.m, trans.m);
		}
		
		attribute m : MassValue;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}
	
	constraint def MassLimit {
		in mass : MassValue; 
		in maxMass : MassValue;
			
		mass <= maxMass
	}
	
	part def Vehicle5 {
		assert constraint ml : MassLimit {
			in mass = m;
			in maxMass = 2500 [kg];
		}
		
		attribute m : MassValue = eng.m + trans.m;
		
		part eng : Engine {
			attribute :>> m : MassValue;
		}
		
		part trans : Transmission {
			attribute :>> m : MassValue;
		}
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_constraint_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassConstraintExample {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;
    part def Engine {
        attribute m :> mass;
    }
    part def Transmission {
        attribute m :> mass;
    }
    part def Vehicle1 {
        attribute m : MassValue = eng.m + trans.m;
        part eng : Engine {
            attribute :>> m : MassValue;
        }
        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }
    part def Vehicle2 {
        assert constraint {
            m == eng.m + trans.m;
        }
        attribute m : MassValue;
        part eng : Engine {
            attribute :>> m : MassValue;
        }
        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }
    constraint def MassConstraint3 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];
        totalMass == sum(partMasses);
    }
    part def Vehicle3 {
        assert constraint massConstraint : MassConstraint3 {
            in totalMass = m;
            in partMasses = (eng.m, trans.m);
        }
        attribute m : MassValue;
        part eng {
            attribute m : MassValue;
        }
        part trans {
            attribute m : MassValue;
        }
    }
    constraint def MassConstraint4 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];
    }
    constraint mc : MassConstraint4 {
        in totalMass : MassValue;
        in partMasses : MassValue[0..*];
        totalMass == sum(partMasses);
    }
    part def Vehicle4 {
        assert mc {
            in totalMass = m;
            in partMasses = (eng.m, trans.m);
        }
        attribute m : MassValue;
        part eng : Engine {
            attribute :>> m : MassValue;
        }
        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }
    constraint def MassLimit {
        in mass : MassValue;
        in maxMass : MassValue;
        mass <= maxMass;
    }
    part def Vehicle5 {
        assert constraint ml : MassLimit {
            in mass = m;
            in maxMass = 2500 [kg];
        }
        attribute m : MassValue = eng.m + trans.m;
        part eng : Engine {
            attribute :>> m : MassValue;
        }
        part trans : Transmission {
            attribute :>> m : MassValue;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 48) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 72) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 72) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 95) (line 4) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 95) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 156) (line 7) (column 18) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 156) (line 7) (column 18) (len 4)))))
    (reference r4 (scope relative) (span (offset 209) (line 11) (column 18) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 209) (line 11) (column 18) (len 4)))))
    (reference r5 (scope relative) (span (offset 257) (line 15) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 257) (line 15) (column 17) (len 9)))))
    (reference r6 (scope relative) (span (offset 269) (line 15) (column 29) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 269) (line 15) (column 29) (len 3)))))
    (reference r7 (scope relative) (span (offset 273) (line 15) (column 33) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 273) (line 15) (column 33) (len 1)))))
    (reference r8 (scope relative) (span (offset 277) (line 15) (column 37) (len 5)) (segments (segment 0 (token "trans") (name "trans") (separator none) (span (offset 277) (line 15) (column 37) (len 5)))))
    (reference r9 (scope relative) (span (offset 283) (line 15) (column 43) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 283) (line 15) (column 43) (len 1)))))
    (reference r10 (scope relative) (span (offset 302) (line 17) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 302) (line 17) (column 14) (len 6)))))
    (reference r11 (scope relative) (span (offset 332) (line 18) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 332) (line 18) (column 22) (len 9)))))
    (reference r12 (scope relative) (span (offset 328) (line 18) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 328) (line 18) (column 18) (len 1)))))
    (reference r13 (scope relative) (span (offset 365) (line 21) (column 16) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 365) (line 21) (column 16) (len 12)))))
    (reference r14 (scope relative) (span (offset 401) (line 22) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 401) (line 22) (column 22) (len 9)))))
    (reference r15 (scope relative) (span (offset 397) (line 22) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 397) (line 22) (column 18) (len 1)))))
    (reference r16 (scope relative) (span (offset 506) (line 29) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 506) (line 29) (column 17) (len 9)))))
    (reference r17 (scope relative) (span (offset 533) (line 31) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 533) (line 31) (column 14) (len 6)))))
    (reference r18 (scope relative) (span (offset 563) (line 32) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 563) (line 32) (column 22) (len 9)))))
    (reference r19 (scope relative) (span (offset 559) (line 32) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 559) (line 32) (column 18) (len 1)))))
    (reference r20 (scope relative) (span (offset 596) (line 35) (column 16) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 596) (line 35) (column 16) (len 12)))))
    (reference r21 (scope relative) (span (offset 632) (line 36) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 632) (line 36) (column 22) (len 9)))))
    (reference r22 (scope relative) (span (offset 628) (line 36) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 628) (line 36) (column 18) (len 1)))))
    (reference r23 (scope relative) (span (offset 756) (line 44) (column 3) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 756) (line 44) (column 3) (len 9)))))
    (reference r24 (scope relative) (span (offset 769) (line 44) (column 16) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 769) (line 44) (column 16) (len 3)))))
    (reference r25 (scope relative) (span (offset 773) (line 44) (column 20) (len 10)) (segments (segment 0 (token "partMasses") (name "partMasses") (separator none) (span (offset 773) (line 44) (column 20) (len 10)))))
    (reference r26 (scope relative) (span (offset 947) (line 53) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 947) (line 53) (column 17) (len 9)))))
    (reference r27 (scope relative) (span (offset 991) (line 56) (column 18) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 991) (line 56) (column 18) (len 9)))))
    (reference r28 (scope relative) (span (offset 1041) (line 60) (column 18) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1041) (line 60) (column 18) (len 9)))))
    (reference r29 (scope relative) (span (offset 1180) (line 69) (column 18) (len 15)) (segments (segment 0 (token "MassConstraint4") (name "MassConstraint4") (separator none) (span (offset 1180) (line 69) (column 18) (len 15)))))
    (reference r30 (scope relative) (span (offset 1267) (line 73) (column 3) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 1267) (line 73) (column 3) (len 9)))))
    (reference r31 (scope relative) (span (offset 1280) (line 73) (column 16) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 1280) (line 73) (column 16) (len 3)))))
    (reference r32 (scope relative) (span (offset 1284) (line 73) (column 20) (len 10)) (segments (segment 0 (token "partMasses") (name "partMasses") (separator none) (span (offset 1284) (line 73) (column 20) (len 10)))))
    (reference r33 (scope relative) (span (offset 1417) (line 82) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1417) (line 82) (column 17) (len 9)))))
    (reference r34 (scope relative) (span (offset 1444) (line 84) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1444) (line 84) (column 14) (len 6)))))
    (reference r35 (scope relative) (span (offset 1474) (line 85) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1474) (line 85) (column 22) (len 9)))))
    (reference r36 (scope relative) (span (offset 1470) (line 85) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1470) (line 85) (column 18) (len 1)))))
    (reference r37 (scope relative) (span (offset 1507) (line 88) (column 16) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1507) (line 88) (column 16) (len 12)))))
    (reference r38 (scope relative) (span (offset 1543) (line 89) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1543) (line 89) (column 22) (len 9)))))
    (reference r39 (scope relative) (span (offset 1539) (line 89) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1539) (line 89) (column 18) (len 1)))))
    (reference r40 (scope relative) (span (offset 1647) (line 97) (column 3) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 1647) (line 97) (column 3) (len 4)))))
    (reference r41 (scope relative) (span (offset 1655) (line 97) (column 11) (len 7)) (segments (segment 0 (token "maxMass") (name "maxMass") (separator none) (span (offset 1655) (line 97) (column 11) (len 7)))))
    (reference r42 (scope relative) (span (offset 1792) (line 106) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1792) (line 106) (column 17) (len 9)))))
    (reference r43 (scope relative) (span (offset 1804) (line 106) (column 29) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 1804) (line 106) (column 29) (len 3)))))
    (reference r44 (scope relative) (span (offset 1808) (line 106) (column 33) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1808) (line 106) (column 33) (len 1)))))
    (reference r45 (scope relative) (span (offset 1812) (line 106) (column 37) (len 5)) (segments (segment 0 (token "trans") (name "trans") (separator none) (span (offset 1812) (line 106) (column 37) (len 5)))))
    (reference r46 (scope relative) (span (offset 1818) (line 106) (column 43) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1818) (line 106) (column 43) (len 1)))))
    (reference r47 (scope relative) (span (offset 1837) (line 108) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1837) (line 108) (column 14) (len 6)))))
    (reference r48 (scope relative) (span (offset 1867) (line 109) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1867) (line 109) (column 22) (len 9)))))
    (reference r49 (scope relative) (span (offset 1863) (line 109) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1863) (line 109) (column 18) (len 1)))))
    (reference r50 (scope relative) (span (offset 1900) (line 112) (column 16) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1900) (line 112) (column 16) (len 12)))))
    (reference r51 (scope relative) (span (offset 1936) (line 113) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1936) (line 113) (column 22) (len 9)))))
    (reference r52 (scope relative) (span (offset 1932) (line 113) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1932) (line 113) (column 18) (len 1)))))
  )
  (root (package (name "MassConstraintExample") (body brace (import (target (span (span (offset 48) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 51) (line 2) (column 20) (len 3))) (separator (span (offset 51) (line 2) (column 20) (len 2))) (marker (span (offset 53) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 72) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 74) (line 3) (column 19) (len 3))) (separator (span (offset 74) (line 3) (column 19) (len 2))) (marker (span (offset 76) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 95) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 113) (line 4) (column 35) (len 3))) (separator (span (offset 113) (line 4) (column 35) (len 2))) (marker (span (offset 115) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Transmission") (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle1") (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 269) (line 15) (column 29) (len 15)) (binary (operator "+") (left (expression (span (offset 269) (line 15) (column 29) (len 5)) (member-access (base (expression (span (offset 269) (line 15) (column 29) (len 3)) (ref r6))) (separator dot) (member (ref r7))))) (right (expression (span (offset 277) (line 15) (column 37) (len 7)) (member-access (base (expression (span (offset 277) (line 15) (column 37) (len 5)) (ref r8))) (separator dot) (member (ref r9)))))))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (part-def (name "Vehicle2") (body brace (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (constraint-def (name "MassConstraint3") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 756) (line 44) (column 3) (len 28)) (binary (operator "==") (left (expression (span (offset 756) (line 44) (column 3) (len 9)) (ref r23))) (right (expression (span (offset 769) (line 44) (column 16) (len 15)) (invocation (callee (expression (span (offset 769) (line 44) (column 16) (len 3)) (ref r24))) (arguments (argument (parameter none) (value (expression (span (offset 773) (line 44) (column 20) (len 10)) (ref r25)))))))))))) (part-def (name "Vehicle3") (body brace (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (constraint-def (name "MassConstraint4") (specializes none) (body brace (in-out-declaration) (in-out-declaration))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "mc") (short-name none) (type (ref r29)) (multiplicity none) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 1267) (line 73) (column 3) (len 28)) (binary (operator "==") (left (expression (span (offset 1267) (line 73) (column 3) (len 9)) (ref r30))) (right (expression (span (offset 1280) (line 73) (column 16) (len 15)) (invocation (callee (expression (span (offset 1280) (line 73) (column 16) (len 3)) (ref r31))) (arguments (argument (parameter none) (value (expression (span (offset 1284) (line 73) (column 20) (len 10)) (ref r32)))))))))))) (part-def (name "Vehicle4") (body brace (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r36)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r39)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (constraint-def (name "MassLimit") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 1647) (line 97) (column 3) (len 15)) (binary (operator "<=") (left (expression (span (offset 1647) (line 97) (column 3) (len 4)) (ref r40))) (right (expression (span (offset 1655) (line 97) (column 11) (len 7)) (ref r41))))))) (part-def (name "Vehicle5") (body brace (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1804) (line 106) (column 29) (len 15)) (binary (operator "+") (left (expression (span (offset 1804) (line 106) (column 29) (len 5)) (member-access (base (expression (span (offset 1804) (line 106) (column 29) (len 3)) (ref r43))) (separator dot) (member (ref r44))))) (right (expression (span (offset 1812) (line 106) (column 37) (len 7)) (member-access (base (expression (span (offset 1812) (line 106) (column 37) (len 5)) (ref r45))) (separator dot) (member (ref r46)))))))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r47)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r48)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r50)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
