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
        part :>> localClock = new Time::Clock();
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
            transition accept after 5 ['SI::min'] then waiting;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 48) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 62) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 140) (line 8) (column 12) (len 10)) (segments (segment 0 (token "localClock") (name "localClock") (separator none) (span (offset 140) (line 8) (column 12) (len 10)))))
    (reference r2 (scope relative) (span (offset 157) (line 8) (column 29) (len 11)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 157) (line 8) (column 29) (len 4))) (segment 1 (token "Clock") (name "Clock") (separator colon-colon) (span (offset 163) (line 8) (column 35) (len 5)))))
    (reference r3 (scope relative) (span (offset 193) (line 10) (column 21) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 193) (line 10) (column 21) (len 6)))))
  )
  (root (package (name "Local Clock Example") (body brace (import (target (span (span (offset 48) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (item-def (name "Start") (individual false) (specializes none) (body semicolon)) (item-def (name "Request") (individual false) (specializes none) (body semicolon)) (part-def (name "Server") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 153) (line 8) (column 25) (len 17)) (constructor (type (ref r2)) (arguments)))))) (body semicolon)) (attribute-usage (declaration-name "today") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "requestPort") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage))))))
)
~~~
