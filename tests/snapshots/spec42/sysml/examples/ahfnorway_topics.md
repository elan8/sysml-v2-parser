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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 1430) (line 47) (column 11) (len 3512)) (message "unrecognized declaration `AHFNorway_LocalCloudDD` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AHFNorway {
    doc
    /* This is the Norwegian use-case for Arrowhead Framework */
    private import AHFProfileLib::*;
    private import AHFProfileMetadata::*;
    private import AHFCoreLib::**;
    private import ScalarValues::*;
    #service def APISService {
        doc
        /* Service design */
        attribute  :>> serviceDefinition = "APISPullService";
        attribute  :>> intrfce_protocol = "{JSON}";
        attribute  :>> serviceURL = "pull";
    }
    #servicedd;
    port def APIS_DD :> APISService {
        doc
        /* Service design description with nested protocol-specific ports */
        #idd;
        port APIS_HTTP {
            out cll : CallGiveItems;
            in retrn : ResultGiveItems;
        }
        #idd;
        port APIS_MQTT {
            out pub : Publish;
            out retall : Return_AllItems;
            in subscr : Subscribe;
        }
    }
    attribute def Publish {
        attribute nametopic : String;
    }
    attribute def Subscribe {
        attribute nametopic : String;
    }
    attribute def Return_AllItems {
        attribute itms : String;
    }
    attribute def Subscribe_giveItems {
        attribute itms : String;
    }
    attribute def Return_Ack {
        attribute ack : Boolean;
    }
    attribute def CallGiveItems {
        attribute itms : String;
    }
    attribute def ResultGiveItems {
        attribute ack : Boolean;
    }
    #clouddd;
    AHFNorway_LocalCloudDD :> ArrowheadCore {	
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 274) (line 6) (column 17) (len 13)) (segments (segment 0 (token "AHFProfileLib") (name "AHFProfileLib") (separator none) (span (offset 274) (line 6) (column 17) (len 13)))))
    (reference r1 (scope relative) (span (offset 308) (line 7) (column 17) (len 18)) (segments (segment 0 (token "AHFProfileMetadata") (name "AHFProfileMetadata") (separator none) (span (offset 308) (line 7) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 347) (line 8) (column 17) (len 10)) (segments (segment 0 (token "AHFCoreLib") (name "AHFCoreLib") (separator none) (span (offset 347) (line 8) (column 17) (len 10)))))
    (reference r3 (scope relative) (span (offset 379) (line 9) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 379) (line 9) (column 17) (len 12)))))
    (reference r4 (scope relative) (span (offset 628) (line 19) (column 33) (len 11)) (segments (segment 0 (token "APISService") (name "APISService") (separator none) (span (offset 628) (line 19) (column 33) (len 11)))))
  )
  (root (package (name "AHFNorway") (body (doc) (import (target (span (span (offset 274) (line 6) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 287) (line 6) (column 30) (len 3))) (separator (span (offset 287) (line 6) (column 30) (len 2))) (marker (span (offset 289) (line 6) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 308) (line 7) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 326) (line 7) (column 35) (len 3))) (separator (span (offset 326) (line 7) (column 35) (len 2))) (marker (span (offset 328) (line 7) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 347) (line 8) (column 17) (len 14))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 357) (line 8) (column 27) (len 4))) (separator (span (offset 357) (line 8) (column 27) (len 2))) (marker (span (offset 359) (line 8) (column 29) (len 2)))))))) (import (target (span (span (offset 379) (line 9) (column 17) (len 15))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 391) (line 9) (column 29) (len 3))) (separator (span (offset 391) (line 9) (column 29) (len 2))) (marker (span (offset 393) (line 9) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (extended-def (prefix-keywords ("service")) (definition-prefix none) (name "APISService") (specializes none) (body (doc) (attribute-usage) (attribute-usage) (attribute-usage))) (metadata-keyword-usage) (port-def (name "APIS_DD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body (doc) (metadata-keyword-usage) (port-usage (declaration-name "APIS_HTTP") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 2))) (metadata-keyword-usage) (port-usage (declaration-name "APIS_MQTT") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 3))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (metadata-keyword-usage) (malformed (code "unrecognized_declaration_in_scope") (found "AHFNorway_LocalCloudDD :> ArrowheadCore {") (span (offset 1430) (line 47) (column 11) (len 3512))))))
)
~~~
