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
            attribute  :>> m : MassValue;
        }
        part trans : Transmission {
            attribute  :>> m : MassValue;
        }
    }
    part def Vehicle2 {
        assert constraint  {
            m == eng.m + trans.m;
        }
        attribute m : MassValue;
        part eng : Engine {
            attribute  :>> m : MassValue;
        }
        part trans : Transmission {
            attribute  :>> m : MassValue;
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
            attribute  :>> m : MassValue;
        }
        part trans : Transmission {
            attribute  :>> m : MassValue;
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
            attribute  :>> m : MassValue;
        }
        part trans : Transmission {
            attribute  :>> m : MassValue;
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
    (reference r10 (scope relative) (span (offset 506) (line 29) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 506) (line 29) (column 17) (len 9)))))
    (reference r11 (scope relative) (span (offset 947) (line 53) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 947) (line 53) (column 17) (len 9)))))
    (reference r12 (scope relative) (span (offset 1417) (line 82) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1417) (line 82) (column 17) (len 9)))))
    (reference r13 (scope relative) (span (offset 1792) (line 106) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1792) (line 106) (column 17) (len 9)))))
    (reference r14 (scope relative) (span (offset 1804) (line 106) (column 29) (len 3)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 1804) (line 106) (column 29) (len 3)))))
    (reference r15 (scope relative) (span (offset 1808) (line 106) (column 33) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1808) (line 106) (column 33) (len 1)))))
    (reference r16 (scope relative) (span (offset 1812) (line 106) (column 37) (len 5)) (segments (segment 0 (token "trans") (name "trans") (separator none) (span (offset 1812) (line 106) (column 37) (len 5)))))
    (reference r17 (scope relative) (span (offset 1818) (line 106) (column 43) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1818) (line 106) (column 43) (len 1)))))
  )
  (root (package (name "MassConstraintExample") (body (import (target (span (span (offset 48) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 51) (line 2) (column 20) (len 3))) (separator (span (offset 51) (line 2) (column 20) (len 2))) (marker (span (offset 53) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 72) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 74) (line 3) (column 19) (len 3))) (separator (span (offset 74) (line 3) (column 19) (len 2))) (marker (span (offset 76) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 95) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 113) (line 4) (column 35) (len 3))) (separator (span (offset 113) (line 4) (column 35) (len 2))) (marker (span (offset 115) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body (attribute-usage (declaration-name "m") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Transmission") (body (attribute-usage (declaration-name "m") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle1") (body (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 269) (line 15) (column 29) (len 15)) (binary (operator "+") (left (expression (span (offset 269) (line 15) (column 29) (len 5)) (member-access (base (expression (span (offset 269) (line 15) (column 29) (len 3)) (ref r6))) (separator dot) (member (ref r7))))) (right (expression (span (offset 277) (line 15) (column 37) (len 7)) (member-access (base (expression (span (offset 277) (line 15) (column 37) (len 5)) (ref r8))) (separator dot) (member (ref r9)))))))))) (body semicolon)) (part-usage) (part-usage))) (part-def (name "Vehicle2") (body (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage))) (constraint-def) (part-def (name "Vehicle3") (body (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage))) (constraint-def) (constraint-usage) (part-def (name "Vehicle4") (body (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage))) (constraint-def) (part-def (name "Vehicle5") (body (assert-constraint) (attribute-usage (declaration-name "m") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1804) (line 106) (column 29) (len 15)) (binary (operator "+") (left (expression (span (offset 1804) (line 106) (column 29) (len 5)) (member-access (base (expression (span (offset 1804) (line 106) (column 29) (len 3)) (ref r14))) (separator dot) (member (ref r15))))) (right (expression (span (offset 1812) (line 106) (column 37) (len 7)) (member-access (base (expression (span (offset 1812) (line 106) (column 37) (len 5)) (ref r16))) (separator dot) (member (ref r17)))))))))) (body semicolon)) (part-usage) (part-usage))))))
)
~~~
