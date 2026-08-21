# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 33 (Analysis): Trade Study Analysis Example"))
~~~
# SOURCE
~~~sysml
package 'Trade Study Analysis Example' {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	
	part def Engine;
	part engine4cyl : Engine;
	part engine6cyl : Engine;
	
	calc def PowerRollup { in engine : Engine; return : ISQ::PowerValue; }
	calc def MassRollup { in engine : Engine; return : ISQ::MassValue; }
	calc def EfficiencyRollup { in engine : Engine; return : Real; }
	calc def CostRollup { in engine : Engine; return : Real; }
	
	calc def EngineEvaluation { 
		in power : ISQ::PowerValue;
		in mass : ISQ::MassValue;
		in efficiency : Real;
		in cost : Real;
		return evaluation : Real;
		// Compute evaluation...
	}
		
	analysis engineTradeStudy : TradeStudy {
		subject : Engine = (engine4cyl, engine6cyl);
		objective : MaximizeObjective;

		calc :>> evaluationFunction {
			in part anEngine :>> alternative : Engine;
			
			calc powerRollup: PowerRollup { in engine = anEngine; return power; }
			calc massRollup: MassRollup { in engine = anEngine; return mass; }
			calc efficiencyRollup: EfficiencyRollup { in engine = anEngine; return efficiency; }
			calc costRollup: CostRollup { in engine = anEngine; return cost; }
			
			return :>> result : Real = EngineEvaluation(
				powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost
			);
		}
		
		return part :>> selectedAlternative : Engine;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "33_trade_study_analysis_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Trade Study Analysis Example' {
    private import ScalarValues::Real;
    private import TradeStudies::*;
    part def Engine;
    part engine4cyl : Engine;
    part engine6cyl : Engine;
    calc def PowerRollup {
        in engine : Engine;
        return : ISQ::PowerValue;
    }
    calc def MassRollup {
        in engine : Engine;
        return : ISQ::MassValue;
    }
    calc def EfficiencyRollup {
        in engine : Engine;
        return : Real;
    }
    calc def CostRollup {
        in engine : Engine;
        return : Real;
    }
    calc def EngineEvaluation {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in efficiency : Real;
        in cost : Real;
        return evaluation : Real;
    }
    analysis engineTradeStudy : TradeStudy {
        subject : Engine = (engine4cyl, engine6cyl);
        objective : MaximizeObjective ;
        calc :>> evaluationFunction {
            in part anEngine : Engine :>> alternative;
            calc powerRollup : PowerRollup {
                in engine = anEngine;
                return power;
            }
            calc massRollup : MassRollup {
                in engine = anEngine;
                return mass;
            }
            calc efficiencyRollup : EfficiencyRollup {
                in engine = anEngine;
                return efficiency;
            }
            calc costRollup : CostRollup {
                in engine = anEngine;
                return cost;
            }
            return :>> result : Real = EngineEvaluation(powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost);
        }
        return part :>> selectedAlternative : Engine;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 57) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 71) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 93) (line 3) (column 17) (len 12)) (segments (segment 0 (token "TradeStudies") (name "TradeStudies") (separator none) (span (offset 93) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 149) (line 6) (column 20) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 149) (line 6) (column 20) (len 6)))))
    (reference r3 (scope relative) (span (offset 176) (line 7) (column 20) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 176) (line 7) (column 20) (len 6)))))
    (reference r4 (scope relative) (span (offset 222) (line 9) (column 37) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 222) (line 9) (column 37) (len 6)))))
    (reference r5 (scope relative) (span (offset 293) (line 10) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 293) (line 10) (column 36) (len 6)))))
    (reference r6 (scope relative) (span (offset 369) (line 11) (column 42) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 369) (line 11) (column 42) (len 6)))))
    (reference r7 (scope relative) (span (offset 429) (line 12) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 429) (line 12) (column 36) (len 6)))))
    (reference r8 (scope relative) (span (offset 499) (line 15) (column 14) (len 15)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 499) (line 15) (column 14) (len 3))) (segment 1 (token "PowerValue") (name "PowerValue") (separator colon-colon) (span (offset 504) (line 15) (column 19) (len 10)))))
    (reference r9 (scope relative) (span (offset 528) (line 16) (column 13) (len 14)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 528) (line 16) (column 13) (len 3))) (segment 1 (token "MassValue") (name "MassValue") (separator colon-colon) (span (offset 533) (line 16) (column 18) (len 9)))))
    (reference r10 (scope relative) (span (offset 562) (line 17) (column 19) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 562) (line 17) (column 19) (len 4)))))
    (reference r11 (scope relative) (span (offset 580) (line 18) (column 13) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 580) (line 18) (column 13) (len 4)))))
    (reference r12 (scope relative) (span (offset 676) (line 23) (column 30) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 676) (line 23) (column 30) (len 10)))))
  )
  (root (package (name "Trade Study Analysis Example") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 93) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 105) (line 3) (column 29) (len 3))) (separator (span (offset 105) (line 3) (column 29) (len 2))) (marker (span (offset 107) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine4cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine6cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-def (name "PowerRollup") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "engine") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 210) (line 9) (column 25) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "MassRollup") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "engine") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 281) (line 10) (column 24) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "EfficiencyRollup") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "engine") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 357) (line 11) (column 30) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "CostRollup") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "engine") (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 417) (line 12) (column 24) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "EngineEvaluation") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "power") (subsets none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 488) (line 15) (column 3) (len 27))) (in-out (direction in) (reference false) (declaration "mass") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 518) (line 16) (column 3) (len 25))) (in-out (direction in) (reference false) (declaration "efficiency") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 546) (line 17) (column 3) (len 21))) (in-out (direction in) (reference false) (declaration "cost") (subsets none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 570) (line 18) (column 3) (len 15))) (return-declaration (name "evaluation") (short-name none)))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeStudy") (type (ref r12)) (subsets none) (redefines none)))))
)
~~~
