# META
~~~sexpr
(snapshot (type recovery) (description "Recovery inside the four brace scopes that own a port usage and resynchronize on their own starter table: an interface definition body, a port definition body, a port usage body and a connection definition body. The malformed run is deliberately unterminated, so a scope that scans to the next `;` instead of to the next member starter swallows the declaration after it; each case is followed by a visibility-prefixed port usage and a second sibling, because `MemberPrefix` precedes the occurrence prefix and a table missing `public`/`private`/`protected` loses the member even when every prefix keyword is listed. The malformed span must cover only the authored garbage and every following sibling must survive, in every one of the four scopes."))
~~~
# SOURCE
~~~sysml
package PortPrefixScopeRecovery {
    port def PowerPort;
    interface def InterfaceScope {
        %%%
        private port hiddenAfterMalformed : PowerPort;
        %%%
        ref port referenceAfterMalformed : PowerPort;
        port plainAfterMalformed : PowerPort;
    }
    port def PortDefScope {
        %%%
        protected port protectedAfterMalformed : PowerPort;
        %%%
        snapshot port portionAfterMalformed;
        port plainAfterMalformed : PowerPort;
    }
    port portUsageScope : PowerPort {
        %%%
        public port exposedAfterMalformed : PowerPort;
        %%%
        #PowerPort port taggedAfterMalformed : PowerPort;
        port plainAfterMalformed : PowerPort;
    }
    connection def ConnectionScope {
        %%%
        private port hiddenAfterMalformed : PowerPort;
        %%%
        individual port individualAfterMalformed : PowerPort;
        port plainAfterMalformed : PowerPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_usage_prefix_scope_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_interface_def_body_element") (severity error) (category parseerror) (span (offset 101) (line 4) (column 9) (len 12)) (message "unexpected token in interface definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 101) (line 4) (column 9) (len 12)) (message "suppressed 7 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 149) (line 5) (column 45) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 149) (line 5) (column 45) (len 9)))))
    (reference r1 (scope relative) (span (offset 215) (line 7) (column 44) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 215) (line 7) (column 44) (len 9)))))
    (reference r2 (scope relative) (span (offset 261) (line 8) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 261) (line 8) (column 36) (len 9)))))
    (reference r3 (scope relative) (span (offset 367) (line 12) (column 50) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 367) (line 12) (column 50) (len 9)))))
    (reference r4 (scope relative) (span (offset 470) (line 15) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 470) (line 15) (column 36) (len 9)))))
    (reference r5 (scope relative) (span (offset 513) (line 17) (column 27) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 513) (line 17) (column 27) (len 9)))))
    (reference r6 (scope relative) (span (offset 581) (line 19) (column 45) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 581) (line 19) (column 45) (len 9)))))
    (reference r7 (scope relative) (span (offset 613) (line 21) (column 10) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 613) (line 21) (column 10) (len 9)))))
    (reference r8 (scope relative) (span (offset 651) (line 21) (column 48) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 651) (line 21) (column 48) (len 9)))))
    (reference r9 (scope relative) (span (offset 697) (line 22) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 697) (line 22) (column 36) (len 9)))))
    (reference r10 (scope relative) (span (offset 807) (line 26) (column 45) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 807) (line 26) (column 45) (len 9)))))
    (reference r11 (scope relative) (span (offset 881) (line 28) (column 52) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 881) (line 28) (column 52) (len 9)))))
    (reference r12 (scope relative) (span (offset 927) (line 29) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 927) (line 29) (column 36) (len 9)))))
  )
  (root (package (name "PortPrefixScopeRecovery") (body brace (port-def (name "PowerPort") (specializes none) (body semicolon)) (interface-def (name "InterfaceScope") (modifiers) (specializes none) (body brace (malformed (code "recovered_interface_def_body_element") (found "%%%") (span (offset 101) (line 4) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hiddenAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_interface_def_body_element") (found "%%%") (span (offset 168) (line 6) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "referenceAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "plainAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "PortDefScope") (specializes none) (body brace (malformed (code "recovered_port_def_body_element") (found "%%%") (span (offset 314) (line 11) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "protectedAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_port_def_body_element") (found "%%%") (span (offset 386) (line 13) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "portionAfterMalformed") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "plainAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "portUsageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "recovered_port_body_element") (found "%%%") (span (offset 533) (line 18) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "exposedAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_port_body_element") (found "%%%") (span (offset 600) (line 20) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "taggedAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "plainAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "ConnectionScope") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "%%%") (span (offset 759) (line 25) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hiddenAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_connection_def_body_element") (found "%%%") (span (offset 826) (line 27) (column 9) (len 12))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "individualAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "plainAfterMalformed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
