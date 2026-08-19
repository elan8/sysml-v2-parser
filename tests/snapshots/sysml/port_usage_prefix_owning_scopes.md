# META
~~~sexpr
(snapshot (type semantic) (description "A prefixed port usage in every scope this parser dispatches one from (planning/port-usage-prefix-matrix.md §3): package/namespace/root, a part definition body, a part usage body, a port definition body, a port usage body, an interface definition body, a connection definition body, a requirement definition body, and a `variant` member. Each scope carries a materially different prefix so a scope that lost a slot shows up as a different projection, and each also repeats one identical declaration -- `ref individual port shared : PowerPort;` -- which must project byte-identically everywhere: the prefix is a property of the production, not of the scope that owns it. `port def` sits beside the usages in the scopes that admit both, so the disambiguation is visible."))
~~~
# SOURCE
~~~sysml
package PortPrefixOwningScopes {
    metadata def Tag;
    port def PowerPort;
    part def Engine;
    ref individual port shared : PowerPort;
    in port packageScope : PowerPort;
    port def PortDefScope {
        ref individual port shared : PowerPort;
        snapshot port inPortDef;
    }
    part def PartDefScope {
        ref individual port shared : PowerPort;
        #Tag port inPartDef : PowerPort;
        port def NestedPortDef;
    }
    part partUsageScope : Engine {
        ref individual port shared : PowerPort;
        timeslice port inPartUsage;
    }
    port portUsageScope : PowerPort {
        ref individual port shared : PowerPort;
        derived port inPortUsage : PowerPort;
    }
    interface def InterfaceDefScope {
        ref individual port shared : PowerPort;
        abstract port inInterfaceDef : PowerPort;
    }
    connection def ConnectionDefScope {
        ref individual port shared : PowerPort;
        constant port inConnectionDef : PowerPort;
    }
    requirement def RequirementDefScope {
        ref individual port shared : PowerPort;
        out port inRequirementDef : PowerPort;
    }
    part def VariantScope {
        variation part variationPoint : Engine {
            variant port firstAlternative : PowerPort;
            variant ref individual port shared : PowerPort;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_usage_prefix_owning_scopes.md"
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
    (reference r0 (scope relative) (span (offset 133) (line 5) (column 34) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 133) (line 5) (column 34) (len 9)))))
    (reference r1 (scope relative) (span (offset 171) (line 6) (column 28) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 171) (line 6) (column 28) (len 9)))))
    (reference r2 (scope relative) (span (offset 247) (line 8) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 247) (line 8) (column 38) (len 9)))))
    (reference r3 (scope relative) (span (offset 362) (line 12) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 362) (line 12) (column 38) (len 9)))))
    (reference r4 (scope relative) (span (offset 382) (line 13) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 382) (line 13) (column 10) (len 3)))))
    (reference r5 (scope relative) (span (offset 403) (line 13) (column 31) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 403) (line 13) (column 31) (len 9)))))
    (reference r6 (scope relative) (span (offset 478) (line 16) (column 27) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 478) (line 16) (column 27) (len 6)))))
    (reference r7 (scope relative) (span (offset 524) (line 17) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 524) (line 17) (column 38) (len 9)))))
    (reference r8 (scope relative) (span (offset 603) (line 20) (column 27) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 603) (line 20) (column 27) (len 9)))))
    (reference r9 (scope relative) (span (offset 652) (line 21) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 652) (line 21) (column 38) (len 9)))))
    (reference r10 (scope relative) (span (offset 698) (line 22) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 698) (line 22) (column 36) (len 9)))))
    (reference r11 (scope relative) (span (offset 790) (line 25) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 790) (line 25) (column 38) (len 9)))))
    (reference r12 (scope relative) (span (offset 840) (line 26) (column 40) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 840) (line 26) (column 40) (len 9)))))
    (reference r13 (scope relative) (span (offset 934) (line 29) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 934) (line 29) (column 38) (len 9)))))
    (reference r14 (scope relative) (span (offset 985) (line 30) (column 41) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 985) (line 30) (column 41) (len 9)))))
    (reference r15 (scope relative) (span (offset 1081) (line 33) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1081) (line 33) (column 38) (len 9)))))
    (reference r16 (scope relative) (span (offset 1128) (line 34) (column 37) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1128) (line 34) (column 37) (len 9)))))
    (reference r17 (scope relative) (span (offset 1213) (line 37) (column 41) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1213) (line 37) (column 41) (len 6)))))
    (reference r18 (scope relative) (span (offset 1266) (line 38) (column 45) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1266) (line 38) (column 45) (len 9)))))
    (reference r19 (scope relative) (span (offset 1326) (line 39) (column 50) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1326) (line 39) (column 50) (len 9)))))
  )
  (root (package (name "PortPrefixOwningScopes") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (port-def (name "PowerPort") (specializes none) (body semicolon)) (part-def (name "Engine") (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "packageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-def (name "PortDefScope") (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "inPortDef") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "PartDefScope") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r4))) (declaration-name "inPartDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-def (name "NestedPortDef") (specializes none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "partUsageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration-name "inPartUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "portUsageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPortUsage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-def (name "InterfaceDefScope") (modifiers) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inInterfaceDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "ConnectionDefScope") (modifiers) (role ordinary) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inConnectionDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (requirement-def (name "RequirementDefScope") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inRequirementDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VariantScope") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "variationPoint") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "firstAlternative") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "shared") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))))))))
)
~~~
