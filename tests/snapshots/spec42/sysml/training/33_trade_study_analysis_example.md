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
  )
  (root (package (name "Trade Study Analysis Example") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 93) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 105) (line 3) (column 29) (len 3))) (separator (span (offset 105) (line 3) (column 29) (len 2))) (marker (span (offset 107) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body semicolon)) (part-usage (declaration-name "engine4cyl") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body semicolon)) (part-usage (declaration-name "engine6cyl") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (body semicolon)) (calc-def) (calc-def) (calc-def) (calc-def) (calc-def) (analysis-case-usage))))
)
~~~
