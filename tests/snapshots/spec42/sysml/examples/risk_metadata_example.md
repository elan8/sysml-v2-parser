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
    (reference r2 (scope relative) (span (offset 129) (line 6) (column 10) (len 4)) (segments (segment 0 (token "Risk") (name "Risk") (separator none) (span (offset 129) (line 6) (column 10) (len 4)))))
    (reference r3 (scope relative) (span (offset 160) (line 7) (column 25) (len 4)) (segments (segment 0 (token "high") (name "high") (separator none) (span (offset 160) (line 7) (column 25) (len 4)))))
    (reference r4 (scope relative) (span (offset 194) (line 8) (column 29) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 194) (line 8) (column 29) (len 6)))))
    (reference r5 (scope relative) (span (offset 229) (line 9) (column 28) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 229) (line 9) (column 28) (len 6)))))
    (reference r6 (scope relative) (span (offset 256) (line 11) (column 10) (len 4)) (segments (segment 0 (token "Risk") (name "Risk") (separator none) (span (offset 256) (line 11) (column 10) (len 4)))))
  )
  (root (package (name "RiskMetadataExample") (body brace (import (target (span (span (offset 46) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 58) (line 2) (column 29) (len 3))) (separator (span (offset 58) (line 2) (column 29) (len 2))) (marker (span (offset 60) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 79) (line 3) (column 17) (len 16))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 92) (line 3) (column 30) (len 3))) (separator (span (offset 92) (line 3) (column 30) (len 2))) (marker (span (offset 94) (line 3) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine4cyl") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body brace (attribute-usage (declaration-name "totalRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 160) (line 7) (column 25) (len 4)) (ref r3))))) (body semicolon)) (attribute-usage (declaration-name "technicalRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 194) (line 8) (column 29) (len 6)) (ref r4))))) (body semicolon)) (attribute-usage (declaration-name "scheduleRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 229) (line 9) (column 28) (len 6)) (ref r5))))) (body semicolon)))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "totalRisk {") (span (offset 272) (line 12) (column 10) (len 94))))))))))
)
~~~
