# META
~~~sexpr
(snapshot (type semantic) (description "Documentation comments in definition and usage bodies"))
~~~
# SOURCE
~~~sysml
package DocTests {
    doc /* Package-level documentation. */

    part def Vehicle {
        doc /* Part def documentation. */
        attribute speed;
    }

    attribute def Speed {
        doc DocName /* Named documentation. */
    }

    enum def Color {
        doc /* Enum def documentation. */
        enum red;
    }

    part vehicle : Vehicle {
        doc /* Usage-level documentation. */
    }

    item def Payload {
        doc <shortName> PayloadDoc locale "en" /* Full form doc with short name and locale. */
    }

    alias Car for Vehicle {
        doc /* Alias documentation. */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "documentation_in_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DocTests {
    doc
    /* Package-level documentation. */
    part def Vehicle {
        doc
        /* Part def documentation. */
        attribute speed;
    }
    attribute def Speed {
        doc DocName
        /* Named documentation. */
    }
    enum def Color {
        doc
        /* Enum def documentation. */
        red;
    }
    part vehicle : Vehicle {
        doc
        /* Usage-level documentation. */
    }
    item def Payload {
        doc <shortName> PayloadDoc locale "en"
        /* Full form doc with short name and locale. */
    }
    alias Car for Vehicle {
        doc
        /* Alias documentation. */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 347) (line 18) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 347) (line 18) (column 20) (len 7)))))
    (reference r1 (scope relative) (span (offset 552) (line 26) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 552) (line 26) (column 19) (len 7)))))
  )
  (root (package (name "DocTests") (body brace (doc) (part-def (name "Vehicle") (body brace (doc) (attribute-usage (declaration-name "speed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (name "Speed") (multiplicity none)) (enum-def (name "Color") (body brace (doc) (enum-value (name "red") (short-name none) (value none) (body semicolon) (span (offset 311) (line 15) (column 9) (len 9))))) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (doc))) (item-def) (alias (name "Car") (target (ref r1)) (body brace (element-count 1))))))
)
~~~
