# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Packet): PacketUsage"))
~~~
# SOURCE
~~~sysml
package 'Packet Usage' {
	public import Packets::*;
	private import ScalarValues::Real;
	
	part packet1: 'Thermal Data Packet';
	part packet2: 'Thermal Data Packet';
	part packet3: 'Thermal Data Packet' {
		attribute 'special data field' redefines 'packet data field'{
			attribute redefines 'user data field' {
				attribute 'special data': Real;
			}
		}
	}
	
}
	
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "packet_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Packet Usage' {
    public import Packets::*;
    private import ScalarValues::Real;
    part packet1 : 'Thermal Data Packet';
    part packet2 : 'Thermal Data Packet';
    part packet3 : 'Thermal Data Packet' {
        attribute 'special data field' :>> 'packet data field' {
            attribute :>> 'user data field' {
                attribute 'special data' : Real;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 40) (line 2) (column 16) (len 7)) (segments (segment 0 (token "Packets") (name "Packets") (separator none) (span (offset 40) (line 2) (column 16) (len 7)))))
    (reference r1 (scope relative) (span (offset 68) (line 3) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 68) (line 3) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 82) (line 3) (column 31) (len 4)))))
    (reference r2 (scope relative) (span (offset 105) (line 5) (column 16) (len 21)) (segments (segment 0 (token "'Thermal Data Packet'") (name "Thermal Data Packet") (separator none) (span (offset 105) (line 5) (column 16) (len 21)))))
    (reference r3 (scope relative) (span (offset 143) (line 6) (column 16) (len 21)) (segments (segment 0 (token "'Thermal Data Packet'") (name "Thermal Data Packet") (separator none) (span (offset 143) (line 6) (column 16) (len 21)))))
    (reference r4 (scope relative) (span (offset 181) (line 7) (column 16) (len 21)) (segments (segment 0 (token "'Thermal Data Packet'") (name "Thermal Data Packet") (separator none) (span (offset 181) (line 7) (column 16) (len 21)))))
    (reference r5 (scope relative) (span (offset 248) (line 8) (column 44) (len 19)) (segments (segment 0 (token "'packet data field'") (name "packet data field") (separator none) (span (offset 248) (line 8) (column 44) (len 19)))))
    (reference r6 (scope relative) (span (offset 292) (line 9) (column 24) (len 17)) (segments (segment 0 (token "'user data field'") (name "user data field") (separator none) (span (offset 292) (line 9) (column 24) (len 17)))))
    (reference r7 (scope relative) (span (offset 342) (line 10) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 342) (line 10) (column 31) (len 4)))))
  )
  (root (package (name "Packet Usage") (body brace (import (target (span (span (offset 40) (line 2) (column 16) (len 10))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 23) (len 3))) (separator (span (offset 47) (line 2) (column 23) (len 2))) (marker (span (offset 49) (line 2) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 68) (line 3) (column 17) (len 18))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "packet1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "packet2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "packet3") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "special data field") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "special data") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))))
)
~~~
