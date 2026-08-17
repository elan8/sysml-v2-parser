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
  )
  (root (package (name "BareRedefinitionShorthand") (body brace (attribute-def (name "kelvin") (multiplicity none)) (item-def))))
)
~~~
