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
  )
  (root (package (name "Packet Usage") (body brace (import (target (span (span (offset 40) (line 2) (column 16) (len 10))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 23) (len 3))) (separator (span (offset 47) (line 2) (column 23) (len 2))) (marker (span (offset 49) (line 2) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 68) (line 3) (column 17) (len 18))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (part-usage) (part-usage) (part-usage))))
)
~~~
