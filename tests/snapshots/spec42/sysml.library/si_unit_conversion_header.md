# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/SI.sysml narrow retained context for the brace-bodied typed `unitConversion` redefinition."))
~~~
# SOURCE
~~~sysml
standard library package SI {
    attribute <kg> kilogram : MassUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = g; } }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "si_unit_conversion_header.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package SI {
    attribute <kg> kilogram : MassUnit {
         : ConversionByPrefix :>> unitConversion {
             :>> prefix = kilo;
             :>> referenceUnit = g;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (library-package (name "SI") (standard true) (body brace (attribute-usage))))
)
~~~
