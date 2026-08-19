# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): ParameterTest"))
~~~
# SOURCE
~~~sysml
package ParameterTest {
	attribute def A {
		attribute x : ScalarValues::String;
		attribute y : A;
	}
	
	attribute a : A;
	
	calc def F { in p : A; in q : ScalarValues::Integer; return :  ScalarValues::Integer; }
	
	attribute f = F(a, 2);
	attribute g = F(q = 1, p = a);
	
	attribute b = new A(y=a, x=""); 
	attribute c = new A("test2");
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parameter_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ParameterTest {
    attribute def A {
        attribute x : ScalarValues::String;
        attribute y : A;
    }
    attribute def a : A;
    calc def F {
        in p : A;
        in q : ScalarValues::Integer;
        return : ScalarValues::Integer;
    }
    attribute def f = F(a, 2);
    attribute def g = F(q = 1, p = a);
    attribute def b = new A(y = a, x = "");
    attribute def c = new A("test2");
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 59) (line 3) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 59) (line 3) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 73) (line 3) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 97) (line 4) (column 17) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 97) (line 4) (column 17) (len 1)))))
    (reference r2 (scope relative) (span (offset 120) (line 7) (column 16) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 120) (line 7) (column 16) (len 1)))))
    (reference r3 (scope relative) (span (offset 231) (line 11) (column 16) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 231) (line 11) (column 16) (len 1)))))
    (reference r4 (scope relative) (span (offset 233) (line 11) (column 18) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 233) (line 11) (column 18) (len 1)))))
    (reference r5 (scope relative) (span (offset 255) (line 12) (column 16) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 255) (line 12) (column 16) (len 1)))))
    (reference r6 (scope relative) (span (offset 257) (line 12) (column 18) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 257) (line 12) (column 18) (len 1)))))
    (reference r7 (scope relative) (span (offset 264) (line 12) (column 25) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 264) (line 12) (column 25) (len 1)))))
    (reference r8 (scope relative) (span (offset 268) (line 12) (column 29) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 268) (line 12) (column 29) (len 1)))))
    (reference r9 (scope relative) (span (offset 293) (line 14) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 293) (line 14) (column 20) (len 1)))))
    (reference r10 (scope relative) (span (offset 295) (line 14) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 295) (line 14) (column 22) (len 1)))))
    (reference r11 (scope relative) (span (offset 297) (line 14) (column 24) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 297) (line 14) (column 24) (len 1)))))
    (reference r12 (scope relative) (span (offset 300) (line 14) (column 27) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 300) (line 14) (column 27) (len 1)))))
    (reference r13 (scope relative) (span (offset 327) (line 15) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 327) (line 15) (column 20) (len 1)))))
  )
  (root (package (name "ParameterTest") (body brace (attribute-def (declaration-name "A") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "y") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (calc-def (name "F") (body brace (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))) (attribute-def (declaration-name "f") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 231) (line 11) (column 16) (len 7)) (invocation (callee (expression (span (offset 231) (line 11) (column 16) (len 1)) (ref r3))) (arguments (argument (parameter none) (value (expression (span (offset 233) (line 11) (column 18) (len 1)) (ref r4)))) (argument (parameter none) (value (expression (span (offset 236) (line 11) (column 21) (len 1)) (integer 2)))))))))) (body semicolon)) (attribute-def (declaration-name "g") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 255) (line 12) (column 16) (len 15)) (invocation (callee (expression (span (offset 255) (line 12) (column 16) (len 1)) (ref r5))) (arguments (argument (parameter (ref r6)) (value (expression (span (offset 261) (line 12) (column 22) (len 1)) (integer 1)))) (argument (parameter (ref r7)) (value (expression (span (offset 268) (line 12) (column 29) (len 1)) (ref r8)))))))))) (body semicolon)) (attribute-def (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 289) (line 14) (column 16) (len 16)) (constructor (type (ref r9)) (arguments (argument (parameter (ref r10)) (value (expression (span (offset 297) (line 14) (column 24) (len 1)) (ref r11)))) (argument (parameter (ref r12)) (value (expression (span (offset 302) (line 14) (column 29) (len 2)) (string "")))))))))) (body semicolon)) (attribute-def (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 323) (line 15) (column 16) (len 14)) (constructor (type (ref r13)) (arguments (argument (parameter none) (value (expression (span (offset 329) (line 15) (column 22) (len 7)) (string "test2")))))))))) (body semicolon)))))
)
~~~
