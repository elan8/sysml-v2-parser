# META
~~~sexpr
(snapshot (type semantic) (description "ActionDefinition and ActionUsage both own ActionBody, whose NonBehaviorBodyItem alternative admits an Import before behavior/action members (SysML textual BNF 894-917 and 937-939; pinned Pilot SysML.xtext 1361-1381 and 1407-1408). Imports with their mandatory visibility prefix retain their typed namespace or membership targets, then leave a following action member intact through FORMAT reparse/idempotence."))
~~~
# SOURCE
~~~sysml
package ActionBodyImports {
    action def DefinitionOwner {
        private import Domain::Units::*;
        private import Domain::Tooling;
        action afterDefinition;
    }
    action usageOwner {
        private import Domain::Actions::*;
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 84) (line 3) (column 24) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 84) (line 3) (column 24) (len 6))) (segment 1 (token "Units") (name "Units") (separator colon-colon) (span (offset 92) (line 3) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 125) (line 4) (column 24) (len 15)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 125) (line 4) (column 24) (len 6))) (segment 1 (token "Tooling") (name "Tooling") (separator colon-colon) (span (offset 133) (line 4) (column 32) (len 7)))))
    (reference r2 (scope relative) (span (offset 227) (line 8) (column 24) (len 15)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 227) (line 8) (column 24) (len 6))) (segment 1 (token "Actions") (name "Actions") (separator colon-colon) (span (offset 235) (line 8) (column 32) (len 7)))))
    (reference r3 (scope relative) (span (offset 269) (line 9) (column 23) (len 17)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 269) (line 9) (column 23) (len 6))) (segment 1 (token "PublicApi") (name "PublicApi") (separator colon-colon) (span (offset 277) (line 9) (column 31) (len 9)))))
    (reference r4 (scope relative) (span (offset 316) (line 10) (column 26) (len 17)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 316) (line 10) (column 26) (len 6))) (segment 1 (token "Utilities") (name "Utilities") (separator colon-colon) (span (offset 324) (line 10) (column 34) (len 9)))))
  )
  (root (package (name "ActionBodyImports") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (import (visibility private) (target (span (span (offset 84) (line 3) (column 24) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 97) (line 3) (column 37) (len 3))) (separator (span (offset 97) (line 3) (column 37) (len 2))) (marker (span (offset 99) (line 3) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 125) (line 4) (column 24) (len 15))) (all none) (ref r1) (shape (membership (recursive-suffix none)))) (body semicolon)) (action-usage (keyword action) (name "afterDefinition") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (action-usage (keyword action) (name "usageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (import (visibility private) (target (span (span (offset 227) (line 8) (column 24) (len 18))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 242) (line 8) (column 39) (len 3))) (separator (span (offset 242) (line 8) (column 39) (len 2))) (marker (span (offset 244) (line 8) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility public) (target (span (span (offset 269) (line 9) (column 23) (len 20))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 286) (line 9) (column 40) (len 3))) (separator (span (offset 286) (line 9) (column 40) (len 2))) (marker (span (offset 288) (line 9) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility protected) (target (span (span (offset 316) (line 10) (column 26) (len 17))) (all none) (ref r4) (shape (membership (recursive-suffix none)))) (body semicolon)) (action-usage (keyword action) (name "afterUsage") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
