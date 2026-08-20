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
    /* Example declarations of a quantity and unit that are not specified in ISQ and SI */
    attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
             :>> quantity = isq.L;
             :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
             :>> quantity = isq.M;
             :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
             :>> quantity = isq.T;
             :>> exponent = -2;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def TensileStrengthValue :> ScalarQuantityValue {
        attribute :>> num : Real;
        attribute :>> mRef : TensileStrengthUnit;
    }
    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm ^ 2;
    attribute def Substance;
    attribute def Material :> Substance;
    /*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */
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
    /*
	 * Value properties bound to specifically constructed compound values.
	 */
    attribute Iron : Metal {
         :>> atomicMass = 55.845[Da];
    }
    attribute Carbon : Metal {
         :>> atomicMass = 12.011[Da];
    }
    attribute Manganese : Metal {
         :>> atomicMass = 54.938[Da];
    }
    attribute Steel_980 : Alloy {
        /*
		 * Value property with redefined/added sub-properties.
		 * (Particular example of high tensile strength steel.)
		 */
        private attribute fraction1 : MaterialFraction {
             :>> material = Iron;
             :>> massFraction = 0.9862[one];
        }
        private attribute fraction2 : MaterialFraction {
             :>> material = Carbon;
             :>> massFraction = 0.0018[one];
        }
        private attribute fraction3 : MaterialFraction {
             :>> material = Manganese;
             :>> massFraction = 0.012[one];
        }
        attribute :>> fractions = (fraction1, fraction2, fraction3);
        attribute tensileStrength : TensileStrengthValue = 980['N/mm²'];
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
    (reference r31 (scope relative) (span (offset 1089) (line 26) (column 28) (len 9)) (segments (segment 0 (token "Substance") (name "Substance") (separator none) (span (offset 1089) (line 26) (column 28) (len 9)))))
    (reference r32 (scope relative) (span (offset 1447) (line 35) (column 28) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1447) (line 35) (column 28) (len 8)))))
    (reference r33 (scope relative) (span (offset 1488) (line 36) (column 31) (len 15)) (segments (segment 0 (token "AtomicMassValue") (name "AtomicMassValue") (separator none) (span (offset 1488) (line 36) (column 31) (len 15)))))
    (reference r34 (scope relative) (span (offset 1542) (line 39) (column 28) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1542) (line 39) (column 28) (len 8)))))
    (reference r35 (scope relative) (span (offset 1582) (line 40) (column 30) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 1582) (line 40) (column 30) (len 16)))))
    (reference r36 (scope relative) (span (offset 1678) (line 44) (column 29) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1678) (line 44) (column 29) (len 8)))))
    (reference r37 (scope relative) (span (offset 1724) (line 45) (column 33) (len 17)) (segments (segment 0 (token "MassFractionValue") (name "MassFractionValue") (separator none) (span (offset 1724) (line 45) (column 33) (len 17)))))
    (reference r38 (scope relative) (span (offset 1792) (line 48) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1792) (line 48) (column 40) (len 17)))))
  )
  (root (package (name "15_19a-Materials with Properties") (body brace (import (target (span (span (offset 61) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 73) (line 2) (column 29) (len 3))) (separator (span (offset 73) (line 2) (column 29) (len 2))) (marker (span (offset 75) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 94) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 104) (line 3) (column 27) (len 3))) (separator (span (offset 104) (line 3) (column 27) (len 2))) (marker (span (offset 106) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 125) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 146) (line 4) (column 38) (len 3))) (separator (span (offset 146) (line 4) (column 38) (len 2))) (marker (span (offset 148) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 167) (line 5) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 169) (line 5) (column 19) (len 3))) (separator (span (offset 169) (line 5) (column 19) (len 2))) (marker (span (offset 171) (line 5) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "AtomicMassValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 232) (line 9) (column 4) (len 82)) (normalized "Example declarations of a quantity and unit that are not specified in ISQ and SI "))) (attribute-def (declaration-name "TensileStrengthUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 446) (line 12) (column 77) (len 5)) (member-access (base (expression (span (offset 446) (line 12) (column 77) (len 3)) (ref r8))) (separator dot) (member (ref r9))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 468) (line 12) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 469) (line 12) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 548) (line 13) (column 75) (len 5)) (member-access (base (expression (span (offset 548) (line 13) (column 75) (len 3)) (ref r13))) (separator dot) (member (ref r14))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 570) (line 13) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 653) (line 14) (column 79) (len 5)) (member-access (base (expression (span (offset 653) (line 14) (column 79) (len 3)) (ref r18))) (separator dot) (member (ref r19))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 675) (line 14) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 676) (line 14) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 750) (line 15) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 751) (line 15) (column 71) (len 8)) (ref r23))) (element comma (expression (span (offset 761) (line 15) (column 81) (len 6)) (ref r24))) (element comma (expression (span (offset 769) (line 15) (column 89) (len 10)) (ref r25))))))))) (body semicolon)))))) (attribute-def (declaration-name "TensileStrengthValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "Substance") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "Material") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1105) (line 28) (column 4) (len 311)) (normalized "The classification of materials into metals and alloys is grossly simplified and not exhaustive.\nA more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,\nClass, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,\netc.\n"))) (attribute-def (declaration-name "Metal") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r32)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "atomicMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Alloy") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "fractions") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MaterialFraction") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "material") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "massFraction") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MassFractionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1819) (line 50) (column 4) (len 75)) (normalized "Value properties bound to specifically constructed compound values.\n"))) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-usage))))
)
~~~
