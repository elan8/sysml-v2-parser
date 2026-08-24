# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 11 (Interfaces): Interface Decomposition Example"))
~~~
# SOURCE
~~~sysml
package 'Interface Decomposition Example' {
	
	port def SpigotBank;
	port def Spigot;
	
	port def Faucet;
	port def FaucetInlet;
	
	interface def WaterDelivery {
		end [1] port suppliedBy : SpigotBank {
			port hot : Spigot;
			port cold : Spigot;
		}
		end [1..*] port deliveredTo : Faucet {
			port hot : FaucetInlet;
			port cold : FaucetInlet;
		}
		
		connect suppliedBy.hot to deliveredTo.hot;
		connect suppliedBy.cold to deliveredTo.cold;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11_interface_decomposition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Interface Decomposition Example' {
    port def SpigotBank;
    port def Spigot;
    port def Faucet;
    port def FaucetInlet;
    interface def WaterDelivery {
        end [1] port suppliedBy : SpigotBank {
            port hot : Spigot;
            port cold : Spigot;
        }
        end [1..*] port deliveredTo : Faucet {
            port hot : FaucetInlet;
            port cold : FaucetInlet;
        }
        connect suppliedBy.hot to deliveredTo.hot;
        connect suppliedBy.cold to deliveredTo.cold;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 190) (line 10) (column 29) (len 10)) (segments (segment 0 (token "SpigotBank") (name "SpigotBank") (separator none) (span (offset 190) (line 10) (column 29) (len 10)))))
    (reference r1 (scope relative) (span (offset 217) (line 11) (column 15) (len 6)) (segments (segment 0 (token "Spigot") (name "Spigot") (separator none) (span (offset 217) (line 11) (column 15) (len 6)))))
    (reference r2 (scope relative) (span (offset 240) (line 12) (column 16) (len 6)) (segments (segment 0 (token "Spigot") (name "Spigot") (separator none) (span (offset 240) (line 12) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 284) (line 14) (column 33) (len 6)) (segments (segment 0 (token "Faucet") (name "Faucet") (separator none) (span (offset 284) (line 14) (column 33) (len 6)))))
    (reference r4 (scope relative) (span (offset 307) (line 15) (column 15) (len 11)) (segments (segment 0 (token "FaucetInlet") (name "FaucetInlet") (separator none) (span (offset 307) (line 15) (column 15) (len 11)))))
    (reference r5 (scope relative) (span (offset 335) (line 16) (column 16) (len 11)) (segments (segment 0 (token "FaucetInlet") (name "FaucetInlet") (separator none) (span (offset 335) (line 16) (column 16) (len 11)))))
  )
  (root (package (name "Interface Decomposition Example") (body brace (port-def (name "SpigotBank") (modifiers) (specializes none) (body semicolon)) (port-def (name "Spigot") (modifiers) (specializes none) (body semicolon)) (port-def (name "Faucet") (modifiers) (specializes none) (body semicolon)) (port-def (name "FaucetInlet") (modifiers) (specializes none) (body semicolon)) (interface-def (name "WaterDelivery") (modifiers) (specializes none) (body brace (port-usage (prefix (end (cross ((direction none) (derived false) (variance none) (constant false) (reference false) (name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 169) (line 10) (column 8) (len 1)) (integer 1))) (upper (expression (span (offset 169) (line 10) (column 8) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (extensions)) (declaration-name "suppliedBy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hot") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cold") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-usage (prefix (end (cross ((direction none) (derived false) (variance none) (constant false) (reference false) (name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 259) (line 14) (column 8) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (extensions)) (declaration-name "deliveredTo") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hot") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cold") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connect (body semicolon)) (connect (body semicolon)))))))
)
~~~
