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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 68) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 17) (len 19)) (segments (segment 0 (token "ServerSequenceModel") (name "ServerSequenceModel") (separator none) (span (offset 92) (line 3) (column 17) (len 19)))))
    (reference r2 (scope relative) (span (offset 132) (line 4) (column 17) (len 13)) (segments (segment 0 (token "Configuration") (name "Configuration") (separator none) (span (offset 132) (line 4) (column 17) (len 13)))))
    (reference r3 (scope relative) (span (offset 291) (line 13) (column 26) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 291) (line 13) (column 26) (len 6)))))
    (reference r4 (scope relative) (span (offset 363) (line 16) (column 28) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 363) (line 16) (column 28) (len 15)))))
    (reference r5 (scope relative) (span (offset 637) (line 26) (column 27) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 637) (line 26) (column 27) (len 15)))))
    (reference r6 (scope relative) (span (offset 681) (line 27) (column 28) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 681) (line 27) (column 28) (len 16)))))
    (reference r7 (scope relative) (span (offset 1361) (line 51) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1361) (line 51) (column 24) (len 6)))))
    (reference r8 (scope relative) (span (offset 1401) (line 53) (column 29) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 1401) (line 53) (column 29) (len 16)))))
    (reference r9 (scope relative) (span (offset 1629) (line 63) (column 23) (len 14)) (segments (segment 0 (token "PubSubSequence") (name "PubSubSequence") (separator none) (span (offset 1629) (line 63) (column 23) (len 14)))))
    (reference r10 (scope relative) (span (offset 1669) (line 64) (column 24) (len 10)) (segments (segment 0 (token "producer_2") (name "producer_2") (separator none) (span (offset 1669) (line 64) (column 24) (len 10)))))
    (reference r11 (scope relative) (span (offset 1691) (line 65) (column 10) (len 24)) (segments (segment 0 (token "producerBehavior") (name "producerBehavior") (separator none) (span (offset 1691) (line 65) (column 10) (len 16))) (segment 1 (token "publish") (name "publish") (separator dot) (span (offset 1708) (line 65) (column 27) (len 7)))))
    (reference r12 (scope relative) (span (offset 1771) (line 68) (column 22) (len 8)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 1771) (line 68) (column 22) (len 8)))))
    (reference r13 (scope relative) (span (offset 1791) (line 69) (column 10) (len 35)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1791) (line 69) (column 10) (len 14))) (segment 1 (token "subscribing") (name "subscribing") (separator dot) (span (offset 1806) (line 69) (column 25) (len 11))) (segment 2 (token "accepter") (name "accepter") (separator dot) (span (offset 1818) (line 69) (column 37) (len 8)))))
    (reference r14 (scope relative) (span (offset 1867) (line 70) (column 10) (len 34)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1867) (line 70) (column 10) (len 14))) (segment 1 (token "delivering") (name "delivering") (separator dot) (span (offset 1882) (line 70) (column 25) (len 10))) (segment 2 (token "accepter") (name "accepter") (separator dot) (span (offset 1893) (line 70) (column 36) (len 8)))))
    (reference r15 (scope relative) (span (offset 1940) (line 71) (column 10) (len 32)) (segments (segment 0 (token "serverBehavior") (name "serverBehavior") (separator none) (span (offset 1940) (line 71) (column 10) (len 14))) (segment 1 (token "delivering") (name "delivering") (separator dot) (span (offset 1955) (line 71) (column 25) (len 10))) (segment 2 (token "effect") (name "effect") (separator dot) (span (offset 1966) (line 71) (column 36) (len 6)))))
    (reference r16 (scope relative) (span (offset 2032) (line 74) (column 24) (len 10)) (segments (segment 0 (token "consumer_2") (name "consumer_2") (separator none) (span (offset 2032) (line 74) (column 24) (len 10)))))
    (reference r17 (scope relative) (span (offset 2054) (line 75) (column 10) (len 26)) (segments (segment 0 (token "consumerBehavior") (name "consumerBehavior") (separator none) (span (offset 2054) (line 75) (column 10) (len 16))) (segment 1 (token "subscribe") (name "subscribe") (separator dot) (span (offset 2071) (line 75) (column 27) (len 9)))))
    (reference r18 (scope relative) (span (offset 2121) (line 76) (column 10) (len 25)) (segments (segment 0 (token "consumerBehavior") (name "consumerBehavior") (separator none) (span (offset 2121) (line 76) (column 10) (len 16))) (segment 1 (token "delivery") (name "delivery") (separator dot) (span (offset 2138) (line 76) (column 27) (len 8)))))
  )
  (root (package (name "ServerSequenceRealization_2") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 22))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 36) (len 3))) (separator (span (offset 111) (line 3) (column 36) (len 2))) (marker (span (offset 113) (line 3) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 132) (line 4) (column 17) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 145) (line 4) (column 30) (len 3))) (separator (span (offset 145) (line 4) (column 30) (len 2))) (marker (span (offset 147) (line 4) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Configuration") (body brace (port-def (name "PublicationPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "SubscriptionPort") (modifiers) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 261) (line 12) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 261) (line 12) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "someTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "somePublication") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (declaration "producerBehavior") (action none) (typing none) (subsets none) (redefines none) (body brace (action))))) (interface-usage (form connection) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 606) (line 25) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 606) (line 25) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))) (interface-usage (form connection) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1333) (line 50) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 1333) (line 50) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "myTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_usage_body_element") (found "perform action consumerBehavior {") (span (offset 1426) (line 55) (column 4) (len 171))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "realization_2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r10))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r11)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r12))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r13)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r14)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r15)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r16))) (value none))) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r17)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r18)) (body semicolon)))) (malformed (code "recovered_part_usage_body_element") (found "flow :>> publish_message: Transfers::MessageTransfer {") (span (offset 2183) (line 79) (column 3) (len 158))) (malformed (code "recovered_part_usage_body_element") (found "flow :>> subscribe_message: Transfers::MessageTransfer {") (span (offset 2341) (line 83) (column 3) (len 162))) (malformed (code "recovered_part_usage_body_element") (found "flow :>> deliver_message: Transfers::MessageTransfer {") (span (offset 2503) (line 87) (column 3) (len 218))) (bind) (bind) (bind) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2960) (line 97) (column 8) (len 9)) (normalized "Accepts "))) (bind) (bind) (bind))))))
)
~~~
