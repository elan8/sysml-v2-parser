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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 92) (line 3) (column 40) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 92) (line 3) (column 40) (len 1)))))
    (reference r1 (scope relative) (span (offset 123) (line 5) (column 23) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 123) (line 5) (column 23) (len 1)))))
    (reference r2 (scope relative) (span (offset 159) (line 6) (column 34) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 159) (line 6) (column 34) (len 1)))))
    (reference r3 (scope relative) (span (offset 198) (line 7) (column 36) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 198) (line 7) (column 36) (len 1)))))
    (reference r4 (scope relative) (span (offset 271) (line 11) (column 20) (len 3)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 271) (line 11) (column 20) (len 3)))))
    (reference r5 (scope relative) (span (offset 295) (line 12) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 295) (line 12) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 299) (line 12) (column 24) (len 16)))))
    (reference r6 (scope relative) (span (offset 336) (line 13) (column 20) (len 23)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 336) (line 13) (column 20) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 356) (line 13) (column 40) (len 3)))))
    (reference r7 (scope relative) (span (offset 380) (line 14) (column 20) (len 40)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 380) (line 14) (column 20) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 400) (line 14) (column 40) (len 3))) (segment 2 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 404) (line 14) (column 44) (len 16)))))
    (reference r8 (scope absolute) (span (offset 441) (line 15) (column 20) (len 43)) (segments (segment 0 (token "VerifyFeatureChain") (name "VerifyFeatureChain") (separator none) (span (offset 444) (line 15) (column 23) (len 18))) (segment 1 (token "rss") (name "rss") (separator colon-colon) (span (offset 464) (line 15) (column 43) (len 3))) (segment 2 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 468) (line 15) (column 47) (len 16)))))
    (reference r9 (scope relative) (span (offset 505) (line 16) (column 20) (len 27)) (segments (segment 0 (token "'quoted usage'") (name "quoted usage") (separator none) (span (offset 505) (line 16) (column 20) (len 14))) (segment 1 (token "'nested req'") (name "nested req") (separator dot) (span (offset 520) (line 16) (column 35) (len 12)))))
    (reference r10 (scope relative) (span (offset 553) (line 17) (column 20) (len 22)) (segments (segment 0 (token "vehicleMassRequirement") (name "vehicleMassRequirement") (separator none) (span (offset 553) (line 17) (column 20) (len 22)))))
    (reference r11 (scope relative) (span (offset 580) (line 17) (column 47) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 580) (line 17) (column 47) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 584) (line 17) (column 51) (len 16)))))
    (reference r12 (scope relative) (span (offset 621) (line 18) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 621) (line 18) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 625) (line 18) (column 24) (len 16)))))
    (reference r13 (scope relative) (span (offset 646) (line 18) (column 45) (len 15)) (segments (segment 0 (token "massRequirement") (name "massRequirement") (separator none) (span (offset 646) (line 18) (column 45) (len 15)))))
    (reference r14 (scope relative) (span (offset 705) (line 19) (column 43) (len 1)) (segments (segment 0 (token "R") (name "R") (separator none) (span (offset 705) (line 19) (column 43) (len 1)))))
  )
  (root (package (name "VerifyFeatureChain") (body brace (requirement-def (name "R") (modifiers) (body brace (requirement-usage (name "recoverFromStall") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)))) (requirement-usage (name "rss") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "quoted usage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (requirement-usage (name "nested req") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)))) (verification-case-def (name "V") (modifiers) (body brace (objective (visibility none) (name none) (type none) (body brace (verify (explicit-requirement false) (requirement none) (target (ref r4)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r5)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r6)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r7)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r8)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r9)) (redefines none)) (verify (explicit-requirement false) (requirement none) (target (ref r10)) (redefines (ref r11))) (verify (explicit-requirement false) (requirement none) (target (ref r12)) (redefines (ref r13))) (verify (explicit-requirement true) (requirement (name "declared") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (body semicolon)) (target none) (redefines none)))))))))
)
~~~
