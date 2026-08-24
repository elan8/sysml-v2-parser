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
        interface producer_2.publicationPort to server_2.publicationPort;
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
        flow : Transfers::MessageTransfer :>> publish_message {
            end :>> source ::> producer.publicationPort;
            end :>> target ::> server.publicationPort;
        }
        flow : Transfers::MessageTransfer :>> subscribe_message {
            end :>> source ::> consumer.subscriptionPort;
            end :>> target ::> server.subscriptionPort;
        }
        flow : Transfers::MessageTransfer :>> deliver_message {
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 68) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 17) (len 19)) (segments (segment 0 (token "ServerSequenceModel") (name "ServerSequenceModel") (separator none) (span (offset 92) (line 3) (column 17) (len 19)))))
    (reference r2 (scope relative) (span (offset 132) (line 4) (column 17) (len 13)) (segments (segment 0 (token "Configuration") (name "Configuration") (separator none) (span (offset 132) (line 4) (column 17) (len 13)))))
    (reference r3 (scope relative) (span (offset 291) (line 13) (column 26) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 291) (line 13) (column 26) (len 6)))))
    (reference r4 (scope relative) (span (offset 363) (line 16) (column 28) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 363) (line 16) (column 28) (len 15)))))
    (reference r5 (scope relative) (span (offset 531) (line 23) (column 13) (len 26)) (segments (segment 0 (token "producer_2") (name "producer_2") (separator none) (span (offset 531) (line 23) (column 13) (len 10))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 542) (line 23) (column 24) (len 15)))))
    (reference r6 (scope relative) (span (offset 561) (line 23) (column 43) (len 24)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 561) (line 23) (column 43) (len 8))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 570) (line 23) (column 52) (len 15)))))
    (reference r7 (scope relative) (span (offset 637) (line 26) (column 27) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 637) (line 26) (column 27) (len 15)))))
    (reference r8 (scope relative) (span (offset 681) (line 27) (column 28) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 681) (line 27) (column 28) (len 16)))))
    (reference r9 (scope relative) (span (offset 1254) (line 48) (column 13) (len 27)) (segments (segment 0 (token "consumer_2") (name "consumer_2") (separator none) (span (offset 1254) (line 48) (column 13) (len 10))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 1265) (line 48) (column 24) (len 16)))))
    (reference r10 (scope relative) (span (offset 1285) (line 48) (column 44) (len 25)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 1285) (line 48) (column 44) (len 8))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 1294) (line 48) (column 53) (len 16)))))
    (reference r11 (scope relative) (span (offset 1361) (line 51) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1361) (line 51) (column 24) (len 6)))))
    (reference r12 (scope relative) (span (offset 1401) (line 53) (column 29) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 1401) (line 53) (column 29) (len 16)))))
    (reference r13 (scope relative) (span (offset 1629) (line 63) (column 23) (len 14)) (segments (segment 0 (token "PubSubSequence") (name "PubSubSequence") (separator none) (span (offset 1629) (line 63) (column 23) (len 14)))))
    (reference r14 (scope relative) (span (offset 1669) (line 64) (column 24) (len 10)) (segments (segment 0 (token "producer_2") (name "producer_2") (separator none) (span (offset 1669) (line 64) (column 24) (len 10)))))
    (reference r15 (scope relative) (span (offset 1691) (line 65) (column 10) (len 24)) (segments (segment 0 (token "producerBehavior") (name "producerBehavior") (separator none) (span (offset 1691) (line 65) (column 10) (len 16))) (segment 1 (token "publish") (name "publish") (separator dot) (span (offset 1708) (line 65) (column 27) (len 7)))))
    (reference r16 (scope relative) (span (offset 1771) (line 68) (column 22) (len 8)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 1771) (line 68) (column 22) (len 8)))))
    (reference r17 (scope relative) (span (offset 1791) (line 69) (column 10) (len 35)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1791) (line 69) (column 10) (len 14))) (segment 1 (token "subscribing") (name "subscribing") (separator dot) (span (offset 1806) (line 69) (column 25) (len 11))) (segment 2 (token "accepter") (name "accepter") (separator dot) (span (offset 1818) (line 69) (column 37) (len 8)))))
    (reference r18 (scope relative) (span (offset 1867) (line 70) (column 10) (len 34)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1867) (line 70) (column 10) (len 14))) (segment 1 (token "delivering") (name "delivering") (separator dot) (span (offset 1882) (line 70) (column 25) (len 10))) (segment 2 (token "accepter") (name "accepter") (separator dot) (span (offset 1893) (line 70) (column 36) (len 8)))))
    (reference r19 (scope relative) (span (offset 1940) (line 71) (column 10) (len 32)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1940) (line 71) (column 10) (len 14))) (segment 1 (token "delivering") (name "delivering") (separator dot) (span (offset 1955) (line 71) (column 25) (len 10))) (segment 2 (token "effect") (name "effect") (separator dot) (span (offset 1966) (line 71) (column 36) (len 6)))))
    (reference r20 (scope relative) (span (offset 2032) (line 74) (column 24) (len 10)) (segments (segment 0 (token "consumer_2") (name "consumer_2") (separator none) (span (offset 2032) (line 74) (column 24) (len 10)))))
    (reference r21 (scope relative) (span (offset 2054) (line 75) (column 10) (len 26)) (segments (segment 0 (token "consumerBehavior") (name "consumerBehavior") (separator none) (span (offset 2054) (line 75) (column 10) (len 16))) (segment 1 (token "subscribe") (name "subscribe") (separator dot) (span (offset 2071) (line 75) (column 27) (len 9)))))
    (reference r22 (scope relative) (span (offset 2121) (line 76) (column 10) (len 25)) (segments (segment 0 (token "consumerBehavior") (name "consumerBehavior") (separator none) (span (offset 2121) (line 76) (column 10) (len 16))) (segment 1 (token "delivery") (name "delivery") (separator dot) (span (offset 2138) (line 76) (column 27) (len 8)))))
    (reference r23 (scope relative) (span (offset 2209) (line 79) (column 29) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2209) (line 79) (column 29) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2220) (line 79) (column 40) (len 15)))))
    (reference r24 (scope relative) (span (offset 2192) (line 79) (column 12) (len 15)) (segments (segment 0 (token "publish_message") (name "publish_message") (separator none) (span (offset 2192) (line 79) (column 12) (len 15)))))
    (reference r25 (scope relative) (span (offset 2261) (line 80) (column 24) (len 24)) (segments (segment 0 (token "producer") (name "producer") (separator none) (span (offset 2261) (line 80) (column 24) (len 8))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 2270) (line 80) (column 33) (len 15)))))
    (reference r26 (scope relative) (span (offset 2250) (line 80) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2250) (line 80) (column 13) (len 6)))))
    (reference r27 (scope relative) (span (offset 2310) (line 81) (column 24) (len 22)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2310) (line 81) (column 24) (len 6))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 2317) (line 81) (column 31) (len 15)))))
    (reference r28 (scope relative) (span (offset 2299) (line 81) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2299) (line 81) (column 13) (len 6)))))
    (reference r29 (scope relative) (span (offset 2369) (line 83) (column 31) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2369) (line 83) (column 31) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2380) (line 83) (column 42) (len 15)))))
    (reference r30 (scope relative) (span (offset 2350) (line 83) (column 12) (len 17)) (segments (segment 0 (token "subscribe_message") (name "subscribe_message") (separator none) (span (offset 2350) (line 83) (column 12) (len 17)))))
    (reference r31 (scope relative) (span (offset 2421) (line 84) (column 24) (len 25)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 2421) (line 84) (column 24) (len 8))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 2430) (line 84) (column 33) (len 16)))))
    (reference r32 (scope relative) (span (offset 2410) (line 84) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2410) (line 84) (column 13) (len 6)))))
    (reference r33 (scope relative) (span (offset 2471) (line 85) (column 24) (len 23)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2471) (line 85) (column 24) (len 6))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 2478) (line 85) (column 31) (len 16)))))
    (reference r34 (scope relative) (span (offset 2460) (line 85) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2460) (line 85) (column 13) (len 6)))))
    (reference r35 (scope relative) (span (offset 2529) (line 87) (column 29) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2529) (line 87) (column 29) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2540) (line 87) (column 40) (len 15)))))
    (reference r36 (scope relative) (span (offset 2512) (line 87) (column 12) (len 15)) (segments (segment 0 (token "deliver_message") (name "deliver_message") (separator none) (span (offset 2512) (line 87) (column 12) (len 15)))))
    (reference r37 (scope relative) (span (offset 2581) (line 88) (column 24) (len 6)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2581) (line 88) (column 24) (len 6)))))
    (reference r38 (scope relative) (span (offset 2570) (line 88) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2570) (line 88) (column 13) (len 6)))))
    (reference r39 (scope relative) (span (offset 2612) (line 89) (column 24) (len 8)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 2612) (line 89) (column 24) (len 8)))))
    (reference r40 (scope relative) (span (offset 2601) (line 89) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2601) (line 89) (column 13) (len 6)))))
  )
  (root (package (name "ServerSequenceRealization_2") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 22))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 36) (len 3))) (separator (span (offset 111) (line 3) (column 36) (len 2))) (marker (span (offset 113) (line 3) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 132) (line 4) (column 17) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 145) (line 4) (column 30) (len 3))) (separator (span (offset 145) (line 4) (column 30) (len 2))) (marker (span (offset 147) (line 4) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Configuration") (body brace (port-def (name "PublicationPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "SubscriptionPort") (modifiers) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 261) (line 12) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 261) (line 12) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "someTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "somePublication") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "producerBehavior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action))))) (interface-usage (form connection) (part (binary (from (interface-end (multiplicity none) (target (ref r5)))) (to (interface-end (multiplicity none) (target (ref r6)))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 606) (line 25) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 606) (line 25) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))) (interface-usage (form connection) (part (binary (from (interface-end (multiplicity none) (target (ref r9)))) (to (interface-end (multiplicity none) (target (ref r10)))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1333) (line 50) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 1333) (line 50) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "myTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "consumerBehavior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action) (action))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "realization_2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r14))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r15)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r16))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r17)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r18)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r19)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r20))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r21)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r22)) (body semicolon)))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r25)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r27)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (crosses none))))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r31)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r33)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (crosses none))))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r36)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r37)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r39)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (crosses none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2636) (line 92) (column 6) (len 63)) (normalized "Binding sent/accept messages to specification model messages. "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2708) (line 93) (column 7) (len 7)) (normalized "Sends "))) (bind) (bind) (bind) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2960) (line 97) (column 8) (len 9)) (normalized "Accepts "))) (bind) (bind) (bind))))))
)
~~~
