# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Arrowhead Framework): AHFCoreLib"))
~~~
# SOURCE
~~~sysml
// /** Mandatory Services and Systems */
library package AHFCoreLib {
	private import AHFProfileLib::*;
	private import ScalarValues::*;
	private import AHFProfileMetadata::*;

	#service port def ServiceDiscovery {
		// The functionalities as Requests (Operations) cannot be defined yet
		// We could consider using flows to designate the functionalities
	}
	
	#service port def ServiceDiscoveryDD :> ServiceDiscovery{
	}
		
	#service port def Authorisation {
		attribute publickey:String; // just as examples
	}

	#service port def AuthorisationDD :> Authorisation{
	}

	
	#clouddd ArrowheadCore{
		// /** Design Level */
		// First the system definitions (SysD) of core systems
		
		#system service_registry {
			#service serviceDiscovery : ServiceDiscovery ;
		}
		
		#system authorization{
			#service authorisation : Authorisation;
			attribute protocol:String = "HTTP";
		}
		
		#system orchestrationDesign; // just indicated for now
		
		// /** Design Description level */		
		#systemdd service_registry_DD :> service_registry{
			#servicedd :>> serviceDiscovery:ServiceDiscoveryDD {
				#idd serviceDiscovery_HTTP ;// nested port for HTTP protocol
				// here we refer the functionalities like operation Register etc.
				#idd serviceDiscovery_MQTT ; // nested port for MQTT protocol
			}
		}
		
		#systemdd authorization_DD :> authorization{
			#servicedd :>> authorisation {
				#idd authorisation_HTTP ; // nested port for HTTP protocol
				#idd authorisation_MQTT ; // nested port for MQTT protocol
			}
			action Echo_behavior :> ServiceMethod;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfcore_lib.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package AHFCoreLib {
    private import AHFProfileLib::*;
    private import ScalarValues::*;
    private import AHFProfileMetadata::*;
    #service
    port def ServiceDiscovery {
    }
    #service
    port def ServiceDiscoveryDD :> ServiceDiscovery {
    }
    #service
    port def Authorisation {
        attribute publickey : String;
    }
    #service
    port def AuthorisationDD :> Authorisation {
    }
    #clouddd ArrowheadCore {
        #system service_registry {
            #service serviceDiscovery : ServiceDiscovery;
        }
        #system authorization {
            #service authorisation : Authorisation;
            attribute protocol : String = "HTTP";
        }
        #system orchestrationDesign;
        #systemdd service_registry_DD :> service_registry {
            #servicedd : ServiceDiscoveryDD :>> serviceDiscovery {
                #idd serviceDiscovery_HTTP;
                #idd serviceDiscovery_MQTT;
            }
        }
        #systemdd authorization_DD :> authorization {
            #servicedd :>> authorisation {
                #idd authorisation_HTTP;
                #idd authorisation_MQTT;
            }
            action Echo_behavior :> ServiceMethod;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 86) (line 3) (column 17) (len 13)) (segments (segment 0 (token "AHFProfileLib") (name "AHFProfileLib") (separator none) (span (offset 86) (line 3) (column 17) (len 13)))))
    (reference r1 (scope relative) (span (offset 120) (line 4) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 120) (line 4) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 153) (line 5) (column 17) (len 18)) (segments (segment 0 (token "AHFProfileMetadata") (name "AHFProfileMetadata") (separator none) (span (offset 153) (line 5) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 179) (line 7) (column 3) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 179) (line 7) (column 3) (len 7)))))
    (reference r4 (scope relative) (span (offset 362) (line 12) (column 3) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 362) (line 12) (column 3) (len 7)))))
    (reference r5 (scope relative) (span (offset 401) (line 12) (column 42) (len 16)) (segments (segment 0 (token "ServiceDiscovery") (name "ServiceDiscovery") (separator none) (span (offset 401) (line 12) (column 42) (len 16)))))
    (reference r6 (scope relative) (span (offset 427) (line 15) (column 3) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 427) (line 15) (column 3) (len 7)))))
    (reference r7 (scope relative) (span (offset 482) (line 16) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 482) (line 16) (column 23) (len 6)))))
    (reference r8 (scope relative) (span (offset 516) (line 19) (column 3) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 516) (line 19) (column 3) (len 7)))))
    (reference r9 (scope relative) (span (offset 552) (line 19) (column 39) (len 13)) (segments (segment 0 (token "Authorisation") (name "Authorisation") (separator none) (span (offset 552) (line 19) (column 39) (len 13)))))
    (reference r10 (scope relative) (span (offset 575) (line 23) (column 3) (len 7)) (segments (segment 0 (token "clouddd") (name "clouddd") (separator none) (span (offset 575) (line 23) (column 3) (len 7)))))
    (reference r11 (scope relative) (span (offset 686) (line 27) (column 4) (len 6)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 686) (line 27) (column 4) (len 6)))))
    (reference r12 (scope relative) (span (offset 716) (line 28) (column 5) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 716) (line 28) (column 5) (len 7)))))
    (reference r13 (scope relative) (span (offset 743) (line 28) (column 32) (len 16)) (segments (segment 0 (token "ServiceDiscovery") (name "ServiceDiscovery") (separator none) (span (offset 743) (line 28) (column 32) (len 16)))))
    (reference r14 (scope relative) (span (offset 772) (line 31) (column 4) (len 6)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 772) (line 31) (column 4) (len 6)))))
    (reference r15 (scope relative) (span (offset 798) (line 32) (column 5) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 798) (line 32) (column 5) (len 7)))))
    (reference r16 (scope relative) (span (offset 822) (line 32) (column 29) (len 13)) (segments (segment 0 (token "Authorisation") (name "Authorisation") (separator none) (span (offset 822) (line 32) (column 29) (len 13)))))
    (reference r17 (scope relative) (span (offset 859) (line 33) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 859) (line 33) (column 23) (len 6)))))
    (reference r18 (scope relative) (span (offset 886) (line 36) (column 4) (len 6)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 886) (line 36) (column 4) (len 6)))))
    (reference r19 (scope relative) (span (offset 985) (line 39) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 985) (line 39) (column 4) (len 8)))))
    (reference r20 (scope relative) (span (offset 1017) (line 39) (column 36) (len 16)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 1017) (line 39) (column 36) (len 16)))))
    (reference r21 (scope relative) (span (offset 1039) (line 40) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 1039) (line 40) (column 5) (len 9)))))
    (reference r22 (scope relative) (span (offset 1070) (line 40) (column 36) (len 18)) (segments (segment 0 (token "ServiceDiscoveryDD") (name "ServiceDiscoveryDD") (separator none) (span (offset 1070) (line 40) (column 36) (len 18)))))
    (reference r23 (scope relative) (span (offset 1053) (line 40) (column 19) (len 16)) (segments (segment 0 (token "serviceDiscovery") (name "serviceDiscovery") (separator none) (span (offset 1053) (line 40) (column 19) (len 16)))))
    (reference r24 (scope relative) (span (offset 1096) (line 41) (column 6) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 1096) (line 41) (column 6) (len 3)))))
    (reference r25 (scope relative) (span (offset 1231) (line 43) (column 6) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 1231) (line 43) (column 6) (len 3)))))
    (reference r26 (scope relative) (span (offset 1307) (line 47) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 1307) (line 47) (column 4) (len 8)))))
    (reference r27 (scope relative) (span (offset 1336) (line 47) (column 33) (len 13)) (segments (segment 0 (token "authorization") (name "authorization") (separator none) (span (offset 1336) (line 47) (column 33) (len 13)))))
    (reference r28 (scope relative) (span (offset 1355) (line 48) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 1355) (line 48) (column 5) (len 9)))))
    (reference r29 (scope relative) (span (offset 1369) (line 48) (column 19) (len 13)) (segments (segment 0 (token "authorisation") (name "authorisation") (separator none) (span (offset 1369) (line 48) (column 19) (len 13)))))
    (reference r30 (scope relative) (span (offset 1390) (line 49) (column 6) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 1390) (line 49) (column 6) (len 3)))))
    (reference r31 (scope relative) (span (offset 1453) (line 50) (column 6) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 1453) (line 50) (column 6) (len 3)))))
  )
  (root (library-package (name "AHFCoreLib") (standard false) (body brace (import (target (span (span (offset 86) (line 3) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 99) (line 3) (column 30) (len 3))) (separator (span (offset 99) (line 3) (column 30) (len 2))) (marker (span (offset 101) (line 3) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 120) (line 4) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 132) (line 4) (column 29) (len 3))) (separator (span (offset 132) (line 4) (column 29) (len 2))) (marker (span (offset 134) (line 4) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 153) (line 5) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 171) (line 5) (column 35) (len 3))) (separator (span (offset 171) (line 5) (column 35) (len 2))) (marker (span (offset 173) (line 5) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-keyword-usage (type (ref r3)) (body none)) (port-def (name "ServiceDiscovery") (modifiers) (specializes none) (body brace)) (metadata-keyword-usage (type (ref r4)) (body none)) (port-def (name "ServiceDiscoveryDD") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (body brace)) (metadata-keyword-usage (type (ref r6)) (body none)) (port-def (name "Authorisation") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "publickey") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-keyword-usage (type (ref r8)) (body none)) (port-def (name "AuthorisationDD") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r9)))) (body brace)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r10)) (declaration (name "ArrowheadCore") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "service_registry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r12)) (declaration (name "serviceDiscovery") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "authorization") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r15)) (declaration (name "authorisation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (attribute-usage (declaration-name "protocol") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 868) (line 33) (column 32) (len 6)) (string "HTTP"))))) (body semicolon)))) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r18)) (declaration (name "orchestrationDesign") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r19)) (declaration (name "service_registry_DD") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r20))) (value none)) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r21)) (declaration (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r24)) (declaration (name "serviceDiscovery_HTTP") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r25)) (declaration (name "serviceDiscovery_MQTT") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r26)) (declaration (name "authorization_DD") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r27))) (value none)) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r28)) (declaration (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none)) (value none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r30)) (declaration (name "authorisation_HTTP") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r31)) (declaration (name "authorisation_MQTT") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (action-usage))))))))
)
~~~
