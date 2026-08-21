# META
~~~sexpr
(snapshot (type semantic) (description "Exact metadata-body usage contexts extracted from SimpleVehicleModel: nested Risk values retain reference redefinition targets and values instead of being reinterpreted as AttributeUsage declarations. SysML textual BNF MetadataBody/MetadataBodyUsage 1678-1693 and pinned Pilot KerML.xtext 1098-1115 agree."))
~~~
# SOURCE
~~~sysml
package SimpleVehicleModel {
    package VehicleTradeOffAnalysis {
        @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl {
            totalRisk = medium;
            technicalRisk = medium;
            scheduleRisk = medium;
            costRisk = RiskLevelEnum::low;
        }
        @Risk about engineTradeOffAnalysis::vehicle_b_engine4cyl::engine::fuelEfficiency {
            technicalRisk {
                probability = 0.3;
                impact = 0.5;
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_metadata_simple_vehicle_model.md"
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
    (reference r0 (scope relative) (span (offset 76) (line 3) (column 10) (len 4)) (segments (segment 0 (token "Risk") (name "Risk") (separator none) (span (offset 76) (line 3) (column 10) (len 4)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 21) (len 44)) (segments (segment 0 (token "engineTradeOffAnalysis") (name "engineTradeOffAnalysis") (separator none) (span (offset 87) (line 3) (column 21) (len 22))) (segment 1 (token "vehicle_b_engine4cyl") (name "vehicle_b_engine4cyl") (separator colon-colon) (span (offset 111) (line 3) (column 45) (len 20)))))
    (reference r2 (scope relative) (span (offset 146) (line 4) (column 13) (len 9)) (segments (segment 0 (token "totalRisk") (name "totalRisk") (separator none) (span (offset 146) (line 4) (column 13) (len 9)))))
    (reference r3 (scope relative) (span (offset 158) (line 4) (column 25) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 158) (line 4) (column 25) (len 6)))))
    (reference r4 (scope relative) (span (offset 178) (line 5) (column 13) (len 13)) (segments (segment 0 (token "technicalRisk") (name "technicalRisk") (separator none) (span (offset 178) (line 5) (column 13) (len 13)))))
    (reference r5 (scope relative) (span (offset 194) (line 5) (column 29) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 194) (line 5) (column 29) (len 6)))))
    (reference r6 (scope relative) (span (offset 214) (line 6) (column 13) (len 12)) (segments (segment 0 (token "scheduleRisk") (name "scheduleRisk") (separator none) (span (offset 214) (line 6) (column 13) (len 12)))))
    (reference r7 (scope relative) (span (offset 229) (line 6) (column 28) (len 6)) (segments (segment 0 (token "medium") (name "medium") (separator none) (span (offset 229) (line 6) (column 28) (len 6)))))
    (reference r8 (scope relative) (span (offset 249) (line 7) (column 13) (len 8)) (segments (segment 0 (token "costRisk") (name "costRisk") (separator none) (span (offset 249) (line 7) (column 13) (len 8)))))
    (reference r9 (scope relative) (span (offset 260) (line 7) (column 24) (len 18)) (segments (segment 0 (token "RiskLevelEnum") (name "RiskLevelEnum") (separator none) (span (offset 260) (line 7) (column 24) (len 13))) (segment 1 (token "low") (name "low") (separator colon-colon) (span (offset 275) (line 7) (column 39) (len 3)))))
    (reference r10 (scope relative) (span (offset 299) (line 9) (column 10) (len 4)) (segments (segment 0 (token "Risk") (name "Risk") (separator none) (span (offset 299) (line 9) (column 10) (len 4)))))
    (reference r11 (scope relative) (span (offset 310) (line 9) (column 21) (len 68)) (segments (segment 0 (token "engineTradeOffAnalysis") (name "engineTradeOffAnalysis") (separator none) (span (offset 310) (line 9) (column 21) (len 22))) (segment 1 (token "vehicle_b_engine4cyl") (name "vehicle_b_engine4cyl") (separator colon-colon) (span (offset 334) (line 9) (column 45) (len 20))) (segment 2 (token "engine") (name "engine") (separator colon-colon) (span (offset 356) (line 9) (column 67) (len 6))) (segment 3 (token "fuelEfficiency") (name "fuelEfficiency") (separator colon-colon) (span (offset 364) (line 9) (column 75) (len 14)))))
    (reference r12 (scope relative) (span (offset 393) (line 10) (column 13) (len 13)) (segments (segment 0 (token "technicalRisk") (name "technicalRisk") (separator none) (span (offset 393) (line 10) (column 13) (len 13)))))
    (reference r13 (scope relative) (span (offset 425) (line 11) (column 17) (len 11)) (segments (segment 0 (token "probability") (name "probability") (separator none) (span (offset 425) (line 11) (column 17) (len 11)))))
    (reference r14 (scope relative) (span (offset 460) (line 12) (column 17) (len 6)) (segments (segment 0 (token "impact") (name "impact") (separator none) (span (offset 460) (line 12) (column 17) (len 6)))))
  )
  (root (package (name "SimpleVehicleModel") (body brace (package (name "VehicleTradeOffAnalysis") (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about (ref r1)) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r2)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 158) (line 4) (column 25) (len 6)) (ref r3))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r4)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 194) (line 5) (column 29) (len 6)) (ref r5))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r6)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 229) (line 6) (column 28) (len 6)) (ref r7))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r8)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 260) (line 7) (column 24) (len 18)) (ref r9))))) (body semicolon)))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r10)) (about (ref r11)) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r12)) (value none) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r13)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 439) (line 11) (column 31) (len 3)) (real "0.3"))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r14)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 469) (line 12) (column 26) (len 3)) (real "0.5"))))) (body semicolon)))))))))))
)
~~~
