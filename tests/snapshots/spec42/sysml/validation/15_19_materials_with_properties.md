# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_19-Materials with Properties"))
~~~
# SOURCE
~~~sysml
package '15_19-Materials with Properties' {
	private import ScalarValues::Real;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import SI::*;
	
    attribute def AtomicMassValue :> MassValue;
    
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

    part def Substance;
    part def Material :> Substance;

	/*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    part def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def MaterialFraction {
        ref material: Material[1]; 
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;    

    part def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }

    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011[Da];
    }

    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938[Da];
    }

    individual def Steel_980 :> Alloy {
	 	/*
		 * Particular example of high tensile strength steel.
		 */
 	
        attribute fraction1 :> fractions {
        	ref :>> material : Iron;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction2 :> fractions {
        	ref :>> material : Carbon;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction3 :> fractions {
        	ref :>> material : Manganese;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute tensileStrength: TensileStrengthValue = 980['N/mm²'];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_19_materials_with_properties.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_19-Materials with Properties' {
    private import ScalarValues::Real;
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
    part def Substance;
    part def Material :> Substance;
    part def Metal :> Material {
        attribute atomicMass : AtomicMassValue[1];
    }
    attribute def MaterialFraction {
        ref material : Material[1];
        attribute massFraction : MassFractionValue[1];
    }
    attribute def MassFractionValue :> DimensionOneValue;
    part def Alloy :> Material {
        attribute fractions : MaterialFraction[2..*];
    }
    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }
    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011 [Da];
    }
    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938 [Da];
    }
    individual def Steel_980 :> Alloy {
        attribute fraction1 :> fractions {
            ref : Iron :>> material;
            attribute :>> massFraction = 0.9862 [one];
        }
        attribute fraction2 :> fractions {
            ref : Carbon :>> material;
            attribute :>> massFraction = 0.9862 [one];
        }
        attribute fraction3 :> fractions {
            ref : Manganese :>> material;
            attribute :>> massFraction = 0.9862 [one];
        }
        attribute tensileStrength : TensileStrengthValue = 980 ['N/mm²'];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 60) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 60) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 74) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 96) (line 3) (column 17) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 96) (line 3) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 127) (line 4) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 127) (line 4) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 169) (line 5) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 169) (line 5) (column 17) (len 2)))))
    (reference r4 (scope relative) (span (offset 215) (line 7) (column 38) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 215) (line 7) (column 38) (len 9)))))
    (reference r5 (scope relative) (span (offset 269) (line 9) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 269) (line 9) (column 39) (len 11)))))
    (reference r6 (scope relative) (span (offset 319) (line 10) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 319) (line 10) (column 37) (len 19)))))
    (reference r7 (scope relative) (span (offset 348) (line 10) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 348) (line 10) (column 66) (len 8)))))
    (reference r8 (scope relative) (span (offset 359) (line 10) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 359) (line 10) (column 77) (len 3)))))
    (reference r9 (scope relative) (span (offset 363) (line 10) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 363) (line 10) (column 81) (len 1)))))
    (reference r10 (scope relative) (span (offset 370) (line 10) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 370) (line 10) (column 88) (len 8)))))
    (reference r11 (scope relative) (span (offset 421) (line 11) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 421) (line 11) (column 35) (len 19)))))
    (reference r12 (scope relative) (span (offset 450) (line 11) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 450) (line 11) (column 64) (len 8)))))
    (reference r13 (scope relative) (span (offset 461) (line 11) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 461) (line 11) (column 75) (len 3)))))
    (reference r14 (scope relative) (span (offset 465) (line 11) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 465) (line 11) (column 79) (len 1)))))
    (reference r15 (scope relative) (span (offset 472) (line 11) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 472) (line 11) (column 86) (len 8)))))
    (reference r16 (scope relative) (span (offset 526) (line 12) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 526) (line 12) (column 39) (len 19)))))
    (reference r17 (scope relative) (span (offset 555) (line 12) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 555) (line 12) (column 68) (len 8)))))
    (reference r18 (scope relative) (span (offset 566) (line 12) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 566) (line 12) (column 79) (len 3)))))
    (reference r19 (scope relative) (span (offset 570) (line 12) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 570) (line 12) (column 83) (len 1)))))
    (reference r20 (scope relative) (span (offset 577) (line 12) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 577) (line 12) (column 90) (len 8)))))
    (reference r21 (scope relative) (span (offset 616) (line 13) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 616) (line 13) (column 23) (len 17)))))
    (reference r22 (scope relative) (span (offset 640) (line 13) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 640) (line 13) (column 47) (len 20)))))
    (reference r23 (scope relative) (span (offset 664) (line 13) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 664) (line 13) (column 71) (len 8)))))
    (reference r24 (scope relative) (span (offset 674) (line 13) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 674) (line 13) (column 81) (len 6)))))
    (reference r25 (scope relative) (span (offset 682) (line 13) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 682) (line 13) (column 89) (len 10)))))
    (reference r26 (scope relative) (span (offset 749) (line 16) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 749) (line 16) (column 43) (len 19)))))
    (reference r27 (scope relative) (span (offset 792) (line 17) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 792) (line 17) (column 22) (len 4)))))
    (reference r28 (scope relative) (span (offset 787) (line 17) (column 17) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 787) (line 17) (column 17) (len 3)))))
    (reference r29 (scope relative) (span (offset 823) (line 18) (column 26) (len 19)) (segments (segment 0 (token "TensileStrengthUnit") (name "TensileStrengthUnit") (separator none) (span (offset 823) (line 18) (column 26) (len 19)))))
    (reference r30 (scope relative) (span (offset 817) (line 18) (column 20) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 817) (line 18) (column 20) (len 4)))))
    (reference r31 (scope relative) (span (offset 913) (line 21) (column 59) (len 19)) (segments (segment 0 (token "TensileStrengthUnit") (name "TensileStrengthUnit") (separator none) (span (offset 913) (line 21) (column 59) (len 19)))))
    (reference r32 (scope relative) (span (offset 935) (line 21) (column 81) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 935) (line 21) (column 81) (len 1)))))
    (reference r33 (scope relative) (span (offset 939) (line 21) (column 85) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 939) (line 21) (column 85) (len 2)))))
    (reference r34 (scope relative) (span (offset 1388) (line 34) (column 31) (len 15)) (segments (segment 0 (token "AtomicMassValue") (name "AtomicMassValue") (separator none) (span (offset 1388) (line 34) (column 31) (len 15)))))
    (reference r35 (scope relative) (span (offset 1474) (line 38) (column 23) (len 8)) (segments (segment 0 (token "Material") (name "Material") (separator none) (span (offset 1474) (line 38) (column 23) (len 8)))))
    (reference r36 (scope relative) (span (offset 1520) (line 39) (column 33) (len 17)) (segments (segment 0 (token "MassFractionValue") (name "MassFractionValue") (separator none) (span (offset 1520) (line 39) (column 33) (len 17)))))
    (reference r37 (scope relative) (span (offset 1588) (line 42) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1588) (line 42) (column 40) (len 17)))))
    (reference r38 (scope relative) (span (offset 1674) (line 45) (column 30) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 1674) (line 45) (column 30) (len 16)))))
  )
  (root (package (name "15_19-Materials with Properties") (body brace (import (target (span (span (offset 60) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 96) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 3) (column 27) (len 3))) (separator (span (offset 106) (line 3) (column 27) (len 2))) (marker (span (offset 108) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 127) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 148) (line 4) (column 38) (len 3))) (separator (span (offset 148) (line 4) (column 38) (len 2))) (marker (span (offset 150) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 169) (line 5) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 171) (line 5) (column 19) (len 3))) (separator (span (offset 171) (line 5) (column 19) (len 2))) (marker (span (offset 173) (line 5) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (declaration-name "AtomicMassValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "TensileStrengthUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 359) (line 10) (column 77) (len 5)) (member-access (base (expression (span (offset 359) (line 10) (column 77) (len 3)) (ref r8))) (separator dot) (member (ref r9))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 381) (line 10) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 382) (line 10) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 461) (line 11) (column 75) (len 5)) (member-access (base (expression (span (offset 461) (line 11) (column 75) (len 3)) (ref r13))) (separator dot) (member (ref r14))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 483) (line 11) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 566) (line 12) (column 79) (len 5)) (member-access (base (expression (span (offset 566) (line 12) (column 79) (len 3)) (ref r18))) (separator dot) (member (ref r19))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 588) (line 12) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 589) (line 12) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 663) (line 13) (column 70) (len 30)) (tuple (expression (span (offset 664) (line 13) (column 71) (len 8)) (ref r23)) (expression (span (offset 674) (line 13) (column 81) (len 6)) (ref r24)) (expression (span (offset 682) (line 13) (column 89) (len 10)) (ref r25))))))) (body semicolon)))))) (attribute-def (declaration-name "TensileStrengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "newton per square millimetre") (short-name "N/mm²") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 935) (line 21) (column 81) (len 8)) (binary (operator "^") (left (expression (span (offset 935) (line 21) (column 81) (len 6)) (binary (operator "/") (left (expression (span (offset 935) (line 21) (column 81) (len 1)) (ref r32))) (right (expression (span (offset 939) (line 21) (column 85) (len 2)) (ref r33)))))) (right (expression (span (offset 942) (line 21) (column 88) (len 1)) (integer 2)))))))) (body semicolon)) (part-def (name "Substance") (body semicolon)) (part-def (name "Material") (body semicolon)) (part-def (name "Metal") (body brace (attribute-usage (declaration-name "atomicMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MaterialFraction") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (ref (name "material") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (redefines none) (subsets none) (body semicolon)) (attribute-usage (declaration-name "massFraction") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MassFractionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (part-def (name "Alloy") (body brace (attribute-usage (declaration-name "fractions") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (individual-def) (individual-def) (individual-def) (individual-def))))
)
~~~
