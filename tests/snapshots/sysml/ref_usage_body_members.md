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
    ref port :>> participant : Port[2..*] nonunique ordered {
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
  (root (interface-def (name "Interface") (modifiers) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 63) (line 2) (column 38) (len 1)) (integer 2))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 103) (line 3) (column 15) (len 49)) (normalized "The participants of an Interface must be Ports. "))) (ref (name "thisParticipant") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (subsets none) (body semicolon)) (ref (name "otherParticipants") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (body semicolon)))))))
)
~~~
