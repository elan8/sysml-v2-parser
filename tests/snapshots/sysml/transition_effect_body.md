# META
~~~sexpr
(snapshot (type semantic) (description "Transition EffectBehaviorUsage retains an anonymous do action brace body through the pinned SysML BNF 1314-1334 and Pilot EffectBehaviorUsage grammar 1909-1919. The effect body is a typed ActionBody, not an absorbed suffix."))
~~~
# SOURCE
~~~sysml
package TransitionEffectBodies {
    state def S {
        transition accept signal do action {
            in value : String;
            action work;
        } then Anonymous;
        state Anonymous;
        transition accept signal do action named : Action {
            action namedWork;
        } then Performed;
        state Performed;
        transition accept signal do accept reply via port {
            action acceptedWork;
        } then Accepted;
        state Accepted;
        transition accept signal do send new Reply() via port to destination {
            action sentWork;
        } then Sent;
        state Sent;
        transition accept signal do assign value := next {
            action assignedWork;
        } then Done;
        state Done;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "transition_effect_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 77) (line 3) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 77) (line 3) (column 27) (len 6)))))
    (reference r1 (scope relative) (span (offset 119) (line 4) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 119) (line 4) (column 24) (len 6)))))
    (reference r2 (scope relative) (span (offset 167) (line 6) (column 16) (len 9)) (segments (segment 0 (token "Anonymous") (name "Anonymous") (separator none) (span (offset 167) (line 6) (column 16) (len 9)))))
    (reference r3 (scope relative) (span (offset 229) (line 8) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 229) (line 8) (column 27) (len 6)))))
    (reference r4 (scope relative) (span (offset 254) (line 8) (column 52) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 254) (line 8) (column 52) (len 6)))))
    (reference r5 (scope relative) (span (offset 308) (line 10) (column 16) (len 9)) (segments (segment 0 (token "Performed") (name "Performed") (separator none) (span (offset 308) (line 10) (column 16) (len 9)))))
    (reference r6 (scope relative) (span (offset 370) (line 12) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 370) (line 12) (column 27) (len 6)))))
    (reference r7 (scope relative) (span (offset 387) (line 12) (column 44) (len 5)) (segments (segment 0 (token "reply") (name "reply") (separator none) (span (offset 387) (line 12) (column 44) (len 5)))))
    (reference r8 (scope relative) (span (offset 397) (line 12) (column 54) (len 4)) (segments (segment 0 (token "port") (name "port") (separator none) (span (offset 397) (line 12) (column 54) (len 4)))))
    (reference r9 (scope relative) (span (offset 452) (line 14) (column 16) (len 8)) (segments (segment 0 (token "Accepted") (name "Accepted") (separator none) (span (offset 452) (line 14) (column 16) (len 8)))))
    (reference r10 (scope relative) (span (offset 512) (line 16) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 512) (line 16) (column 27) (len 6)))))
    (reference r11 (scope relative) (span (offset 531) (line 16) (column 46) (len 5)) (segments (segment 0 (token "Reply") (name "Reply") (separator none) (span (offset 531) (line 16) (column 46) (len 5)))))
    (reference r12 (scope relative) (span (offset 543) (line 16) (column 58) (len 4)) (segments (segment 0 (token "port") (name "port") (separator none) (span (offset 543) (line 16) (column 58) (len 4)))))
    (reference r13 (scope relative) (span (offset 551) (line 16) (column 66) (len 11)) (segments (segment 0 (token "destination") (name "destination") (separator none) (span (offset 551) (line 16) (column 66) (len 11)))))
    (reference r14 (scope relative) (span (offset 609) (line 18) (column 16) (len 4)) (segments (segment 0 (token "Sent") (name "Sent") (separator none) (span (offset 609) (line 18) (column 16) (len 4)))))
    (reference r15 (scope relative) (span (offset 661) (line 20) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 661) (line 20) (column 27) (len 6)))))
    (reference r16 (scope relative) (span (offset 678) (line 20) (column 44) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 678) (line 20) (column 44) (len 5)))))
    (reference r17 (scope relative) (span (offset 687) (line 20) (column 53) (len 4)) (segments (segment 0 (token "next") (name "next") (separator none) (span (offset 687) (line 20) (column 53) (len 4)))))
    (reference r18 (scope relative) (span (offset 742) (line 22) (column 16) (len 4)) (segments (segment 0 (token "Done") (name "Done") (separator none) (span (offset 742) (line 22) (column 16) (len 4)))))
  )
  (root (package (name "TransitionEffectBodies") (body brace (state-def (name "S") (modifiers) (body brace (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 77) (line 3) (column 27) (len 6)) (ref r0)) (via none))) (guard none) (effect (perform (name none) (type none) (body (body brace (in-out (direction in) (reference false) (declaration "value") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 108) (line 4) (column 13) (len 18))) (action-usage (name "work") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 167) (line 6) (column 16) (len 9)) (ref r2))) (body semicolon)) (state-usage (name "Anonymous") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 229) (line 8) (column 27) (len 6)) (ref r3)) (via none))) (guard none) (effect (perform (name "named") (type (ref r4)) (body (body brace (action-usage (name "namedWork") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 308) (line 10) (column 16) (len 9)) (ref r5))) (body semicolon)) (state-usage (name "Performed") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 370) (line 12) (column 27) (len 6)) (ref r6)) (via none))) (guard none) (effect (accept (payload (expression (span (offset 387) (line 12) (column 44) (len 5)) (ref r7))) (type none) (via (expression (span (offset 397) (line 12) (column 54) (len 4)) (ref r8))) (body (body brace (action-usage (name "acceptedWork") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 452) (line 14) (column 16) (len 8)) (ref r9))) (body semicolon)) (state-usage (name "Accepted") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 512) (line 16) (column 27) (len 6)) (ref r10)) (via none))) (guard none) (effect (send (payload (expression (span (offset 527) (line 16) (column 42) (len 11)) (constructor (type (ref r11)) (arguments)))) (type none) (via (expression (span (offset 543) (line 16) (column 58) (len 4)) (ref r12))) (to (expression (span (offset 551) (line 16) (column 66) (len 11)) (ref r13))) (body (body brace (action-usage (name "sentWork") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 609) (line 18) (column 16) (len 4)) (ref r14))) (body semicolon)) (state-usage (name "Sent") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 661) (line 20) (column 27) (len 6)) (ref r15)) (via none))) (guard none) (effect (assign (lhs (expression (span (offset 678) (line 20) (column 44) (len 5)) (ref r16))) (rhs (expression (span (offset 687) (line 20) (column 53) (len 4)) (ref r17))) (body (body brace (action-usage (name "assignedWork") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 742) (line 22) (column 16) (len 4)) (ref r18))) (body semicolon)) (state-usage (name "Done") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
