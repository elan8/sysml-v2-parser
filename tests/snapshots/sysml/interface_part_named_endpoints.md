# META
~~~sexpr
(snapshot (type semantic) (description "InterfacePart retains the binary and n-ary alternatives, including source-backed endpoint multiplicities, declared end names, authored `::>`/`references` operators, and dotted OwnedFeatureChain targets. This exercises both declared `connect` and bare InterfacePart forms (SysML textual BNF 763-784; pinned Pilot SysML.xtext 1155-1186)."))
~~~
# SOURCE
~~~sysml
package InterfacePartNamedEndpoints {
    part host {
        interface i : I connect [1] input ::> source.port to output references target.port;
        interface (alpha ::> a.port, second references b.port, c.port);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_part_named_endpoints.md"
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
    (reference r0 (scope relative) (span (offset 100) (line 3) (column 47) (len 11)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 100) (line 3) (column 47) (len 6))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 107) (line 3) (column 54) (len 4)))))
    (reference r1 (scope relative) (span (offset 133) (line 3) (column 80) (len 11)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 133) (line 3) (column 80) (len 6))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 140) (line 3) (column 87) (len 4)))))
    (reference r2 (scope relative) (span (offset 175) (line 4) (column 30) (len 6)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 175) (line 4) (column 30) (len 1))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 177) (line 4) (column 32) (len 4)))))
    (reference r3 (scope relative) (span (offset 201) (line 4) (column 56) (len 6)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 201) (line 4) (column 56) (len 1))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 203) (line 4) (column 58) (len 4)))))
    (reference r4 (scope relative) (span (offset 209) (line 4) (column 64) (len 6)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 209) (line 4) (column 64) (len 1))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 211) (line 4) (column 66) (len 4)))))
  )
  (root (package (name "InterfacePartNamedEndpoints") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity (lower (expression (span (offset 87) (line 3) (column 34) (len 1)) (integer 1))) (upper (expression (span (offset 87) (line 3) (column 34) (len 1)) (integer 1)))) (target (named (name "input") (references symbol) (target (ref r0)))))) (to (interface-end (multiplicity none) (target (named (name "output") (references keyword) (target (ref r1)))))))) (body semicolon)) (interface-usage (form connection) (part (nary (interface-end (multiplicity none) (target (named (name "alpha") (references symbol) (target (ref r2))))) (interface-end (multiplicity none) (target (named (name "second") (references keyword) (target (ref r3))))) (interface-end (multiplicity none) (target (ref r4))))) (body semicolon)))))))
)
~~~
