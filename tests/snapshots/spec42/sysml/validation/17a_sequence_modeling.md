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
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 50) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 83) (line 3) (column 17) (len 18)) (segments (segment 0 (token "PayloadDefinitions") (name "PayloadDefinitions") (separator none) (span (offset 83) (line 3) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 187) (line 7) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 187) (line 7) (column 25) (len 6)))))
    (reference r3 (scope relative) (span (offset 277) (line 12) (column 22) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 277) (line 12) (column 22) (len 6)))))
  )
  (root (package (name "17a-Sequence-Modeling") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 62) (line 2) (column 29) (len 3))) (separator (span (offset 62) (line 2) (column 29) (len 2))) (marker (span (offset 64) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 83) (line 3) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 101) (line 3) (column 35) (len 3))) (separator (span (offset 101) (line 3) (column 35) (len 2))) (marker (span (offset 103) (line 3) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "PayloadDefinitions") (body brace (item-def (name "Subscribe") (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "subscriber") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-def (name "Publish") (individual false) (specializes none) (body brace (attribute-usage (declaration-name "topic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))) (item-def (name "Deliver") (individual false) (specializes none) (body brace (ref (name "publication") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines none) (subsets none) (body semicolon)))))) (occurrence-def))))
)
~~~
