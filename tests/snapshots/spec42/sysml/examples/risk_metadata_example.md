# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Metadata): RiskMetadataExample"))
~~~
# SOURCE
~~~sysml
package RiskMetadataExample {
	private import RiskMetadata::*;
	private import RiskLevelEnum::*;
	
    part engine4cyl{
        @Risk {
            totalRisk = high;
            technicalRisk = medium;
            scheduleRisk = medium;
        }
        @Risk {
        	totalRisk { 
        		probability = 0.3;
        		impact = 0.7;
        	}        	
        }
    }
        
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "risk_metadata_example.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 272) (line 12) (column 10) (len 94)) (message "unrecognized declaration `totalRisk` in metadata body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package RiskMetadataExample {
    private import RiskMetadata::*;
    private import RiskLevelEnum::*;
    part engine4cyl {
        @Risk {
            attribute totalRisk = high;
            attribute technicalRisk = medium;
            attribute scheduleRisk = medium;
        }
        @Risk {
            totalRisk { 
        		probability = 0.3;
        		impact = 0.7;
        	}
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 46) (line 2) (column 17) (len 12)) (segments (segment 0 (token "RiskMetadata") (name "RiskMetadata") (separator none) (span (offset 46) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 17) (len 13)) (segments (segment 0 (token "RiskLevelEnum") (name "RiskLevelEnum") (separator none) (span (offset 79) (line 3) (column 17) (len 13)))))
  )
  (root (package (name "RiskMetadataExample") (body (import (target (span (span (offset 46) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 58) (line 2) (column 29) (len 3))) (separator (span (offset 58) (line 2) (column 29) (len 2))) (marker (span (offset 60) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 79) (line 3) (column 17) (len 16))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 92) (line 3) (column 30) (len 3))) (separator (span (offset 92) (line 3) (column 30) (len 2))) (marker (span (offset 94) (line 3) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage))))
)
~~~
