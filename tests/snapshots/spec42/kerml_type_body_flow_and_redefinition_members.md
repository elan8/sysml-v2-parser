# META
~~~sexpr
(snapshot (type semantic) (description "A KerML type body owns the FeatureElement spellings `flow a.y to b.x1;` and `redefines predecessors [0];`, plus its direct AliasMember `alias name for Target::feature;`. Flow is TypeBodyElement -> FeatureMember -> OwnedFeatureMember -> FeatureElement -> Flow, redefines is a nameless Feature whose FeatureDeclaration is a bare FeatureSpecializationPart of Redefines plus a MultiplicityPart, and AliasMember is an explicit TypeBodyElement alternative. All three had no calc-shaped-body dispatch arm: flow/redefines were shredded into bare expressions and alias was split into `alias`, name, `for`, and target expressions. This pins each typed member in every represented KerML scope (classifier, struct, class, behavior, datatype, function) and the SysML calculation body, which reaches AliasMember through CalculationBodyItem -> ActionBodyItem -> NonBehaviorBodyItem. The calculation body also owns SysML-only `message` through ActionBodyItem -> StructureUsageMember -> Message (spec42 Gap 61 / RC7)."))
~~~
# SOURCE
~~~sysml
package Gap61KermlTypeBodyMembers {
    classifier FlowClassifier {
        flow a.y to b.x1;
        alias classifierFlow for Target::feature;
        redefines predecessors [0];
    }
    struct FlowStruct {
        flow a.y to b.x1;
        alias structFlow for Target::feature;
        redefines predecessors [0];
    }
    class FlowClass {
        flow a.y to b.x1;
        alias classFlow for Target::feature;
        redefines predecessors [0];
    }
    behavior FlowBehavior {
        flow a.y to b.x1;
        alias behaviorFlow for Target::feature;
        redefines predecessors [0];
    }
    datatype FlowDatatype {
        flow a.y to b.x1;
        alias datatypeFlow for Target::feature;
        redefines predecessors [0];
    }
    function FlowFunction {
        flow a.y to b.x1;
        alias functionFlow for Target::feature;
        redefines predecessors [0];
    }
    calc def CalculationBody {
        flow a.y to b.x1;
        message m of T;
        alias calculationFlow for Target::feature;
        redefines predecessors [0];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_type_body_flow_and_redefinition_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Gap61KermlTypeBodyMembers {
    classifier FlowClassifier {
        flow from a.y to b.x1;
        alias classifierFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    struct FlowStruct {
        flow from a.y to b.x1;
        alias structFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    class FlowClass {
        flow from a.y to b.x1;
        alias classFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    behavior FlowBehavior {
        flow from a.y to b.x1;
        alias behaviorFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    datatype FlowDatatype {
        flow from a.y to b.x1;
        alias datatypeFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    function FlowFunction {
        flow from a.y to b.x1;
        alias functionFlow for Target::feature;
        attribute :>> predecessors[0];
    }
    calc def CalculationBody {
        flow from a.y to b.x1;
        message m of T;
        alias calculationFlow for Target::feature;
        attribute :>> predecessors[0];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 127) (line 4) (column 34) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 127) (line 4) (column 34) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 135) (line 4) (column 42) (len 7)))))
    (reference r1 (scope relative) (span (offset 162) (line 5) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 162) (line 5) (column 19) (len 12)))))
    (reference r2 (scope relative) (span (offset 265) (line 9) (column 30) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 265) (line 9) (column 30) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 273) (line 9) (column 38) (len 7)))))
    (reference r3 (scope relative) (span (offset 300) (line 10) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 300) (line 10) (column 19) (len 12)))))
    (reference r4 (scope relative) (span (offset 400) (line 14) (column 29) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 400) (line 14) (column 29) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 408) (line 14) (column 37) (len 7)))))
    (reference r5 (scope relative) (span (offset 435) (line 15) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 435) (line 15) (column 19) (len 12)))))
    (reference r6 (scope relative) (span (offset 544) (line 19) (column 32) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 544) (line 19) (column 32) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 552) (line 19) (column 40) (len 7)))))
    (reference r7 (scope relative) (span (offset 579) (line 20) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 579) (line 20) (column 19) (len 12)))))
    (reference r8 (scope relative) (span (offset 688) (line 24) (column 32) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 688) (line 24) (column 32) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 696) (line 24) (column 40) (len 7)))))
    (reference r9 (scope relative) (span (offset 723) (line 25) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 723) (line 25) (column 19) (len 12)))))
    (reference r10 (scope relative) (span (offset 832) (line 29) (column 32) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 832) (line 29) (column 32) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 840) (line 29) (column 40) (len 7)))))
    (reference r11 (scope relative) (span (offset 867) (line 30) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 867) (line 30) (column 19) (len 12)))))
    (reference r12 (scope relative) (span (offset 1006) (line 35) (column 35) (len 15)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 1006) (line 35) (column 35) (len 6))) (segment 1 (token "feature") (name "feature") (separator colon-colon) (span (offset 1014) (line 35) (column 43) (len 7)))))
    (reference r13 (scope relative) (span (offset 1041) (line 36) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 1041) (line 36) (column 19) (len 12)))))
  )
  (root (package (name "Gap61KermlTypeBodyMembers") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "FlowClassifier") (specializes none) (body brace (flow-usage) (alias (name "classifierFlow") (target (ref r0)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword struct) (abstract false) (name "FlowStruct") (specializes none) (body brace (flow-usage) (alias (name "structFlow") (target (ref r2)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract false) (name "FlowClass") (specializes none) (body brace (flow-usage) (alias (name "classFlow") (target (ref r4)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword behavior) (abstract false) (name "FlowBehavior") (specializes none) (body brace (flow-usage) (alias (name "behaviorFlow") (target (ref r6)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "FlowDatatype") (specializes none) (body brace (flow-usage) (alias (name "datatypeFlow") (target (ref r8)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword function) (abstract false) (name "FlowFunction") (specializes none) (body brace (flow-usage) (alias (name "functionFlow") (target (ref r10)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (calc-def (name "CalculationBody") (modifiers) (body brace (flow-usage) (flow-usage) (alias (name "calculationFlow") (target (ref r12)) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
