# META
~~~sexpr
(snapshot (type semantic) (description "Exact nested VariantUsage context from Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml:1485-1499: part alternatives and attribute alternatives in the shared AttributeBody."))
~~~
# SOURCE
~~~sysml
package SimpleVehicleModel {
    package VehicleSuperSetModel {
        package VehiclePartsTree {
            abstract part vehicleFamily {
                variation part engine : Engine {
                    variant part engine4Cyl : Engine4Cyl;
                    variant part engine6Cyl : Engine6Cyl {
                        part cylinder : Cylinder [6] {
                            variation attribute diameter : LengthValue {
                                variant attribute smallDiameter : LengthValue;
                                variant attribute largeDiagmeter : LengthValue;
                            }
                        }
                    }
                }
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_variant_simple_vehicle_model.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SimpleVehicleModel {
    package VehicleSuperSetModel {
        package VehiclePartsTree {
            abstract part vehicleFamily {
                variation part engine : Engine {
                    variant part engine4Cyl : Engine4Cyl;
                    variant part engine6Cyl : Engine6Cyl {
                        part cylinder : Cylinder[6] {
                            variation attribute diameter : LengthValue {
                                variant attribute smallDiameter : LengthValue;
                                variant attribute largeDiagmeter : LengthValue;
                            }
                        }
                    }
                }
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 181) (line 5) (column 41) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 181) (line 5) (column 41) (len 6)))))
    (reference r1 (scope relative) (span (offset 236) (line 6) (column 47) (len 10)) (segments (segment 0 (token "Engine4Cyl") (name "Engine4Cyl") (separator none) (span (offset 236) (line 6) (column 47) (len 10)))))
    (reference r2 (scope relative) (span (offset 294) (line 7) (column 47) (len 10)) (segments (segment 0 (token "Engine6Cyl") (name "Engine6Cyl") (separator none) (span (offset 294) (line 7) (column 47) (len 10)))))
    (reference r3 (scope relative) (span (offset 347) (line 8) (column 41) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 347) (line 8) (column 41) (len 8)))))
    (reference r4 (scope relative) (span (offset 421) (line 9) (column 60) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 421) (line 9) (column 60) (len 11)))))
    (reference r5 (scope relative) (span (offset 501) (line 10) (column 67) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 501) (line 10) (column 67) (len 11)))))
    (reference r6 (scope relative) (span (offset 581) (line 11) (column 68) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 581) (line 11) (column 68) (len 11)))))
  )
  (root (package (name "SimpleVehicleModel") (body brace (package (name "VehicleSuperSetModel") (body brace (package (name "VehiclePartsTree") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleFamily") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine4Cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine6Cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cylinder") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 357) (line 8) (column 51) (len 1)) (integer 6))) (upper (expression (span (offset 357) (line 8) (column 51) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "diameter") (direction none) (derived false) (usage-prefix variation) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (variant-usage (target none) (usage (attribute-usage (declaration-name "smallDiameter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (attribute-usage (declaration-name "largeDiagmeter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent))))))))) (body absent)))))))))))))
)
~~~
