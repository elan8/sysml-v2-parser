# META
~~~sexpr
(snapshot (type semantic) (description "A KerML type body owns the two FeatureElement spellings it had no dispatch arm for. `flow a.y to b.x1;` is TypeBodyElement -> FeatureMember -> OwnedFeatureMember -> FeatureElement -> Flow, and `redefines predecessors [0];` is a nameless Feature whose FeatureDeclaration is a bare FeatureSpecializationPart of Redefines plus a MultiplicityPart. Both were shredded into unrelated bare expressions with no diagnostic -- four members for the flow, two for the redefinition -- so this pins one typed member each in every scope reaching the calc-shaped body: classifier, struct, class, behavior, datatype and function, plus the SysML calculation body that shares it. The calculation body also owns `message`, which it reaches through ActionBodyItem -> StructureUsageMember -> Message; that is a SysML-only production, so the KerML scopes deliberately have no arm for it (spec42 Gap 61)."))
~~~
# SOURCE
~~~sysml
package Gap61KermlTypeBodyMembers {
    classifier FlowClassifier {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    struct FlowStruct {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    class FlowClass {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    behavior FlowBehavior {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    datatype FlowDatatype {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    function FlowFunction {
        flow a.y to b.x1;
        redefines predecessors [0];
    }
    calc def CalculationBody {
        flow a.y to b.x1;
        message m of T;
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
        attribute :>> predecessors[0];
    }
    struct FlowStruct {
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
    class FlowClass {
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
    behavior FlowBehavior {
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
    datatype FlowDatatype {
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
    function FlowFunction {
        flow from a.y to b.x1;
        attribute :>> predecessors[0];
    }
    calc def CalculationBody {
        flow from a.y to b.x1;
        message m of T;
        attribute :>> predecessors[0];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 112) (line 4) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 112) (line 4) (column 19) (len 12)))))
    (reference r1 (scope relative) (span (offset 204) (line 8) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 204) (line 8) (column 19) (len 12)))))
    (reference r2 (scope relative) (span (offset 294) (line 12) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 294) (line 12) (column 19) (len 12)))))
    (reference r3 (scope relative) (span (offset 390) (line 16) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 390) (line 16) (column 19) (len 12)))))
    (reference r4 (scope relative) (span (offset 486) (line 20) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 486) (line 20) (column 19) (len 12)))))
    (reference r5 (scope relative) (span (offset 582) (line 24) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 582) (line 24) (column 19) (len 12)))))
    (reference r6 (scope relative) (span (offset 705) (line 29) (column 19) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 705) (line 29) (column 19) (len 12)))))
  )
  (root (package (name "Gap61KermlTypeBodyMembers") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "FlowClassifier") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword struct) (abstract false) (name "FlowStruct") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword class) (abstract false) (name "FlowClass") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword behavior) (abstract false) (name "FlowBehavior") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "FlowDatatype") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (kerml-classifier (keyword function) (abstract false) (name "FlowFunction") (specializes none) (body brace (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (calc-def (name "CalculationBody") (modifiers) (body brace (flow-usage) (flow-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
