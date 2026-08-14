# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Arrowhead Framework): AHFNorwayTopics"))
~~~
# SOURCE
~~~sysml
package AHFNorway {
	doc /* This is the Norwegian use-case for Arrowhead Framework */
	// The use-case is for Productive4.0 and Arrowhead Tools
	// The system is taken from a chemical factory
	// This is focusing on the monitoring of products when delivered
	private import AHFProfileLib::*;
	private import AHFProfileMetadata::*;
	private import AHFCoreLib::**;
	private import ScalarValues::*;

	#service def APISService {
		doc /* Service design */		

		attribute :>> serviceDefinition = "APISPullService";
		attribute :>> intrfce_protocol = "{JSON}";
		attribute :>> serviceURL = "pull";
	}

	#servicedd port def APIS_DD :> APISService {
		doc /* Service design description with nested protocol-specific ports */	

		#idd port APIS_HTTP {
			// the asynch implementation of synchronous remote calls
			out cll:CallGiveItems;
			in retrn:ResultGiveItems;
		}
		
		#idd port APIS_MQTT  {
			// GetAllItems functionality
			out pub:Publish;
			out retall:Return_AllItems;
			in subscr:Subscribe;
		}
	}
	
	// Asynchronous signals
	attribute def Publish {nametopic:String;}
	attribute def Subscribe{nametopic:String;}
	attribute def Return_AllItems {itms:String;}
	attribute def Subscribe_giveItems{itms:String;}
	attribute def Return_Ack{ack:Boolean;}
	
	// Signals for implementing the remote procedure call by asynch signals
	attribute def CallGiveItems{itms:String; } 
	attribute def ResultGiveItems{ack:Boolean;}
	
	#clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {	
		#systemdd TellUConsumer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd apisp:APIS_DD ;
			
			attribute :>> systemname = "UngerApisClient";
			attribute :>> address = "Unger_network_ip";
			attribute :>> portno = 0;
						
			// We want an operation call to GiveItems, and actually sending the payload
			// Call apisp::APIS_HTTP::giveItems(in allitems: String = "All the items", out ackback:Boolean);
			
			state TellUbehavior{
				entry send new CallGiveItems("All the items") via apisp.APIS_HTTP;
				then Wait;
				state Wait;
					accept rs:ResultGiveItems
					// Here do whatever about the result rs.ret 
				then Wait;
			}
						
		}
		
		#systemdd APISProducer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd tellu:~APIS_DD; // providing the APISService
			#servicedd apisc:APIS_DD ; // talking to APISConsumer
			
			:>> systemname = "PrediktorApisServer";
			:>> address = "Prediktor_network_ip";
			:>> portno = 6565;
			attribute x:Boolean;
			
			action giveItems :> ServiceMethod
			 {  in itms:String; out ack:Boolean;
			 	/* Forward itms and return an ack */
			 	first start;
			 	then send new Return_AllItems(itms) via apisc.APIS_MQTT;
			 	success = true;
			 	bind ack = success;
			 }
			
			state APISPbehavior{
				entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
				then WaitOnData; 
				
				state WaitOnData;
					accept cl:CallGiveItems via tellu.APIS_HTTP
					do action {
						first start;
						then action giveItems{ in itms=cl.itms; out ack=x; }
						then send new ResultGiveItems(x) via tellu.APIS_HTTP;
					}
				then WaitOnData;		
			}
		}
		
		#systemdd APISConsumer {
			#servicedd serviceDiscovery:~ServiceDiscovery ; // communicating with ServiceRegistry
			#servicedd apisp:~APIS_DD ;
			:>> systemname = "TellUClient";
			:>> address = "Prediktor_network_ip";
			:>> portno = 1;
			
			// Now sending signal to the remote behavior through the port functionality
			state MQTT_APISP {
				entry send new Subscribe("Return_AllItems") via apisp.APIS_MQTT; 
				then Idle;		
				state Idle;
					accept Return_AllItems via apisp.APIS_MQTT
					// Get the stuff and do something with them
					then Idle;
			}
		}
 		
 		part MQTTServer {
 			port getTopic:~APIS_DD;
 			port giveTopic:APIS_DD;
 			
 			state Serve{				
 				entry;
 				then Publ;
 				state Publ;
 					accept pub:Publish via getTopic.APIS_MQTT
 					// store information about who will provide "Publish::nametopic"
 				then Subsr;
 				
 				state Subsr;
 					accept Subscribe via giveTopic.APIS_MQTT
 					// store information about who want to receive "Subscribe::nametopic"
 				then Idle;
 				
 				state Idle;
 					accept retrnall:Return_AllItems via getTopic.APIS_MQTT
 					do send retrnall via giveTopic.APIS_MQTT
 				then Idle;
 			} 			
 		}
 				
 		connect APISProducer.apisc to MQTTServer.getTopic; 
 		connect MQTTServer.giveTopic to APISConsumer.apisp; 
		
 		connect TellUConsumer.apisp to APISProducer.tellu; 
 		
 		// Then we need to connect the application systems to the mandatory systems
 		connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		
 		// Same procedure for the other mandatory services
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfnorway_topics.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1515) (line 49) (column 15) (len 80)) (message "unrecognized declaration `serviceDiscovery` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 2223) (line 71) (column 15) (len 80)) (message "unrecognized declaration `serviceDiscovery` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 2314) (line 72) (column 15) (len 48)) (message "unrecognized declaration `tellu` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 3236) (line 105) (column 15) (len 78)) (message "unrecognized declaration `serviceDiscovery` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 3325) (line 106) (column 15) (len 198)) (message "unrecognized declaration `apisp` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(unavailable (reason opaque-ast))
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 274) (line 6) (column 17) (len 13)) (segments (segment 0 (token "AHFProfileLib") (name "AHFProfileLib") (separator none) (span (offset 274) (line 6) (column 17) (len 13)))))
    (reference r1 (scope relative) (span (offset 308) (line 7) (column 17) (len 18)) (segments (segment 0 (token "AHFProfileMetadata") (name "AHFProfileMetadata") (separator none) (span (offset 308) (line 7) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 347) (line 8) (column 17) (len 10)) (segments (segment 0 (token "AHFCoreLib") (name "AHFCoreLib") (separator none) (span (offset 347) (line 8) (column 17) (len 10)))))
    (reference r3 (scope relative) (span (offset 379) (line 9) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 379) (line 9) (column 17) (len 12)))))
    (reference r4 (scope relative) (span (offset 628) (line 19) (column 33) (len 11)) (segments (segment 0 (token "APISService") (name "APISService") (separator none) (span (offset 628) (line 19) (column 33) (len 11)))))
    (reference r5 (scope relative) (span (offset 1456) (line 47) (column 37) (len 13)) (segments (segment 0 (token "ArrowheadCore") (name "ArrowheadCore") (separator none) (span (offset 1456) (line 47) (column 37) (len 13)))))
    (reference r6 (scope relative) (span (offset 4397) (line 146) (column 12) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4397) (line 146) (column 12) (len 12))) (segment 1 (token "apisc") (name "apisc") (separator dot) (span (offset 4410) (line 146) (column 25) (len 5)))))
    (reference r7 (scope relative) (span (offset 4419) (line 146) (column 34) (len 19)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4419) (line 146) (column 34) (len 10))) (segment 1 (token "getTopic") (name "getTopic") (separator dot) (span (offset 4430) (line 146) (column 45) (len 8)))))
    (reference r8 (scope relative) (span (offset 4452) (line 147) (column 12) (len 20)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4452) (line 147) (column 12) (len 10))) (segment 1 (token "giveTopic") (name "giveTopic") (separator dot) (span (offset 4463) (line 147) (column 23) (len 9)))))
    (reference r9 (scope relative) (span (offset 4476) (line 147) (column 36) (len 18)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4476) (line 147) (column 36) (len 12))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4489) (line 147) (column 49) (len 5)))))
    (reference r10 (scope relative) (span (offset 4511) (line 149) (column 12) (len 19)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4511) (line 149) (column 12) (len 13))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4525) (line 149) (column 26) (len 5)))))
    (reference r11 (scope relative) (span (offset 4534) (line 149) (column 35) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4534) (line 149) (column 35) (len 12))) (segment 1 (token "tellu") (name "tellu") (separator dot) (span (offset 4547) (line 149) (column 48) (len 5)))))
    (reference r12 (scope relative) (span (offset 4649) (line 152) (column 12) (len 29)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4649) (line 152) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4662) (line 152) (column 25) (len 16)))))
    (reference r13 (scope relative) (span (offset 4682) (line 152) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4682) (line 152) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4699) (line 152) (column 62) (len 16)))))
    (reference r14 (scope relative) (span (offset 4728) (line 153) (column 12) (len 30)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4728) (line 153) (column 12) (len 13))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4742) (line 153) (column 26) (len 16)))))
    (reference r15 (scope relative) (span (offset 4762) (line 153) (column 46) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4762) (line 153) (column 46) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4779) (line 153) (column 63) (len 16)))))
    (reference r16 (scope relative) (span (offset 4808) (line 154) (column 12) (len 29)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4808) (line 154) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4821) (line 154) (column 25) (len 16)))))
    (reference r17 (scope relative) (span (offset 4841) (line 154) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4841) (line 154) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4858) (line 154) (column 62) (len 16)))))
  )
  (root (package (name "AHFNorway") (body (doc) (import (target (span (span (offset 274) (line 6) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 287) (line 6) (column 30) (len 3))) (separator (span (offset 287) (line 6) (column 30) (len 2))) (marker (span (offset 289) (line 6) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 308) (line 7) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 326) (line 7) (column 35) (len 3))) (separator (span (offset 326) (line 7) (column 35) (len 2))) (marker (span (offset 328) (line 7) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 347) (line 8) (column 17) (len 14))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 357) (line 8) (column 27) (len 4))) (separator (span (offset 357) (line 8) (column 27) (len 2))) (marker (span (offset 359) (line 8) (column 29) (len 2)))))))) (import (target (span (span (offset 379) (line 9) (column 17) (len 15))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 391) (line 9) (column 29) (len 3))) (separator (span (offset 391) (line 9) (column 29) (len 2))) (marker (span (offset 393) (line 9) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (extended-def (prefix-keywords ("service")) (definition-prefix none) (def true) (name "APISService") (specializes none) (body (doc) (attribute-usage) (attribute-usage) (attribute-usage))) (metadata-keyword-usage) (port-def (name "APIS_DD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body (doc) (metadata-keyword-usage) (port-usage (declaration-name "APIS_HTTP") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 2))) (metadata-keyword-usage) (port-usage (declaration-name "APIS_MQTT") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 3))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (extended-def (prefix-keywords ("clouddd")) (definition-prefix none) (def false) (name "AHFNorway_LocalCloudDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (body (extended-def (prefix-keywords ("systemdd")) (definition-prefix none) (def false) (name "TellUConsumer") (specializes none) (body (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscoveryDD ; // communicating with") (span (offset 1515) (line 49) (column 15) (len 80))) (metadata-keyword-usage) (default-reference-usage) (attribute-usage) (attribute-usage) (attribute-usage) (state-usage))) (extended-def (prefix-keywords ("systemdd")) (definition-prefix none) (def false) (name "APISProducer") (specializes none) (body (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscoveryDD ; // communicating with") (span (offset 2223) (line 71) (column 15) (len 80))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "tellu:~APIS_DD; // providing the APISService") (span (offset 2314) (line 72) (column 15) (len 48))) (metadata-keyword-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (attribute-def) (action-usage) (state-usage))) (extended-def (prefix-keywords ("systemdd")) (definition-prefix none) (def false) (name "APISConsumer") (specializes none) (body (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscovery ; // communicating with S") (span (offset 3236) (line 105) (column 15) (len 78))) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "apisp:~APIS_DD ;") (span (offset 3325) (line 106) (column 15) (len 198))) (state-usage))) (part-usage) (connect (from (expression (span (offset 4397) (line 146) (column 12) (len 18)) (ref r6))) (to (expression (span (offset 4419) (line 146) (column 34) (len 19)) (ref r7))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4452) (line 147) (column 12) (len 20)) (ref r8))) (to (expression (span (offset 4476) (line 147) (column 36) (len 18)) (ref r9))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4511) (line 149) (column 12) (len 19)) (ref r10))) (to (expression (span (offset 4534) (line 149) (column 35) (len 18)) (ref r11))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4649) (line 152) (column 12) (len 29)) (ref r12))) (to (expression (span (offset 4682) (line 152) (column 45) (len 33)) (ref r13))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4728) (line 153) (column 12) (len 30)) (ref r14))) (to (expression (span (offset 4762) (line 153) (column 46) (len 33)) (ref r15))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4808) (line 154) (column 12) (len 29)) (ref r16))) (to (expression (span (offset 4841) (line 154) (column 45) (len 33)) (ref r17))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
