# META
~~~sexpr
(snapshot (type semantic) (description "KerML Simple Tests: Expressions"))
~~~
# SOURCE
~~~sysml
package Expressions {
	private import ScalarFunctions::*;
	private import BaseFunctions::ToString;
	private import ControlFunctions::*;
	
	a: Integer;
	aa : Boolean;
	x = ToString(a * a + 3 == 4);
	y = NumericalFunctions::'+'(1,2);
	z : Boolean = aa & true xor zz | false implies z;
	zz : Boolean = aa and true xor aa or false implies z;
	grp = -x + x * y * y + a ** 3 ^ 4;
	
	b = if x > y? x-y else y-x;
	c = x->collect {in xx; xx + 1}; 
	c1 = x.{in xx; xx + 1}; 
	d = x->select {in xx; xx != null};
	d1 = x.?{in xx; xx != null};
	e = x->reduce {in s; in t; s + t}->reduce '+';
	
	behavior w { inout v : Integer;
	    step : ControlPerformances::LoopPerformance {
    		in expr whileTest {v > 3}
    		in step body {
    			step decrement {
    				out v_decr : Integer = v - 1;			
    			}
    			succession decrement then update;
    			step update : FeatureReferencingPerformances::FeatureWritePerformance {
    				in onOccurrence = w::self {
    					feature redefines startingAt : w {
    						inout feature redefines accessedFeature redefines v;
    					}
    				}
    				inout replacementValues = decrement.v_decr;
    			}
    		}
		}
	}
	
	xx = if x == 1 and y == 2? a
	     else if x == 2? b
	     else if x == 3? c
	     else 0;
    
    function TotalMass { in partMass; in subparts;
		partMass + (subparts->collect {in p; totalMass(partMass, subparts)}->reduce '+' ?? 0.0)
	}
	
	expr totalMass: TotalMass { in mass; in sub; }
	
	feature f {
		expr s { in x; return : Boolean; }
	}
	
	bb : Boolean = f.s(1);
	
	class C {
		var count : ScalarValues::Integer := 0;
	}
	
	feature obj1 : C;
	feature obj2 : C;
	
	test1 = obj1 === obj2;
	test2 = x !== obj2;
	
	class L {
		feature c : C[*];
		feature count : ScalarValues::Integer =  c#(1).count;
	}
	
	feature l = new L();
	feature w1 = w(xx);
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expressions.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 957) (line 31) (column 10) (len 117)) (message "unrecognized declaration `feature` in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Expressions {
    private import ScalarFunctions::*;
    private import BaseFunctions::ToString;
    private import ControlFunctions::*;
    a : Integer;
    aa : Boolean;
    x = ToString(a * a + 3 == 4);
    y = NumericalFunctions::'+'(1, 2);
    z : Boolean = aa & true xor zz | false implies z;
    zz : Boolean = aa && true xor aa || false implies z;
    grp = -x + x * y * y + a ** 3 ^ 4;
    b = if x > y ? x - y else y - x;
    c = x->collect { in xx; xx + 1 };
    c1 = x.{ in xx; xx + 1 };
    d = x->select { in xx; xx != null };
    d1 = x.?{ in xx; xx != null };
    e = x->reduce { in s; in t; s + t }->reduce('+');
    behavior w {
        inout v : Integer;
        step : ControlPerformances::LoopPerformance {
            in expr whileTest {
                v > 3;
            }
            in step body {
                step decrement {
                    out v_decr : Integer = v - 1;
                }
                succession decrement then update;
                step update : FeatureReferencingPerformances::FeatureWritePerformance {
                    in onOccurrence = w::self {
                        feature redefines startingAt : w {
    						inout feature redefines accessedFeature redefines v;
    					}
                    }
                    inout replacementValues = decrement.v_decr;
                }
            }
        }
    }
    xx = if x == 1 && y == 2 ? a else if x == 2 ? b else if x == 3 ? c else 0;
    function TotalMass {
        in partMass;
        in subparts;
        partMass + (subparts->collect { in p; totalMass(partMass, subparts) }->reduce('+') ?? 0.0);
    }
    expr totalMass : TotalMass {
        in mass;
        in sub;
    }
    feature f {
        expr s {
            in x;
            return : Boolean;
        }
    }
    bb : Boolean = f.s(1);
    class def C {
        var count : ScalarValues::Integer := 0;
    }
    feature obj1 : C;
    feature obj2 : C;
    test1 = obj1 === obj2;
    test2 = x !== obj2;
    class def L {
        feature c : C[*];
        feature count : ScalarValues::Integer = c#(1).count;
    }
    feature l = new L();
    feature w1 = w(xx);
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 17) (len 15)) (segments (segment 0 (token "ScalarFunctions") (name "ScalarFunctions") (separator none) (span (offset 38) (line 2) (column 17) (len 15)))))
    (reference r1 (scope relative) (span (offset 74) (line 3) (column 17) (len 23)) (segments (segment 0 (token "BaseFunctions") (name "BaseFunctions") (separator none) (span (offset 74) (line 3) (column 17) (len 13))) (segment 1 (token "ToString") (name "ToString") (separator colon-colon) (span (offset 89) (line 3) (column 32) (len 8)))))
    (reference r2 (scope relative) (span (offset 115) (line 4) (column 17) (len 16)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 115) (line 4) (column 17) (len 16)))))
  )
  (root (package (name "Expressions") (body brace (import (target (span (span (offset 38) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 32) (len 3))) (separator (span (offset 53) (line 2) (column 32) (len 2))) (marker (span (offset 55) (line 2) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 115) (line 4) (column 17) (len 19))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 4) (column 33) (len 3))) (separator (span (offset 131) (line 4) (column 33) (len 2))) (marker (span (offset 133) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (kerml-classifier (keyword behavior) (abstract false) (name "w") (specializes none)) (default-reference-usage) (kerml-classifier (keyword function) (abstract false) (name "TotalMass") (specializes none)) (kerml-feature (name "totalMass")) (kerml-feature (name "f")) (default-reference-usage) (class-def) (kerml-feature (name "obj1")) (kerml-feature (name "obj2")) (default-reference-usage) (default-reference-usage) (class-def) (kerml-feature (name "l")) (kerml-feature (name "w1")))))
)
~~~
