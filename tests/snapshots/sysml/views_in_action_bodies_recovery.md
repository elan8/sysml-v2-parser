# META
~~~sexpr
(snapshot (type recovery) (description "Malformed views in action definition and usage bodies retain their exact source and valid later views; control-node ActionBody also retains typed views."))
~~~
# SOURCE
~~~sysml
package P {
    action def A {
        view bad : ;
        view good;
        merge m { view nested; }
    }
    action a {
        view bad : ;
        view good;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "views_in_action_bodies_recovery.md"
    (diagnostics
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 39) (line 3) (column 9) (len 21)) (message "expected ';' or '{' after view declaration header"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 39) (line 3) (column 9) (len 21)) (message "suppressed 1 cascading missing_body_or_semicolon diagnostic after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        view bad : ;
        view good;
        merge m {
            view nested;
        }
    }
    action a {
        view bad : ;
        view good;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 85) (line 5) (column 15) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 85) (line 5) (column 15) (len 1)))))
  )
  (root (package (visibility none) (name "P") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (malformed (code "missing_body_or_semicolon") (found "view bad : ;") (span (offset 39) (line 3) (column 9) (len 21))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "good") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 85) (line 5) (column 15) (len 1)) (ref r0)))) (body brace (open-brace (span (offset 87) (line 5) (column 17) (len 1))) (members (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "nested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon))) (close-brace (span (offset 102) (line 5) (column 32) (len 1))))))) (action-usage (keyword action) (name "a") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (malformed (code "missing_body_or_semicolon") (found "view bad : ;") (span (offset 133) (line 8) (column 9) (len 21))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "good") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))
)
~~~
