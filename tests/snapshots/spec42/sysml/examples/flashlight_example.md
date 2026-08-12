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
  )
  (root (package (name "Flashlight Example") (body (attribute-def) (attribute-def) (port-def (name "OnOffCmdPort") (specializes none) (body (in-out-declaration))) (port-def (name "LightPort") (specializes none) (body (in-out-declaration))) (part-usage) (action-usage))))
)
~~~
