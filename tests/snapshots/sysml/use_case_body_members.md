# META
~~~sexpr
(snapshot (type semantic) (description "Use-case-family bodies accept directed parameter shorthands (in/out with no kind keyword, spec42 Gap 45) and bare untyped actor declarations (spec42 Gap 46); variation requirement bodies accept the variant requirement kind keyword (spec42 Gap 44). A return declaration keeps a :>> clause written after its type, which is a different position from the leading anonymous return :>> target form. Nested case usages (use case / case / verification) are members of these bodies and keep the multiplicity and every subsets target their declaration tail used to discard."))
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
        abstract verification subVerificationCases : VerificationCase[0..*] :> verificationCases, subcases;
    }
    use case def UseCase {
        abstract use case subUseCases : UseCase[0..*] :> useCases, subcases;
    }
    case def Case {
        abstract case subcases : Case[0..*] :> cases, subcalculations;
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
        abstract verification subVerificationCases : VerificationCase[0..*] :> verificationCases, subcases;
    }
    use case def UseCase {
        abstract use case subUseCases : UseCase[0..*] :> useCases, subcases;
    }
    case def Case {
        abstract case subcases : Case[0..*] :> cases, subcalculations;
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
    (reference r4 (scope relative) (span (offset 623) (line 20) (column 54) (len 16)) (segments (segment 0 (token "VerificationCase") (name "VerificationCase") (separator none) (span (offset 623) (line 20) (column 54) (len 16)))))
    (reference r5 (scope relative) (span (offset 649) (line 20) (column 80) (len 17)) (segments (segment 0 (token "verificationCases") (name "verificationCases") (separator none) (span (offset 649) (line 20) (column 80) (len 17)))))
    (reference r6 (scope relative) (span (offset 668) (line 20) (column 99) (len 8)) (segments (segment 0 (token "subcases") (name "subcases") (separator none) (span (offset 668) (line 20) (column 99) (len 8)))))
    (reference r7 (scope relative) (span (offset 751) (line 23) (column 41) (len 7)) (segments (segment 0 (token "UseCase") (name "UseCase") (separator none) (span (offset 751) (line 23) (column 41) (len 7)))))
    (reference r8 (scope relative) (span (offset 768) (line 23) (column 58) (len 8)) (segments (segment 0 (token "useCases") (name "useCases") (separator none) (span (offset 768) (line 23) (column 58) (len 8)))))
    (reference r9 (scope relative) (span (offset 778) (line 23) (column 68) (len 8)) (segments (segment 0 (token "subcases") (name "subcases") (separator none) (span (offset 778) (line 23) (column 68) (len 8)))))
  )
  (root (package (name "UseCaseBodyMembers") (body brace (analysis-case-def) (use-case-def (name "U") (body brace (actor (name "environment") (type none)) (actor (name "passenger") (type none)) (actor (name "driver") (type (ref r0))) (analysis-case-usage))) (requirement-usage (name "r") (multiplicity none)) (verification-case-def (name "VerificationCase") (body brace (case-return (declaration "verdict") (target none) (type (ref r1)) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (feature-kind none) (subsetting false) (value none)) (case-return (declaration "") (target (ref r3)) (type none) (redefines none) (feature-kind none) (subsetting false) (value none)) (verification-case-usage (name "subVerificationCases") (abstract true) (type (ref r4)) (subsets (relationship (kind subsets) (implied false) (targets (ref r5) (ref r6))))))) (use-case-def (name "UseCase") (body brace (use-case-usage (name "subUseCases") (abstract true) (type (ref r7)) (subsets (relationship (kind subsets) (implied false) (targets (ref r8) (ref r9))))))) (case-def))))
)
~~~
