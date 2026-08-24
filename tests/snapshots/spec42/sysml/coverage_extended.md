# META
~~~sexpr
(snapshot (type semantic) (description "Group 12: Extended Definitions and Usages (SysML §8.2.2.27)"))
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_extended.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure {
        part p;
    }
    #situation batteryLow;
    #situation x : T;
    #situation x : T {}
    variation #situation def V;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 32) (line 2) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 32) (line 2) (column 6) (len 9)))))
    (reference r1 (scope relative) (span (offset 60) (line 3) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 60) (line 3) (column 6) (len 9)))))
    (reference r2 (scope relative) (span (offset 85) (line 3) (column 31) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 85) (line 3) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 105) (line 4) (column 15) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 105) (line 4) (column 15) (len 9)))))
    (reference r4 (scope relative) (span (offset 141) (line 5) (column 6) (len 15)) (segments (segment 0 (token "SecurityRelated") (name "SecurityRelated") (separator none) (span (offset 141) (line 5) (column 6) (len 15)))))
    (reference r5 (scope relative) (span (offset 158) (line 5) (column 23) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 158) (line 5) (column 23) (len 9)))))
    (reference r6 (scope relative) (span (offset 192) (line 6) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 192) (line 6) (column 6) (len 9)))))
    (reference r7 (scope relative) (span (offset 231) (line 7) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 231) (line 7) (column 6) (len 9)))))
    (reference r8 (scope relative) (span (offset 258) (line 8) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 258) (line 8) (column 6) (len 9)))))
    (reference r9 (scope relative) (span (offset 272) (line 8) (column 20) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 272) (line 8) (column 20) (len 1)))))
    (reference r10 (scope relative) (span (offset 280) (line 9) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 280) (line 9) (column 6) (len 9)))))
    (reference r11 (scope relative) (span (offset 294) (line 9) (column 20) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 294) (line 9) (column 20) (len 1)))))
    (reference r12 (scope relative) (span (offset 315) (line 10) (column 16) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 315) (line 10) (column 16) (len 9)))))
  )
  (root (package (name "ExtendedExamples") (body brace (extended-def (prefix-keywords ((ref r0))) (definition-prefix none) (name "Failure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Failure") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body semicolon)) (extended-def (prefix-keywords ((ref r3))) (definition-prefix abstract) (name "AbstractFailure") (specializes none) (body semicolon)) (extended-def (prefix-keywords ((ref r4) (ref r5))) (definition-prefix none) (name "Vulnerability") (specializes none) (body semicolon)) (extended-def (prefix-keywords ((ref r6))) (definition-prefix none) (name "Failure") (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r7)) (declaration (name "batteryLow") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r10)) (declaration (name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace)) (extended-def (prefix-keywords ((ref r12))) (definition-prefix variation) (name "V") (specializes none) (body semicolon)))))
)
~~~
