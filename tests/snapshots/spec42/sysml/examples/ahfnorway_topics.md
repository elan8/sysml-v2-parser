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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 2220) (line 71) (column 15) (len 80)) (message "unrecognized declaration `serviceDiscovery` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 2311) (line 72) (column 15) (len 48)) (message "unrecognized declaration `tellu` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 3233) (line 105) (column 15) (len 78)) (message "unrecognized declaration `serviceDiscovery` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 3322) (line 106) (column 15) (len 195)) (message "unrecognized declaration `apisp` in package body"))
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
        attribute :>> serviceDefinition = "APISPullService";
        attribute :>> intrfce_protocol = "{JSON}";
        attribute :>> serviceURL = "pull";
    }
    #servicedd
    port def APIS_DD :> APISService {
        doc
        /* Service design description with nested protocol-specific ports */
        #idd port APIS_HTTP {
            out cll : CallGiveItems;
            in retrn : ResultGiveItems;
        }
        #idd port APIS_MQTT {
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
    #clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {
        #systemdd TellUConsumer {
            #servicedd
            serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
            #servicedd
            apisp : APIS_DD;
            attribute :>> systemname = "UngerApisClient";
            attribute :>> address = "Unger_network_ip";
            attribute :>> portno = 0;
            state TellUbehavior {
                entry send new CallGiveItems("All the items") via apisp.APIS_HTTP;
                then Wait;
                state Wait;
                transition accept rs : ResultGiveItems then Wait;
            }
        }
        #systemdd APISProducer {
            #servicedd
            serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
            #servicedd
            tellu:~APIS_DD; // providing the APISService
            #servicedd
            apisc : APIS_DD;
            '' :>> systemname = "PrediktorApisServer";
            '' :>> address = "Prediktor_network_ip";
            '' :>> portno = 6565;
            attribute def x : Boolean;
            action giveItems :> ServiceMethod {
                in itms : String;
                out ack : Boolean;
                first start;
                then send new Return_AllItems(itms) via apisc.APIS_MQTT;
                success = true;
                bind ack = success;
            }
            state APISPbehavior {
                entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
                then WaitOnData;
                state WaitOnData;
                transition accept cl : CallGiveItems via tellu.APIS_HTTP do 'action' then WaitOnData;
            }
        }
        #systemdd APISConsumer {
            #servicedd
            serviceDiscovery:~ServiceDiscovery ; // communicating with ServiceRegistry
            #servicedd
            apisp:~APIS_DD ;
			:>> systemname = "TellUClient";
			:>> address = "Prediktor_network_ip";
			:>> portno = 1;

			// Now sending signal to the remote behavior through the port functionality
            state MQTT_APISP {
                entry send new Subscribe("Return_AllItems") via apisp.APIS_MQTT;
                then Idle;
                state Idle;
                transition accept Return_AllItems via apisp.APIS_MQTT then Idle;
            }
        }
        part MQTTServer {
            port getTopic : ~APIS_DD;
            port giveTopic : APIS_DD;
            state Serve {
                entry;
                then Publ;
                state Publ;
                transition accept pub : Publish via getTopic.APIS_MQTT then Subsr;
                state Subsr;
                transition accept Subscribe via giveTopic.APIS_MQTT then Idle;
                state Idle;
                transition accept retrnall : Return_AllItems via getTopic.APIS_MQTT do send retrnall via giveTopic.APIS_MQTT then Idle;
            }
        }
        connect APISProducer.apisc to MQTTServer.getTopic;
        connect MQTTServer.giveTopic to APISConsumer.apisp;
        connect TellUConsumer.apisp to APISProducer.tellu;
        connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
        connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
        connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;
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
    (reference r4 (scope relative) (span (offset 399) (line 11) (column 3) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 399) (line 11) (column 3) (len 7)))))
    (reference r5 (scope relative) (span (offset 598) (line 19) (column 3) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 598) (line 19) (column 3) (len 9)))))
    (reference r6 (scope relative) (span (offset 628) (line 19) (column 33) (len 11)) (segments (segment 0 (token "APISService") (name "APISService") (separator none) (span (offset 628) (line 19) (column 33) (len 11)))))
    (reference r7 (scope relative) (span (offset 722) (line 22) (column 4) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 722) (line 22) (column 4) (len 3)))))
    (reference r8 (scope relative) (span (offset 868) (line 28) (column 4) (len 3)) (segments (segment 0 (token "idd") (name "idd") (separator none) (span (offset 868) (line 28) (column 4) (len 3)))))
    (reference r9 (scope relative) (span (offset 1065) (line 37) (column 35) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1065) (line 37) (column 35) (len 6)))))
    (reference r10 (scope relative) (span (offset 1109) (line 38) (column 36) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1109) (line 38) (column 36) (len 6)))))
    (reference r11 (scope relative) (span (offset 1155) (line 39) (column 38) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1155) (line 39) (column 38) (len 6)))))
    (reference r12 (scope relative) (span (offset 1204) (line 40) (column 41) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1204) (line 40) (column 41) (len 6)))))
    (reference r13 (scope relative) (span (offset 1243) (line 41) (column 31) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1243) (line 41) (column 31) (len 7)))))
    (reference r14 (scope relative) (span (offset 1362) (line 44) (column 35) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1362) (line 44) (column 35) (len 6)))))
    (reference r15 (scope relative) (span (offset 1408) (line 45) (column 36) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1408) (line 45) (column 36) (len 7)))))
    (reference r16 (scope relative) (span (offset 1422) (line 47) (column 3) (len 7)) (segments (segment 0 (token "clouddd") (name "clouddd") (separator none) (span (offset 1422) (line 47) (column 3) (len 7)))))
    (reference r17 (scope relative) (span (offset 1456) (line 47) (column 37) (len 13)) (segments (segment 0 (token "ArrowheadCore") (name "ArrowheadCore") (separator none) (span (offset 1456) (line 47) (column 37) (len 13)))))
    (reference r18 (scope relative) (span (offset 1476) (line 48) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 1476) (line 48) (column 4) (len 8)))))
    (reference r19 (scope relative) (span (offset 1505) (line 49) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 1505) (line 49) (column 5) (len 9)))))
    (reference r20 (scope relative) (span (offset 1596) (line 50) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 1596) (line 50) (column 5) (len 9)))))
    (reference r21 (scope relative) (span (offset 2182) (line 70) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 2182) (line 70) (column 4) (len 8)))))
    (reference r22 (scope relative) (span (offset 2210) (line 71) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2210) (line 71) (column 5) (len 9)))))
    (reference r23 (scope relative) (span (offset 2301) (line 72) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2301) (line 72) (column 5) (len 9)))))
    (reference r24 (scope relative) (span (offset 2360) (line 73) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2360) (line 73) (column 5) (len 9)))))
    (reference r25 (scope relative) (span (offset 2538) (line 78) (column 16) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 2538) (line 78) (column 16) (len 7)))))
    (reference r26 (scope relative) (span (offset 3195) (line 104) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 3195) (line 104) (column 4) (len 8)))))
    (reference r27 (scope relative) (span (offset 3223) (line 105) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 3223) (line 105) (column 5) (len 9)))))
    (reference r28 (scope relative) (span (offset 3312) (line 106) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 3312) (line 106) (column 5) (len 9)))))
    (reference r29 (scope relative) (span (offset 3805) (line 123) (column 20) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 3805) (line 123) (column 20) (len 7)))))
    (reference r30 (scope relative) (span (offset 3833) (line 124) (column 20) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 3833) (line 124) (column 20) (len 7)))))
    (reference r31 (scope relative) (span (offset 4391) (line 146) (column 12) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4391) (line 146) (column 12) (len 12))) (segment 1 (token "apisc") (name "apisc") (separator dot) (span (offset 4404) (line 146) (column 25) (len 5)))))
    (reference r32 (scope relative) (span (offset 4413) (line 146) (column 34) (len 19)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4413) (line 146) (column 34) (len 10))) (segment 1 (token "getTopic") (name "getTopic") (separator dot) (span (offset 4424) (line 146) (column 45) (len 8)))))
    (reference r33 (scope relative) (span (offset 4446) (line 147) (column 12) (len 20)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4446) (line 147) (column 12) (len 10))) (segment 1 (token "giveTopic") (name "giveTopic") (separator dot) (span (offset 4457) (line 147) (column 23) (len 9)))))
    (reference r34 (scope relative) (span (offset 4470) (line 147) (column 36) (len 18)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4470) (line 147) (column 36) (len 12))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4483) (line 147) (column 49) (len 5)))))
    (reference r35 (scope relative) (span (offset 4505) (line 149) (column 12) (len 19)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4505) (line 149) (column 12) (len 13))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4519) (line 149) (column 26) (len 5)))))
    (reference r36 (scope relative) (span (offset 4528) (line 149) (column 35) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4528) (line 149) (column 35) (len 12))) (segment 1 (token "tellu") (name "tellu") (separator dot) (span (offset 4541) (line 149) (column 48) (len 5)))))
    (reference r37 (scope relative) (span (offset 4643) (line 152) (column 12) (len 29)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4643) (line 152) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4656) (line 152) (column 25) (len 16)))))
    (reference r38 (scope relative) (span (offset 4676) (line 152) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4676) (line 152) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4693) (line 152) (column 62) (len 16)))))
    (reference r39 (scope relative) (span (offset 4722) (line 153) (column 12) (len 30)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4722) (line 153) (column 12) (len 13))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4736) (line 153) (column 26) (len 16)))))
    (reference r40 (scope relative) (span (offset 4756) (line 153) (column 46) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4756) (line 153) (column 46) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4773) (line 153) (column 63) (len 16)))))
    (reference r41 (scope relative) (span (offset 4802) (line 154) (column 12) (len 29)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4802) (line 154) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4815) (line 154) (column 25) (len 16)))))
    (reference r42 (scope relative) (span (offset 4835) (line 154) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4835) (line 154) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4852) (line 154) (column 62) (len 16)))))
  )
  (root (package (name "AHFNorway") (body brace (doc) (import (target (span (span (offset 274) (line 6) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 287) (line 6) (column 30) (len 3))) (separator (span (offset 287) (line 6) (column 30) (len 2))) (marker (span (offset 289) (line 6) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 308) (line 7) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 326) (line 7) (column 35) (len 3))) (separator (span (offset 326) (line 7) (column 35) (len 2))) (marker (span (offset 328) (line 7) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 347) (line 8) (column 17) (len 14))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 357) (line 8) (column 27) (len 4))) (separator (span (offset 357) (line 8) (column 27) (len 2))) (marker (span (offset 359) (line 8) (column 29) (len 2)))))))) (import (target (span (span (offset 379) (line 9) (column 17) (len 15))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 391) (line 9) (column 29) (len 3))) (separator (span (offset 391) (line 9) (column 29) (len 2))) (marker (span (offset 393) (line 9) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (def true) (name "APISService") (specializes none) (body brace (doc) (attribute-usage) (attribute-usage) (attribute-usage))) (metadata-keyword-usage (type (ref r5)) (body none)) (port-def (name "APIS_DD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6)))) (body brace (doc) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "APIS_HTTP") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration) (in-out-declaration))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r8))) (declaration-name "APIS_MQTT") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration))))) (attribute-def (declaration-name "Publish") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "nametopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Subscribe") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "nametopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Return_AllItems") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "itms") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Subscribe_giveItems") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "itms") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Return_Ack") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "ack") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "CallGiveItems") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "itms") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "ResultGiveItems") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "ack") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (def false) (name "AHFNorway_LocalCloudDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (body brace (extended-def (prefix-keywords ((ref r18))) (definition-prefix none) (def false) (name "TellUConsumer") (specializes none) (body brace (metadata-keyword-usage (type (ref r19)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscoveryDD ; // communicating with") (span (offset 1515) (line 49) (column 15) (len 80))) (metadata-keyword-usage (type (ref r20)) (body none)) (default-reference-usage) (attribute-usage) (attribute-usage) (attribute-usage) (state-usage))) (extended-def (prefix-keywords ((ref r21))) (definition-prefix none) (def false) (name "APISProducer") (specializes none) (body brace (metadata-keyword-usage (type (ref r22)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscoveryDD ; // communicating with") (span (offset 2220) (line 71) (column 15) (len 80))) (metadata-keyword-usage (type (ref r23)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "tellu:~APIS_DD; // providing the APISService") (span (offset 2311) (line 72) (column 15) (len 48))) (metadata-keyword-usage (type (ref r24)) (body none)) (default-reference-usage) (default-reference-usage) (default-reference-usage) (default-reference-usage) (attribute-def (declaration-name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (action-usage (name "giveItems") (short-name none) (body brace (in-out-declaration) (in-out-declaration) (first) (then-action) (default-reference-usage) (bind))) (state-usage))) (extended-def (prefix-keywords ((ref r26))) (definition-prefix none) (def false) (name "APISConsumer") (specializes none) (body brace (metadata-keyword-usage (type (ref r27)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "serviceDiscovery:~ServiceDiscovery ; // communicating with S") (span (offset 3233) (line 105) (column 15) (len 78))) (metadata-keyword-usage (type (ref r28)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "apisp:~APIS_DD ;") (span (offset 3322) (line 106) (column 15) (len 195))) (state-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "MQTTServer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "getTopic") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "giveTopic") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))) (connect (from (expression (span (offset 4391) (line 146) (column 12) (len 18)) (ref r31))) (to (expression (span (offset 4413) (line 146) (column 34) (len 19)) (ref r32))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4446) (line 147) (column 12) (len 20)) (ref r33))) (to (expression (span (offset 4470) (line 147) (column 36) (len 18)) (ref r34))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4505) (line 149) (column 12) (len 19)) (ref r35))) (to (expression (span (offset 4528) (line 149) (column 35) (len 18)) (ref r36))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4643) (line 152) (column 12) (len 29)) (ref r37))) (to (expression (span (offset 4676) (line 152) (column 45) (len 33)) (ref r38))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4722) (line 153) (column 12) (len 30)) (ref r39))) (to (expression (span (offset 4756) (line 153) (column 46) (len 33)) (ref r40))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4802) (line 154) (column 12) (len 29)) (ref r41))) (to (expression (span (offset 4835) (line 154) (column 45) (len 33)) (ref r42))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
