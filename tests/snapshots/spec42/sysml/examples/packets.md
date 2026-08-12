# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Packet): Packets"))
~~~
# SOURCE
~~~sysml
package Packets {
	private import ScalarValues::*;
	private import Time::DateTime;
	
	attribute 'packet header' { }
	
	attribute 'packet data field' {	
		attribute 'packet secondary header' redefines 'packet header';
		attribute 'user data field';
	}
	
	part def 'Data Packet' {
		attribute 'packet primary header' redefines 'packet header' {
			attribute 'packet version number': Integer;
			attribute 'packet identification': String;
			attribute 'packet data length': Integer;
		}
		attribute redefines 'packet data field';
	}
	
	part def 'Thermal Data Packet' :> 'Data Packet' {
		attribute 'packet data field' redefines Packets::'packet data field'{
			attribute 'packet secondary header' redefines 'packet header' {
				attribute 'packet timestamp': DateTime;
				attribute 'telemetry packet type': String;
			}
			
			attribute 'user data field' redefines Packets::'packet data field'::'user data field' {
				attribute timestamp: DateTime;
				attribute temperature: Real;
			}
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "packets.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Packets {
    private import ScalarValues::*;
    private import Time::DateTime;
    attribute def 'packet header' {
    }
    attribute def 'packet data field' {
        attribute 'packet secondary header' :>> 'packet header';
        attribute 'user data field';
    }
    part def 'Data Packet' {
        attribute 'packet primary header' :>> 'packet header' {
            attribute 'packet version number' : Integer;
            attribute 'packet identification' : String;
            attribute 'packet data length' : Integer;
        }
        attribute  :>> 'packet data field';
    }
    part def 'Thermal Data Packet' :> 'Data Packet' {
        attribute 'packet data field' :>> Packets::'packet data field' {
            attribute 'packet secondary header' :>> 'packet header' {
                attribute 'packet timestamp' : DateTime;
                attribute 'telemetry packet type' : String;
            }
            attribute 'user data field' :>> Packets::'packet data field'::'user data field' {
                attribute timestamp : DateTime;
                attribute temperature : Real;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 34) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 34) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 67) (line 3) (column 17) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 67) (line 3) (column 17) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 73) (line 3) (column 23) (len 8)))))
    (reference r2 (scope relative) (span (offset 325) (line 13) (column 47) (len 15)) (segments (segment 0 (token "'packet header'") (name "packet header") (separator none) (span (offset 325) (line 13) (column 47) (len 15)))))
    (reference r3 (scope relative) (span (offset 506) (line 18) (column 23) (len 19)) (segments (segment 0 (token "'packet data field'") (name "packet data field") (separator none) (span (offset 506) (line 18) (column 23) (len 19)))))
    (reference r4 (scope relative) (span (offset 625) (line 22) (column 43) (len 28)) (segments (segment 0 (token "Packets") (name "Packets") (separator none) (span (offset 625) (line 22) (column 43) (len 7))) (segment 1 (token "'packet data field'") (name "packet data field") (separator colon-colon) (span (offset 634) (line 22) (column 52) (len 19)))))
  )
  (root (package (name "Packets") (body (import (target (span (span (offset 34) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 46) (line 2) (column 29) (len 3))) (separator (span (offset 46) (line 2) (column 29) (len 2))) (marker (span (offset 48) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 67) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (attribute-def) (attribute-def) (part-def (name "Data Packet") (body (attribute-usage (declaration-name "packet primary header") (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 3))) (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Thermal Data Packet") (body (attribute-usage (declaration-name "packet data field") (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 2))))))))
)
~~~
