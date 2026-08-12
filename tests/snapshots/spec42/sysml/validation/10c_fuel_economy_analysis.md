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
    attribute def distancePerVolume : ScalarQuantityValue = length / volume;
    attribute def gallon : MeasurementUnit = 231.0 * in ^ 3;
    package FuelEconomyRequirementsModel {
        requirement def FuelEconomyRequirement {
            attribute actualFuelEconomy :> distancePerVolume;
            attribute requiredFuelEconomy :> distancePerVolume;
            require constraint {
                actualFuelEconomy >= requiredFuelEconomy;
            }
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
            attribute position : LengthValue;
            attribute velocity : SpeedValue;
            attribute acceleration : AccelerationValue;
            attribute inclineAngle : AngularMeasureValue;
        }
        calc def NominalScenario {
            in t : TimeValue;
            return : ScenarioState;
        }
        calc def cityScenario;
        calc def highwayScenario;
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
            }
            action fuelConsumptionAnalysis {
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
                    vehicle.cargoWeight == 1000 [lb];
                }
            }
            requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
                doc
                /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */
                :>> actualFuelEconomy = vehicle.fuelEconomy_highway;
                assume constraint {
                    vehicle.cargoWeight == 1000 [lb];
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
                attribute  :>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
                attribute  :>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
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
    (reference r8 (scope relative) (span (offset 1499) (line 61) (column 18) (len 18)) (segments (segment 0 (token "VehicleDesignModel") (name "VehicleDesignModel") (separator none) (span (offset 1499) (line 61) (column 18) (len 18)))))
    (reference r9 (scope relative) (span (offset 1539) (line 62) (column 18) (len 28)) (segments (segment 0 (token "FuelEconomyRequirementsModel") (name "FuelEconomyRequirementsModel") (separator none) (span (offset 1539) (line 62) (column 18) (len 28)))))
  )
  (root (package (name "10c-Fuel Economy Analysis") (body (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 97) (line 3) (column 27) (len 3))) (separator (span (offset 97) (line 3) (column 27) (len 2))) (marker (span (offset 99) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 118) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 139) (line 4) (column 38) (len 3))) (separator (span (offset 139) (line 4) (column 38) (len 2))) (marker (span (offset 141) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 160) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 163) (line 5) (column 20) (len 3))) (separator (span (offset 163) (line 5) (column 20) (len 2))) (marker (span (offset 165) (line 5) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 184) (line 6) (column 17) (len 19))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 200) (line 6) (column 33) (len 3))) (separator (span (offset 200) (line 6) (column 33) (len 2))) (marker (span (offset 202) (line 6) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (attribute-def) (package (name "FuelEconomyRequirementsModel") (body (requirement-def (name "FuelEconomyRequirement") (body (attribute-usage) (attribute-usage) (require-constraint))) (requirement-usage) (requirement-usage))) (package (name "VehicleDesignModel") (body (part-def (name "Vehicle") (body (attribute-usage (declaration-name "fuelEconomy_city") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelEconomy_highway") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r6)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "cargoWeight") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-usage))) (package (name "FuelEconomyAnalysisModel") (body (import (target (span (span (offset 1499) (line 61) (column 18) (len 21))) (all none) (ref r8) (shape (namespace (wildcard-suffix (span (span (offset 1517) (line 61) (column 36) (len 3))) (separator (span (offset 1517) (line 61) (column 36) (len 2))) (marker (span (offset 1519) (line 61) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1539) (line 62) (column 18) (len 31))) (all none) (ref r9) (shape (namespace (wildcard-suffix (span (span (offset 1567) (line 62) (column 46) (len 3))) (separator (span (offset 1567) (line 62) (column 46) (len 2))) (marker (span (offset 1569) (line 62) (column 48) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (calc-def) (calc-def) (calc-def) (analysis-case-def) (requirement-usage) (part-usage))))))
)
~~~
