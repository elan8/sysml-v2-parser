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
  )
  (root (package (name "ServerSequenceModel") (body brace (import (target (span (span (offset 46) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 83) (line 3) (column 16) (len 20))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 100) (line 3) (column 33) (len 3))) (separator (span (offset 100) (line 3) (column 33) (len 2))) (marker (span (offset 102) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "SignalDefinitions") (body brace (item-def) (item-def) (item-def))) (part-def (name "PubSubSequence") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "producer") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 402) (line 22) (column 17) (len 1)) (integer 1))) (upper (expression (span (offset 402) (line 22) (column 17) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "publish_source_event") (short-name none) (target none)))) (flow-usage) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "server") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 566) (line 28) (column 15) (len 1)) (integer 1))) (upper (expression (span (offset 566) (line 28) (column 15) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "subscribe_target_event") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "publish_target_event") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "deliver_source_event") (short-name none) (target none)))) (flow-usage) (flow-usage) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "consumer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "subscribe_source_event") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "deliver_target_event") (short-name none) (target none)))))))))
)
~~~
