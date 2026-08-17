# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/Quantities"))
~~~
# SOURCE
~~~sysml
standard library package Quantities {
	doc
	/*
	 * This package defines the root representations for quantities and their values.
	 */

	private import Collections::*;
	private import ScalarValues::NumericalValue;
	private import ScalarValues::Number;
	private import ScalarValues::Real;
	private import ScalarValues::Natural;
	private import ScalarValues::Boolean;
	private import ScalarValues::String;
	private import VectorValues::NumericalVectorValue;
	private import VectorValues::ThreeVectorValue;

	abstract attribute def TensorQuantityValue :> Array {
		doc
		/*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */
	
		attribute isBound: Boolean;
		attribute num: Number[1..*] ordered nonunique :>> elements;
		attribute mRef: MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
		attribute order :>> rank;
        attribute contravariantOrder: Natural;
        attribute covariantOrder: Natural;

        assert constraint orderSum { contravariantOrder + covariantOrder == order }
        assert constraint boundMatch { (isBound == mRef.isBound) or (not isBound and mRef.isBound) }
	}

	abstract attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
		attribute :>> mRef: MeasurementReferences::VectorMeasurementReference;
	}

	abstract attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
		attribute :>> mRef: MeasurementReferences::ScalarMeasurementReference;
	}
	
	abstract attribute tensorQuantities: TensorQuantityValue[*] nonunique {
		doc
		/*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
	}
	abstract attribute vectorQuantities: VectorQuantityValue[*] nonunique :> tensorQuantities;
	abstract attribute scalarQuantities: ScalarQuantityValue[*] nonunique :> vectorQuantities;

	abstract attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc
    	/*
    	 * Most general representation of real 3-vector quantities
    	 */

        attribute :>> num: Real[3];
	}
	alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';
	
    /*
     * Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.
     */
	alias QuantityValue for TensorQuantityValue;
	alias quantities for tensorQuantities;

	attribute def SystemOfQuantities {
		doc
		/*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */
	
		attribute baseQuantities: ScalarQuantityValue[*] ordered :> scalarQuantities;
	}

	attribute def QuantityPowerFactor {
		doc
		/*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */
	
		attribute quantity: ScalarQuantityValue[1];
		attribute exponent: Real[1];
	}

	attribute def QuantityDimension {
		doc
		/*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */
	
		 attribute quantityPowerFactors: QuantityPowerFactor[*] ordered;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "quantities.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Quantities {
    doc
    /*
	 * This package defines the root representations for quantities and their values.
	 */
    private import Collections::*;
    private import ScalarValues::NumericalValue;
    private import ScalarValues::Number;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
    private import ScalarValues::Boolean;
    private import ScalarValues::String;
    private import VectorValues::NumericalVectorValue;
    private import VectorValues::ThreeVectorValue;
    attribute def TensorQuantityValue :> Array {
        doc
        /*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */
        attribute isBound : Boolean;
        attribute num : Number[1..*] ordered nonunique :>> elements;
        attribute mRef : MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
        attribute order :>> rank;
        attribute contravariantOrder : Natural;
        attribute covariantOrder : Natural;
        assert constraint orderSum {
            contravariantOrder + covariantOrder == order;
        }
        assert constraint boundMatch {
            (isBound == mRef.isBound) || (not isBound && mRef.isBound);
        }
    }
    attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
        attribute :>> mRef : MeasurementReferences::VectorMeasurementReference;
    }
    attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
        attribute :>> mRef : MeasurementReferences::ScalarMeasurementReference;
    }
    attribute def tensorQuantities : TensorQuantityValue[*] nonunique {
        doc
        /*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
    }
    attribute def vectorQuantities : VectorQuantityValue[*] nonunique;
    attribute def scalarQuantities : ScalarQuantityValue[*] nonunique;
    attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc
        /*
    	 * Most general representation of real 3-vector quantities
    	 */
        attribute :>> num : Real[3];
    }
    alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';
    alias QuantityValue for TensorQuantityValue;
    alias quantities for tensorQuantities;
    attribute def SystemOfQuantities {
        doc
        /*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */
        attribute baseQuantities : ScalarQuantityValue[*] ordered :> scalarQuantities;
    }
    attribute def QuantityPowerFactor {
        doc
        /*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */
        attribute quantity : ScalarQuantityValue[1];
        attribute exponent : Real[1];
    }
    attribute def QuantityDimension {
        doc
        /*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */
        attribute quantityPowerFactors : QuantityPowerFactor[*] ordered;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 152) (line 7) (column 17) (len 11)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 152) (line 7) (column 17) (len 11)))))
    (reference r1 (scope relative) (span (offset 184) (line 8) (column 17) (len 28)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 184) (line 8) (column 17) (len 12))) (segment 1 (token "NumericalValue") (name "NumericalValue") (separator colon-colon) (span (offset 198) (line 8) (column 31) (len 14)))))
    (reference r2 (scope relative) (span (offset 230) (line 9) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 230) (line 9) (column 17) (len 12))) (segment 1 (token "Number") (name "Number") (separator colon-colon) (span (offset 244) (line 9) (column 31) (len 6)))))
    (reference r3 (scope relative) (span (offset 268) (line 10) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 268) (line 10) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 282) (line 10) (column 31) (len 4)))))
    (reference r4 (scope relative) (span (offset 304) (line 11) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 304) (line 11) (column 17) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 318) (line 11) (column 31) (len 7)))))
    (reference r5 (scope relative) (span (offset 343) (line 12) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 343) (line 12) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 357) (line 12) (column 31) (len 7)))))
    (reference r6 (scope relative) (span (offset 382) (line 13) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 382) (line 13) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 396) (line 13) (column 31) (len 6)))))
    (reference r7 (scope relative) (span (offset 420) (line 14) (column 17) (len 34)) (segments (segment 0 (token "VectorValues") (name "VectorValues") (separator none) (span (offset 420) (line 14) (column 17) (len 12))) (segment 1 (token "NumericalVectorValue") (name "NumericalVectorValue") (separator colon-colon) (span (offset 434) (line 14) (column 31) (len 20)))))
    (reference r8 (scope relative) (span (offset 472) (line 15) (column 17) (len 30)) (segments (segment 0 (token "VectorValues") (name "VectorValues") (separator none) (span (offset 472) (line 15) (column 17) (len 12))) (segment 1 (token "ThreeVectorValue") (name "ThreeVectorValue") (separator colon-colon) (span (offset 486) (line 15) (column 31) (len 16)))))
    (reference r9 (scope relative) (span (offset 3027) (line 67) (column 38) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 3027) (line 67) (column 38) (len 23)))))
    (reference r10 (scope relative) (span (offset 3209) (line 72) (column 26) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 3209) (line 72) (column 26) (len 19)))))
    (reference r11 (scope relative) (span (offset 3252) (line 73) (column 23) (len 16)) (segments (segment 0 (token "tensorQuantities") (name "tensorQuantities") (separator none) (span (offset 3252) (line 73) (column 23) (len 16)))))
  )
  (root (library-package (name "Quantities") (standard true) (body brace (doc) (import (target (span (span (offset 152) (line 7) (column 17) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 163) (line 7) (column 28) (len 3))) (separator (span (offset 163) (line 7) (column 28) (len 2))) (marker (span (offset 165) (line 7) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 184) (line 8) (column 17) (len 28))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 230) (line 9) (column 17) (len 20))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 268) (line 10) (column 17) (len 18))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 304) (line 11) (column 17) (len 21))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 343) (line 12) (column 17) (len 21))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 382) (line 13) (column 17) (len 20))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 420) (line 14) (column 17) (len 34))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 472) (line 15) (column 17) (len 30))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (attribute-def (name "TensorQuantityValue") (multiplicity none)) (attribute-def (name "VectorQuantityValue") (multiplicity none)) (attribute-def (name "ScalarQuantityValue") (multiplicity none)) (attribute-def (name "tensorQuantities") (multiplicity (lower unbounded) (upper unbounded))) (attribute-def (name "vectorQuantities") (multiplicity (lower unbounded) (upper unbounded))) (attribute-def (name "scalarQuantities") (multiplicity (lower unbounded) (upper unbounded))) (attribute-def (name "3dVectorQuantityValue") (multiplicity none)) (alias (name "ThreeDVectorQuantityValue") (target (ref r9)) (body semicolon)) (alias (name "QuantityValue") (target (ref r10)) (body semicolon)) (alias (name "quantities") (target (ref r11)) (body semicolon)) (attribute-def (name "SystemOfQuantities") (multiplicity none)) (attribute-def (name "QuantityPowerFactor") (multiplicity none)) (attribute-def (name "QuantityDimension") (multiplicity none)))))
)
~~~
