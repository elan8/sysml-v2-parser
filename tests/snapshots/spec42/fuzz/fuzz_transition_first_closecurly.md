# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: transition with 'first' ending at CloseCurly preserves name"))
~~~
# SOURCE
~~~sysml
package P {
state def S {
    entry; then off;
    state off;
    transition t first }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_transition_first_closecurly.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 66) (line 5) (column 5) (len 19)) (message "missing semicolon before next declaration"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    state def S {
        entry;
        then off;
        state off;
        transition t first
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 42) (line 3) (column 17) (len 3)) (segments (segment 0 (token "off") (name "off") (separator none) (span (offset 42) (line 3) (column 17) (len 3)))))
  )
  (root (package (name "P") (body brace (state-def (name "S") (modifiers) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage (name "off") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (malformed (code "missing_semicolon") (found "transition t first") (span (offset 66) (line 5) (column 5) (len 19))))))))
)
~~~
