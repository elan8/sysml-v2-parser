# META
~~~sexpr
(snapshot (type semantic) (description "Use-case-family bodies accept directed parameter shorthands (in/out with no kind keyword, spec42 Gap 45) and bare untyped actor declarations (spec42 Gap 46); variation requirement bodies accept the variant requirement kind keyword (spec42 Gap 44)."))
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
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 274) (line 9) (column 24) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 274) (line 9) (column 24) (len 6)))))
  )
  (root (package (name "UseCaseBodyMembers") (body (analysis-case-def) (use-case-def (name "U") (body (actor (name "environment") (type none)) (actor (name "passenger") (type none)) (actor (name "driver") (type (ref r0))) (analysis-case-usage))) (requirement-usage))))
)
~~~
