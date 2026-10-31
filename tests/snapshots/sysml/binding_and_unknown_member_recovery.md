# META
~~~sexpr
(snapshot (type recovery) (description "SysML 8.2.2.13.2 requires bind in BindingConnectorAsUsage: an invalid binding header and unknown state/part members recover with errors, while a valid named binding and later typed siblings survive."))
~~~
# SOURCE
~~~sysml
package Recovery {
    attribute def A {
        binding b = c;
        binding retained bind b = c;
        attribute later;
    }
    state def S {
        unknown stuff;
        transition t then Ready;
    }
    part def Q {
        ??? junk ???
        part later;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "binding_and_unknown_member_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_attribute_body_element") (severity error) (category parseerror) (span (offset 49) (line 3) (column 9) (len 23)) (message "unexpected token in attribute body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 158) (line 8) (column 9) (len 23)) (message "unrecognized declaration `unknown` in state body"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 237) (line 12) (column 9) (len 21)) (message "unexpected token in part definition body"))
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
    (reference r0 (scope relative) (span (offset 199) (line 9) (column 27) (len 5)) (segments (segment 0 (token "Ready") (name "Ready") (separator none) (span (offset 199) (line 9) (column 27) (len 5)))))
  )
  (root (package (visibility none) (name "Recovery") (body brace (attribute-def (declaration-name "A") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "recovered_attribute_body_element") (found "binding b = c;") (span (offset 49) (line 3) (column 9) (len 23))) (bind) (attribute-usage (declaration-name "later") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (state-def (name "S") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "unknown stuff;") (span (offset 158) (line 8) (column 9) (len 23))) (transition (name "t") (short-name none) (source none) (initial false) (accept none) (guard none) (effect none) (target (expression (span (offset 199) (line 9) (column 27) (len 5)) (ref r0))) (body semicolon)))) (part-def (name "Q") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "??? junk ???") (span (offset 237) (line 12) (column 9) (len 21))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
