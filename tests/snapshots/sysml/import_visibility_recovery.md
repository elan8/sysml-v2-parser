# META
~~~sexpr
(snapshot (type recovery) (description "Import requires an explicit VisibilityIndicator in both pinned SysML and KerML grammars. Bare imports at root and nested in package, type and action bodies produce missing_import_visibility exactly on the import keyword, retain the malformed statement, allocate no target references, and preserve later siblings and incomplete root relationship bodies. Explicit public/private/protected visibility and note-separated tokens remain valid parser controls; top-level visibility semantics belong to the compiler."))
~~~
# SOURCE
~~~sysml
import RootMissing::*;
private import RootPresent::*;
package ImportVisibility {
    import Missing::*;
    private import Present::*;
    public/* before import */import/* before target */Visible::*;
    protected import Protected::*;
    action def A {
        import ActionMissing::*;
        private import ActionPresent::*;
        action retained;
    }
    class T {
        import TypeMissing::*;
        private import TypePresent::*;
        feature retained;
    }
    import BlockMissing { doc /* retain ; and } inside this comment */ }
    attribute retained = 1;
}
import Unterminated { doc /* Retain this incomplete relationship body. */
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_visibility_recovery.md"
    (diagnostics
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 85) (line 4) (column 5) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 263) (line 9) (column 9) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 382) (line 14) (column 9) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 480) (line 18) (column 5) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_import_visibility") (severity error) (category parseerror) (span (offset 579) (line 21) (column 1) (len 6)) (message "an import requires an explicit visibility indicator"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 652) (line 21) (column 74) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
import RootMissing::*;

private import RootPresent::*;

package ImportVisibility {
    import Missing::*;
    private import Present::*;
    public import Visible::*;
    protected import Protected::*;
    action def A {
        import ActionMissing::*;
        private import ActionPresent::*;
        action retained;
    }
    class T {
        import TypeMissing::*;
        private import TypePresent::*;
        feature retained;
    }
    import BlockMissing { doc /* retain ; and } inside this comment */ }
    attribute retained = 1;
}

import Unterminated { doc /* Retain this incomplete relationship body. */
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 16) (len 11)) (segments (segment 0 (token "RootPresent") (name "RootPresent") (separator none) (span (offset 38) (line 2) (column 16) (len 11)))))
    (reference r1 (scope relative) (span (offset 123) (line 5) (column 20) (len 7)) (segments (segment 0 (token "Present") (name "Present") (separator none) (span (offset 123) (line 5) (column 20) (len 7)))))
    (reference r2 (scope relative) (span (offset 189) (line 6) (column 55) (len 7)) (segments (segment 0 (token "Visible") (name "Visible") (separator none) (span (offset 189) (line 6) (column 55) (len 7)))))
    (reference r3 (scope relative) (span (offset 222) (line 7) (column 22) (len 9)) (segments (segment 0 (token "Protected") (name "Protected") (separator none) (span (offset 222) (line 7) (column 22) (len 9)))))
    (reference r4 (scope relative) (span (offset 311) (line 10) (column 24) (len 13)) (segments (segment 0 (token "ActionPresent") (name "ActionPresent") (separator none) (span (offset 311) (line 10) (column 24) (len 13)))))
  )
  (root (malformed (code "missing_import_visibility") (found "import") (span (offset 0) (line 1) (column 1) (len 22)) (diagnostic-span (span (offset 0) (line 1) (column 1) (len 6)))) (import (target (span (span (offset 38) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 49) (line 2) (column 27) (len 3))) (separator (span (offset 49) (line 2) (column 27) (len 2))) (marker (span (offset 51) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "ImportVisibility") (body brace (malformed (code "missing_import_visibility") (found "import") (span (offset 85) (line 4) (column 5) (len 18)) (diagnostic-span (span (offset 85) (line 4) (column 5) (len 6)))) (import (target (span (span (offset 123) (line 5) (column 20) (len 10))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 130) (line 5) (column 27) (len 3))) (separator (span (offset 130) (line 5) (column 27) (len 2))) (marker (span (offset 132) (line 5) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 189) (line 6) (column 55) (len 10))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 196) (line 6) (column 62) (len 3))) (separator (span (offset 196) (line 6) (column 62) (len 2))) (marker (span (offset 198) (line 6) (column 64) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 222) (line 7) (column 22) (len 12))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 231) (line 7) (column 31) (len 3))) (separator (span (offset 231) (line 7) (column 31) (len 2))) (marker (span (offset 233) (line 7) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-def (name "A") (modifiers) (specializes none) (body brace (malformed (code "missing_import_visibility") (found "import") (span (offset 263) (line 9) (column 9) (len 33)) (diagnostic-span (span (offset 263) (line 9) (column 9) (len 6)))) (import (target (span (span (offset 311) (line 10) (column 24) (len 16))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 324) (line 10) (column 37) (len 3))) (separator (span (offset 324) (line 10) (column 37) (len 2))) (marker (span (offset 326) (line 10) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage (keyword action) (name "retained") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (kerml-classifier (keyword class) (abstract false) (name "T") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "missing_import_visibility") (found "import") (span (offset 382) (line 14) (column 9) (len 31)) (diagnostic-span (span (offset 382) (line 14) (column 9) (len 6)))) (import) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "retained") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (malformed (code "missing_import_visibility") (found "import") (span (offset 480) (line 18) (column 5) (len 68)) (diagnostic-span (span (offset 480) (line 18) (column 5) (len 6)))) (attribute-usage))) (malformed (code "missing_import_visibility") (found "import") (span (offset 579) (line 21) (column 1) (len 73)) (diagnostic-span (span (offset 579) (line 21) (column 1) (len 6)))))
)
~~~
