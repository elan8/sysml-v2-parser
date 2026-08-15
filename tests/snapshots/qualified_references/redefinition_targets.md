# META
~~~sexpr
(snapshot (type semantic) (description "Verifies attribute and port redefinition targets stay distinct from declaration names across shorthand, keyword, subset, value, unit, and directed forms."))
~~~
# SOURCE
~~~sysml
package RedefinitionTargets {
    part def RedefinitionExamples {
        attribute :>> name = "My Laptop";
        attribute :>> researchAndDevelopmentCost = 5E9 ['$'];
        attribute :>> outlet :> electricGrid.outlets;
        attribute redefines architecture = EeArchitecture::arm;
        port redefines rotationSpeedIn;
    }

    port def SuctionLevelPort :> Base::PowerOutPort {
        out attribute redefines suctionPower :> ISQ::power;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "redefinition_targets.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RedefinitionTargets {
    part def RedefinitionExamples {
        attribute :>> name = "My Laptop";
        attribute :>> researchAndDevelopmentCost = 5E9 ['$'];
        attribute :> electricGrid.outlets :>> outlet;
        attribute :>> architecture = EeArchitecture::arm;
        port  :>> rotationSpeedIn;
    }
    port def SuctionLevelPort :> Base::PowerOutPort {
        out attribute :> ISQ::power :>> suctionPower;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 88) (line 3) (column 23) (len 4)) (segments (segment 0 (token "name") (name "name") (separator none) (span (offset 88) (line 3) (column 23) (len 4)))))
    (reference r1 (scope relative) (span (offset 130) (line 4) (column 23) (len 26)) (segments (segment 0 (token "researchAndDevelopmentCost") (name "researchAndDevelopmentCost") (separator none) (span (offset 130) (line 4) (column 23) (len 26)))))
    (reference r2 (scope relative) (span (offset 202) (line 5) (column 33) (len 20)) (segments (segment 0 (token "electricGrid") (name "electricGrid") (separator none) (span (offset 202) (line 5) (column 33) (len 12))) (segment 1 (token "outlets") (name "outlets") (separator dot) (span (offset 215) (line 5) (column 46) (len 7)))))
    (reference r3 (scope relative) (span (offset 192) (line 5) (column 23) (len 6)) (segments (segment 0 (token "outlet") (name "outlet") (separator none) (span (offset 192) (line 5) (column 23) (len 6)))))
    (reference r4 (scope relative) (span (offset 252) (line 6) (column 29) (len 12)) (segments (segment 0 (token "architecture") (name "architecture") (separator none) (span (offset 252) (line 6) (column 29) (len 12)))))
    (reference r5 (scope relative) (span (offset 267) (line 6) (column 44) (len 19)) (segments (segment 0 (token "EeArchitecture") (name "EeArchitecture") (separator none) (span (offset 267) (line 6) (column 44) (len 14))) (segment 1 (token "arm") (name "arm") (separator colon-colon) (span (offset 283) (line 6) (column 60) (len 3)))))
    (reference r6 (scope relative) (span (offset 311) (line 7) (column 24) (len 15)) (segments (segment 0 (token "rotationSpeedIn") (name "rotationSpeedIn") (separator none) (span (offset 311) (line 7) (column 24) (len 15)))))
    (reference r7 (scope relative) (span (offset 368) (line 10) (column 34) (len 18)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 368) (line 10) (column 34) (len 4))) (segment 1 (token "PowerOutPort") (name "PowerOutPort") (separator colon-colon) (span (offset 374) (line 10) (column 40) (len 12)))))
    (reference r8 (scope relative) (span (offset 437) (line 11) (column 49) (len 10)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 437) (line 11) (column 49) (len 3))) (segment 1 (token "power") (name "power") (separator colon-colon) (span (offset 442) (line 11) (column 54) (len 5)))))
    (reference r9 (scope relative) (span (offset 421) (line 11) (column 33) (len 12)) (segments (segment 0 (token "suctionPower") (name "suctionPower") (separator none) (span (offset 421) (line 11) (column 33) (len 12)))))
  )
  (root (package (name "RedefinitionTargets") (body brace (part-def (name "RedefinitionExamples") (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 95) (line 3) (column 30) (len 11)) (string "My Laptop"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 159) (line 4) (column 52) (len 9)) (literal-with-unit (value (expression (span (offset 159) (line 4) (column 52) (len 3)) (real "5E9"))) (unit (expression (span (offset 164) (line 4) (column 57) (len 3)) (bracket (expression (span (offset 164) (line 4) (column 57) (len 3)) (unit "$")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 267) (line 6) (column 44) (len 19)) (ref r5))))) (body semicolon)) (port-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "SuctionLevelPort") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (body brace (attribute-usage (declaration-name none) (direction out) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r8)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
