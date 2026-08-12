# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 25 (Transitions): Local Clock Example"))
~~~
# SOURCE
~~~sysml
package 'Local Clock Example' {
	private import ScalarValues::String;
	
	item def Start;
	item def Request;
	
	part def Server {
		part :>> localClock = new Time::Clock();

		attribute today : String;
				
		port requestPort;
		
		state ServerBehavior {
			entry; then off;
			
			state off;
			accept Start via requestPort
				then waiting;
			
			state waiting;
			accept request : Request via requestPort
				then responding;
			accept at new Time::Iso8601DateTime(today + "11:59:00")
				then off;
			
			state responding;
			accept after 5 [SI::min]
				then waiting;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "25_local_clock_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Local Clock Example' {
    private import ScalarValues::String;
    item def Start;
    item def Request;
    part def Server {
        part  :>> localClock = new Time::Clock();
        attribute today : String;
        port requestPort;
        state ServerBehavior {
            entry;
            then off;
            state off;
            transition accept Start via requestPort then waiting;
            state waiting;
            transition accept request : Request via requestPort then responding;
            transition accept at new Time::Iso8601DateTime(today + "11:59:00") then off;
            state responding;
            transition accept after 5 [SI::min] then waiting;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 48) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 62) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 193) (line 10) (column 21) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 193) (line 10) (column 21) (len 6)))))
  )
  (root (package (name "Local Clock Example") (body (import (target (span (span (offset 48) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (item-def) (item-def) (part-def (name "Server") (body (part-usage) (attribute-usage (declaration-name "today") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "requestPort") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))))))
)
~~~
