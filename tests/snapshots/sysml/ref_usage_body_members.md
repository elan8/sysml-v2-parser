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
    ref port : Port[2..*] ordered nonunique :>> participant {
        doc
        /* The participants of an Interface must be Ports. */
        protected ref thisParticipant :>> self;
        protected ref otherParticipants : Port[1..*] nonunique :> interfacingPorts default participant->excludingOnce(thisParticipant);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 32) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 57) (line 2) (column 32) (len 4)))))
    (reference r1 (scope relative) (span (offset 43) (line 2) (column 18) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 43) (line 2) (column 18) (len 11)))))
    (reference r2 (scope relative) (span (offset 197) (line 4) (column 43) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 197) (line 4) (column 43) (len 4)))))
    (reference r3 (scope relative) (span (offset 245) (line 5) (column 43) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 245) (line 5) (column 43) (len 4)))))
    (reference r4 (scope relative) (span (offset 270) (line 5) (column 68) (len 16)) (segments (segment 0 (token "interfacingPorts") (name "interfacingPorts") (separator none) (span (offset 270) (line 5) (column 68) (len 16)))))
  )
  (root (interface-def (name "Interface") (specializes none) (body (ref (name "") (kind port) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body (doc) (ref (name "thisParticipant") (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (subsets none) (body semicolon)) (ref (name "otherParticipants") (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (body semicolon)))))))
)
~~~
