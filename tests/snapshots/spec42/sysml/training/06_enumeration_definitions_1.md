# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 06 (Enumeration Definitions): Enumeration Definitions-1"))
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-1' {
	private import ScalarValues::Real;
	
	enum def TrafficLightColor {
		enum green;
		enum yellow;
		enum red;
	}
	
	part def TrafficLight {
		attribute currentColor : TrafficLightColor;
	}
	
	part def TrafficLightGo specializes TrafficLight {
		attribute redefines currentColor = TrafficLightColor::green;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "06_enumeration_definitions_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Enumeration Definitions-1' {
    private import ScalarValues::Real;
    enum def TrafficLightColor {
        green;
        yellow;
        red;
    }
    part def TrafficLight {
        attribute currentColor : TrafficLightColor;
    }
    part def TrafficLightGo specializes TrafficLight {
        attribute  :>> currentColor = TrafficLightColor::green;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 68) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 204) (line 11) (column 28) (len 17)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 204) (line 11) (column 28) (len 17)))))
    (reference r2 (scope relative) (span (offset 302) (line 15) (column 23) (len 12)) (segments (segment 0 (token "currentColor") (name "currentColor") (separator none) (span (offset 302) (line 15) (column 23) (len 12)))))
    (reference r3 (scope relative) (span (offset 317) (line 15) (column 38) (len 24)) (segments (segment 0 (token "TrafficLightColor") (name "TrafficLightColor") (separator none) (span (offset 317) (line 15) (column 38) (len 17))) (segment 1 (token "green") (name "green") (separator colon-colon) (span (offset 336) (line 15) (column 57) (len 5)))))
  )
  (root (package (name "Enumeration Definitions-1") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (enum-def) (part-def (name "TrafficLight") (body (attribute-usage (declaration-name "currentColor") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "TrafficLightGo") (body (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 317) (line 15) (column 38) (len 24)) (ref r3))))) (body semicolon)))))))
)
~~~
