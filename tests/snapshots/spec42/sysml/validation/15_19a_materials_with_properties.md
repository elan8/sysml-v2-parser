# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_19a-Materials with Properties"))
~~~
# SOURCE
~~~sysml
package '15_19a-Materials with Properties' {
	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import SI::*;
	
    attribute def AtomicMassValue :> MassValue;
    
	/* Example declarations of a quantity and unit that are not specified in ISQ and SI */

	attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }		
	}
    
    attribute def TensileStrengthValue :> ScalarQuantityValue {
		attribute :>> num: Real;
    	attribute :>> mRef: TensileStrengthUnit;
    }
    
    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm^2;

    attribute def Substance;
	attribute def Material :> Substance;
	
	/*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    attribute def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    attribute def MaterialFraction {
        attribute material: Material[1]; 
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;    

	/*
	 * Value properties bound to specifically constructed compound values.
	 */
    attribute Iron: Metal { :>> atomicMass = 55.845[Da]; }
    attribute Carbon: Metal { :>> atomicMass = 12.011[Da]; }
    attribute Manganese: Metal { :>> atomicMass = 54.938[Da]; }
    
    attribute Steel_980: Alloy {
		/*
		 * Value property with redefined/added sub-properties.
		 * (Particular example of high tensile strength steel.)
		 */
	
        private attribute fraction1: MaterialFraction { :>> material = Iron; :>> massFraction = 0.9862[one]; }
        private attribute fraction2: MaterialFraction { :>> material = Carbon; :>> massFraction = 0.0018[one]; }
        private attribute fraction3: MaterialFraction { :>> material = Manganese; :>> massFraction = 0.012[one]; }
    	attribute :>> fractions = (fraction1, fraction2, fraction3);
        attribute tensileStrength: TensileStrengthValue = 980 ['N/mm²'];
    } 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_19a_materials_with_properties.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_19a-Materials with Properties' {
    private import ScalarValues::*;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import SI::*;
    attribute def AtomicMassValue :> MassValue;
    attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def TensileStrengthValue :> ScalarQuantityValue {
        attribute :>> num : Real;
        attribute :>> mRef : TensileStrengthUnit;
    }
    attribute def <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm ^ 2;
    attribute def Substance;
    attribute def Material :> Substance;
    attribute def Metal :> Material {
        attribute atomicMass : AtomicMassValue[1];
    }
    attribute def Alloy :> Material {
        attribute fractions : MaterialFraction[2..*];
    }
    attribute def MaterialFraction {
        attribute material : Material[1];
        attribute massFraction : MassFractionValue[1];
    }
    attribute def MassFractionValue :> DimensionOneValue;
    attribute def Iron : Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }
    attribute def Carbon : Metal {
        attribute :>> atomicMass = 12.011 [Da];
    }
    attribute def Manganese : Metal {
        attribute :>> atomicMass = 54.938 [Da];
    }
    attribute def Steel_980 : Alloy {
        private attribute fraction1 : MaterialFraction {
            attribute :>> material = Iron;
            attribute :>> massFraction = 0.9862 [one];
        }
        private attribute fraction2 : MaterialFraction {
            attribute :>> material = Carbon;
            attribute :>> massFraction = 0.0018 [one];
        }
        private attribute fraction3 : MaterialFraction {
            attribute :>> material = Manganese;
            attribute :>> massFraction = 0.012 [one];
        }
        attribute :>> fractions = (fraction1, fraction2, fraction3);
        attribute tensileStrength : TensileStrengthValue = 980 ['N/mm²'];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 61) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 61) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 94) (line 3) (column 17) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 94) (line 3) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 125) (line 4) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 125) (line 4) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 167) (line 5) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 167) (line 5) (column 17) (len 2)))))
    (reference r4 (scope relative) (span (offset 213) (line 7) (column 38) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 213) (line 7) (column 38) (len 9)))))
    (reference r5 (scope relative) (span (offset 356) (line 11) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 356) (line 11) (column 39) (len 11)))))
    (reference r6 (scope relative) (span (offset 406) (line 12) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 406) (line 12) (column 37) (len 19)))))
    (reference r7 (scope relative) (span (offset 435) (line 12) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 435) (line 12) (column 66) (len 8)))))
    (reference r8 (scope relative) (span (offset 446) (line 12) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 446) (line 12) (column 77) (len 3)))))
    (reference r9 (scope relative) (span (offset 450) (line 12) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 450) (line 12) (column 81) (len 1)))))
    (reference r10 (scope relative) (span (offset 457) (line 12) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 457) (line 12) (column 88) (len 8)))))
    (reference r11 (scope relative) (span (offset 508) (line 13) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 508) (line 13) (column 35) (len 19)))))
    (reference r12 (scope relative) (span (offset 537) (line 13) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 537) (line 13) (column 64) (len 8)))))
    (reference r13 (scope relative) (span (offset 548) (line 13) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 548) (line 13) (column 75) (len 3)))))
    (reference r14 (scope relative) (span (offset 552) (line 13) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 552) (line 13) (column 79) (len 1)))))
    (reference r15 (scope relative) (span (offset 559) (line 13) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 559) (line 13) (column 86) (len 8)))))
    (reference r16 (scope relative) (span (offset 613) (line 14) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 613) (line 14) (column 39) (len 19)))))
    (reference r17 (scope relative) (span (offset 642) (line 14) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 642) (line 14) (column 68) (len 8)))))
    (reference r18 (scope relative) (span (offset 653) (line 14) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 653) (line 14) (column 79) (len 3)))))
    (reference r19 (scope relative) (span (offset 657) (line 14) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 657) (line 14) (column 83) (len 1)))))
    (reference r20 (scope relative) (span (offset 664) (line 14) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 664) (line 14) (column 90) (len 8)))))
    (reference r21 (scope relative) (span (offset 703) (line 15) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 703) (line 15) (column 23) (len 17)))))
    (reference r22 (scope relative) (span (offset 727) (line 15) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 727) (line 15) (column 47) (len 20)))))
    (reference r23 (scope relative) (span (offset 751) (line 15) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 751) (line 15) (column 71) (len 8)))))
    (reference r24 (scope relative) (span (offset 761) (line 15) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 761) (line 15) (column 81) (len 6)))))
    (reference r25 (scope relative) (span (offset 769) (line 15) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 769) (line 15) (column 89) (len 10)))))
    (reference r26 (scope relative) (span (offset 836) (line 18) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 836) (line 18) (column 43) (len 19)))))
    (reference r27 (scope relative) (span (offset 879) (line 19) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 879) (line 19) (column 22) (len 4)))))
    (reference r28 (scope relative) (span (offset 874) (line 19) (column 17) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 874) (line 19) (column 17) (len 3)))))
    (reference r29 (scope relative) (span (offset 910) (line 20) (column 26) (len 19)) (segments (segment 0 (token "TensileStrengthUnit") (name "TensileStrengthUnit") (separator none) (span (offset 910) (line 20) (column 26) (len 19)))))
    (reference r30 (scope relative) (span (offset 904) (line 20) (column 20) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 904) (line 20) (column 20) (len 4)))))
    (reference r31 (scope relative) (span (offset 1000) (line 23) (column 59) (len 19)) (segments (segment 0 (token "TensileStrengthUnit") (name "TensileStrengthUnit") (separator none) (span (offset 1000) (line 23) (column 59) (len 19)))))
    (reference r32 (scope relative) (span (offset 1022) (line 23) (column 81) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 1022) (line 23) (column 81) (len 1)))))
    (reference r33 (scope relative) (span (offset 1026) (line 23) (column 85) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 1026) (line 23) (column 85) (len 2)))))
    (reference r34 (scope relative) (span (offset 1089) (line 26) (column 28) (len 9)) (segments (segment 0 (token "Substance") (name "Substance") (separator none) (span (offset 1089) (line 26) (column 28) (len 9)))))
    (reference r35 (scope relative) (span (offset 1447) (line 35) (column 28) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1447) (line 35) (column 28) (len 8)))))
    (reference r36 (scope relative) (span (offset 1488) (line 36) (column 31) (len 15)) (segments (segment 0 (token "AtomicMassValue") (name "AtomicMassValue") (separator none) (span (offset 1488) (line 36) (column 31) (len 15)))))
    (reference r37 (scope relative) (span (offset 1542) (line 39) (column 28) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1542) (line 39) (column 28) (len 8)))))
    (reference r38 (scope relative) (span (offset 1582) (line 40) (column 30) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 1582) (line 40) (column 30) (len 16)))))
    (reference r39 (scope relative) (span (offset 1678) (line 44) (column 29) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1678) (line 44) (column 29) (len 8)))))
    (reference r40 (scope relative) (span (offset 1724) (line 45) (column 33) (len 17)) (segments (segment 0 (token "MassFractionValue") (name "MassFractionValue") (separator none) (span (offset 1724) (line 45) (column 33) (len 17)))))
    (reference r41 (scope relative) (span (offset 1792) (line 48) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1792) (line 48) (column 40) (len 17)))))
    (reference r42 (scope relative) (span (offset 1917) (line 53) (column 21) (len 5)) (segments (segment 0 (token "Metal") (name "Metal") (separator none) (span (offset 1917) (line 53) (column 21) (len 5)))))
    (reference r43 (scope relative) (span (offset 1929) (line 53) (column 33) (len 10)) (segments (segment 0 (token "atomicMass") (name "atomicMass") (separator none) (span (offset 1929) (line 53) (column 33) (len 10)))))
    (reference r44 (scope relative) (span (offset 1978) (line 54) (column 23) (len 5)) (segments (segment 0 (token "Metal") (name "Metal") (separator none) (span (offset 1978) (line 54) (column 23) (len 5)))))
    (reference r45 (scope relative) (span (offset 1990) (line 54) (column 35) (len 10)) (segments (segment 0 (token "atomicMass") (name "atomicMass") (separator none) (span (offset 1990) (line 54) (column 35) (len 10)))))
    (reference r46 (scope relative) (span (offset 2042) (line 55) (column 26) (len 5)) (segments (segment 0 (token "Metal") (name "Metal") (separator none) (span (offset 2042) (line 55) (column 26) (len 5)))))
    (reference r47 (scope relative) (span (offset 2054) (line 55) (column 38) (len 10)) (segments (segment 0 (token "atomicMass") (name "atomicMass") (separator none) (span (offset 2054) (line 55) (column 38) (len 10)))))
    (reference r48 (scope relative) (span (offset 2111) (line 57) (column 26) (len 5)) (segments (segment 0 (token "Alloy") (name "Alloy") (separator none) (span (offset 2111) (line 57) (column 26) (len 5)))))
    (reference r49 (scope relative) (span (offset 2284) (line 63) (column 38) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 2284) (line 63) (column 38) (len 16)))))
    (reference r50 (scope relative) (span (offset 2307) (line 63) (column 61) (len 8)) (segments (segment 0 (token "material") (name "material") (separator none) (span (offset 2307) (line 63) (column 61) (len 8)))))
    (reference r51 (scope relative) (span (offset 2318) (line 63) (column 72) (len 4)) (segments (segment 0 (token "Iron") (name "Iron") (separator none) (span (offset 2318) (line 63) (column 72) (len 4)))))
    (reference r52 (scope relative) (span (offset 2328) (line 63) (column 82) (len 12)) (segments (segment 0 (token "massFraction") (name "massFraction") (separator none) (span (offset 2328) (line 63) (column 82) (len 12)))))
    (reference r53 (scope relative) (span (offset 2395) (line 64) (column 38) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 2395) (line 64) (column 38) (len 16)))))
    (reference r54 (scope relative) (span (offset 2418) (line 64) (column 61) (len 8)) (segments (segment 0 (token "material") (name "material") (separator none) (span (offset 2418) (line 64) (column 61) (len 8)))))
    (reference r55 (scope relative) (span (offset 2429) (line 64) (column 72) (len 6)) (segments (segment 0 (token "Carbon") (name "Carbon") (separator none) (span (offset 2429) (line 64) (column 72) (len 6)))))
    (reference r56 (scope relative) (span (offset 2441) (line 64) (column 84) (len 12)) (segments (segment 0 (token "massFraction") (name "massFraction") (separator none) (span (offset 2441) (line 64) (column 84) (len 12)))))
    (reference r57 (scope relative) (span (offset 2508) (line 65) (column 38) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 2508) (line 65) (column 38) (len 16)))))
    (reference r58 (scope relative) (span (offset 2531) (line 65) (column 61) (len 8)) (segments (segment 0 (token "material") (name "material") (separator none) (span (offset 2531) (line 65) (column 61) (len 8)))))
    (reference r59 (scope relative) (span (offset 2542) (line 65) (column 72) (len 9)) (segments (segment 0 (token "Manganese") (name "Manganese") (separator none) (span (offset 2542) (line 65) (column 72) (len 9)))))
    (reference r60 (scope relative) (span (offset 2557) (line 65) (column 87) (len 12)) (segments (segment 0 (token "massFraction") (name "massFraction") (separator none) (span (offset 2557) (line 65) (column 87) (len 12)))))
    (reference r61 (scope relative) (span (offset 2605) (line 66) (column 20) (len 9)) (segments (segment 0 (token "fractions") (name "fractions") (separator none) (span (offset 2605) (line 66) (column 20) (len 9)))))
    (reference r62 (scope relative) (span (offset 2618) (line 66) (column 33) (len 9)) (segments (segment 0 (token "fraction1") (name "fraction1") (separator none) (span (offset 2618) (line 66) (column 33) (len 9)))))
    (reference r63 (scope relative) (span (offset 2629) (line 66) (column 44) (len 9)) (segments (segment 0 (token "fraction2") (name "fraction2") (separator none) (span (offset 2629) (line 66) (column 44) (len 9)))))
    (reference r64 (scope relative) (span (offset 2640) (line 66) (column 55) (len 9)) (segments (segment 0 (token "fraction3") (name "fraction3") (separator none) (span (offset 2640) (line 66) (column 55) (len 9)))))
    (reference r65 (scope relative) (span (offset 2687) (line 67) (column 36) (len 20)) (segments (segment 0 (token "TensileStrengthValue") (name "TensileStrengthValue") (separator none) (span (offset 2687) (line 67) (column 36) (len 20)))))
  )
  (root (package (name "15_19a-Materials with Properties") (body brace (import (target (span (span (offset 61) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 73) (line 2) (column 29) (len 3))) (separator (span (offset 73) (line 2) (column 29) (len 2))) (marker (span (offset 75) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 94) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 104) (line 3) (column 27) (len 3))) (separator (span (offset 104) (line 3) (column 27) (len 2))) (marker (span (offset 106) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 125) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 146) (line 4) (column 38) (len 3))) (separator (span (offset 146) (line 4) (column 38) (len 2))) (marker (span (offset 148) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 167) (line 5) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 169) (line 5) (column 19) (len 3))) (separator (span (offset 169) (line 5) (column 19) (len 2))) (marker (span (offset 171) (line 5) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "AtomicMassValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "TensileStrengthUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 446) (line 12) (column 77) (len 5)) (member-access (base (expression (span (offset 446) (line 12) (column 77) (len 3)) (ref r8))) (separator dot) (member (ref r9))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 468) (line 12) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 469) (line 12) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 548) (line 13) (column 75) (len 5)) (member-access (base (expression (span (offset 548) (line 13) (column 75) (len 3)) (ref r13))) (separator dot) (member (ref r14))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 570) (line 13) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 653) (line 14) (column 79) (len 5)) (member-access (base (expression (span (offset 653) (line 14) (column 79) (len 3)) (ref r18))) (separator dot) (member (ref r19))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 675) (line 14) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 676) (line 14) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 750) (line 15) (column 70) (len 30)) (tuple (expression (span (offset 751) (line 15) (column 71) (len 8)) (ref r23)) (expression (span (offset 761) (line 15) (column 81) (len 6)) (ref r24)) (expression (span (offset 769) (line 15) (column 89) (len 10)) (ref r25))))))) (body semicolon)))))) (attribute-def (declaration-name "TensileStrengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "newton per square millimetre") (short-name "N/mm²") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1022) (line 23) (column 81) (len 8)) (binary (operator "^") (left (expression (span (offset 1022) (line 23) (column 81) (len 6)) (binary (operator "/") (left (expression (span (offset 1022) (line 23) (column 81) (len 1)) (ref r32))) (right (expression (span (offset 1026) (line 23) (column 85) (len 2)) (ref r33)))))) (right (expression (span (offset 1029) (line 23) (column 88) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-def (declaration-name "Substance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "Material") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "Metal") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "atomicMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Alloy") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "fractions") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MaterialFraction") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "material") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "massFraction") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r40)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MassFractionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r41)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "Iron") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1942) (line 53) (column 46) (len 10)) (literal-with-unit (value (expression (span (offset 1942) (line 53) (column 46) (len 6)) (real "55.845"))) (unit (expression (span (offset 1949) (line 53) (column 53) (len 2)) (bracket (expression (span (offset 1949) (line 53) (column 53) (len 2)) (unit "Da")))))))))) (body semicolon)))) (attribute-def (declaration-name "Carbon") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r44)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2003) (line 54) (column 48) (len 10)) (literal-with-unit (value (expression (span (offset 2003) (line 54) (column 48) (len 6)) (real "12.011"))) (unit (expression (span (offset 2010) (line 54) (column 55) (len 2)) (bracket (expression (span (offset 2010) (line 54) (column 55) (len 2)) (unit "Da")))))))))) (body semicolon)))) (attribute-def (declaration-name "Manganese") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2067) (line 55) (column 51) (len 10)) (literal-with-unit (value (expression (span (offset 2067) (line 55) (column 51) (len 6)) (real "54.938"))) (unit (expression (span (offset 2074) (line 55) (column 58) (len 2)) (bracket (expression (span (offset 2074) (line 55) (column 58) (len 2)) (unit "Da")))))))))) (body semicolon)))) (attribute-def (declaration-name "Steel_980") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r48)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "fraction1") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r49)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2318) (line 63) (column 72) (len 4)) (ref r51))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2343) (line 63) (column 97) (len 11)) (literal-with-unit (value (expression (span (offset 2343) (line 63) (column 97) (len 6)) (real "0.9862"))) (unit (expression (span (offset 2350) (line 63) (column 104) (len 3)) (bracket (expression (span (offset 2350) (line 63) (column 104) (len 3)) (unit "one")))))))))) (body semicolon)))) (attribute-usage (declaration-name "fraction2") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r54)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2429) (line 64) (column 72) (len 6)) (ref r55))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2456) (line 64) (column 99) (len 11)) (literal-with-unit (value (expression (span (offset 2456) (line 64) (column 99) (len 6)) (real "0.0018"))) (unit (expression (span (offset 2463) (line 64) (column 106) (len 3)) (bracket (expression (span (offset 2463) (line 64) (column 106) (len 3)) (unit "one")))))))))) (body semicolon)))) (attribute-usage (declaration-name "fraction3") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2542) (line 65) (column 72) (len 9)) (ref r59))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2572) (line 65) (column 102) (len 10)) (literal-with-unit (value (expression (span (offset 2572) (line 65) (column 102) (len 5)) (real "0.012"))) (unit (expression (span (offset 2578) (line 65) (column 108) (len 3)) (bracket (expression (span (offset 2578) (line 65) (column 108) (len 3)) (unit "one")))))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r61)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2617) (line 66) (column 32) (len 33)) (tuple (expression (span (offset 2618) (line 66) (column 33) (len 9)) (ref r62)) (expression (span (offset 2629) (line 66) (column 44) (len 9)) (ref r63)) (expression (span (offset 2640) (line 66) (column 55) (len 9)) (ref r64))))))) (body semicolon)) (attribute-usage (declaration-name "tensileStrength") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r65)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2710) (line 67) (column 59) (len 14)) (literal-with-unit (value (expression (span (offset 2710) (line 67) (column 59) (len 3)) (integer 980))) (unit (expression (span (offset 2715) (line 67) (column 64) (len 8)) (bracket (expression (span (offset 2715) (line 67) (column 64) (len 8)) (unit "N/mm²")))))))))) (body semicolon)))))))
)
~~~
