# META
~~~sexpr
(snapshot (type semantic) (description "Exercises qualified references across imports, dependencies, requirements, views, use cases, state transitions, events, and exhibits."))
~~~
# SOURCE
~~~sysml
package Demo {
    private import $::Base::Types::*;
    private import Base::Recursive::**;
    private import Base::Filtered[Members::One];

    dependency from Client::A to $::Supplier::B;

    requirement def Req {
        subject 'item' : $::Domain::Subject = Vehicle.mass;
        verify Requirements::Mass :>> Base::Mass;
    }

    view overview : Views::General {
        expose Vehicle::structure.mass;
        expose Vehicle::filtered[Filters::visible];
        satisfy Viewpoints::Safety;
    }

    use case def Use {
        include Cases::UC;
    }

    requirement def Roles {
        stakeholder $::Concerns::Safety;
    }

    state def Machine {
        then Modes::ready;
    }

    event sequence.publishMessage;

    part def Vehicle {
        exhibit vehicleStates.on;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "semantic_references.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Demo {
    private import $::Base::Types::*;
    private import Base::Recursive::**;
    private import Base::Filtered [Members::One];
    dependency from Client::A to $::Supplier::B;
    requirement def Req {
        subject 'item' : $::Domain::Subject = Vehicle.mass;
        verify Requirements::Mass :>> Base::Mass;
    }
    view overview : Views::General {
        expose Vehicle::structure.mass;
        expose Vehicle::filtered [Filters::visible];
        satisfy Viewpoints::Safety;
    }
    use case def Use {
        include Cases::UC;
    }
    requirement def Roles {
        stakeholder $::Concerns::Safety;
    }
    state def Machine {
        then Modes::ready;
    }
    event sequence.publishMessage;
    part def Vehicle {
        exhibit vehicleStates.on;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope absolute) (span (offset 34) (line 2) (column 20) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 37) (line 2) (column 23) (len 4))) (segment 1 (token "Types") (name "Types") (separator colon-colon) (span (offset 43) (line 2) (column 29) (len 5)))))
    (reference r1 (scope relative) (span (offset 72) (line 3) (column 20) (len 15)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 72) (line 3) (column 20) (len 4))) (segment 1 (token "Recursive") (name "Recursive") (separator colon-colon) (span (offset 78) (line 3) (column 26) (len 9)))))
    (reference r2 (scope relative) (span (offset 112) (line 4) (column 20) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 112) (line 4) (column 20) (len 4))) (segment 1 (token "Filtered") (name "Filtered") (separator colon-colon) (span (offset 118) (line 4) (column 26) (len 8)))))
    (reference r3 (scope relative) (span (offset 127) (line 4) (column 35) (len 12)) (segments (segment 0 (token "Members") (name "Members") (separator none) (span (offset 127) (line 4) (column 35) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 136) (line 4) (column 44) (len 3)))))
    (reference r4 (scope relative) (span (offset 163) (line 6) (column 21) (len 9)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 163) (line 6) (column 21) (len 6))) (segment 1 (token "A") (name "A") (separator colon-colon) (span (offset 171) (line 6) (column 29) (len 1)))))
    (reference r5 (scope absolute) (span (offset 176) (line 6) (column 34) (len 14)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 179) (line 6) (column 37) (len 8))) (segment 1 (token "B") (name "B") (separator colon-colon) (span (offset 189) (line 6) (column 47) (len 1)))))
    (reference r6 (scope absolute) (span (offset 244) (line 9) (column 26) (len 18)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 247) (line 9) (column 29) (len 6))) (segment 1 (token "Subject") (name "Subject") (separator colon-colon) (span (offset 255) (line 9) (column 37) (len 7)))))
    (reference r7 (scope relative) (span (offset 265) (line 9) (column 47) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 265) (line 9) (column 47) (len 7)))))
    (reference r8 (scope relative) (span (offset 273) (line 9) (column 55) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 273) (line 9) (column 55) (len 4)))))
    (reference r9 (scope relative) (span (offset 294) (line 10) (column 16) (len 18)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 294) (line 10) (column 16) (len 12))) (segment 1 (token "Mass") (name "Mass") (separator colon-colon) (span (offset 308) (line 10) (column 30) (len 4)))))
    (reference r10 (scope relative) (span (offset 317) (line 10) (column 39) (len 10)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 317) (line 10) (column 39) (len 4))) (segment 1 (token "Mass") (name "Mass") (separator colon-colon) (span (offset 323) (line 10) (column 45) (len 4)))))
    (reference r11 (scope relative) (span (offset 356) (line 13) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 356) (line 13) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 363) (line 13) (column 28) (len 7)))))
    (reference r12 (scope relative) (span (offset 388) (line 14) (column 16) (len 23)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 388) (line 14) (column 16) (len 7))) (segment 1 (token "structure") (name "structure") (separator colon-colon) (span (offset 397) (line 14) (column 25) (len 9))) (segment 2 (token "mass") (name "mass") (separator dot) (span (offset 407) (line 14) (column 35) (len 4)))))
    (reference r13 (scope relative) (span (offset 428) (line 15) (column 16) (len 17)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 428) (line 15) (column 16) (len 7))) (segment 1 (token "filtered") (name "filtered") (separator colon-colon) (span (offset 437) (line 15) (column 25) (len 8)))))
    (reference r14 (scope relative) (span (offset 446) (line 15) (column 34) (len 16)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 446) (line 15) (column 34) (len 7))) (segment 1 (token "visible") (name "visible") (separator colon-colon) (span (offset 455) (line 15) (column 43) (len 7)))))
    (reference r15 (scope relative) (span (offset 481) (line 16) (column 17) (len 18)) (segments (segment 0 (token "Viewpoints") (name "Viewpoints") (separator none) (span (offset 481) (line 16) (column 17) (len 10))) (segment 1 (token "Safety") (name "Safety") (separator colon-colon) (span (offset 493) (line 16) (column 29) (len 6)))))
    (reference r16 (scope relative) (span (offset 547) (line 20) (column 17) (len 9)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 547) (line 20) (column 17) (len 5))) (segment 1 (token "UC") (name "UC") (separator colon-colon) (span (offset 554) (line 20) (column 24) (len 2)))))
    (reference r17 (scope absolute) (span (offset 613) (line 24) (column 21) (len 19)) (segments (segment 0 (token "Concerns") (name "Concerns") (separator none) (span (offset 616) (line 24) (column 24) (len 8))) (segment 1 (token "Safety") (name "Safety") (separator colon-colon) (span (offset 626) (line 24) (column 34) (len 6)))))
    (reference r18 (scope relative) (span (offset 678) (line 28) (column 14) (len 12)) (segments (segment 0 (token "Modes") (name "Modes") (separator none) (span (offset 678) (line 28) (column 14) (len 5))) (segment 1 (token "ready") (name "ready") (separator colon-colon) (span (offset 685) (line 28) (column 21) (len 5)))))
    (reference r19 (scope relative) (span (offset 709) (line 31) (column 11) (len 23)) (segments (segment 0 (token "sequence") (name "sequence") (separator none) (span (offset 709) (line 31) (column 11) (len 8))) (segment 1 (token "publishMessage") (name "publishMessage") (separator dot) (span (offset 718) (line 31) (column 20) (len 14)))))
    (reference r20 (scope relative) (span (offset 774) (line 34) (column 17) (len 16)) (segments (segment 0 (token "vehicleStates") (name "vehicleStates") (separator none) (span (offset 774) (line 34) (column 17) (len 13))) (segment 1 (token "on") (name "on") (separator dot) (span (offset 788) (line 34) (column 31) (len 2)))))
  )
  (root (package (name "Demo") (body brace (import (visibility private) (target (span (span (offset 34) (line 2) (column 20) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 48) (line 2) (column 34) (len 3))) (separator (span (offset 48) (line 2) (column 34) (len 2))) (marker (span (offset 50) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 72) (line 3) (column 20) (len 19))) (all none) (ref r1) (shape (membership (recursive-suffix (span (span (offset 87) (line 3) (column 35) (len 4))) (separator (span (offset 87) (line 3) (column 35) (len 2))) (marker (span (offset 89) (line 3) (column 37) (len 2))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 112) (line 4) (column 20) (len 28))) (all none) (ref r2) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 126) (line 4) (column 34) (len 14))) (open (span (offset 126) (line 4) (column 34) (len 1))) (expression (expression (span (offset 127) (line 4) (column 35) (len 12)) (ref r3))) (close (span (offset 139) (line 4) (column 47) (len 1)))))))) (body semicolon)) (dependency (clients (ref r4)) (suppliers (ref r5)) (body semicolon)) (requirement-def (name "Req") (modifiers) (body brace (subject (name "item") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 265) (line 9) (column 47) (len 12)) (member-access (base (expression (span (offset 265) (line 9) (column 47) (len 7)) (ref r7))) (separator dot) (member (ref r8))))))) (body semicolon)) (verify (explicit-requirement false) (requirement none) (target (ref r9)) (redefines (ref r10))))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "overview") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (expose (target (span (span (offset 388) (line 14) (column 16) (len 23))) (all none) (ref r12) (shape (membership (recursive-suffix none)))) (body semicolon)) (expose (target (span (span (offset 428) (line 15) (column 16) (len 35))) (all none) (ref r13) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 445) (line 15) (column 33) (len 18))) (open (span (offset 445) (line 15) (column 33) (len 1))) (expression (expression (span (offset 446) (line 15) (column 34) (len 16)) (ref r14))) (close (span (offset 462) (line 15) (column 50) (len 1)))))))) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r15))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (use-case-def (name "Use") (modifiers) (body brace (include (target (ref r16))))) (requirement-def (name "Roles") (modifiers) (body brace (stakeholder (declaration none) (short-name none) (target (ref r17)) (type none) (redefinition false)))) (state-def (name "Machine") (modifiers) (body brace (then (state (ref r18))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r19)) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body brace (exhibit (declaration none) (short-name none) (state (ref r20))))))))
)
~~~
