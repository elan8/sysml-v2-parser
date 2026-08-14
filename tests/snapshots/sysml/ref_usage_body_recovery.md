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
  )
  (root (interface-def (name "Broken") (specializes none) (body (ref))))
)
~~~
