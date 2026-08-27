# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/MeasurementReferences"))
~~~
# SOURCE
~~~sysml
standard library package MeasurementReferences {
	doc
	/*
	 * This package defines the representations for measurement references.
	 */

	private import Collections::Array;
	private import Collections::List;
	private import ScalarValues::*;
	private import VectorValues::ThreeVectorValue;

	private import SequenceFunctions::size;
	private import SequenceFunctions::equals;
	private import ControlFunctions::forAll;
	private import Quantities::QuantityDimension;
	private import Quantities::VectorQuantityValue;
	private import Quantities::scalarQuantities;
	private import Quantities::ScalarQuantityValue;
	private import Quantities::SystemOfQuantities;
	private import ISQSpaceTime::angularMeasure;

	attribute def TensorMeasurementReference :> Array {
		doc
		/*
		 * TensorMeasurementReference is the most general AttributeDefinition to represent measurement references.
		 *
		 * The concept "measurement reference" is defined in [VIM] "quantity" NOTE 2 as "A reference can be a measurement unit,
		 * a measurement procedure, a reference material, or a combination of such.", see https://jcgm.bipm.org/vim/en/1.1.html .
		 * In addition [VIM] "quantity" NOTE 5 states that "A quantity as defined here is a scalar. However, a vector or a tensor, 
		 * the components of which are quantities, is also considered to be a quantity". However, the rest of [VIM] does not explicitly 
		 * define how tensor and vector quantities can be or should be supported.
		 *
		 * In this package, in line with TensorQuantityValue in package Quantities, the most general kind of measurement reference
		 * is TensorMeasurementReference that represents a measurement reference for any order of tensor quantity. Since the order can 
		 * also be one or zero, this includes vector and scalar quantities. The specializations VectorMeasurementReference and 
		 * ScalarMeasurementReference are defined to specifically represent measurement references for vector and scalar quantities.
		 * 
		 * TensorMeasurementReference specializes Array, which provides its multi-dimensional structure. The order of a tensor is equivalent
		 * to the rank of an Array.
		 * 
		 * Attribute isBound specifies whether the vector space product is bound (isBound is true) or free (isBound is false).
		 * 
		 * Attribute mRefs specifies the scalar measurement references for all dimensions of a tensor quantity.
		 *
		 * The short name of a TensorMeasurementReference is the unique symbol by which the measurement reference is known.
		 * The name of a TensorMeasurementReference is spelled-out human readable name of the measurement reference.
		 *
		 * For example, typical measurement references for (scalar) quantity speed are declared with the following humanId and name:
		 * <'m/s'> and 'metre per second',
		 * <'km/h'> and 'kilometre per hour',
		 * <'mi/h'> and 'mile per hour'.
		 *
		 * A measurement reference can have zero or more definitionalQuantityValues that allow to specify
		 * quantity values that carry a particular meaning or relevance for the measurement reference.
		 */
	
		attribute isBound: Boolean[1] default false;
		attribute order :>> rank;
		attribute mRefs: ScalarMeasurementReference[1..*] nonunique :>> elements;
		attribute definitionalQuantityValues: DefinitionalQuantityValue[0..*];
	}

	attribute def VectorMeasurementReference :> TensorMeasurementReference {
		doc
		/*
		 * A VectorMeasurementReference is a specialization of TensorMeasurementReference for vector quantities that are
		 * typed by a VectorQuantityValue. Its order is one. Implicitly, it defines a vector space of dimension `n` = dimensions[1].
		 * The magnitudes of the `n` basis unit vectors that span the vector space are defined by the mRefs which each are
		 * a ScalarMeasurementReference, typically a MeasurementUnit or an IntervalScale.
		 * 
		 * Attribute isOrthogonal declares whether the basis vectors of the vector space are orthogonal, i.e., whether all
		 * inner products of any pair of basis vectors are equal to zero.
		 * 
		 * A pair of a specialization of VectorQuantityValue and a specialization of VectorMeasurementReference can also be used to
		 * define a vector space for state vectors as used in state-space representation models.
		 */
	
		attribute :>> dimensions: Positive[0..1];
		attribute isOrthogonal: Boolean[1] default true;
	}

	abstract attribute def ScalarMeasurementReference :> VectorMeasurementReference {
		doc
		/*
		 * A ScalarMeasurementReference is a specialization of VectorMeasurementReference for scalar quantities
		 * that are typed by a ScalarQuantityValue and for components of tensor or vector quantities.
		 * Its order is zero. A ScalarMeasurementReference is also a generalization of MeasurementUnit and MeasurementScale.
		 * It establishes how to interpret the numerical value (num) of a ScalarQuantityValue or a component of
		 * a tensor or vector quantity value, and establishes its actual quantity dimension.
		 *
		 * Attribute mRefs is bound to self for a ScalarMeasurementReference, for consistency with tensor and vector measurement references,
		 * as the dimension or component of a scalar quantity is itself.
		 */
	
		attribute :>> dimensions = ();
		attribute :>> isOrthogonal = true;
		attribute :>> mRefs = self;
		attribute quantityDimension: QuantityDimension[1];
	}
	
	attribute def CoordinateFrame :> VectorMeasurementReference {
		doc
		/*
		 * CoordinateFrame is a VectorMeasurementReference with the specific purpose to quantify (i.e., coordinatize) a vector space, 
		 * and locate and orient it with respect to another CoordinateFrame.
		 * 
		 * Optional attribute transformation enables specification of the location and orientation of this CoordinateFrame as dependent
		 * and nested with respect to another (reference) coordinate frame. Typically the other CoordinateFrame is the frame of 
		 * the next higher element (Object, Item or Part) in a composite structure.
		 */
	
		attribute transformation: CoordinateTransformation[0..1] {
			attribute :>> target = that;
		}
	}

    attribute def '3dCoordinateFrame' :> CoordinateFrame {
        doc
    	/*
         * Most general 3-dimensional coordinate frame
         */
        attribute :>> dimensions = 3;
    }
    alias ThreeDCoordinateFrame for '3dCoordinateFrame';

    abstract attribute def CoordinateTransformation {
        doc
        /*
	     * CoordinateTransformation is the most general representation of the transformation of a target VectorMeasurementReference 
	     * with respect to a source VectorMeasurementReference.
	     */
	 	attribute source: VectorMeasurementReference[1];
	 	attribute target: VectorMeasurementReference[1];
	 	assert constraint validSourceTargetDimensions { source.dimensions == target.dimensions }
    }

	attribute def CoordinateFramePlacement :> CoordinateTransformation {
    	doc
    	/*
    	 * CoordinateFramePlacement is a CoordinateTransformation by placement of the target frame in the source frame.
    	 *  
    	 * Attribute origin specifies the location of the origin of the target frame as a vector in the source frame.
    	 * 
    	 * Attribute basisDirections specifies the orientation of the target frame by specifying the directions of 
    	 * the respective basis vectors of the target frame via direction vectors in the source frame. An empty sequence of
    	 * basisDirections signifies no change of orientation of the target coordinate frame.
    	 */

		attribute origin : VectorQuantityValue[1];
		attribute basisDirections : VectorQuantityValue[0..*] ordered nonunique;
		assert constraint validOriginDimensions { origin.dimensions == source.dimensions }
		assert constraint { size(basisDirections) == 0 or size(basisDirections) == source.dimensions#(1)}
        assert constraint validateBasisDirections { basisDirections->forAll { in basisDirection : VectorQuantityValue; 
            basisDirection.dimensions->equals(source.dimensions) }
        }
	 }

	abstract attribute def TranslationOrRotation {
		doc
		/*
		 * TranslationOrRotation is an abstract union of Translation and Rotation
		 */
	}

	attribute def Translation :> TranslationOrRotation {
		doc
		/*
		 * Representation of a translation with respect to a coordinate frame
		 * 
		 * Attribute translationVector specifies the displacement vector that constitutes the translation.
		 */
	
		attribute translationVector : VectorQuantityValue[1];
	}

	attribute def Rotation :> TranslationOrRotation {
		doc
		/*
		 * Representation of a rotation about an axis over an angle
		 * 
		 * Attribute axisDirection specifies the direction of the rotation axis.
		 * Attribute angle specifies the angle of rotation, where a positive value implies right-handed rotation.
		 * Attribute isIntrinsic asserts whether the intermediate coordinate frame moves with the rotation or not, 
		 * i.e. whether an instrinsic or extrinsic rotation is specified.
		 * 
		 * See https://en.wikipedia.org/wiki/Davenport_chained_rotations for details.
		 */
	
		attribute axisDirection : VectorQuantityValue[1];
		attribute angle :>> angularMeasure;
		attribute isIntrinsic : Boolean[1] default true;
	}

	attribute def TranslationRotationSequence :> CoordinateTransformation, List {
	doc
	/*
	 * Coordinate frame transformation specified by a sequence of translations and/or rotations
	 *
	 * Note: This is a coordinate transformation that is convenient for interpretation by humans.
	 * In particular a sequence of rotations about the principal axes of a coordinate frame is much more easy understandable 
	 * than a rotation about an arbitrary axis.
	 * Any sequence can be reduced to a single combination of a translation and a rotation about a particular axis, but in general 
	 * the original sequence cannot be retrieved as there are infinitely many sequences representing the reduced transformation.
	 */
	
		attribute :>> elements : TranslationOrRotation[1..*] ordered nonunique;
	}

	attribute def AffineTransformationMatrix3d :> CoordinateTransformation, Array {
		doc
		/*
		 * AffineTransformationMatrix3d is a three dimensional CoordinateTransformation specified via an affine transformation matrix
		 *
		 * The interpretation of the matrix is as follows:
		 * - the upper left 3x3 matrix represents the rotation matrix
		 * - the uper right 3x1 column vector represents the translation vector
		 * - the bottom row must be the row vector (0, 0, 0, 1).
		 *
		 * I.e. the matrix has the following form:
		 * ( R, R, R, T,
		 *   R, R, R, T,
		 *   R, R, R, T,
		 *   0, 0, 0, 1 )
		 * where the cells marked R form the rotation matrix and the cells marked T form the translation vector.
		 * 
		 * Note: See https://en.wikipedia.org/wiki/Transformation_matrix, under affine transformations for a general explanation.
		 */
	
		    attribute rotationMatrix : Array {
				attribute :>> elements : Real[9] ordered nonunique;
				attribute :>> dimensions = (3, 3);
		    }
			attribute translationVector : ThreeVectorValue[1] { :>> elements : Real[3]; }
			attribute :>> dimensions = (4, 4);
			attribute :>> elements : Real[16] ordered nonunique = (
				rotationMatrix.elements#(1), rotationMatrix.elements#(2), rotationMatrix.elements#(3), translationVector#(1),
				rotationMatrix.elements#(4), rotationMatrix.elements#(5), rotationMatrix.elements#(6), translationVector#(2),
				rotationMatrix.elements#(7), rotationMatrix.elements#(8), rotationMatrix.elements#(9), translationVector#(3),
				0, 0, 0, 1);
			assert constraint validSourceDimensions { source.dimensions == 3 }
	}

	attribute def NullTransformation :> AffineTransformationMatrix3d {
		doc
		/*
		 * NullTransformation is a three dimensional CoordinateTransformation that places the target CoordinateFrame at the
		 * same position and orientation as the source CoordinateFrame.
		 */
		 attribute :>> rotationMatrix {
		     attribute :>> elements = (1, 0, 0, 0, 1, 0, 0, 0, 1);
		 }
		 attribute :>> translationVector {
		     attribute :>> elements = (0, 0, 0);
		 }
 	}

	attribute nullTransformation : NullTransformation [1];

	abstract attribute def MeasurementUnit :> ScalarMeasurementReference {
		doc
		/*
		 * Representation of a measurement unit.
		 *
		 * Note: MeasurementUnit directly specializes ScalarMeasurementReference in order to allow for efficient and intuitive definition of a ratio scale.
		 *
		 * A MeasurementUnit can be used in two ways:
		 * 1. Directly as the mRef in a ScalarQuantityValue, which implies that the effective measurement reference is a ratio scale defined by the unit.
		 * 2. As the unit of a MeasurementScale.
		 *
		 * A MeasurementUnit specifies one or more UnitPowerFactors.
		 */
	
		attribute :>> isBound = false;
		attribute unitPowerFactors: UnitPowerFactor[0..*] ordered;
		attribute unitConversion: UnitConversion[0..1];
        assert constraint hasValidUnitPowerFactors : VerifyUnitPowerFactors {
        	in unitPowerFactors = MeasurementUnit::unitPowerFactors;
        	in quantityDimension = MeasurementUnit::quantityDimension;
		}
	}


	abstract attribute def SimpleUnit :> MeasurementUnit {
		doc
		/*
		 * Representation of a measurement unit that does not depend on any other measurement unit.
		 */
	
		private attribute simpleUnitSelf: SimpleUnit = self;
	    attribute :>> unitPowerFactors: UnitPowerFactor[1] {
			attribute unit :>> UnitPowerFactor::unit = simpleUnitSelf;
			attribute exponent :>> UnitPowerFactor::exponent = 1;
		}
	}


	abstract attribute def DerivedUnit :> MeasurementUnit {
		doc
		/*
		 * Representation of a derived measurement unit that depends on one or more powers of other measurement units.
		 *
		 * VIM defines "derived unit" as "measurement unit for a derived quantity", see https://jcgm.bipm.org/vim/en/1.11.html .
		 */
	}


	attribute def UnitPowerFactor {
		doc
		/*
		 * Representation of a measurement unit power factor, which is a tuple
		 * of a referenced measurement unit and an exponent.
		 */
	
		attribute unit: MeasurementUnit;
		attribute exponent: Real;
	}

	abstract attribute def UnitConversion {
		doc
		/*
		 * Representation of the linear conversion relationship between one measurement unit and another measurement unit, that acts as a reference.
		 *
		 * Attribute isExact asserts whether the conversionFactor is exact or not. By default it is set true.
		 */
	
		attribute referenceUnit: MeasurementUnit;
		attribute conversionFactor: Real;
		attribute isExact: Boolean default true;
	}

	attribute def ConversionByConvention :> UnitConversion {
		doc
		/*
		 * Representation of a UnitConversion that is defined according to some convention.
		 */
	}

	attribute def ConversionByPrefix :> UnitConversion {
		doc
		/*
		 * Representation of a UnitConversion that is defined through reference to a named unit prefix,
		 * that in turn represents a decimal or binary multiple or sub-multiple, as defined in ISO/IEC 80000-1.
		 *
		 * Note: The actual value of the conversion factor is derived from the definition of the unit prefix.
		 *
		 * Examples: kilometre for conversion factor 1000 with reference unit metre, nanofarad for 1E-9 farad.
		 */
	
		attribute prefix: UnitPrefix[1];
		attribute conversionFactor redefines UnitConversion::conversionFactor = prefix.conversionFactor;
	}

	attribute def UnitPrefix {
		doc
		/*
		 * Representation of a multiple or sub-multiple measurement unit prefix as defined in ISO/IEC 80000-1.
		 */
	
		attribute longName: String;
		attribute symbol: String;
		attribute conversionFactor: Real;
	}


	abstract attribute def MeasurementScale :> ScalarMeasurementReference {
		doc
		/*
		 * Representation of a non-ratio measurement scale as opposed to a ratio measurement scale defined by a MeasurementUnit.
		 *
		 * Note: A ratio scale is implied by direct use of a MeasurementUnit as the mRef in a ScalarQuantityValue.
		 */
	
		attribute unit: MeasurementUnit;
		attribute quantityValueMapping: QuantityValueMapping[0..1];
	}

	attribute def OrdinalScale :> MeasurementScale {
		doc
		/*
		 * Representation of an ordinal measurement scale.
		 */
	}

	attribute def IntervalScale :> MeasurementScale, CoordinateFrame {
		doc
		/*
		 * Representation of an interval measurement scale.
		 *
		 * An IntervalScale is also a CoordinateFrame
		 * The offset of one interval measurement scale w.r.t. another interval or ratio scale is defined through a quantityValueMapping, see MeasurementReference.
		 */
	
		attribute :>> isBound = true;
	}

	attribute def CyclicRatioScale :> MeasurementScale {
		doc
		/*
		 * Representation of a ratio measurement scale with a periodic cycle.
		 *
		 * Note: The magnitude of the periodic cycle is defined by the modulus of the scale.
		 * Example: Planar angle with modulus 360 degrees, therefore on such a cyclic ratio scale,
		 * an angle of 450 degrees is equivalent to an angle of 90 degrees, and -60 degrees is equivalent to 300 degrees.
		 */
	
		attribute modulus: Number;
	}

	attribute def LogarithmicScale :> MeasurementScale {
		doc
		/*
		 * Representation of a logarithmic measurement scale
		 *
		 * The magnitude v of a ratio quantity value expressed on a logarithmic scale
		 * for a magnitude x of a quantity value expressed on a ratio scale is computed as follows:
		 *   v = f * log_base( (x / x_ref )^a )
	     * where:
		 *   f is a multiplication factor,
	     *   log_base is the log function for the given logarithm base,
	     *   x is the actual quantity,
	     *   x_ref is a reference quantity,
	     *   a is an exponent.
		 */
	
		attribute logarithmBase: Number;
		attribute factor: Number;
		attribute exponent: Number;
		attribute referenceQuantity: ScalarQuantityValue[0..1];
	}

	attribute def QuantityValueMapping {
		doc
		/*
		 * Representation of the mapping of equivalent quantity values expressed on two different MeasurementReferences
		 *
		 * A QuantityValueMapping specifies a mapping from a given mappedQuantityValue owned by the MeasurementReference
		 * that owns the QuantityValueMapping to a referenceQuantityValue owned by another MeasurementReference.
		 *
		 * Example: The mapping between the temperature value of 0.01 degree Celsius on the celsius temperature scale
		 * to the equivalent temperature value of 273.16 K on the kelvin temperature scale,
		 * would specify a mappedQuantityValue referencing the
		 * the DefinitionalQuantityValue (0.01, "absolute thermodynamic temperature of the triple point of water")
		 * of the celsius interval scale, and a referenceQuantityValue referencing the
		 * DefinitionalQuantityValue (273.16, "absolute thermodynamic temperature of the triple point of water")
		 * of the kelvin ratio scale.
		 */
	
		attribute mappedQuantityValue: DefinitionalQuantityValue;
		attribute referenceQuantityValue: DefinitionalQuantityValue;
	}

	attribute def DefinitionalQuantityValue {
		doc
		/*
		 * Representation of a particular quantity value that is used in the definition of a TensorMeasurementReference
		 *
		 * Typically such a particular value is defined by convention. It can be used to define a selected reference value,
		 * such as the meaning of zero on a measurement scale or the origin of a top-level coordinate frame.
		 *
		 * Example: The 'kelvin' MeasurementReference for thermodynamic temperature could have a
		 * DefinitionalQuantityValue {
		 *     :>> num = 273.16;
		 *     :>> definition = "thermodynamic temperature of the triple point of Vienna Standard Mean Ocean Water in kelvin";
		 * }
		 * that is value of the definition of the scale.
		 */
	
		attribute num: Number[1..*];
		attribute definition: String;
	}

	attribute def DimensionOneUnit :> DerivedUnit {
		doc
		/*
		 * Explicit definition of "unit of dimension one", also known as "dimensionless unit".
		 */
	
		attribute :>> unitPowerFactors = ();
	}
	attribute def DimensionOneValue :> ScalarQuantityValue {
		doc
		/*
		 * A ScalarQuantityValue with a DimensionOneUnit.
		 */
		attribute :>> num: Real;
		attribute :>> mRef: DimensionOneUnit;
	}
	attribute dimensionOneQuantities : DimensionOneValue[*] nonunique :> scalarQuantities;

	attribute one : DimensionOneUnit[1] = new DimensionOneUnit();

	attribute def CountValue :> DimensionOneValue {
		doc
		/*
		 * Explicit definition of a generic "count" quantity as a DimensionOneValue.
		 */
	}
	attribute countQuantities : CountValue[*] nonunique :> dimensionOneQuantities;

	attribute def SystemOfUnits {
		doc
		/*
		 * A SystemOfUnits represents the essentials of [VIM] concept "system of units" (https://jcgm.bipm.org/vim/en/1.13.html), defined as a
		 * "set of base units and derived units, together with their multiples and submultiples, defined in accordance with given rules,
		 * for a given system of quantities".
		 * The base units are a particular selection of measurement units for each of the base quantities of a system of quantities,
		 * that form the basis on top of which all other (derived) units are defined.
		 *
		 * Attribute systemOfQuantities speficies the associated SystemOfQuantities.
		 */
	
		attribute longName: String[1];
		attribute systemOfQuantities : SystemOfQuantities[1];
		attribute baseUnits: SimpleUnit[1..*] ordered;
	}

    constraint def VerifyUnitPowerFactors {
		doc
		/*
		 * Constraint definition to verify that the given unit power factors comply with the required quantity dimension
		 */
	
    	in unitPowerFactors: UnitPowerFactor[*] ordered;
    	in quantityDimension: QuantityDimension[1];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "measurement_references.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package MeasurementReferences {
    doc
    /*
	 * This package defines the representations for measurement references.
	 */
    private import Collections::Array;
    private import Collections::List;
    private import ScalarValues::*;
    private import VectorValues::ThreeVectorValue;
    private import SequenceFunctions::size;
    private import SequenceFunctions::equals;
    private import ControlFunctions::forAll;
    private import Quantities::QuantityDimension;
    private import Quantities::VectorQuantityValue;
    private import Quantities::scalarQuantities;
    private import Quantities::ScalarQuantityValue;
    private import Quantities::SystemOfQuantities;
    private import ISQSpaceTime::angularMeasure;
    attribute def TensorMeasurementReference :> Array {
        doc
        /*
		 * TensorMeasurementReference is the most general AttributeDefinition to represent measurement references.
		 *
		 * The concept "measurement reference" is defined in [VIM] "quantity" NOTE 2 as "A reference can be a measurement unit,
		 * a measurement procedure, a reference material, or a combination of such.", see https://jcgm.bipm.org/vim/en/1.1.html .
		 * In addition [VIM] "quantity" NOTE 5 states that "A quantity as defined here is a scalar. However, a vector or a tensor, 
		 * the components of which are quantities, is also considered to be a quantity". However, the rest of [VIM] does not explicitly 
		 * define how tensor and vector quantities can be or should be supported.
		 *
		 * In this package, in line with TensorQuantityValue in package Quantities, the most general kind of measurement reference
		 * is TensorMeasurementReference that represents a measurement reference for any order of tensor quantity. Since the order can 
		 * also be one or zero, this includes vector and scalar quantities. The specializations VectorMeasurementReference and 
		 * ScalarMeasurementReference are defined to specifically represent measurement references for vector and scalar quantities.
		 * 
		 * TensorMeasurementReference specializes Array, which provides its multi-dimensional structure. The order of a tensor is equivalent
		 * to the rank of an Array.
		 * 
		 * Attribute isBound specifies whether the vector space product is bound (isBound is true) or free (isBound is false).
		 * 
		 * Attribute mRefs specifies the scalar measurement references for all dimensions of a tensor quantity.
		 *
		 * The short name of a TensorMeasurementReference is the unique symbol by which the measurement reference is known.
		 * The name of a TensorMeasurementReference is spelled-out human readable name of the measurement reference.
		 *
		 * For example, typical measurement references for (scalar) quantity speed are declared with the following humanId and name:
		 * <'m/s'> and 'metre per second',
		 * <'km/h'> and 'kilometre per hour',
		 * <'mi/h'> and 'mile per hour'.
		 *
		 * A measurement reference can have zero or more definitionalQuantityValues that allow to specify
		 * quantity values that carry a particular meaning or relevance for the measurement reference.
		 */
        attribute isBound : Boolean[1] default false;
        attribute order :>> rank;
        attribute mRefs : ScalarMeasurementReference[1..*] nonunique :>> elements;
        attribute definitionalQuantityValues : DefinitionalQuantityValue[0..*];
    }
    attribute def VectorMeasurementReference :> TensorMeasurementReference {
        doc
        /*
		 * A VectorMeasurementReference is a specialization of TensorMeasurementReference for vector quantities that are
		 * typed by a VectorQuantityValue. Its order is one. Implicitly, it defines a vector space of dimension `n` = dimensions[1].
		 * The magnitudes of the `n` basis unit vectors that span the vector space are defined by the mRefs which each are
		 * a ScalarMeasurementReference, typically a MeasurementUnit or an IntervalScale.
		 * 
		 * Attribute isOrthogonal declares whether the basis vectors of the vector space are orthogonal, i.e., whether all
		 * inner products of any pair of basis vectors are equal to zero.
		 * 
		 * A pair of a specialization of VectorQuantityValue and a specialization of VectorMeasurementReference can also be used to
		 * define a vector space for state vectors as used in state-space representation models.
		 */
        attribute :>> dimensions : Positive[0..1];
        attribute isOrthogonal : Boolean[1] default true;
    }
    abstract attribute def ScalarMeasurementReference :> VectorMeasurementReference {
        doc
        /*
		 * A ScalarMeasurementReference is a specialization of VectorMeasurementReference for scalar quantities
		 * that are typed by a ScalarQuantityValue and for components of tensor or vector quantities.
		 * Its order is zero. A ScalarMeasurementReference is also a generalization of MeasurementUnit and MeasurementScale.
		 * It establishes how to interpret the numerical value (num) of a ScalarQuantityValue or a component of
		 * a tensor or vector quantity value, and establishes its actual quantity dimension.
		 *
		 * Attribute mRefs is bound to self for a ScalarMeasurementReference, for consistency with tensor and vector measurement references,
		 * as the dimension or component of a scalar quantity is itself.
		 */
        attribute :>> dimensions = null;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs = self;
        attribute quantityDimension : QuantityDimension[1];
    }
    attribute def CoordinateFrame :> VectorMeasurementReference {
        doc
        /*
		 * CoordinateFrame is a VectorMeasurementReference with the specific purpose to quantify (i.e., coordinatize) a vector space, 
		 * and locate and orient it with respect to another CoordinateFrame.
		 * 
		 * Optional attribute transformation enables specification of the location and orientation of this CoordinateFrame as dependent
		 * and nested with respect to another (reference) coordinate frame. Typically the other CoordinateFrame is the frame of 
		 * the next higher element (Object, Item or Part) in a composite structure.
		 */
        attribute transformation : CoordinateTransformation[0..1] {
            attribute :>> target = that;
        }
    }
    attribute def '3dCoordinateFrame' :> CoordinateFrame {
        doc
        /*
         * Most general 3-dimensional coordinate frame
         */
        attribute :>> dimensions = 3;
    }
    alias ThreeDCoordinateFrame for '3dCoordinateFrame';
    abstract attribute def CoordinateTransformation {
        doc
        /*
	     * CoordinateTransformation is the most general representation of the transformation of a target VectorMeasurementReference 
	     * with respect to a source VectorMeasurementReference.
	     */
        attribute source : VectorMeasurementReference[1];
        attribute target : VectorMeasurementReference[1];
        assert constraint validSourceTargetDimensions {
            source.dimensions == target.dimensions;
        }
    }
    attribute def CoordinateFramePlacement :> CoordinateTransformation {
        doc
        /*
    	 * CoordinateFramePlacement is a CoordinateTransformation by placement of the target frame in the source frame.
    	 *  
    	 * Attribute origin specifies the location of the origin of the target frame as a vector in the source frame.
    	 * 
    	 * Attribute basisDirections specifies the orientation of the target frame by specifying the directions of 
    	 * the respective basis vectors of the target frame via direction vectors in the source frame. An empty sequence of
    	 * basisDirections signifies no change of orientation of the target coordinate frame.
    	 */
        attribute origin : VectorQuantityValue[1];
        attribute basisDirections : VectorQuantityValue[0..*] ordered nonunique;
        assert constraint validOriginDimensions {
            origin.dimensions == source.dimensions;
        }
        assert constraint {
            size(basisDirections) == 0 || size(basisDirections) == source.dimensions#(1);
        }
        assert constraint validateBasisDirections {
            basisDirections->forAll { in basisDirection : VectorQuantityValue; basisDirection.dimensions->equals(source.dimensions) };
        }
    }
    abstract attribute def TranslationOrRotation {
        doc
        /*
		 * TranslationOrRotation is an abstract union of Translation and Rotation
		 */
    }
    attribute def Translation :> TranslationOrRotation {
        doc
        /*
		 * Representation of a translation with respect to a coordinate frame
		 * 
		 * Attribute translationVector specifies the displacement vector that constitutes the translation.
		 */
        attribute translationVector : VectorQuantityValue[1];
    }
    attribute def Rotation :> TranslationOrRotation {
        doc
        /*
		 * Representation of a rotation about an axis over an angle
		 * 
		 * Attribute axisDirection specifies the direction of the rotation axis.
		 * Attribute angle specifies the angle of rotation, where a positive value implies right-handed rotation.
		 * Attribute isIntrinsic asserts whether the intermediate coordinate frame moves with the rotation or not, 
		 * i.e. whether an instrinsic or extrinsic rotation is specified.
		 * 
		 * See https://en.wikipedia.org/wiki/Davenport_chained_rotations for details.
		 */
        attribute axisDirection : VectorQuantityValue[1];
        attribute angle :>> angularMeasure;
        attribute isIntrinsic : Boolean[1] default true;
    }
    attribute def TranslationRotationSequence :> CoordinateTransformation, List {
        doc
        /*
	 * Coordinate frame transformation specified by a sequence of translations and/or rotations
	 *
	 * Note: This is a coordinate transformation that is convenient for interpretation by humans.
	 * In particular a sequence of rotations about the principal axes of a coordinate frame is much more easy understandable 
	 * than a rotation about an arbitrary axis.
	 * Any sequence can be reduced to a single combination of a translation and a rotation about a particular axis, but in general 
	 * the original sequence cannot be retrieved as there are infinitely many sequences representing the reduced transformation.
	 */
        attribute :>> elements : TranslationOrRotation[1..*] ordered nonunique;
    }
    attribute def AffineTransformationMatrix3d :> CoordinateTransformation, Array {
        doc
        /*
		 * AffineTransformationMatrix3d is a three dimensional CoordinateTransformation specified via an affine transformation matrix
		 *
		 * The interpretation of the matrix is as follows:
		 * - the upper left 3x3 matrix represents the rotation matrix
		 * - the uper right 3x1 column vector represents the translation vector
		 * - the bottom row must be the row vector (0, 0, 0, 1).
		 *
		 * I.e. the matrix has the following form:
		 * ( R, R, R, T,
		 *   R, R, R, T,
		 *   R, R, R, T,
		 *   0, 0, 0, 1 )
		 * where the cells marked R form the rotation matrix and the cells marked T form the translation vector.
		 * 
		 * Note: See https://en.wikipedia.org/wiki/Transformation_matrix, under affine transformations for a general explanation.
		 */
        attribute rotationMatrix : Array {
            attribute :>> elements : Real[9] ordered nonunique;
            attribute :>> dimensions = (3, 3);
        }
        attribute translationVector : ThreeVectorValue[1] {
             : Real[3] :>> elements;
        }
        attribute :>> dimensions = (4, 4);
        attribute :>> elements : Real[16] ordered nonunique = (rotationMatrix.elements#(1), rotationMatrix.elements#(2), rotationMatrix.elements#(3), translationVector#(1), rotationMatrix.elements#(4), rotationMatrix.elements#(5), rotationMatrix.elements#(6), translationVector#(2), rotationMatrix.elements#(7), rotationMatrix.elements#(8), rotationMatrix.elements#(9), translationVector#(3), 0, 0, 0, 1);
        assert constraint validSourceDimensions {
            source.dimensions == 3;
        }
    }
    attribute def NullTransformation :> AffineTransformationMatrix3d {
        doc
        /*
		 * NullTransformation is a three dimensional CoordinateTransformation that places the target CoordinateFrame at the
		 * same position and orientation as the source CoordinateFrame.
		 */
        attribute :>> rotationMatrix {
            attribute :>> elements = (1, 0, 0, 0, 1, 0, 0, 0, 1);
        }
        attribute :>> translationVector {
            attribute :>> elements = (0, 0, 0);
        }
    }
    attribute nullTransformation : NullTransformation[1];
    abstract attribute def MeasurementUnit :> ScalarMeasurementReference {
        doc
        /*
		 * Representation of a measurement unit.
		 *
		 * Note: MeasurementUnit directly specializes ScalarMeasurementReference in order to allow for efficient and intuitive definition of a ratio scale.
		 *
		 * A MeasurementUnit can be used in two ways:
		 * 1. Directly as the mRef in a ScalarQuantityValue, which implies that the effective measurement reference is a ratio scale defined by the unit.
		 * 2. As the unit of a MeasurementScale.
		 *
		 * A MeasurementUnit specifies one or more UnitPowerFactors.
		 */
        attribute :>> isBound = false;
        attribute unitPowerFactors : UnitPowerFactor[0..*] ordered;
        attribute unitConversion : UnitConversion[0..1];
        assert constraint hasValidUnitPowerFactors : VerifyUnitPowerFactors {
            in unitPowerFactors = MeasurementUnit::unitPowerFactors;
            in quantityDimension = MeasurementUnit::quantityDimension;
        }
    }
    abstract attribute def SimpleUnit :> MeasurementUnit {
        doc
        /*
		 * Representation of a measurement unit that does not depend on any other measurement unit.
		 */
        private attribute simpleUnitSelf : SimpleUnit = self;
        attribute :>> unitPowerFactors : UnitPowerFactor[1] {
            attribute unit :>> UnitPowerFactor::unit = simpleUnitSelf;
            attribute exponent :>> UnitPowerFactor::exponent = 1;
        }
    }
    abstract attribute def DerivedUnit :> MeasurementUnit {
        doc
        /*
		 * Representation of a derived measurement unit that depends on one or more powers of other measurement units.
		 *
		 * VIM defines "derived unit" as "measurement unit for a derived quantity", see https://jcgm.bipm.org/vim/en/1.11.html .
		 */
    }
    attribute def UnitPowerFactor {
        doc
        /*
		 * Representation of a measurement unit power factor, which is a tuple
		 * of a referenced measurement unit and an exponent.
		 */
        attribute unit : MeasurementUnit;
        attribute exponent : Real;
    }
    abstract attribute def UnitConversion {
        doc
        /*
		 * Representation of the linear conversion relationship between one measurement unit and another measurement unit, that acts as a reference.
		 *
		 * Attribute isExact asserts whether the conversionFactor is exact or not. By default it is set true.
		 */
        attribute referenceUnit : MeasurementUnit;
        attribute conversionFactor : Real;
        attribute isExact : Boolean default true;
    }
    attribute def ConversionByConvention :> UnitConversion {
        doc
        /*
		 * Representation of a UnitConversion that is defined according to some convention.
		 */
    }
    attribute def ConversionByPrefix :> UnitConversion {
        doc
        /*
		 * Representation of a UnitConversion that is defined through reference to a named unit prefix,
		 * that in turn represents a decimal or binary multiple or sub-multiple, as defined in ISO/IEC 80000-1.
		 *
		 * Note: The actual value of the conversion factor is derived from the definition of the unit prefix.
		 *
		 * Examples: kilometre for conversion factor 1000 with reference unit metre, nanofarad for 1E-9 farad.
		 */
        attribute prefix : UnitPrefix[1];
        attribute conversionFactor redefines UnitConversion::conversionFactor = prefix.conversionFactor;
    }
    attribute def UnitPrefix {
        doc
        /*
		 * Representation of a multiple or sub-multiple measurement unit prefix as defined in ISO/IEC 80000-1.
		 */
        attribute longName : String;
        attribute symbol : String;
        attribute conversionFactor : Real;
    }
    abstract attribute def MeasurementScale :> ScalarMeasurementReference {
        doc
        /*
		 * Representation of a non-ratio measurement scale as opposed to a ratio measurement scale defined by a MeasurementUnit.
		 *
		 * Note: A ratio scale is implied by direct use of a MeasurementUnit as the mRef in a ScalarQuantityValue.
		 */
        attribute unit : MeasurementUnit;
        attribute quantityValueMapping : QuantityValueMapping[0..1];
    }
    attribute def OrdinalScale :> MeasurementScale {
        doc
        /*
		 * Representation of an ordinal measurement scale.
		 */
    }
    attribute def IntervalScale :> MeasurementScale, CoordinateFrame {
        doc
        /*
		 * Representation of an interval measurement scale.
		 *
		 * An IntervalScale is also a CoordinateFrame
		 * The offset of one interval measurement scale w.r.t. another interval or ratio scale is defined through a quantityValueMapping, see MeasurementReference.
		 */
        attribute :>> isBound = true;
    }
    attribute def CyclicRatioScale :> MeasurementScale {
        doc
        /*
		 * Representation of a ratio measurement scale with a periodic cycle.
		 *
		 * Note: The magnitude of the periodic cycle is defined by the modulus of the scale.
		 * Example: Planar angle with modulus 360 degrees, therefore on such a cyclic ratio scale,
		 * an angle of 450 degrees is equivalent to an angle of 90 degrees, and -60 degrees is equivalent to 300 degrees.
		 */
        attribute modulus : Number;
    }
    attribute def LogarithmicScale :> MeasurementScale {
        doc
        /*
		 * Representation of a logarithmic measurement scale
		 *
		 * The magnitude v of a ratio quantity value expressed on a logarithmic scale
		 * for a magnitude x of a quantity value expressed on a ratio scale is computed as follows:
		 *   v = f * log_base( (x / x_ref )^a )
	     * where:
		 *   f is a multiplication factor,
	     *   log_base is the log function for the given logarithm base,
	     *   x is the actual quantity,
	     *   x_ref is a reference quantity,
	     *   a is an exponent.
		 */
        attribute logarithmBase : Number;
        attribute factor : Number;
        attribute exponent : Number;
        attribute referenceQuantity : ScalarQuantityValue[0..1];
    }
    attribute def QuantityValueMapping {
        doc
        /*
		 * Representation of the mapping of equivalent quantity values expressed on two different MeasurementReferences
		 *
		 * A QuantityValueMapping specifies a mapping from a given mappedQuantityValue owned by the MeasurementReference
		 * that owns the QuantityValueMapping to a referenceQuantityValue owned by another MeasurementReference.
		 *
		 * Example: The mapping between the temperature value of 0.01 degree Celsius on the celsius temperature scale
		 * to the equivalent temperature value of 273.16 K on the kelvin temperature scale,
		 * would specify a mappedQuantityValue referencing the
		 * the DefinitionalQuantityValue (0.01, "absolute thermodynamic temperature of the triple point of water")
		 * of the celsius interval scale, and a referenceQuantityValue referencing the
		 * DefinitionalQuantityValue (273.16, "absolute thermodynamic temperature of the triple point of water")
		 * of the kelvin ratio scale.
		 */
        attribute mappedQuantityValue : DefinitionalQuantityValue;
        attribute referenceQuantityValue : DefinitionalQuantityValue;
    }
    attribute def DefinitionalQuantityValue {
        doc
        /*
		 * Representation of a particular quantity value that is used in the definition of a TensorMeasurementReference
		 *
		 * Typically such a particular value is defined by convention. It can be used to define a selected reference value,
		 * such as the meaning of zero on a measurement scale or the origin of a top-level coordinate frame.
		 *
		 * Example: The 'kelvin' MeasurementReference for thermodynamic temperature could have a
		 * DefinitionalQuantityValue {
		 *     :>> num = 273.16;
		 *     :>> definition = "thermodynamic temperature of the triple point of Vienna Standard Mean Ocean Water in kelvin";
		 * }
		 * that is value of the definition of the scale.
		 */
        attribute num : Number[1..*];
        attribute definition : String;
    }
    attribute def DimensionOneUnit :> DerivedUnit {
        doc
        /*
		 * Explicit definition of "unit of dimension one", also known as "dimensionless unit".
		 */
        attribute :>> unitPowerFactors = null;
    }
    attribute def DimensionOneValue :> ScalarQuantityValue {
        doc
        /*
		 * A ScalarQuantityValue with a DimensionOneUnit.
		 */
        attribute :>> num : Real;
        attribute :>> mRef : DimensionOneUnit;
    }
    attribute dimensionOneQuantities : DimensionOneValue[*] nonunique :> scalarQuantities;
    attribute one : DimensionOneUnit[1] = new DimensionOneUnit();
    attribute def CountValue :> DimensionOneValue {
        doc
        /*
		 * Explicit definition of a generic "count" quantity as a DimensionOneValue.
		 */
    }
    attribute countQuantities : CountValue[*] nonunique :> dimensionOneQuantities;
    attribute def SystemOfUnits {
        doc
        /*
		 * A SystemOfUnits represents the essentials of [VIM] concept "system of units" (https://jcgm.bipm.org/vim/en/1.13.html), defined as a
		 * "set of base units and derived units, together with their multiples and submultiples, defined in accordance with given rules,
		 * for a given system of quantities".
		 * The base units are a particular selection of measurement units for each of the base quantities of a system of quantities,
		 * that form the basis on top of which all other (derived) units are defined.
		 *
		 * Attribute systemOfQuantities speficies the associated SystemOfQuantities.
		 */
        attribute longName : String[1];
        attribute systemOfQuantities : SystemOfQuantities[1];
        attribute baseUnits : SimpleUnit[1..*] ordered;
    }
    constraint def VerifyUnitPowerFactors {
        doc
        /*
		 * Constraint definition to verify that the given unit power factors comply with the required quantity dimension
		 */
        in unitPowerFactors : UnitPowerFactor[*] ordered;
        in quantityDimension : QuantityDimension[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 153) (line 7) (column 17) (len 18)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 153) (line 7) (column 17) (len 11))) (segment 1 (token "Array") (name "Array") (separator colon-colon) (span (offset 166) (line 7) (column 30) (len 5)))))
    (reference r1 (scope relative) (span (offset 189) (line 8) (column 17) (len 17)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 189) (line 8) (column 17) (len 11))) (segment 1 (token "List") (name "List") (separator colon-colon) (span (offset 202) (line 8) (column 30) (len 4)))))
    (reference r2 (scope relative) (span (offset 224) (line 9) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 224) (line 9) (column 17) (len 12)))))
    (reference r3 (scope relative) (span (offset 257) (line 10) (column 17) (len 30)) (segments (segment 0 (token "VectorValues") (name "VectorValues") (separator none) (span (offset 257) (line 10) (column 17) (len 12))) (segment 1 (token "ThreeVectorValue") (name "ThreeVectorValue") (separator colon-colon) (span (offset 271) (line 10) (column 31) (len 16)))))
    (reference r4 (scope relative) (span (offset 306) (line 12) (column 17) (len 23)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 306) (line 12) (column 17) (len 17))) (segment 1 (token "size") (name "size") (separator colon-colon) (span (offset 325) (line 12) (column 36) (len 4)))))
    (reference r5 (scope relative) (span (offset 347) (line 13) (column 17) (len 25)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 347) (line 13) (column 17) (len 17))) (segment 1 (token "equals") (name "equals") (separator colon-colon) (span (offset 366) (line 13) (column 36) (len 6)))))
    (reference r6 (scope relative) (span (offset 390) (line 14) (column 17) (len 24)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 390) (line 14) (column 17) (len 16))) (segment 1 (token "forAll") (name "forAll") (separator colon-colon) (span (offset 408) (line 14) (column 35) (len 6)))))
    (reference r7 (scope relative) (span (offset 432) (line 15) (column 17) (len 29)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 432) (line 15) (column 17) (len 10))) (segment 1 (token "QuantityDimension") (name "QuantityDimension") (separator colon-colon) (span (offset 444) (line 15) (column 29) (len 17)))))
    (reference r8 (scope relative) (span (offset 479) (line 16) (column 17) (len 31)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 479) (line 16) (column 17) (len 10))) (segment 1 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator colon-colon) (span (offset 491) (line 16) (column 29) (len 19)))))
    (reference r9 (scope relative) (span (offset 528) (line 17) (column 17) (len 28)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 528) (line 17) (column 17) (len 10))) (segment 1 (token "scalarQuantities") (name "scalarQuantities") (separator colon-colon) (span (offset 540) (line 17) (column 29) (len 16)))))
    (reference r10 (scope relative) (span (offset 574) (line 18) (column 17) (len 31)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 574) (line 18) (column 17) (len 10))) (segment 1 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator colon-colon) (span (offset 586) (line 18) (column 29) (len 19)))))
    (reference r11 (scope relative) (span (offset 623) (line 19) (column 17) (len 30)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 623) (line 19) (column 17) (len 10))) (segment 1 (token "SystemOfQuantities") (name "SystemOfQuantities") (separator colon-colon) (span (offset 635) (line 19) (column 29) (len 18)))))
    (reference r12 (scope relative) (span (offset 671) (line 20) (column 17) (len 28)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 671) (line 20) (column 17) (len 12))) (segment 1 (token "angularMeasure") (name "angularMeasure") (separator colon-colon) (span (offset 685) (line 20) (column 31) (len 14)))))
    (reference r13 (scope relative) (span (offset 747) (line 22) (column 46) (len 5)) (segments (segment 0 (token "Array") (name "Array") (separator none) (span (offset 747) (line 22) (column 46) (len 5)))))
    (reference r14 (scope relative) (span (offset 3087) (line 57) (column 22) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 3087) (line 57) (column 22) (len 7)))))
    (reference r15 (scope relative) (span (offset 3135) (line 58) (column 23) (len 4)) (segments (segment 0 (token "rank") (name "rank") (separator none) (span (offset 3135) (line 58) (column 23) (len 4)))))
    (reference r16 (scope relative) (span (offset 3160) (line 59) (column 20) (len 26)) (segments (segment 0 (token "ScalarMeasurementReference") (name "ScalarMeasurementReference") (separator none) (span (offset 3160) (line 59) (column 20) (len 26)))))
    (reference r17 (scope relative) (span (offset 3207) (line 59) (column 67) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 3207) (line 59) (column 67) (len 8)))))
    (reference r18 (scope relative) (span (offset 3257) (line 60) (column 41) (len 25)) (segments (segment 0 (token "DefinitionalQuantityValue") (name "DefinitionalQuantityValue") (separator none) (span (offset 3257) (line 60) (column 41) (len 25)))))
    (reference r19 (scope relative) (span (offset 3339) (line 63) (column 46) (len 26)) (segments (segment 0 (token "TensorMeasurementReference") (name "TensorMeasurementReference") (separator none) (span (offset 3339) (line 63) (column 46) (len 26)))))
    (reference r20 (scope relative) (span (offset 4272) (line 78) (column 29) (len 8)) (segments (segment 0 (token "Positive") (name "Positive") (separator none) (span (offset 4272) (line 78) (column 29) (len 8)))))
    (reference r21 (scope relative) (span (offset 4260) (line 78) (column 17) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 4260) (line 78) (column 17) (len 10)))))
    (reference r22 (scope relative) (span (offset 4314) (line 79) (column 27) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 4314) (line 79) (column 27) (len 7)))))
    (reference r23 (scope relative) (span (offset 4397) (line 82) (column 55) (len 26)) (segments (segment 0 (token "VectorMeasurementReference") (name "VectorMeasurementReference") (separator none) (span (offset 4397) (line 82) (column 55) (len 26)))))
    (reference r24 (scope relative) (span (offset 5182) (line 95) (column 17) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 5182) (line 95) (column 17) (len 10)))))
    (reference r25 (scope relative) (span (offset 5215) (line 96) (column 17) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 5215) (line 96) (column 17) (len 12)))))
    (reference r26 (scope relative) (span (offset 5252) (line 97) (column 17) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 5252) (line 97) (column 17) (len 5)))))
    (reference r27 (scope relative) (span (offset 5260) (line 97) (column 25) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 5260) (line 97) (column 25) (len 4)))))
    (reference r28 (scope relative) (span (offset 5297) (line 98) (column 32) (len 17)) (segments (segment 0 (token "QuantityDimension") (name "QuantityDimension") (separator none) (span (offset 5297) (line 98) (column 32) (len 17)))))
    (reference r29 (scope relative) (span (offset 5358) (line 101) (column 35) (len 26)) (segments (segment 0 (token "VectorMeasurementReference") (name "VectorMeasurementReference") (separator none) (span (offset 5358) (line 101) (column 35) (len 26)))))
    (reference r30 (scope relative) (span (offset 5971) (line 112) (column 29) (len 24)) (segments (segment 0 (token "CoordinateTransformation") (name "CoordinateTransformation") (separator none) (span (offset 5971) (line 112) (column 29) (len 24)))))
    (reference r31 (scope relative) (span (offset 6021) (line 113) (column 18) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 6021) (line 113) (column 18) (len 6)))))
    (reference r32 (scope relative) (span (offset 6030) (line 113) (column 27) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 6030) (line 113) (column 27) (len 4)))))
    (reference r33 (scope relative) (span (offset 6085) (line 117) (column 42) (len 15)) (segments (segment 0 (token "CoordinateFrame") (name "CoordinateFrame") (separator none) (span (offset 6085) (line 117) (column 42) (len 15)))))
    (reference r34 (scope relative) (span (offset 6212) (line 122) (column 23) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 6212) (line 122) (column 23) (len 10)))))
    (reference r35 (scope relative) (span (offset 6270) (line 124) (column 37) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 6270) (line 124) (column 37) (len 19)))))
    (reference r36 (scope relative) (span (offset 6590) (line 132) (column 22) (len 26)) (segments (segment 0 (token "VectorMeasurementReference") (name "VectorMeasurementReference") (separator none) (span (offset 6590) (line 132) (column 22) (len 26)))))
    (reference r37 (scope relative) (span (offset 6642) (line 133) (column 22) (len 26)) (segments (segment 0 (token "VectorMeasurementReference") (name "VectorMeasurementReference") (separator none) (span (offset 6642) (line 133) (column 22) (len 26)))))
    (reference r38 (scope relative) (span (offset 6815) (line 137) (column 44) (len 24)) (segments (segment 0 (token "CoordinateTransformation") (name "CoordinateTransformation") (separator none) (span (offset 6815) (line 137) (column 44) (len 24)))))
    (reference r39 (scope relative) (span (offset 7466) (line 149) (column 22) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 7466) (line 149) (column 22) (len 19)))))
    (reference r40 (scope relative) (span (offset 7520) (line 150) (column 31) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 7520) (line 150) (column 31) (len 19)))))
    (reference r41 (scope relative) (span (offset 8127) (line 165) (column 31) (len 21)) (segments (segment 0 (token "TranslationOrRotation") (name "TranslationOrRotation") (separator none) (span (offset 8127) (line 165) (column 31) (len 21)))))
    (reference r42 (scope relative) (span (offset 8381) (line 173) (column 33) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 8381) (line 173) (column 33) (len 19)))))
    (reference r43 (scope relative) (span (offset 8436) (line 176) (column 28) (len 21)) (segments (segment 0 (token "TranslationOrRotation") (name "TranslationOrRotation") (separator none) (span (offset 8436) (line 176) (column 28) (len 21)))))
    (reference r44 (scope relative) (span (offset 9022) (line 189) (column 29) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 9022) (line 189) (column 29) (len 19)))))
    (reference r45 (scope relative) (span (offset 9068) (line 190) (column 23) (len 14)) (segments (segment 0 (token "angularMeasure") (name "angularMeasure") (separator none) (span (offset 9068) (line 190) (column 23) (len 14)))))
    (reference r46 (scope relative) (span (offset 9110) (line 191) (column 27) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 9110) (line 191) (column 27) (len 7)))))
    (reference r47 (scope relative) (span (offset 9185) (line 194) (column 47) (len 24)) (segments (segment 0 (token "CoordinateTransformation") (name "CoordinateTransformation") (separator none) (span (offset 9185) (line 194) (column 47) (len 24)))))
    (reference r48 (scope relative) (span (offset 9211) (line 194) (column 73) (len 4)) (segments (segment 0 (token "List") (name "List") (separator none) (span (offset 9211) (line 194) (column 73) (len 4)))))
    (reference r49 (scope relative) (span (offset 9876) (line 206) (column 28) (len 21)) (segments (segment 0 (token "TranslationOrRotation") (name "TranslationOrRotation") (separator none) (span (offset 9876) (line 206) (column 28) (len 21)))))
    (reference r50 (scope relative) (span (offset 9865) (line 206) (column 17) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 9865) (line 206) (column 17) (len 8)))))
    (reference r51 (scope relative) (span (offset 9974) (line 209) (column 48) (len 24)) (segments (segment 0 (token "CoordinateTransformation") (name "CoordinateTransformation") (separator none) (span (offset 9974) (line 209) (column 48) (len 24)))))
    (reference r52 (scope relative) (span (offset 10000) (line 209) (column 74) (len 5)) (segments (segment 0 (token "Array") (name "Array") (separator none) (span (offset 10000) (line 209) (column 74) (len 5)))))
    (reference r53 (scope relative) (span (offset 10807) (line 229) (column 34) (len 5)) (segments (segment 0 (token "Array") (name "Array") (separator none) (span (offset 10807) (line 229) (column 34) (len 5)))))
    (reference r54 (scope relative) (span (offset 10844) (line 230) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10844) (line 230) (column 30) (len 4)))))
    (reference r55 (scope relative) (span (offset 10833) (line 230) (column 19) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 10833) (line 230) (column 19) (len 8)))))
    (reference r56 (scope relative) (span (offset 10889) (line 231) (column 19) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 10889) (line 231) (column 19) (len 10)))))
    (reference r57 (scope relative) (span (offset 10951) (line 233) (column 34) (len 16)) (segments (segment 0 (token "ThreeVectorValue") (name "ThreeVectorValue") (separator none) (span (offset 10951) (line 233) (column 34) (len 16)))))
    (reference r58 (scope relative) (span (offset 10988) (line 233) (column 71) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10988) (line 233) (column 71) (len 4)))))
    (reference r59 (scope relative) (span (offset 10977) (line 233) (column 60) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 10977) (line 233) (column 60) (len 8)))))
    (reference r60 (scope relative) (span (offset 11016) (line 234) (column 18) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 11016) (line 234) (column 18) (len 10)))))
    (reference r61 (scope relative) (span (offset 11065) (line 235) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 11065) (line 235) (column 29) (len 4)))))
    (reference r62 (scope relative) (span (offset 11054) (line 235) (column 18) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11054) (line 235) (column 18) (len 8)))))
    (reference r63 (scope relative) (span (offset 11100) (line 236) (column 5) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11100) (line 236) (column 5) (len 14)))))
    (reference r64 (scope relative) (span (offset 11115) (line 236) (column 20) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11115) (line 236) (column 20) (len 8)))))
    (reference r65 (scope relative) (span (offset 11129) (line 236) (column 34) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11129) (line 236) (column 34) (len 14)))))
    (reference r66 (scope relative) (span (offset 11144) (line 236) (column 49) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11144) (line 236) (column 49) (len 8)))))
    (reference r67 (scope relative) (span (offset 11158) (line 236) (column 63) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11158) (line 236) (column 63) (len 14)))))
    (reference r68 (scope relative) (span (offset 11173) (line 236) (column 78) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11173) (line 236) (column 78) (len 8)))))
    (reference r69 (scope relative) (span (offset 11187) (line 236) (column 92) (len 17)) (segments (segment 0 (token "translationVector") (name "translationVector") (separator none) (span (offset 11187) (line 236) (column 92) (len 17)))))
    (reference r70 (scope relative) (span (offset 11214) (line 237) (column 5) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11214) (line 237) (column 5) (len 14)))))
    (reference r71 (scope relative) (span (offset 11229) (line 237) (column 20) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11229) (line 237) (column 20) (len 8)))))
    (reference r72 (scope relative) (span (offset 11243) (line 237) (column 34) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11243) (line 237) (column 34) (len 14)))))
    (reference r73 (scope relative) (span (offset 11258) (line 237) (column 49) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11258) (line 237) (column 49) (len 8)))))
    (reference r74 (scope relative) (span (offset 11272) (line 237) (column 63) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11272) (line 237) (column 63) (len 14)))))
    (reference r75 (scope relative) (span (offset 11287) (line 237) (column 78) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11287) (line 237) (column 78) (len 8)))))
    (reference r76 (scope relative) (span (offset 11301) (line 237) (column 92) (len 17)) (segments (segment 0 (token "translationVector") (name "translationVector") (separator none) (span (offset 11301) (line 237) (column 92) (len 17)))))
    (reference r77 (scope relative) (span (offset 11328) (line 238) (column 5) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11328) (line 238) (column 5) (len 14)))))
    (reference r78 (scope relative) (span (offset 11343) (line 238) (column 20) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11343) (line 238) (column 20) (len 8)))))
    (reference r79 (scope relative) (span (offset 11357) (line 238) (column 34) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11357) (line 238) (column 34) (len 14)))))
    (reference r80 (scope relative) (span (offset 11372) (line 238) (column 49) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11372) (line 238) (column 49) (len 8)))))
    (reference r81 (scope relative) (span (offset 11386) (line 238) (column 63) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11386) (line 238) (column 63) (len 14)))))
    (reference r82 (scope relative) (span (offset 11401) (line 238) (column 78) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11401) (line 238) (column 78) (len 8)))))
    (reference r83 (scope relative) (span (offset 11415) (line 238) (column 92) (len 17)) (segments (segment 0 (token "translationVector") (name "translationVector") (separator none) (span (offset 11415) (line 238) (column 92) (len 17)))))
    (reference r84 (scope relative) (span (offset 11566) (line 243) (column 38) (len 28)) (segments (segment 0 (token "AffineTransformationMatrix3d") (name "AffineTransformationMatrix3d") (separator none) (span (offset 11566) (line 243) (column 38) (len 28)))))
    (reference r85 (scope relative) (span (offset 11815) (line 249) (column 18) (len 14)) (segments (segment 0 (token "rotationMatrix") (name "rotationMatrix") (separator none) (span (offset 11815) (line 249) (column 18) (len 14)))))
    (reference r86 (scope relative) (span (offset 11853) (line 250) (column 22) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11853) (line 250) (column 22) (len 8)))))
    (reference r87 (scope relative) (span (offset 11915) (line 252) (column 18) (len 17)) (segments (segment 0 (token "translationVector") (name "translationVector") (separator none) (span (offset 11915) (line 252) (column 18) (len 17)))))
    (reference r88 (scope relative) (span (offset 11956) (line 253) (column 22) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 11956) (line 253) (column 22) (len 8)))))
    (reference r89 (scope relative) (span (offset 12088) (line 259) (column 44) (len 26)) (segments (segment 0 (token "ScalarMeasurementReference") (name "ScalarMeasurementReference") (separator none) (span (offset 12088) (line 259) (column 44) (len 26)))))
    (reference r90 (scope relative) (span (offset 12662) (line 273) (column 17) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 12662) (line 273) (column 17) (len 7)))))
    (reference r91 (scope relative) (span (offset 12709) (line 274) (column 31) (len 15)) (segments (segment 0 (token "UnitPowerFactor") (name "UnitPowerFactor") (separator none) (span (offset 12709) (line 274) (column 31) (len 15)))))
    (reference r92 (scope relative) (span (offset 12768) (line 275) (column 29) (len 14)) (segments (segment 0 (token "UnitConversion") (name "UnitConversion") (separator none) (span (offset 12768) (line 275) (column 29) (len 14)))))
    (reference r93 (scope relative) (span (offset 13049) (line 283) (column 39) (len 15)) (segments (segment 0 (token "MeasurementUnit") (name "MeasurementUnit") (separator none) (span (offset 13049) (line 283) (column 39) (len 15)))))
    (reference r94 (scope relative) (span (offset 13216) (line 289) (column 37) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 13216) (line 289) (column 37) (len 10)))))
    (reference r95 (scope relative) (span (offset 13229) (line 289) (column 50) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 13229) (line 289) (column 50) (len 4)))))
    (reference r96 (scope relative) (span (offset 13272) (line 290) (column 38) (len 15)) (segments (segment 0 (token "UnitPowerFactor") (name "UnitPowerFactor") (separator none) (span (offset 13272) (line 290) (column 38) (len 15)))))
    (reference r97 (scope relative) (span (offset 13254) (line 290) (column 20) (len 16)) (segments (segment 0 (token "unitPowerFactors") (name "unitPowerFactors") (separator none) (span (offset 13254) (line 290) (column 20) (len 16)))))
    (reference r98 (scope relative) (span (offset 13315) (line 291) (column 23) (len 21)) (segments (segment 0 (token "UnitPowerFactor") (name "UnitPowerFactor") (separator none) (span (offset 13315) (line 291) (column 23) (len 15))) (segment 1 (token "unit") (name "unit") (separator colon-colon) (span (offset 13332) (line 291) (column 40) (len 4)))))
    (reference r99 (scope relative) (span (offset 13339) (line 291) (column 47) (len 14)) (segments (segment 0 (token "simpleUnitSelf") (name "simpleUnitSelf") (separator none) (span (offset 13339) (line 291) (column 47) (len 14)))))
    (reference r100 (scope relative) (span (offset 13381) (line 292) (column 27) (len 25)) (segments (segment 0 (token "UnitPowerFactor") (name "UnitPowerFactor") (separator none) (span (offset 13381) (line 292) (column 27) (len 15))) (segment 1 (token "exponent") (name "exponent") (separator colon-colon) (span (offset 13398) (line 292) (column 44) (len 8)))))
    (reference r101 (scope relative) (span (offset 13460) (line 297) (column 40) (len 15)) (segments (segment 0 (token "MeasurementUnit") (name "MeasurementUnit") (separator none) (span (offset 13460) (line 297) (column 40) (len 15)))))
    (reference r102 (scope relative) (span (offset 13939) (line 314) (column 19) (len 15)) (segments (segment 0 (token "MeasurementUnit") (name "MeasurementUnit") (separator none) (span (offset 13939) (line 314) (column 19) (len 15)))))
    (reference r103 (scope relative) (span (offset 13978) (line 315) (column 23) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 13978) (line 315) (column 23) (len 4)))))
    (reference r104 (scope relative) (span (offset 14327) (line 326) (column 28) (len 15)) (segments (segment 0 (token "MeasurementUnit") (name "MeasurementUnit") (separator none) (span (offset 14327) (line 326) (column 28) (len 15)))))
    (reference r105 (scope relative) (span (offset 14374) (line 327) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 14374) (line 327) (column 31) (len 4)))))
    (reference r106 (scope relative) (span (offset 14401) (line 328) (column 22) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 14401) (line 328) (column 22) (len 7)))))
    (reference r107 (scope relative) (span (offset 14468) (line 331) (column 42) (len 14)) (segments (segment 0 (token "UnitConversion") (name "UnitConversion") (separator none) (span (offset 14468) (line 331) (column 42) (len 14)))))
    (reference r108 (scope relative) (span (offset 14629) (line 338) (column 38) (len 14)) (segments (segment 0 (token "UnitConversion") (name "UnitConversion") (separator none) (span (offset 14629) (line 338) (column 38) (len 14)))))
    (reference r109 (scope relative) (span (offset 15108) (line 349) (column 21) (len 10)) (segments (segment 0 (token "UnitPrefix") (name "UnitPrefix") (separator none) (span (offset 15108) (line 349) (column 21) (len 10)))))
    (reference r110 (scope relative) (span (offset 15162) (line 350) (column 40) (len 32)) (segments (segment 0 (token "UnitConversion") (name "UnitConversion") (separator none) (span (offset 15162) (line 350) (column 40) (len 14))) (segment 1 (token "conversionFactor") (name "conversionFactor") (separator colon-colon) (span (offset 15178) (line 350) (column 56) (len 16)))))
    (reference r111 (scope relative) (span (offset 15197) (line 350) (column 75) (len 6)) (segments (segment 0 (token "prefix") (name "prefix") (separator none) (span (offset 15197) (line 350) (column 75) (len 6)))))
    (reference r112 (scope relative) (span (offset 15204) (line 350) (column 82) (len 16)) (segments (segment 0 (token "conversionFactor") (name "conversionFactor") (separator none) (span (offset 15204) (line 350) (column 82) (len 16)))))
    (reference r113 (scope relative) (span (offset 15400) (line 359) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 15400) (line 359) (column 23) (len 6)))))
    (reference r114 (scope relative) (span (offset 15428) (line 360) (column 21) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 15428) (line 360) (column 21) (len 6)))))
    (reference r115 (scope relative) (span (offset 15466) (line 361) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 15466) (line 361) (column 31) (len 4)))))
    (reference r116 (scope relative) (span (offset 15521) (line 365) (column 45) (len 26)) (segments (segment 0 (token "ScalarMeasurementReference") (name "ScalarMeasurementReference") (separator none) (span (offset 15521) (line 365) (column 45) (len 26)))))
    (reference r117 (scope relative) (span (offset 15824) (line 373) (column 19) (len 15)) (segments (segment 0 (token "MeasurementUnit") (name "MeasurementUnit") (separator none) (span (offset 15824) (line 373) (column 19) (len 15)))))
    (reference r118 (scope relative) (span (offset 15875) (line 374) (column 35) (len 20)) (segments (segment 0 (token "QuantityValueMapping") (name "QuantityValueMapping") (separator none) (span (offset 15875) (line 374) (column 35) (len 20)))))
    (reference r119 (scope relative) (span (offset 15938) (line 377) (column 32) (len 16)) (segments (segment 0 (token "MeasurementScale") (name "MeasurementScale") (separator none) (span (offset 15938) (line 377) (column 32) (len 16)))))
    (reference r120 (scope relative) (span (offset 16063) (line 384) (column 33) (len 16)) (segments (segment 0 (token "MeasurementScale") (name "MeasurementScale") (separator none) (span (offset 16063) (line 384) (column 33) (len 16)))))
    (reference r121 (scope relative) (span (offset 16081) (line 384) (column 51) (len 15)) (segments (segment 0 (token "CoordinateFrame") (name "CoordinateFrame") (separator none) (span (offset 16081) (line 384) (column 51) (len 15)))))
    (reference r122 (scope relative) (span (offset 16399) (line 393) (column 17) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 16399) (line 393) (column 17) (len 7)))))
    (reference r123 (scope relative) (span (offset 16454) (line 396) (column 36) (len 16)) (segments (segment 0 (token "MeasurementScale") (name "MeasurementScale") (separator none) (span (offset 16454) (line 396) (column 36) (len 16)))))
    (reference r124 (scope relative) (span (offset 16886) (line 406) (column 22) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 16886) (line 406) (column 22) (len 6)))))
    (reference r125 (scope relative) (span (offset 16933) (line 409) (column 36) (len 16)) (segments (segment 0 (token "MeasurementScale") (name "MeasurementScale") (separator none) (span (offset 16933) (line 409) (column 36) (len 16)))))
    (reference r126 (scope relative) (span (offset 17500) (line 425) (column 28) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 17500) (line 425) (column 28) (len 6)))))
    (reference r127 (scope relative) (span (offset 17528) (line 426) (column 21) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 17528) (line 426) (column 21) (len 6)))))
    (reference r128 (scope relative) (span (offset 17558) (line 427) (column 23) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 17558) (line 427) (column 23) (len 6)))))
    (reference r129 (scope relative) (span (offset 17597) (line 428) (column 32) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 17597) (line 428) (column 32) (len 19)))))
    (reference r130 (scope relative) (span (offset 18648) (line 448) (column 34) (len 25)) (segments (segment 0 (token "DefinitionalQuantityValue") (name "DefinitionalQuantityValue") (separator none) (span (offset 18648) (line 448) (column 34) (len 25)))))
    (reference r131 (scope relative) (span (offset 18711) (line 449) (column 37) (len 25)) (segments (segment 0 (token "DefinitionalQuantityValue") (name "DefinitionalQuantityValue") (separator none) (span (offset 18711) (line 449) (column 37) (len 25)))))
    (reference r132 (scope relative) (span (offset 19496) (line 468) (column 18) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 19496) (line 468) (column 18) (len 6)))))
    (reference r133 (scope relative) (span (offset 19534) (line 469) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 19534) (line 469) (column 25) (len 6)))))
    (reference r134 (scope relative) (span (offset 19581) (line 472) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19581) (line 472) (column 36) (len 11)))))
    (reference r135 (scope relative) (span (offset 19719) (line 478) (column 17) (len 16)) (segments (segment 0 (token "unitPowerFactors") (name "unitPowerFactors") (separator none) (span (offset 19719) (line 478) (column 17) (len 16)))))
    (reference r136 (scope relative) (span (offset 19781) (line 480) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 19781) (line 480) (column 37) (len 19)))))
    (reference r137 (scope relative) (span (offset 19893) (line 485) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 19893) (line 485) (column 22) (len 4)))))
    (reference r138 (scope relative) (span (offset 19888) (line 485) (column 17) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19888) (line 485) (column 17) (len 3)))))
    (reference r139 (scope relative) (span (offset 19921) (line 486) (column 23) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 19921) (line 486) (column 23) (len 16)))))
    (reference r140 (scope relative) (span (offset 19915) (line 486) (column 17) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19915) (line 486) (column 17) (len 4)))))
    (reference r141 (scope relative) (span (offset 20124) (line 492) (column 30) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 20124) (line 492) (column 30) (len 17)))))
    (reference r142 (scope relative) (span (offset 20995) (line 512) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 20995) (line 512) (column 23) (len 6)))))
    (reference r143 (scope relative) (span (offset 21039) (line 513) (column 34) (len 18)) (segments (segment 0 (token "SystemOfQuantities") (name "SystemOfQuantities") (separator none) (span (offset 21039) (line 513) (column 34) (len 18)))))
    (reference r144 (scope relative) (span (offset 21085) (line 514) (column 24) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 21085) (line 514) (column 24) (len 10)))))
  )
  (root (library-package (name "MeasurementReferences") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 57) (line 3) (column 4) (len 76)) (normalized "This package defines the representations for measurement references.\n"))) (import (target (span (span (offset 153) (line 7) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 189) (line 8) (column 17) (len 17))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 224) (line 9) (column 17) (len 15))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 236) (line 9) (column 29) (len 3))) (separator (span (offset 236) (line 9) (column 29) (len 2))) (marker (span (offset 238) (line 9) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 257) (line 10) (column 17) (len 30))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 306) (line 12) (column 17) (len 23))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 347) (line 13) (column 17) (len 25))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 390) (line 14) (column 17) (len 24))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 432) (line 15) (column 17) (len 29))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 479) (line 16) (column 17) (len 31))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 528) (line 17) (column 17) (len 28))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 574) (line 18) (column 17) (len 31))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 623) (line 19) (column 17) (len 30))) (all none) (ref r11) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 671) (line 20) (column 17) (len 28))) (all none) (ref r12) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "TensorMeasurementReference") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 765) (line 24) (column 5) (len 2296)) (normalized "TensorMeasurementReference is the most general AttributeDefinition to represent measurement references.\n\nThe concept \"measurement reference\" is defined in [VIM] \"quantity\" NOTE 2 as \"A reference can be a measurement unit,\na measurement procedure, a reference material, or a combination of such.\", see https://jcgm.bipm.org/vim/en/1.1.html .\nIn addition [VIM] \"quantity\" NOTE 5 states that \"A quantity as defined here is a scalar. However, a vector or a tensor, \nthe components of which are quantities, is also considered to be a quantity\". However, the rest of [VIM] does not explicitly \ndefine how tensor and vector quantities can be or should be supported.\n\nIn this package, in line with TensorQuantityValue in package Quantities, the most general kind of measurement reference\nis TensorMeasurementReference that represents a measurement reference for any order of tensor quantity. Since the order can \nalso be one or zero, this includes vector and scalar quantities. The specializations VectorMeasurementReference and \nScalarMeasurementReference are defined to specifically represent measurement references for vector and scalar quantities.\n\nTensorMeasurementReference specializes Array, which provides its multi-dimensional structure. The order of a tensor is equivalent\nto the rank of an Array.\n\nAttribute isBound specifies whether the vector space product is bound (isBound is true) or free (isBound is false).\n\nAttribute mRefs specifies the scalar measurement references for all dimensions of a tensor quantity.\n\nThe short name of a TensorMeasurementReference is the unique symbol by which the measurement reference is known.\nThe name of a TensorMeasurementReference is spelled-out human readable name of the measurement reference.\n\nFor example, typical measurement references for (scalar) quantity speed are declared with the following humanId and name:\n<'m/s'> and 'metre per second',\n<'km/h'> and 'kilometre per hour',\n<'mi/h'> and 'mile per hour'.\n\nA measurement reference can have zero or more definitionalQuantityValues that allow to specify\nquantity values that carry a particular meaning or relevance for the measurement reference.\n"))) (attribute-usage (declaration-name "isBound") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 3106) (line 57) (column 41) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name "order") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "mRefs") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "definitionalQuantityValues") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "VectorMeasurementReference") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3378) (line 65) (column 5) (len 861)) (normalized "A VectorMeasurementReference is a specialization of TensorMeasurementReference for vector quantities that are\ntyped by a VectorQuantityValue. Its order is one. Implicitly, it defines a vector space of dimension `n` = dimensions[1].\nThe magnitudes of the `n` basis unit vectors that span the vector space are defined by the mRefs which each are\na ScalarMeasurementReference, typically a MeasurementUnit or an IntervalScale.\n\nAttribute isOrthogonal declares whether the basis vectors of the vector space are orthogonal, i.e., whether all\ninner products of any pair of basis vectors are equal to zero.\n\nA pair of a specialization of VectorQuantityValue and a specialization of VectorMeasurementReference can also be used to\ndefine a vector space for state vectors as used in state-space representation models.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "isOrthogonal") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 4333) (line 79) (column 46) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "ScalarMeasurementReference") (short-name none) (modifiers (abstract (span (offset 4344) (line 82) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4436) (line 84) (column 5) (len 725)) (normalized "A ScalarMeasurementReference is a specialization of VectorMeasurementReference for scalar quantities\nthat are typed by a ScalarQuantityValue and for components of tensor or vector quantities.\nIts order is zero. A ScalarMeasurementReference is also a generalization of MeasurementUnit and MeasurementScale.\nIt establishes how to interpret the numerical value (num) of a ScalarQuantityValue or a component of\na tensor or vector quantity value, and establishes its actual quantity dimension.\n\nAttribute mRefs is bound to self for a ScalarMeasurementReference, for consistency with tensor and vector measurement references,\nas the dimension or component of a scalar quantity is itself.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5195) (line 95) (column 30) (len 2)) (null))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5230) (line 96) (column 32) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5260) (line 97) (column 25) (len 4)) (ref r27))))) (body semicolon)) (attribute-usage (declaration-name "quantityDimension") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "CoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5397) (line 103) (column 5) (len 541)) (normalized "CoordinateFrame is a VectorMeasurementReference with the specific purpose to quantify (i.e., coordinatize) a vector space, \nand locate and orient it with respect to another CoordinateFrame.\n\nOptional attribute transformation enables specification of the location and orientation of this CoordinateFrame as dependent\nand nested with respect to another (reference) coordinate frame. Typically the other CoordinateFrame is the frame of \nthe next higher element (Object, Item or Part) in a composite structure.\n"))) (attribute-usage (declaration-name "transformation") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r31)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6030) (line 113) (column 27) (len 4)) (ref r32))))) (body semicolon)))))) (attribute-def (declaration-name "3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6122) (line 119) (column 8) (len 65)) (normalized "Most general 3-dimensional coordinate frame\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6225) (line 122) (column 36) (len 1)) (integer 3))))) (body semicolon)))) (alias (name "ThreeDCoordinateFrame") (target (ref r35)) (body semicolon)) (attribute-def (declaration-name "CoordinateTransformation") (short-name none) (modifiers (abstract (span (offset 6296) (line 126) (column 5) (len 8)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6368) (line 128) (column 11) (len 198)) (normalized "CoordinateTransformation is the most general representation of the transformation of a target VectorMeasurementReference \nwith respect to a source VectorMeasurementReference.\n"))) (attribute-usage (declaration-name "source") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "target") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint))) (attribute-def (declaration-name "CoordinateFramePlacement") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6858) (line 139) (column 8) (len 583)) (normalized "CoordinateFramePlacement is a CoordinateTransformation by placement of the target frame in the source frame.\n \nAttribute origin specifies the location of the origin of the target frame as a vector in the source frame.\n\nAttribute basisDirections specifies the orientation of the target frame by specifying the directions of \nthe respective basis vectors of the target frame via direction vectors in the source frame. An empty sequence of\nbasisDirections signifies no change of orientation of the target coordinate frame.\n"))) (attribute-usage (declaration-name "origin") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "basisDirections") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r40)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint) (assert-constraint) (assert-constraint))) (attribute-def (declaration-name "TranslationOrRotation") (short-name none) (modifiers (abstract (span (offset 7953) (line 158) (column 2) (len 8)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8010) (line 160) (column 5) (len 80)) (normalized "TranslationOrRotation is an abstract union of Translation and Rotation\n"))))) (attribute-def (declaration-name "Translation") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r41)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8161) (line 167) (column 5) (len 183)) (normalized "Representation of a translation with respect to a coordinate frame\n\nAttribute translationVector specifies the displacement vector that constitutes the translation.\n"))) (attribute-usage (declaration-name "translationVector") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Rotation") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r43)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8470) (line 178) (column 5) (len 519)) (normalized "Representation of a rotation about an axis over an angle\n\nAttribute axisDirection specifies the direction of the rotation axis.\nAttribute angle specifies the angle of rotation, where a positive value implies right-handed rotation.\nAttribute isIntrinsic asserts whether the intermediate coordinate frame moves with the rotation or not, \ni.e. whether an instrinsic or extrinsic rotation is specified.\n\nSee https://en.wikipedia.org/wiki/Davenport_chained_rotations for details.\n"))) (attribute-usage (declaration-name "axisDirection") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r44)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "angle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "isIntrinsic") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 9129) (line 191) (column 46) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "TranslationRotationSequence") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r47) (ref r48)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9226) (line 196) (column 4) (len 618)) (normalized "Coordinate frame transformation specified by a sequence of translations and/or rotations\n\nNote: This is a coordinate transformation that is convenient for interpretation by humans.\nIn particular a sequence of rotations about the principal axes of a coordinate frame is much more easy understandable \nthan a rotation about an arbitrary axis.\nAny sequence can be reduced to a single combination of a translation and a rotation about a particular axis, but in general \nthe original sequence cannot be retrieved as there are infinitely many sequences representing the reduced transformation.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r49)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "AffineTransformationMatrix3d") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r51) (ref r52)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 10018) (line 211) (column 5) (len 751)) (normalized "AffineTransformationMatrix3d is a three dimensional CoordinateTransformation specified via an affine transformation matrix\n\nThe interpretation of the matrix is as follows:\n- the upper left 3x3 matrix represents the rotation matrix\n- the uper right 3x1 column vector represents the translation vector\n- the bottom row must be the row vector (0, 0, 0, 1).\n\nI.e. the matrix has the following form:\n( R, R, R, T,\n  R, R, R, T,\n  R, R, R, T,\n  0, 0, 0, 1 )\nwhere the cells marked R form the rotation matrix and the cells marked T form the translation vector.\n\nNote: See https://en.wikipedia.org/wiki/Transformation_matrix, under affine transformations for a general explanation.\n"))) (attribute-usage (declaration-name "rotationMatrix") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r54)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r55)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10902) (line 231) (column 32) (len 6)) (sequence (sequence-list (element first (expression (span (offset 10903) (line 231) (column 33) (len 1)) (integer 3))) (element comma (expression (span (offset 10906) (line 231) (column 36) (len 1)) (integer 3))))))))) (body semicolon)))) (attribute-usage (declaration-name "translationVector") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r58)))) (multiplicity (lower (expression (span (offset 10993) (line 233) (column 76) (len 1)) (integer 3))) (upper (expression (span (offset 10993) (line 233) (column 76) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11029) (line 234) (column 31) (len 6)) (sequence (sequence-list (element first (expression (span (offset 11030) (line 234) (column 32) (len 1)) (integer 4))) (element comma (expression (span (offset 11033) (line 234) (column 35) (len 1)) (integer 4))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r62)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11094) (line 235) (column 58) (len 359)) (sequence (sequence-list (element first (expression (span (offset 11100) (line 236) (column 5) (len 27)) (index (base (expression (span (offset 11100) (line 236) (column 5) (len 23)) (member-access (base (expression (span (offset 11100) (line 236) (column 5) (len 14)) (ref r63))) (separator dot) (member (ref r64))))) (operands (sequence-list (element first (expression (span (offset 11125) (line 236) (column 30) (len 1)) (integer 1)))))))) (element comma (expression (span (offset 11129) (line 236) (column 34) (len 27)) (index (base (expression (span (offset 11129) (line 236) (column 34) (len 23)) (member-access (base (expression (span (offset 11129) (line 236) (column 34) (len 14)) (ref r65))) (separator dot) (member (ref r66))))) (operands (sequence-list (element first (expression (span (offset 11154) (line 236) (column 59) (len 1)) (integer 2)))))))) (element comma (expression (span (offset 11158) (line 236) (column 63) (len 27)) (index (base (expression (span (offset 11158) (line 236) (column 63) (len 23)) (member-access (base (expression (span (offset 11158) (line 236) (column 63) (len 14)) (ref r67))) (separator dot) (member (ref r68))))) (operands (sequence-list (element first (expression (span (offset 11183) (line 236) (column 88) (len 1)) (integer 3)))))))) (element comma (expression (span (offset 11187) (line 236) (column 92) (len 21)) (index (base (expression (span (offset 11187) (line 236) (column 92) (len 17)) (ref r69))) (operands (sequence-list (element first (expression (span (offset 11206) (line 236) (column 111) (len 1)) (integer 1)))))))) (element comma (expression (span (offset 11214) (line 237) (column 5) (len 27)) (index (base (expression (span (offset 11214) (line 237) (column 5) (len 23)) (member-access (base (expression (span (offset 11214) (line 237) (column 5) (len 14)) (ref r70))) (separator dot) (member (ref r71))))) (operands (sequence-list (element first (expression (span (offset 11239) (line 237) (column 30) (len 1)) (integer 4)))))))) (element comma (expression (span (offset 11243) (line 237) (column 34) (len 27)) (index (base (expression (span (offset 11243) (line 237) (column 34) (len 23)) (member-access (base (expression (span (offset 11243) (line 237) (column 34) (len 14)) (ref r72))) (separator dot) (member (ref r73))))) (operands (sequence-list (element first (expression (span (offset 11268) (line 237) (column 59) (len 1)) (integer 5)))))))) (element comma (expression (span (offset 11272) (line 237) (column 63) (len 27)) (index (base (expression (span (offset 11272) (line 237) (column 63) (len 23)) (member-access (base (expression (span (offset 11272) (line 237) (column 63) (len 14)) (ref r74))) (separator dot) (member (ref r75))))) (operands (sequence-list (element first (expression (span (offset 11297) (line 237) (column 88) (len 1)) (integer 6)))))))) (element comma (expression (span (offset 11301) (line 237) (column 92) (len 21)) (index (base (expression (span (offset 11301) (line 237) (column 92) (len 17)) (ref r76))) (operands (sequence-list (element first (expression (span (offset 11320) (line 237) (column 111) (len 1)) (integer 2)))))))) (element comma (expression (span (offset 11328) (line 238) (column 5) (len 27)) (index (base (expression (span (offset 11328) (line 238) (column 5) (len 23)) (member-access (base (expression (span (offset 11328) (line 238) (column 5) (len 14)) (ref r77))) (separator dot) (member (ref r78))))) (operands (sequence-list (element first (expression (span (offset 11353) (line 238) (column 30) (len 1)) (integer 7)))))))) (element comma (expression (span (offset 11357) (line 238) (column 34) (len 27)) (index (base (expression (span (offset 11357) (line 238) (column 34) (len 23)) (member-access (base (expression (span (offset 11357) (line 238) (column 34) (len 14)) (ref r79))) (separator dot) (member (ref r80))))) (operands (sequence-list (element first (expression (span (offset 11382) (line 238) (column 59) (len 1)) (integer 8)))))))) (element comma (expression (span (offset 11386) (line 238) (column 63) (len 27)) (index (base (expression (span (offset 11386) (line 238) (column 63) (len 23)) (member-access (base (expression (span (offset 11386) (line 238) (column 63) (len 14)) (ref r81))) (separator dot) (member (ref r82))))) (operands (sequence-list (element first (expression (span (offset 11411) (line 238) (column 88) (len 1)) (integer 9)))))))) (element comma (expression (span (offset 11415) (line 238) (column 92) (len 21)) (index (base (expression (span (offset 11415) (line 238) (column 92) (len 17)) (ref r83))) (operands (sequence-list (element first (expression (span (offset 11434) (line 238) (column 111) (len 1)) (integer 3)))))))) (element comma (expression (span (offset 11442) (line 239) (column 5) (len 1)) (integer 0))) (element comma (expression (span (offset 11445) (line 239) (column 8) (len 1)) (integer 0))) (element comma (expression (span (offset 11448) (line 239) (column 11) (len 1)) (integer 0))) (element comma (expression (span (offset 11451) (line 239) (column 14) (len 1)) (integer 1))))))))) (body semicolon)) (assert-constraint))) (attribute-def (declaration-name "NullTransformation") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r84)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 11607) (line 245) (column 5) (len 188)) (normalized "NullTransformation is a three dimensional CoordinateTransformation that places the target CoordinateFrame at the\nsame position and orientation as the source CoordinateFrame.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r86)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11864) (line 250) (column 33) (len 27)) (sequence (sequence-list (element first (expression (span (offset 11865) (line 250) (column 34) (len 1)) (integer 1))) (element comma (expression (span (offset 11868) (line 250) (column 37) (len 1)) (integer 0))) (element comma (expression (span (offset 11871) (line 250) (column 40) (len 1)) (integer 0))) (element comma (expression (span (offset 11874) (line 250) (column 43) (len 1)) (integer 0))) (element comma (expression (span (offset 11877) (line 250) (column 46) (len 1)) (integer 1))) (element comma (expression (span (offset 11880) (line 250) (column 49) (len 1)) (integer 0))) (element comma (expression (span (offset 11883) (line 250) (column 52) (len 1)) (integer 0))) (element comma (expression (span (offset 11886) (line 250) (column 55) (len 1)) (integer 0))) (element comma (expression (span (offset 11889) (line 250) (column 58) (len 1)) (integer 1))))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r87)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r88)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11967) (line 253) (column 33) (len 9)) (sequence (sequence-list (element first (expression (span (offset 11968) (line 253) (column 34) (len 1)) (integer 0))) (element comma (expression (span (offset 11971) (line 253) (column 37) (len 1)) (integer 0))) (element comma (expression (span (offset 11974) (line 253) (column 40) (len 1)) (integer 0))))))))) (body semicolon)))))) (attribute-usage) (attribute-def (declaration-name "MeasurementUnit") (short-name none) (modifiers (abstract (span (offset 12046) (line 259) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r89)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12127) (line 261) (column 5) (len 514)) (normalized "Representation of a measurement unit.\n\nNote: MeasurementUnit directly specializes ScalarMeasurementReference in order to allow for efficient and intuitive definition of a ratio scale.\n\nA MeasurementUnit can be used in two ways:\n1. Directly as the mRef in a ScalarQuantityValue, which implies that the effective measurement reference is a ratio scale defined by the unit.\n2. As the unit of a MeasurementScale.\n\nA MeasurementUnit specifies one or more UnitPowerFactors.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r90)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12672) (line 273) (column 27) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name "unitPowerFactors") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r91)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "unitConversion") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r92)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint))) (attribute-def (declaration-name "SimpleUnit") (short-name none) (modifiers (abstract (span (offset 13012) (line 283) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r93)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 13077) (line 285) (column 5) (len 98)) (normalized "Representation of a measurement unit that does not depend on any other measurement unit.\n"))) (attribute-usage (declaration-name "simpleUnitSelf") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r94)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13229) (line 289) (column 50) (len 4)) (ref r95))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r96)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r97)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "unit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r98)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13339) (line 291) (column 47) (len 14)) (ref r99))))) (body semicolon)) (attribute-usage (declaration-name "exponent") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13409) (line 292) (column 55) (len 1)) (integer 1))))) (body semicolon)))))) (attribute-def (declaration-name "DerivedUnit") (short-name none) (modifiers (abstract (span (offset 13422) (line 297) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r101)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 13488) (line 299) (column 5) (len 245)) (normalized "Representation of a derived measurement unit that depends on one or more powers of other measurement units.\n\nVIM defines \"derived unit\" as \"measurement unit for a derived quantity\", see https://jcgm.bipm.org/vim/en/1.11.html .\n"))))) (attribute-def (declaration-name "UnitPowerFactor") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 13784) (line 309) (column 5) (len 132)) (normalized "Representation of a measurement unit power factor, which is a tuple\nof a referenced measurement unit and an exponent.\n"))) (attribute-usage (declaration-name "unit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r102)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "exponent") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r103)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "UnitConversion") (short-name none) (modifiers (abstract (span (offset 13989) (line 318) (column 2) (len 8)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14039) (line 320) (column 5) (len 256)) (normalized "Representation of the linear conversion relationship between one measurement unit and another measurement unit, that acts as a reference.\n\nAttribute isExact asserts whether the conversionFactor is exact or not. By default it is set true.\n"))) (attribute-usage (declaration-name "referenceUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r104)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "conversionFactor") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r105)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "isExact") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r106)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 14417) (line 328) (column 38) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "ConversionByConvention") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r107)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14495) (line 333) (column 5) (len 90)) (normalized "Representation of a UnitConversion that is defined according to some convention.\n"))))) (attribute-def (declaration-name "ConversionByPrefix") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r108)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14656) (line 340) (column 5) (len 427)) (normalized "Representation of a UnitConversion that is defined through reference to a named unit prefix,\nthat in turn represents a decimal or binary multiple or sub-multiple, as defined in ISO/IEC 80000-1.\n\nNote: The actual value of the conversion factor is derived from the definition of the unit prefix.\n\nExamples: kilometre for conversion factor 1000 with reference unit metre, nanofarad for 1E-9 farad.\n"))) (attribute-usage (declaration-name "prefix") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r109)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "conversionFactor") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r110)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15197) (line 350) (column 75) (len 23)) (member-access (base (expression (span (offset 15197) (line 350) (column 75) (len 6)) (ref r111))) (separator dot) (member (ref r112))))))) (body semicolon)))) (attribute-def (declaration-name "UnitPrefix") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15264) (line 355) (column 5) (len 109)) (normalized "Representation of a multiple or sub-multiple measurement unit prefix as defined in ISO/IEC 80000-1.\n"))) (attribute-usage (declaration-name "longName") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r113)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "symbol") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r114)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "conversionFactor") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r115)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "MeasurementScale") (short-name none) (modifiers (abstract (span (offset 15478) (line 365) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r116)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15560) (line 367) (column 5) (len 241)) (normalized "Representation of a non-ratio measurement scale as opposed to a ratio measurement scale defined by a MeasurementUnit.\n\nNote: A ratio scale is implied by direct use of a MeasurementUnit as the mRef in a ScalarQuantityValue.\n"))) (attribute-usage (declaration-name "unit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r117)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "quantityValueMapping") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r118)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "OrdinalScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r119)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15967) (line 379) (column 5) (len 57)) (normalized "Representation of an ordinal measurement scale.\n"))))) (attribute-def (declaration-name "IntervalScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r120) (ref r121)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16109) (line 386) (column 5) (len 269)) (normalized "Representation of an interval measurement scale.\n\nAn IntervalScale is also a CoordinateFrame\nThe offset of one interval measurement scale w.r.t. another interval or ratio scale is defined through a quantityValueMapping, see MeasurementReference.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r122)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16409) (line 393) (column 27) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "CyclicRatioScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r123)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16483) (line 398) (column 5) (len 377)) (normalized "Representation of a ratio measurement scale with a periodic cycle.\n\nNote: The magnitude of the periodic cycle is defined by the modulus of the scale.\nExample: Planar angle with modulus 360 degrees, therefore on such a cyclic ratio scale,\nan angle of 450 degrees is equivalent to an angle of 90 degrees, and -60 degrees is equivalent to 300 degrees.\n"))) (attribute-usage (declaration-name "modulus") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r124)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "LogarithmicScale") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r125)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16962) (line 411) (column 5) (len 506)) (normalized "Representation of a logarithmic measurement scale\n\nThe magnitude v of a ratio quantity value expressed on a logarithmic scale\nfor a magnitude x of a quantity value expressed on a ratio scale is computed as follows:\n  v = f * log_base( (x / x_ref )^a )\nwhere:\n  f is a multiplication factor,\n  log_base is the log function for the given logarithm base,\n  x is the actual quantity,\n  x_ref is a reference quantity,\n  a is an exponent.\n"))) (attribute-usage (declaration-name "logarithmBase") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r126)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "factor") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r127)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "exponent") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r128)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "referenceQuantity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r129)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "QuantityValueMapping") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 17676) (line 433) (column 5) (len 934)) (normalized "Representation of the mapping of equivalent quantity values expressed on two different MeasurementReferences\n\nA QuantityValueMapping specifies a mapping from a given mappedQuantityValue owned by the MeasurementReference\nthat owns the QuantityValueMapping to a referenceQuantityValue owned by another MeasurementReference.\n\nExample: The mapping between the temperature value of 0.01 degree Celsius on the celsius temperature scale\nto the equivalent temperature value of 273.16 K on the kelvin temperature scale,\nwould specify a mappedQuantityValue referencing the\nthe DefinitionalQuantityValue (0.01, \"absolute thermodynamic temperature of the triple point of water\")\nof the celsius interval scale, and a referenceQuantityValue referencing the\nDefinitionalQuantityValue (273.16, \"absolute thermodynamic temperature of the triple point of water\")\nof the kelvin ratio scale.\n"))) (attribute-usage (declaration-name "mappedQuantityValue") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r130)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "referenceQuantityValue") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r131)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "DefinitionalQuantityValue") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 18795) (line 454) (column 5) (len 679)) (normalized "Representation of a particular quantity value that is used in the definition of a TensorMeasurementReference\n\nTypically such a particular value is defined by convention. It can be used to define a selected reference value,\nsuch as the meaning of zero on a measurement scale or the origin of a top-level coordinate frame.\n\nExample: The 'kelvin' MeasurementReference for thermodynamic temperature could have a\nDefinitionalQuantityValue {\n    :>> num = 273.16;\n    :>> definition = \"thermodynamic temperature of the triple point of Vienna Standard Mean Ocean Water in kelvin\";\n}\nthat is value of the definition of the scale.\n"))) (attribute-usage (declaration-name "num") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "definition") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r133)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "DimensionOneUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r134)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19605) (line 474) (column 5) (len 93)) (normalized "Explicit definition of \"unit of dimension one\", also known as \"dimensionless unit\".\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r135)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19738) (line 478) (column 36) (len 2)) (null))))) (body semicolon)))) (attribute-def (declaration-name "DimensionOneValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r136)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19813) (line 482) (column 5) (len 56)) (normalized "A ScalarQuantityValue with a DimensionOneUnit.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r137)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r138)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r139)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r140)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-usage) (attribute-def (declaration-name "CountValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r141)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 20154) (line 494) (column 5) (len 83)) (normalized "Explicit definition of a generic \"count\" quantity as a DimensionOneValue.\n"))))) (attribute-usage) (attribute-def (declaration-name "SystemOfUnits") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 20365) (line 502) (column 5) (len 603)) (normalized "A SystemOfUnits represents the essentials of [VIM] concept \"system of units\" (https://jcgm.bipm.org/vim/en/1.13.html), defined as a\n\"set of base units and derived units, together with their multiples and submultiples, defined in accordance with given rules,\nfor a given system of quantities\".\nThe base units are a particular selection of measurement units for each of the base quantities of a system of quantities,\nthat form the basis on top of which all other (derived) units are defined.\n\nAttribute systemOfQuantities speficies the associated SystemOfQuantities.\n"))) (attribute-usage (declaration-name "longName") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r142)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "systemOfQuantities") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r143)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "baseUnits") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r144)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (constraint-def (name "VerifyUnitPowerFactors") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 21169) (line 519) (column 5) (len 119)) (normalized "Constraint definition to verify that the given unit power factors comply with the required quantity dimension\n"))) (in-out-declaration) (in-out-declaration))))))
)
~~~
