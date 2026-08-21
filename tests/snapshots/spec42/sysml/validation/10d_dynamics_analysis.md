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
            for i in 1 .. powerProfile->size() - 1 {
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
    (reference r2 (scope relative) (span (offset 726) (line 36) (column 21) (len 10)) (segments (segment 0 (token "PowerValue") (name "PowerValue") (separator none) (span (offset 726) (line 36) (column 21) (len 10)))))
    (reference r3 (scope relative) (span (offset 757) (line 37) (column 20) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 757) (line 37) (column 20) (len 9)))))
    (reference r4 (scope relative) (span (offset 790) (line 38) (column 23) (len 9)) (segments (segment 0 (token "TimeValue") (name "TimeValue") (separator none) (span (offset 790) (line 38) (column 23) (len 9)))))
    (reference r5 (scope relative) (span (offset 820) (line 39) (column 20) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 820) (line 39) (column 20) (len 11)))))
    (reference r6 (scope relative) (span (offset 852) (line 40) (column 20) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 852) (line 40) (column 20) (len 10)))))
    (reference r7 (scope relative) (span (offset 885) (line 41) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 885) (line 41) (column 22) (len 11)))))
    (reference r8 (scope relative) (span (offset 899) (line 41) (column 36) (len 8)) (segments (segment 0 (token "Position") (name "Position") (separator none) (span (offset 899) (line 41) (column 36) (len 8)))))
    (reference r9 (scope relative) (span (offset 908) (line 41) (column 45) (len 4)) (segments (segment 0 (token "x_in") (name "x_in") (separator none) (span (offset 908) (line 41) (column 45) (len 4)))))
    (reference r10 (scope relative) (span (offset 914) (line 41) (column 51) (len 4)) (segments (segment 0 (token "v_in") (name "v_in") (separator none) (span (offset 914) (line 41) (column 51) (len 4)))))
    (reference r11 (scope relative) (span (offset 920) (line 41) (column 57) (len 7)) (segments (segment 0 (token "delta_t") (name "delta_t") (separator none) (span (offset 920) (line 41) (column 57) (len 7)))))
    (reference r12 (scope relative) (span (offset 951) (line 42) (column 22) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 951) (line 42) (column 22) (len 10)))))
    (reference r13 (scope relative) (span (offset 964) (line 42) (column 35) (len 8)) (segments (segment 0 (token "Velocity") (name "Velocity") (separator none) (span (offset 964) (line 42) (column 35) (len 8)))))
    (reference r14 (scope relative) (span (offset 973) (line 42) (column 44) (len 4)) (segments (segment 0 (token "v_in") (name "v_in") (separator none) (span (offset 973) (line 42) (column 44) (len 4)))))
    (reference r15 (scope relative) (span (offset 979) (line 42) (column 50) (len 5)) (segments (segment 0 (token "a_out") (name "a_out") (separator none) (span (offset 979) (line 42) (column 50) (len 5)))))
    (reference r16 (scope relative) (span (offset 986) (line 42) (column 57) (len 7)) (segments (segment 0 (token "delta_t") (name "delta_t") (separator none) (span (offset 986) (line 42) (column 57) (len 7)))))
    (reference r17 (scope relative) (span (offset 1017) (line 43) (column 22) (len 17)) (segments (segment 0 (token "AccelerationValue") (name "AccelerationValue") (separator none) (span (offset 1017) (line 43) (column 22) (len 17)))))
    (reference r18 (scope relative) (span (offset 1037) (line 43) (column 42) (len 12)) (segments (segment 0 (token "Acceleration") (name "Acceleration") (separator none) (span (offset 1037) (line 43) (column 42) (len 12)))))
    (reference r19 (scope relative) (span (offset 1050) (line 43) (column 55) (len 5)) (segments (segment 0 (token "power") (name "power") (separator none) (span (offset 1050) (line 43) (column 55) (len 5)))))
    (reference r20 (scope relative) (span (offset 1057) (line 43) (column 62) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 1057) (line 43) (column 62) (len 4)))))
    (reference r21 (scope relative) (span (offset 1063) (line 43) (column 68) (len 4)) (segments (segment 0 (token "v_in") (name "v_in") (separator none) (span (offset 1063) (line 43) (column 68) (len 4)))))
    (reference r22 (scope relative) (span (offset 1124) (line 48) (column 18) (len 12)) (segments (segment 0 (token "VehicleModel") (name "VehicleModel") (separator none) (span (offset 1124) (line 48) (column 18) (len 12)))))
    (reference r23 (scope relative) (span (offset 1158) (line 49) (column 18) (len 13)) (segments (segment 0 (token "DynamicsModel") (name "DynamicsModel") (separator none) (span (offset 1158) (line 49) (column 18) (len 13)))))
    (reference r24 (scope relative) (span (offset 1193) (line 50) (column 18) (len 16)) (segments (segment 0 (token "SampledFunctions") (name "SampledFunctions") (separator none) (span (offset 1193) (line 50) (column 18) (len 16)))))
    (reference r25 (scope relative) (span (offset 1231) (line 51) (column 18) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 1231) (line 51) (column 18) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 1245) (line 51) (column 32) (len 7)))))
    (reference r26 (scope relative) (span (offset 1271) (line 52) (column 18) (len 17)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 1271) (line 52) (column 18) (len 17)))))
  )
  (root (package (name "10d-Dynamics Analysis") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 53) (line 2) (column 20) (len 3))) (separator (span (offset 53) (line 2) (column 20) (len 2))) (marker (span (offset 55) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "VehicleModel") (body brace (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "DynamicsModel") (body brace (calc-def (name "Acceleration") (modifiers) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))) (calc-def (name "Velocity") (modifiers) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))) (calc-def (name "Position") (modifiers) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))) (action-def (name "StraightLineDynamics") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "power") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 715) (line 36) (column 10) (len 22))) (in-out (direction in) (reference false) (declaration "mass") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 747) (line 37) (column 10) (len 20))) (in-out (direction in) (reference false) (declaration "delta_t") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 777) (line 38) (column 10) (len 23))) (in-out (direction in) (reference false) (declaration "x_in") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 810) (line 39) (column 10) (len 22))) (in-out (direction in) (reference false) (declaration "v_in") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 842) (line 40) (column 10) (len 21))) (in-out (direction out) (reference false) (declaration "x_out") (subsets none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 899) (line 41) (column 36) (len 29)) (invocation (callee (expression (span (offset 899) (line 41) (column 36) (len 8)) (ref r8))) (arguments (argument (parameter none) (value (expression (span (offset 908) (line 41) (column 45) (len 4)) (ref r9)))) (argument (parameter none) (value (expression (span (offset 914) (line 41) (column 51) (len 4)) (ref r10)))) (argument (parameter none) (value (expression (span (offset 920) (line 41) (column 57) (len 7)) (ref r11)))))))))) (span (offset 873) (line 41) (column 10) (len 56))) (in-out (direction out) (reference false) (declaration "v_out") (subsets none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 964) (line 42) (column 35) (len 30)) (invocation (callee (expression (span (offset 964) (line 42) (column 35) (len 8)) (ref r13))) (arguments (argument (parameter none) (value (expression (span (offset 973) (line 42) (column 44) (len 4)) (ref r14)))) (argument (parameter none) (value (expression (span (offset 979) (line 42) (column 50) (len 5)) (ref r15)))) (argument (parameter none) (value (expression (span (offset 986) (line 42) (column 57) (len 7)) (ref r16)))))))))) (span (offset 939) (line 42) (column 10) (len 56))) (in-out (direction out) (reference false) (declaration "a_out") (subsets none) (type (ref r17)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1037) (line 43) (column 42) (len 31)) (invocation (callee (expression (span (offset 1037) (line 43) (column 42) (len 12)) (ref r18))) (arguments (argument (parameter none) (value (expression (span (offset 1050) (line 43) (column 55) (len 5)) (ref r19)))) (argument (parameter none) (value (expression (span (offset 1057) (line 43) (column 62) (len 4)) (ref r20)))) (argument (parameter none) (value (expression (span (offset 1063) (line 43) (column 68) (len 4)) (ref r21)))))))))) (span (offset 1005) (line 43) (column 10) (len 64))))))) (package (name "AnalysisModel") (body brace (import (target (span (span (offset 1124) (line 48) (column 18) (len 15))) (all none) (ref r22) (shape (namespace (wildcard-suffix (span (span (offset 1136) (line 48) (column 30) (len 3))) (separator (span (offset 1136) (line 48) (column 30) (len 2))) (marker (span (offset 1138) (line 48) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1158) (line 49) (column 18) (len 16))) (all none) (ref r23) (shape (namespace (wildcard-suffix (span (span (offset 1171) (line 49) (column 31) (len 3))) (separator (span (offset 1171) (line 49) (column 31) (len 2))) (marker (span (offset 1173) (line 49) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1193) (line 50) (column 18) (len 19))) (all none) (ref r24) (shape (namespace (wildcard-suffix (span (span (offset 1209) (line 50) (column 34) (len 3))) (separator (span (offset 1209) (line 50) (column 34) (len 2))) (marker (span (offset 1211) (line 50) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1231) (line 51) (column 18) (len 21))) (all none) (ref r25) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1271) (line 52) (column 18) (len 20))) (all none) (ref r26) (shape (namespace (wildcard-suffix (span (span (offset 1288) (line 52) (column 35) (len 3))) (separator (span (offset 1288) (line 52) (column 35) (len 2))) (marker (span (offset 1290) (line 52) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (analysis-case-def (modifiers)))))))
)
~~~
