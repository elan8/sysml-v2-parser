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
        enum red;
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
  (root (package (name "DocTests") (body brace (doc (name none) (locale none) (body (span (offset 29) (line 2) (column 11) (len 30)) (normalized "Package-level documentation. "))) (part-def (name "Vehicle") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 100) (line 5) (column 15) (len 25)) (normalized "Part def documentation. "))) (attribute-usage (declaration-name "speed") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Speed") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name "DocName") (locale none) (body (span (offset 208) (line 10) (column 23) (len 22)) (normalized "Named documentation. "))))) (enum-def (name "Color") (body brace (doc (name none) (locale none) (body (span (offset 275) (line 14) (column 15) (len 25)) (normalized "Enum def documentation. "))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "red") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 311) (line 15) (column 9) (len 9))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 371) (line 19) (column 15) (len 28)) (normalized "Usage-level documentation. "))))) (item-def (name "Payload") (modifiers) (individual false) (specializes none) (body brace (doc (name "PayloadDoc") (locale "en") (body (span (offset 481) (line 23) (column 50) (len 43)) (normalized "Full form doc with short name and locale. "))))) (alias (name "Car") (target (ref r1)) (body brace (element-count 1))))))
)
~~~
