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
    (reference r1 (scope relative) (span (offset 263) (line 17) (column 12) (len 29)) (segments (segment 0 (token "illuminateRegion") (name "illuminateRegion") (separator none) (span (offset 263) (line 17) (column 12) (len 16))) (segment 1 (token "sendOnOffCmd") (name "sendOnOffCmd") (separator dot) (span (offset 280) (line 17) (column 29) (len 12)))))
    (reference r2 (scope relative) (span (offset 303) (line 18) (column 9) (len 8)) (segments (segment 0 (token "onOffCmd") (name "onOffCmd") (separator none) (span (offset 303) (line 18) (column 9) (len 8)))))
    (reference r3 (scope relative) (span (offset 314) (line 18) (column 20) (len 21)) (segments (segment 0 (token "onOffCmdPort") (name "onOffCmdPort") (separator none) (span (offset 314) (line 18) (column 20) (len 12))) (segment 1 (token "onOffCmd") (name "onOffCmd") (separator dot) (span (offset 327) (line 18) (column 33) (len 8)))))
    (reference r4 (scope relative) (span (offset 386) (line 22) (column 38) (len 17)) (segments (segment 0 (token "user") (name "user") (separator none) (span (offset 386) (line 22) (column 38) (len 4))) (segment 1 (token "onOffCmdPort") (name "onOffCmdPort") (separator dot) (span (offset 391) (line 22) (column 43) (len 12)))))
    (reference r5 (scope relative) (span (offset 407) (line 22) (column 59) (len 23)) (segments (segment 0 (token "flashlight") (name "flashlight") (separator none) (span (offset 407) (line 22) (column 59) (len 10))) (segment 1 (token "onOffCmdPort") (name "onOffCmdPort") (separator dot) (span (offset 418) (line 22) (column 70) (len 12)))))
    (reference r6 (scope relative) (span (offset 444) (line 23) (column 12) (len 29)) (segments (segment 0 (token "illuminateRegion") (name "illuminateRegion") (separator none) (span (offset 444) (line 23) (column 12) (len 16))) (segment 1 (token "onOffCmdFlow") (name "onOffCmdFlow") (separator dot) (span (offset 461) (line 23) (column 29) (len 12)))))
    (reference r7 (scope relative) (span (offset 526) (line 27) (column 24) (len 12)) (segments (segment 0 (token "OnOffCmdPort") (name "OnOffCmdPort") (separator none) (span (offset 526) (line 27) (column 24) (len 12)))))
    (reference r8 (scope relative) (span (offset 555) (line 29) (column 12) (len 37)) (segments (segment 0 (token "illuminateRegion") (name "illuminateRegion") (separator none) (span (offset 555) (line 29) (column 12) (len 16))) (segment 1 (token "produceDirectedLight") (name "produceDirectedLight") (separator dot) (span (offset 572) (line 29) (column 29) (len 20)))))
    (reference r9 (scope relative) (span (offset 602) (line 30) (column 8) (len 8)) (segments (segment 0 (token "onOffCmd") (name "onOffCmd") (separator none) (span (offset 602) (line 30) (column 8) (len 8)))))
    (reference r10 (scope relative) (span (offset 613) (line 30) (column 19) (len 21)) (segments (segment 0 (token "onOffCmdPort") (name "onOffCmdPort") (separator none) (span (offset 613) (line 30) (column 19) (len 12))) (segment 1 (token "onOffCmd") (name "onOffCmd") (separator dot) (span (offset 626) (line 30) (column 32) (len 8)))))
    (reference r11 (scope relative) (span (offset 644) (line 31) (column 9) (len 5)) (segments (segment 0 (token "light") (name "light") (separator none) (span (offset 644) (line 31) (column 9) (len 5)))))
    (reference r12 (scope relative) (span (offset 652) (line 31) (column 17) (len 15)) (segments (segment 0 (token "lightPort") (name "lightPort") (separator none) (span (offset 652) (line 31) (column 17) (len 9))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 662) (line 31) (column 27) (len 5)))))
    (reference r13 (scope relative) (span (offset 697) (line 34) (column 20) (len 9)) (segments (segment 0 (token "LightPort") (name "LightPort") (separator none) (span (offset 697) (line 34) (column 20) (len 9)))))
    (reference r14 (scope relative) (span (offset 759) (line 37) (column 21) (len 9)) (segments (segment 0 (token "LightPort") (name "LightPort") (separator none) (span (offset 759) (line 37) (column 21) (len 9)))))
    (reference r15 (scope relative) (span (offset 785) (line 39) (column 12) (len 29)) (segments (segment 0 (token "illuminateRegion") (name "illuminateRegion") (separator none) (span (offset 785) (line 39) (column 12) (len 16))) (segment 1 (token "reflectLight") (name "reflectLight") (separator dot) (span (offset 802) (line 39) (column 29) (len 12)))))
    (reference r16 (scope relative) (span (offset 824) (line 40) (column 8) (len 5)) (segments (segment 0 (token "light") (name "light") (separator none) (span (offset 824) (line 40) (column 8) (len 5)))))
    (reference r17 (scope relative) (span (offset 832) (line 40) (column 16) (len 15)) (segments (segment 0 (token "lightPort") (name "lightPort") (separator none) (span (offset 832) (line 40) (column 16) (len 9))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 842) (line 40) (column 26) (len 5)))))
    (reference r18 (scope relative) (span (offset 979) (line 48) (column 37) (len 21)) (segments (segment 0 (token "sendOnOffCmd") (name "sendOnOffCmd") (separator none) (span (offset 979) (line 48) (column 37) (len 12))) (segment 1 (token "onOffCmd") (name "onOffCmd") (separator dot) (span (offset 992) (line 48) (column 50) (len 8)))))
    (reference r19 (scope relative) (span (offset 1004) (line 48) (column 62) (len 29)) (segments (segment 0 (token "produceDirectedLight") (name "produceDirectedLight") (separator none) (span (offset 1004) (line 48) (column 62) (len 20))) (segment 1 (token "onOffCmd") (name "onOffCmd") (separator dot) (span (offset 1025) (line 48) (column 83) (len 8)))))
    (reference r20 (scope relative) (span (offset 1139) (line 52) (column 34) (len 26)) (segments (segment 0 (token "produceDirectedLight") (name "produceDirectedLight") (separator none) (span (offset 1139) (line 52) (column 34) (len 20))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 1160) (line 52) (column 55) (len 5)))))
    (reference r21 (scope relative) (span (offset 1169) (line 52) (column 64) (len 18)) (segments (segment 0 (token "reflectLight") (name "reflectLight") (separator none) (span (offset 1169) (line 52) (column 64) (len 12))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 1182) (line 52) (column 77) (len 5)))))
  )
  (root (package (name "Flashlight Example") (body brace (attribute-def (declaration-name "OnOffCmd") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Light") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (port-def (name "OnOffCmdPort") (modifiers) (specializes none) (body brace (in-out-declaration))) (port-def (name "LightPort") (modifiers) (specializes none) (body brace (in-out-declaration))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "user") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "onOffCmdPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r1)) (redefines none))) (value none) (body brace (binding (direction out) (target (ref r2)) (value (expression (span (offset 314) (line 18) (column 20) (len 21)) (ref r3)))))))) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r4)))) (to (interface-end (multiplicity none) (target (ref r5)))))) (body brace (perform (target (reference (action (ref r6)) (redefines none))) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "flashlight") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "onOffCmdPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r8)) (redefines none))) (value none) (body brace (binding (direction in) (target (ref r9)) (value (expression (span (offset 613) (line 30) (column 19) (len 21)) (ref r10)))) (binding (direction out) (target (ref r11)) (value (expression (span (offset 652) (line 31) (column 17) (len 15)) (ref r12)))))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lightPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "reflectingSource") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lightPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r15)) (redefines none))) (value none) (body brace (binding (direction in) (target (ref r16)) (value (expression (span (offset 832) (line 40) (column 16) (len 15)) (ref r17)))))))))) (action-usage (keyword action) (name "illuminateRegion") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (keyword action) (name "sendOnOffCmd") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))) (flow-usage (kind succession-flow) (visibility none) (declaration (declared (name "onOffCmdFlow") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r18)) (references none))) (to (connector-end (multiplicity none) (target (ref r19)) (references none)))))) (body (body semicolon))) (action-usage (keyword action) (name "produceDirectedLight") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (flow-usage (kind succession-flow) (visibility none) (declaration (declared (name "lightFlow") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r20)) (references none))) (to (connector-end (multiplicity none) (target (ref r21)) (references none)))))) (body (body semicolon))) (action-usage (keyword action) (name "reflectLight") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration))))))))
)
~~~
