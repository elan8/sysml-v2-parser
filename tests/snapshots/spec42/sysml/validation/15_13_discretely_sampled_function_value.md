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
            attribute :>> basisDirections = 1[UTC];
        }
    }
    attribute mets : MissionElapsedTimeScale {
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
    attribute spatialCF : CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
		 * Define Cartesian 3D coordinate systems for position and velocity
		 * Create a velocity coordinate system from the spatial coordinate system through division by second
		 */
        attribute :>> mRefs = (m, m, m);
    }
    attribute velocityCF : CartesianVelocity3dCoordinateFrame[1] = spatialCF / s;
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
    attribute ascentProfile1 : AscentProfile {
        doc
        /* Example ascent profile */
        attribute sample1 : AscentSample {
            attribute :>> domainValue = 0.0[mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0, 0, 0)[spatialCF];
                attribute :>> velocity = (0, 0, 0)[velocityCF];
            }
        }
        attribute sample2 : AscentSample {
            attribute :>> domainValue = 2.5[mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0.01, 0.03, 8.6)[spatialCF];
                attribute :>> velocity = (0, 0, 5.5)[velocityCF];
            }
        }
        attribute sample3 : AscentSample {
            attribute :>> domainValue = 5.1[mets];
            attribute :>> rangeValue = pv1;
            attribute pv1 : PositionAndVelocity {
                attribute :>> position = (0.04, 0.12, 18.6)[spatialCF];
                attribute :>> velocity = (0.05, 0.03, 25.3)[velocityCF];
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
    (reference r21 (scope relative) (span (offset 846) (line 23) (column 29) (len 3)) (segments (segment 0 (token "UTC") (name "UTC") (separator none) (span (offset 846) (line 23) (column 29) (len 3)))))
    (reference r22 (scope relative) (span (offset 1142) (line 35) (column 43) (len 16)) (segments (segment 0 (token "TimeInstantValue") (name "TimeInstantValue") (separator none) (span (offset 1142) (line 35) (column 43) (len 16)))))
    (reference r23 (scope relative) (span (offset 1249) (line 40) (column 8) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1249) (line 40) (column 8) (len 4)))))
    (reference r24 (scope relative) (span (offset 1256) (line 40) (column 15) (len 4)) (segments (segment 0 (token "mets") (name "mets") (separator none) (span (offset 1256) (line 40) (column 15) (len 4)))))
    (reference r25 (scope relative) (span (offset 1685) (line 54) (column 24) (len 25)) (segments (segment 0 (token "CartesianPosition3dVector") (name "CartesianPosition3dVector") (separator none) (span (offset 1685) (line 54) (column 24) (len 25)))))
    (reference r26 (scope relative) (span (offset 1738) (line 55) (column 24) (len 25)) (segments (segment 0 (token "CartesianVelocity3dVector") (name "CartesianVelocity3dVector") (separator none) (span (offset 1738) (line 55) (column 24) (len 25)))))
    (reference r27 (scope relative) (span (offset 1804) (line 58) (column 33) (len 15)) (segments (segment 0 (token "SampledFunction") (name "SampledFunction") (separator none) (span (offset 1804) (line 58) (column 33) (len 15)))))
    (reference r28 (scope relative) (span (offset 1854) (line 59) (column 33) (len 10)) (segments (segment 0 (token "SamplePair") (name "SamplePair") (separator none) (span (offset 1854) (line 59) (column 33) (len 10)))))
    (reference r29 (scope relative) (span (offset 1897) (line 60) (column 31) (len 23)) (segments (segment 0 (token "MissionElapsedTimeValue") (name "MissionElapsedTimeValue") (separator none) (span (offset 1897) (line 60) (column 31) (len 23)))))
    (reference r30 (scope relative) (span (offset 1884) (line 60) (column 18) (len 11)) (segments (segment 0 (token "domainValue") (name "domainValue") (separator none) (span (offset 1884) (line 60) (column 18) (len 11)))))
    (reference r31 (scope relative) (span (offset 1954) (line 61) (column 30) (len 19)) (segments (segment 0 (token "PositionAndVelocity") (name "PositionAndVelocity") (separator none) (span (offset 1954) (line 61) (column 30) (len 19)))))
    (reference r32 (scope relative) (span (offset 1942) (line 61) (column 18) (len 10)) (segments (segment 0 (token "rangeValue") (name "rangeValue") (separator none) (span (offset 1942) (line 61) (column 18) (len 10)))))
    (reference r33 (scope relative) (span (offset 2007) (line 63) (column 26) (len 12)) (segments (segment 0 (token "AscentSample") (name "AscentSample") (separator none) (span (offset 2007) (line 63) (column 26) (len 12)))))
    (reference r34 (scope relative) (span (offset 1998) (line 63) (column 17) (len 7)) (segments (segment 0 (token "samples") (name "samples") (separator none) (span (offset 1998) (line 63) (column 17) (len 7)))))
  )
  (root (package (name "15_13-Discretely Sampled Function Value") (body brace (import (target (span (span (offset 68) (line 2) (column 17) (len 33))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 119) (line 3) (column 17) (len 28))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 165) (line 4) (column 17) (len 18))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 201) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 204) (line 5) (column 20) (len 3))) (separator (span (offset 204) (line 5) (column 20) (len 2))) (marker (span (offset 206) (line 5) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 225) (line 6) (column 17) (len 5))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 227) (line 6) (column 19) (len 3))) (separator (span (offset 227) (line 6) (column 19) (len 2))) (marker (span (offset 229) (line 6) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 248) (line 7) (column 17) (len 24))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 269) (line 7) (column 38) (len 3))) (separator (span (offset 269) (line 7) (column 38) (len 2))) (marker (span (offset 271) (line 7) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 290) (line 8) (column 17) (len 7))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 294) (line 8) (column 21) (len 3))) (separator (span (offset 294) (line 8) (column 21) (len 2))) (marker (span (offset 296) (line 8) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "MissionElapsedTimeScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 367) (line 11) (column 14) (len 1)) (ref r9))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 419) (line 13) (column 14) (len 1)) (integer 0))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 442) (line 14) (column 21) (len 29)) (string "time instant zero at launch"))))) (body semicolon)))) (attribute-usage (declaration-name "definitionalEpochInUTC") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 773) (line 21) (column 17) (len 3)) (ref r17))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 794) (line 22) (column 17) (len 22)) (ref r19))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 843) (line 23) (column 26) (len 7)) (bracket (base (expression (span (offset 843) (line 23) (column 26) (len 1)) (integer 1))) (operands (sequence-list (element first (expression (span (offset 846) (line 23) (column 29) (len 3)) (ref r21)))))))))) (body semicolon)))))) (attribute-usage) (attribute-def (declaration-name "MissionElapsedTimeValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1171) (line 37) (column 5) (len 68)) (normalized "Define scalar quantity value type for mission elapsed time\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1256) (line 40) (column 15) (len 4)) (ref r24))))) (body semicolon)))) (attribute-usage) (attribute-usage) (attribute-def (declaration-name "PositionAndVelocity") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "position") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "velocity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "AscentProfile") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-def (declaration-name "AscentSample") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage))))
)
~~~
