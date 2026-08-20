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
        nametopic : String;
    }
    attribute def Subscribe {
        nametopic : String;
    }
    attribute def Return_AllItems {
        itms : String;
    }
    attribute def Subscribe_giveItems {
        itms : String;
    }
    attribute def Return_Ack {
        ack : Boolean;
    }
    attribute def CallGiveItems {
        itms : String;
    }
    attribute def ResultGiveItems {
        ack : Boolean;
    }
    #clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {
        #systemdd TellUConsumer {
            #servicedd
            serviceDiscovery : ~ServiceDiscoveryDD;
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
            serviceDiscovery : ~ServiceDiscoveryDD;
            #servicedd
            tellu : ~APIS_DD;
            #servicedd
            apisc : APIS_DD;
             :>> systemname = "PrediktorApisServer";
             :>> address = "Prediktor_network_ip";
             :>> portno = 6565;
            attribute x : Boolean;
            action giveItems :> ServiceMethod {
                in itms : String;
                out ack : Boolean;
                /* Forward itms and return an ack */
                first start;
                then send new Return_AllItems(itms) via apisc.APIS_MQTT;
                success = true;
                bind ack = success;
            }
            state APISPbehavior {
                entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
                then WaitOnData;
                state WaitOnData;
                transition accept cl : CallGiveItems via tellu.APIS_HTTP do action {
                    first start;
                    then action giveItems {
                        in itms = cl.itms;
                        out ack = x;
                    }
                    then send new ResultGiveItems(x) via tellu.APIS_HTTP;
                } then WaitOnData;
            }
        }
        #systemdd APISConsumer {
            #servicedd
            serviceDiscovery : ~ServiceDiscovery;
            #servicedd
            apisp : ~APIS_DD;
             :>> systemname = "TellUClient";
             :>> address = "Prediktor_network_ip";
             :>> portno = 1;
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
    (reference r20 (scope relative) (span (offset 1533) (line 49) (column 33) (len 18)) (segments (segment 0 (token "ServiceDiscoveryDD") (name "ServiceDiscoveryDD") (separator none) (span (offset 1533) (line 49) (column 33) (len 18)))))
    (reference r21 (scope relative) (span (offset 1596) (line 50) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 1596) (line 50) (column 5) (len 9)))))
    (reference r22 (scope relative) (span (offset 1612) (line 50) (column 21) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 1612) (line 50) (column 21) (len 7)))))
    (reference r23 (scope relative) (span (offset 2182) (line 70) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 2182) (line 70) (column 4) (len 8)))))
    (reference r24 (scope relative) (span (offset 2210) (line 71) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2210) (line 71) (column 5) (len 9)))))
    (reference r25 (scope relative) (span (offset 2238) (line 71) (column 33) (len 18)) (segments (segment 0 (token "ServiceDiscoveryDD") (name "ServiceDiscoveryDD") (separator none) (span (offset 2238) (line 71) (column 33) (len 18)))))
    (reference r26 (scope relative) (span (offset 2301) (line 72) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2301) (line 72) (column 5) (len 9)))))
    (reference r27 (scope relative) (span (offset 2318) (line 72) (column 22) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 2318) (line 72) (column 22) (len 7)))))
    (reference r28 (scope relative) (span (offset 2360) (line 73) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 2360) (line 73) (column 5) (len 9)))))
    (reference r29 (scope relative) (span (offset 2376) (line 73) (column 21) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 2376) (line 73) (column 21) (len 7)))))
    (reference r30 (scope relative) (span (offset 2424) (line 75) (column 8) (len 10)) (segments (segment 0 (token "systemname") (name "systemname") (separator none) (span (offset 2424) (line 75) (column 8) (len 10)))))
    (reference r31 (scope relative) (span (offset 2467) (line 76) (column 8) (len 7)) (segments (segment 0 (token "address") (name "address") (separator none) (span (offset 2467) (line 76) (column 8) (len 7)))))
    (reference r32 (scope relative) (span (offset 2508) (line 77) (column 8) (len 6)) (segments (segment 0 (token "portno") (name "portno") (separator none) (span (offset 2508) (line 77) (column 8) (len 6)))))
    (reference r33 (scope relative) (span (offset 3195) (line 104) (column 4) (len 8)) (segments (segment 0 (token "systemdd") (name "systemdd") (separator none) (span (offset 3195) (line 104) (column 4) (len 8)))))
    (reference r34 (scope relative) (span (offset 3223) (line 105) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 3223) (line 105) (column 5) (len 9)))))
    (reference r35 (scope relative) (span (offset 3251) (line 105) (column 33) (len 16)) (segments (segment 0 (token "ServiceDiscovery") (name "ServiceDiscovery") (separator none) (span (offset 3251) (line 105) (column 33) (len 16)))))
    (reference r36 (scope relative) (span (offset 3312) (line 106) (column 5) (len 9)) (segments (segment 0 (token "servicedd") (name "servicedd") (separator none) (span (offset 3312) (line 106) (column 5) (len 9)))))
    (reference r37 (scope relative) (span (offset 3329) (line 106) (column 22) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 3329) (line 106) (column 22) (len 7)))))
    (reference r38 (scope relative) (span (offset 3346) (line 107) (column 8) (len 10)) (segments (segment 0 (token "systemname") (name "systemname") (separator none) (span (offset 3346) (line 107) (column 8) (len 10)))))
    (reference r39 (scope relative) (span (offset 3381) (line 108) (column 8) (len 7)) (segments (segment 0 (token "address") (name "address") (separator none) (span (offset 3381) (line 108) (column 8) (len 7)))))
    (reference r40 (scope relative) (span (offset 3422) (line 109) (column 8) (len 6)) (segments (segment 0 (token "portno") (name "portno") (separator none) (span (offset 3422) (line 109) (column 8) (len 6)))))
    (reference r41 (scope relative) (span (offset 3805) (line 123) (column 20) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 3805) (line 123) (column 20) (len 7)))))
    (reference r42 (scope relative) (span (offset 3833) (line 124) (column 20) (len 7)) (segments (segment 0 (token "APIS_DD") (name "APIS_DD") (separator none) (span (offset 3833) (line 124) (column 20) (len 7)))))
    (reference r43 (scope relative) (span (offset 4391) (line 146) (column 12) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4391) (line 146) (column 12) (len 12))) (segment 1 (token "apisc") (name "apisc") (separator dot) (span (offset 4404) (line 146) (column 25) (len 5)))))
    (reference r44 (scope relative) (span (offset 4413) (line 146) (column 34) (len 19)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4413) (line 146) (column 34) (len 10))) (segment 1 (token "getTopic") (name "getTopic") (separator dot) (span (offset 4424) (line 146) (column 45) (len 8)))))
    (reference r45 (scope relative) (span (offset 4446) (line 147) (column 12) (len 20)) (segments (segment 0 (token "MQTTServer") (name "MQTTServer") (separator none) (span (offset 4446) (line 147) (column 12) (len 10))) (segment 1 (token "giveTopic") (name "giveTopic") (separator dot) (span (offset 4457) (line 147) (column 23) (len 9)))))
    (reference r46 (scope relative) (span (offset 4470) (line 147) (column 36) (len 18)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4470) (line 147) (column 36) (len 12))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4483) (line 147) (column 49) (len 5)))))
    (reference r47 (scope relative) (span (offset 4505) (line 149) (column 12) (len 19)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4505) (line 149) (column 12) (len 13))) (segment 1 (token "apisp") (name "apisp") (separator dot) (span (offset 4519) (line 149) (column 26) (len 5)))))
    (reference r48 (scope relative) (span (offset 4528) (line 149) (column 35) (len 18)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4528) (line 149) (column 35) (len 12))) (segment 1 (token "tellu") (name "tellu") (separator dot) (span (offset 4541) (line 149) (column 48) (len 5)))))
    (reference r49 (scope relative) (span (offset 4643) (line 152) (column 12) (len 29)) (segments (segment 0 (token "APISProducer") (name "APISProducer") (separator none) (span (offset 4643) (line 152) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4656) (line 152) (column 25) (len 16)))))
    (reference r50 (scope relative) (span (offset 4676) (line 152) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4676) (line 152) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4693) (line 152) (column 62) (len 16)))))
    (reference r51 (scope relative) (span (offset 4722) (line 153) (column 12) (len 30)) (segments (segment 0 (token "TellUConsumer") (name "TellUConsumer") (separator none) (span (offset 4722) (line 153) (column 12) (len 13))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4736) (line 153) (column 26) (len 16)))))
    (reference r52 (scope relative) (span (offset 4756) (line 153) (column 46) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4756) (line 153) (column 46) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4773) (line 153) (column 63) (len 16)))))
    (reference r53 (scope relative) (span (offset 4802) (line 154) (column 12) (len 29)) (segments (segment 0 (token "APISConsumer") (name "APISConsumer") (separator none) (span (offset 4802) (line 154) (column 12) (len 12))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4815) (line 154) (column 25) (len 16)))))
    (reference r54 (scope relative) (span (offset 4835) (line 154) (column 45) (len 33)) (segments (segment 0 (token "service_registry") (name "service_registry") (separator none) (span (offset 4835) (line 154) (column 45) (len 16))) (segment 1 (token "serviceDiscovery") (name "serviceDiscovery") (separator dot) (span (offset 4852) (line 154) (column 62) (len 16)))))
  )
  (root (package (name "AHFNorway") (body brace (doc (name none) (locale none) (body (span (offset 27) (line 2) (column 8) (len 56)) (normalized "This is the Norwegian use-case for Arrowhead Framework "))) (import (target (span (span (offset 274) (line 6) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 287) (line 6) (column 30) (len 3))) (separator (span (offset 287) (line 6) (column 30) (len 2))) (marker (span (offset 289) (line 6) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 308) (line 7) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 326) (line 7) (column 35) (len 3))) (separator (span (offset 326) (line 7) (column 35) (len 2))) (marker (span (offset 328) (line 7) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 347) (line 8) (column 17) (len 14))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 357) (line 8) (column 27) (len 4))) (separator (span (offset 357) (line 8) (column 27) (len 2))) (marker (span (offset 359) (line 8) (column 29) (len 2)))))))) (import (target (span (span (offset 379) (line 9) (column 17) (len 15))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 391) (line 9) (column 29) (len 3))) (separator (span (offset 391) (line 9) (column 29) (len 2))) (marker (span (offset 393) (line 9) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (def true) (name "APISService") (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 433) (line 12) (column 9) (len 16)) (normalized "Service design "))) (attribute-usage) (attribute-usage) (attribute-usage))) (metadata-keyword-usage (type (ref r5)) (body none)) (port-def (name "APIS_DD") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6)))) (body brace (doc (name none) (locale none) (body (span (offset 650) (line 20) (column 9) (len 64)) (normalized "Service design description with nested protocol-specific ports "))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "APIS_HTTP") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration) (in-out-declaration))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r8))) (declaration-name "APIS_MQTT") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration))))) (attribute-def (declaration-name "Publish") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "nametopic") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Subscribe") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "nametopic") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Return_AllItems") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "itms") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Subscribe_giveItems") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "itms") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Return_Ack") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "ack") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "CallGiveItems") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "itms") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "ResultGiveItems") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "ack") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (def false) (name "AHFNorway_LocalCloudDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (body brace (extended-def (prefix-keywords ((ref r18))) (definition-prefix none) (def false) (name "TellUConsumer") (specializes none) (body brace (metadata-keyword-usage (type (ref r19)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "serviceDiscovery") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r21)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "apisp") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (attribute-usage) (attribute-usage) (state-usage))) (extended-def (prefix-keywords ((ref r23))) (definition-prefix none) (def false) (name "APISProducer") (specializes none) (body brace (metadata-keyword-usage (type (ref r24)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "serviceDiscovery") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r26)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "tellu") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r28)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "apisc") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2437) (line 75) (column 21) (len 21)) (string "PrediktorApisServer"))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r31)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2477) (line 76) (column 18) (len 22)) (string "Prediktor_network_ip"))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2517) (line 77) (column 17) (len 4)) (integer 6565))))) (body semicolon)) (attribute-usage) (action-usage (name "giveItems") (short-name none) (body brace (in-out-declaration) (in-out-declaration) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2635) (line 82) (column 8) (len 32)) (normalized "Forward itms and return an ack "))) (first) (then-action) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "success") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2765) (line 85) (column 16) (len 4)) (boolean true))))) (body semicolon)) (bind))) (state-usage))) (extended-def (prefix-keywords ((ref r33))) (definition-prefix none) (def false) (name "APISConsumer") (specializes none) (body brace (metadata-keyword-usage (type (ref r34)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "serviceDiscovery") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r36)) (body none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "apisp") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3359) (line 107) (column 21) (len 13)) (string "TellUClient"))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r39)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3391) (line 108) (column 18) (len 22)) (string "Prediktor_network_ip"))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3431) (line 109) (column 17) (len 1)) (integer 1))))) (body semicolon)) (state-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "MQTTServer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "getTopic") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r41)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "giveTopic") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))) (connect (from (expression (span (offset 4391) (line 146) (column 12) (len 18)) (ref r43))) (to (expression (span (offset 4413) (line 146) (column 34) (len 19)) (ref r44))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4446) (line 147) (column 12) (len 20)) (ref r45))) (to (expression (span (offset 4470) (line 147) (column 36) (len 18)) (ref r46))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4505) (line 149) (column 12) (len 19)) (ref r47))) (to (expression (span (offset 4528) (line 149) (column 35) (len 18)) (ref r48))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4643) (line 152) (column 12) (len 29)) (ref r49))) (to (expression (span (offset 4676) (line 152) (column 45) (len 33)) (ref r50))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4722) (line 153) (column 12) (len 30)) (ref r51))) (to (expression (span (offset 4756) (line 153) (column 46) (len 33)) (ref r52))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 4802) (line 154) (column 12) (len 29)) (ref r53))) (to (expression (span (offset 4835) (line 154) (column 45) (len 33)) (ref r54))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
