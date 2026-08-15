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
      (diagnostic (code "recovered_interface_def_body_element") (severity error) (category parseerror) (span (offset 164) (line 10) (column 3) (len 90)) (message "unexpected token in interface definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 164) (line 10) (column 3) (len 90)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
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
  )
  (root (package (name "Interface Decomposition Example") (body brace (port-def (name "SpigotBank") (specializes none) (body semicolon)) (port-def (name "Spigot") (specializes none) (body semicolon)) (port-def (name "Faucet") (specializes none) (body semicolon)) (port-def (name "FaucetInlet") (specializes none) (body semicolon)) (interface-def (name "WaterDelivery") (specializes none) (body brace (malformed (code "recovered_interface_def_body_element") (found "end [1] port suppliedBy : SpigotBank {") (span (offset 164) (line 10) (column 3) (len 90))) (malformed (code "recovered_interface_def_body_element") (found "end [1..*] port deliveredTo : Faucet {") (span (offset 254) (line 14) (column 3) (len 103))) (connect) (connect))))))
)
~~~
