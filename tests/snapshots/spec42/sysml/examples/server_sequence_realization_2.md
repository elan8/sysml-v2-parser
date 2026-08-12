# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Interaction Sequencing): ServerSequenceRealization-2"))
~~~
# SOURCE
~~~sysml
package ServerSequenceRealization_2 {
	private import ScalarValues::String;
	private import ServerSequenceModel::*;
	private import Configuration::*;
	
	package Configuration {
		
		port def PublicationPort;
		
		port def SubscriptionPort;
		
		part producer_2[1] {
			attribute someTopic : String;
			private item somePublication;
			
			port publicationPort : ~PublicationPort;
			
			perform action producerBehavior {	
				action publish send new Publish(someTopic, somePublication) via publicationPort;
			}
		}
		
		interface producer_2.publicationPort to server_2.publicationPort;
		
		part server_2[1] {
			port publicationPort : PublicationPort;
			port subscriptionPort : SubscriptionPort;
			
			exhibit state serverBehavior {
				entry; then waitForSubscription;
				
				state waitForSubscription;
				transition subscribing
					first waitForSubscription
					accept sub : Subscribe via subscriptionPort
					then waitForPublication;
					
				state waitForPublication;
				transition delivering
					first waitForPublication
					accept pub : Publish via publicationPort
					if pub.topic == subscribing.sub.topic
					do send new Deliver(pub.publication) to subscribing.sub.subscriber
					then waitForPublication;
			}
		}
		
		interface consumer_2.subscriptionPort to server_2.subscriptionPort;
		
		part consumer_2[1] {
			attribute myTopic : String;
			
			port subscriptionPort : ~SubscriptionPort;
			
			perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
		}
		
	}
	
	part realization_2 : PubSubSequence {
		part :>> producer :> producer_2 {
			event producerBehavior.publish[1] :>> publish_source_event;
		}

		part :>> server :> server_2 {
			event serverBehavior.subscribing.accepter[1] :>> subscribe_target_event;
			event serverBehavior.delivering.accepter[1] :>> publish_target_event;
			event serverBehavior.delivering.effect[1] :>> deliver_source_event;
		}
		
		part :>> consumer :> consumer_2 {
			event consumerBehavior.subscribe[1] :>> subscribe_source_event;
			event consumerBehavior.delivery[1] :>> deliver_target_event;
		}

		flow :>> publish_message: Transfers::MessageTransfer {
 			end :>> source ::> producer.publicationPort;
 			end :>> target ::> server.publicationPort;
 		}
		flow :>> subscribe_message: Transfers::MessageTransfer {
 			end :>> source ::> consumer.subscriptionPort;
 			end :>> target ::> server.subscriptionPort;
 		}
		flow :>> deliver_message: Transfers::MessageTransfer {
 			end :>> source ::> server;
 			end :>> target ::> consumer;
 		}
 		
 		/* Binding sent/accept messages to specification model messages. */
		  /* Sends */
 		bind producer_2.producerBehavior.publish.sentMessage = publish_message;
 		bind consumer_2.consumerBehavior.subscribe.sentMessage = subscribe_message;
 		bind server_2.serverBehavior.delivering.effect.sentMessage = deliver_message;
 		  /* Accepts */
 		bind consumer_2.consumerBehavior.delivery.acceptedMessage = subscribe_message;
 		bind server_2.serverBehavior.subscribing.accepter.acceptedMessage = subscribe_message;
 		bind server_2.serverBehavior.delivering.accepter.acceptedMessage = publish_message;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "server_sequence_realization_2.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 1426) (line 55) (column 4) (len 171)) (message "unexpected token in part usage body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 1426) (line 55) (column 4) (len 171)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ServerSequenceRealization_2 {
    private import ScalarValues::String;
    private import ServerSequenceModel::*;
    private import Configuration::*;
    package Configuration {
        port def PublicationPort;
        port def SubscriptionPort;
        part producer_2[1] {
            attribute someTopic : String;
            private item somePublication;
            port publicationPort : ~PublicationPort;
            perform action producerBehavior {
                action publish send new Publish(someTopic, somePublication) via publicationPort;
            }
        }
        interface producer_2.publicationPort to server_2.publicationPort {}
        part server_2[1] {
            port publicationPort : PublicationPort;
            port subscriptionPort : SubscriptionPort;
            state serverBehavior {
                entry;
                then waitForSubscription;
                state waitForSubscription;
                transition subscribing first waitForSubscription accept sub : Subscribe via subscriptionPort then waitForPublication;
                state waitForPublication;
                transition delivering first waitForPublication accept pub : Publish via publicationPort if pub.topic == subscribing.sub.topic do send new Deliver(pub.publication) to subscribing.sub.subscriber then waitForPublication;
            }
        }
        interface consumer_2.subscriptionPort to server_2.subscriptionPort {}
        part consumer_2[1] {
            attribute myTopic : String;
            port subscriptionPort : ~SubscriptionPort;
            perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
        }
    }
    part realization_2 : PubSubSequence {
        part producer :> producer_2 {
            event producerBehavior.publish[1] :>> publish_source_event;
        }
        part server :> server_2 {
            event serverBehavior.subscribing.accepter[1] :>> subscribe_target_event;
            event serverBehavior.delivering.accepter[1] :>> publish_target_event;
            event serverBehavior.delivering.effect[1] :>> deliver_source_event;
        }
        part consumer :> consumer_2 {
            event consumerBehavior.subscribe[1] :>> subscribe_source_event;
            event consumerBehavior.delivery[1] :>> deliver_target_event;
        }
        flow :>> publish_message: Transfers::MessageTransfer {
 			end :>> source ::> producer.publicationPort;
 			end :>> target ::> server.publicationPort;
 		}
        flow :>> subscribe_message: Transfers::MessageTransfer {
 			end :>> source ::> consumer.subscriptionPort;
 			end :>> target ::> server.subscriptionPort;
 		}
        flow :>> deliver_message: Transfers::MessageTransfer {
 			end :>> source ::> server;
 			end :>> target ::> consumer;
 		}
 		
 		/* Binding sent/accept messages to specification model messages. */
		  /* Sends */
        bind producer_2.producerBehavior.publish.sentMessage = publish_message;
        bind consumer_2.consumerBehavior.subscribe.sentMessage = subscribe_message;
        bind server_2.serverBehavior.delivering.effect.sentMessage = deliver_message;
        bind consumer_2.consumerBehavior.delivery.acceptedMessage = subscribe_message;
        bind server_2.serverBehavior.subscribing.accepter.acceptedMessage = subscribe_message;
        bind server_2.serverBehavior.delivering.accepter.acceptedMessage = publish_message;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 68) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 17) (len 19)) (segments (segment 0 (token "ServerSequenceModel") (name "ServerSequenceModel") (separator none) (span (offset 92) (line 3) (column 17) (len 19)))))
    (reference r2 (scope relative) (span (offset 132) (line 4) (column 17) (len 13)) (segments (segment 0 (token "Configuration") (name "Configuration") (separator none) (span (offset 132) (line 4) (column 17) (len 13)))))
  )
  (root (package (name "ServerSequenceRealization_2") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 22))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 36) (len 3))) (separator (span (offset 111) (line 3) (column 36) (len 2))) (marker (span (offset 113) (line 3) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 132) (line 4) (column 17) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 145) (line 4) (column 30) (len 3))) (separator (span (offset 145) (line 4) (column 30) (len 2))) (marker (span (offset 147) (line 4) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Configuration") (body (port-def (name "PublicationPort") (specializes none) (body semicolon)) (port-def (name "SubscriptionPort") (specializes none) (body semicolon)) (part-usage) (interface-usage) (part-usage) (interface-usage) (part-usage))) (part-usage))))
)
~~~
