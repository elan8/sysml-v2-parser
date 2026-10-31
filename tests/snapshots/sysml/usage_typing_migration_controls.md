# META
~~~sexpr
(snapshot (type semantic) (description "Preserves migrated allocation, anonymous item, concern, requirement and usage-header regression cases through the canonical parser projection."))
~~~
# SOURCE
~~~sysml
package TypingMigrationControls {
    // allocation_usage_accepts_named_reference_ends
    allocation a : $::Allocations::T allocate logical ::> src to physical ::> dst;
    // item_usage_accepts_anonymous_redefinition_with_value
    item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);
    // item_usage_accepts_anonymous_redefinition_with_type_and_body
    item :>> shape : Cylinder { :>> radius = 14 [mm]; :>> height = 40 [mm]; }
    // item_usage_named_form_still_parses
    item wheelShape : Circle;
    // concern_usage_def_form_with_type_and_body_is_a_definition
    concern def SafetyConcern : BaseConcern { doc /* d */ }
    // requirement_usage_accepts_short_name
    requirement <'1.1'> vehicleMass1: MassLimitationRequirement { subject vehicle : Vehicle; }
    // usage_header_accepts_typing_then_specialization
    part usage_header_accepts_typing_then_specialization : Engine :> BasePart :>> oldPart ;
    // usage_header_accepts_specialization_then_typing
    part usage_header_accepts_specialization_then_typing subsets base : Engine ;
    // usage_header_preserves_references_and_crosses
    part usage_header_preserves_references_and_crosses : T references a crosses b ;
    // usage_header_preserves_intersects
    part usage_header_preserves_intersects : T intersects a ;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_typing_migration_controls.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package TypingMigrationControls {
    allocation a : $::Allocations::T allocate logical references src to physical references dst;
    item :>> shape = new Box(4800[mm], 1840[mm], 1350[mm]);
    item :>> shape : Cylinder {
        :>> radius = 14[mm];
        :>> height = 40[mm];
    }
    item wheelShape : Circle;
    concern def SafetyConcern : BaseConcern {
        doc
        /* d */
    }
    requirement <'1.1'> vehicleMass1 : MassLimitationRequirement {
        subject vehicle : Vehicle;
    }
    part usage_header_accepts_typing_then_specialization : Engine :> BasePart :>> oldPart;
    part usage_header_accepts_specialization_then_typing : Engine subsets base;
    part usage_header_preserves_references_and_crosses : T references a crosses b;
    part usage_header_preserves_intersects : T intersects a;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope absolute) (span (offset 106) (line 3) (column 20) (len 17)) (segments (segment 0 (token "Allocations") (name "Allocations") (separator none) (span (offset 109) (line 3) (column 23) (len 11))) (segment 1 (token "T") (name "T") (separator colon-colon) (span (offset 122) (line 3) (column 36) (len 1)))))
    (reference r1 (scope relative) (span (offset 133) (line 3) (column 47) (len 7)) (segments (segment 0 (token "logical") (name "logical") (separator none) (span (offset 133) (line 3) (column 47) (len 7)))))
    (reference r2 (scope relative) (span (offset 145) (line 3) (column 59) (len 3)) (segments (segment 0 (token "src") (name "src") (separator none) (span (offset 145) (line 3) (column 59) (len 3)))))
    (reference r3 (scope relative) (span (offset 152) (line 3) (column 66) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 152) (line 3) (column 66) (len 8)))))
    (reference r4 (scope relative) (span (offset 165) (line 3) (column 79) (len 3)) (segments (segment 0 (token "dst") (name "dst") (separator none) (span (offset 165) (line 3) (column 79) (len 3)))))
    (reference r5 (scope relative) (span (offset 243) (line 5) (column 14) (len 5)) (segments (segment 0 (token "shape") (name "shape") (separator none) (span (offset 243) (line 5) (column 14) (len 5)))))
    (reference r6 (scope relative) (span (offset 255) (line 5) (column 26) (len 3)) (segments (segment 0 (token "Box") (name "Box") (separator none) (span (offset 255) (line 5) (column 26) (len 3)))))
    (reference r7 (scope relative) (span (offset 265) (line 5) (column 36) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 265) (line 5) (column 36) (len 2)))))
    (reference r8 (scope relative) (span (offset 276) (line 5) (column 47) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 276) (line 5) (column 47) (len 2)))))
    (reference r9 (scope relative) (span (offset 287) (line 5) (column 58) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 287) (line 5) (column 58) (len 2)))))
    (reference r10 (scope relative) (span (offset 382) (line 7) (column 22) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 382) (line 7) (column 22) (len 8)))))
    (reference r11 (scope relative) (span (offset 374) (line 7) (column 14) (len 5)) (segments (segment 0 (token "shape") (name "shape") (separator none) (span (offset 374) (line 7) (column 14) (len 5)))))
    (reference r12 (scope relative) (span (offset 397) (line 7) (column 37) (len 6)) (segments (segment 0 (token "radius") (name "radius") (separator none) (span (offset 397) (line 7) (column 37) (len 6)))))
    (reference r13 (scope relative) (span (offset 410) (line 7) (column 50) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 410) (line 7) (column 50) (len 2)))))
    (reference r14 (scope relative) (span (offset 419) (line 7) (column 59) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 419) (line 7) (column 59) (len 6)))))
    (reference r15 (scope relative) (span (offset 432) (line 7) (column 72) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 432) (line 7) (column 72) (len 2)))))
    (reference r16 (scope relative) (span (offset 503) (line 9) (column 23) (len 6)) (segments (segment 0 (token "Circle") (name "Circle") (separator none) (span (offset 503) (line 9) (column 23) (len 6)))))
    (reference r17 (scope relative) (span (offset 608) (line 11) (column 33) (len 11)) (segments (segment 0 (token "BaseConcern") (name "BaseConcern") (separator none) (span (offset 608) (line 11) (column 33) (len 11)))))
    (reference r18 (scope relative) (span (offset 718) (line 13) (column 39) (len 25)) (segments (segment 0 (token "MassLimitationRequirement") (name "MassLimitationRequirement") (separator none) (span (offset 718) (line 13) (column 39) (len 25)))))
    (reference r19 (scope relative) (span (offset 764) (line 13) (column 85) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 764) (line 13) (column 85) (len 7)))))
    (reference r20 (scope relative) (span (offset 889) (line 15) (column 60) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 889) (line 15) (column 60) (len 6)))))
    (reference r21 (scope relative) (span (offset 899) (line 15) (column 70) (len 8)) (segments (segment 0 (token "BasePart") (name "BasePart") (separator none) (span (offset 899) (line 15) (column 70) (len 8)))))
    (reference r22 (scope relative) (span (offset 912) (line 15) (column 83) (len 7)) (segments (segment 0 (token "oldPart") (name "oldPart") (separator none) (span (offset 912) (line 15) (column 83) (len 7)))))
    (reference r23 (scope relative) (span (offset 1049) (line 17) (column 73) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1049) (line 17) (column 73) (len 6)))))
    (reference r24 (scope relative) (span (offset 1042) (line 17) (column 66) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 1042) (line 17) (column 66) (len 4)))))
    (reference r25 (scope relative) (span (offset 1168) (line 19) (column 58) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 1168) (line 19) (column 58) (len 1)))))
    (reference r26 (scope relative) (span (offset 1181) (line 19) (column 71) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 1181) (line 19) (column 71) (len 1)))))
    (reference r27 (scope relative) (span (offset 1191) (line 19) (column 81) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 1191) (line 19) (column 81) (len 1)))))
    (reference r28 (scope relative) (span (offset 1281) (line 21) (column 46) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 1281) (line 21) (column 46) (len 1)))))
    (reference r29 (scope relative) (span (offset 1294) (line 21) (column 59) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 1294) (line 21) (column 59) (len 1)))))
  )
  (root (package (visibility none) (name "TypingMigrationControls") (body brace (allocation-usage (name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (ends (connector-end (multiplicity none) (target (ref r1)) (references (ref r2))) (connector-end (multiplicity none) (target (ref r3)) (references (ref r4)))) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 251) (line 5) (column 22) (len 40)) (constructor (type (ref r6)) (arguments (argument (parameter none) (value (expression (span (offset 259) (line 5) (column 30) (len 9)) (bracket (base (expression (span (offset 259) (line 5) (column 30) (len 4)) (integer 4800))) (operands (sequence-list (element first (expression (span (offset 265) (line 5) (column 36) (len 2)) (ref r7))))))))) (argument (parameter none) (value (expression (span (offset 270) (line 5) (column 41) (len 9)) (bracket (base (expression (span (offset 270) (line 5) (column 41) (len 4)) (integer 1840))) (operands (sequence-list (element first (expression (span (offset 276) (line 5) (column 47) (len 2)) (ref r8))))))))) (argument (parameter none) (value (expression (span (offset 281) (line 5) (column 52) (len 9)) (bracket (base (expression (span (offset 281) (line 5) (column 52) (len 4)) (integer 1350))) (operands (sequence-list (element first (expression (span (offset 287) (line 5) (column 58) (len 2)) (ref r9))))))))))))))) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration none) (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 406) (line 7) (column 46) (len 7)) (bracket (base (expression (span (offset 406) (line 7) (column 46) (len 2)) (integer 14))) (operands (sequence-list (element first (expression (span (offset 410) (line 7) (column 50) (len 2)) (ref r13)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 428) (line 7) (column 68) (len 7)) (bracket (base (expression (span (offset 428) (line 7) (column 68) (len 2)) (integer 40))) (operands (sequence-list (element first (expression (span (offset 432) (line 7) (column 72) (len 2)) (ref r15)))))))))) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "wheelShape") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (concern-usage (name "SafetyConcern") (short-name none) (visibility none) (abstract false) (individual false) (definition true) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (subsets none) (redefines none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 628) (line 11) (column 53) (len 3)) (normalized "d "))))) (requirement-usage (name "vehicleMass1") (short-name "1.1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage_header_accepts_typing_then_specialization") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r21))) (value none))) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage_header_accepts_specialization_then_typing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r24))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage_header_preserves_references_and_crosses") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r26)))) (crosses (relationship (kind crosses) (implied false) (targets (ref r27)))) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage_header_preserves_intersects") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects (relationship (kind intersects) (implied false) (targets (ref r29)))) (value none) (body semicolon)))))
)
~~~
