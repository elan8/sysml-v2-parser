# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (10-Analysis and Trades): 10b-Trade-off Among Alternative Configurations"))
~~~
# SOURCE
~~~sysml
package '10b-Trade-off Among Alternative Configurations' {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	private import Definitions::*;
	private import Usages::*;
	
	package Definitions {
		
		part def Vehicle;
		
		part def Engine {
			power : ISQ::PowerValue;
			mass : ISQ::MassValue;
			efficiency : Real;
			reliability : Real;
			cost : Real;
		}
		
		part def Piston;
		part def Cylinder;
		part def ConnectingRod;
		part def CrankShaft;
		
		part def '4CylCrankShaft' :> CrankShaft;
		part def '6CylCrankShaft' :> CrankShaft;
		
	}
	
	package Usages {
		
		part engine : Engine {
			part cyl[*] : Cylinder {
				part p[1] : Piston;
				part rod[1] : ConnectingRod;
			}
			
			part cs : CrankShaft;
		}
		
		variation part engineChoice :> engine {
			variant part '4cylEngine' {
				part :>> cyl[4];
				part :>> cs : '4CylCrankShaft';
			}
			
			variant part '6cylEngine' {
				part :>> cyl[6];
				part :>> cs : '6CylCrankShaft';
			}
		}
		
		part vehicle : Vehicle {
			part engine[1] :> engineChoice = engineChoice::'6cylEngine' {
				assert constraint engineSelectionRational { 
					doc /* Selected the best engine based on the 'engineTradeStudy'. */
					engine == Analysis::engineTradeStudy.selectedAlternative
				}
			}
			
		}
	}
	
	package Analysis {

		calc def EngineEvaluation {
			doc /* Evaluation function with criteria power, mass, efficency and cost. */
			in power : ISQ::PowerValue;
			in mass : ISQ::MassValue; 
			in efficiency : Real; 
			in cost : Real;
			return evaluation : Real;
			// Compute evaluation...
		}
			
		analysis engineTradeStudy : TradeStudy {
			subject : Engine[1..*] = all engineChoice;
			objective : MaximizeObjective;

			calc :>> evaluationFunction {
				in part anEngine :>> alternative : Engine;
				
				calc powerRollup { in engine = anEngine; return power:>ISQ::power; }
				calc massRollup { in engine = anEngine; return mass:>ISQ::mass; }
				calc efficiencyRollup { in engine = anEngine; return efficiency: Real; }
				calc costRollup { in engine = anEngine; return cost: Real; }
				
				return :>> result : Real = EngineEvaluation(
					powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost
				);
			}
			
			return part :>> selectedAlternative : Engine;
		}
        
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10b_trade_off_among_alternative_configurations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '10b-Trade-off Among Alternative Configurations' {
    private import ScalarValues::Real;
    private import TradeStudies::*;
    private import Definitions::*;
    private import Usages::*;
    package Definitions {
        part def Vehicle;
        part def Engine {
            power : ISQ::PowerValue;
            mass : ISQ::MassValue;
            efficiency : Real;
            reliability : Real;
            cost : Real;
        }
        part def Piston;
        part def Cylinder;
        part def ConnectingRod;
        part def CrankShaft;
        part def '4CylCrankShaft' :> CrankShaft;
        part def '6CylCrankShaft' :> CrankShaft;
    }
    package Usages {
        part engine : Engine {
            part cyl : Cylinder[*] {
                part p : Piston[1];
                part rod : ConnectingRod[1];
            }
            part cs : CrankShaft;
        }
        variation part engineChoice :> engine {
            variant part '4cylEngine' {
                part :>> cyl[4];
                part :>> cs : '4CylCrankShaft';
            }
            variant part '6cylEngine' {
                part :>> cyl[6];
                part :>> cs : '6CylCrankShaft';
            }
        }
        part vehicle : Vehicle {
            part engine[1] :> engineChoice = engineChoice::'6cylEngine' {
                assert constraint engineSelectionRational {
                    doc
                    /* Selected the best engine based on the 'engineTradeStudy'. */
                    engine == Analysis::engineTradeStudy.selectedAlternative;
                }
            }
        }
    }
    package Analysis {
        calc def EngineEvaluation {
            doc
            /* Evaluation function with criteria power, mass, efficency and cost. */
            in power : ISQ::PowerValue;
            in mass : ISQ::MassValue;
            in efficiency : Real;
            in cost : Real;
            return evaluation : Real;
        }
        analysis engineTradeStudy : TradeStudy {
            subject : Engine[1..*] = all engineChoice;
            objective : MaximizeObjective ;
            calc :>> evaluationFunction {
                in part anEngine : Engine :>> alternative;
                calc powerRollup {
                    in engine = anEngine;
                    return power :> ISQ::power;
                }
                calc massRollup {
                    in engine = anEngine;
                    return mass :> ISQ::mass;
                }
                calc efficiencyRollup {
                    in engine = anEngine;
                    return efficiency : Real;
                }
                calc costRollup {
                    in engine = anEngine;
                    return cost : Real;
                }
                return :>> result : Real = EngineEvaluation(powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost);
            }
            return part :>> selectedAlternative : Engine;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 75) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 75) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 89) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 111) (line 3) (column 17) (len 12)) (segments (segment 0 (token "TradeStudies") (name "TradeStudies") (separator none) (span (offset 111) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 144) (line 4) (column 17) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 144) (line 4) (column 17) (len 11)))))
    (reference r3 (scope relative) (span (offset 176) (line 5) (column 17) (len 6)) (segments (segment 0 (token "Usages") (name "Usages") (separator none) (span (offset 176) (line 5) (column 17) (len 6)))))
    (reference r4 (scope relative) (span (offset 269) (line 12) (column 12) (len 15)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 269) (line 12) (column 12) (len 3))) (segment 1 (token "PowerValue") (name "PowerValue") (separator colon-colon) (span (offset 274) (line 12) (column 17) (len 10)))))
    (reference r5 (scope relative) (span (offset 296) (line 13) (column 11) (len 14)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 296) (line 13) (column 11) (len 3))) (segment 1 (token "MassValue") (name "MassValue") (separator colon-colon) (span (offset 301) (line 13) (column 16) (len 9)))))
    (reference r6 (scope relative) (span (offset 328) (line 14) (column 17) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 328) (line 14) (column 17) (len 4)))))
    (reference r7 (scope relative) (span (offset 351) (line 15) (column 18) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 351) (line 15) (column 18) (len 4)))))
    (reference r8 (scope relative) (span (offset 367) (line 16) (column 11) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 367) (line 16) (column 11) (len 4)))))
    (reference r9 (scope relative) (span (offset 603) (line 31) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 603) (line 31) (column 17) (len 6)))))
    (reference r10 (scope relative) (span (offset 629) (line 32) (column 18) (len 8)) (segments (segment 0 (token "Cylinder") (name "Cylinder") (separator none) (span (offset 629) (line 32) (column 18) (len 8)))))
    (reference r11 (scope relative) (span (offset 656) (line 33) (column 17) (len 6)) (segments (segment 0 (token "Piston") (name "Piston") (separator none) (span (offset 656) (line 33) (column 17) (len 6)))))
    (reference r12 (scope relative) (span (offset 682) (line 34) (column 19) (len 13)) (segments (segment 0 (token "ConnectingRod") (name "ConnectingRod") (separator none) (span (offset 682) (line 34) (column 19) (len 13)))))
    (reference r13 (scope relative) (span (offset 719) (line 37) (column 14) (len 10)) (segments (segment 0 (token "CrankShaft") (name "CrankShaft") (separator none) (span (offset 719) (line 37) (column 14) (len 10)))))
    (reference r14 (scope relative) (span (offset 771) (line 40) (column 34) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 771) (line 40) (column 34) (len 6)))))
    (reference r15 (scope relative) (span (offset 824) (line 42) (column 14) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 824) (line 42) (column 14) (len 3)))))
    (reference r16 (scope relative) (span (offset 850) (line 43) (column 19) (len 16)) (segments (segment 0 (token "'4CylCrankShaft'") (name "4CylCrankShaft") (separator none) (span (offset 850) (line 43) (column 19) (len 16)))))
    (reference r17 (scope relative) (span (offset 845) (line 43) (column 14) (len 2)) (segments (segment 0 (token "cs") (name "cs") (separator none) (span (offset 845) (line 43) (column 14) (len 2)))))
    (reference r18 (scope relative) (span (offset 921) (line 47) (column 14) (len 3)) (segments (segment 0 (token "cyl") (name "cyl") (separator none) (span (offset 921) (line 47) (column 14) (len 3)))))
    (reference r19 (scope relative) (span (offset 947) (line 48) (column 19) (len 16)) (segments (segment 0 (token "'6CylCrankShaft'") (name "6CylCrankShaft") (separator none) (span (offset 947) (line 48) (column 19) (len 16)))))
    (reference r20 (scope relative) (span (offset 942) (line 48) (column 14) (len 2)) (segments (segment 0 (token "cs") (name "cs") (separator none) (span (offset 942) (line 48) (column 14) (len 2)))))
    (reference r21 (scope relative) (span (offset 994) (line 52) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 994) (line 52) (column 18) (len 7)))))
    (reference r22 (scope relative) (span (offset 1025) (line 53) (column 22) (len 12)) (segments (segment 0 (token "engineChoice") (name "engineChoice") (separator none) (span (offset 1025) (line 53) (column 22) (len 12)))))
    (reference r23 (scope relative) (span (offset 1040) (line 53) (column 37) (len 26)) (segments (segment 0 (token "engineChoice") (name "engineChoice") (separator none) (span (offset 1040) (line 53) (column 37) (len 12))) (segment 1 (token "'6cylEngine'") (name "6cylEngine") (separator colon-colon) (span (offset 1054) (line 53) (column 51) (len 12)))))
    (reference r24 (scope relative) (span (offset 1422) (line 67) (column 15) (len 15)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 1422) (line 67) (column 15) (len 3))) (segment 1 (token "PowerValue") (name "PowerValue") (separator colon-colon) (span (offset 1427) (line 67) (column 20) (len 10)))))
    (reference r25 (scope relative) (span (offset 1452) (line 68) (column 14) (len 14)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 1452) (line 68) (column 14) (len 3))) (segment 1 (token "MassValue") (name "MassValue") (separator colon-colon) (span (offset 1457) (line 68) (column 19) (len 9)))))
    (reference r26 (scope relative) (span (offset 1488) (line 69) (column 20) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1488) (line 69) (column 20) (len 4)))))
    (reference r27 (scope relative) (span (offset 1508) (line 70) (column 14) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1508) (line 70) (column 14) (len 4)))))
    (reference r28 (scope relative) (span (offset 1609) (line 75) (column 31) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 1609) (line 75) (column 31) (len 10)))))
  )
  (root (package (name "10b-Trade-off Among Alternative Configurations") (body brace (import (target (span (span (offset 75) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 111) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 123) (line 3) (column 29) (len 3))) (separator (span (offset 123) (line 3) (column 29) (len 2))) (marker (span (offset 125) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 144) (line 4) (column 17) (len 14))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 155) (line 4) (column 28) (len 3))) (separator (span (offset 155) (line 4) (column 28) (len 2))) (marker (span (offset 157) (line 4) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 176) (line 5) (column 17) (len 9))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 182) (line 5) (column 23) (len 3))) (separator (span (offset 182) (line 5) (column 23) (len 2))) (marker (span (offset 184) (line 5) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Engine") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "power") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "efficiency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "reliability") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "cost") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Piston") (modifiers) (body semicolon)) (part-def (name "Cylinder") (modifiers) (body semicolon)) (part-def (name "ConnectingRod") (modifiers) (body semicolon)) (part-def (name "CrankShaft") (modifiers) (body semicolon)) (part-def (name "4CylCrankShaft") (modifiers) (body semicolon)) (part-def (name "6CylCrankShaft") (modifiers) (body semicolon)))) (package (name "Usages") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cyl") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity (lower (expression (span (offset 651) (line 33) (column 12) (len 1)) (integer 1))) (upper (expression (span (offset 651) (line 33) (column 12) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rod") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity (lower (expression (span (offset 677) (line 34) (column 14) (len 1)) (integer 1))) (upper (expression (span (offset 677) (line 34) (column 14) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cs") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineChoice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r14))) (value none))) (redefines none) (value none) (body brace (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "4cylEngine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 828) (line 42) (column 18) (len 1)) (integer 4))) (upper (expression (span (offset 828) (line 42) (column 18) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (value none) (body semicolon))))) (body absent)) (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "6cylEngine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 925) (line 47) (column 18) (len 1)) (integer 6))) (upper (expression (span (offset 925) (line 47) (column 18) (len 1)) (integer 6)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (value none) (body semicolon))))) (body absent)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 1019) (line 53) (column 16) (len 1)) (integer 1))) (upper (expression (span (offset 1019) (line 53) (column 16) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r22))) (value (expression (span (offset 1040) (line 53) (column 37) (len 26)) (ref r23))))) (redefines none) (value none) (body brace (assert-constraint))))))) (package (name "Analysis") (body brace (calc-def (name "EngineEvaluation") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 1337) (line 66) (column 10) (len 68)) (normalized "Evaluation function with criteria power, mass, efficency and cost. "))) (in-out (direction in) (reference false) (declaration "power") (subsets none) (type (ref r24)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1411) (line 67) (column 4) (len 27))) (in-out (direction in) (reference false) (declaration "mass") (subsets none) (type (ref r25)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1442) (line 68) (column 4) (len 25))) (in-out (direction in) (reference false) (declaration "efficiency") (subsets none) (type (ref r26)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1472) (line 69) (column 4) (len 21))) (in-out (direction in) (reference false) (declaration "cost") (subsets none) (type (ref r27)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 1498) (line 70) (column 4) (len 15))) (return-declaration (name "evaluation") (short-name none)))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeStudy") (type (ref r28)) (subsets none) (redefines none)))))))
)
~~~
