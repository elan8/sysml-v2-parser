# META
~~~sexpr
(snapshot (type semantic) (description "Generic ref declarations remain members of every usage body whose grammar allows one, and their supported feature-kind keywords survive. The typed `ref action` spelling is deliberately no longer a generic RefDecl: requirement bodies route it to ActionUsage, matching the same production in action, state, and package owners. Port, requirement, and view bodies otherwise project generic ref declarations structurally. The emitted form applies one canonical clause order -- typing before the subsetting family, multiplicity before it -- so `ref requirement :>> self : RequirementCheck;` comes back as `ref requirement : RequirementCheck :>> self;`; that is emission policy, and the AST section shows nothing was lost."))
~~~
# SOURCE
~~~sysml
package RefDeclarationScopes {
    port def Port {
        ref self : Port :>> Object::self;
        abstract ref port interfacingPorts : Port[0..*] nonunique :> ports;
        ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf;
    }
    requirement def RequirementCheck {
        ref requirement :>> self : RequirementCheck;
        ref part actors : Part[0..*];
        ref item everyItem : T;
        ref use case everyUseCase : T;
        ref case everyCase : T;
        ref verification everyVerification : T;
        ref concern everyConcern : T;
        ref viewpoint everyViewpoint : T;
        ref rendering everyRendering : T;
        ref view everyView : T;
        ref action everyAction : T;
    }
    view columnView[0..*] ordered {
        abstract ref rendering :>> viewRendering[0..1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_declaration_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RefDeclarationScopes {
    port def Port {
        ref self : Port :>> Object::self;
        abstract ref port interfacingPorts : Port[0..*] nonunique :> ports;
        ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf;
    }
    requirement def RequirementCheck {
        ref requirement : RequirementCheck :>> self;
        ref part actors : Part[0..*];
        ref item everyItem : T;
        ref use case everyUseCase : T;
        ref case everyCase : T;
        ref verification everyVerification : T;
        ref concern everyConcern : T;
        ref viewpoint everyViewpoint : T;
        ref rendering everyRendering : T;
        ref view everyView : T;
        ref action everyAction : T;
    }
    view columnView[0..*] ordered {
        abstract ref rendering [0..1] :>> viewRendering;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 70) (line 3) (column 20) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 70) (line 3) (column 20) (len 4)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 29) (len 12)) (segments (segment 0 (token "Object") (name "Object") (separator none) (span (offset 79) (line 3) (column 29) (len 6))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 87) (line 3) (column 37) (len 4)))))
    (reference r2 (scope relative) (span (offset 138) (line 4) (column 46) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 138) (line 4) (column 46) (len 4)))))
    (reference r3 (scope relative) (span (offset 162) (line 4) (column 70) (len 5)) (segments (segment 0 (token "ports") (name "ports") (separator none) (span (offset 162) (line 4) (column 70) (len 5)))))
    (reference r4 (scope relative) (span (offset 185) (line 5) (column 17) (len 25)) (segments (segment 0 (token "outgoingTransfersFromSelf") (name "outgoingTransfersFromSelf") (separator none) (span (offset 185) (line 5) (column 17) (len 25)))))
    (reference r5 (scope relative) (span (offset 214) (line 5) (column 46) (len 40)) (segments (segment 0 (token "interfacingPorts") (name "interfacingPorts") (separator none) (span (offset 214) (line 5) (column 46) (len 16))) (segment 1 (token "incomingTransfersToSelf") (name "incomingTransfersToSelf") (separator dot) (span (offset 231) (line 5) (column 63) (len 23)))))
    (reference r6 (scope relative) (span (offset 336) (line 8) (column 36) (len 16)) (segments (segment 0 (token "RequirementCheck") (name "RequirementCheck") (separator none) (span (offset 336) (line 8) (column 36) (len 16)))))
    (reference r7 (scope relative) (span (offset 329) (line 8) (column 29) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 329) (line 8) (column 29) (len 4)))))
    (reference r8 (scope relative) (span (offset 380) (line 9) (column 27) (len 4)) (segments (segment 0 (token "Part") (name "Part") (separator none) (span (offset 380) (line 9) (column 27) (len 4)))))
    (reference r9 (scope relative) (span (offset 421) (line 10) (column 30) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 421) (line 10) (column 30) (len 1)))))
    (reference r10 (scope relative) (span (offset 460) (line 11) (column 37) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 460) (line 11) (column 37) (len 1)))))
    (reference r11 (scope relative) (span (offset 492) (line 12) (column 30) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 492) (line 12) (column 30) (len 1)))))
    (reference r12 (scope relative) (span (offset 540) (line 13) (column 46) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 540) (line 13) (column 46) (len 1)))))
    (reference r13 (scope relative) (span (offset 578) (line 14) (column 36) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 578) (line 14) (column 36) (len 1)))))
    (reference r14 (scope relative) (span (offset 620) (line 15) (column 40) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 620) (line 15) (column 40) (len 1)))))
    (reference r15 (scope relative) (span (offset 662) (line 16) (column 40) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 662) (line 16) (column 40) (len 1)))))
    (reference r16 (scope relative) (span (offset 694) (line 17) (column 30) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 694) (line 17) (column 30) (len 1)))))
    (reference r17 (scope relative) (span (offset 730) (line 18) (column 34) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 730) (line 18) (column 34) (len 1)))))
    (reference r18 (scope relative) (span (offset 810) (line 21) (column 36) (len 13)) (segments (segment 0 (token "viewRendering") (name "viewRendering") (separator none) (span (offset 810) (line 21) (column 36) (len 13)))))
  )
  (root (package (name "RefDeclarationScopes") (body brace (port-def (name "Port") (modifiers) (specializes none) (body brace (ref (name "self") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "interfacingPorts") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 143) (line 4) (column 51) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (body semicolon)))) (requirement-def (name "RequirementCheck") (modifiers) (body brace (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind requirement) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (subsets none) (body semicolon)) (ref (name "actors") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind part) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 385) (line 9) (column 32) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyItem") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind item) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyUseCase") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind use case) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyCase") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind case) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyVerification") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind verification) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyConcern") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind concern) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyViewpoint") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind viewpoint) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyRendering") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind rendering) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "everyView") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind view) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (action-usage (name "everyAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (view (name "columnView") (short-name none) (type none) (body brace (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (extensions) (kind rendering) (typing none) (multiplicity (lower (expression (span (offset 824) (line 21) (column 50) (len 1)) (integer 0))) (upper (expression (span (offset 827) (line 21) (column 53) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (subsets none) (body semicolon)))))))
)
~~~
