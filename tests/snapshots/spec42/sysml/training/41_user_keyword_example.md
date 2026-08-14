# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 41 (Language Extension): User Keyword Example"))
~~~
# SOURCE
~~~sysml
package 'User Keyword Example' {
	private import ScalarValues::Real;
	private import 'Semantic Metadata Example'::*;
	private import RiskMetadata::LevelEnum;
	
	part def Device {
		part battery {
			attribute power : Real;
		}
	}
	
	#scenario def DeviceFailure {
		ref device : Device;
		attribute minPower : Real;
		
		#cause 'battery old' {
			:>> probability = 0.01;			
		}
		
		#causation connect 'battery old' to 'power low';
		
		#situation 'power low' {
			constraint { device.battery.power < minPower }			
		}
		
		#causation connect 'power low' to 'device shutoff';
		
		#failure 'device shutoff' {
			:>> severity = LevelEnum::high;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_user_keyword_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'User Keyword Example' {
    private import ScalarValues::Real;
    private import 'Semantic Metadata Example'::*;
    private import RiskMetadata::LevelEnum;
    part def Device {
        part battery {
            attribute power : Real;
        }
    }
    #scenario def DeviceFailure {
        ref device : Device;
        attribute def minPower : Real;
        #cause 'battery old' {
            '' :>> probability = 0.01;
        }
        #causation
        connect 'battery old' to 'power low';
        #situation 'power low' {
            constraint {
                device.battery.power < minPower;
            }
        }
        #causation
        connect 'power low' to 'device shutoff';
        #failure 'device shutoff' {
            '' :>> severity = LevelEnum::high;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 49) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 49) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 63) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 85) (line 3) (column 17) (len 27)) (segments (segment 0 (token "'Semantic Metadata Example'") (name "Semantic Metadata Example") (separator none) (span (offset 85) (line 3) (column 17) (len 27)))))
    (reference r2 (scope relative) (span (offset 133) (line 4) (column 17) (len 23)) (segments (segment 0 (token "RiskMetadata") (name "RiskMetadata") (separator none) (span (offset 133) (line 4) (column 17) (len 12))) (segment 1 (token "LevelEnum") (name "LevelEnum") (separator colon-colon) (span (offset 147) (line 4) (column 31) (len 9)))))
    (reference r3 (scope relative) (span (offset 278) (line 13) (column 16) (len 6)) (segments (segment 0 (token "Device") (name "Device") (separator none) (span (offset 278) (line 13) (column 16) (len 6)))))
    (reference r4 (scope relative) (span (offset 401) (line 20) (column 22) (len 13)) (segments (segment 0 (token "'battery old'") (name "battery old") (separator none) (span (offset 401) (line 20) (column 22) (len 13)))))
    (reference r5 (scope relative) (span (offset 418) (line 20) (column 39) (len 11)) (segments (segment 0 (token "'power low'") (name "power low") (separator none) (span (offset 418) (line 20) (column 39) (len 11)))))
    (reference r6 (scope relative) (span (offset 542) (line 26) (column 22) (len 11)) (segments (segment 0 (token "'power low'") (name "power low") (separator none) (span (offset 542) (line 26) (column 22) (len 11)))))
    (reference r7 (scope relative) (span (offset 557) (line 26) (column 37) (len 16)) (segments (segment 0 (token "'device shutoff'") (name "device shutoff") (separator none) (span (offset 557) (line 26) (column 37) (len 16)))))
  )
  (root (package (name "User Keyword Example") (body (import (target (span (span (offset 49) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 85) (line 3) (column 17) (len 30))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 112) (line 3) (column 44) (len 3))) (separator (span (offset 112) (line 3) (column 44) (len 2))) (marker (span (offset 114) (line 3) (column 46) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 133) (line 4) (column 17) (len 23))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (part-def (name "Device") (body (part-usage))) (extended-def (prefix-keywords ("scenario")) (definition-prefix none) (def true) (name "DeviceFailure") (specializes none) (body (ref (name "device") (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (redefines none) (subsets none) (body semicolon)) (attribute-def) (extended-def (prefix-keywords ("cause")) (definition-prefix none) (def false) (name "battery old") (specializes none) (body (default-reference-usage))) (metadata-keyword-usage) (connect (from (expression (span (offset 401) (line 20) (column 22) (len 13)) (ref r4))) (to (expression (span (offset 418) (line 20) (column 39) (len 11)) (ref r5))) (body semicolon) (subsets none) (redefines none)) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def false) (name "power low") (specializes none) (body (constraint-usage))) (metadata-keyword-usage) (connect (from (expression (span (offset 542) (line 26) (column 22) (len 11)) (ref r6))) (to (expression (span (offset 557) (line 26) (column 37) (len 16)) (ref r7))) (body semicolon) (subsets none) (redefines none)) (extended-def (prefix-keywords ("failure")) (definition-prefix none) (def false) (name "device shutoff") (specializes none) (body (default-reference-usage))))))))
)
~~~
