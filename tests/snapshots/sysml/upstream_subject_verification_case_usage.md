# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage form and verification context from training/34. Verification/Verification Case Usage Example.sysml:12: a verification subject with :> subsetting."))
~~~
# SOURCE
~~~sysml
package VerificationCaseUsageExample {
    part vehicleTestConfig : Vehicle;

    verification vehicleMassTest : VehicleMassTest {
        subject testVehicle :> vehicleTestConfig;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_verification_case_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VerificationCaseUsageExample {
    part vehicleTestConfig : Vehicle;
    verification vehicleMassTest : VehicleMassTest {
        subject testVehicle :> vehicleTestConfig;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 68) (line 2) (column 30) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 68) (line 2) (column 30) (len 7)))))
  )
  (root (package (name "VerificationCaseUsageExample") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleTestConfig") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (verification-case-usage))))
)
~~~
