# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Flashlight): Flashlight Example"))
~~~
# SOURCE
~~~sysml
package 'Flashlight Example' {
	
	attribute def OnOffCmd;
	attribute def Light;
	
	port def OnOffCmdPort {
		out onOffCmd : OnOffCmd;
	}
	
	port def LightPort {
		out light: Light;
	}
	
	part context {
		part user {
			port onOffCmdPort: OnOffCmdPort;
			perform illuminateRegion.sendOnOffCmd {
				out onOffCmd = onOffCmdPort.onOffCmd;
			}
		}
		
		interface userToFlashlight connect user.onOffCmdPort to flashlight.onOffCmdPort {
			perform illuminateRegion.onOffCmdFlow; 
		}
		
		part flashlight {
			port onOffCmdPort: ~OnOffCmdPort;
			
			perform illuminateRegion.produceDirectedLight {
				in onOffCmd = onOffCmdPort.onOffCmd;
				out light = lightPort.light;
			}
			
			port lightPort: LightPort ;
		}
		part reflectingSource {
			port lightPort: ~LightPort;
			
			perform illuminateRegion.reflectLight {
				in light = lightPort.light;
			}
		}
	}
	
	action illuminateRegion {
		action sendOnOffCmd { out onOffCmd: OnOffCmd; }
		
		succession flow onOffCmdFlow from sendOnOffCmd.onOffCmd to produceDirectedLight.onOffCmd;
		
		action produceDirectedLight { in onOffCmd; out light: Light; }
		
		succession flow lightFlow from produceDirectedLight.light to reflectLight.light;
		
		action reflectLight { in light: Light; }
	}
	
	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flashlight_example.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 351) (line 22) (column 3) (len 134)) (message "unexpected token in part usage body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Flashlight Example' {
    attribute def OnOffCmd;
    attribute def Light;
    port def OnOffCmdPort {
        out onOffCmd : OnOffCmd;
    }
    port def LightPort {
        out light : Light;
    }
    part context {
        part user {
            port onOffCmdPort : OnOffCmdPort;
            perform illuminateRegion.sendOnOffCmd {
                out onOffCmd = onOffCmdPort.onOffCmd;
            }
        }
        interface userToFlashlight connect user.onOffCmdPort to flashlight.onOffCmdPort {
			perform illuminateRegion.onOffCmdFlow; 
		}
        part flashlight {
            port onOffCmdPort : ~OnOffCmdPort;
            perform illuminateRegion.produceDirectedLight {
                in onOffCmd = onOffCmdPort.onOffCmd;
                out light = lightPort.light;
            }
            port lightPort : LightPort;
        }
        part reflectingSource {
            port lightPort : ~LightPort;
            perform illuminateRegion.reflectLight {
                in light = lightPort.light;
            }
        }
    }
    action illuminateRegion {
        action sendOnOffCmd {
            out onOffCmd : OnOffCmd;
        }
        succession flow onOffCmdFlow from sendOnOffCmd.onOffCmd to produceDirectedLight.onOffCmd;
        action produceDirectedLight {
            in onOffCmd;
            out light : Light;
        }
        succession flow lightFlow from produceDirectedLight.light to reflectLight.light;
        action reflectLight {
            in light : Light;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 238) (line 16) (column 23) (len 12)) (segments (segment 0 (token "OnOffCmdPort") (name "OnOffCmdPort") (separator none) (span (offset 238) (line 16) (column 23) (len 12)))))
    (reference r1 (scope relative) (span (offset 526) (line 27) (column 24) (len 12)) (segments (segment 0 (token "OnOffCmdPort") (name "OnOffCmdPort") (separator none) (span (offset 526) (line 27) (column 24) (len 12)))))
    (reference r2 (scope relative) (span (offset 697) (line 34) (column 20) (len 9)) (segments (segment 0 (token "LightPort") (name "LightPort") (separator none) (span (offset 697) (line 34) (column 20) (len 9)))))
    (reference r3 (scope relative) (span (offset 759) (line 37) (column 21) (len 9)) (segments (segment 0 (token "LightPort") (name "LightPort") (separator none) (span (offset 759) (line 37) (column 21) (len 9)))))
  )
  (root (package (name "Flashlight Example") (body brace (attribute-def (declaration-name "OnOffCmd") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Light") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (port-def (name "OnOffCmdPort") (specializes none) (body brace (in-out-declaration))) (port-def (name "LightPort") (specializes none) (body brace (in-out-declaration))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "user") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "onOffCmdPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform))) (malformed (code "recovered_part_usage_body_element") (found "interface userToFlashlight connect user.onOffCmdPort to flas") (span (offset 351) (line 22) (column 3) (len 134))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "flashlight") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "onOffCmdPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lightPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "reflectingSource") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lightPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform))))) (action-usage (name "illuminateRegion") (short-name none) (body brace (action-usage (name "sendOnOffCmd") (short-name none) (body brace (in-out-declaration))) (flow-usage) (action-usage (name "produceDirectedLight") (short-name none) (body brace (in-out-declaration) (in-out-declaration))) (flow-usage) (action-usage (name "reflectLight") (short-name none) (body brace (in-out-declaration))))))))
)
~~~
