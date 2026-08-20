# META
~~~sexpr
(snapshot (type semantic) (description "Verifies actor redefinition values and reference redefinition bodies retain structured expressions, nested facts, source-backed targets, and aggregate spans."))
~~~
# SOURCE
~~~sysml
package RedefinitionFacts {
    use case def Transport {
        actor :>> driver = 'provide transportation'::driver;
        ref :>> start {
            actor :>> fueler = Crew.driver;
            ref :>> nested {
                first Actions::prepare;
            }
        }
        ref :>> done;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "actor_ref_redefinitions.md"
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
    (reference r0 (scope relative) (span (offset 75) (line 3) (column 19) (len 6)) (segments (segment 0 (token "driver") (name "driver") (separator none) (span (offset 75) (line 3) (column 19) (len 6)))))
    (reference r1 (scope relative) (span (offset 84) (line 3) (column 28) (len 32)) (segments (segment 0 (token "'provide transportation'") (name "provide transportation") (separator none) (span (offset 84) (line 3) (column 28) (len 24))) (segment 1 (token "driver") (name "driver") (separator colon-colon) (span (offset 110) (line 3) (column 54) (len 6)))))
    (reference r2 (scope relative) (span (offset 134) (line 4) (column 17) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 134) (line 4) (column 17) (len 5)))))
    (reference r3 (scope relative) (span (offset 164) (line 5) (column 23) (len 6)) (segments (segment 0 (token "fueler") (name "fueler") (separator none) (span (offset 164) (line 5) (column 23) (len 6)))))
    (reference r4 (scope relative) (span (offset 173) (line 5) (column 32) (len 4)) (segments (segment 0 (token "Crew") (name "Crew") (separator none) (span (offset 173) (line 5) (column 32) (len 4)))))
    (reference r5 (scope relative) (span (offset 178) (line 5) (column 37) (len 6)) (segments (segment 0 (token "driver") (name "driver") (separator none) (span (offset 178) (line 5) (column 37) (len 6)))))
    (reference r6 (scope relative) (span (offset 206) (line 6) (column 21) (len 6)) (segments (segment 0 (token "nested") (name "nested") (separator none) (span (offset 206) (line 6) (column 21) (len 6)))))
    (reference r7 (scope relative) (span (offset 237) (line 7) (column 23) (len 16)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 237) (line 7) (column 23) (len 7))) (segment 1 (token "prepare") (name "prepare") (separator colon-colon) (span (offset 246) (line 7) (column 32) (len 7)))))
    (reference r8 (scope relative) (span (offset 295) (line 10) (column 17) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 295) (line 10) (column 17) (len 4)))))
  )
  (root (package (name "RedefinitionFacts") (body brace (use-case-def (name "Transport") (modifiers) (body brace (actor-redefinition (target (ref r0)) (value (expression (span (offset 84) (line 3) (column 28) (len 32)) (ref r1)))) (ref-redefinition (target (ref r2)) (body-span (span (offset 140) (line 4) (column 23) (len 138))) (body brace (actor-redefinition (target (ref r3)) (value (expression (span (offset 173) (line 5) (column 32) (len 11)) (member-access (base (expression (span (offset 173) (line 5) (column 32) (len 4)) (ref r4))) (separator dot) (member (ref r5)))))) (ref-redefinition (target (ref r6)) (body-span (span (offset 213) (line 6) (column 28) (len 55))) (body brace (first (target (ref r7))))))) (ref-redefinition (target (ref r8)) (body-span (span (offset 299) (line 10) (column 21) (len 1))) (body semicolon)))))))
)
~~~
