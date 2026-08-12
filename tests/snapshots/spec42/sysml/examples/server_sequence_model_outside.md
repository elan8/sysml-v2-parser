# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Interaction Sequencing): ServerSequenceModelOutside"))
~~~
# SOURCE
~~~sysml
package ServerSequenceModelOutside {
	public import ServerSequenceModel::*;

	part def PubSubSequenceOutside :> PubSubSequence {
		part :>> producer {
			event publish_source_event = publish_message.start;
		}
		
		part :>> server {
			event occurrence :>> subscribe_target_event = subscribe_message.done;
			then event occurrence :>> publish_target_event = publish_message.done;
			then event occurrence :>> deliver_source_event = deliver_message.start;
		}
		
		part :>> consumer {  /* Redundant with timing constraints on server and generic transfers. */
			event occurrence :>> subscribe_source_event = subscribe_message.start;
			then event occurrence :>> deliver_target_event = deliver_message.done;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "server_sequence_model_outside.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 154) (line 6) (column 4) (len 54)) (message "unexpected token in part usage body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 154) (line 6) (column 4) (len 54)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ServerSequenceModelOutside {
    public import ServerSequenceModel::*;
    part def PubSubSequenceOutside :> PubSubSequence {
        part  :>> producer {
            event publish_source_event = publish_message.start;
        }
        part  :>> server {
            event occurrence :>> subscribe_target_event = subscribe_message.done;
			then event occurrence :>> publish_target_event = publish_message.done;
			then event occurrence :>> deliver_source_event = deliver_message.start;
        }
        part  :>> consumer {
            event occurrence :>> subscribe_source_event = subscribe_message.start;
			then event occurrence :>> deliver_target_event = deliver_message.done;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 16) (len 19)) (segments (segment 0 (token "ServerSequenceModel") (name "ServerSequenceModel") (separator none) (span (offset 52) (line 2) (column 16) (len 19)))))
  )
  (root (package (name "ServerSequenceModelOutside") (body (import (target (span (span (offset 52) (line 2) (column 16) (len 22))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 35) (len 3))) (separator (span (offset 71) (line 2) (column 35) (len 2))) (marker (span (offset 73) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "PubSubSequenceOutside") (body (part-usage) (part-usage) (part-usage))))))
)
~~~
