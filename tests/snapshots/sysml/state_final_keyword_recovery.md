# META
~~~sexpr
(snapshot (type recovery) (description "Neither pinned grammar has a `final` or `final state` production, and `StateBodyItem` has no such member. A state machine finishes by transitioning to the `done` action it inherits, so both spellings are errors, and later siblings still parse. `final` is not a reserved word, so a member can still be named it."))
~~~
# SOURCE
~~~sysml
state def Machine {
    final completed;
    final state stopped;
    state running;
    transition first running then done;
    state final;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_final_keyword_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 24) (line 2) (column 5) (len 21)) (message "unrecognized declaration `final` in state body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 45) (line 3) (column 5) (len 25)) (message "unrecognized declaration `final` in state body"))
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 106) (line 5) (column 22) (len 7)) (segments (segment 0 (token "running") (name "running") (separator none) (span (offset 106) (line 5) (column 22) (len 7)))))
    (reference r1 (scope relative) (span (offset 119) (line 5) (column 35) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 119) (line 5) (column 35) (len 4)))))
  )
  (root (state-def (name "Machine") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "final completed;") (span (offset 24) (line 2) (column 5) (len 21))) (malformed (code "unrecognized_declaration_in_scope") (found "final state stopped;") (span (offset 45) (line 3) (column 5) (len 25))) (state-usage (name "running") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (short-name none) (source (expression (span (offset 106) (line 5) (column 22) (len 7)) (ref r0))) (initial true) (accept none) (guard none) (effect none) (target (expression (span (offset 119) (line 5) (column 35) (len 4)) (ref r1))) (body semicolon)) (state-usage (name "final") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))
)
~~~
