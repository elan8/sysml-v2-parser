# META
~~~sexpr
(snapshot (type semantic) (description "Both sigil productions in every materially distinct owning scope, each with a valid sibling before and after it. `@` reaches these bodies as the MetadataFeature alternative of AnnotatingElement; `#` reaches them as PrefixMetadataMember on the following member and as the standalone ExtendedUsage member. The `#refinement dependency X to Y;` rows are the PrefixMetadataAnnotation half of the Dependency production, which action and requirement bodies own through DefinitionMember."))
~~~
# SOURCE
~~~sysml
package MetadataSigilOwningScopes {
    metadata def Tag;
    part def Supplier;
    part def Client;
    part def InPartDef {
        part beforeDef;
        @Tag;
        #Tag;
        part afterDef;
    }
    part inPartUsage : InPartDef {
        part beforeUsage;
        @Tag;
        #Tag;
        part afterUsage;
    }
    attribute def InAttributeDef {
        attribute beforeAttribute : Anything;
        @Tag;
        #Tag;
        attribute afterAttribute : Anything;
    }
    port def InPortDef {
        attribute beforePort : Anything;
        @Tag;
        #Tag;
        attribute afterPort : Anything;
    }
    occurrence def InOccurrenceDef {
        attribute beforeOccurrence : Anything;
        @Tag;
        #Tag;
        attribute afterOccurrence : Anything;
    }
    action def InActionDef {
        action beforeAction;
        @Tag;
        #Tag;
        #refinement dependency Client to Supplier;
        action afterAction;
    }
    state def InStateDef {
        state beforeState;
        @Tag;
        #Tag;
        state afterState;
    }
    requirement def InRequirementDef {
        attribute beforeRequirement : Anything;
        @Tag;
        #Tag;
        #refinement dependency Client to Supplier;
        attribute afterRequirement : Anything;
    }
    use case def InUseCaseDef {
        attribute beforeUseCase : Anything;
        @Tag;
        #Tag;
        attribute afterUseCase : Anything;
    }
    calc def InCalcDef {
        attribute beforeCalc : Anything;
        @Tag;
        #Tag;
        attribute afterCalc : Anything;
    }
    view def InViewDef {
        @Tag;
        #Tag;
    }
    enum def InEnumDef {
        @Tag;
        enum afterEnum;
    }
    interface def InInterfaceDef {
        @Tag;
        #Tag;
    }
    connection def InConnectionDef {
        @Tag;
        #Tag;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_sigil_owning_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MetadataSigilOwningScopes {
    metadata def Tag;
    part def Supplier;
    part def Client;
    part def InPartDef {
        part beforeDef;
        @Tag;
        #Tag;
        part afterDef;
    }
    part inPartUsage : InPartDef {
        part beforeUsage;
        @Tag;
        #Tag;
        part afterUsage;
    }
    attribute def InAttributeDef {
        attribute beforeAttribute : Anything;
        @Tag;
        #Tag;
        attribute afterAttribute : Anything;
    }
    port def InPortDef {
        attribute beforePort : Anything;
        @Tag;
        #Tag;
        attribute afterPort : Anything;
    }
    occurrence def InOccurrenceDef {
        attribute beforeOccurrence : Anything;
        @Tag;
        #Tag;
        attribute afterOccurrence : Anything;
    }
    action def InActionDef {
        action beforeAction;
        @Tag;
        #Tag;
        #refinement
        dependency from Client to Supplier;
        action afterAction;
    }
    state def InStateDef {
        state beforeState;
        @Tag;
        #Tag;
        state afterState;
    }
    requirement def InRequirementDef {
        attribute beforeRequirement : Anything;
        @Tag;
        #Tag;
        #refinement
        dependency from Client to Supplier;
        attribute afterRequirement : Anything;
    }
    use case def InUseCaseDef {
        attribute beforeUseCase : Anything;
        @Tag;
        #Tag;
        attribute afterUseCase : Anything;
    }
    calc def InCalcDef {
        attribute beforeCalc : Anything;
        @Tag;
        #Tag;
        attribute afterCalc : Anything;
    }
    view def InViewDef {
        @Tag;
        #Tag;
    }
    enum def InEnumDef {
        @Tag;
        enum afterEnum;
    }
    interface def InInterfaceDef {
        @Tag;
        #Tag;
    }
    connection def InConnectionDef {
        @Tag;
        #Tag;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 160) (line 7) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 160) (line 7) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 174) (line 8) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 174) (line 8) (column 10) (len 3)))))
    (reference r2 (scope relative) (span (offset 231) (line 11) (column 24) (len 9)) (segments (segment 0 (token "InPartDef") (name "InPartDef") (separator none) (span (offset 231) (line 11) (column 24) (len 9)))))
    (reference r3 (scope relative) (span (offset 278) (line 13) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 278) (line 13) (column 10) (len 3)))))
    (reference r4 (scope relative) (span (offset 292) (line 14) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 292) (line 14) (column 10) (len 3)))))
    (reference r5 (scope relative) (span (offset 399) (line 18) (column 37) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 399) (line 18) (column 37) (len 8)))))
    (reference r6 (scope relative) (span (offset 418) (line 19) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 418) (line 19) (column 10) (len 3)))))
    (reference r7 (scope relative) (span (offset 432) (line 20) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 432) (line 20) (column 10) (len 3)))))
    (reference r8 (scope relative) (span (offset 472) (line 21) (column 36) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 472) (line 21) (column 36) (len 8)))))
    (reference r9 (scope relative) (span (offset 544) (line 24) (column 32) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 544) (line 24) (column 32) (len 8)))))
    (reference r10 (scope relative) (span (offset 563) (line 25) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 563) (line 25) (column 10) (len 3)))))
    (reference r11 (scope relative) (span (offset 577) (line 26) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 577) (line 26) (column 10) (len 3)))))
    (reference r12 (scope relative) (span (offset 612) (line 27) (column 31) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 612) (line 27) (column 31) (len 8)))))
    (reference r13 (scope relative) (span (offset 859) (line 37) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 859) (line 37) (column 10) (len 3)))))
    (reference r14 (scope relative) (span (offset 873) (line 38) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 873) (line 38) (column 10) (len 3)))))
    (reference r15 (scope relative) (span (offset 887) (line 39) (column 10) (len 10)) (segments (segment 0 (token "refinement") (name "refinement") (separator none) (span (offset 887) (line 39) (column 10) (len 10)))))
    (reference r16 (scope relative) (span (offset 909) (line 39) (column 32) (len 6)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 909) (line 39) (column 32) (len 6)))))
    (reference r17 (scope relative) (span (offset 919) (line 39) (column 42) (len 8)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 919) (line 39) (column 42) (len 8)))))
    (reference r18 (scope relative) (span (offset 1026) (line 44) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1026) (line 44) (column 10) (len 3)))))
    (reference r19 (scope relative) (span (offset 1040) (line 45) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1040) (line 45) (column 10) (len 3)))))
    (reference r20 (scope relative) (span (offset 1173) (line 50) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1173) (line 50) (column 10) (len 3)))))
    (reference r21 (scope relative) (span (offset 1187) (line 51) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1187) (line 51) (column 10) (len 3)))))
    (reference r22 (scope relative) (span (offset 1201) (line 52) (column 10) (len 10)) (segments (segment 0 (token "refinement") (name "refinement") (separator none) (span (offset 1201) (line 52) (column 10) (len 10)))))
    (reference r23 (scope relative) (span (offset 1223) (line 52) (column 32) (len 6)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 1223) (line 52) (column 32) (len 6)))))
    (reference r24 (scope relative) (span (offset 1233) (line 52) (column 42) (len 8)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 1233) (line 52) (column 42) (len 8)))))
    (reference r25 (scope relative) (span (offset 1381) (line 57) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1381) (line 57) (column 10) (len 3)))))
    (reference r26 (scope relative) (span (offset 1395) (line 58) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1395) (line 58) (column 10) (len 3)))))
    (reference r27 (scope relative) (span (offset 1505) (line 62) (column 32) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 1505) (line 62) (column 32) (len 8)))))
    (reference r28 (scope relative) (span (offset 1524) (line 63) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1524) (line 63) (column 10) (len 3)))))
    (reference r29 (scope relative) (span (offset 1538) (line 64) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1538) (line 64) (column 10) (len 3)))))
    (reference r30 (scope relative) (span (offset 1573) (line 65) (column 31) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 1573) (line 65) (column 31) (len 8)))))
    (reference r31 (scope relative) (span (offset 1623) (line 68) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1623) (line 68) (column 10) (len 3)))))
    (reference r32 (scope relative) (span (offset 1637) (line 69) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1637) (line 69) (column 10) (len 3)))))
    (reference r33 (scope relative) (span (offset 1682) (line 72) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1682) (line 72) (column 10) (len 3)))))
    (reference r34 (scope relative) (span (offset 1761) (line 76) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1761) (line 76) (column 10) (len 3)))))
    (reference r35 (scope relative) (span (offset 1775) (line 77) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1775) (line 77) (column 10) (len 3)))))
    (reference r36 (scope relative) (span (offset 1832) (line 80) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1832) (line 80) (column 10) (len 3)))))
    (reference r37 (scope relative) (span (offset 1846) (line 81) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1846) (line 81) (column 10) (len 3)))))
  )
  (root (package (name "MetadataSigilOwningScopes") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (part-def (name "Supplier") (modifiers) (body semicolon)) (part-def (name "Client") (modifiers) (body semicolon)) (part-def (name "InPartDef") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeDef") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r1)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDef") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPartUsage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "beforeUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r4)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (attribute-def (declaration-name "InAttributeDef") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "beforeAttribute") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r7)) (body semicolon)) (attribute-usage (declaration-name "afterAttribute") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "InPortDef") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "beforePort") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r10)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r11)) (body semicolon)) (attribute-usage (declaration-name "afterPort") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (occurrence-def (modifiers)) (action-def (name "InActionDef") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "beforeAction") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r13)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r14)) (body semicolon)) (metadata-keyword-usage (type (ref r15)) (body none)) (dependency (clients (ref r16)) (suppliers (ref r17)) (body semicolon)) (action-usage (keyword action) (name "afterAction") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (state-def (name "InStateDef") (modifiers) (body brace (state-usage (name "beforeState") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r18)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r19)) (body semicolon)) (state-usage (name "afterState") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (requirement-def (name "InRequirementDef") (modifiers) (body brace (attribute-usage) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r20)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r21)) (body semicolon)) (metadata-keyword-usage (type (ref r22)) (body none)) (dependency (clients (ref r23)) (suppliers (ref r24)) (body semicolon)) (attribute-usage))) (use-case-def (name "InUseCaseDef") (modifiers) (body brace (attribute-usage) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r25)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r26)) (body semicolon)) (attribute-usage))) (calc-def (name "InCalcDef") (modifiers) (body brace (attribute-usage (declaration-name "beforeCalc") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r28)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r29)) (body semicolon)) (attribute-usage (declaration-name "afterCalc") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (view-def (name "InViewDef") (short-name none) (modifiers) (specializes none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r31)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r32)) (body semicolon)))) (enum-def (name "InEnumDef") (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r33)) (about) (body semicolon)) (enum-value (extensions) (enum-keyword present) (visibility none) (name "afterEnum") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1695) (line 73) (column 9) (len 15))))) (interface-def (name "InInterfaceDef") (modifiers) (specializes none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r34)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r35)) (body semicolon)))) (connection-def (name "InConnectionDef") (modifiers) (role ordinary) (specializes none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r36)) (about) (body semicolon)) (metadata-keyword-usage (type (ref r37)) (body semicolon)))))))
)
~~~
