# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (10-Analysis and Trades): 10d-Dynamics Analysis"))
~~~
# SOURCE
~~~sysml
package '10d-Dynamics Analysis' {
	private import ISQ::*;
	
	package VehicleModel {
	
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}
	
	}
	
	package DynamicsModel {
	    
	    calc def Acceleration {
	    	in p : PowerValue;
	    	in m : MassValue;
	    	in v : SpeedValue;
	    	return : AccelerationValue = p / (m * v);
	    }
	    
	    calc def Velocity {
	    	in v0 : SpeedValue; 
	    	in a : AccelerationValue; 
	    	in dt : TimeValue;
	    	return : SpeedValue = v0 + a * dt;
	    }
	    
	    calc def Position {
	    	in x0 : LengthValue;
	    	in v : SpeedValue; 
	    	in dt : TimeValue;
	    	return : LengthValue = x0 + v * dt;
	    }
	    
	    action def StraightLineDynamics {
	        in power : PowerValue;
	        in mass : MassValue;
	        in delta_t : TimeValue;
	        in x_in : LengthValue;
	        in v_in : SpeedValue;
	        out x_out : LengthValue = Position(x_in, v_in, delta_t);
	        out v_out : SpeedValue = Velocity(v_in, a_out, delta_t);
	        out a_out : AccelerationValue = Acceleration(power, mass, v_in);
	    }
	}
	
	package AnalysisModel {
		private import VehicleModel::*;
		private import DynamicsModel::*;
		private import SampledFunctions::*;
		private import ScalarValues::Natural;
		private import SequenceFunctions::*;
		
		analysis def DynamicsAnalysis {
			subject vehicle : Vehicle;
			in attribute powerProfile :> ISQ::power[*];
			in attribute initialPosition :> ISQ::length;
			in attribute initialSpeed :> ISQ::speed;
			in attribute deltaT :> ISQ::time;
			return attribute accelerationProfile :> ISQ::acceleration[*] := ();
			
			private attribute position := initialPosition;
			private attribute speed := initialSpeed;
			
			for i in 1..powerProfile->size()-1 {
				perform action dynamics : StraightLineDynamics {
					in power = powerProfile#(i);
					in mass = vehicle.mass;
					in delta_t = deltaT;
					in x_in = position;
					in v_in = speed;
				}
				then assign position := dynamics.x_out;
				then assign speed := dynamics.v_out;
				then assign accelerationProfile := accelerationProfile->including(dynamics.a_out);
			}
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10d_dynamics_analysis.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '10d-Dynamics Analysis' {
    private import ISQ::*;
    package VehicleModel {
        part def Vehicle {
            attribute mass :> ISQ::mass;
        }
    }
    package DynamicsModel {
        calc def Acceleration {
            in p : PowerValue;
            in m : MassValue;
            in v : SpeedValue;
            return : AccelerationValue = p / (m * v);
        }
        calc def Velocity {
            in v0 : SpeedValue;
            in a : AccelerationValue;
            in dt : TimeValue;
            return : SpeedValue = v0 + a * dt;
        }
        calc def Position {
            in x0 : LengthValue;
            in v : SpeedValue;
            in dt : TimeValue;
            return : LengthValue = x0 + v * dt;
        }
        action def StraightLineDynamics {
            in power : PowerValue;
            in mass : MassValue;
            in delta_t : TimeValue;
            in x_in : LengthValue;
            in v_in : SpeedValue;
            out x_out : LengthValue = Position(x_in, v_in, delta_t);
            out v_out : SpeedValue = Velocity(v_in, a_out, delta_t);
            out a_out : AccelerationValue = Acceleration(power, mass, v_in);
        }
    }
    package AnalysisModel {
        private import VehicleModel::*;
        private import DynamicsModel::*;
        private import SampledFunctions::*;
        private import ScalarValues::Natural;
        private import SequenceFunctions::*;
        analysis def DynamicsAnalysis {
            subject vehicle : Vehicle;
            in attribute powerProfile[*] :> ISQ::power;
            in attribute initialPosition :> ISQ::length;
            in attribute initialSpeed :> ISQ::speed;
            in attribute deltaT :> ISQ::time;
            return attribute accelerationProfile :> ISQ::acceleration[*] := null;
            private attribute position := initialPosition;
            private attribute speed := initialSpeed;
            for i in 1..powerProfile->size() - 1  {
                perform action dynamics : StraightLineDynamics {
                    in power = powerProfile#(i);
                    in mass = vehicle.mass;
                    in delta_t = deltaT;
                    in x_in = position;
                    in v_in = speed;
                }
                then assign position := dynamics.x_out;
                then assign speed := dynamics.v_out;
                then assign accelerationProfile := accelerationProfile->including(dynamics.a_out);
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 50) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 128) (line 7) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 128) (line 7) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 133) (line 7) (column 27) (len 4)))))
    (reference r2 (scope relative) (span (offset 1124) (line 48) (column 18) (len 12)) (segments (segment 0 (token "VehicleModel") (name "VehicleModel") (separator none) (span (offset 1124) (line 48) (column 18) (len 12)))))
    (reference r3 (scope relative) (span (offset 1158) (line 49) (column 18) (len 13)) (segments (segment 0 (token "DynamicsModel") (name "DynamicsModel") (separator none) (span (offset 1158) (line 49) (column 18) (len 13)))))
    (reference r4 (scope relative) (span (offset 1193) (line 50) (column 18) (len 16)) (segments (segment 0 (token "SampledFunctions") (name "SampledFunctions") (separator none) (span (offset 1193) (line 50) (column 18) (len 16)))))
    (reference r5 (scope relative) (span (offset 1231) (line 51) (column 18) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 1231) (line 51) (column 18) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 1245) (line 51) (column 32) (len 7)))))
    (reference r6 (scope relative) (span (offset 1271) (line 52) (column 18) (len 17)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 1271) (line 52) (column 18) (len 17)))))
  )
  (root (package (name "10d-Dynamics Analysis") (body (import (target (span (span (offset 50) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 20) (len 3))) (separator (span (offset 53) (line 2) (column 20) (len 2))) (marker (span (offset 55) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "VehicleModel") (body (part-def (name "Vehicle") (body (attribute-usage (declaration-name "mass") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "DynamicsModel") (body (calc-def) (calc-def) (calc-def) (action-def))) (package (name "AnalysisModel") (body (import (target (span (span (offset 1124) (line 48) (column 18) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 1136) (line 48) (column 30) (len 3))) (separator (span (offset 1136) (line 48) (column 30) (len 2))) (marker (span (offset 1138) (line 48) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1158) (line 49) (column 18) (len 16))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 1171) (line 49) (column 31) (len 3))) (separator (span (offset 1171) (line 49) (column 31) (len 2))) (marker (span (offset 1173) (line 49) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1193) (line 50) (column 18) (len 19))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 1209) (line 50) (column 34) (len 3))) (separator (span (offset 1209) (line 50) (column 34) (len 2))) (marker (span (offset 1211) (line 50) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1231) (line 51) (column 18) (len 21))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1271) (line 52) (column 18) (len 20))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 1288) (line 52) (column 35) (len 3))) (separator (span (offset 1288) (line 52) (column 35) (len 2))) (marker (span (offset 1290) (line 52) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (analysis-case-def))))))
)
~~~
