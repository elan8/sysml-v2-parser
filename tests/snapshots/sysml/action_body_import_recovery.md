# META
~~~sexpr
(snapshot (type recovery) (description "Action-body recovery synchronizes at a visibility-prefixed Import after malformed content, then retains the import and following action sibling. ActionBodyItem -> NonBehaviorBodyItem -> Import is in SysML textual BNF 901-917 and pinned Pilot SysML.xtext 1368-1381."))
~~~
# SOURCE
~~~sysml
package ActionBodyImportRecovery {
    action def Owner {
        nonsense ???;
        private import Domain::Tooling::*;
        action retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_import_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 66) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in action body"))
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
    (reference r0 (scope relative) (span (offset 103) (line 4) (column 24) (len 15)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 103) (line 4) (column 24) (len 6))) (segment 1 (token "Tooling") (name "Tooling") (separator colon-colon) (span (offset 111) (line 4) (column 32) (len 7)))))
  )
  (root (package (name "ActionBodyImportRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 66) (line 3) (column 9) (len 22))) (import (target (span (span (offset 103) (line 4) (column 24) (len 18))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 118) (line 4) (column 39) (len 3))) (separator (span (offset 118) (line 4) (column 39) (len 2))) (marker (span (offset 120) (line 4) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage (keyword action) (name "retained") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
