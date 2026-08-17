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
  )
  (root (package (name "15_19a-Materials with Properties") (body brace (import (target (span (span (offset 61) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 73) (line 2) (column 29) (len 3))) (separator (span (offset 73) (line 2) (column 29) (len 2))) (marker (span (offset 75) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 94) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 104) (line 3) (column 27) (len 3))) (separator (span (offset 104) (line 3) (column 27) (len 2))) (marker (span (offset 106) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 125) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 146) (line 4) (column 38) (len 3))) (separator (span (offset 146) (line 4) (column 38) (len 2))) (marker (span (offset 148) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 167) (line 5) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 169) (line 5) (column 19) (len 3))) (separator (span (offset 169) (line 5) (column 19) (len 2))) (marker (span (offset 171) (line 5) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def (name "AtomicMassValue") (multiplicity none)) (attribute-def (name "TensileStrengthUnit") (multiplicity none)) (attribute-def (name "TensileStrengthValue") (multiplicity none)) (attribute-def (name "newton per square millimetre") (multiplicity none)) (attribute-def (name "Substance") (multiplicity none)) (attribute-def (name "Material") (multiplicity none)) (attribute-def (name "Metal") (multiplicity none)) (attribute-def (name "Alloy") (multiplicity none)) (attribute-def (name "MaterialFraction") (multiplicity none)) (attribute-def (name "MassFractionValue") (multiplicity none)) (attribute-def (name "Iron") (multiplicity none)) (attribute-def (name "Carbon") (multiplicity none)) (attribute-def (name "Manganese") (multiplicity none)) (attribute-def (name "Steel_980") (multiplicity none)))))
)
~~~
