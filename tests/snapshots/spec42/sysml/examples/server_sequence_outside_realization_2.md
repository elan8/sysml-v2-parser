# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Interaction Sequencing): ServerSequenceOutsideRealization-2"))
~~~
# SOURCE
~~~sysml
package ServerSequenceOutsideRealization_2 {
	private import ScalarValues::String;
	private import ServerSequenceModelOutside::*;
	private import Configuration::*;
	
	package Configuration {
		
		port def PublicationPort;
		
		port def SubscriptionPort;
		
		part producer_2[1] {
			attribute someTopic : String;
			private item somePublication;
			/* Requiring FIFO sort (as opposed to just default) to make arrival/leave ordering
			 * in ServerSequenceModelOutside.sysml equivalent to accept/send new ordering in
			 * ServerSquenceRealization-2.sysml. */
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
			port publicationPort : ~PublicationPort;
			
			perform action producerBehavior {
				action publish send new Publish(someTopic, somePublication) via publicationPort;
			}
		}
		
		interface producer_2.publicationPort to server_2.publicationPort;
		
		part server_2[1] {
			port publicationPort : PublicationPort;
			port subscriptionPort : SubscriptionPort;
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
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
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
			port subscriptionPort : ~SubscriptionPort;
			
			perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
		}
		
	}
	
	part realization_2 : PubSubSequence {
		part :>> producer :> producer_2;
		part :>> server :> server_2;
		part :>> consumer :> consumer_2;

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
  (document "server_sequence_outside_realization_2.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 1883) (line 61) (column 4) (len 171)) (message "unexpected token in part usage body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 1883) (line 61) (column 4) (len 171)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ServerSequenceOutsideRealization_2 {
    private import ScalarValues::String;
    private import ServerSequenceModelOutside::*;
    private import Configuration::*;
    package Configuration {
        port def PublicationPort;
        port def SubscriptionPort;
        part producer_2[1] {
            attribute someTopic : String;
            private item somePublication;
            attribute :>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
            port publicationPort : ~PublicationPort;
            perform action producerBehavior {
                action publish send new Publish(someTopic, somePublication) via publicationPort;
            }
        }
        interface producer_2.publicationPort to server_2.publicationPort;
        part server_2[1] {
            port publicationPort : PublicationPort;
            port subscriptionPort : SubscriptionPort;
            attribute :>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
            state serverBehavior {
                entry;
                then waitForSubscription;
                state waitForSubscription;
                transition subscribing first waitForSubscription accept sub : Subscribe via subscriptionPort then waitForPublication;
                state waitForPublication;
                transition delivering first waitForPublication accept pub : Publish via publicationPort if pub.topic == subscribing.sub.topic do send new Deliver(pub.publication) to subscribing.sub.subscriber then waitForPublication;
            }
        }
        interface consumer_2.subscriptionPort to server_2.subscriptionPort;
        part consumer_2[1] {
            attribute myTopic : String;
            attribute :>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
            port subscriptionPort : ~SubscriptionPort;
            perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
        }
    }
    part realization_2 : PubSubSequence {
        part producer :> producer_2;
        part server :> server_2;
        part consumer :> consumer_2;
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
    (reference r0 (scope relative) (span (offset 61) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 61) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 75) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 99) (line 3) (column 17) (len 26)) (segments (segment 0 (token "ServerSequenceModelOutside") (name "ServerSequenceModelOutside") (separator none) (span (offset 99) (line 3) (column 17) (len 26)))))
    (reference r2 (scope relative) (span (offset 146) (line 4) (column 17) (len 13)) (segments (segment 0 (token "Configuration") (name "Configuration") (separator none) (span (offset 146) (line 4) (column 17) (len 13)))))
    (reference r3 (scope relative) (span (offset 305) (line 13) (column 26) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 305) (line 13) (column 26) (len 6)))))
    (reference r4 (scope relative) (span (offset 566) (line 18) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 566) (line 18) (column 8) (len 20)))))
    (reference r5 (scope relative) (span (offset 589) (line 18) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 589) (line 18) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 602) (line 18) (column 44) (len 32)))))
    (reference r6 (scope relative) (span (offset 1009) (line 32) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 1009) (line 32) (column 8) (len 20)))))
    (reference r7 (scope relative) (span (offset 1032) (line 32) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1032) (line 32) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 1045) (line 32) (column 44) (len 32)))))
    (reference r8 (scope relative) (span (offset 1741) (line 56) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1741) (line 56) (column 24) (len 6)))))
    (reference r9 (scope relative) (span (offset 1756) (line 57) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 1756) (line 57) (column 8) (len 20)))))
    (reference r10 (scope relative) (span (offset 1779) (line 57) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1779) (line 57) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 1792) (line 57) (column 44) (len 32)))))
    (reference r11 (scope relative) (span (offset 2086) (line 69) (column 23) (len 14)) (segments (segment 0 (token "PubSubSequence") (name "PubSubSequence") (separator none) (span (offset 2086) (line 69) (column 23) (len 14)))))
  )
  (root (package (name "ServerSequenceOutsideRealization_2") (body brace (import (target (span (span (offset 61) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 99) (line 3) (column 17) (len 29))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 125) (line 3) (column 43) (len 3))) (separator (span (offset 125) (line 3) (column 43) (len 2))) (marker (span (offset 127) (line 3) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 146) (line 4) (column 17) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 159) (line 4) (column 30) (len 3))) (separator (span (offset 159) (line 4) (column 30) (len 2))) (marker (span (offset 161) (line 4) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Configuration") (body brace (port-def (name "PublicationPort") (specializes none) (body semicolon)) (port-def (name "SubscriptionPort") (specializes none) (body semicolon)) (part-usage (declaration-name "producer_2") (typing none) (body brace (attribute-usage (declaration-name "someTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 589) (line 18) (column 31) (len 45)) (ref r5))))) (body semicolon)) (port-usage) (perform))) (interface-usage) (part-usage (declaration-name "server_2") (typing none) (body brace (port-usage) (port-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1032) (line 32) (column 31) (len 45)) (ref r7))))) (body semicolon)) (state-usage))) (interface-usage) (part-usage (declaration-name "consumer_2") (typing none) (body brace (attribute-usage (declaration-name "myTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1779) (line 57) (column 31) (len 45)) (ref r10))))) (body semicolon)) (port-usage) (malformed (code "recovered_part_usage_body_element") (found "perform action consumerBehavior {") (span (offset 1883) (line 61) (column 4) (len 171))))))) (part-usage (declaration-name "realization_2") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (body brace (part-usage) (part-usage) (part-usage) (malformed (code "recovered_part_usage_body_element") (found "flow :>> publish_message: Transfers::MessageTransfer {") (span (offset 2207) (line 74) (column 3) (len 158))) (malformed (code "recovered_part_usage_body_element") (found "flow :>> subscribe_message: Transfers::MessageTransfer {") (span (offset 2365) (line 78) (column 3) (len 162))) (malformed (code "recovered_part_usage_body_element") (found "flow :>> deliver_message: Transfers::MessageTransfer {") (span (offset 2527) (line 82) (column 3) (len 218))) (bind) (bind) (bind) (bind) (bind) (bind))))))
)
~~~
