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
    (reference r3 (scope relative) (span (offset 142) (line 6) (column 5) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 142) (line 6) (column 5) (len 7)))))
    (reference r4 (scope relative) (span (offset 157) (line 7) (column 7) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 157) (line 7) (column 7) (len 7)))))
    (reference r5 (scope relative) (span (offset 171) (line 8) (column 6) (len 8)) (segments (segment 0 (token "ToString") (name "ToString") (separator none) (span (offset 171) (line 8) (column 6) (len 8)))))
    (reference r6 (scope relative) (span (offset 180) (line 8) (column 15) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 180) (line 8) (column 15) (len 1)))))
    (reference r7 (scope relative) (span (offset 184) (line 8) (column 19) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 184) (line 8) (column 19) (len 1)))))
    (reference r8 (scope relative) (span (offset 202) (line 9) (column 6) (len 23)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 202) (line 9) (column 6) (len 18))) (segment 1 (token "'+'") (name "+") (separator colon-colon) (span (offset 222) (line 9) (column 26) (len 3)))))
    (reference r9 (scope relative) (span (offset 237) (line 10) (column 6) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 237) (line 10) (column 6) (len 7)))))
    (reference r10 (scope relative) (span (offset 247) (line 10) (column 16) (len 2)) (segments (segment 0 (token "aa") (name "aa") (separator none) (span (offset 247) (line 10) (column 16) (len 2)))))
    (reference r11 (scope relative) (span (offset 261) (line 10) (column 30) (len 2)) (segments (segment 0 (token "zz") (name "zz") (separator none) (span (offset 261) (line 10) (column 30) (len 2)))))
    (reference r12 (scope relative) (span (offset 280) (line 10) (column 49) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 280) (line 10) (column 49) (len 1)))))
    (reference r13 (scope relative) (span (offset 289) (line 11) (column 7) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 289) (line 11) (column 7) (len 7)))))
    (reference r14 (scope relative) (span (offset 299) (line 11) (column 17) (len 2)) (segments (segment 0 (token "aa") (name "aa") (separator none) (span (offset 299) (line 11) (column 17) (len 2)))))
    (reference r15 (scope relative) (span (offset 315) (line 11) (column 33) (len 2)) (segments (segment 0 (token "aa") (name "aa") (separator none) (span (offset 315) (line 11) (column 33) (len 2)))))
    (reference r16 (scope relative) (span (offset 335) (line 11) (column 53) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 335) (line 11) (column 53) (len 1)))))
    (reference r17 (scope relative) (span (offset 346) (line 12) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 346) (line 12) (column 9) (len 1)))))
    (reference r18 (scope relative) (span (offset 350) (line 12) (column 13) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 350) (line 12) (column 13) (len 1)))))
    (reference r19 (scope relative) (span (offset 354) (line 12) (column 17) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 354) (line 12) (column 17) (len 1)))))
    (reference r20 (scope relative) (span (offset 358) (line 12) (column 21) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 358) (line 12) (column 21) (len 1)))))
    (reference r21 (scope relative) (span (offset 362) (line 12) (column 25) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 362) (line 12) (column 25) (len 1)))))
    (reference r22 (scope relative) (span (offset 384) (line 14) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 384) (line 14) (column 9) (len 1)))))
    (reference r23 (scope relative) (span (offset 388) (line 14) (column 13) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 388) (line 14) (column 13) (len 1)))))
    (reference r24 (scope relative) (span (offset 391) (line 14) (column 16) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 391) (line 14) (column 16) (len 1)))))
    (reference r25 (scope relative) (span (offset 393) (line 14) (column 18) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 393) (line 14) (column 18) (len 1)))))
    (reference r26 (scope relative) (span (offset 400) (line 14) (column 25) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 400) (line 14) (column 25) (len 1)))))
    (reference r27 (scope relative) (span (offset 402) (line 14) (column 27) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 402) (line 14) (column 27) (len 1)))))
    (reference r28 (scope relative) (span (offset 410) (line 15) (column 6) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 410) (line 15) (column 6) (len 1)))))
    (reference r29 (scope relative) (span (offset 429) (line 15) (column 25) (len 2)) (segments (segment 0 (token "xx") (name "xx") (separator none) (span (offset 429) (line 15) (column 25) (len 2)))))
    (reference r30 (scope relative) (span (offset 445) (line 16) (column 7) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 445) (line 16) (column 7) (len 1)))))
    (reference r31 (scope relative) (span (offset 455) (line 16) (column 17) (len 2)) (segments (segment 0 (token "xx") (name "xx") (separator none) (span (offset 455) (line 16) (column 17) (len 2)))))
    (reference r32 (scope relative) (span (offset 470) (line 17) (column 6) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 470) (line 17) (column 6) (len 1)))))
    (reference r33 (scope relative) (span (offset 488) (line 17) (column 24) (len 2)) (segments (segment 0 (token "xx") (name "xx") (separator none) (span (offset 488) (line 17) (column 24) (len 2)))))
    (reference r34 (scope relative) (span (offset 507) (line 18) (column 7) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 507) (line 18) (column 7) (len 1)))))
    (reference r35 (scope relative) (span (offset 518) (line 18) (column 18) (len 2)) (segments (segment 0 (token "xx") (name "xx") (separator none) (span (offset 518) (line 18) (column 18) (len 2)))))
    (reference r36 (scope relative) (span (offset 536) (line 19) (column 6) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 536) (line 19) (column 6) (len 1)))))
    (reference r37 (scope relative) (span (offset 559) (line 19) (column 29) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 559) (line 19) (column 29) (len 1)))))
    (reference r38 (scope relative) (span (offset 563) (line 19) (column 33) (len 1)) (segments (segment 0 (token "t") (name "t") (separator none) (span (offset 563) (line 19) (column 33) (len 1)))))
    (reference r39 (scope relative) (span (offset 574) (line 19) (column 44) (len 3)) (segments (segment 0 (token "'+'") (name "+") (separator none) (span (offset 574) (line 19) (column 44) (len 3)))))
    (reference r40 (scope relative) (span (offset 1163) (line 41) (column 10) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 1163) (line 41) (column 10) (len 1)))))
    (reference r41 (scope relative) (span (offset 1174) (line 41) (column 21) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 1174) (line 41) (column 21) (len 1)))))
    (reference r42 (scope relative) (span (offset 1182) (line 41) (column 29) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 1182) (line 41) (column 29) (len 1)))))
    (reference r43 (scope relative) (span (offset 1198) (line 42) (column 15) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 1198) (line 42) (column 15) (len 1)))))
    (reference r44 (scope relative) (span (offset 1206) (line 42) (column 23) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 1206) (line 42) (column 23) (len 1)))))
    (reference r45 (scope relative) (span (offset 1222) (line 43) (column 15) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 1222) (line 43) (column 15) (len 1)))))
    (reference r46 (scope relative) (span (offset 1230) (line 43) (column 23) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 1230) (line 43) (column 23) (len 1)))))
    (reference r47 (scope relative) (span (offset 1304) (line 47) (column 3) (len 8)) (segments (segment 0 (token "partMass") (name "partMass") (separator none) (span (offset 1304) (line 47) (column 3) (len 8)))))
    (reference r48 (scope relative) (span (offset 1316) (line 47) (column 15) (len 8)) (segments (segment 0 (token "subparts") (name "subparts") (separator none) (span (offset 1316) (line 47) (column 15) (len 8)))))
    (reference r49 (scope relative) (span (offset 1341) (line 47) (column 40) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 1341) (line 47) (column 40) (len 9)))))
    (reference r50 (scope relative) (span (offset 1351) (line 47) (column 50) (len 8)) (segments (segment 0 (token "partMass") (name "partMass") (separator none) (span (offset 1351) (line 47) (column 50) (len 8)))))
    (reference r51 (scope relative) (span (offset 1361) (line 47) (column 60) (len 8)) (segments (segment 0 (token "subparts") (name "subparts") (separator none) (span (offset 1361) (line 47) (column 60) (len 8)))))
    (reference r52 (scope relative) (span (offset 1380) (line 47) (column 79) (len 3)) (segments (segment 0 (token "'+'") (name "+") (separator none) (span (offset 1380) (line 47) (column 79) (len 3)))))
    (reference r53 (scope relative) (span (offset 1508) (line 56) (column 7) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1508) (line 56) (column 7) (len 7)))))
    (reference r54 (scope relative) (span (offset 1518) (line 56) (column 17) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 1518) (line 56) (column 17) (len 1)))))
    (reference r55 (scope relative) (span (offset 1520) (line 56) (column 19) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 1520) (line 56) (column 19) (len 1)))))
    (reference r56 (scope relative) (span (offset 1635) (line 65) (column 10) (len 4)) (segments (segment 0 (token "obj1") (name "obj1") (separator none) (span (offset 1635) (line 65) (column 10) (len 4)))))
    (reference r57 (scope relative) (span (offset 1644) (line 65) (column 19) (len 4)) (segments (segment 0 (token "obj2") (name "obj2") (separator none) (span (offset 1644) (line 65) (column 19) (len 4)))))
    (reference r58 (scope relative) (span (offset 1659) (line 66) (column 10) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 1659) (line 66) (column 10) (len 1)))))
    (reference r59 (scope relative) (span (offset 1665) (line 66) (column 16) (len 4)) (segments (segment 0 (token "obj2") (name "obj2") (separator none) (span (offset 1665) (line 66) (column 16) (len 4)))))
  )
  (root (package (name "Expressions") (body brace (import (target (span (span (offset 38) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 32) (len 3))) (separator (span (offset 53) (line 2) (column 32) (len 2))) (marker (span (offset 55) (line 2) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 74) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 115) (line 4) (column 17) (len 19))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 4) (column 33) (len 3))) (separator (span (offset 131) (line 4) (column 33) (len 2))) (marker (span (offset 133) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "aa") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "x") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 171) (line 8) (column 6) (len 24)) (invocation (callee (expression (span (offset 171) (line 8) (column 6) (len 8)) (ref r5))) (arguments (argument (parameter none) (value (expression (span (offset 180) (line 8) (column 15) (len 14)) (binary (operator "==") (left (expression (span (offset 180) (line 8) (column 15) (len 9)) (binary (operator "+") (left (expression (span (offset 180) (line 8) (column 15) (len 5)) (binary (operator "*") (left (expression (span (offset 180) (line 8) (column 15) (len 1)) (ref r6))) (right (expression (span (offset 184) (line 8) (column 19) (len 1)) (ref r7)))))) (right (expression (span (offset 188) (line 8) (column 23) (len 1)) (integer 3)))))) (right (expression (span (offset 193) (line 8) (column 28) (len 1)) (integer 4))))))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "y") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 202) (line 9) (column 6) (len 28)) (invocation (callee (expression (span (offset 202) (line 9) (column 6) (len 23)) (ref r8))) (arguments (argument (parameter none) (value (expression (span (offset 226) (line 9) (column 30) (len 1)) (integer 1)))) (argument (parameter none) (value (expression (span (offset 228) (line 9) (column 32) (len 1)) (integer 2)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "z") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 247) (line 10) (column 16) (len 34)) (binary (operator "implies") (left (expression (span (offset 247) (line 10) (column 16) (len 24)) (binary (operator "xor") (left (expression (span (offset 247) (line 10) (column 16) (len 9)) (binary (operator "&") (left (expression (span (offset 247) (line 10) (column 16) (len 2)) (ref r10))) (right (expression (span (offset 252) (line 10) (column 21) (len 4)) (boolean true)))))) (right (expression (span (offset 261) (line 10) (column 30) (len 10)) (binary (operator "|") (left (expression (span (offset 261) (line 10) (column 30) (len 2)) (ref r11))) (right (expression (span (offset 266) (line 10) (column 35) (len 5)) (boolean false))))))))) (right (expression (span (offset 280) (line 10) (column 49) (len 1)) (ref r12)))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "zz") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 299) (line 11) (column 17) (len 37)) (binary (operator "implies") (left (expression (span (offset 299) (line 11) (column 17) (len 27)) (binary (operator "||") (left (expression (span (offset 299) (line 11) (column 17) (len 18)) (binary (operator "xor") (left (expression (span (offset 299) (line 11) (column 17) (len 11)) (binary (operator "&&") (left (expression (span (offset 299) (line 11) (column 17) (len 2)) (ref r14))) (right (expression (span (offset 306) (line 11) (column 24) (len 4)) (boolean true)))))) (right (expression (span (offset 315) (line 11) (column 33) (len 2)) (ref r15)))))) (right (expression (span (offset 321) (line 11) (column 39) (len 5)) (boolean false)))))) (right (expression (span (offset 335) (line 11) (column 53) (len 1)) (ref r16)))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "grp") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 345) (line 12) (column 8) (len 27)) (binary (operator "+") (left (expression (span (offset 345) (line 12) (column 8) (len 14)) (binary (operator "+") (left (expression (span (offset 345) (line 12) (column 8) (len 2)) (unary (operator "-") (operand (expression (span (offset 346) (line 12) (column 9) (len 1)) (ref r17)))))) (right (expression (span (offset 350) (line 12) (column 13) (len 9)) (binary (operator "*") (left (expression (span (offset 350) (line 12) (column 13) (len 5)) (binary (operator "*") (left (expression (span (offset 350) (line 12) (column 13) (len 1)) (ref r18))) (right (expression (span (offset 354) (line 12) (column 17) (len 1)) (ref r19)))))) (right (expression (span (offset 358) (line 12) (column 21) (len 1)) (ref r20))))))))) (right (expression (span (offset 362) (line 12) (column 25) (len 10)) (binary (operator "^") (left (expression (span (offset 362) (line 12) (column 25) (len 6)) (binary (operator "**") (left (expression (span (offset 362) (line 12) (column 25) (len 1)) (ref r21))) (right (expression (span (offset 367) (line 12) (column 30) (len 1)) (integer 3)))))) (right (expression (span (offset 371) (line 12) (column 34) (len 1)) (integer 4))))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 381) (line 14) (column 6) (len 22)) (conditional (test (expression (span (offset 384) (line 14) (column 9) (len 5)) (binary (operator ">") (left (expression (span (offset 384) (line 14) (column 9) (len 1)) (ref r22))) (right (expression (span (offset 388) (line 14) (column 13) (len 1)) (ref r23)))))) (then (expression (span (offset 391) (line 14) (column 16) (len 3)) (binary (operator "-") (left (expression (span (offset 391) (line 14) (column 16) (len 1)) (ref r24))) (right (expression (span (offset 393) (line 14) (column 18) (len 1)) (ref r25)))))) (else (expression (span (offset 400) (line 14) (column 25) (len 3)) (binary (operator "-") (left (expression (span (offset 400) (line 14) (column 25) (len 1)) (ref r26))) (right (expression (span (offset 402) (line 14) (column 27) (len 1)) (ref r27))))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 410) (line 15) (column 6) (len 26)) (collection-op (operator "collect") (base (expression (span (offset 410) (line 15) (column 6) (len 1)) (ref r28))) (arguments) (brace-body (body (span (offset 421) (line 15) (column 17) (len 15)) (open-brace (span (offset 421) (line 15) (column 17) (len 1))) (parameters (parameter (span (offset 422) (line 15) (column 18) (len 6)) (direction in (span (offset 422) (line 15) (column 18) (len 2))) (reference-keyword none) (name "xx" (span (offset 425) (line 15) (column 21) (len 2))) (typing none) (terminator (semicolon (span (offset 427) (line 15) (column 23) (len 1)))))) (result (expression (span (offset 429) (line 15) (column 25) (len 6)) (binary (operator "+") (left (expression (span (offset 429) (line 15) (column 25) (len 2)) (ref r29))) (right (expression (span (offset 434) (line 15) (column 30) (len 1)) (integer 1)))))) (close-brace (span (offset 435) (line 15) (column 31) (len 1)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 445) (line 16) (column 7) (len 17)) (collection-op (operator "collect") (base (expression (span (offset 445) (line 16) (column 7) (len 1)) (ref r30))) (arguments) (brace-body (body (span (offset 447) (line 16) (column 9) (len 15)) (open-brace (span (offset 447) (line 16) (column 9) (len 1))) (parameters (parameter (span (offset 448) (line 16) (column 10) (len 6)) (direction in (span (offset 448) (line 16) (column 10) (len 2))) (reference-keyword none) (name "xx" (span (offset 451) (line 16) (column 13) (len 2))) (typing none) (terminator (semicolon (span (offset 453) (line 16) (column 15) (len 1)))))) (result (expression (span (offset 455) (line 16) (column 17) (len 6)) (binary (operator "+") (left (expression (span (offset 455) (line 16) (column 17) (len 2)) (ref r31))) (right (expression (span (offset 460) (line 16) (column 22) (len 1)) (integer 1)))))) (close-brace (span (offset 461) (line 16) (column 23) (len 1)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "d") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 470) (line 17) (column 6) (len 29)) (collection-op (operator "select") (base (expression (span (offset 470) (line 17) (column 6) (len 1)) (ref r32))) (arguments) (brace-body (body (span (offset 480) (line 17) (column 16) (len 19)) (open-brace (span (offset 480) (line 17) (column 16) (len 1))) (parameters (parameter (span (offset 481) (line 17) (column 17) (len 6)) (direction in (span (offset 481) (line 17) (column 17) (len 2))) (reference-keyword none) (name "xx" (span (offset 484) (line 17) (column 20) (len 2))) (typing none) (terminator (semicolon (span (offset 486) (line 17) (column 22) (len 1)))))) (result (expression (span (offset 488) (line 17) (column 24) (len 10)) (binary (operator "!=") (left (expression (span (offset 488) (line 17) (column 24) (len 2)) (ref r33))) (right (expression (span (offset 494) (line 17) (column 30) (len 4)) (null)))))) (close-brace (span (offset 498) (line 17) (column 34) (len 1)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "d1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 507) (line 18) (column 7) (len 22)) (collection-op (operator "select") (base (expression (span (offset 507) (line 18) (column 7) (len 1)) (ref r34))) (arguments) (brace-body (body (span (offset 510) (line 18) (column 10) (len 19)) (open-brace (span (offset 510) (line 18) (column 10) (len 1))) (parameters (parameter (span (offset 511) (line 18) (column 11) (len 6)) (direction in (span (offset 511) (line 18) (column 11) (len 2))) (reference-keyword none) (name "xx" (span (offset 514) (line 18) (column 14) (len 2))) (typing none) (terminator (semicolon (span (offset 516) (line 18) (column 16) (len 1)))))) (result (expression (span (offset 518) (line 18) (column 18) (len 10)) (binary (operator "!=") (left (expression (span (offset 518) (line 18) (column 18) (len 2)) (ref r35))) (right (expression (span (offset 524) (line 18) (column 24) (len 4)) (null)))))) (close-brace (span (offset 528) (line 18) (column 28) (len 1)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "e") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 536) (line 19) (column 6) (len 41)) (collection-op (operator "reduce") (base (expression (span (offset 536) (line 19) (column 6) (len 29)) (collection-op (operator "reduce") (base (expression (span (offset 536) (line 19) (column 6) (len 1)) (ref r36))) (arguments) (brace-body (body (span (offset 546) (line 19) (column 16) (len 19)) (open-brace (span (offset 546) (line 19) (column 16) (len 1))) (parameters (parameter (span (offset 547) (line 19) (column 17) (len 5)) (direction in (span (offset 547) (line 19) (column 17) (len 2))) (reference-keyword none) (name "s" (span (offset 550) (line 19) (column 20) (len 1))) (typing none) (terminator (semicolon (span (offset 551) (line 19) (column 21) (len 1))))) (parameter (span (offset 553) (line 19) (column 23) (len 5)) (direction in (span (offset 553) (line 19) (column 23) (len 2))) (reference-keyword none) (name "t" (span (offset 556) (line 19) (column 26) (len 1))) (typing none) (terminator (semicolon (span (offset 557) (line 19) (column 27) (len 1)))))) (result (expression (span (offset 559) (line 19) (column 29) (len 5)) (binary (operator "+") (left (expression (span (offset 559) (line 19) (column 29) (len 1)) (ref r37))) (right (expression (span (offset 563) (line 19) (column 33) (len 1)) (ref r38)))))) (close-brace (span (offset 564) (line 19) (column 34) (len 1)))))))) (arguments (argument (parameter none) (value (expression (span (offset 574) (line 19) (column 44) (len 3)) (ref r39))))) (brace-body none)))))) (body semicolon)) (kerml-classifier (keyword behavior) (abstract false) (name "w") (specializes none) (body brace (in-out-declaration) (kerml-feature))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "xx") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1160) (line 41) (column 7) (len 84)) (conditional (test (expression (span (offset 1163) (line 41) (column 10) (len 17)) (binary (operator "&&") (left (expression (span (offset 1163) (line 41) (column 10) (len 6)) (binary (operator "==") (left (expression (span (offset 1163) (line 41) (column 10) (len 1)) (ref r40))) (right (expression (span (offset 1168) (line 41) (column 15) (len 1)) (integer 1)))))) (right (expression (span (offset 1174) (line 41) (column 21) (len 6)) (binary (operator "==") (left (expression (span (offset 1174) (line 41) (column 21) (len 1)) (ref r41))) (right (expression (span (offset 1179) (line 41) (column 26) (len 1)) (integer 2))))))))) (then (expression (span (offset 1182) (line 41) (column 29) (len 1)) (ref r42))) (else (expression (span (offset 1195) (line 42) (column 12) (len 49)) (conditional (test (expression (span (offset 1198) (line 42) (column 15) (len 6)) (binary (operator "==") (left (expression (span (offset 1198) (line 42) (column 15) (len 1)) (ref r43))) (right (expression (span (offset 1203) (line 42) (column 20) (len 1)) (integer 2)))))) (then (expression (span (offset 1206) (line 42) (column 23) (len 1)) (ref r44))) (else (expression (span (offset 1219) (line 43) (column 12) (len 25)) (conditional (test (expression (span (offset 1222) (line 43) (column 15) (len 6)) (binary (operator "==") (left (expression (span (offset 1222) (line 43) (column 15) (len 1)) (ref r45))) (right (expression (span (offset 1227) (line 43) (column 20) (len 1)) (integer 3)))))) (then (expression (span (offset 1230) (line 43) (column 23) (len 1)) (ref r46))) (else (expression (span (offset 1243) (line 44) (column 12) (len 1)) (integer 0)))))))))))))) (body semicolon)) (kerml-classifier (keyword function) (abstract false) (name "TotalMass") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (expression (span (offset 1304) (line 47) (column 3) (len 87)) (binary (operator "+") (left (expression (span (offset 1304) (line 47) (column 3) (len 8)) (ref r47))) (right (expression (span (offset 1315) (line 47) (column 14) (len 76)) (sequence (sequence-list (element first (expression (span (offset 1316) (line 47) (column 15) (len 74)) (binary (operator "??") (left (expression (span (offset 1316) (line 47) (column 15) (len 67)) (collection-op (operator "reduce") (base (expression (span (offset 1316) (line 47) (column 15) (len 55)) (collection-op (operator "collect") (base (expression (span (offset 1316) (line 47) (column 15) (len 8)) (ref r48))) (arguments) (brace-body (body (span (offset 1334) (line 47) (column 33) (len 37)) (open-brace (span (offset 1334) (line 47) (column 33) (len 1))) (parameters (parameter (span (offset 1335) (line 47) (column 34) (len 5)) (direction in (span (offset 1335) (line 47) (column 34) (len 2))) (reference-keyword none) (name "p" (span (offset 1338) (line 47) (column 37) (len 1))) (typing none) (terminator (semicolon (span (offset 1339) (line 47) (column 38) (len 1)))))) (result (expression (span (offset 1341) (line 47) (column 40) (len 29)) (invocation (callee (expression (span (offset 1341) (line 47) (column 40) (len 9)) (ref r49))) (arguments (argument (parameter none) (value (expression (span (offset 1351) (line 47) (column 50) (len 8)) (ref r50)))) (argument (parameter none) (value (expression (span (offset 1361) (line 47) (column 60) (len 8)) (ref r51)))))))) (close-brace (span (offset 1370) (line 47) (column 69) (len 1)))))))) (arguments (argument (parameter none) (value (expression (span (offset 1380) (line 47) (column 79) (len 3)) (ref r52))))) (brace-body none)))) (right (expression (span (offset 1387) (line 47) (column 86) (len 3)) (real "0.0"))))))))))))))) (kerml-feature (name "totalMass") (body brace (in-out-declaration) (in-out-declaration))) (kerml-feature (name "f") (body brace (kerml-feature))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "bb") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1518) (line 56) (column 17) (len 6)) (invocation (callee (expression (span (offset 1518) (line 56) (column 17) (len 3)) (member-access (base (expression (span (offset 1518) (line 56) (column 17) (len 1)) (ref r54))) (separator dot) (member (ref r55))))) (arguments (argument (parameter none) (value (expression (span (offset 1522) (line 56) (column 21) (len 1)) (integer 1)))))))))) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (body brace (kerml-feature))) (kerml-feature (name "obj1") (body semicolon)) (kerml-feature (name "obj2") (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "test1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1635) (line 65) (column 10) (len 13)) (binary (operator "===") (left (expression (span (offset 1635) (line 65) (column 10) (len 4)) (ref r56))) (right (expression (span (offset 1644) (line 65) (column 19) (len 4)) (ref r57)))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "test2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1659) (line 66) (column 10) (len 10)) (binary (operator "!==") (left (expression (span (offset 1659) (line 66) (column 10) (len 1)) (ref r58))) (right (expression (span (offset 1665) (line 66) (column 16) (len 4)) (ref r59)))))))) (body semicolon)) (kerml-classifier (keyword class) (abstract false) (name "L") (specializes none) (body brace (kerml-feature) (kerml-feature))) (kerml-feature (name "l") (body semicolon)) (kerml-feature (name "w1") (body semicolon)))))
)
~~~
