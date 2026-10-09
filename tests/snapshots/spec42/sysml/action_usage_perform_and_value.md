# META
~~~sexpr
(snapshot (type semantic) (description "An action usage body is the same ActionBody as an action definition body, so it owns `perform` members in both PerformActionUsageDeclaration spellings; and an action usage carries the ValuePart of ActionUsageDeclaration, including inside a `perform action :>> x { ... }` body (sysml-v2-parser #105, spec42 #256)."))
~~~
# SOURCE
~~~sysml
package Functions {
    action def Step;
    action stepA : Step;
    action stepB : Step;

    action group {
        perform stepA;
        perform action second :> stepB;
        action nested { perform stepA; }
    }
    action def Group {
        perform stepA;
    }

    action def Torque { action generate; action amplify; }
    action fourCylinder;
    part vehicle {
        perform action :>> Torque {
            action :>> generate = fourCylinder;
            action named :>> amplify default = fourCylinder;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_usage_perform_and_value.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Functions {
    action def Step;
    action stepA : Step;
    action stepB : Step;
    action group {
        perform stepA;
        perform action second :> stepB;
        action nested {
            perform stepA;
        }
    }
    action def Group {
        perform stepA;
    }
    action def Torque {
        action generate;
        action amplify;
    }
    action fourCylinder;
    part vehicle {
        perform action :>> Torque {
            action :>> generate = fourCylinder;
            action named :>> amplify default = fourCylinder;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 60) (line 3) (column 20) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 60) (line 3) (column 20) (len 4)))))
    (reference r1 (scope relative) (span (offset 85) (line 4) (column 20) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 85) (line 4) (column 20) (len 4)))))
    (reference r2 (scope relative) (span (offset 127) (line 7) (column 17) (len 5)) (segments (segment 0 (token "stepA") (name "stepA") (separator none) (span (offset 127) (line 7) (column 17) (len 5)))))
    (reference r3 (scope relative) (span (offset 167) (line 8) (column 34) (len 5)) (segments (segment 0 (token "stepB") (name "stepB") (separator none) (span (offset 167) (line 8) (column 34) (len 5)))))
    (reference r4 (scope relative) (span (offset 206) (line 9) (column 33) (len 5)) (segments (segment 0 (token "stepA") (name "stepA") (separator none) (span (offset 206) (line 9) (column 33) (len 5)))))
    (reference r5 (scope relative) (span (offset 260) (line 12) (column 17) (len 5)) (segments (segment 0 (token "stepA") (name "stepA") (separator none) (span (offset 260) (line 12) (column 17) (len 5)))))
    (reference r6 (scope relative) (span (offset 404) (line 18) (column 28) (len 6)) (segments (segment 0 (token "Torque") (name "Torque") (separator none) (span (offset 404) (line 18) (column 28) (len 6)))))
  )
  (root (package (name "Functions") (body brace (action-def (name "Step") (modifiers) (specializes none) (body semicolon)) (action-usage (keyword action) (name "stepA") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "stepB") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "group") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (perform (target (reference (action (ref r2)) (redefines none))) (value none) (body semicolon)) (perform (target (action (name "second") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3))) (value none)) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (action-usage (keyword action) (name "nested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (perform (target (reference (action (ref r4)) (redefines none))) (value none) (body semicolon)))))) (action-def (name "Group") (modifiers) (specializes none) (body brace (perform (target (reference (action (ref r5)) (redefines none))) (value none) (body semicolon)))) (action-def (name "Torque") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "generate") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "amplify") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (action-usage (keyword action) (name "fourCylinder") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none))) (value none) (body brace (action) (action))))))))
)
~~~
