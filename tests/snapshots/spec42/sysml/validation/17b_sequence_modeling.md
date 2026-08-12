# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (17-Sequence Modeling): 17b-Sequence-Modeling"))
~~~
# SOURCE
~~~sysml
package '17b-Sequence-Modeling' {
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
			event publish_message.sourceEvent;
		}
		
		message publish_message of Publish[1];
		
		part server[1] {
			event subscribe_message.targetEvent;
			then event publish_message.targetEvent;
			then event deliver_message.sourceEvent;
		}
		
		message subscribe_message of Subscribe[1];
		message deliver_message of Deliver[1];
		
		part consumer[1] {
			event subscribe_message.sourceEvent;
			then event deliver_message.targetEvent;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17b_sequence_modeling.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '17b-Sequence-Modeling' {
    private import ScalarValues::*;
    private import PayloadDefinitions::*;
    package PayloadDefinitions {
        item def Subscribe {
            attribute topic : String;
            ref subscriber;
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
            event publish_message.sourceEvent;
        }
        message publish_message of Publish[1];
        part server[1] {
            event subscribe_message.targetEvent;
            then event publish_message.targetEvent;
            then event deliver_message.sourceEvent;
        }
        message subscribe_message of Subscribe[1];
        message deliver_message of Deliver[1];
        part consumer[1] {
            event subscribe_message.sourceEvent;
            then event deliver_message.targetEvent;
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
  )
  (root (package (name "17b-Sequence-Modeling") (body (import (target (span (span (offset 50) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 62) (line 2) (column 29) (len 3))) (separator (span (offset 62) (line 2) (column 29) (len 2))) (marker (span (offset 64) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 83) (line 3) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 101) (line 3) (column 35) (len 3))) (separator (span (offset 101) (line 3) (column 35) (len 2))) (marker (span (offset 103) (line 3) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "PayloadDefinitions") (body (item-def) (item-def) (item-def))) (occurrence-def))))
)
~~~
