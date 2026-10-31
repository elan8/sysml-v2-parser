# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (17-Sequence Modeling): 17a-Sequence-Modeling"))
~~~
# SOURCE
~~~sysml
package '17a-Sequence-Modeling' {
	private import ScalarValues::*;
	private import PayloadDefinitions::*;

	package PayloadDefinitions {
	    item def Subscribe {
	    	attribute topic : String;
	    	ref part subscriber;
	    }
	    
		item def Publish {
			attribute topic : String;
			ref publication;
		}
		
		item def Deliver {
			ref publication;
		}
	}

	occurrence def PubSubSequence {
		part producer[1] {
			event occurrence publish_source_event;
		}
		
		message publish_message of Publish[1] from producer.publish_source_event to server.publish_target_event;
		
		part server[1] {
			event occurrence subscribe_target_event;
			then event occurrence publish_target_event;
			then event occurrence deliver_source_event;
		}
		
		message subscribe_message of Subscribe[1] from consumer.subscribe_source_event to server.subscribe_target_event;
		message deliver_message of Deliver[1] from server.deliver_source_event to consumer.deliver_target_event;
		
		part consumer[1] {
			event occurrence subscribe_source_event;
			then event occurrence deliver_target_event;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17a_sequence_modeling.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '17a-Sequence-Modeling' {
    private import ScalarValues::*;
    private import PayloadDefinitions::*;
    package PayloadDefinitions {
        item def Subscribe {
            attribute topic : String;
            ref part subscriber;
        }
        item def Publish {
            attribute topic : String;
            ref publication;
        }
        item def Deliver {
            ref publication;
        }
    }
    occurrence def PubSubSequence {
        part producer[1] {
            event occurrence publish_source_event;
        }
        message publish_message of Publish[1] from producer.publish_source_event to server.publish_target_event;
        part server[1] {
            event occurrence subscribe_target_event;
            then event occurrence publish_target_event;
            then event occurrence deliver_source_event;
        }
        message subscribe_message of Subscribe[1] from consumer.subscribe_source_event to server.subscribe_target_event;
        message deliver_message of Deliver[1] from server.deliver_source_event to consumer.deliver_target_event;
        part consumer[1] {
            event occurrence subscribe_source_event;
            then event occurrence deliver_target_event;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 50) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 83) (line 3) (column 17) (len 18)) (segments (segment 0 (token "PayloadDefinitions") (name "PayloadDefinitions") (separator none) (span (offset 83) (line 3) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 187) (line 7) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 187) (line 7) (column 25) (len 6)))))
    (reference r3 (scope relative) (span (offset 277) (line 12) (column 22) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 277) (line 12) (column 22) (len 6)))))
    (reference r4 (scope relative) (span (offset 493) (line 26) (column 30) (len 7)) (segments (segment 0 (token "Publish") (name "Publish") (separator none) (span (offset 493) (line 26) (column 30) (len 7)))))
    (reference r5 (scope relative) (span (offset 509) (line 26) (column 46) (len 29)) (segments (segment 0 (token "producer") (name "producer") (separator none) (span (offset 509) (line 26) (column 46) (len 8))) (segment 1 (token "publish_source_event") (name "publish_source_event") (separator dot) (span (offset 518) (line 26) (column 55) (len 20)))))
    (reference r6 (scope relative) (span (offset 542) (line 26) (column 79) (len 27)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 542) (line 26) (column 79) (len 6))) (segment 1 (token "publish_target_event") (name "publish_target_event") (separator dot) (span (offset 549) (line 26) (column 86) (len 20)))))
    (reference r7 (scope relative) (span (offset 769) (line 34) (column 32) (len 9)) (segments (segment 0 (token "Subscribe") (name "Subscribe") (separator none) (span (offset 769) (line 34) (column 32) (len 9)))))
    (reference r8 (scope relative) (span (offset 787) (line 34) (column 50) (len 31)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 787) (line 34) (column 50) (len 8))) (segment 1 (token "subscribe_source_event") (name "subscribe_source_event") (separator dot) (span (offset 796) (line 34) (column 59) (len 22)))))
    (reference r9 (scope relative) (span (offset 822) (line 34) (column 85) (len 29)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 822) (line 34) (column 85) (len 6))) (segment 1 (token "subscribe_target_event") (name "subscribe_target_event") (separator dot) (span (offset 829) (line 34) (column 92) (len 22)))))
    (reference r10 (scope relative) (span (offset 882) (line 35) (column 30) (len 7)) (segments (segment 0 (token "Deliver") (name "Deliver") (separator none) (span (offset 882) (line 35) (column 30) (len 7)))))
    (reference r11 (scope relative) (span (offset 898) (line 35) (column 46) (len 27)) (segments (segment 0 (token "server") (name "server") (separator none) (span (offset 898) (line 35) (column 46) (len 6))) (segment 1 (token "deliver_source_event") (name "deliver_source_event") (separator dot) (span (offset 905) (line 35) (column 53) (len 20)))))
    (reference r12 (scope relative) (span (offset 929) (line 35) (column 77) (len 29)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 929) (line 35) (column 77) (len 8))) (segment 1 (token "deliver_target_event") (name "deliver_target_event") (separator dot) (span (offset 938) (line 35) (column 86) (len 20)))))
  )
  (root (package (name "17a-Sequence-Modeling") (body brace (import (visibility private) (target (span (span (offset 50) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 62) (line 2) (column 29) (len 3))) (separator (span (offset 62) (line 2) (column 29) (len 2))) (marker (span (offset 64) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 83) (line 3) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 101) (line 3) (column 35) (len 3))) (separator (span (offset 101) (line 3) (column 35) (len 2))) (marker (span (offset 103) (line 3) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (package (name "PayloadDefinitions") (body brace (item-def (name "Subscribe") (modifiers) (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "subscriber") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (item-def (name "Publish") (modifiers) (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))) (item-def (name "Deliver") (modifiers) (individual false) (specializes none) (body brace (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))) (occurrence-def (name "PubSubSequence") (short-name none) (modifiers) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 410) (line 22) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "publish_source_event") (short-name none) (target none) (body semicolon)))) (flow-usage (kind message) (visibility none) (declaration (declared (name "publish_message") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 490) (line 26) (column 27) (len 2))) (feature (name none) (type (ref r4)) (conjugated false) (multiplicity (lower none) (upper (expression (span (offset 501) (line 26) (column 38) (len 1)) (integer 1))))))) (endpoints (from (connector-end (multiplicity none) (target (ref r5)) (references none))) (to (connector-end (multiplicity none) (target (ref r6)) (references none)))))) (body (body semicolon))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 588) (line 28) (column 15) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "subscribe_target_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "publish_target_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "deliver_source_event") (short-name none) (target none) (body semicolon)))) (flow-usage (kind message) (visibility none) (declaration (declared (name "subscribe_message") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 766) (line 34) (column 29) (len 2))) (feature (name none) (type (ref r7)) (conjugated false) (multiplicity (lower none) (upper (expression (span (offset 779) (line 34) (column 42) (len 1)) (integer 1))))))) (endpoints (from (connector-end (multiplicity none) (target (ref r8)) (references none))) (to (connector-end (multiplicity none) (target (ref r9)) (references none)))))) (body (body semicolon))) (flow-usage (kind message) (visibility none) (declaration (declared (name "deliver_message") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 879) (line 35) (column 27) (len 2))) (feature (name none) (type (ref r10)) (conjugated false) (multiplicity (lower none) (upper (expression (span (offset 890) (line 35) (column 38) (len 1)) (integer 1))))))) (endpoints (from (connector-end (multiplicity none) (target (ref r11)) (references none))) (to (connector-end (multiplicity none) (target (ref r12)) (references none)))))) (body (body semicolon))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 979) (line 37) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "subscribe_source_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "deliver_target_event") (short-name none) (target none) (body semicolon)))))))))
)
~~~
