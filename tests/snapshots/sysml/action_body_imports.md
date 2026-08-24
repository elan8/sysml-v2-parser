# META
~~~sexpr
(snapshot (type semantic) (description "ActionDefinition and ActionUsage both own ActionBody, whose NonBehaviorBodyItem alternative admits an Import before behavior/action members (SysML textual BNF 894-917 and 937-939; pinned Pilot SysML.xtext 1361-1381 and 1407-1408). Bare and visibility-prefixed imports retain their typed namespace or membership targets, then leave a following action member intact through FORMAT reparse/idempotence."))
~~~
# SOURCE
~~~sysml
package ActionBodyImports {
    action def DefinitionOwner {
        import Domain::Units::*;
        private import Domain::Tooling;
        action afterDefinition;
    }
    action usageOwner {
        import Domain::Actions::*;
        public import Domain::PublicApi::*;
        protected import Domain::Utilities;
        action afterUsage;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_imports.md"
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
    (reference r0 (scope relative) (span (offset 76) (line 3) (column 16) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 76) (line 3) (column 16) (len 6))) (segment 1 (token "Units") (name "Units") (separator colon-colon) (span (offset 84) (line 3) (column 24) (len 5)))))
    (reference r1 (scope relative) (span (offset 117) (line 4) (column 24) (len 15)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 117) (line 4) (column 24) (len 6))) (segment 1 (token "Tooling") (name "Tooling") (separator colon-colon) (span (offset 125) (line 4) (column 32) (len 7)))))
    (reference r2 (scope relative) (span (offset 211) (line 8) (column 16) (len 15)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 211) (line 8) (column 16) (len 6))) (segment 1 (token "Actions") (name "Actions") (separator colon-colon) (span (offset 219) (line 8) (column 24) (len 7)))))
    (reference r3 (scope relative) (span (offset 253) (line 9) (column 23) (len 17)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 253) (line 9) (column 23) (len 6))) (segment 1 (token "PublicApi") (name "PublicApi") (separator colon-colon) (span (offset 261) (line 9) (column 31) (len 9)))))
    (reference r4 (scope relative) (span (offset 300) (line 10) (column 26) (len 17)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 300) (line 10) (column 26) (len 6))) (segment 1 (token "Utilities") (name "Utilities") (separator colon-colon) (span (offset 308) (line 10) (column 34) (len 9)))))
  )
  (root (package (name "ActionBodyImports") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (import (target (span (span (offset 76) (line 3) (column 16) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 29) (len 3))) (separator (span (offset 89) (line 3) (column 29) (len 2))) (marker (span (offset 91) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 117) (line 4) (column 24) (len 15))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (action-usage (keyword action) (name "afterDefinition") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (action-usage (keyword action) (name "usageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (import (target (span (span (offset 211) (line 8) (column 16) (len 18))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 226) (line 8) (column 31) (len 3))) (separator (span (offset 226) (line 8) (column 31) (len 2))) (marker (span (offset 228) (line 8) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 253) (line 9) (column 23) (len 20))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 270) (line 9) (column 40) (len 3))) (separator (span (offset 270) (line 9) (column 40) (len 2))) (marker (span (offset 272) (line 9) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 300) (line 10) (column 26) (len 17))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (action-usage (keyword action) (name "afterUsage") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
