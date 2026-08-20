# META
~~~sexpr
(snapshot (type semantic) (description "A requirement body owns the general usage families it inherits from DefinitionBodyItem -- action, succession, perform, state, item, part and both ConnectionUsage spellings -- in source order alongside its requirement-specific members, and keeps a malformed member as an explicit recovery node without consuming the valid siblings around it."))
~~~
# SOURCE
~~~sysml
package RequirementBodyUsageMembers {
    requirement def MassLimit {
        subject vehicle : Vehicle;
        attribute massActual : MassValue;
        part fuelTank : FuelTank;
        item cargo : Cargo;
        action weigh : Weigh;
        perform action recordMass;
        perform vehicle.selfTest;
        state monitoring : MonitoringStates;
        succession weighed first weigh then recordMass;
        connect fuelTank to cargo;
        connection mounting connect fuelTank to vehicle;
        require constraint { massActual <= massReqd; }
    }

    requirement massLimit : MassLimit {
        part spare : FuelTank;
        action reweigh;
        connect spare to spare;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_body_usage_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementBodyUsageMembers {
    requirement def MassLimit {
        subject vehicle : Vehicle;
        attribute massActual : MassValue;
        part fuelTank : FuelTank;
        item cargo : Cargo;
        action weigh : Weigh;
        perform action recordMass;
        perform vehicle.selfTest;
        state monitoring : MonitoringStates;
        succession weighed first weigh then recordMass;
        connect fuelTank to cargo;
        connection mounting connect fuelTank to vehicle;
        require constraint {
            massActual <= massReqd;
        }
    }
    requirement massLimit : MassLimit {
        part spare : FuelTank;
        action reweigh;
        connect spare to spare;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 96) (line 3) (column 27) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 96) (line 3) (column 27) (len 7)))))
    (reference r1 (scope relative) (span (offset 171) (line 5) (column 25) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 171) (line 5) (column 25) (len 8)))))
    (reference r2 (scope relative) (span (offset 202) (line 6) (column 22) (len 5)) (segments (segment 0 (token "Cargo") (name "Cargo") (separator none) (span (offset 202) (line 6) (column 22) (len 5)))))
  )
  (root (package (name "RequirementBodyUsageMembers") (body brace (requirement-def (name "MassLimit") (modifiers) (body brace (subject (name "vehicle") (short-name none) (type (ref r0)) (redefines none) (value none) (body semicolon)) (attribute-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cargo") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage) (perform) (perform) (state-usage) (succession-usage) (connect) (connection) (require-constraint))) (requirement-usage (name "massLimit") (multiplicity none)))))
)
~~~
