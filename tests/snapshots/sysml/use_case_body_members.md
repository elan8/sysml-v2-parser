# META
~~~sexpr
(snapshot (type semantic) (description "Use-case-family bodies accept directed parameter shorthands (in/out with no kind keyword, spec42 Gap 45) and bare untyped actor declarations (spec42 Gap 46); variation requirement bodies accept the variant requirement kind keyword (spec42 Gap 44). A return declaration keeps a :>> clause written after its type, which is a different position from the leading anonymous return :>> target form."))
~~~
# SOURCE
~~~sysml
package UseCaseBodyMembers {
    analysis def A {
        out voltage :> electricPotential = vehicle.battery.voltage;
        out current = vehicle.battery.current;
    }
    use case def U {
        actor environment;
        actor passenger [0..4];
        actor driver : Person;
        analysis fuelEconomyAnalysis : A {
            in scenario = cityScenario;
        }
    }
    variation requirement r {
        variant requirement r1;
    }
    verification def VerificationCase {
        return verdict : VerdictKind :>> result;
        return :>> otherResult;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "use_case_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package UseCaseBodyMembers {
    analysis def A {
        out voltage :> electricPotential = vehicle.battery.voltage;
        out current = vehicle.battery.current;
    }
    use case def U {
        actor environment;
        actor passenger[0..4];
        actor driver : Person;
        analysis fuelEconomyAnalysis : A {
            in scenario = cityScenario;
        }
    }
    variation requirement r {
        variant requirement r1;
    }
    verification def VerificationCase {
        return verdict : VerdictKind :>> result;
        return :>> otherResult;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 274) (line 9) (column 24) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 274) (line 9) (column 24) (len 6)))))
    (reference r1 (scope relative) (span (offset 514) (line 18) (column 26) (len 11)) (segments (segment 0 (token "VerdictKind") (name "VerdictKind") (separator none) (span (offset 514) (line 18) (column 26) (len 11)))))
    (reference r2 (scope relative) (span (offset 530) (line 18) (column 42) (len 6)) (segments (segment 0 (token "result") (name "result") (separator none) (span (offset 530) (line 18) (column 42) (len 6)))))
    (reference r3 (scope relative) (span (offset 557) (line 19) (column 20) (len 11)) (segments (segment 0 (token "otherResult") (name "otherResult") (separator none) (span (offset 557) (line 19) (column 20) (len 11)))))
  )
  (root (package (name "UseCaseBodyMembers") (body (analysis-case-def) (use-case-def (name "U") (body (actor (name "environment") (type none)) (actor (name "passenger") (type none)) (actor (name "driver") (type (ref r0))) (analysis-case-usage))) (requirement-usage) (verification-case-def (name "VerificationCase") (body (case-return (declaration "verdict") (target none) (type (ref r1)) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (feature-kind none) (subsetting false) (value none)) (case-return (declaration "") (target (ref r3)) (type none) (redefines none) (feature-kind none) (subsetting false) (value none)))))))
)
~~~
