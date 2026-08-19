# META
~~~sexpr
(snapshot (type semantic) (description "Bare :>>/:> shorthand clauses accept comma-separated multi-target lists like every other redefinition clause (spec42 Gap 49b), and ref declarations no longer consume the redefines keyword as their declared name (spec42 Gap 49d)."))
~~~
# SOURCE
~~~sysml
package BareRedefinitionShorthand {
    attribute def kelvin {
        :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
    }
    item def I {
        private ref redefines Item::incomingTransferSort, subobjects::incomingTransferSort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bare_redefinition_shorthand.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package BareRedefinitionShorthand {
    attribute def kelvin {
        attribute :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
    }
    item def I {
        private ref :>> Item::incomingTransferSort, subobjects::incomingTransferSort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 75) (line 3) (column 13) (len 69)) (segments (segment 0 (token "ThermodynamicTemperatureUnit") (name "ThermodynamicTemperatureUnit") (separator none) (span (offset 75) (line 3) (column 13) (len 28))) (segment 1 (token "quantityDimension") (name "quantityDimension") (separator colon-colon) (span (offset 105) (line 3) (column 43) (len 17))) (segment 2 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator colon-colon) (span (offset 124) (line 3) (column 62) (len 20)))))
    (reference r1 (scope relative) (span (offset 146) (line 3) (column 84) (len 66)) (segments (segment 0 (token "TemperatureDifferenceUnit") (name "TemperatureDifferenceUnit") (separator none) (span (offset 146) (line 3) (column 84) (len 25))) (segment 1 (token "quantityDimension") (name "quantityDimension") (separator colon-colon) (span (offset 173) (line 3) (column 111) (len 17))) (segment 2 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator colon-colon) (span (offset 192) (line 3) (column 130) (len 20)))))
    (reference r2 (scope relative) (span (offset 267) (line 6) (column 31) (len 26)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 267) (line 6) (column 31) (len 4))) (segment 1 (token "incomingTransferSort") (name "incomingTransferSort") (separator colon-colon) (span (offset 273) (line 6) (column 37) (len 20)))))
    (reference r3 (scope relative) (span (offset 295) (line 6) (column 59) (len 32)) (segments (segment 0 (token "subobjects") (name "subobjects") (separator none) (span (offset 295) (line 6) (column 59) (len 10))) (segment 1 (token "incomingTransferSort") (name "incomingTransferSort") (separator colon-colon) (span (offset 307) (line 6) (column 71) (len 20)))))
  )
  (root (package (name "BareRedefinitionShorthand") (body brace (attribute-def (declaration-name "kelvin") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0) (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (item-def (name "I") (individual false) (specializes none) (body brace (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2) (ref r3)))) (subsets none) (body semicolon)))))))
)
~~~
