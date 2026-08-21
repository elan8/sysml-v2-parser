# META
~~~sexpr
(snapshot (type recovery) (description "An unrecognized member inside a ref declaration body becomes an explicit recovery node while the valid nested ref declaration after it still parses."))
~~~
# SOURCE
~~~sysml
interface def Broken {
    ref port :>> participant : Port {
        !!not a member;
        protected ref thisParticipant :>> self;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_usage_body_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_port_body_element") (severity error) (category parseerror) (span (offset 69) (line 3) (column 9) (len 24)) (message "unexpected token in port body"))
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
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 32) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 54) (line 2) (column 32) (len 4)))))
    (reference r1 (scope relative) (span (offset 40) (line 2) (column 18) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 40) (line 2) (column 18) (len 11)))))
    (reference r2 (scope relative) (span (offset 127) (line 4) (column 43) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 127) (line 4) (column 43) (len 4)))))
  )
  (root (interface-def (name "Broken") (modifiers) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "recovered_port_body_element") (found "!!not a member;") (span (offset 69) (line 3) (column 9) (len 24))) (ref (name "thisParticipant") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (subsets none) (body semicolon)))))))
)
~~~
