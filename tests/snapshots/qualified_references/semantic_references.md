# META
~~~sexpr
(snapshot (type semantic) (description "Exercises qualified references across imports, dependencies, requirements, views, use cases, state transitions, events, and exhibits."))
~~~
# SOURCE
~~~sysml
package Demo {
    private import $::Base::Types::*;
    import Base::Recursive::**;
    import Base::Filtered[Members::One];

    dependency from Client::A to $::Supplier::B;

    requirement def Req {
        subject item : $::Domain::Subject = Vehicle.mass;
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
    import Base::Recursive::**;
    import Base::Filtered [Members::One];
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
  (references
    (reference r0 (scope absolute) (span (offset 34) (line 2) (column 20) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 37) (line 2) (column 23) (len 4))) (segment 1 (token "Types") (name "Types") (separator colon-colon) (span (offset 43) (line 2) (column 29) (len 5)))))
    (reference r1 (scope relative) (span (offset 64) (line 3) (column 12) (len 15)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 64) (line 3) (column 12) (len 4))) (segment 1 (token "Recursive") (name "Recursive") (separator colon-colon) (span (offset 70) (line 3) (column 18) (len 9)))))
    (reference r2 (scope relative) (span (offset 96) (line 4) (column 12) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 96) (line 4) (column 12) (len 4))) (segment 1 (token "Filtered") (name "Filtered") (separator colon-colon) (span (offset 102) (line 4) (column 18) (len 8)))))
    (reference r3 (scope relative) (span (offset 111) (line 4) (column 27) (len 12)) (segments (segment 0 (token "Members") (name "Members") (separator none) (span (offset 111) (line 4) (column 27) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 120) (line 4) (column 36) (len 3)))))
    (reference r4 (scope relative) (span (offset 147) (line 6) (column 21) (len 9)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 147) (line 6) (column 21) (len 6))) (segment 1 (token "A") (name "A") (separator colon-colon) (span (offset 155) (line 6) (column 29) (len 1)))))
    (reference r5 (scope absolute) (span (offset 160) (line 6) (column 34) (len 14)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 163) (line 6) (column 37) (len 8))) (segment 1 (token "B") (name "B") (separator colon-colon) (span (offset 173) (line 6) (column 47) (len 1)))))
    (reference r6 (scope absolute) (span (offset 226) (line 9) (column 24) (len 18)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 229) (line 9) (column 27) (len 6))) (segment 1 (token "Subject") (name "Subject") (separator colon-colon) (span (offset 237) (line 9) (column 35) (len 7)))))
    (reference r7 (scope relative) (span (offset 247) (line 9) (column 45) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 247) (line 9) (column 45) (len 7)))))
    (reference r8 (scope relative) (span (offset 255) (line 9) (column 53) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 255) (line 9) (column 53) (len 4)))))
    (reference r9 (scope relative) (span (offset 276) (line 10) (column 16) (len 18)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 276) (line 10) (column 16) (len 12))) (segment 1 (token "Mass") (name "Mass") (separator colon-colon) (span (offset 290) (line 10) (column 30) (len 4)))))
    (reference r10 (scope relative) (span (offset 299) (line 10) (column 39) (len 10)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 299) (line 10) (column 39) (len 4))) (segment 1 (token "Mass") (name "Mass") (separator colon-colon) (span (offset 305) (line 10) (column 45) (len 4)))))
    (reference r11 (scope relative) (span (offset 338) (line 13) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 338) (line 13) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 345) (line 13) (column 28) (len 7)))))
    (reference r12 (scope relative) (span (offset 370) (line 14) (column 16) (len 23)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 370) (line 14) (column 16) (len 7))) (segment 1 (token "structure") (name "structure") (separator colon-colon) (span (offset 379) (line 14) (column 25) (len 9))) (segment 2 (token "mass") (name "mass") (separator dot) (span (offset 389) (line 14) (column 35) (len 4)))))
    (reference r13 (scope relative) (span (offset 410) (line 15) (column 16) (len 17)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 410) (line 15) (column 16) (len 7))) (segment 1 (token "filtered") (name "filtered") (separator colon-colon) (span (offset 419) (line 15) (column 25) (len 8)))))
    (reference r14 (scope relative) (span (offset 428) (line 15) (column 34) (len 16)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 428) (line 15) (column 34) (len 7))) (segment 1 (token "visible") (name "visible") (separator colon-colon) (span (offset 437) (line 15) (column 43) (len 7)))))
    (reference r15 (scope relative) (span (offset 463) (line 16) (column 17) (len 18)) (segments (segment 0 (token "Viewpoints") (name "Viewpoints") (separator none) (span (offset 463) (line 16) (column 17) (len 10))) (segment 1 (token "Safety") (name "Safety") (separator colon-colon) (span (offset 475) (line 16) (column 29) (len 6)))))
    (reference r16 (scope relative) (span (offset 529) (line 20) (column 17) (len 9)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 529) (line 20) (column 17) (len 5))) (segment 1 (token "UC") (name "UC") (separator colon-colon) (span (offset 536) (line 20) (column 24) (len 2)))))
    (reference r17 (scope absolute) (span (offset 595) (line 24) (column 21) (len 19)) (segments (segment 0 (token "Concerns") (name "Concerns") (separator none) (span (offset 598) (line 24) (column 24) (len 8))) (segment 1 (token "Safety") (name "Safety") (separator colon-colon) (span (offset 608) (line 24) (column 34) (len 6)))))
    (reference r18 (scope relative) (span (offset 660) (line 28) (column 14) (len 12)) (segments (segment 0 (token "Modes") (name "Modes") (separator none) (span (offset 660) (line 28) (column 14) (len 5))) (segment 1 (token "ready") (name "ready") (separator colon-colon) (span (offset 667) (line 28) (column 21) (len 5)))))
    (reference r19 (scope relative) (span (offset 691) (line 31) (column 11) (len 23)) (segments (segment 0 (token "sequence") (name "sequence") (separator none) (span (offset 691) (line 31) (column 11) (len 8))) (segment 1 (token "publishMessage") (name "publishMessage") (separator dot) (span (offset 700) (line 31) (column 20) (len 14)))))
    (reference r20 (scope relative) (span (offset 756) (line 34) (column 17) (len 16)) (segments (segment 0 (token "vehicleStates") (name "vehicleStates") (separator none) (span (offset 756) (line 34) (column 17) (len 13))) (segment 1 (token "on") (name "on") (separator dot) (span (offset 770) (line 34) (column 31) (len 2)))))
  )
  (root (package (name "Demo") (body brace (import (target (span (span (offset 34) (line 2) (column 20) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 48) (line 2) (column 34) (len 3))) (separator (span (offset 48) (line 2) (column 34) (len 2))) (marker (span (offset 50) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 64) (line 3) (column 12) (len 19))) (all none) (ref r1) (shape (membership (recursive-suffix (span (span (offset 79) (line 3) (column 27) (len 4))) (separator (span (offset 79) (line 3) (column 27) (len 2))) (marker (span (offset 81) (line 3) (column 29) (len 2)))))))) (import (target (span (span (offset 96) (line 4) (column 12) (len 28))) (all none) (ref r2) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 110) (line 4) (column 26) (len 14))) (open (span (offset 110) (line 4) (column 26) (len 1))) (expression (expression (span (offset 111) (line 4) (column 27) (len 12)) (ref r3))) (close (span (offset 123) (line 4) (column 39) (len 1))))))))) (dependency (clients (ref r4)) (suppliers (ref r5)) (body semicolon)) (requirement-def (name "Req") (body brace (subject (name "item") (type (ref r6)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 247) (line 9) (column 45) (len 12)) (member-access (base (expression (span (offset 247) (line 9) (column 45) (len 7)) (ref r7))) (separator dot) (member (ref r8)))))))) (verify (target (ref r9)) (redefines (ref r10))))) (view (name "overview") (short-name none) (type (ref r11)) (body brace (expose (target (span (span (offset 370) (line 14) (column 16) (len 23))) (all none) (ref r12) (shape (membership (recursive-suffix none)))) (body semicolon)) (expose (target (span (span (offset 410) (line 15) (column 16) (len 35))) (all none) (ref r13) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 427) (line 15) (column 33) (len 18))) (open (span (offset 427) (line 15) (column 33) (len 1))) (expression (expression (span (offset 428) (line 15) (column 34) (len 16)) (ref r14))) (close (span (offset 444) (line 15) (column 50) (len 1)))))))) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r15))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (use-case-def (name "Use") (body brace (include (target (ref r16))))) (requirement-def (name "Roles") (body brace (stakeholder (declaration "") (target (ref r17)) (type none) (redefinition false)))) (state-def (name "Machine") (body brace (then (state (ref r18))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target (ref r19))) (part-def (name "Vehicle") (body brace (exhibit (declaration "") (state (ref r20))))))))
)
~~~
