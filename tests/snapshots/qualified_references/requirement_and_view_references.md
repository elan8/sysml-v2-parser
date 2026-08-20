# META
~~~sexpr
(snapshot (type semantic) (description "GH-119: requirement, dependency and view members hold document-local reference ids rather than copied strings, so every qualified name they mention appears once in the reference arena with its own segments, separators and span. The `$::` root prefix is recorded as an absolute scope rather than as part of the first segment's text."))
~~~
# SOURCE
~~~sysml
package P {
    dependency from Client::A to $::Supplier::B;
    requirement def R {
        subject s : $::Domain::Subject;
        actor a : Domain::Actor;
        verify Requirements::Req :>> Base::Req;
    }
    view v : $::Views::General {
        satisfy Viewpoints::VP;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_and_view_references.md"
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
    (reference r0 (scope relative) (span (offset 32) (line 2) (column 21) (len 9)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 32) (line 2) (column 21) (len 6))) (segment 1 (token "A") (name "A") (separator colon-colon) (span (offset 40) (line 2) (column 29) (len 1)))))
    (reference r1 (scope absolute) (span (offset 45) (line 2) (column 34) (len 14)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 48) (line 2) (column 37) (len 8))) (segment 1 (token "B") (name "B") (separator colon-colon) (span (offset 58) (line 2) (column 47) (len 1)))))
    (reference r2 (scope absolute) (span (offset 105) (line 4) (column 21) (len 18)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 108) (line 4) (column 24) (len 6))) (segment 1 (token "Subject") (name "Subject") (separator colon-colon) (span (offset 116) (line 4) (column 32) (len 7)))))
    (reference r3 (scope relative) (span (offset 143) (line 5) (column 19) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 143) (line 5) (column 19) (len 6))) (segment 1 (token "Actor") (name "Actor") (separator colon-colon) (span (offset 151) (line 5) (column 27) (len 5)))))
    (reference r4 (scope relative) (span (offset 173) (line 6) (column 16) (len 17)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 173) (line 6) (column 16) (len 12))) (segment 1 (token "Req") (name "Req") (separator colon-colon) (span (offset 187) (line 6) (column 30) (len 3)))))
    (reference r5 (scope relative) (span (offset 195) (line 6) (column 38) (len 9)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 195) (line 6) (column 38) (len 4))) (segment 1 (token "Req") (name "Req") (separator colon-colon) (span (offset 201) (line 6) (column 44) (len 3)))))
    (reference r6 (scope absolute) (span (offset 225) (line 8) (column 14) (len 17)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 228) (line 8) (column 17) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 235) (line 8) (column 24) (len 7)))))
    (reference r7 (scope relative) (span (offset 261) (line 9) (column 17) (len 14)) (segments (segment 0 (token "Viewpoints") (name "Viewpoints") (separator none) (span (offset 261) (line 9) (column 17) (len 10))) (segment 1 (token "VP") (name "VP") (separator colon-colon) (span (offset 273) (line 9) (column 29) (len 2)))))
  )
  (root (package (name "P") (body brace (dependency (clients (ref r0)) (suppliers (ref r1)) (body semicolon)) (requirement-def (name "R") (modifiers) (body brace (subject (name "s") (short-name none) (type (ref r2)) (redefines none) (value none) (body semicolon)) (actor (name "a") (short-name none) (type (ref r3)) (multiplicity none)) (verify (target (ref r4)) (redefines (ref r5))))) (view (name "v") (short-name none) (type (ref r6)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r7))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))))))
)
~~~
