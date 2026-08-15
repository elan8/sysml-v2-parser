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
    (reference r4 (scope relative) (span (offset 1388) (line 34) (column 31) (len 15)) (segments (segment 0 (token "AtomicMassValue") (name "AtomicMassValue") (separator none) (span (offset 1388) (line 34) (column 31) (len 15)))))
    (reference r5 (scope relative) (span (offset 1674) (line 45) (column 30) (len 16)) (segments (segment 0 (token "MaterialFraction") (name "MaterialFraction") (separator none) (span (offset 1674) (line 45) (column 30) (len 16)))))
  )
  (root (package (name "15_19-Materials with Properties") (body brace (import (target (span (span (offset 60) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 96) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 3) (column 27) (len 3))) (separator (span (offset 106) (line 3) (column 27) (len 2))) (marker (span (offset 108) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 127) (line 4) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 148) (line 4) (column 38) (len 3))) (separator (span (offset 148) (line 4) (column 38) (len 2))) (marker (span (offset 150) (line 4) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 169) (line 5) (column 17) (len 5))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 171) (line 5) (column 19) (len 3))) (separator (span (offset 171) (line 5) (column 19) (len 2))) (marker (span (offset 173) (line 5) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (part-def (name "Substance") (body semicolon)) (part-def (name "Material") (body semicolon)) (part-def (name "Metal") (body brace (attribute-usage (declaration-name "atomicMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def) (attribute-def) (part-def (name "Alloy") (body brace (attribute-usage (declaration-name "fractions") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (individual-def) (individual-def) (individual-def) (individual-def))))
)
~~~
