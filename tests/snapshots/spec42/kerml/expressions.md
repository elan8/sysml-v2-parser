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
    class C {
        var count : ScalarValues::Integer := 0;
    }
    feature obj1 : C;
    feature obj2 : C;
    test1 = obj1 === obj2;
    test2 = x !== obj2;
    class L {
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
    (reference r3 (scope relative) (span (offset 1304) (line 47) (column 3) (len 8)) (segments (segment 0 (token "partMass") (name "partMass") (separator none) (span (offset 1304) (line 47) (column 3) (len 8)))))
    (reference r4 (scope relative) (span (offset 1316) (line 47) (column 15) (len 8)) (segments (segment 0 (token "subparts") (name "subparts") (separator none) (span (offset 1316) (line 47) (column 15) (len 8)))))
    (reference r5 (scope relative) (span (offset 1341) (line 47) (column 40) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 1341) (line 47) (column 40) (len 9)))))
    (reference r6 (scope relative) (span (offset 1351) (line 47) (column 50) (len 8)) (segments (segment 0 (token "partMass") (name "partMass") (separator none) (span (offset 1351) (line 47) (column 50) (len 8)))))
    (reference r7 (scope relative) (span (offset 1361) (line 47) (column 60) (len 8)) (segments (segment 0 (token "subparts") (name "subparts") (separator none) (span (offset 1361) (line 47) (column 60) (len 8)))))
    (reference r8 (scope relative) (span (offset 1380) (line 47) (column 79) (len 3)) (segments (segment 0 (token "'+'") (name "+") (separator none) (span (offset 1380) (line 47) (column 79) (len 3)))))
  )
  (root (package (name "Expressions") (body brace (import (target (span (span (offset 38) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 32) (len 3))) (separator (span (offset 53) (line 2) (column 32) (len 2))) (marker (span (offset 55) (line 2) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 115) (line 4) (column 17) (len 19))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 4) (column 33) (len 3))) (separator (span (offset 131) (line 4) (column 33) (len 2))) (marker (span (offset 133) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (kerml-classifier (keyword behavior) (abstract false) (name "w") (specializes none) (body brace (in-out-declaration) (kerml-feature))) (default-reference-usage) (kerml-classifier (keyword function) (abstract false) (name "TotalMass") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (expression (span (offset 1304) (line 47) (column 3) (len 87)) (binary (operator "+") (left (expression (span (offset 1304) (line 47) (column 3) (len 8)) (ref r3))) (right (expression (span (offset 1315) (line 47) (column 14) (len 76)) (sequence (sequence-list (element first (expression (span (offset 1316) (line 47) (column 15) (len 74)) (binary (operator "??") (left (expression (span (offset 1316) (line 47) (column 15) (len 67)) (collection-op (operator "reduce") (base (expression (span (offset 1316) (line 47) (column 15) (len 55)) (collection-op (operator "collect") (base (expression (span (offset 1316) (line 47) (column 15) (len 8)) (ref r4))) (arguments) (brace-body (body (span (offset 1334) (line 47) (column 33) (len 37)) (open-brace (span (offset 1334) (line 47) (column 33) (len 1))) (parameters (parameter (span (offset 1335) (line 47) (column 34) (len 5)) (direction in (span (offset 1335) (line 47) (column 34) (len 2))) (reference-keyword none) (name "p" (span (offset 1338) (line 47) (column 37) (len 1))) (typing none) (terminator (semicolon (span (offset 1339) (line 47) (column 38) (len 1)))))) (result (expression (span (offset 1341) (line 47) (column 40) (len 29)) (invocation (callee (expression (span (offset 1341) (line 47) (column 40) (len 9)) (ref r5))) (arguments (argument (parameter none) (value (expression (span (offset 1351) (line 47) (column 50) (len 8)) (ref r6)))) (argument (parameter none) (value (expression (span (offset 1361) (line 47) (column 60) (len 8)) (ref r7)))))))) (close-brace (span (offset 1370) (line 47) (column 69) (len 1)))))))) (arguments (argument (parameter none) (value (expression (span (offset 1380) (line 47) (column 79) (len 3)) (ref r8))))) (brace-body none)))) (right (expression (span (offset 1387) (line 47) (column 86) (len 3)) (real "0.0"))))))))))))))) (kerml-feature (name "totalMass") (body brace (in-out-declaration) (in-out-declaration))) (kerml-feature (name "f") (body brace (kerml-feature))) (default-reference-usage) (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (body brace (kerml-feature))) (kerml-feature (name "obj1") (body semicolon)) (kerml-feature (name "obj2") (body semicolon)) (default-reference-usage) (default-reference-usage) (kerml-classifier (keyword class) (abstract false) (name "L") (specializes none) (body brace (kerml-feature) (kerml-feature))) (kerml-feature (name "l") (body semicolon)) (kerml-feature (name "w1") (body semicolon)))))
)
~~~
