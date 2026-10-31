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
        objective  : MaximizeObjective;
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 41) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 55) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 77) (line 3) (column 17) (len 12)) (segments (segment 0 (token "TradeStudies") (name "TradeStudies") (separator none) (span (offset 77) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 129) (line 6) (column 16) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 129) (line 6) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 152) (line 7) (column 16) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 152) (line 7) (column 16) (len 6)))))
    (reference r4 (scope relative) (span (offset 191) (line 9) (column 30) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 191) (line 9) (column 30) (len 10)))))
    (reference r5 (scope relative) (span (offset 216) (line 10) (column 13) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 216) (line 10) (column 13) (len 6)))))
    (reference r6 (scope relative) (span (offset 232) (line 10) (column 29) (len 7)) (segments (segment 0 (token "engine1") (name "engine1") (separator none) (span (offset 232) (line 10) (column 29) (len 7)))))
    (reference r7 (scope relative) (span (offset 241) (line 10) (column 38) (len 7)) (segments (segment 0 (token "engine2") (name "engine2") (separator none) (span (offset 241) (line 10) (column 38) (len 7)))))
    (reference r8 (scope relative) (span (offset 265) (line 11) (column 15) (len 17)) (segments (segment 0 (token "MaximizeObjective") (name "MaximizeObjective") (separator none) (span (offset 265) (line 11) (column 15) (len 17)))))
    (reference r9 (scope relative) (span (offset 296) (line 13) (column 12) (len 18)) (segments (segment 0 (token "evaluationFunction") (name "evaluationFunction") (separator none) (span (offset 296) (line 13) (column 12) (len 18)))))
    (reference r10 (scope relative) (span (offset 330) (line 14) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 330) (line 14) (column 14) (len 6)))))
    (reference r11 (scope relative) (span (offset 379) (line 18) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 379) (line 18) (column 17) (len 6)))))
  )
  (root (package (name "TradeStudyTest") (body brace (import (visibility private) (target (span (span (offset 41) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 77) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 29) (len 3))) (separator (span (offset 89) (line 3) (column 29) (len 2))) (marker (span (offset 91) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeStudy") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (body brace (subject (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 223) (line 10) (column 20) (len 1)) (integer 1))) (upper (expression (span (offset 226) (line 10) (column 23) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 231) (line 10) (column 28) (len 18)) (sequence (sequence-list (element first (expression (span (offset 232) (line 10) (column 29) (len 7)) (ref r6))) (element comma (expression (span (offset 241) (line 10) (column 38) (len 7)) (ref r7))))))))) (body semicolon)) (objective (visibility none) (name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (body semicolon)) (calc-usage (name none) (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (ref r9)) (value none) (body brace (part-usage (then false) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))) (case-return (declaration none) (target none) (type (ref r11)) (redefines none) (feature-kind part) (subsetting false) (value none)))))))
)
~~~
