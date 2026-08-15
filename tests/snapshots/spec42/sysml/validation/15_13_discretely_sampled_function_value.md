# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_13-Discretely Sampled Function Value"))
~~~
# SOURCE
~~~sysml
package '15_13-Discretely Sampled Function Value' {
	private import SampledFunctions::SampledFunction;
	private import SampledFunctions::SamplePair;
	private import Collections::Array;
	private import ISQ::*;
	private import SI::*;
	private import MeasurementReferences::*;
	private import Time::*;

	attribute def MissionElapsedTimeScale :> TimeScale {
		:>> unit = s;
		attribute :>> definitionalEpoch {
			:>> num = 0;
			:>> definition = "time instant zero at launch";
		}
		attribute definitionalEpochInUTC : Iso8601DateTime;
		
		// Map the definitional epoch (t = 0) of this scale to a reference epoch expressed in UTC
		// This modeled as a 1D coordinate transformation (translation only)
		attribute :>> transformation : CoordinateFramePlacement {
			:>> source = UTC;
			:>> origin = definitionalEpochInUTC;
			:>> basisDirections = 1 [UTC];
		}
  }

	attribute mets: MissionElapsedTimeScale { 
		doc
		/*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
		:>> definitionalEpochInUTC { :>> val = "2020-08-23T22:42:32.924534Z";}		
	}

	attribute def MissionElapsedTimeValue :> TimeInstantValue {
		doc
		/*
		 * Define scalar quantity value type for mission elapsed time
		 */
	 	:>> mRef = mets; 
	}

	attribute spatialCF: CartesianSpatial3dCoordinateFrame[1] {
		doc
		/*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
	   :>> mRefs = (m, m, m);
	}
	attribute velocityCF: CartesianVelocity3dCoordinateFrame[1] = spatialCF/s;

	attribute def PositionAndVelocity {
		attribute position : CartesianPosition3dVector[1];
		attribute velocity : CartesianVelocity3dVector[1];
	}

	attribute def AscentProfile :> SampledFunction {
		attribute def AscentSample :> SamplePair {
			attribute :>> domainValue: MissionElapsedTimeValue[1];
			attribute :>> rangeValue: PositionAndVelocity[1];
		}
		attribute :>> samples: AscentSample[*] ordered;
	}

	attribute ascentProfile1: AscentProfile {
		doc /* Example ascent profile */
		attribute sample1: AscentSample { :>> domainValue = 0.0 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0, 0, 0) [spatialCF]; :>> velocity = (0, 0, 0) [velocityCF]; } }
		attribute sample2: AscentSample { :>> domainValue = 2.5 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.01, 0.03, 8.6) [spatialCF]; :>> velocity = (0, 0, 5.5) [velocityCF]; } }
		attribute sample3: AscentSample { :>> domainValue = 5.1 [mets]; :>> rangeValue = pv1;
			attribute pv1: PositionAndVelocity {:>> position = (0.04, 0.12, 18.6) [spatialCF]; :>> velocity = (0.05, 0.03, 25.3) [velocityCF]; } }
		attribute :>> samples = (sample1, sample2, sample3);
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_13_discretely_sampled_function_value.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_13-Discretely Sampled Function Value' {
    private import SampledFunctions::SampledFunction;
    private import SampledFunctions::SamplePair;
    private import Collections::Array;
    private import ISQ::*;
    private import SI::*;
    private import MeasurementReferences::*;
    private import Time::*;
    attribute def MissionElapsedTimeScale :> TimeScale {
        attribute :>> unit = s;
        attribute :>> definitionalEpoch {
            attribute :>> num = 0;
            attribute :>> definition = "time instant zero at launch";
        }
        attribute definitionalEpochInUTC : Iso8601DateTime;
        attribute :>> transformation : CoordinateFramePlacement {
            attribute :>> source = UTC;
            attribute :>> origin = definitionalEpochInUTC;
            attribute :>> basisDirections = 1 [UTC];
        }
    }
    attribute def mets : MissionElapsedTimeScale {
        doc
        /*
		 * Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)
		 */
        attribute :>> definitionalEpochInUTC {
            attribute :>> val = "2020-08-23T22:42:32.924534Z";
        }
    }
    attribute def MissionElapsedTimeValue :> TimeInstantValue {
        doc
        /*
		 * Define scalar quantity value type for mission elapsed time
		 */
        attribute :>> mRef = mets;
    }
    attribute def spatialCF : CartesianSpatial3dCoordinateFrame {
        doc
        /*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
        attribute :>> mRefs = (m, m, m);
    }
    attribute def velocityCF : CartesianVelocity3dCoordinateFrame = spatialCF / s;
    attribute def PositionAndVelocity {
        attribute position : CartesianPosition3dVector[1];
        attribute velocity : CartesianVelocity3dVector[1];
    }
    attribute def AscentProfile :> SampledFunction {
        attribute def AscentSample :> SamplePair {
            attribute :>> domainValue : MissionElapsedTimeValue[1];
            attribute :>> rangeValue : PositionAndVelocity[1];
        }
        attribute :>> samples : AscentSample[*] ordered;
    }
    attribute def ascentProfile1 : AscentProfile {
        doc
        /* Example ascent profile */
        attribute sample1 : AscentSample {
            attribute :>> domainValue = 0.0 [mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0, 0, 0) [spatialCF];
                attribute :>> velocity = (0, 0, 0) [velocityCF];
            }
        }
        attribute sample2 : AscentSample {
            attribute :>> domainValue = 2.5 [mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0.01, 0.03, 8.6) [spatialCF];
                attribute :>> velocity = (0, 0, 5.5) [velocityCF];
            }
        }
        attribute sample3 : AscentSample {
            attribute :>> domainValue = 5.1 [mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0.04, 0.12, 18.6) [spatialCF];
                attribute :>> velocity = (0.05, 0.03, 25.3) [velocityCF];
            }
        }
        attribute :>> samples = (sample1, sample2, sample3);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 68) (line 2) (column 17) (len 33)) (segments (segment 0 (token "SampledFunctions") (name "SampledFunctions") (separator none) (span (offset 68) (line 2) (column 17) (len 16))) (segment 1 (token "SampledFunction") (name "SampledFunction") (separator colon-colon) (span (offset 86) (line 2) (column 35) (len 15)))))
    (reference r1 (scope relative) (span (offset 119) (line 3) (column 17) (len 28)) (segments (segment 0 (token "SampledFunctions") (name "SampledFunctions") (separator none) (span (offset 119) (line 3) (column 17) (len 16))) (segment 1 (token "SamplePair") (name "SamplePair") (separator colon-colon) (span (offset 137) (line 3) (column 35) (len 10)))))
    (reference r2 (scope relative) (span (offset 165) (line 4) (column 17) (len 18)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 165) (line 4) (column 17) (len 11))) (segment 1 (token "Array") (name "Array") (separator colon-colon) (span (offset 178) (line 4) (column 30) (len 5)))))
    (reference r3 (scope relative) (span (offset 201) (line 5) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 201) (line 5) (column 17) (len 3)))))
    (reference r4 (scope relative) (span (offset 225) (line 6) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 225) (line 6) (column 17) (len 2)))))
    (reference r5 (scope relative) (span (offset 248) (line 7) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 248) (line 7) (column 17) (len 21)))))
    (reference r6 (scope relative) (span (offset 290) (line 8) (column 17) (len 4)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 290) (line 8) (column 17) (len 4)))))
  )
  (root (package (name "15_13-Discretely Sampled Function Value") (body brace (import (target (span (span (offset 68) (line 2) (column 17) (len 33))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 119) (line 3) (column 17) (len 28))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 165) (line 4) (column 17) (len 18))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 201) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 204) (line 5) (column 20) (len 3))) (separator (span (offset 204) (line 5) (column 20) (len 2))) (marker (span (offset 206) (line 5) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 225) (line 6) (column 17) (len 5))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 227) (line 6) (column 19) (len 3))) (separator (span (offset 227) (line 6) (column 19) (len 2))) (marker (span (offset 229) (line 6) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 248) (line 7) (column 17) (len 24))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 269) (line 7) (column 38) (len 3))) (separator (span (offset 269) (line 7) (column 38) (len 2))) (marker (span (offset 271) (line 7) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 290) (line 8) (column 17) (len 7))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 294) (line 8) (column 21) (len 3))) (separator (span (offset 294) (line 8) (column 21) (len 2))) (marker (span (offset 296) (line 8) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def))))
)
~~~
