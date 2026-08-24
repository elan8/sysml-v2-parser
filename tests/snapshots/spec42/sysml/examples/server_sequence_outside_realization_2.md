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
            /* Requiring FIFO sort (as opposed to just default) to make arrival/leave ordering
			 * in ServerSequenceModelOutside.sysml equivalent to accept/send new ordering in
			 * ServerSquenceRealization-2.sysml. */
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
    (reference r0 (scope relative) (span (offset 61) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 61) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 75) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 99) (line 3) (column 17) (len 26)) (segments (segment 0 (token "ServerSequenceModelOutside") (name "ServerSequenceModelOutside") (separator none) (span (offset 99) (line 3) (column 17) (len 26)))))
    (reference r2 (scope relative) (span (offset 146) (line 4) (column 17) (len 13)) (segments (segment 0 (token "Configuration") (name "Configuration") (separator none) (span (offset 146) (line 4) (column 17) (len 13)))))
    (reference r3 (scope relative) (span (offset 305) (line 13) (column 26) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 305) (line 13) (column 26) (len 6)))))
    (reference r4 (scope relative) (span (offset 566) (line 18) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 566) (line 18) (column 8) (len 20)))))
    (reference r5 (scope relative) (span (offset 589) (line 18) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 589) (line 18) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 602) (line 18) (column 44) (len 32)))))
    (reference r6 (scope relative) (span (offset 667) (line 20) (column 28) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 667) (line 20) (column 28) (len 15)))))
    (reference r7 (scope relative) (span (offset 834) (line 27) (column 13) (len 26)) (segments (segment 0 (token "producer_2") (name "producer_2") (separator none) (span (offset 834) (line 27) (column 13) (len 10))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 845) (line 27) (column 24) (len 15)))))
    (reference r8 (scope relative) (span (offset 864) (line 27) (column 43) (len 24)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 864) (line 27) (column 43) (len 8))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 873) (line 27) (column 52) (len 15)))))
    (reference r9 (scope relative) (span (offset 940) (line 30) (column 27) (len 15)) (segments (segment 0 (token "PublicationPort") (name "PublicationPort") (separator none) (span (offset 940) (line 30) (column 27) (len 15)))))
    (reference r10 (scope relative) (span (offset 984) (line 31) (column 28) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 984) (line 31) (column 28) (len 16)))))
    (reference r11 (scope relative) (span (offset 1009) (line 32) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 1009) (line 32) (column 8) (len 20)))))
    (reference r12 (scope relative) (span (offset 1032) (line 32) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1032) (line 32) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 1045) (line 32) (column 44) (len 32)))))
    (reference r13 (scope relative) (span (offset 1634) (line 53) (column 13) (len 27)) (segments (segment 0 (token "consumer_2") (name "consumer_2") (separator none) (span (offset 1634) (line 53) (column 13) (len 10))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 1645) (line 53) (column 24) (len 16)))))
    (reference r14 (scope relative) (span (offset 1665) (line 53) (column 44) (len 25)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 1665) (line 53) (column 44) (len 8))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 1674) (line 53) (column 53) (len 16)))))
    (reference r15 (scope relative) (span (offset 1741) (line 56) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1741) (line 56) (column 24) (len 6)))))
    (reference r16 (scope relative) (span (offset 1756) (line 57) (column 8) (len 20)) (segments (segment 0 (token "incomingTransferSort") (name "incomingTransferSort") (separator none) (span (offset 1756) (line 57) (column 8) (len 20)))))
    (reference r17 (scope relative) (span (offset 1779) (line 57) (column 31) (len 45)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 1779) (line 57) (column 31) (len 11))) (segment 1 (token "earlierFirstIncomingTransferSort") (name "earlierFirstIncomingTransferSort") (separator colon-colon) (span (offset 1792) (line 57) (column 44) (len 32)))))
    (reference r18 (scope relative) (span (offset 1858) (line 59) (column 29) (len 16)) (segments (segment 0 (token "SubscriptionPort") (name "SubscriptionPort") (separator none) (span (offset 1858) (line 59) (column 29) (len 16)))))
    (reference r19 (scope relative) (span (offset 2086) (line 69) (column 23) (len 14)) (segments (segment 0 (token "PubSubSequence") (name "PubSubSequence") (separator none) (span (offset 2086) (line 69) (column 23) (len 14)))))
    (reference r20 (scope relative) (span (offset 2126) (line 70) (column 24) (len 10)) (segments (segment 0 (token "producer_2") (name "producer_2") (separator none) (span (offset 2126) (line 70) (column 24) (len 10)))))
    (reference r21 (scope relative) (span (offset 2159) (line 71) (column 22) (len 8)) (segments (segment 0 (token "server_2") (name "server_2") (separator none) (span (offset 2159) (line 71) (column 22) (len 8)))))
    (reference r22 (scope relative) (span (offset 2192) (line 72) (column 24) (len 10)) (segments (segment 0 (token "consumer_2") (name "consumer_2") (separator none) (span (offset 2192) (line 72) (column 24) (len 10)))))
    (reference r23 (scope relative) (span (offset 2233) (line 74) (column 29) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2233) (line 74) (column 29) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2244) (line 74) (column 40) (len 15)))))
    (reference r24 (scope relative) (span (offset 2216) (line 74) (column 12) (len 15)) (segments (segment 0 (token "publish_message") (name "publish_message") (separator none) (span (offset 2216) (line 74) (column 12) (len 15)))))
    (reference r25 (scope relative) (span (offset 2285) (line 75) (column 24) (len 24)) (segments (segment 0 (token "producer") (name "producer") (separator none) (span (offset 2285) (line 75) (column 24) (len 8))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 2294) (line 75) (column 33) (len 15)))))
    (reference r26 (scope relative) (span (offset 2274) (line 75) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2274) (line 75) (column 13) (len 6)))))
    (reference r27 (scope relative) (span (offset 2334) (line 76) (column 24) (len 22)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2334) (line 76) (column 24) (len 6))) (segment 1 (token "publicationPort") (name "publicationPort") (separator dot) (span (offset 2341) (line 76) (column 31) (len 15)))))
    (reference r28 (scope relative) (span (offset 2323) (line 76) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2323) (line 76) (column 13) (len 6)))))
    (reference r29 (scope relative) (span (offset 2393) (line 78) (column 31) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2393) (line 78) (column 31) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2404) (line 78) (column 42) (len 15)))))
    (reference r30 (scope relative) (span (offset 2374) (line 78) (column 12) (len 17)) (segments (segment 0 (token "subscribe_message") (name "subscribe_message") (separator none) (span (offset 2374) (line 78) (column 12) (len 17)))))
    (reference r31 (scope relative) (span (offset 2445) (line 79) (column 24) (len 25)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 2445) (line 79) (column 24) (len 8))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 2454) (line 79) (column 33) (len 16)))))
    (reference r32 (scope relative) (span (offset 2434) (line 79) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2434) (line 79) (column 13) (len 6)))))
    (reference r33 (scope relative) (span (offset 2495) (line 80) (column 24) (len 23)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2495) (line 80) (column 24) (len 6))) (segment 1 (token "subscriptionPort") (name "subscriptionPort") (separator dot) (span (offset 2502) (line 80) (column 31) (len 16)))))
    (reference r34 (scope relative) (span (offset 2484) (line 80) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2484) (line 80) (column 13) (len 6)))))
    (reference r35 (scope relative) (span (offset 2553) (line 82) (column 29) (len 26)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 2553) (line 82) (column 29) (len 9))) (segment 1 (token "MessageTransfer") (name "MessageTransfer") (separator colon-colon) (span (offset 2564) (line 82) (column 40) (len 15)))))
    (reference r36 (scope relative) (span (offset 2536) (line 82) (column 12) (len 15)) (segments (segment 0 (token "deliver_message") (name "deliver_message") (separator none) (span (offset 2536) (line 82) (column 12) (len 15)))))
    (reference r37 (scope relative) (span (offset 2605) (line 83) (column 24) (len 6)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 2605) (line 83) (column 24) (len 6)))))
    (reference r38 (scope relative) (span (offset 2594) (line 83) (column 13) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 2594) (line 83) (column 13) (len 6)))))
    (reference r39 (scope relative) (span (offset 2636) (line 84) (column 24) (len 8)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 2636) (line 84) (column 24) (len 8)))))
    (reference r40 (scope relative) (span (offset 2625) (line 84) (column 13) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 2625) (line 84) (column 13) (len 6)))))
  )
  (root (package (name "ServerSequenceOutsideRealization_2") (body brace (import (target (span (span (offset 61) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 99) (line 3) (column 17) (len 29))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 125) (line 3) (column 43) (len 3))) (separator (span (offset 125) (line 3) (column 43) (len 2))) (marker (span (offset 127) (line 3) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 146) (line 4) (column 17) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 159) (line 4) (column 30) (len 3))) (separator (span (offset 159) (line 4) (column 30) (len 2))) (marker (span (offset 161) (line 4) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Configuration") (body brace (port-def (name "PublicationPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "SubscriptionPort") (modifiers) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 275) (line 12) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 275) (line 12) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "someTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "somePublication") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 351) (line 15) (column 6) (len 205)) (normalized "Requiring FIFO sort (as opposed to just default) to make arrival/leave ordering\nin ServerSequenceModelOutside.sysml equivalent to accept/send new ordering in\nServerSquenceRealization-2.sysml. "))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 589) (line 18) (column 31) (len 45)) (ref r5))))) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "producerBehavior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action))))) (interface-usage (form connection) (part (binary (from (interface-end (multiplicity none) (target (ref r7)))) (to (interface-end (multiplicity none) (target (ref r8)))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 909) (line 29) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 909) (line 29) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "publicationPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1032) (line 32) (column 31) (len 45)) (ref r12))))) (body semicolon)) (state-usage))) (interface-usage (form connection) (part (binary (from (interface-end (multiplicity none) (target (ref r13)))) (to (interface-end (multiplicity none) (target (ref r14)))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer_2") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1713) (line 55) (column 19) (len 1)) (integer 1))) (upper (expression (span (offset 1713) (line 55) (column 19) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "myTopic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1779) (line 57) (column 31) (len 45)) (ref r17))))) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subscriptionPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "consumerBehavior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action) (action))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "realization_2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r20))) (value none))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r21))) (value none))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r22))) (value none))) (redefines none) (value none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r25)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r27)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (crosses none))))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r31)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r33)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (crosses none))))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r36)))) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints none))) (body (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r37)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r39)))) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (crosses none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2660) (line 87) (column 6) (len 63)) (normalized "Binding sent/accept messages to specification model messages. "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2732) (line 88) (column 7) (len 7)) (normalized "Sends "))) (bind) (bind) (bind) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2984) (line 92) (column 8) (len 9)) (normalized "Accepts "))) (bind) (bind) (bind))))))
)
~~~
