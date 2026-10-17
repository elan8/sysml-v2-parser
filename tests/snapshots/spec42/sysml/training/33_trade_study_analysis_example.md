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
        objective : MaximizeObjective;
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
  (name-language sysml)
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
    (reference r13 (scope relative) (span (offset 701) (line 24) (column 13) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 701) (line 24) (column 13) (len 6)))))
    (reference r14 (scope relative) (span (offset 711) (line 24) (column 23) (len 10)) (segments (segment 0 (token "engine4cyl") (name "engine4cyl") (separator none) (span (offset 711) (line 24) (column 23) (len 10)))))
    (reference r15 (scope relative) (span (offset 723) (line 24) (column 35) (len 10)) (segments (segment 0 (token "engine6cyl") (name "engine6cyl") (separator none) (span (offset 723) (line 24) (column 35) (len 10)))))
    (reference r16 (scope relative) (span (offset 750) (line 25) (column 15) (len 17)) (segments (segment 0 (token "MaximizeObjective") (name "MaximizeObjective") (separator none) (span (offset 750) (line 25) (column 15) (len 17)))))
    (reference r17 (scope relative) (span (offset 781) (line 27) (column 12) (len 18)) (segments (segment 0 (token "evaluationFunction") (name "evaluationFunction") (separator none) (span (offset 781) (line 27) (column 12) (len 18)))))
    (reference r18 (scope relative) (span (offset 840) (line 28) (column 39) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 840) (line 28) (column 39) (len 6)))))
    (reference r19 (scope relative) (span (offset 826) (line 28) (column 25) (len 11)) (segments (segment 0 (token "alternative") (name "alternative") (separator none) (span (offset 826) (line 28) (column 25) (len 11)))))
    (reference r20 (scope relative) (span (offset 873) (line 30) (column 22) (len 11)) (segments (segment 0 (token "PowerRollup") (name "PowerRollup") (separator none) (span (offset 873) (line 30) (column 22) (len 11)))))
    (reference r21 (scope relative) (span (offset 899) (line 30) (column 48) (len 8)) (segments (segment 0 (token "anEngine") (name "anEngine") (separator none) (span (offset 899) (line 30) (column 48) (len 8)))))
    (reference r22 (scope relative) (span (offset 945) (line 31) (column 21) (len 10)) (segments (segment 0 (token "MassRollup") (name "MassRollup") (separator none) (span (offset 945) (line 31) (column 21) (len 10)))))
    (reference r23 (scope relative) (span (offset 970) (line 31) (column 46) (len 8)) (segments (segment 0 (token "anEngine") (name "anEngine") (separator none) (span (offset 970) (line 31) (column 46) (len 8)))))
    (reference r24 (scope relative) (span (offset 1021) (line 32) (column 27) (len 16)) (segments (segment 0 (token "EfficiencyRollup") (name "EfficiencyRollup") (separator none) (span (offset 1021) (line 32) (column 27) (len 16)))))
    (reference r25 (scope relative) (span (offset 1052) (line 32) (column 58) (len 8)) (segments (segment 0 (token "anEngine") (name "anEngine") (separator none) (span (offset 1052) (line 32) (column 58) (len 8)))))
    (reference r26 (scope relative) (span (offset 1103) (line 33) (column 21) (len 10)) (segments (segment 0 (token "CostRollup") (name "CostRollup") (separator none) (span (offset 1103) (line 33) (column 21) (len 10)))))
    (reference r27 (scope relative) (span (offset 1128) (line 33) (column 46) (len 8)) (segments (segment 0 (token "anEngine") (name "anEngine") (separator none) (span (offset 1128) (line 33) (column 46) (len 8)))))
    (reference r28 (scope relative) (span (offset 1321) (line 40) (column 19) (len 19)) (segments (segment 0 (token "selectedAlternative") (name "selectedAlternative") (separator none) (span (offset 1321) (line 40) (column 19) (len 19)))))
    (reference r29 (scope relative) (span (offset 1343) (line 40) (column 41) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1343) (line 40) (column 41) (len 6)))))
  )
  (root (package (name "Trade Study Analysis Example") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 93) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 105) (line 3) (column 29) (len 3))) (separator (span (offset 105) (line 3) (column 29) (len 2))) (marker (span (offset 107) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine4cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine6cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (calc-def (name "PowerRollup") (modifiers) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 210) (line 9) (column 25) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "MassRollup") (modifiers) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 281) (line 10) (column 24) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "EfficiencyRollup") (modifiers) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 357) (line 11) (column 30) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "CostRollup") (modifiers) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 417) (line 12) (column 24) (len 19))) (return-declaration (name none) (short-name none)))) (calc-def (name "EngineEvaluation") (modifiers) (body brace (in-out (direction in) (kind none) (reference false) (declaration "power") (short-name none) (subsets none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 488) (line 15) (column 3) (len 27))) (in-out (direction in) (kind none) (reference false) (declaration "mass") (short-name none) (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 518) (line 16) (column 3) (len 25))) (in-out (direction in) (kind none) (reference false) (declaration "efficiency") (short-name none) (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 546) (line 17) (column 3) (len 21))) (in-out (direction in) (kind none) (reference false) (declaration "cost") (short-name none) (subsets none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 570) (line 18) (column 3) (len 15))) (return-declaration (name "evaluation") (short-name none)))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeStudy") (short-name none) (type (ref r12)) (subsets none) (redefines none) (body brace (subject (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 710) (line 24) (column 22) (len 24)) (sequence (sequence-list (element first (expression (span (offset 711) (line 24) (column 23) (len 10)) (ref r14))) (element comma (expression (span (offset 723) (line 24) (column 35) (len 10)) (ref r15))))))))) (body semicolon)) (objective (visibility none) (name none) (type (ref r16)) (body semicolon)) (calc-usage (name none) (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (ref r17)) (value none) (body brace (part-usage (then false) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "anEngine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (calc-usage (name "powerRollup") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 899) (line 30) (column 48) (len 8)) (ref r21))))) (span (offset 887) (line 30) (column 36) (len 21))) (return-declaration (name "power") (short-name none)))) (calc-usage (name "massRollup") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 970) (line 31) (column 46) (len 8)) (ref r23))))) (span (offset 958) (line 31) (column 34) (len 21))) (return-declaration (name "mass") (short-name none)))) (calc-usage (name "efficiencyRollup") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1052) (line 32) (column 58) (len 8)) (ref r25))))) (span (offset 1040) (line 32) (column 46) (len 21))) (return-declaration (name "efficiency") (short-name none)))) (calc-usage (name "costRollup") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "engine") (short-name none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1128) (line 33) (column 46) (len 8)) (ref r27))))) (span (offset 1116) (line 33) (column 34) (len 21))) (return-declaration (name "cost") (short-name none)))) (return-declaration (name none) (short-name none)))) (case-return (declaration none) (target (ref r28)) (type (ref r29)) (redefines none) (feature-kind part) (subsetting false) (value none)))))))
)
~~~
