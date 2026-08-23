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
    attribute 'packet header' {
    }
    attribute 'packet data field' {
        attribute 'packet secondary header' redefines 'packet header';
        attribute 'user data field';
    }
    part def 'Data Packet' {
        attribute 'packet primary header' redefines 'packet header' {
            attribute 'packet version number' : Integer;
            attribute 'packet identification' : String;
            attribute 'packet data length' : Integer;
        }
        attribute redefines 'packet data field';
    }
    part def 'Thermal Data Packet' :> 'Data Packet' {
        attribute 'packet data field' redefines Packets::'packet data field' {
            attribute 'packet secondary header' redefines 'packet header' {
                attribute 'packet timestamp' : DateTime;
                attribute 'telemetry packet type' : String;
            }
            attribute 'user data field' redefines Packets::'packet data field'::'user data field' {
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
    (reference r3 (scope relative) (span (offset 381) (line 14) (column 39) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 381) (line 14) (column 39) (len 7)))))
    (reference r4 (scope relative) (span (offset 428) (line 15) (column 39) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 428) (line 15) (column 39) (len 6)))))
    (reference r5 (scope relative) (span (offset 471) (line 16) (column 36) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 471) (line 16) (column 36) (len 7)))))
    (reference r6 (scope relative) (span (offset 506) (line 18) (column 23) (len 19)) (segments (segment 0 (token "'packet data field'") (name "packet data field") (separator none) (span (offset 506) (line 18) (column 23) (len 19)))))
    (reference r7 (scope relative) (span (offset 625) (line 22) (column 43) (len 28)) (segments (segment 0 (token "Packets") (name "Packets") (separator none) (span (offset 625) (line 22) (column 43) (len 7))) (segment 1 (token "'packet data field'") (name "packet data field") (separator colon-colon) (span (offset 634) (line 22) (column 52) (len 19)))))
    (reference r8 (scope relative) (span (offset 704) (line 23) (column 50) (len 15)) (segments (segment 0 (token "'packet header'") (name "packet header") (separator none) (span (offset 704) (line 23) (column 50) (len 15)))))
    (reference r9 (scope relative) (span (offset 756) (line 24) (column 35) (len 8)) (segments (segment 0 (token "DateTime") (name "DateTime") (separator none) (span (offset 756) (line 24) (column 35) (len 8)))))
    (reference r10 (scope relative) (span (offset 805) (line 25) (column 40) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 805) (line 25) (column 40) (len 6)))))
    (reference r11 (scope relative) (span (offset 863) (line 28) (column 42) (len 47)) (segments (segment 0 (token "Packets") (name "Packets") (separator none) (span (offset 863) (line 28) (column 42) (len 7))) (segment 1 (token "'packet data field'") (name "packet data field") (separator colon-colon) (span (offset 872) (line 28) (column 51) (len 19))) (segment 2 (token "'user data field'") (name "user data field") (separator colon-colon) (span (offset 893) (line 28) (column 72) (len 17)))))
    (reference r12 (scope relative) (span (offset 938) (line 29) (column 26) (len 8)) (segments (segment 0 (token "DateTime") (name "DateTime") (separator none) (span (offset 938) (line 29) (column 26) (len 8)))))
    (reference r13 (scope relative) (span (offset 975) (line 30) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 975) (line 30) (column 28) (len 4)))))
  )
  (root (package (name "Packets") (body brace (import (target (span (span (offset 34) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 46) (line 2) (column 29) (len 3))) (separator (span (offset 46) (line 2) (column 29) (len 2))) (marker (span (offset 48) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 67) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (attribute-usage) (attribute-usage) (part-def (name "Data Packet") (modifiers) (body brace (attribute-usage (declaration-name "packet primary header") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "packet version number") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "packet identification") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "packet data length") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Thermal Data Packet") (modifiers) (body brace (attribute-usage (declaration-name "packet data field") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "packet secondary header") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "packet timestamp") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "telemetry packet type") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage (declaration-name "user data field") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "timestamp") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "temperature") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))))
)
~~~
