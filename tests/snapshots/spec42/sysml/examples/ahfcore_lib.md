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
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 1038) (line 40) (column 4) (len 261)) (message "incomplete parser support for annotation syntax in package body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 1354) (line 48) (column 4) (len 165)) (message "incomplete parser support for annotation syntax in package body"))
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
            #service
            serviceDiscovery : ServiceDiscovery;
        }
        #system authorization {
            #service
            authorisation : Authorisation;
            attribute def protocol : String = "HTTP";
        }
        #system orchestrationDesign;
        #systemdd service_registry_DD :> service_registry {
            #servicedd :>> serviceDiscovery:ServiceDiscoveryDD {
				#idd serviceDiscovery_HTTP ;// nested port for HTTP protocol
				// here we refer the functionalities like operation Register etc.
				#idd serviceDiscovery_MQTT ; // nested port for MQTT protocol
			}
        }
        #systemdd authorization_DD :> authorization {
            #servicedd :>> authorisation {
				#idd authorisation_HTTP ; // nested port for HTTP protocol
				#idd authorisation_MQTT ; // nested port for MQTT protocol
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
    (reference r3 (scope relative) (span (offset 401) (line 12) (column 42) (len 16)) (segments (segment 0 (token "ServiceDiscovery") (name "ServiceDiscovery") (separator none) (span (offset 401) (line 12) (column 42) (len 16)))))
    (reference r4 (scope relative) (span (offset 482) (line 16) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 482) (line 16) (column 23) (len 6)))))
    (reference r5 (scope relative) (span (offset 552) (line 19) (column 39) (len 13)) (segments (segment 0 (token "Authorisation") (name "Authorisation") (separator none) (span (offset 552) (line 19) (column 39) (len 13)))))
    (reference r6 (scope relative) (span (offset 1017) (line 39) (column 36) (len 16)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 1017) (line 39) (column 36) (len 16)))))
    (reference r7 (scope relative) (span (offset 1336) (line 47) (column 33) (len 13)) (segments (segment 0 (token "authorization") (name "authorization") (separator none) (span (offset 1336) (line 47) (column 33) (len 13)))))
  )
  (root (library-package (name "AHFCoreLib") (standard false) (body (import (target (span (span (offset 86) (line 3) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 99) (line 3) (column 30) (len 3))) (separator (span (offset 99) (line 3) (column 30) (len 2))) (marker (span (offset 101) (line 3) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 120) (line 4) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 132) (line 4) (column 29) (len 3))) (separator (span (offset 132) (line 4) (column 29) (len 2))) (marker (span (offset 134) (line 4) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 153) (line 5) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 171) (line 5) (column 35) (len 3))) (separator (span (offset 171) (line 5) (column 35) (len 2))) (marker (span (offset 173) (line 5) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-keyword-usage) (port-def (name "ServiceDiscovery") (specializes none) (body )) (metadata-keyword-usage) (port-def (name "ServiceDiscoveryDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3)))) (body )) (metadata-keyword-usage) (port-def (name "Authorisation") (specializes none) (body (attribute-usage (declaration-name "publickey") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-keyword-usage) (port-def (name "AuthorisationDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (body )) (extended-def (prefix-keywords ("clouddd")) (definition-prefix none) (def false) (name "ArrowheadCore") (specializes none) (body (extended-def (prefix-keywords ("system")) (definition-prefix none) (def false) (name "service_registry") (specializes none) (body (metadata-keyword-usage) (default-reference-usage))) (extended-def (prefix-keywords ("system")) (definition-prefix none) (def false) (name "authorization") (specializes none) (body (metadata-keyword-usage) (default-reference-usage) (attribute-def))) (extended-def (prefix-keywords ("system")) (definition-prefix none) (def false) (name "orchestrationDesign") (specializes none) (body semicolon)) (extended-def (prefix-keywords ("systemdd")) (definition-prefix none) (def false) (name "service_registry_DD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6)))) (body (malformed (code "unsupported_annotation_syntax") (found "#servicedd :>> serviceDiscovery:ServiceDiscoveryDD {") (span (offset 1038) (line 40) (column 4) (len 261))))) (extended-def (prefix-keywords ("systemdd")) (definition-prefix none) (def false) (name "authorization_DD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (body (malformed (code "unsupported_annotation_syntax") (found "#servicedd :>> authorisation {") (span (offset 1354) (line 48) (column 4) (len 165))) (action-usage))))))))
)
~~~
