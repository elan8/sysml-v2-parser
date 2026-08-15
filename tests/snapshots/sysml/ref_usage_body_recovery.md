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
      (diagnostic (code "recovered_ref_body_element") (severity error) (category parseerror) (span (offset 69) (line 3) (column 9) (len 24)) (message "unexpected token in ref usage body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
interface def Broken {
    ref port : Port :>> participant {
        !!not a member;
        protected ref thisParticipant :>> self;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 32) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 54) (line 2) (column 32) (len 4)))))
    (reference r1 (scope relative) (span (offset 40) (line 2) (column 18) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 40) (line 2) (column 18) (len 11)))))
    (reference r2 (scope relative) (span (offset 127) (line 4) (column 43) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 127) (line 4) (column 43) (len 4)))))
  )
  (root (interface-def (name "Broken") (specializes none) (body (ref (name "") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind port) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body (malformed (code "recovered_ref_body_element") (found "!!not a member;") (span (offset 69) (line 3) (column 9) (len 24))) (ref (name "thisParticipant") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (subsets none) (body semicolon)))))))
)
~~~
