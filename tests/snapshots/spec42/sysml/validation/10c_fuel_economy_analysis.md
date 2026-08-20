# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (10-Analysis and Trades): 10c-Fuel Economy Analysis"))
~~~
# SOURCE
~~~sysml
package '10c-Fuel Economy Analysis' {
	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import ISQ::*;
	private import USCustomaryUnits::*;
	
	attribute distancePerVolume : ScalarQuantityValue = length / volume;	
	attribute gallon : MeasurementUnit = 231.0 * 'in'^3;
	
	package FuelEconomyRequirementsModel {
		
		requirement def FuelEconomyRequirement {
			attribute actualFuelEconomy :> distancePerVolume;
			attribute requiredFuelEconomy :> distancePerVolume;
			
			require constraint { actualFuelEconomy >= requiredFuelEconomy }
		}
		
		requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
			:>> requiredFuelEconomy = 25 [mi/gallon];
		}
		
		requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
			:>> requiredFuelEconomy = 30 [mi/gallon];
		}
		
	}
		
	package VehicleDesignModel {
		
		part def Vehicle {
			attribute fuelEconomy_city :> distancePerVolume;
			attribute fuelEconomy_highway :> distancePerVolume;
			
			attribute cargoWeight : MassValue;
		}
		
		part def Engine;
		part def Transmission;
		
		part vehicle1_c1 : Vehicle {
			part engine : Engine;
			part transmission : Transmission {
				exhibit state transmissionState {
					entry; then '1stGear';
					state '1stGear';
					then '2ndGear';
					state '2ndGear';
					then '3rdGear';
					state '3rdGear';
					then '4thGear';
					state '4thGear';
				}
			}
		}
		
	}
	
	package FuelEconomyAnalysisModel {
		private import VehicleDesignModel::*;
		private import FuelEconomyRequirementsModel::*;
		
		attribute def ScenarioState {
			position : LengthValue;
			velocity : SpeedValue;
			acceleration : AccelerationValue;
			inclineAngle : AngularMeasureValue;
		}
		
		abstract calc def NominalScenario { 
			in t : TimeValue; 
			return : ScenarioState;
		}
		calc cityScenario : NominalScenario;
		calc highwayScenario : NominalScenario;
		
		analysis def FuelEconomyAnalysis {
			subject vehicle : Vehicle;
			in calc scenario : NominalScenario;
			in requirement fuelEconomyRequirement : FuelEconomyRequirement;
			return calculatedFuelEconomy : ScalarQuantityValue;
			
			objective fuelEconomyAnalysisObjective {
				doc /*
				     * The objective of this analysis is to determine whether the
				     * current vehicle design configuration can satisfy the fuel
				     * economy requirement.
				     */
				 
				 assume constraint {
				 	doc /* wheelDiameter == 33 inches
				 	     * drive train efficiency == 0.4
				 	     */
				 }
				 
				 require fuelEconomyRequirement {
				 	:>> actualFuelEconomy = calculatedFuelEconomy;
				 }
			}
			
			action dynamicsAnalysis {
				/*
				 * Solve for the required engine power as a function of time
				 * to support the nominal scenarios.
				 * 
				 * Note: Vehicle force = power/speed
				 * Note: EngineRPM * EngineGearRatio/WheelRPM = constant
				 */
			}
			
			action fuelConsumptionAnalysis {
				/*
				 * Solve the engine equations to determine how much fuel is
				 * consumed. The engine RPM is a function of the speed of the
				 * vehicle and the gear state.
				 */
			}
		}
		
		requirement vehicleFuelEconomyRequirementsGroup {
			subject vehicle : Vehicle;
			requirement vehicleFuelEconomyRequirement_city :> cityFuelEconomyRequirement {
				doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 25 miles per gallon for the nominal city driving scenarios.
				     */
				 
				:>> actualFuelEconomy = vehicle.fuelEconomy_city;
				
				assume constraint { vehicle.cargoWeight == 1000 [lb] }
			}

			requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
				doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */
				
				:>> actualFuelEconomy = vehicle.fuelEconomy_highway;
				
				assume constraint { vehicle.cargoWeight == 1000 [lb] }
			}

		}

		part analysisContext {
			
			analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis {
				subject vehicle = vehicle1_c1;
				in calc scenario = cityScenario;
				in requirement fuelEconomyRequirement = cityFuelEconomyRequirement;
			} 
			
			analysis highwayFuelEconomyAnalysis : FuelEconomyAnalysis {
				subject vehicle = vehicle1_c1;
				in calc scenario = highwayScenario;
				in requirement fuelEconomyRequirement = highwayFuelEconomyRequirement;
			}
			
			part vehicle1_c1_analysized :> vehicle1_c1 {
				:>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
				:>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
			}		
			
			satisfy vehicleFuelEconomyRequirementsGroup by vehicle1_c1_analysized;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10c_fuel_economy_analysis.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '10c-Fuel Economy Analysis' {
    private import ScalarValues::*;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQ::*;
    private import USCustomaryUnits::*;
    attribute distancePerVolume : ScalarQuantityValue = length / volume;
    attribute gallon : MeasurementUnit = 231.0 * 'in' ^ 3;
    package FuelEconomyRequirementsModel {
        requirement def FuelEconomyRequirement {
            attribute actualFuelEconomy :> distancePerVolume;
            attribute requiredFuelEconomy :> distancePerVolume;
            require constraint {
                actualFuelEconomy >= requiredFuelEconomy;
            }
        }
        requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 25[mi / gallon];
        }
        requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 30[mi / gallon];
        }
    }
    package VehicleDesignModel {
        part def Vehicle {
            attribute fuelEconomy_city :> distancePerVolume;
            attribute fuelEconomy_highway :> distancePerVolume;
            attribute cargoWeight : MassValue;
        }
        part def Engine;
        part def Transmission;
        part vehicle1_c1 : Vehicle {
            part engine : Engine;
            part transmission : Transmission {
                state transmissionState {
                    entry;
                    then '1stGear';
                    state '1stGear';
                    then '2ndGear';
                    state '2ndGear';
                    then '3rdGear';
                    state '3rdGear';
                    then '4thGear';
                    state '4thGear';
                }
            }
        }
    }
    package FuelEconomyAnalysisModel {
        private import VehicleDesignModel::*;
        private import FuelEconomyRequirementsModel::*;
        attribute def ScenarioState {
            position : LengthValue;
            velocity : SpeedValue;
            acceleration : AccelerationValue;
            inclineAngle : AngularMeasureValue;
        }
        abstract calc def NominalScenario {
            in t : TimeValue;
            return : ScenarioState;
        }
        calc def cityScenario : NominalScenario;
        calc def highwayScenario : NominalScenario;
        analysis def FuelEconomyAnalysis {
            subject vehicle : Vehicle;
            in calc scenario : NominalScenario;
            in requirement fuelEconomyRequirement : FuelEconomyRequirement;
            return calculatedFuelEconomy : ScalarQuantityValue;
            objective fuelEconomyAnalysisObjective  {
                doc
                /*
				     * The objective of this analysis is to determine whether the
				     * current vehicle design configuration can satisfy the fuel
				     * economy requirement.
				     */
                assume constraint {
                    doc
                    /* wheelDiameter == 33 inches
				 	     * drive train efficiency == 0.4
				 	     */
                }
                require fuelEconomyRequirement {
                    :>> actualFuelEconomy = calculatedFuelEconomy;
                }
            }
            action dynamicsAnalysis {
                /*
				 * Solve for the required engine power as a function of time
				 * to support the nominal scenarios.
				 * 
				 * Note: Vehicle force = power/speed
				 * Note: EngineRPM * EngineGearRatio/WheelRPM = constant
				 */
            }
            action fuelConsumptionAnalysis {
                /*
				 * Solve the engine equations to determine how much fuel is
				 * consumed. The engine RPM is a function of the speed of the
				 * vehicle and the gear state.
				 */
            }
        }
        requirement vehicleFuelEconomyRequirementsGroup {
            subject vehicle : Vehicle;
            requirement vehicleFuelEconomyRequirement_city :> cityFuelEconomyRequirement {
                doc
                /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 25 miles per gallon for the nominal city driving scenarios.
				     */
                :>> actualFuelEconomy = vehicle.fuelEconomy_city;
                assume constraint {
                    vehicle.cargoWeight == 1000[lb];
                }
            }
            requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
                doc
                /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */
                :>> actualFuelEconomy = vehicle.fuelEconomy_highway;
                assume constraint {
                    vehicle.cargoWeight == 1000[lb];
                }
            }
        }
        part analysisContext {
            analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis {
                subject vehicle = vehicle1_c1;
                in calc scenario = cityScenario;
                in requirement fuelEconomyRequirement = cityFuelEconomyRequirement;
            }
            analysis highwayFuelEconomyAnalysis : FuelEconomyAnalysis {
                subject vehicle = vehicle1_c1;
                in calc scenario = highwayScenario;
                in requirement fuelEconomyRequirement = highwayFuelEconomyRequirement;
            }
            part vehicle1_c1_analysized :> vehicle1_c1 {
                attribute :>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
                attribute :>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
            }
            satisfy vehicleFuelEconomyRequirementsGroup by vehicle1_c1_analysized;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 87) (line 3) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 118) (line 4) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 118) (line 4) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 160) (line 5) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 160) (line 5) (column 17) (len 3)))))
    (reference r4 (scope relative) (span (offset 184) (line 6) (column 17) (len 16)) (segments (segment 0 (token "USCustomaryUnits") (name "USCustomaryUnits") (separator none) (span (offset 184) (line 6) (column 17) (len 16)))))
    (reference r5 (scope relative) (span (offset 942) (line 33) (column 34) (len 17)) (segments (segment 0 (token "distancePerVolume") (name "distancePerVolume") (separator none) (span (offset 942) (line 33) (column 34) (len 17)))))
    (reference r6 (scope relative) (span (offset 997) (line 34) (column 37) (len 17)) (segments (segment 0 (token "distancePerVolume") (name "distancePerVolume") (separator none) (span (offset 997) (line 34) (column 37) (len 17)))))
    (reference r7 (scope relative) (span (offset 1047) (line 36) (column 28) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 1047) (line 36) (column 28) (len 9)))))
    (reference r8 (scope relative) (span (offset 1133) (line 42) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1133) (line 42) (column 22) (len 7)))))
    (reference r9 (scope relative) (span (offset 1160) (line 43) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1160) (line 43) (column 18) (len 6)))))
    (reference r10 (scope relative) (span (offset 1191) (line 44) (column 24) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1191) (line 44) (column 24) (len 12)))))
    (reference r11 (scope relative) (span (offset 1499) (line 61) (column 18) (len 18)) (segments (segment 0 (token "VehicleDesignModel") (name "VehicleDesignModel") (separator none) (span (offset 1499) (line 61) (column 18) (len 18)))))
    (reference r12 (scope relative) (span (offset 1539) (line 62) (column 18) (len 28)) (segments (segment 0 (token "FuelEconomyRequirementsModel") (name "FuelEconomyRequirementsModel") (separator none) (span (offset 1539) (line 62) (column 18) (len 28)))))
    (reference r13 (scope relative) (span (offset 1621) (line 65) (column 15) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 1621) (line 65) (column 15) (len 11)))))
    (reference r14 (scope relative) (span (offset 1648) (line 66) (column 15) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 1648) (line 66) (column 15) (len 10)))))
    (reference r15 (scope relative) (span (offset 1678) (line 67) (column 19) (len 17)) (segments (segment 0 (token "AccelerationValue") (name "AccelerationValue") (separator none) (span (offset 1678) (line 67) (column 19) (len 17)))))
    (reference r16 (scope relative) (span (offset 1715) (line 68) (column 19) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 1715) (line 68) (column 19) (len 19)))))
    (reference r17 (scope relative) (span (offset 4491) (line 159) (column 35) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 4491) (line 159) (column 35) (len 11)))))
    (reference r18 (scope relative) (span (offset 4513) (line 160) (column 9) (len 16)) (segments (segment 0 (token "fuelEconomy_city") (name "fuelEconomy_city") (separator none) (span (offset 4513) (line 160) (column 9) (len 16)))))
    (reference r19 (scope relative) (span (offset 4532) (line 160) (column 28) (len 23)) (segments (segment 0 (token "cityFuelEconomyAnalysis") (name "cityFuelEconomyAnalysis") (separator none) (span (offset 4532) (line 160) (column 28) (len 23)))))
    (reference r20 (scope relative) (span (offset 4556) (line 160) (column 52) (len 21)) (segments (segment 0 (token "calculatedFuelEconomy") (name "calculatedFuelEconomy") (separator none) (span (offset 4556) (line 160) (column 52) (len 21)))))
    (reference r21 (scope relative) (span (offset 4587) (line 161) (column 9) (len 19)) (segments (segment 0 (token "fuelEconomy_highway") (name "fuelEconomy_highway") (separator none) (span (offset 4587) (line 161) (column 9) (len 19)))))
    (reference r22 (scope relative) (span (offset 4609) (line 161) (column 31) (len 26)) (segments (segment 0 (token "highwayFuelEconomyAnalysis") (name "highwayFuelEconomyAnalysis") (separator none) (span (offset 4609) (line 161) (column 31) (len 26)))))
    (reference r23 (scope relative) (span (offset 4636) (line 161) (column 58) (len 21)) (segments (segment 0 (token "calculatedFuelEconomy") (name "calculatedFuelEconomy") (separator none) (span (offset 4636) (line 161) (column 58) (len 21)))))
    (reference r24 (scope relative) (span (offset 4681) (line 164) (column 12) (len 35)) (segments (segment 0 (token "vehicleFuelEconomyRequirementsGroup") (name "vehicleFuelEconomyRequirementsGroup") (separator none) (span (offset 4681) (line 164) (column 12) (len 35)))))
    (reference r25 (scope relative) (span (offset 4720) (line 164) (column 51) (len 22)) (segments (segment 0 (token "vehicle1_c1_analysized") (name "vehicle1_c1_analysized") (separator none) (span (offset 4720) (line 164) (column 51) (len 22)))))
  )
  (root (package (name "10c-Fuel Economy Analysis") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 97) (line 3) (column 27) (len 3))) (separator (span (offset 97) (line 3) (column 27) (len 2))) (marker (span (offset 99) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 118) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 139) (line 4) (column 38) (len 3))) (separator (span (offset 139) (line 4) (column 38) (len 2))) (marker (span (offset 141) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 160) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 163) (line 5) (column 20) (len 3))) (separator (span (offset 163) (line 5) (column 20) (len 2))) (marker (span (offset 165) (line 5) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 184) (line 6) (column 17) (len 19))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 200) (line 6) (column 33) (len 3))) (separator (span (offset 200) (line 6) (column 33) (len 2))) (marker (span (offset 202) (line 6) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-usage) (attribute-usage) (package (name "FuelEconomyRequirementsModel") (body brace (requirement-def (name "FuelEconomyRequirement") (modifiers) (body brace (attribute-usage) (attribute-usage) (require-constraint))) (requirement-usage (name "cityFuelEconomyRequirement") (multiplicity none)) (requirement-usage (name "highwayFuelEconomyRequirement") (multiplicity none)))) (package (name "VehicleDesignModel") (body brace (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "fuelEconomy_city") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelEconomy_highway") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r6)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "cargoWeight") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (modifiers) (body semicolon)) (part-def (name "Transmission") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (state-usage))))))) (package (name "FuelEconomyAnalysisModel") (body brace (import (target (span (span (offset 1499) (line 61) (column 18) (len 21))) (all none) (ref r11) (shape (namespace (wildcard-suffix (span (span (offset 1517) (line 61) (column 36) (len 3))) (separator (span (offset 1517) (line 61) (column 36) (len 2))) (marker (span (offset 1519) (line 61) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1539) (line 62) (column 18) (len 31))) (all none) (ref r12) (shape (namespace (wildcard-suffix (span (span (offset 1567) (line 62) (column 46) (len 3))) (separator (span (offset 1567) (line 62) (column 46) (len 2))) (marker (span (offset 1569) (line 62) (column 48) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "ScenarioState") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "position") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "velocity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "acceleration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "inclineAngle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (calc-def (name "NominalScenario") (modifiers (abstract (span (offset 1745) (line 71) (column 3) (len 8)))) (body brace (in-out-declaration) (return-declaration (name none) (short-name none)))) (calc-def (name "cityScenario") (modifiers) (body semicolon)) (calc-def (name "highwayScenario") (modifiers) (body semicolon)) (analysis-case-def (modifiers)) (requirement-usage (name "vehicleFuelEconomyRequirementsGroup") (multiplicity none)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "analysisContext") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (analysis-case-usage) (analysis-case-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1_analysized") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r17))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4532) (line 160) (column 28) (len 45)) (member-access (base (expression (span (offset 4532) (line 160) (column 28) (len 23)) (ref r19))) (separator dot) (member (ref r20))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4609) (line 161) (column 31) (len 48)) (member-access (base (expression (span (offset 4609) (line 161) (column 31) (len 26)) (ref r22))) (separator dot) (member (ref r23))))))) (body semicolon)))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r24))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r25)) (body semicolon)))))))))
)
~~~
