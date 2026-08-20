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
    attribute def spatialCF : CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
        attribute :>> mRefs = (m, m, m);
    }
    attribute def velocityCF : CartesianVelocity3dCoordinateFrame[1] = spatialCF / s;
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
    (reference r7 (scope relative) (span (offset 342) (line 10) (column 43) (len 9)) (segments (segment 0 (token "TimeScale") (name "TimeScale") (separator none) (span (offset 342) (line 10) (column 43) (len 9)))))
    (reference r8 (scope relative) (span (offset 360) (line 11) (column 7) (len 4)) (segments (segment 0 (token "unit") (name "unit") (separator none) (span (offset 360) (line 11) (column 7) (len 4)))))
    (reference r9 (scope relative) (span (offset 367) (line 11) (column 14) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 367) (line 11) (column 14) (len 1)))))
    (reference r10 (scope relative) (span (offset 386) (line 12) (column 17) (len 17)) (segments (segment 0 (token "definitionalEpoch") (name "definitionalEpoch") (separator none) (span (offset 386) (line 12) (column 17) (len 17)))))
    (reference r11 (scope relative) (span (offset 413) (line 13) (column 8) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 413) (line 13) (column 8) (len 3)))))
    (reference r12 (scope relative) (span (offset 429) (line 14) (column 8) (len 10)) (segments (segment 0 (token "definition") (name "definition") (separator none) (span (offset 429) (line 14) (column 8) (len 10)))))
    (reference r13 (scope relative) (span (offset 514) (line 16) (column 38) (len 15)) (segments (segment 0 (token "Iso8601DateTime") (name "Iso8601DateTime") (separator none) (span (offset 514) (line 16) (column 38) (len 15)))))
    (reference r14 (scope relative) (span (offset 730) (line 20) (column 34) (len 24)) (segments (segment 0 (token "CoordinateFramePlacement") (name "CoordinateFramePlacement") (separator none) (span (offset 730) (line 20) (column 34) (len 24)))))
    (reference r15 (scope relative) (span (offset 713) (line 20) (column 17) (len 14)) (segments (segment 0 (token "transformation") (name "transformation") (separator none) (span (offset 713) (line 20) (column 17) (len 14)))))
    (reference r16 (scope relative) (span (offset 764) (line 21) (column 8) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 764) (line 21) (column 8) (len 6)))))
    (reference r17 (scope relative) (span (offset 773) (line 21) (column 17) (len 3)) (segments (segment 0 (token "UTC") (name "UTC") (separator none) (span (offset 773) (line 21) (column 17) (len 3)))))
    (reference r18 (scope relative) (span (offset 785) (line 22) (column 8) (len 6)) (segments (segment 0 (token "origin") (name "origin") (separator none) (span (offset 785) (line 22) (column 8) (len 6)))))
    (reference r19 (scope relative) (span (offset 794) (line 22) (column 17) (len 22)) (segments (segment 0 (token "definitionalEpochInUTC") (name "definitionalEpochInUTC") (separator none) (span (offset 794) (line 22) (column 17) (len 22)))))
    (reference r20 (scope relative) (span (offset 825) (line 23) (column 8) (len 15)) (segments (segment 0 (token "basisDirections") (name "basisDirections") (separator none) (span (offset 825) (line 23) (column 8) (len 15)))))
    (reference r21 (scope relative) (span (offset 878) (line 27) (column 18) (len 23)) (segments (segment 0 (token "MissionElapsedTimeScale") (name "MissionElapsedTimeScale") (separator none) (span (offset 878) (line 27) (column 18) (len 23)))))
    (reference r22 (scope relative) (span (offset 1027) (line 32) (column 7) (len 22)) (segments (segment 0 (token "definitionalEpochInUTC") (name "definitionalEpochInUTC") (separator none) (span (offset 1027) (line 32) (column 7) (len 22)))))
    (reference r23 (scope relative) (span (offset 1056) (line 32) (column 36) (len 3)) (segments (segment 0 (token "val") (name "val") (separator none) (span (offset 1056) (line 32) (column 36) (len 3)))))
    (reference r24 (scope relative) (span (offset 1142) (line 35) (column 43) (len 16)) (segments (segment 0 (token "TimeInstantValue") (name "TimeInstantValue") (separator none) (span (offset 1142) (line 35) (column 43) (len 16)))))
    (reference r25 (scope relative) (span (offset 1249) (line 40) (column 8) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1249) (line 40) (column 8) (len 4)))))
    (reference r26 (scope relative) (span (offset 1256) (line 40) (column 15) (len 4)) (segments (segment 0 (token "mets") (name "mets") (separator none) (span (offset 1256) (line 40) (column 15) (len 4)))))
    (reference r27 (scope relative) (span (offset 1289) (line 43) (column 23) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 1289) (line 43) (column 23) (len 33)))))
    (reference r28 (scope relative) (span (offset 1526) (line 49) (column 9) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 1526) (line 49) (column 9) (len 5)))))
    (reference r29 (scope relative) (span (offset 1535) (line 49) (column 18) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1535) (line 49) (column 18) (len 1)))))
    (reference r30 (scope relative) (span (offset 1538) (line 49) (column 21) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1538) (line 49) (column 21) (len 1)))))
    (reference r31 (scope relative) (span (offset 1541) (line 49) (column 24) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1541) (line 49) (column 24) (len 1)))))
    (reference r32 (scope relative) (span (offset 1571) (line 51) (column 24) (len 34)) (segments (segment 0 (token "CartesianVelocity3dCoordinateFrame") (name "CartesianVelocity3dCoordinateFrame") (separator none) (span (offset 1571) (line 51) (column 24) (len 34)))))
    (reference r33 (scope relative) (span (offset 1611) (line 51) (column 64) (len 9)) (segments (segment 0 (token "spatialCF") (name "spatialCF") (separator none) (span (offset 1611) (line 51) (column 64) (len 9)))))
    (reference r34 (scope relative) (span (offset 1621) (line 51) (column 74) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 1621) (line 51) (column 74) (len 1)))))
    (reference r35 (scope relative) (span (offset 1685) (line 54) (column 24) (len 25)) (segments (segment 0 (token "CartesianPosition3dVector") (name "CartesianPosition3dVector") (separator none) (span (offset 1685) (line 54) (column 24) (len 25)))))
    (reference r36 (scope relative) (span (offset 1738) (line 55) (column 24) (len 25)) (segments (segment 0 (token "CartesianVelocity3dVector") (name "CartesianVelocity3dVector") (separator none) (span (offset 1738) (line 55) (column 24) (len 25)))))
    (reference r37 (scope relative) (span (offset 1804) (line 58) (column 33) (len 15)) (segments (segment 0 (token "SampledFunction") (name "SampledFunction") (separator none) (span (offset 1804) (line 58) (column 33) (len 15)))))
    (reference r38 (scope relative) (span (offset 1854) (line 59) (column 33) (len 10)) (segments (segment 0 (token "SamplePair") (name "SamplePair") (separator none) (span (offset 1854) (line 59) (column 33) (len 10)))))
    (reference r39 (scope relative) (span (offset 1897) (line 60) (column 31) (len 23)) (segments (segment 0 (token "MissionElapsedTimeValue") (name "MissionElapsedTimeValue") (separator none) (span (offset 1897) (line 60) (column 31) (len 23)))))
    (reference r40 (scope relative) (span (offset 1884) (line 60) (column 18) (len 11)) (segments (segment 0 (token "domainValue") (name "domainValue") (separator none) (span (offset 1884) (line 60) (column 18) (len 11)))))
    (reference r41 (scope relative) (span (offset 1954) (line 61) (column 30) (len 19)) (segments (segment 0 (token "PositionAndVelocity") (name "PositionAndVelocity") (separator none) (span (offset 1954) (line 61) (column 30) (len 19)))))
    (reference r42 (scope relative) (span (offset 1942) (line 61) (column 18) (len 10)) (segments (segment 0 (token "rangeValue") (name "rangeValue") (separator none) (span (offset 1942) (line 61) (column 18) (len 10)))))
    (reference r43 (scope relative) (span (offset 2007) (line 63) (column 26) (len 12)) (segments (segment 0 (token "AscentSample") (name "AscentSample") (separator none) (span (offset 2007) (line 63) (column 26) (len 12)))))
    (reference r44 (scope relative) (span (offset 1998) (line 63) (column 17) (len 7)) (segments (segment 0 (token "samples") (name "samples") (separator none) (span (offset 1998) (line 63) (column 17) (len 7)))))
    (reference r45 (scope relative) (span (offset 2063) (line 66) (column 28) (len 13)) (segments (segment 0 (token "AscentProfile") (name "AscentProfile") (separator none) (span (offset 2063) (line 66) (column 28) (len 13)))))
    (reference r46 (scope relative) (span (offset 2135) (line 68) (column 22) (len 12)) (segments (segment 0 (token "AscentSample") (name "AscentSample") (separator none) (span (offset 2135) (line 68) (column 22) (len 12)))))
    (reference r47 (scope relative) (span (offset 2154) (line 68) (column 41) (len 11)) (segments (segment 0 (token "domainValue") (name "domainValue") (separator none) (span (offset 2154) (line 68) (column 41) (len 11)))))
    (reference r48 (scope relative) (span (offset 2184) (line 68) (column 71) (len 10)) (segments (segment 0 (token "rangeValue") (name "rangeValue") (separator none) (span (offset 2184) (line 68) (column 71) (len 10)))))
    (reference r49 (scope relative) (span (offset 2197) (line 68) (column 84) (len 3)) (segments (segment 0 (token "pv1") (name "pv1") (separator none) (span (offset 2197) (line 68) (column 84) (len 3)))))
    (reference r50 (scope relative) (span (offset 2220) (line 69) (column 19) (len 19)) (segments (segment 0 (token "PositionAndVelocity") (name "PositionAndVelocity") (separator none) (span (offset 2220) (line 69) (column 19) (len 19)))))
    (reference r51 (scope relative) (span (offset 2245) (line 69) (column 44) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 2245) (line 69) (column 44) (len 8)))))
    (reference r52 (scope relative) (span (offset 2283) (line 69) (column 82) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 2283) (line 69) (column 82) (len 8)))))
    (reference r53 (scope relative) (span (offset 2343) (line 70) (column 22) (len 12)) (segments (segment 0 (token "AscentSample") (name "AscentSample") (separator none) (span (offset 2343) (line 70) (column 22) (len 12)))))
    (reference r54 (scope relative) (span (offset 2362) (line 70) (column 41) (len 11)) (segments (segment 0 (token "domainValue") (name "domainValue") (separator none) (span (offset 2362) (line 70) (column 41) (len 11)))))
    (reference r55 (scope relative) (span (offset 2392) (line 70) (column 71) (len 10)) (segments (segment 0 (token "rangeValue") (name "rangeValue") (separator none) (span (offset 2392) (line 70) (column 71) (len 10)))))
    (reference r56 (scope relative) (span (offset 2405) (line 70) (column 84) (len 3)) (segments (segment 0 (token "pv1") (name "pv1") (separator none) (span (offset 2405) (line 70) (column 84) (len 3)))))
    (reference r57 (scope relative) (span (offset 2428) (line 71) (column 19) (len 19)) (segments (segment 0 (token "PositionAndVelocity") (name "PositionAndVelocity") (separator none) (span (offset 2428) (line 71) (column 19) (len 19)))))
    (reference r58 (scope relative) (span (offset 2453) (line 71) (column 44) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 2453) (line 71) (column 44) (len 8)))))
    (reference r59 (scope relative) (span (offset 2499) (line 71) (column 90) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 2499) (line 71) (column 90) (len 8)))))
    (reference r60 (scope relative) (span (offset 2561) (line 72) (column 22) (len 12)) (segments (segment 0 (token "AscentSample") (name "AscentSample") (separator none) (span (offset 2561) (line 72) (column 22) (len 12)))))
    (reference r61 (scope relative) (span (offset 2580) (line 72) (column 41) (len 11)) (segments (segment 0 (token "domainValue") (name "domainValue") (separator none) (span (offset 2580) (line 72) (column 41) (len 11)))))
    (reference r62 (scope relative) (span (offset 2610) (line 72) (column 71) (len 10)) (segments (segment 0 (token "rangeValue") (name "rangeValue") (separator none) (span (offset 2610) (line 72) (column 71) (len 10)))))
    (reference r63 (scope relative) (span (offset 2623) (line 72) (column 84) (len 3)) (segments (segment 0 (token "pv1") (name "pv1") (separator none) (span (offset 2623) (line 72) (column 84) (len 3)))))
    (reference r64 (scope relative) (span (offset 2646) (line 73) (column 19) (len 19)) (segments (segment 0 (token "PositionAndVelocity") (name "PositionAndVelocity") (separator none) (span (offset 2646) (line 73) (column 19) (len 19)))))
    (reference r65 (scope relative) (span (offset 2671) (line 73) (column 44) (len 8)) (segments (segment 0 (token "position") (name "position") (separator none) (span (offset 2671) (line 73) (column 44) (len 8)))))
    (reference r66 (scope relative) (span (offset 2718) (line 73) (column 91) (len 8)) (segments (segment 0 (token "velocity") (name "velocity") (separator none) (span (offset 2718) (line 73) (column 91) (len 8)))))
    (reference r67 (scope relative) (span (offset 2782) (line 74) (column 17) (len 7)) (segments (segment 0 (token "samples") (name "samples") (separator none) (span (offset 2782) (line 74) (column 17) (len 7)))))
    (reference r68 (scope relative) (span (offset 2793) (line 74) (column 28) (len 7)) (segments (segment 0 (token "sample1") (name "sample1") (separator none) (span (offset 2793) (line 74) (column 28) (len 7)))))
    (reference r69 (scope relative) (span (offset 2802) (line 74) (column 37) (len 7)) (segments (segment 0 (token "sample2") (name "sample2") (separator none) (span (offset 2802) (line 74) (column 37) (len 7)))))
    (reference r70 (scope relative) (span (offset 2811) (line 74) (column 46) (len 7)) (segments (segment 0 (token "sample3") (name "sample3") (separator none) (span (offset 2811) (line 74) (column 46) (len 7)))))
  )
  (root (package (name "15_13-Discretely Sampled Function Value") (body brace (import (target (span (span (offset 68) (line 2) (column 17) (len 33))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 119) (line 3) (column 17) (len 28))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 165) (line 4) (column 17) (len 18))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 201) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 204) (line 5) (column 20) (len 3))) (separator (span (offset 204) (line 5) (column 20) (len 2))) (marker (span (offset 206) (line 5) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 225) (line 6) (column 17) (len 5))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 227) (line 6) (column 19) (len 3))) (separator (span (offset 227) (line 6) (column 19) (len 2))) (marker (span (offset 229) (line 6) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 248) (line 7) (column 17) (len 24))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 269) (line 7) (column 38) (len 3))) (separator (span (offset 269) (line 7) (column 38) (len 2))) (marker (span (offset 271) (line 7) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 290) (line 8) (column 17) (len 7))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 294) (line 8) (column 21) (len 3))) (separator (span (offset 294) (line 8) (column 21) (len 2))) (marker (span (offset 296) (line 8) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "MissionElapsedTimeScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 367) (line 11) (column 14) (len 1)) (ref r9))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 419) (line 13) (column 14) (len 1)) (integer 0))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 442) (line 14) (column 21) (len 29)) (string "time instant zero at launch"))))) (body semicolon)))) (attribute-usage (declaration-name "definitionalEpochInUTC") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 773) (line 21) (column 17) (len 3)) (ref r17))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 794) (line 22) (column 17) (len 22)) (ref r19))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 843) (line 23) (column 26) (len 7)) (literal-with-unit (value (expression (span (offset 843) (line 23) (column 26) (len 1)) (integer 1))) (unit (expression (span (offset 846) (line 23) (column 29) (len 3)) (bracket (expression (span (offset 846) (line 23) (column 29) (len 3)) (unit "UTC")))))))))) (body semicolon)))))) (attribute-def (declaration-name "mets") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 915) (line 29) (column 5) (len 103)) (normalized "Define mission elapsed time scale starting at given UTC date time (in microsecond resolution)\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1062) (line 32) (column 42) (len 29)) (string "2020-08-23T22:42:32.924534Z"))))) (body semicolon)))))) (attribute-def (declaration-name "MissionElapsedTimeValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1171) (line 37) (column 5) (len 68)) (normalized "Define scalar quantity value type for mission elapsed time\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1256) (line 40) (column 15) (len 4)) (ref r26))))) (body semicolon)))) (attribute-def (declaration-name "spatialCF") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity (lower (expression (span (offset 1323) (line 43) (column 57) (len 1)) (integer 1))) (upper (expression (span (offset 1323) (line 43) (column 57) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1338) (line 45) (column 5) (len 177)) (normalized "Define Cartesian 3D coordinate systems for position and velocity\nCreate a velocity coordinate system from the spatial coordinate system through division by second\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1534) (line 49) (column 17) (len 9)) (tuple (expression (span (offset 1535) (line 49) (column 18) (len 1)) (ref r29)) (expression (span (offset 1538) (line 49) (column 21) (len 1)) (ref r30)) (expression (span (offset 1541) (line 49) (column 24) (len 1)) (ref r31))))))) (body semicolon)))) (attribute-def (declaration-name "velocityCF") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r32)))) (multiplicity (lower (expression (span (offset 1606) (line 51) (column 59) (len 1)) (integer 1))) (upper (expression (span (offset 1606) (line 51) (column 59) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1611) (line 51) (column 64) (len 11)) (binary (operator "/") (left (expression (span (offset 1611) (line 51) (column 64) (len 9)) (ref r33))) (right (expression (span (offset 1621) (line 51) (column 74) (len 1)) (ref r34)))))))) (body semicolon)) (attribute-def (declaration-name "PositionAndVelocity") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "position") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "velocity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "AscentProfile") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-def (declaration-name "AscentSample") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r41)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r43)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r44)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "ascentProfile1") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2087) (line 67) (column 9) (len 24)) (normalized "Example ascent profile "))) (attribute-usage (declaration-name "sample1") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2168) (line 68) (column 55) (len 10)) (literal-with-unit (value (expression (span (offset 2168) (line 68) (column 55) (len 3)) (real "0.0"))) (unit (expression (span (offset 2173) (line 68) (column 60) (len 4)) (bracket (expression (span (offset 2173) (line 68) (column 60) (len 4)) (unit "mets")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r48)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2197) (line 68) (column 84) (len 3)) (ref r49))))) (body semicolon)) (attribute-usage (declaration-name "pv1") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r50)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2256) (line 69) (column 55) (len 21)) (literal-with-unit (value (expression (span (offset 2256) (line 69) (column 55) (len 9)) (tuple (expression (span (offset 2257) (line 69) (column 56) (len 1)) (integer 0)) (expression (span (offset 2260) (line 69) (column 59) (len 1)) (integer 0)) (expression (span (offset 2263) (line 69) (column 62) (len 1)) (integer 0))))) (unit (expression (span (offset 2267) (line 69) (column 66) (len 9)) (bracket (expression (span (offset 2267) (line 69) (column 66) (len 9)) (unit "spatialCF")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2294) (line 69) (column 93) (len 22)) (literal-with-unit (value (expression (span (offset 2294) (line 69) (column 93) (len 9)) (tuple (expression (span (offset 2295) (line 69) (column 94) (len 1)) (integer 0)) (expression (span (offset 2298) (line 69) (column 97) (len 1)) (integer 0)) (expression (span (offset 2301) (line 69) (column 100) (len 1)) (integer 0))))) (unit (expression (span (offset 2305) (line 69) (column 104) (len 10)) (bracket (expression (span (offset 2305) (line 69) (column 104) (len 10)) (unit "velocityCF")))))))))) (body semicolon)))))) (attribute-usage (declaration-name "sample2") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r54)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2376) (line 70) (column 55) (len 10)) (literal-with-unit (value (expression (span (offset 2376) (line 70) (column 55) (len 3)) (real "2.5"))) (unit (expression (span (offset 2381) (line 70) (column 60) (len 4)) (bracket (expression (span (offset 2381) (line 70) (column 60) (len 4)) (unit "mets")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r55)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2405) (line 70) (column 84) (len 3)) (ref r56))))) (body semicolon)) (attribute-usage (declaration-name "pv1") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2464) (line 71) (column 55) (len 29)) (literal-with-unit (value (expression (span (offset 2464) (line 71) (column 55) (len 17)) (tuple (expression (span (offset 2465) (line 71) (column 56) (len 4)) (real "0.01")) (expression (span (offset 2471) (line 71) (column 62) (len 4)) (real "0.03")) (expression (span (offset 2477) (line 71) (column 68) (len 3)) (real "8.6"))))) (unit (expression (span (offset 2483) (line 71) (column 74) (len 9)) (bracket (expression (span (offset 2483) (line 71) (column 74) (len 9)) (unit "spatialCF")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2510) (line 71) (column 101) (len 24)) (literal-with-unit (value (expression (span (offset 2510) (line 71) (column 101) (len 11)) (tuple (expression (span (offset 2511) (line 71) (column 102) (len 1)) (integer 0)) (expression (span (offset 2514) (line 71) (column 105) (len 1)) (integer 0)) (expression (span (offset 2517) (line 71) (column 108) (len 3)) (real "5.5"))))) (unit (expression (span (offset 2523) (line 71) (column 114) (len 10)) (bracket (expression (span (offset 2523) (line 71) (column 114) (len 10)) (unit "velocityCF")))))))))) (body semicolon)))))) (attribute-usage (declaration-name "sample3") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r60)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r61)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2594) (line 72) (column 55) (len 10)) (literal-with-unit (value (expression (span (offset 2594) (line 72) (column 55) (len 3)) (real "5.1"))) (unit (expression (span (offset 2599) (line 72) (column 60) (len 4)) (bracket (expression (span (offset 2599) (line 72) (column 60) (len 4)) (unit "mets")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r62)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2623) (line 72) (column 84) (len 3)) (ref r63))))) (body semicolon)) (attribute-usage (declaration-name "pv1") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r64)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2682) (line 73) (column 55) (len 30)) (literal-with-unit (value (expression (span (offset 2682) (line 73) (column 55) (len 18)) (tuple (expression (span (offset 2683) (line 73) (column 56) (len 4)) (real "0.04")) (expression (span (offset 2689) (line 73) (column 62) (len 4)) (real "0.12")) (expression (span (offset 2695) (line 73) (column 68) (len 4)) (real "18.6"))))) (unit (expression (span (offset 2702) (line 73) (column 75) (len 9)) (bracket (expression (span (offset 2702) (line 73) (column 75) (len 9)) (unit "spatialCF")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2729) (line 73) (column 102) (len 31)) (literal-with-unit (value (expression (span (offset 2729) (line 73) (column 102) (len 18)) (tuple (expression (span (offset 2730) (line 73) (column 103) (len 4)) (real "0.05")) (expression (span (offset 2736) (line 73) (column 109) (len 4)) (real "0.03")) (expression (span (offset 2742) (line 73) (column 115) (len 4)) (real "25.3"))))) (unit (expression (span (offset 2749) (line 73) (column 122) (len 10)) (bracket (expression (span (offset 2749) (line 73) (column 122) (len 10)) (unit "velocityCF")))))))))) (body semicolon)))))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r67)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2792) (line 74) (column 27) (len 27)) (tuple (expression (span (offset 2793) (line 74) (column 28) (len 7)) (ref r68)) (expression (span (offset 2802) (line 74) (column 37) (len 7)) (ref r69)) (expression (span (offset 2811) (line 74) (column 46) (len 7)) (ref r70))))))) (body semicolon)))))))
)
~~~
