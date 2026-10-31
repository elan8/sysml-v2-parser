# META
~~~sexpr
(snapshot (type semantic) (description "Fuzzer crash: malformed package with unclosed braces"))
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;

	part def MassedThing {
		attribute simpleMass :> ISQ::mass;
		attribute totalMass :> ISQ::mass default sLmpleMass;
	}

	part composicomackagteThing : MassedThing {
		p@rt subcomponents: MassedThing[*]ature redefin;
		arValuete :>> totalMass default
			simleMass + sum(subcomponents.totalMass);
	}

	part filter   ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		wtes A;

	simpleMass + sum(subcomackage eMassponents import Numeric.totalMassFpackage 'Metadata Example-1' {

	metadata def SafetyFeature;
	metadata def Securi
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_unclosed_package.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 229) (line 10) (column 3) (len 51)) (message "unrecognized declaration `p` in part usage body"))
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 362) (line 15) (column 2) (len 264)) (message "unexpected token in package body"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 626) (line 23) (column 21) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRollup2 {
    private import NumericalFunctions::*;
    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass default sLmpleMass;
    }
    part composicomackagteThing : MassedThing {
        p@rt subcomponents: MassedThing[*]ature redefin;
        arValuete :>> totalMass default simleMass + sum(subcomponents.totalMass);
    }
    part filter   ssThing :> compositeThing {
		attribute minMass :> ISQ::mass;
		atribute :>> totalMass =
		wtes A;

	simpleMass + sum(subcomackage eMassponents import Numeric.totalMassFpackage 'Metadata Example-1' {

	metadata def SafetyFeature;
	metadata def Securi
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 38) (line 2) (column 17) (len 18)))))
    (reference r1 (scope relative) (span (offset 112) (line 5) (column 27) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 112) (line 5) (column 27) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 117) (line 5) (column 32) (len 4)))))
    (reference r2 (scope relative) (span (offset 148) (line 6) (column 26) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 148) (line 6) (column 26) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 153) (line 6) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 166) (line 6) (column 44) (len 10)) (segments (segment 0 (token "sLmpleMass") (name "sLmpleMass") (separator none) (span (offset 166) (line 6) (column 44) (len 10)))))
    (reference r4 (scope relative) (span (offset 213) (line 9) (column 32) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 213) (line 9) (column 32) (len 11)))))
    (reference r5 (scope relative) (span (offset 294) (line 11) (column 17) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 294) (line 11) (column 17) (len 9)))))
    (reference r6 (scope relative) (span (offset 315) (line 12) (column 4) (len 9)) (segments (segment 0 (token "simleMass") (name "simleMass") (separator none) (span (offset 315) (line 12) (column 4) (len 9)))))
    (reference r7 (scope relative) (span (offset 327) (line 12) (column 16) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 327) (line 12) (column 16) (len 3)))))
    (reference r8 (scope relative) (span (offset 331) (line 12) (column 20) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 331) (line 12) (column 20) (len 13)))))
    (reference r9 (scope relative) (span (offset 345) (line 12) (column 34) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 345) (line 12) (column 34) (len 9)))))
  )
  (root (package (name "MassRollup2") (body brace (unterminated) (import (visibility private) (target (span (span (offset 38) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 56) (line 2) (column 35) (len 3))) (separator (span (offset 56) (line 2) (column 35) (len 2))) (marker (span (offset 58) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (part-def (name "MassedThing") (modifiers) (body brace (attribute-usage (declaration-name "simpleMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 166) (line 6) (column 44) (len 10)) (ref r3))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "composicomackagteThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "p@rt subcomponents: MassedThing[*]ature redefin;") (span (offset 229) (line 10) (column 3) (len 51))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "arValuete") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 315) (line 12) (column 4) (len 40)) (binary (operator "+") (left (expression (span (offset 315) (line 12) (column 4) (len 9)) (ref r6))) (right (expression (span (offset 327) (line 12) (column 16) (len 28)) (invocation (callee (expression (span (offset 327) (line 12) (column 16) (len 3)) (ref r7))) (arguments (argument (parameter none) (value (expression (span (offset 331) (line 12) (column 20) (len 23)) (member-access (base (expression (span (offset 331) (line 12) (column 20) (len 13)) (ref r8))) (separator dot) (member (ref r9))))))))))))))) (body semicolon)))) (malformed (code "recovered_package_body_element") (found "part filter   ssThing :> compositeThing {") (span (offset 362) (line 15) (column 2) (len 264))))))
)
~~~
