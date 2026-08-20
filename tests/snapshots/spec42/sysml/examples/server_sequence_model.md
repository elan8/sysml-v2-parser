# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Interaction Sequencing): ServerSequenceModel"))
~~~
# SOURCE
~~~sysml
package ServerSequenceModel {
	private import ScalarValues::String;
	public import SignalDefinitions::*;

	package SignalDefinitions {
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

	part def PubSubSequence {
		part producer[1] {
			event occurrence publish_source_event;
		}
		
		message publish_message from producer.publish_source_event to server.publish_target_event;
		
		part server[1] {
			event occurrence subscribe_target_event;
			then event occurrence publish_target_event;
			then event occurrence deliver_source_event;
		}
		
		message subscribe_message from consumer.subscribe_source_event to server.subscribe_target_event;
		message deliver_message from server.deliver_source_event to consumer.deliver_target_event;
		
		part consumer {
			event occurrence subscribe_source_event;
			then event occurrence deliver_target_event;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "server_sequence_model.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ServerSequenceModel {
    private import ScalarValues::String;
    public import SignalDefinitions::*;
    package SignalDefinitions {
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
    part def PubSubSequence {
        part producer[1] {
            event occurrence publish_source_event;
        }
        message publish_message from producer.publish_source_event to server.publish_target_event;
        part server[1] {
            event occurrence subscribe_target_event;
            then event occurrence publish_target_event;
            then event occurrence deliver_source_event;
        }
        message subscribe_message from consumer.subscribe_source_event to server.subscribe_target_event;
        message deliver_message from server.deliver_source_event to consumer.deliver_target_event;
        part consumer {
            event occurrence subscribe_source_event;
            then event occurrence deliver_target_event;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 46) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 46) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 60) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 83) (line 3) (column 16) (len 17)) (segments (segment 0 (token "SignalDefinitions") (name "SignalDefinitions") (separator none) (span (offset 83) (line 3) (column 16) (len 17)))))
    (reference r2 (scope relative) (span (offset 185) (line 7) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 185) (line 7) (column 25) (len 6)))))
    (reference r3 (scope relative) (span (offset 275) (line 12) (column 22) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 275) (line 12) (column 22) (len 6)))))
  )
  (root (package (name "ServerSequenceModel") (body brace (import (target (span (span (offset 46) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 83) (line 3) (column 16) (len 20))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 100) (line 3) (column 33) (len 3))) (separator (span (offset 100) (line 3) (column 33) (len 2))) (marker (span (offset 102) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "SignalDefinitions") (body brace (item-def (name "Subscribe") (modifiers) (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "subscriber") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-def (name "Publish") (modifiers) (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))) (item-def (name "Deliver") (modifiers) (individual false) (specializes none) (body brace (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))) (part-def (name "PubSubSequence") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 402) (line 22) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 402) (line 22) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "publish_source_event") (short-name none) (target none) (body semicolon)))) (flow-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 566) (line 28) (column 15) (len 1)) (integer 1))) (upper (expression (span (offset 566) (line 28) (column 15) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "subscribe_target_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "publish_target_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "deliver_source_event") (short-name none) (target none) (body semicolon)))) (flow-usage) (flow-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "subscribe_source_event") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "deliver_target_event") (short-name none) (target none) (body semicolon)))))))))
)
~~~
