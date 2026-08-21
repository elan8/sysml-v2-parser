# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/USCustomaryUnits.sysml narrow retained context for the brace-bodied typed `unitConversion` redefinition."))
~~~
# SOURCE
~~~sysml
standard library package <USCU> USCustomaryUnits {
    attribute 'acre (based on US survey foot)' : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m^2; :>> conversionFactor = 4.046873E+03; :>> isExact = false; } }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "us_customary_units_unit_conversion_header.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package <USCU> USCustomaryUnits {
    attribute 'acre (based on US survey foot)' : AreaUnit {
         : ConversionByConvention :>> unitConversion {
             :>> referenceUnit = m ^ 2;
             :>> conversionFactor = 4.046873E+03;
             :>> isExact = false;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (library-package (name "USCustomaryUnits") (standard true) (body brace (attribute-usage))))
)
~~~
