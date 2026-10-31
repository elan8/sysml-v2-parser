# META
~~~sexpr
(snapshot (type malformed) (description "SysML 8.2.2.1.2 excludes reserved words from BASIC_NAME. Preserve malformed declaration and short names with exact token diagnostics, quoted and case-distinct controls, named connector ends, and later siblings.") (name-language sysml))
~~~
# SOURCE
~~~sysml
package ReservedDeclarationNames {
    attribute port;
    ref from;
    ref to;
    part def port;
    item <from> validItem;
    action to;
    port <to> validPort;
    private alias port for Domain::Target;
    attribute 'port';
    ref 'from';
    ref 'to';
    part def Port;
    item <'from'> quotedShort;
    private alias 'port' for Domain::Target;
    connection invalidNamedEnd connect from references left to to references right;
    connection named connect 'from' references left to 'to' references right;
    attribute retained;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "reserved_declaration_names.md"
    (diagnostics
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 49) (line 2) (column 15) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 63) (line 3) (column 9) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 77) (line 4) (column 9) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 94) (line 5) (column 14) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 110) (line 6) (column 11) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 138) (line 7) (column 12) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 152) (line 8) (column 11) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 185) (line 9) (column 19) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 396) (line 16) (column 40) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 420) (line 16) (column 64) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 194) (line 9) (column 28) (len 14)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 194) (line 9) (column 28) (len 6))) (segment 1 (token "Target") (name "Target") (separator colon-colon) (span (offset 202) (line 9) (column 36) (len 6)))))
    (reference r1 (scope relative) (span (offset 341) (line 15) (column 30) (len 14)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 341) (line 15) (column 30) (len 6))) (segment 1 (token "Target") (name "Target") (separator colon-colon) (span (offset 349) (line 15) (column 38) (len 6)))))
    (reference r2 (scope relative) (span (offset 412) (line 16) (column 56) (len 4)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 412) (line 16) (column 56) (len 4)))))
    (reference r3 (scope relative) (span (offset 434) (line 16) (column 78) (len 5)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 434) (line 16) (column 78) (len 5)))))
    (reference r4 (scope relative) (span (offset 488) (line 17) (column 48) (len 4)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 488) (line 17) (column 48) (len 4)))))
    (reference r5 (scope relative) (span (offset 512) (line 17) (column 72) (len 5)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 512) (line 17) (column 72) (len 5)))))
  )
  (root (package (visibility none) (name "ReservedDeclarationNames") (body brace (attribute-usage) (ref (name (invalid-name (reason reserved-keyword) (token "from") (span (offset 63) (line 3) (column 9) (len 4)))) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name (invalid-name (reason reserved-keyword) (token "to") (span (offset 77) (line 4) (column 9) (len 2)))) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (part-def (name (invalid-name (reason reserved-keyword) (token "port") (span (offset 94) (line 5) (column 14) (len 4)))) (modifiers) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validItem") (short-name (invalid-name (reason reserved-keyword) (token "from") (span (offset 110) (line 6) (column 11) (len 4)))) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage (keyword action) (name (invalid-name (reason reserved-keyword) (token "to") (span (offset 138) (line 7) (column 12) (len 2)))) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validPort") (short-name (invalid-name (reason reserved-keyword) (token "to") (span (offset 152) (line 8) (column 11) (len 2)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (alias (name (invalid-name (reason reserved-keyword) (token "port") (span (offset 185) (line 9) (column 19) (len 4)))) (target (ref r0)) (body semicolon)) (attribute-usage) (ref (name "from") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "to") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (part-def (name "Port") (modifiers) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "quotedShort") (short-name "from") (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (alias (name "port") (target (ref r1)) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "invalidNamedEnd") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect (end-span (span (offset 396) (line 16) (column 40) (len 20))) (declared-name (invalid-name (reason reserved-keyword) (token "from") (span (offset 396) (line 16) (column 40) (len 4)))) (operator references) (expression (span (offset 412) (line 16) (column 56) (len 4)) (ref r2)) (end-span (span (offset 420) (line 16) (column 64) (len 19))) (declared-name (invalid-name (reason reserved-keyword) (token "to") (span (offset 420) (line 16) (column 64) (len 2)))) (operator references) (expression (span (offset 434) (line 16) (column 78) (len 5)) (ref r3))) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "named") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect (end-span (span (offset 470) (line 17) (column 30) (len 22))) (declared-name "from") (operator references) (expression (span (offset 488) (line 17) (column 48) (len 4)) (ref r4)) (end-span (span (offset 496) (line 17) (column 56) (len 21))) (declared-name "to") (operator references) (expression (span (offset 512) (line 17) (column 72) (len 5)) (ref r5))) (body semicolon)) (attribute-usage))))
)
~~~
