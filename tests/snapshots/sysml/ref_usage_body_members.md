# META
~~~sexpr
(snapshot (type semantic) (description "A connection/interface ref declaration body accepts nested visibility-prefixed ref declarations with multiplicity properties, subsets, and default values, alongside doc annotations."))
~~~
# SOURCE
~~~sysml
interface def Interface {
    ref port :>> participant : Port [2..*] nonunique ordered {
        doc /* The participants of an Interface must be Ports. */
        protected ref thisParticipant :>> self;
        protected ref otherParticipants : Port [1..*] nonunique :> interfacingPorts
            default participant->excludingOnce(thisParticipant);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_usage_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
interface def Interface {
    ref port '' : Port[2..*] ordered nonunique :>> participant {
        doc
        /* The participants of an Interface must be Ports. */
        protected ref thisParticipant :>> self;
        protected ref otherParticipants : Port[1..*] nonunique :> interfacingPorts default = participant->excludingOnce(thisParticipant);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (interface-def (name "Interface") (specializes none) (body (ref))))
)
~~~
