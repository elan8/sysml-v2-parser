# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): TradeStudyTest"))
~~~
# SOURCE
~~~sysml
package TradeStudyTest {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	
	part def Engine;
	part engine1: Engine;
	part engine2: Engine;
	
	analysis engineTradeStudy : TradeStudy {
		subject : Engine[1..*] = (engine1, engine2);
		objective : MaximizeObjective;

		calc :>> evaluationFunction {
			in part : Engine;
			return : Real;
		}
		
		return part : Engine;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "trade_study_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package TradeStudyTest {
    private import ScalarValues::Real;
    private import TradeStudies::*;
    part def Engine;
    part engine1 : Engine;
    part engine2 : Engine;
    analysis engineTradeStudy : TradeStudy {
        subject : Engine[1..*] = (engine1, engine2);
        objective : MaximizeObjective ;
        calc :>> evaluationFunction {
            in part : Engine;
            return : Real;
        }
        return part : Engine;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 41) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 55) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 77) (line 3) (column 17) (len 12)) (segments (segment 0 (token "TradeStudies") (name "TradeStudies") (separator none) (span (offset 77) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 129) (line 6) (column 16) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 129) (line 6) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 152) (line 7) (column 16) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 152) (line 7) (column 16) (len 6)))))
    (reference r4 (scope relative) (span (offset 191) (line 9) (column 30) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 191) (line 9) (column 30) (len 10)))))
  )
  (root (package (name "TradeStudyTest") (body brace (import (target (span (span (offset 41) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 77) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 29) (len 3))) (separator (span (offset 89) (line 3) (column 29) (len 2))) (marker (span (offset 91) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeStudy") (type (ref r4)) (subsets none) (redefines none)))))
)
~~~
