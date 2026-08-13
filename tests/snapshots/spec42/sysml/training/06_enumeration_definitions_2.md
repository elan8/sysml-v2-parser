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
        unclassified;
        confidential;
        secret;
    }
    enum def GradePoints :> Real {
        A;
        B;
        C;
        D;
        F;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 27)) (segments (segment 0 (token "'Enumeration Definitions-1'") (name "Enumeration Definitions-1") (separator none) (span (offset 87) (line 3) (column 17) (len 27)))))
  )
  (root (package (name "Enumeration Definitions-2") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 30))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 114) (line 3) (column 44) (len 3))) (separator (span (offset 114) (line 3) (column 44) (len 2))) (marker (span (offset 116) (line 3) (column 46) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (enum-def) (enum-def))))
)
~~~
