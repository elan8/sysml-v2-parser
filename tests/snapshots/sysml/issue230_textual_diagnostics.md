# META
~~~sexpr
(snapshot (type recovery) (description "Spec42 issue 230: SysML 8.2.2.1.2 reserved words require unrestricted declaration names, Import in 8.2.2.5.1 requires an explicit VisibilityIndicator, and CalculationBody in 8.2.2.16 does not terminate its result expression with a semicolon. Reject each offending token while preserving quoted names, explicit imports, valid constraint results, and later declarations."))
~~~
# SOURCE
~~~sysml
package TextualDiagnostics {
    private import ScalarValues::*;
    import ScalarValues::*;
    attribute def PortNumber;
    item def Lifeline;
    attribute port : PortNumber;
    ref from : Lifeline;
    ref to : Lifeline;
    constraint ohmsLaw { 1 == 1; }
    attribute 'port' : PortNumber;
    ref 'from' : Lifeline;
    ref 'to' : Lifeline;
    constraint validResult { 1 == 1 }
    attribute retained : PortNumber;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "issue230_textual_diagnostics.md"
    (diagnostics
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 69) (line 3) (column 5) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 160) (line 6) (column 15) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 187) (line 7) (column 9) (len 4)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "reserved_declaration_name") (severity error) (category parseerror) (span (offset 212) (line 8) (column 9) (len 2)) (message "a reserved keyword cannot be an unquoted declaration name"))
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 258) (line 9) (column 32) (len 1)) (message "a result expression must not end with a semicolon"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TextualDiagnostics {
    private import ScalarValues::*;
    import ScalarValues::*;
    attribute def PortNumber;
    item def Lifeline;
    attribute port : PortNumber;
    ref from : Lifeline;
    ref to : Lifeline;
    constraint ohmsLaw {
        1 == 1;
    }
    attribute 'port' : PortNumber;
    ref 'from' : Lifeline;
    ref 'to' : Lifeline;
    constraint validResult {
        1 == 1
    }
    attribute retained : PortNumber;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 48) (line 2) (column 20) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 48) (line 2) (column 20) (len 12)))))
    (reference r1 (scope relative) (span (offset 194) (line 7) (column 16) (len 8)) (segments (segment 0 (token "Lifeline") (name "Lifeline") (separator none) (span (offset 194) (line 7) (column 16) (len 8)))))
    (reference r2 (scope relative) (span (offset 217) (line 8) (column 14) (len 8)) (segments (segment 0 (token "Lifeline") (name "Lifeline") (separator none) (span (offset 217) (line 8) (column 14) (len 8)))))
    (reference r3 (scope relative) (span (offset 314) (line 11) (column 18) (len 8)) (segments (segment 0 (token "Lifeline") (name "Lifeline") (separator none) (span (offset 314) (line 11) (column 18) (len 8)))))
    (reference r4 (scope relative) (span (offset 339) (line 12) (column 16) (len 8)) (segments (segment 0 (token "Lifeline") (name "Lifeline") (separator none) (span (offset 339) (line 12) (column 16) (len 8)))))
  )
  (root (package (name "TextualDiagnostics") (body brace (import (target (span (span (offset 48) (line 2) (column 20) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 60) (line 2) (column 32) (len 3))) (separator (span (offset 60) (line 2) (column 32) (len 2))) (marker (span (offset 62) (line 2) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (malformed (code "missing_import_visibility") (found "import") (span (offset 69) (line 3) (column 5) (len 23)) (diagnostic-span (span (offset 69) (line 3) (column 5) (len 6)))) (attribute-def (declaration-name "PortNumber") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (item-def (name "Lifeline") (modifiers) (individual false) (specializes none) (body semicolon)) (attribute-usage) (ref (name (invalid-name (reason reserved-keyword) (token "from") (span (offset 187) (line 7) (column 9) (len 4)))) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name (invalid-name (reason reserved-keyword) (token "to") (span (offset 212) (line 8) (column 9) (len 2)))) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "ohmsLaw") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 252) (line 9) (column 26) (len 7)) (diagnostic-span (span (offset 258) (line 9) (column 32) (len 1)))))) (attribute-usage) (ref (name "from") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "to") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validResult") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 378) (line 13) (column 30) (len 6)) (binary (operator "==") (left (expression (span (offset 378) (line 13) (column 30) (len 1)) (integer 1))) (right (expression (span (offset 383) (line 13) (column 35) (len 1)) (integer 1))))))) (attribute-usage))))
)
~~~
