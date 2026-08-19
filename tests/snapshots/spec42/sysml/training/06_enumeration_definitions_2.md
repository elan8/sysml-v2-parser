# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 06 (Enumeration Definitions): Enumeration Definitions-2"))
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-2' {
	private import ScalarValues::*;
	private import 'Enumeration Definitions-1'::*;
	
	attribute def ClassificationLevel {
		attribute code : String;
		attribute color : TrafficLightColor;
	}
	
	enum def ClassificationKind specializes ClassificationLevel {
		unclassified {
			:>> code = "uncl";
			:>> color = TrafficLightColor::green;
		}
		confidential {
			:>> code = "conf";
			:>> color = TrafficLightColor::yellow;
		}
		secret {
			:>> code = "secr";
			:>> color = TrafficLightColor::red;
		}
	}
	
	enum def GradePoints :> Real {
		A = 4.0;
		B = 3.0;
		C = 2.0;
		D = 1.0;
		F = 0.0;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "06_enumeration_definitions_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Enumeration Definitions-2' {
    private import ScalarValues::*;
    private import 'Enumeration Definitions-1'::*;
    attribute def ClassificationLevel {
        attribute code : String;
        attribute color : TrafficLightColor;
    }
    enum def ClassificationKind specializes ClassificationLevel {
        unclassified {
            attribute :>> code = "uncl";
            attribute :>> color = TrafficLightColor::green;
        }
        confidential {
            attribute :>> code = "conf";
            attribute :>> color = TrafficLightColor::yellow;
        }
        secret {
            attribute :>> code = "secr";
            attribute :>> color = TrafficLightColor::red;
        }
    }
    enum def GradePoints :> Real {
        A = 4.0;
        B = 3.0;
        C = 2.0;
        D = 1.0;
        F = 0.0;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 27)) (segments (segment 0 (token "'Enumeration Definitions-1'") (name "Enumeration Definitions-1") (separator none) (span (offset 87) (line 3) (column 17) (len 27)))))
    (reference r2 (scope relative) (span (offset 177) (line 6) (column 20) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 177) (line 6) (column 20) (len 6)))))
    (reference r3 (scope relative) (span (offset 205) (line 7) (column 21) (len 17)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 205) (line 7) (column 21) (len 17)))))
    (reference r4 (scope relative) (span (offset 316) (line 12) (column 8) (len 4)) (segments (segment 0 (token "code") (name "code") (separator none) (span (offset 316) (line 12) (column 8) (len 4)))))
    (reference r5 (scope relative) (span (offset 338) (line 13) (column 8) (len 5)) (segments (segment 0 (token "color") (name "color") (separator none) (span (offset 338) (line 13) (column 8) (len 5)))))
    (reference r6 (scope relative) (span (offset 346) (line 13) (column 16) (len 24)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 346) (line 13) (column 16) (len 17))) (segment 1 (token "green") (name "green") (separator colon-colon) (span (offset 365) (line 13) (column 35) (len 5)))))
    (reference r7 (scope relative) (span (offset 400) (line 16) (column 8) (len 4)) (segments (segment 0 (token "code") (name "code") (separator none) (span (offset 400) (line 16) (column 8) (len 4)))))
    (reference r8 (scope relative) (span (offset 422) (line 17) (column 8) (len 5)) (segments (segment 0 (token "color") (name "color") (separator none) (span (offset 422) (line 17) (column 8) (len 5)))))
    (reference r9 (scope relative) (span (offset 430) (line 17) (column 16) (len 25)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 430) (line 17) (column 16) (len 17))) (segment 1 (token "yellow") (name "yellow") (separator colon-colon) (span (offset 449) (line 17) (column 35) (len 6)))))
    (reference r10 (scope relative) (span (offset 479) (line 20) (column 8) (len 4)) (segments (segment 0 (token "code") (name "code") (separator none) (span (offset 479) (line 20) (column 8) (len 4)))))
    (reference r11 (scope relative) (span (offset 501) (line 21) (column 8) (len 5)) (segments (segment 0 (token "color") (name "color") (separator none) (span (offset 501) (line 21) (column 8) (len 5)))))
    (reference r12 (scope relative) (span (offset 509) (line 21) (column 16) (len 22)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 509) (line 21) (column 16) (len 17))) (segment 1 (token "red") (name "red") (separator colon-colon) (span (offset 528) (line 21) (column 35) (len 3)))))
  )
  (root (package (name "Enumeration Definitions-2") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 30))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 114) (line 3) (column 44) (len 3))) (separator (span (offset 114) (line 3) (column 44) (len 2))) (marker (span (offset 116) (line 3) (column 46) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "ClassificationLevel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "code") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "color") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (enum-def (name "ClassificationKind") (body brace (enum-value (name "unclassified") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 323) (line 12) (column 15) (len 6)) (string "uncl"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 346) (line 13) (column 16) (len 24)) (ref r6))))) (body semicolon))) (span (offset 294) (line 11) (column 3) (len 81))) (enum-value (name "confidential") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 407) (line 16) (column 15) (len 6)) (string "conf"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 430) (line 17) (column 16) (len 25)) (ref r9))))) (body semicolon))) (span (offset 378) (line 15) (column 3) (len 82))) (enum-value (name "secret") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 486) (line 20) (column 15) (len 6)) (string "secr"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 509) (line 21) (column 16) (len 22)) (ref r12))))) (body semicolon))) (span (offset 463) (line 19) (column 3) (len 73))))) (enum-def (name "GradePoints") (body brace (enum-value (name "A") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 580) (line 26) (column 7) (len 3)) (real "4.0"))))) (body semicolon) (span (offset 576) (line 26) (column 3) (len 8))) (enum-value (name "B") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 591) (line 27) (column 7) (len 3)) (real "3.0"))))) (body semicolon) (span (offset 587) (line 27) (column 3) (len 8))) (enum-value (name "C") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 602) (line 28) (column 7) (len 3)) (real "2.0"))))) (body semicolon) (span (offset 598) (line 28) (column 3) (len 8))) (enum-value (name "D") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 613) (line 29) (column 7) (len 3)) (real "1.0"))))) (body semicolon) (span (offset 609) (line 29) (column 3) (len 8))) (enum-value (name "F") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 624) (line 30) (column 7) (len 3)) (real "0.0"))))) (body semicolon) (span (offset 620) (line 30) (column 3) (len 8))))))))
)
~~~
