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
    (reference r3 (scope relative) (span (offset 232) (line 7) (column 24) (len 5)) (segments (segment 0 (token "Weigh") (name "Weigh") (separator none) (span (offset 232) (line 7) (column 24) (len 5)))))
    (reference r4 (scope relative) (span (offset 335) (line 10) (column 28) (len 16)) (segments (segment 0 (token "MonitoringStates") (name "MonitoringStates") (separator none) (span (offset 335) (line 10) (column 28) (len 16)))))
    (reference r5 (scope relative) (span (offset 530) (line 14) (column 30) (len 10)) (segments (segment 0 (token "massActual") (name "massActual") (separator none) (span (offset 530) (line 14) (column 30) (len 10)))))
    (reference r6 (scope relative) (span (offset 544) (line 14) (column 44) (len 8)) (segments (segment 0 (token "massReqd") (name "massReqd") (separator none) (span (offset 544) (line 14) (column 44) (len 8)))))
  )
  (root (package (name "RequirementBodyUsageMembers") (body brace (requirement-def (name "MassLimit") (modifiers) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "cargo") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage (keyword action) (name "weigh") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (perform) (perform) (state-usage (name "monitoring") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (succession-usage) (connect) (connection) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (typing none) (body brace (expression (span (offset 530) (line 14) (column 30) (len 22)) (binary (operator "<=") (left (expression (span (offset 530) (line 14) (column 30) (len 10)) (ref r5))) (right (expression (span (offset 544) (line 14) (column 44) (len 8)) (ref r6))))))))) (requirement-usage (name "massLimit") (multiplicity none)))))
)
~~~
