# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_08-Range Restriction"))
~~~
# SOURCE
~~~sysml
package '15_08-Range Restriction' {
	private import ISQ::*;
	private import SI::*;
	private import '15_01-Constants'::'Mathematical Constants'::pi;
	
	part def HeadLightsTiltKnob {
		attribute headLightsTile : LightBeamTiltAngleValue[1];
	}
	
	attribute def LightBeamTiltAngleValue :> PlaneAngleValue {
		attribute angle: LightBeamTiltAngleValue :>> self {
			doc
			/*
			 * Tilt angle shall be limited to the range between 50 and 80 degrees (inclusive).
			 */
		}
		assert constraint { angle >= 50 ['°'] and angle <= 80 ['°'] }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_08_range_restriction.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_08-Range Restriction' {
    private import ISQ::*;
    private import SI::*;
    private import '15_01-Constants'::'Mathematical Constants'::pi;
    part def HeadLightsTiltKnob {
        attribute headLightsTile : LightBeamTiltAngleValue[1];
    }
    attribute def LightBeamTiltAngleValue :> PlaneAngleValue {
        attribute angle : LightBeamTiltAngleValue :>> self {
            doc
            /*
			 * Tilt angle shall be limited to the range between 50 and 80 degrees (inclusive).
			 */
        }
        assert constraint {
            angle >= 50 ['°'] && angle <= 80 ['°'];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 52) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 76) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 76) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 99) (line 4) (column 17) (len 47)) (segments (segment 0 (token "'15_01-Constants'") (name "15_01-Constants") (separator none) (span (offset 99) (line 4) (column 17) (len 17))) (segment 1 (token "'Mathematical Constants'") (name "Mathematical Constants") (separator colon-colon) (span (offset 118) (line 4) (column 36) (len 24))) (segment 2 (token "pi") (name "pi") (separator colon-colon) (span (offset 144) (line 4) (column 62) (len 2)))))
    (reference r3 (scope relative) (span (offset 210) (line 7) (column 30) (len 23)) (segments (segment 0 (token "LightBeamTiltAngleValue") (name "LightBeamTiltAngleValue") (separator none) (span (offset 210) (line 7) (column 30) (len 23)))))
  )
  (root (package (name "15_08-Range Restriction") (body (import (target (span (span (offset 52) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 20) (len 3))) (separator (span (offset 55) (line 2) (column 20) (len 2))) (marker (span (offset 57) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 3) (column 19) (len 3))) (separator (span (offset 78) (line 3) (column 19) (len 2))) (marker (span (offset 80) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 99) (line 4) (column 17) (len 47))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (part-def (name "HeadLightsTiltKnob") (body (attribute-usage (declaration-name "headLightsTile") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def))))
)
~~~
