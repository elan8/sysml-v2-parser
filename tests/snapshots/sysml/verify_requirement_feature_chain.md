# META
~~~sexpr
(snapshot (type semantic) (description "RequirementVerificationUsage's OwnedReferenceSubsetting alternative is a qualified name or a `.`-separated feature chain, absolute or relative, quoted or unquoted, with or without a `:>>` redefinition whose target is the same kind of path. `verify rss.recoverFromStall;` inside a verification objective is therefore a typed member, not recovered_requirement_body_element; `::` qualification, a bare name, and the explicit `requirement` declaration remain distinct alternatives of the same production (SysML textual BNF 8.2.2.24)."))
~~~
# SOURCE
~~~sysml
package VerifyFeatureChain {
    requirement def R {
        requirement recoverFromStall : R;
    }
    requirement rss : R;
    requirement 'quoted usage' : R {
        requirement 'nested req' : R;
    }
    verification def V {
        objective {
            verify rss;
            verify rss.recoverFromStall;
            verify VerifyFeatureChain::rss;
            verify VerifyFeatureChain::rss.recoverFromStall;
            verify $::VerifyFeatureChain::rss.recoverFromStall;
            verify 'quoted usage'.'nested req';
            verify vehicleMassRequirement :>> rss.recoverFromStall;
            verify rss.recoverFromStall :>> massRequirement;
            verify requirement declared : R;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "verify_requirement_feature_chain.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 271) (line 11) (column 20) (len 3)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 271) (line 11) (column 20) (len 3)))))
    (reference r1 (scope relative) (span (offset 295) (line 12) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 295) (line 12) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 299) (line 12) (column 24) (len 16)))))
    (reference r2 (scope relative) (span (offset 336) (line 13) (column 20) (len 23)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 336) (line 13) (column 20) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 356) (line 13) (column 40) (len 3)))))
    (reference r3 (scope relative) (span (offset 380) (line 14) (column 20) (len 40)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 380) (line 14) (column 20) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 400) (line 14) (column 40) (len 3))) (segment 2 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 404) (line 14) (column 44) (len 16)))))
    (reference r4 (scope absolute) (span (offset 441) (line 15) (column 20) (len 43)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 444) (line 15) (column 23) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 464) (line 15) (column 43) (len 3))) (segment 2 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 468) (line 15) (column 47) (len 16)))))
    (reference r5 (scope relative) (span (offset 505) (line 16) (column 20) (len 27)) (segments (segment 0 (token "'quoted usage'") (name "quoted usage") (separator none) (span (offset 505) (line 16) (column 20) (len 14))) (segment 1 (token "'nested req'") (name "nested req") (separator dot) (span (offset 520) (line 16) (column 35) (len 12)))))
    (reference r6 (scope relative) (span (offset 553) (line 17) (column 20) (len 22)) (segments (segment 0 (token "vehicleMassRequirement") (name "vehicleMassRequirement") (separator none) (span (offset 553) (line 17) (column 20) (len 22)))))
    (reference r7 (scope relative) (span (offset 580) (line 17) (column 47) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 580) (line 17) (column 47) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 584) (line 17) (column 51) (len 16)))))
    (reference r8 (scope relative) (span (offset 621) (line 18) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 621) (line 18) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 625) (line 18) (column 24) (len 16)))))
    (reference r9 (scope relative) (span (offset 646) (line 18) (column 45) (len 15)) (segments (segment 0 (token "massRequirement") (name "massRequirement") (separator none) (span (offset 646) (line 18) (column 45) (len 15)))))
    (reference r10 (scope relative) (span (offset 705) (line 19) (column 43) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 705) (line 19) (column 43) (len 1)))))
  )
  (root (package (name "VerifyFeatureChain") (body brace (requirement-def (name "R") (modifiers) (body brace (requirement-usage (name "recoverFromStall") (multiplicity none)))) (requirement-usage (name "rss") (multiplicity none)) (requirement-usage (name "quoted usage") (multiplicity none)) (verification-case-def (name "V") (modifiers) (body brace (objective (visibility none) (name none) (type none) (body brace (verify (explicit-requirement false) (requirement none) (target (ref r0)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r1)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r2)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r3)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r4)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r5)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r6)) (redefines (ref r7))) (verify (explicit-requirement false) (requirement none) (target (ref r8)) (redefines (ref r9))) (verify (explicit-requirement true) (requirement (name "declared") (type (ref r10)) (body semicolon)) (target none) (redefines none)))))))))
)
~~~
