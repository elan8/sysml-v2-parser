# META
~~~sexpr
(snapshot (type semantic) (description "Pinned SysML DefaultReferenceUsage permits an empty declaration name followed by a leading `:>>` redefinition, typing, multiplicity modifiers, and an AttributeBody. The typed header remains source-backed for both a brace-bodied unit conversion and a semicolon-bodied array element redefinition."))
~~~
# SOURCE
~~~sysml
package DefaultReferenceUsageLeadingRedefinitionHeader {
    attribute def Outer :> Unit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = metre;
            :>> conversionFactor = 1;
        }
        :>> elements : Real[3] ordered nonunique;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage_leading_redefinition_header.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DefaultReferenceUsageLeadingRedefinitionHeader {
    attribute def Outer :> Unit {
         : ConversionByConvention :>> unitConversion {
             :>> referenceUnit = metre;
             :>> conversionFactor = 1;
        }
         : Real[3] ordered nonunique :>> elements;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 84) (line 2) (column 28) (len 4)) (segments (segment 0 (token "Unit") (name "Unit") (separator none) (span (offset 84) (line 2) (column 28) (len 4)))))
    (reference r1 (scope relative) (span (offset 120) (line 3) (column 30) (len 22)) (segments (segment 0 (token "ConversionByConvention") (name "ConversionByConvention") (separator none) (span (offset 120) (line 3) (column 30) (len 22)))))
    (reference r2 (scope relative) (span (offset 103) (line 3) (column 13) (len 14)) (segments (segment 0 (token "unitConversion") (name "unitConversion") (separator none) (span (offset 103) (line 3) (column 13) (len 14)))))
    (reference r3 (scope relative) (span (offset 161) (line 4) (column 17) (len 13)) (segments (segment 0 (token "referenceUnit") (name "referenceUnit") (separator none) (span (offset 161) (line 4) (column 17) (len 13)))))
    (reference r4 (scope relative) (span (offset 177) (line 4) (column 33) (len 5)) (segments (segment 0 (token "metre") (name "metre") (separator none) (span (offset 177) (line 4) (column 33) (len 5)))))
    (reference r5 (scope relative) (span (offset 200) (line 5) (column 17) (len 16)) (segments (segment 0 (token "conversionFactor") (name "conversionFactor") (separator none) (span (offset 200) (line 5) (column 17) (len 16)))))
    (reference r6 (scope relative) (span (offset 255) (line 7) (column 24) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 255) (line 7) (column 24) (len 4)))))
    (reference r7 (scope relative) (span (offset 244) (line 7) (column 13) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 244) (line 7) (column 13) (len 8)))))
  )
  (root (package (name "DefaultReferenceUsageLeadingRedefinitionHeader") (body brace (attribute-def (declaration-name "Outer") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 177) (line 4) (column 33) (len 5)) (ref r4))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 219) (line 5) (column 36) (len 1)) (integer 1))))) (body semicolon)))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 260) (line 7) (column 29) (len 1)) (integer 3))) (upper (expression (span (offset 260) (line 7) (column 29) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
