# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: named arguments in invocations use = not => for idempotent reparse"))
~~~
# SOURCE
~~~sysml
package P {
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_named_argument.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    calc def F {
        in p : A;
    }
    attribute def f = F(q = 1, p = a);
    attribute def b = new A(y = a, x = "");
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 59) (line 3) (column 19) (len 1)) (segments (segment 0 (token "F") (name "F") (separator none) (span (offset 59) (line 3) (column 19) (len 1)))))
    (reference r1 (scope relative) (span (offset 61) (line 3) (column 21) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 61) (line 3) (column 21) (len 1)))))
    (reference r2 (scope relative) (span (offset 68) (line 3) (column 28) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 68) (line 3) (column 28) (len 1)))))
    (reference r3 (scope relative) (span (offset 72) (line 3) (column 32) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 72) (line 3) (column 32) (len 1)))))
    (reference r4 (scope relative) (span (offset 98) (line 4) (column 23) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 98) (line 4) (column 23) (len 1)))))
    (reference r5 (scope relative) (span (offset 100) (line 4) (column 25) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 100) (line 4) (column 25) (len 1)))))
    (reference r6 (scope relative) (span (offset 104) (line 4) (column 29) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 104) (line 4) (column 29) (len 1)))))
    (reference r7 (scope relative) (span (offset 107) (line 4) (column 32) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 107) (line 4) (column 32) (len 1)))))
  )
  (root (package (name "P") (body brace (calc-def (name "F") (body brace (in-out-declaration))) (attribute-def (declaration-name "f") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59) (line 3) (column 19) (len 15)) (invocation (callee (expression (span (offset 59) (line 3) (column 19) (len 1)) (ref r0))) (arguments (argument (parameter (ref r1)) (value (expression (span (offset 65) (line 3) (column 25) (len 1)) (integer 1)))) (argument (parameter (ref r2)) (value (expression (span (offset 72) (line 3) (column 32) (len 1)) (ref r3)))))))))) (body semicolon)) (attribute-def (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 94) (line 4) (column 19) (len 20)) (constructor (type (ref r4)) (arguments (argument (parameter (ref r5)) (value (expression (span (offset 104) (line 4) (column 29) (len 1)) (ref r6)))) (argument (parameter (ref r7)) (value (expression (span (offset 111) (line 4) (column 36) (len 2)) (string "")))))))))) (body semicolon)))))
)
~~~
