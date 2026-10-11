# META
~~~sexpr
(snapshot (type semantic) (description "Succession members and body expressions from the release corpus: a named time trigger as a `then` target, `then state` in a state body, `then message` with a valued payload in an occurrence definition body, a feature-chain succession in an interface usage body (#166), and members between the parameters and the result of a body expression (#167)."))
~~~
# SOURCE
~~~sysml
package Successions {
    action def Wait {
        first start;
        then accept sig after 10 [SI::s];
        then accept at deadline;
        then done;
    }
    state def Counting {
        entry assign count := 0;
        then state wait;
        state stateful;
        then stateful;
        accept Incr
            then wait;
    }
    occurrence def Control {
        message setSpeed of SetSpeed from a to b;
        then message sensed of SensedSpeed from b to c;
        then message command of fuelCommand : FuelCommand = sensed.fuelCommand from c to d;
    }
    part system {
        interface link : Link connect a to b {
            succession first call.start then ack.done;
        }
    }
    constraint def Dynamics {
        in n : Natural;
        (1..n)->forAll {
            in i : Natural;
            private attribute frame = frames#(i);
            private next : Sample = samples#(i + 1);
            frame.valid and next.valid
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "succession_targets_and_body_expression_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Successions {
    action def Wait {
        first start;
        then accept sig after 10[SI::s];
        then accept at deadline;
        then done;
    }
    state def Counting {
        entry assign count := 0;
        then state wait;
        state stateful;
        then stateful;
        transition accept Incr then wait;
    }
    occurrence def Control {
        message setSpeed of SetSpeed from a to b;
        then message sensed of SensedSpeed from b to c;
        then message command of fuelCommand : FuelCommand = sensed.fuelCommand from c to d;
    }
    part system {
        interface link : Link connect a to b {
            succession first call.start then ack.done;
        }
    }
    constraint def Dynamics {
        in n : Natural;
        (1 .. n)->forAll { in i : Natural; private attribute frame = frames#(i); private next : Sample = samples#(i + 1); frame.valid && next.valid };
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 3) (column 15) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 58) (line 3) (column 15) (len 5)))))
    (reference r1 (scope relative) (span (offset 285) (line 12) (column 14) (len 8)) (segments (segment 0 (token "stateful") (name "stateful") (separator none) (span (offset 285) (line 12) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 310) (line 13) (column 16) (len 4)) (segments (segment 0 (token "Incr") (name "Incr") (separator none) (span (offset 310) (line 13) (column 16) (len 4)))))
    (reference r3 (scope relative) (span (offset 332) (line 14) (column 18) (len 4)) (segments (segment 0 (token "wait") (name "wait") (separator none) (span (offset 332) (line 14) (column 18) (len 4)))))
    (reference r4 (scope relative) (span (offset 620) (line 22) (column 26) (len 4)) (segments (segment 0 (token "Link") (name "Link") (separator none) (span (offset 620) (line 22) (column 26) (len 4)))))
    (reference r5 (scope relative) (span (offset 633) (line 22) (column 39) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 633) (line 22) (column 39) (len 1)))))
    (reference r6 (scope relative) (span (offset 638) (line 22) (column 44) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 638) (line 22) (column 44) (len 1)))))
    (reference r7 (scope relative) (span (offset 779) (line 28) (column 13) (len 1)) (segments (segment 0 (token "n") (name "n") (separator none) (span (offset 779) (line 28) (column 13) (len 1)))))
    (reference r8 (scope relative) (span (offset 811) (line 29) (column 20) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 811) (line 29) (column 20) (len 7)))))
    (reference r9 (scope relative) (span (offset 858) (line 30) (column 39) (len 6)) (segments (segment 0 (token "frames") (name "frames") (separator none) (span (offset 858) (line 30) (column 39) (len 6)))))
    (reference r10 (scope relative) (span (offset 866) (line 30) (column 47) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 866) (line 30) (column 47) (len 1)))))
    (reference r11 (scope relative) (span (offset 897) (line 31) (column 28) (len 6)) (segments (segment 0 (token "Sample") (name "Sample") (separator none) (span (offset 897) (line 31) (column 28) (len 6)))))
    (reference r12 (scope relative) (span (offset 906) (line 31) (column 37) (len 7)) (segments (segment 0 (token "samples") (name "samples") (separator none) (span (offset 906) (line 31) (column 37) (len 7)))))
    (reference r13 (scope relative) (span (offset 915) (line 31) (column 46) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 915) (line 31) (column 46) (len 1)))))
    (reference r14 (scope relative) (span (offset 935) (line 32) (column 13) (len 5)) (segments (segment 0 (token "frame") (name "frame") (separator none) (span (offset 935) (line 32) (column 13) (len 5)))))
    (reference r15 (scope relative) (span (offset 941) (line 32) (column 19) (len 5)) (segments (segment 0 (token "valid") (name "valid") (separator none) (span (offset 941) (line 32) (column 19) (len 5)))))
    (reference r16 (scope relative) (span (offset 951) (line 32) (column 29) (len 4)) (segments (segment 0 (token "next") (name "next") (separator none) (span (offset 951) (line 32) (column 29) (len 4)))))
    (reference r17 (scope relative) (span (offset 956) (line 32) (column 34) (len 5)) (segments (segment 0 (token "valid") (name "valid") (separator none) (span (offset 956) (line 32) (column 34) (len 5)))))
  )
  (root (package (name "Successions") (body brace (action-def (name "Wait") (modifiers) (specializes none) (body brace (first (source (expression (span (offset 58) (line 3) (column 15) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 63) (line 3) (column 20) (len 1))))) (then-action) (then-action) (then-action))) (state-def (name "Counting") (modifiers) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect true) (body semicolon)) (state-usage (then true) (name "wait") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "stateful") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (then (state (ref r1))) (transition (name none) (short-name none) (source none) (initial false) (accept (shorthand (expression (span (offset 310) (line 13) (column 16) (len 4)) (ref r2)) (via none))) (guard none) (effect none) (target (expression (span (offset 332) (line 14) (column 18) (len 4)) (ref r3))) (body semicolon)))) (occurrence-def (modifiers)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "system") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "link") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r5)))) (to (interface-end (multiplicity none) (target (ref r6)))))) (body brace (succession-usage (keyword true) (name none) (short-name none)))))) (constraint-def (name "Dynamics") (modifiers) (specializes none) (body brace (in-out-declaration) (expression (span (offset 775) (line 28) (column 9) (len 196)) (collection-op (operator "forAll") (base (expression (span (offset 775) (line 28) (column 9) (len 6)) (sequence (sequence-list (element first (expression (span (offset 776) (line 28) (column 10) (len 4)) (binary (operator "..") (left (expression (span (offset 776) (line 28) (column 10) (len 1)) (integer 1))) (right (expression (span (offset 779) (line 28) (column 13) (len 1)) (ref r7)))))))))) (arguments) (brace-body (body (span (offset 790) (line 28) (column 24) (len 181)) (open-brace (span (offset 790) (line 28) (column 24) (len 1))) (parameters (parameter (span (offset 804) (line 29) (column 13) (len 15)) (direction in (span (offset 804) (line 29) (column 13) (len 2))) (reference-keyword none) (declaration (name "i") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 818) (line 29) (column 27) (len 1)))))) (members (body brace (attribute-usage (declaration-name "frame") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 858) (line 30) (column 39) (len 10)) (index (base (expression (span (offset 858) (line 30) (column 39) (len 6)) (ref r9))) (operands (sequence-list (element first (expression (span (offset 866) (line 30) (column 47) (len 1)) (ref r10)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "next") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 906) (line 31) (column 37) (len 15)) (index (base (expression (span (offset 906) (line 31) (column 37) (len 7)) (ref r12))) (operands (sequence-list (element first (expression (span (offset 915) (line 31) (column 46) (len 5)) (binary (operator "+") (left (expression (span (offset 915) (line 31) (column 46) (len 1)) (ref r13))) (right (expression (span (offset 919) (line 31) (column 50) (len 1)) (integer 1))))))))))))) (body semicolon)))) (result (expression (span (offset 935) (line 32) (column 13) (len 26)) (binary (operator "&&") (left (expression (span (offset 935) (line 32) (column 13) (len 11)) (member-access (base (expression (span (offset 935) (line 32) (column 13) (len 5)) (ref r14))) (separator dot) (member (ref r15))))) (right (expression (span (offset 951) (line 32) (column 29) (len 10)) (member-access (base (expression (span (offset 951) (line 32) (column 29) (len 4)) (ref r16))) (separator dot) (member (ref r17)))))))) (close-brace (span (offset 970) (line 33) (column 9) (len 1))))))))))))
)
~~~
