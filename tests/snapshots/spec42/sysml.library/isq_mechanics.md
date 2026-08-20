# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQMechanics"))
~~~
# SOURCE
~~~sysml
standard library package ISQMechanics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-4:2019 "Mechanics"
     * see also https://www.iso.org/standard/64975.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-4 item 4-1 mass */
    /* See package ISQBase for the declarations of MassValue and MassUnit */

    /* ISO-80000-4 item 4-2 mass density, density */
    attribute def MassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-2 mass density, density
         * symbol(s): `ρ`, `ρ_m`
         * application domain: generic
         * name: MassDensity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quantity representing the spatial distribution of mass of a continuous material: `ρ(vec(r)) = (dm)/(dV)` where `m` is mass of the material contained in an infinitesimal domain at point `vec(r)` and `V` is volume of this domain
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassDensityUnit[1];
    }

    attribute massDensity: MassDensityValue[*] nonunique :> scalarQuantities;

    attribute def MassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias DensityUnit for MassDensityUnit;
    alias DensityValue for MassDensityValue;
    alias density for massDensity;

    /* ISO-80000-4 item 4-3 specific volume */
    attribute def SpecificVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-3 specific volume
         * symbol(s): `v`
         * application domain: generic
         * name: SpecificVolume
         * quantity dimension: L^3*M^-1
         * measurement unit(s): kg^-1*m^3
         * tensor order: 0
         * definition: reciprocal of mass density `ρ` (item 4-2): `v = 1/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificVolumeUnit[1];
    }

    attribute specificVolume: SpecificVolumeValue[*] nonunique :> scalarQuantities;

    attribute def SpecificVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-4 item 4-4 relative mass density, relative density */
    attribute def RelativeMassDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-4 relative mass density, relative density
         * symbol(s): `d`
         * application domain: generic
         * name: RelativeMassDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass density of a substance `ρ` and mass density of a reference substance `ρ_0` : `d = ρ/ρ_0`
         * remarks: Conditions and material should be specified for the reference substance.
         */
    }
    attribute relativeMassDensity: RelativeMassDensityValue :> scalarQuantities;

    alias relativeDensity for relativeMassDensity;

    /* ISO-80000-4 item 4-5 surface mass density, surface density */
    attribute def SurfaceMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-5 surface mass density, surface density
         * symbol(s): `ρ_A`
         * application domain: generic
         * name: SurfaceMassDensity
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: quantity representing the areal distribution of mass of a continuous material: `ρ_A(vec(r)) = (dm)/(dA)` where `m` is the mass of the material at position `vec(r)` and `A` is area
         * remarks: The name "grammage" should not be used for this quantity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceMassDensityUnit[1];
    }

    attribute surfaceMassDensity: SurfaceMassDensityValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceMassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias SurfaceDensityUnit for SurfaceMassDensityUnit;
    alias SurfaceDensityValue for SurfaceMassDensityValue;
    alias surfaceDensity for surfaceMassDensity;

    /* ISO-80000-4 item 4-6 linear mass density, linear density */
    attribute def LinearMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-6 linear mass density, linear density
         * symbol(s): `ρ_I`
         * application domain: generic
         * name: LinearMassDensity
         * quantity dimension: L^-1*M^1
         * measurement unit(s): kg*m^-1
         * tensor order: 0
         * definition: quantity representing the linear distribution of mass of a continuous material: `ρ_I(vec(r)) = (dm)/(dI)` where `m` is the mass of the material at position `vec(r)` and `l` is length
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearMassDensityUnit[1];
    }

    attribute linearMassDensity: LinearMassDensityValue[*] nonunique :> scalarQuantities;

    attribute def LinearMassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias LinearDensityUnit for LinearMassDensityUnit;
    alias LinearDensityValue for LinearMassDensityValue;
    alias linearDensity for linearMassDensity;

    /* ISO-80000-4 item 4-7 moment of inertia */
    attribute def MomentOfInertiaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentOfInertiaUnit[1];
    }

    attribute momentOfInertia: MomentOfInertiaValue[*] nonunique :> scalarQuantities;

    attribute def MomentOfInertiaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    attribute def Cartesian3dMomentOfInertiaTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (tensor)
         * symbol(s): `vec(vec(J))`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dMomentOfInertiaMeasurementReference[1];
    }

    attribute momentOfInertiaTensor: Cartesian3dMomentOfInertiaTensor :> tensorQuantities;

    attribute def Cartesian3dMomentOfInertiaMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: MomentOfInertiaUnit[9];
    }

    /* ISO-80000-4 item 4-8 momentum */
    attribute def MomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-8 momentum (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 0
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentumUnit[1];
    }

    attribute momentum: MomentumValue[*] nonunique :> scalarQuantities;

    attribute def MomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-8 momentum (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 1
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMomentum3dCoordinateFrame[1];
    }

    attribute cartesianMomentum3dVector: CartesianMomentum3dVector :> vectorQuantities;

    attribute def CartesianMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MomentumUnit[3];
    }

    /* ISO-80000-4 item 4-9.1 force */
    attribute def ForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-9.1 force (magnitude)
         * symbol(s): `F`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ForceUnit[1];
    }

    attribute force: ForceValue[*] nonunique :> scalarQuantities;

    attribute def ForceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.1 force (vector)
         * symbol(s): `vec(F)`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianForce3dVector: CartesianForce3dVector :> vectorQuantities;

    attribute def CartesianForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ForceUnit[3];
    }

    /* ISO-80000-4 item 4-9.2 weight */
    attribute def CartesianWeight3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.2 weight
         * symbol(s): `vec(F_g)`
         * application domain: generic
         * name: Weight (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) acting on a body in the gravitational field of Earth: `vec(F_g) = m vec(g)` where `m` (item 4-1) is the mass of the body and `vec(g)` is the local acceleration of free fall (ISO 80000-3)
         * remarks: In colloquial language, the name "weight" continues to be used where "mass" is meant. This practice should be avoided. Weight is an example of a gravitational force. Weight comprises not only the local gravitational force but also the local centrifugal force due to the rotation of the Earth.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianWeight3dVector: CartesianWeight3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-9.3 static friction force, static friction */
    attribute def CartesianStaticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.3 static friction force, static friction
         * symbol(s): `vec(F_s)`
         * application domain: generic
         * name: StaticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion before a body starts to slide on a surface
         * remarks: For the static friction coefficient, see item 4-23.1.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianStaticFrictionForce3dVector: CartesianStaticFrictionForce3dVector :> vectorQuantities;

    alias cartesianStaticFriction3dVector for cartesianStaticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.4 kinetic friction force, dynamic friction force */
    attribute def CartesianKineticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.4 kinetic friction force, dynamic friction force
         * symbol(s): `vec(F_μ)`
         * application domain: generic
         * name: KineticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body slides on a surface
         * remarks: For the kinetic friction factor, see item 4-23.2.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianKineticFrictionForce3dVector: CartesianKineticFrictionForce3dVector :> vectorQuantities;

    alias cartesianDynamicFrictionForce3dVector for cartesianKineticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.5 rolling resistance, rolling drag, rolling friction force */
    attribute def CartesianRollingResistance3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.5 rolling resistance, rolling drag, rolling friction force
         * symbol(s): `vec(F_"rr")`
         * application domain: generic
         * name: RollingResistance (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body rolls on a surface
         * remarks: For the rolling resistance factor, see item 4-23.3.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianRollingResistance3dVector: CartesianRollingResistance3dVector :> vectorQuantities;

    alias cartesianRollingDrag3dVector for cartesianRollingResistance3dVector;

    alias cartesianRollingFrictionForce3dVector for cartesianRollingResistance3dVector;

    /* ISO-80000-4 item 4-9.6 drag force */
    attribute def CartesianDragForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.6 drag force
         * symbol(s): `vec(F_D)`
         * application domain: generic
         * name: DragForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion of a body in a fluid
         * remarks: For the drag coefficient, see item 4-23.4.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianDragForce3dVector: CartesianDragForce3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-10 impulse */
    attribute def ImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-10 impulse (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ImpulseUnit[1];
    }

    attribute impulse: ImpulseValue[*] nonunique :> scalarQuantities;

    attribute def ImpulseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-10 impulse (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianImpulse3dCoordinateFrame[1];
    }

    attribute cartesianImpulse3dVector: CartesianImpulse3dVector :> vectorQuantities;

    attribute def CartesianImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ImpulseUnit[3];
    }

    /* ISO-80000-4 item 4-11 angular momentum */
    attribute def AngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-11 angular momentum (magnitude)
         * symbol(s): `L`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularMomentumUnit[1];
    }

    attribute angularMomentum: AngularMomentumValue[*] nonunique :> scalarQuantities;

    attribute def AngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-11 angular momentum (vector)
         * symbol(s): `vec(L)`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularMomentum3dCoordinateFrame[1];
    }

    attribute cartesianAngularMomentum3dVector: CartesianAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularMomentumUnit[3];
    }

    /* ISO-80000-4 item 4-12.1 moment of force */
    attribute def MomentOfForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.1 moment of force (magnitude)
         * symbol(s): `M`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentOfForceUnit[1];
    }

    attribute momentOfForce: MomentOfForceValue[*] nonunique :> scalarQuantities;

    attribute def MomentOfForceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMomentOfForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-12.1 moment of force (vector)
         * symbol(s): `vec(M)`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMomentOfForce3dCoordinateFrame[1];
    }

    attribute cartesianMomentOfForce3dVector: CartesianMomentOfForce3dVector :> vectorQuantities;

    attribute def CartesianMomentOfForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MomentOfForceUnit[3];
    }

    /* ISO-80000-4 item 4-12.2 torque */
    attribute def TorqueValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.2 torque
         * symbol(s): `T`, `M_Q`
         * application domain: generic
         * name: Torque
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: quantity described by the scalar product: `T = vec(M)*vec(e_Q)` where `vec(M)` is moment of force (item 4-12.1) and `vec(e_Q)` is unit vector of direction with respect to which the torque is considered
         * remarks: For example, torque is the twisting moment of force with respect to the longitudinal axis of a beam or shaft.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TorqueUnit[1];
    }

    attribute torque: TorqueValue[*] nonunique :> scalarQuantities;

    attribute def TorqueUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-13 angular impulse */
    attribute def AngularImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-13 angular impulse (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularImpulseUnit[1];
    }

    attribute angularImpulse: AngularImpulseValue[*] nonunique :> scalarQuantities;

    attribute def AngularImpulseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianAngularImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-13 angular impulse (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularImpulse3dCoordinateFrame[1];
    }

    attribute cartesianAngularImpulse3dVector: CartesianAngularImpulse3dVector :> vectorQuantities;

    attribute def CartesianAngularImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularImpulseUnit[3];
    }

    /* ISO-80000-4 item 4-14.1 pressure */
    attribute def PressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-14.1 pressure
         * symbol(s): `p`
         * application domain: generic
         * name: Pressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of the component of a force normal to a surface and its area: `p = (vec(e_n) * vec(F)) / A` where `vec(e_n)` is unit vector of the surface normal, `vec(F)` is force (item 4-9.1) and `A` is area (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PressureUnit[1];
    }

    attribute pressure: PressureValue[*] nonunique :> scalarQuantities;

    attribute def PressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-14.2 gauge pressure */
    attribute gaugePressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 4-14.2 gauge pressure
         * symbol(s): `p_e`
         * application domain: generic
         * name: GaugePressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure `p` (item 4-14.1) decremented by ambient pressure `p_amb` : `p_e = p - p_amb`
         * remarks: Often, `p_amb` is chosen as a standard pressure. Gauge pressure is positive or negative.
         */
    }

    /* ISO-80000-4 item 4-15 stress */
    attribute def StressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-15 stress (magnitude)
         * symbol(s): `σ`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> num: Real;
        attribute :>> mRef: StressUnit[1];
    }

    attribute stress: StressValue[*] nonunique :> scalarQuantities;

    attribute def StressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def Cartesian3dStressTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-15 stress (tensor)
         * symbol(s): `vec(vec(σ))`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dStressMeasurementReference[1];
    }

    attribute stressTensor: Cartesian3dStressTensor :> tensorQuantities;

    attribute def Cartesian3dStressMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: StressUnit[9];
    }

    /* ISO-80000-4 item 4-16.1 normal stress */
    attribute def NormalStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.1 normal stress
         * symbol(s): `σ_n`, `σ`
         * application domain: generic
         * name: NormalStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `σ_n = (d F_n)/(dA)` where `F_n` is the normal component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter normal to it, and evenly distributed, cause a constant normal stress `σ_n = F A` in the slice (layer).
         */
        attribute :>> num: Real;
        attribute :>> mRef: NormalStressUnit[1];
    }

    attribute normalStress: NormalStressValue[*] nonunique :> scalarQuantities;

    attribute def NormalStressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-16.2 shear stress */
    attribute def ShearStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.2 shear stress
         * symbol(s): `τ_s`, `τ`
         * application domain: generic
         * name: ShearStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `τ_s = (d F_t)/(dA)` where `F_t` is the tangential component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter parallel to it, and evenly distributed, cause a constant shear stress `τ = F/A` in the slice (layer).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ShearStressUnit[1];
    }

    attribute shearStress: ShearStressValue[*] nonunique :> scalarQuantities;

    attribute def ShearStressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-17.1 strain */
    attribute def StrainValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (magnitude)
         * symbol(s): `ε`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> num: Real;
        attribute :>> mRef: StrainUnit[1];
    }

    attribute strain: StrainValue[*] nonunique :> scalarQuantities;

    attribute def StrainUnit :> DimensionOneUnit {
    }

    attribute def Cartesian3dStrainTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (tensor)
         * symbol(s): `vec(vec(ε))`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dStrainMeasurementReference[1];
    }

    attribute strainTensor: Cartesian3dStrainTensor :> tensorQuantities;

    attribute def Cartesian3dStrainMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: StrainUnit[9];
    }

    /* ISO-80000-4 item 4-17.2 relative linear strain */
    attribute def RelativeLinearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.2 relative linear strain
         * symbol(s): `ε`, `(e)`
         * application domain: generic
         * name: RelativeLinearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in length `Δl` (ISO 80000-3) of an object and its length `l` (ISO 80000-3): `ε = (Δl)/l`
         * remarks: None.
         */
    }
    attribute relativeLinearStrain: RelativeLinearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.3 shear strain */
    attribute def ShearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.3 shear strain
         * symbol(s): `γ`
         * application domain: generic
         * name: ShearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of parallel displacement `Δx` (ISO 80000-3) of two surfaces of a layer and the thickness `d` (ISO 80000-3) of the layer: `γ = (Δx)/d`
         * remarks: None.
         */
    }
    attribute shearStrain: ShearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.4 relative volume strain */
    attribute def RelativeVolumeStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.4 relative volume strain
         * symbol(s): `θ`
         * application domain: generic
         * name: RelativeVolumeStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in volume `ΔV` (ISO 80000-3) of an object and its volume `V_0` (ISO 80000-3): `θ = (ΔV)/V_0`
         * remarks: None.
         */
    }
    attribute relativeVolumeStrain: RelativeVolumeStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-18 Poisson number */
    attribute def PoissonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-18 Poisson number
         * symbol(s): `μ`, `(v)`
         * application domain: generic
         * name: PoissonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in width `Δb` (width is defined in ISO 80000-3) and change in length `Δl` (length is defined in ISO 80000-3) of an object: `μ = (Δb)/(Δl)`
         * remarks: None.
         */
    }
    attribute poissonNumber: PoissonNumberValue :> scalarQuantities;

    /* ISO-80000-4 item 4-19.1 modulus of elasticity, Young modulus */
    attribute def ModulusOfElasticityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.1 modulus of elasticity, Young modulus
         * symbol(s): `E`, `E_m`, `Y`
         * application domain: generic
         * name: ModulusOfElasticity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of normal stress `σ` (item 4-16.1) and relative linear strain `ε` (item 4-17.2): `E = σ/ε`
         * remarks: Conditions should be specified (e.g. adiabatic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfElasticityUnit[1];
    }

    attribute modulusOfElasticity: ModulusOfElasticityValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfElasticityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias YoungModulusUnit for ModulusOfElasticityUnit;
    alias YoungModulusValue for ModulusOfElasticityValue;
    alias youngModulus for modulusOfElasticity;

    /* ISO-80000-4 item 4-19.2 modulus of rigidity, shear modulus */
    attribute def ModulusOfRigidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.2 modulus of rigidity, shear modulus
         * symbol(s): `G`
         * application domain: generic
         * name: ModulusOfRigidity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of shear stress `τ` (item 4-16.2) and shear strain `γ` (item 4-17.3): `G = τ/γ`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfRigidityUnit[1];
    }

    attribute modulusOfRigidity: ModulusOfRigidityValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfRigidityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias ShearModulusUnit for ModulusOfRigidityUnit;
    alias ShearModulusValue for ModulusOfRigidityValue;
    alias shearModulus for modulusOfRigidity;

    /* ISO-80000-4 item 4-19.3 modulus of compression, bulk modulus */
    attribute def ModulusOfCompressionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.3 modulus of compression, bulk modulus
         * symbol(s): `K`, `K_m`, `B`
         * application domain: generic
         * name: ModulusOfCompression
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: negative of the quotient of pressure `p` (item 4-14.1) and relative volume strain `θ` (item 4-17.4): `K = -(p/θ)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfCompressionUnit[1];
    }

    attribute modulusOfCompression: ModulusOfCompressionValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfCompressionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias BulkModulusUnit for ModulusOfCompressionUnit;
    alias BulkModulusValue for ModulusOfCompressionValue;
    alias bulkModulus for modulusOfCompression;

    /* ISO-80000-4 item 4-20 compressibility */
    attribute def CompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-20 compressibility
         * symbol(s): `ϰ`
         * application domain: generic
         * name: Compressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume `V` (ISO 80000-3) of an object under pressure `p` (item 4-14.1) expressed by: `ϰ = -(1/V)(dV)/(dp)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process). See also ISO 80000-5.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CompressibilityUnit[1];
    }

    attribute compressibility: CompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def CompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-21.1 second axial moment of area */
    attribute def SecondAxialMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.1 second axial moment of area
         * symbol(s): `I_a`
         * application domain: generic
         * name: SecondAxialMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_a = int int_M r_Q^2 dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis in the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `a`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SecondAxialMomentOfAreaUnit[1];
    }

    attribute secondAxialMomentOfArea: SecondAxialMomentOfAreaValue[*] nonunique :> scalarQuantities;

    attribute def SecondAxialMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-21.2 second polar moment of area */
    attribute def SecondPolarMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.2 second polar moment of area
         * symbol(s): `I_p`
         * application domain: generic
         * name: SecondPolarMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_p = int int_M r_Q^2 * dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis perpendicular to the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `p`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SecondPolarMomentOfAreaUnit[1];
    }

    attribute secondPolarMomentOfArea: SecondPolarMomentOfAreaValue[*] nonunique :> scalarQuantities;

    attribute def SecondPolarMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-22 section modulus */
    attribute def SectionModulusValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-22 section modulus
         * symbol(s): `Z`, `(W)`
         * application domain: generic
         * name: SectionModulus
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `Z = I_a/r_(Q_max)` where `I_a` is the second axial moment of area (item 4-21.1) and `r_(Q,max)` is the maximum radial distance (ISO 80000-3) of any point in the surface considered from the Q-axis with respect to which `I_a` is defined
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SectionModulusUnit[1];
    }

    attribute sectionModulus: SectionModulusValue[*] nonunique :> scalarQuantities;

    attribute def SectionModulusUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction */
    attribute def StaticFrictionCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction
         * symbol(s): `μ_s`, `(f_s)`
         * application domain: generic
         * name: StaticFrictionCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the maximum magnitude of the tangential component `F_max` of the static friction force (item 4-9.3) and the magnitude of the normal component `N` of the contact force (item 4-9.1) between two bodies at relative rest with respect to each other: `F_max = μ_s * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both.
         */
    }
    attribute staticFrictionCoefficient: StaticFrictionCoefficientValue :> scalarQuantities;

    alias staticFrictionFactor for staticFrictionCoefficient;

    alias coefficientOfStaticFriction for staticFrictionCoefficient;

    /* ISO-80000-4 item 4-23.2 kinetic friction factor, dynamic friction factor */
    attribute def KineticFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.2 kinetic friction factor, dynamic friction factor
         * symbol(s): `μ`, `(f)`
         * application domain: generic
         * name: KineticFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitudes of the kinetic friction force, `F_μ` (item 4-9.4) and the normal component `N` of the contact force (item 4-9.1): `F_μ = μ * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both. The dynamic friction factor `µ` is independent in first approximation of the contact surface.
         */
    }
    attribute kineticFrictionFactor: KineticFrictionFactorValue :> scalarQuantities;

    alias dynamicFrictionFactor for kineticFrictionFactor;

    /* ISO-80000-4 item 4-23.3 rolling resistance factor */
    attribute def RollingResistanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.3 rolling resistance factor
         * symbol(s): `C_"rr"`
         * application domain: generic
         * name: RollingResistanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitude of the tangential component `F` and the magnitude of the normal component `N` of the force applied to a body rolling on a surface at constant speed: `F = C_(rr)*N`
         * remarks: Also known as rolling resistance coefficient, RRC.
         */
    }
    attribute rollingResistanceFactor: RollingResistanceFactorValue :> scalarQuantities;

    /* ISO-80000-4 item 4-23.4 drag coefficient, drag factor */
    attribute def DragCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.4 drag coefficient, drag factor
         * symbol(s): `C_D`
         * application domain: generic
         * name: DragCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor proportional to magnitude `F_D` of the drag force (item 4-9.6) of a body moving in a fluid, dependent on the shape and speed `v` (ISO 80000-3) of a body: `F_D = 1/2 * C_D * ρ * v^2 * A` where `ρ` is mass density (item 4-2) of the fluid and `A` is cross-section area (ISO 80000-3) of the body
         * remarks: None.
         */
    }
    attribute dragCoefficient: DragCoefficientValue :> scalarQuantities;

    alias dragFactor for dragCoefficient;

    /* ISO-80000-4 item 4-24 dynamic viscosity, viscosity */
    attribute def DynamicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-24 dynamic viscosity, viscosity
         * symbol(s): `η`
         * application domain: generic
         * name: DynamicViscosity
         * quantity dimension: L^-1*M^1*T^-1
         * measurement unit(s): Pa*s, kg*m^-1*s^-1
         * tensor order: 0
         * definition: for laminar flows, proportionality constant between shear stress `τ_(xz)` (item 4-16.2) in a fluid moving with a velocity `v_x` (ISO 80000-3) and gradient `(d v_x)/dz` perpendicular to the plane of shear: `τ_(xz) = η (d v_x)/(dz)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DynamicViscosityUnit[1];
    }

    attribute dynamicViscosity: DynamicViscosityValue[*] nonunique :> scalarQuantities;

    attribute def DynamicViscosityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias ViscosityUnit for DynamicViscosityUnit;
    alias ViscosityValue for DynamicViscosityValue;
    alias viscosity for dynamicViscosity;

    /* ISO-80000-4 item 4-25 kinematic viscosity */
    attribute def KinematicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-25 kinematic viscosity
         * symbol(s): `v`
         * application domain: generic
         * name: KinematicViscosity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of dynamic viscosity `η` (item 4-24) and mass density `ρ` (item 4-2) of a fluid: `v = η/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KinematicViscosityUnit[1];
    }

    attribute kinematicViscosity: KinematicViscosityValue[*] nonunique :> scalarQuantities;

    attribute def KinematicViscosityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-4 item 4-26 surface tension */
    attribute def SurfaceTensionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-26 surface tension
         * symbol(s): `γ`, `σ`
         * application domain: generic
         * name: SurfaceTension
         * quantity dimension: M^1*T^-2
         * measurement unit(s): N*m^-1, kg*s^-2
         * tensor order: 0
         * definition: magnitude of a force acting against the enlargement of area portion of a surface separating a liquid from its surrounding
         * remarks: The concept of surface energy is closely related to surface tension and has the same dimension.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceTensionUnit[1];
    }

    attribute surfaceTension: SurfaceTensionValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceTensionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-27.1 power */
    attribute def PowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-27.1 power
         * symbol(s): `P`
         * application domain: generic
         * name: Power
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: quotient of energy (ISO 80000-5) and duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PowerUnit[1];
    }

    attribute power: PowerValue[*] nonunique :> scalarQuantities;

    attribute def PowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-27 mechanical power */
    attribute mechanicalPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 4-27 mechanical power
         * symbol(s): `P`
         * application domain: mechanics
         * name: MechanicalPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, N*m*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: scalar product of force `vec(F)` (item 4-9.1) acting to a body and its velocity `vec(v)` (ISO 80000-3): `P = vec(F) * vec(v)`
         * remarks: None.
         */
    }

    /* ISO-80000-4 item 4-28.1 potential energy */
    attribute potentialEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.1 potential energy
         * symbol(s): `V`, `E_p`
         * application domain: generic
         * name: PotentialEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: for conservative force `vec(F)`, scalar additive quantity obeying condition `vec(F) = -nabla F`, if it exists
         * remarks: For the definition of energy, see ISO 80000-5. A force is conservative when the force field is irrotational, i.e. `rot(F) = 0` , or `vec(F)` is perpendicular to the speed of the body to ensure `vec(F) * d vec(r) = 0` .
         */
    }

    /* ISO-80000-4 item 4-28.2 kinetic energy */
    attribute kineticEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.2 kinetic energy
         * symbol(s): `T`, `E_k`
         * application domain: generic
         * name: KineticEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing a moving body expressed by: `T = 1/2 m v^2` where `m` is mass (item 4-1) of the body and `v` is its speed (ISO 80000-3)
         * remarks: For the definition of energy, see ISO 80000-5.
         */
    }

    /* ISO-80000-4 item 4-28.3 mechanical energy */
    attribute mechanicalEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.3 mechanical energy
         * symbol(s): `E`, `W`
         * application domain: generic
         * name: MechanicalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of kinetic energy `T` (item 4-28.2) and potential energy `V` (item 4-28.1): `E = T+V`
         * remarks: The symbols `E` and `W` are also used for other kinds of energy. This definition is understood in a classical way and it does not include thermal motion.
         */
    }

    /* ISO-80000-4 item 4-28.4 mechanical work, work */
    attribute mechanicalWork: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.4 mechanical work, work
         * symbol(s): `A`, `W`
         * application domain: generic
         * name: MechanicalWork (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: process quantity describing the total action of a force `vec(F)` (item 4-9.1) along a continuous curve `Γ` in three-dimensional space with infinitesimal displacement (ISO 80000-3) `dvec(r)`, as a line integral of their scalar product: `A = int_Γ vec(F) * d vec(r)`
         * remarks: The definition covers the case `A = -int_Γ p*dV` where `Γ` is a curve in the phase space and implies that work generally depends upon `Γ`, and that type of process must be defined (e.g. isentropic or isothermic).
         */
    }

    alias work for mechanicalWork;

    /* ISO-80000-4 item 4-29 mechanical efficiency */
    attribute def MechanicalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-29 mechanical efficiency
         * symbol(s): `η`
         * application domain: mechanics
         * name: MechanicalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of output power `P_"out"` (item 4-27) from a system and input power `P_"in"` (item 4-27) to this system: `η = P_"out"/P_"in"`
         * remarks: The system must be specified. This quantity is often expressed by the unit percent, symbol %.
         */
    }
    attribute mechanicalEfficiency: MechanicalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-4 item 4-30.1 mass flow */
    attribute def MassFlowValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.1 mass flow (magnitude)
         * symbol(s): `j_m`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassFlowUnit[1];
    }

    attribute massFlow: MassFlowValue[*] nonunique :> scalarQuantities;

    attribute def MassFlowUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMassFlow3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-30.1 mass flow (vector)
         * symbol(s): `vec(j_m)`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMassFlow3dCoordinateFrame[1];
    }

    attribute cartesianMassFlow3dVector: CartesianMassFlow3dVector :> vectorQuantities;

    attribute def CartesianMassFlow3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MassFlowUnit[3];
    }

    /* ISO-80000-4 item 4-30.2 mass flow rate */
    attribute def MassFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.2 mass flow rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassFlowRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with normal vector `vec(e)_n` of a flowing fluid with mass flow `vec(j)_m` (item 4-30.1) as an integral: `q_m = int int_A vec(j)_m * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassFlowRateUnit[1];
    }

    attribute massFlowRate: MassFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def MassFlowRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-30.3 mass change rate */
    attribute def MassChangeRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.3 mass change rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassChangeRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: rate of increment of mass `m` (item 4-1): `q_m = (dm)/(dt)` where `dm` is the infinitesimal mass (item 4-1) increment and `dt` is the infinitesimal duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassChangeRateUnit[1];
    }

    attribute massChangeRate: MassChangeRateValue[*] nonunique :> scalarQuantities;

    attribute def MassChangeRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-31 volume flow rate */
    attribute def VolumeFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-31 volume flow rate
         * symbol(s): `q_v`
         * application domain: generic
         * name: VolumeFlowRate
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with the normal vector `vec(e)_n` of a flowing fluid with velocity `vec(v)` (ISO 80000-3) as an integral: `q_v = int int_A vec(v) * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeFlowRateUnit[1];
    }

    attribute volumeFlowRate: VolumeFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def VolumeFlowRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-4 item 4-32 action quantity */
    attribute def ActionQuantityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-32 action quantity
         * symbol(s): `S`
         * application domain: generic
         * name: ActionQuantity
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: time integral of energy `E` over a time interval `(t_1, t_2)`: `S = int_(t_1)^(t_2) E dt`
         * remarks: The energy may be expressed by a Lagrangian or Hamiltonian function. Note for SysML: the ISQ quantity "action" has been renamed to "action quantity" to avoid the name clash with the SysML action keyword.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ActionQuantityUnit[1];
    }

    attribute actionQuantity: ActionQuantityValue[*] nonunique :> scalarQuantities;

    attribute def ActionQuantityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_mechanics.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQMechanics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-4:2019 "Mechanics"
     * see also https://www.iso.org/standard/64975.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;
    /* Quantity definitions referenced from other ISQ packages */
    private import ISQThermodynamics::EnergyValue;
    /* ISO-80000-4 item 4-1 mass */
    /* See package ISQBase for the declarations of MassValue and MassUnit */
    /* ISO-80000-4 item 4-2 mass density, density */
    attribute def MassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-2 mass density, density
         * symbol(s): `ρ`, `ρ_m`
         * application domain: generic
         * name: MassDensity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quantity representing the spatial distribution of mass of a continuous material: `ρ(vec(r)) = (dm)/(dV)` where `m` is mass of the material contained in an infinitesimal domain at point `vec(r)` and `V` is volume of this domain
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassDensityUnit[1];
    }
    attribute massDensity : MassDensityValue[*] nonunique :> scalarQuantities;
    attribute def MassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    alias DensityUnit for MassDensityUnit;
    alias DensityValue for MassDensityValue;
    alias density for massDensity;
    /* ISO-80000-4 item 4-3 specific volume */
    attribute def SpecificVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-3 specific volume
         * symbol(s): `v`
         * application domain: generic
         * name: SpecificVolume
         * quantity dimension: L^3*M^-1
         * measurement unit(s): kg^-1*m^3
         * tensor order: 0
         * definition: reciprocal of mass density `ρ` (item 4-2): `v = 1/ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificVolumeUnit[1];
    }
    attribute specificVolume : SpecificVolumeValue[*] nonunique :> scalarQuantities;
    attribute def SpecificVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    /* ISO-80000-4 item 4-4 relative mass density, relative density */
    attribute def RelativeMassDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-4 relative mass density, relative density
         * symbol(s): `d`
         * application domain: generic
         * name: RelativeMassDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass density of a substance `ρ` and mass density of a reference substance `ρ_0` : `d = ρ/ρ_0`
         * remarks: Conditions and material should be specified for the reference substance.
         */
    }
    attribute relativeMassDensity : RelativeMassDensityValue :> scalarQuantities;
    alias relativeDensity for relativeMassDensity;
    /* ISO-80000-4 item 4-5 surface mass density, surface density */
    attribute def SurfaceMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-5 surface mass density, surface density
         * symbol(s): `ρ_A`
         * application domain: generic
         * name: SurfaceMassDensity
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: quantity representing the areal distribution of mass of a continuous material: `ρ_A(vec(r)) = (dm)/(dA)` where `m` is the mass of the material at position `vec(r)` and `A` is area
         * remarks: The name "grammage" should not be used for this quantity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceMassDensityUnit[1];
    }
    attribute surfaceMassDensity : SurfaceMassDensityValue[*] nonunique :> scalarQuantities;
    attribute def SurfaceMassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    alias SurfaceDensityUnit for SurfaceMassDensityUnit;
    alias SurfaceDensityValue for SurfaceMassDensityValue;
    alias surfaceDensity for surfaceMassDensity;
    /* ISO-80000-4 item 4-6 linear mass density, linear density */
    attribute def LinearMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-6 linear mass density, linear density
         * symbol(s): `ρ_I`
         * application domain: generic
         * name: LinearMassDensity
         * quantity dimension: L^-1*M^1
         * measurement unit(s): kg*m^-1
         * tensor order: 0
         * definition: quantity representing the linear distribution of mass of a continuous material: `ρ_I(vec(r)) = (dm)/(dI)` where `m` is the mass of the material at position `vec(r)` and `l` is length
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearMassDensityUnit[1];
    }
    attribute linearMassDensity : LinearMassDensityValue[*] nonunique :> scalarQuantities;
    attribute def LinearMassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    alias LinearDensityUnit for LinearMassDensityUnit;
    alias LinearDensityValue for LinearMassDensityValue;
    alias linearDensity for linearMassDensity;
    /* ISO-80000-4 item 4-7 moment of inertia */
    attribute def MomentOfInertiaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentOfInertiaUnit[1];
    }
    attribute momentOfInertia : MomentOfInertiaValue[*] nonunique :> scalarQuantities;
    attribute def MomentOfInertiaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    attribute def Cartesian3dMomentOfInertiaTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (tensor)
         * symbol(s): `vec(vec(J))`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real[9];
        attribute :>> mRef : Cartesian3dMomentOfInertiaMeasurementReference[1];
    }
    attribute momentOfInertiaTensor : Cartesian3dMomentOfInertiaTensor :> tensorQuantities;
    attribute def Cartesian3dMomentOfInertiaMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : MomentOfInertiaUnit[9];
    }
    /* ISO-80000-4 item 4-8 momentum */
    attribute def MomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-8 momentum (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 0
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentumUnit[1];
    }
    attribute momentum : MomentumValue[*] nonunique :> scalarQuantities;
    attribute def MomentumUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CartesianMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-8 momentum (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 1
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMomentum3dCoordinateFrame[1];
    }
    attribute cartesianMomentum3dVector : CartesianMomentum3dVector :> vectorQuantities;
    attribute def CartesianMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MomentumUnit[3];
    }
    /* ISO-80000-4 item 4-9.1 force */
    attribute def ForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-9.1 force (magnitude)
         * symbol(s): `F`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ForceUnit[1];
    }
    attribute force : ForceValue[*] nonunique :> scalarQuantities;
    attribute def ForceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
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
    attribute def CartesianForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.1 force (vector)
         * symbol(s): `vec(F)`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianForce3dVector : CartesianForce3dVector :> vectorQuantities;
    attribute def CartesianForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ForceUnit[3];
    }
    /* ISO-80000-4 item 4-9.2 weight */
    attribute def CartesianWeight3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.2 weight
         * symbol(s): `vec(F_g)`
         * application domain: generic
         * name: Weight (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) acting on a body in the gravitational field of Earth: `vec(F_g) = m vec(g)` where `m` (item 4-1) is the mass of the body and `vec(g)` is the local acceleration of free fall (ISO 80000-3)
         * remarks: In colloquial language, the name "weight" continues to be used where "mass" is meant. This practice should be avoided. Weight is an example of a gravitational force. Weight comprises not only the local gravitational force but also the local centrifugal force due to the rotation of the Earth.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianWeight3dVector : CartesianWeight3dVector :> vectorQuantities;
    /* ISO-80000-4 item 4-9.3 static friction force, static friction */
    attribute def CartesianStaticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.3 static friction force, static friction
         * symbol(s): `vec(F_s)`
         * application domain: generic
         * name: StaticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion before a body starts to slide on a surface
         * remarks: For the static friction coefficient, see item 4-23.1.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianStaticFrictionForce3dVector : CartesianStaticFrictionForce3dVector :> vectorQuantities;
    alias cartesianStaticFriction3dVector for cartesianStaticFrictionForce3dVector;
    /* ISO-80000-4 item 4-9.4 kinetic friction force, dynamic friction force */
    attribute def CartesianKineticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.4 kinetic friction force, dynamic friction force
         * symbol(s): `vec(F_μ)`
         * application domain: generic
         * name: KineticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body slides on a surface
         * remarks: For the kinetic friction factor, see item 4-23.2.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianKineticFrictionForce3dVector : CartesianKineticFrictionForce3dVector :> vectorQuantities;
    alias cartesianDynamicFrictionForce3dVector for cartesianKineticFrictionForce3dVector;
    /* ISO-80000-4 item 4-9.5 rolling resistance, rolling drag, rolling friction force */
    attribute def CartesianRollingResistance3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.5 rolling resistance, rolling drag, rolling friction force
         * symbol(s): `vec(F_"rr")`
         * application domain: generic
         * name: RollingResistance (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body rolls on a surface
         * remarks: For the rolling resistance factor, see item 4-23.3.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianRollingResistance3dVector : CartesianRollingResistance3dVector :> vectorQuantities;
    alias cartesianRollingDrag3dVector for cartesianRollingResistance3dVector;
    alias cartesianRollingFrictionForce3dVector for cartesianRollingResistance3dVector;
    /* ISO-80000-4 item 4-9.6 drag force */
    attribute def CartesianDragForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.6 drag force
         * symbol(s): `vec(F_D)`
         * application domain: generic
         * name: DragForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion of a body in a fluid
         * remarks: For the drag coefficient, see item 4-23.4.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame[1];
    }
    attribute cartesianDragForce3dVector : CartesianDragForce3dVector :> vectorQuantities;
    /* ISO-80000-4 item 4-10 impulse */
    attribute def ImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-10 impulse (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ImpulseUnit[1];
    }
    attribute impulse : ImpulseValue[*] nonunique :> scalarQuantities;
    attribute def ImpulseUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CartesianImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-10 impulse (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianImpulse3dCoordinateFrame[1];
    }
    attribute cartesianImpulse3dVector : CartesianImpulse3dVector :> vectorQuantities;
    attribute def CartesianImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ImpulseUnit[3];
    }
    /* ISO-80000-4 item 4-11 angular momentum */
    attribute def AngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-11 angular momentum (magnitude)
         * symbol(s): `L`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularMomentumUnit[1];
    }
    attribute angularMomentum : AngularMomentumValue[*] nonunique :> scalarQuantities;
    attribute def AngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CartesianAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-11 angular momentum (vector)
         * symbol(s): `vec(L)`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularMomentum3dCoordinateFrame[1];
    }
    attribute cartesianAngularMomentum3dVector : CartesianAngularMomentum3dVector :> vectorQuantities;
    attribute def CartesianAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularMomentumUnit[3];
    }
    /* ISO-80000-4 item 4-12.1 moment of force */
    attribute def MomentOfForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.1 moment of force (magnitude)
         * symbol(s): `M`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentOfForceUnit[1];
    }
    attribute momentOfForce : MomentOfForceValue[*] nonunique :> scalarQuantities;
    attribute def MomentOfForceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
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
    attribute def CartesianMomentOfForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-12.1 moment of force (vector)
         * symbol(s): `vec(M)`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMomentOfForce3dCoordinateFrame[1];
    }
    attribute cartesianMomentOfForce3dVector : CartesianMomentOfForce3dVector :> vectorQuantities;
    attribute def CartesianMomentOfForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MomentOfForceUnit[3];
    }
    /* ISO-80000-4 item 4-12.2 torque */
    attribute def TorqueValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.2 torque
         * symbol(s): `T`, `M_Q`
         * application domain: generic
         * name: Torque
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: quantity described by the scalar product: `T = vec(M)*vec(e_Q)` where `vec(M)` is moment of force (item 4-12.1) and `vec(e_Q)` is unit vector of direction with respect to which the torque is considered
         * remarks: For example, torque is the twisting moment of force with respect to the longitudinal axis of a beam or shaft.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TorqueUnit[1];
    }
    attribute torque : TorqueValue[*] nonunique :> scalarQuantities;
    attribute def TorqueUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
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
    /* ISO-80000-4 item 4-13 angular impulse */
    attribute def AngularImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-13 angular impulse (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularImpulseUnit[1];
    }
    attribute angularImpulse : AngularImpulseValue[*] nonunique :> scalarQuantities;
    attribute def AngularImpulseUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CartesianAngularImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-13 angular impulse (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularImpulse3dCoordinateFrame[1];
    }
    attribute cartesianAngularImpulse3dVector : CartesianAngularImpulse3dVector :> vectorQuantities;
    attribute def CartesianAngularImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularImpulseUnit[3];
    }
    /* ISO-80000-4 item 4-14.1 pressure */
    attribute def PressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-14.1 pressure
         * symbol(s): `p`
         * application domain: generic
         * name: Pressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of the component of a force normal to a surface and its area: `p = (vec(e_n) * vec(F)) / A` where `vec(e_n)` is unit vector of the surface normal, `vec(F)` is force (item 4-9.1) and `A` is area (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PressureUnit[1];
    }
    attribute pressure : PressureValue[*] nonunique :> scalarQuantities;
    attribute def PressureUnit :> DerivedUnit {
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
    /* ISO-80000-4 item 4-14.2 gauge pressure */
    attribute gaugePressure : PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 4-14.2 gauge pressure
         * symbol(s): `p_e`
         * application domain: generic
         * name: GaugePressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure `p` (item 4-14.1) decremented by ambient pressure `p_amb` : `p_e = p - p_amb`
         * remarks: Often, `p_amb` is chosen as a standard pressure. Gauge pressure is positive or negative.
         */
    }
    /* ISO-80000-4 item 4-15 stress */
    attribute def StressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-15 stress (magnitude)
         * symbol(s): `σ`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> num : Real;
        attribute :>> mRef : StressUnit[1];
    }
    attribute stress : StressValue[*] nonunique :> scalarQuantities;
    attribute def StressUnit :> DerivedUnit {
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
    attribute def Cartesian3dStressTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-15 stress (tensor)
         * symbol(s): `vec(vec(σ))`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real[9];
        attribute :>> mRef : Cartesian3dStressMeasurementReference[1];
    }
    attribute stressTensor : Cartesian3dStressTensor :> tensorQuantities;
    attribute def Cartesian3dStressMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : StressUnit[9];
    }
    /* ISO-80000-4 item 4-16.1 normal stress */
    attribute def NormalStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.1 normal stress
         * symbol(s): `σ_n`, `σ`
         * application domain: generic
         * name: NormalStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `σ_n = (d F_n)/(dA)` where `F_n` is the normal component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter normal to it, and evenly distributed, cause a constant normal stress `σ_n = F A` in the slice (layer).
         */
        attribute :>> num : Real;
        attribute :>> mRef : NormalStressUnit[1];
    }
    attribute normalStress : NormalStressValue[*] nonunique :> scalarQuantities;
    attribute def NormalStressUnit :> DerivedUnit {
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
    /* ISO-80000-4 item 4-16.2 shear stress */
    attribute def ShearStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.2 shear stress
         * symbol(s): `τ_s`, `τ`
         * application domain: generic
         * name: ShearStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `τ_s = (d F_t)/(dA)` where `F_t` is the tangential component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter parallel to it, and evenly distributed, cause a constant shear stress `τ = F/A` in the slice (layer).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ShearStressUnit[1];
    }
    attribute shearStress : ShearStressValue[*] nonunique :> scalarQuantities;
    attribute def ShearStressUnit :> DerivedUnit {
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
    /* ISO-80000-4 item 4-17.1 strain */
    attribute def StrainValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (magnitude)
         * symbol(s): `ε`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> num : Real;
        attribute :>> mRef : StrainUnit[1];
    }
    attribute strain : StrainValue[*] nonunique :> scalarQuantities;
    attribute def StrainUnit :> DimensionOneUnit {
    }
    attribute def Cartesian3dStrainTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (tensor)
         * symbol(s): `vec(vec(ε))`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real[9];
        attribute :>> mRef : Cartesian3dStrainMeasurementReference[1];
    }
    attribute strainTensor : Cartesian3dStrainTensor :> tensorQuantities;
    attribute def Cartesian3dStrainMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : StrainUnit[9];
    }
    /* ISO-80000-4 item 4-17.2 relative linear strain */
    attribute def RelativeLinearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.2 relative linear strain
         * symbol(s): `ε`, `(e)`
         * application domain: generic
         * name: RelativeLinearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in length `Δl` (ISO 80000-3) of an object and its length `l` (ISO 80000-3): `ε = (Δl)/l`
         * remarks: None.
         */
    }
    attribute relativeLinearStrain : RelativeLinearStrainValue :> scalarQuantities;
    /* ISO-80000-4 item 4-17.3 shear strain */
    attribute def ShearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.3 shear strain
         * symbol(s): `γ`
         * application domain: generic
         * name: ShearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of parallel displacement `Δx` (ISO 80000-3) of two surfaces of a layer and the thickness `d` (ISO 80000-3) of the layer: `γ = (Δx)/d`
         * remarks: None.
         */
    }
    attribute shearStrain : ShearStrainValue :> scalarQuantities;
    /* ISO-80000-4 item 4-17.4 relative volume strain */
    attribute def RelativeVolumeStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.4 relative volume strain
         * symbol(s): `θ`
         * application domain: generic
         * name: RelativeVolumeStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in volume `ΔV` (ISO 80000-3) of an object and its volume `V_0` (ISO 80000-3): `θ = (ΔV)/V_0`
         * remarks: None.
         */
    }
    attribute relativeVolumeStrain : RelativeVolumeStrainValue :> scalarQuantities;
    /* ISO-80000-4 item 4-18 Poisson number */
    attribute def PoissonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-18 Poisson number
         * symbol(s): `μ`, `(v)`
         * application domain: generic
         * name: PoissonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in width `Δb` (width is defined in ISO 80000-3) and change in length `Δl` (length is defined in ISO 80000-3) of an object: `μ = (Δb)/(Δl)`
         * remarks: None.
         */
    }
    attribute poissonNumber : PoissonNumberValue :> scalarQuantities;
    /* ISO-80000-4 item 4-19.1 modulus of elasticity, Young modulus */
    attribute def ModulusOfElasticityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.1 modulus of elasticity, Young modulus
         * symbol(s): `E`, `E_m`, `Y`
         * application domain: generic
         * name: ModulusOfElasticity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of normal stress `σ` (item 4-16.1) and relative linear strain `ε` (item 4-17.2): `E = σ/ε`
         * remarks: Conditions should be specified (e.g. adiabatic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfElasticityUnit[1];
    }
    attribute modulusOfElasticity : ModulusOfElasticityValue[*] nonunique :> scalarQuantities;
    attribute def ModulusOfElasticityUnit :> DerivedUnit {
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
    alias YoungModulusUnit for ModulusOfElasticityUnit;
    alias YoungModulusValue for ModulusOfElasticityValue;
    alias youngModulus for modulusOfElasticity;
    /* ISO-80000-4 item 4-19.2 modulus of rigidity, shear modulus */
    attribute def ModulusOfRigidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.2 modulus of rigidity, shear modulus
         * symbol(s): `G`
         * application domain: generic
         * name: ModulusOfRigidity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of shear stress `τ` (item 4-16.2) and shear strain `γ` (item 4-17.3): `G = τ/γ`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfRigidityUnit[1];
    }
    attribute modulusOfRigidity : ModulusOfRigidityValue[*] nonunique :> scalarQuantities;
    attribute def ModulusOfRigidityUnit :> DerivedUnit {
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
    alias ShearModulusUnit for ModulusOfRigidityUnit;
    alias ShearModulusValue for ModulusOfRigidityValue;
    alias shearModulus for modulusOfRigidity;
    /* ISO-80000-4 item 4-19.3 modulus of compression, bulk modulus */
    attribute def ModulusOfCompressionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.3 modulus of compression, bulk modulus
         * symbol(s): `K`, `K_m`, `B`
         * application domain: generic
         * name: ModulusOfCompression
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: negative of the quotient of pressure `p` (item 4-14.1) and relative volume strain `θ` (item 4-17.4): `K = -(p/θ)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfCompressionUnit[1];
    }
    attribute modulusOfCompression : ModulusOfCompressionValue[*] nonunique :> scalarQuantities;
    attribute def ModulusOfCompressionUnit :> DerivedUnit {
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
    alias BulkModulusUnit for ModulusOfCompressionUnit;
    alias BulkModulusValue for ModulusOfCompressionValue;
    alias bulkModulus for modulusOfCompression;
    /* ISO-80000-4 item 4-20 compressibility */
    attribute def CompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-20 compressibility
         * symbol(s): `ϰ`
         * application domain: generic
         * name: Compressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume `V` (ISO 80000-3) of an object under pressure `p` (item 4-14.1) expressed by: `ϰ = -(1/V)(dV)/(dp)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process). See also ISO 80000-5.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CompressibilityUnit[1];
    }
    attribute compressibility : CompressibilityValue[*] nonunique :> scalarQuantities;
    attribute def CompressibilityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-21.1 second axial moment of area */
    attribute def SecondAxialMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.1 second axial moment of area
         * symbol(s): `I_a`
         * application domain: generic
         * name: SecondAxialMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_a = int int_M r_Q^2 dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis in the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `a`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SecondAxialMomentOfAreaUnit[1];
    }
    attribute secondAxialMomentOfArea : SecondAxialMomentOfAreaValue[*] nonunique :> scalarQuantities;
    attribute def SecondAxialMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 4;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-4 item 4-21.2 second polar moment of area */
    attribute def SecondPolarMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.2 second polar moment of area
         * symbol(s): `I_p`
         * application domain: generic
         * name: SecondPolarMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_p = int int_M r_Q^2 * dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis perpendicular to the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `p`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SecondPolarMomentOfAreaUnit[1];
    }
    attribute secondPolarMomentOfArea : SecondPolarMomentOfAreaValue[*] nonunique :> scalarQuantities;
    attribute def SecondPolarMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 4;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-4 item 4-22 section modulus */
    attribute def SectionModulusValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-22 section modulus
         * symbol(s): `Z`, `(W)`
         * application domain: generic
         * name: SectionModulus
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `Z = I_a/r_(Q_max)` where `I_a` is the second axial moment of area (item 4-21.1) and `r_(Q,max)` is the maximum radial distance (ISO 80000-3) of any point in the surface considered from the Q-axis with respect to which `I_a` is defined
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SectionModulusUnit[1];
    }
    attribute sectionModulus : SectionModulusValue[*] nonunique :> scalarQuantities;
    attribute def SectionModulusUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-4 item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction */
    attribute def StaticFrictionCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction
         * symbol(s): `μ_s`, `(f_s)`
         * application domain: generic
         * name: StaticFrictionCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the maximum magnitude of the tangential component `F_max` of the static friction force (item 4-9.3) and the magnitude of the normal component `N` of the contact force (item 4-9.1) between two bodies at relative rest with respect to each other: `F_max = μ_s * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both.
         */
    }
    attribute staticFrictionCoefficient : StaticFrictionCoefficientValue :> scalarQuantities;
    alias staticFrictionFactor for staticFrictionCoefficient;
    alias coefficientOfStaticFriction for staticFrictionCoefficient;
    /* ISO-80000-4 item 4-23.2 kinetic friction factor, dynamic friction factor */
    attribute def KineticFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.2 kinetic friction factor, dynamic friction factor
         * symbol(s): `μ`, `(f)`
         * application domain: generic
         * name: KineticFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitudes of the kinetic friction force, `F_μ` (item 4-9.4) and the normal component `N` of the contact force (item 4-9.1): `F_μ = μ * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both. The dynamic friction factor `µ` is independent in first approximation of the contact surface.
         */
    }
    attribute kineticFrictionFactor : KineticFrictionFactorValue :> scalarQuantities;
    alias dynamicFrictionFactor for kineticFrictionFactor;
    /* ISO-80000-4 item 4-23.3 rolling resistance factor */
    attribute def RollingResistanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.3 rolling resistance factor
         * symbol(s): `C_"rr"`
         * application domain: generic
         * name: RollingResistanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitude of the tangential component `F` and the magnitude of the normal component `N` of the force applied to a body rolling on a surface at constant speed: `F = C_(rr)*N`
         * remarks: Also known as rolling resistance coefficient, RRC.
         */
    }
    attribute rollingResistanceFactor : RollingResistanceFactorValue :> scalarQuantities;
    /* ISO-80000-4 item 4-23.4 drag coefficient, drag factor */
    attribute def DragCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.4 drag coefficient, drag factor
         * symbol(s): `C_D`
         * application domain: generic
         * name: DragCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor proportional to magnitude `F_D` of the drag force (item 4-9.6) of a body moving in a fluid, dependent on the shape and speed `v` (ISO 80000-3) of a body: `F_D = 1/2 * C_D * ρ * v^2 * A` where `ρ` is mass density (item 4-2) of the fluid and `A` is cross-section area (ISO 80000-3) of the body
         * remarks: None.
         */
    }
    attribute dragCoefficient : DragCoefficientValue :> scalarQuantities;
    alias dragFactor for dragCoefficient;
    /* ISO-80000-4 item 4-24 dynamic viscosity, viscosity */
    attribute def DynamicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-24 dynamic viscosity, viscosity
         * symbol(s): `η`
         * application domain: generic
         * name: DynamicViscosity
         * quantity dimension: L^-1*M^1*T^-1
         * measurement unit(s): Pa*s, kg*m^-1*s^-1
         * tensor order: 0
         * definition: for laminar flows, proportionality constant between shear stress `τ_(xz)` (item 4-16.2) in a fluid moving with a velocity `v_x` (ISO 80000-3) and gradient `(d v_x)/dz` perpendicular to the plane of shear: `τ_(xz) = η (d v_x)/(dz)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DynamicViscosityUnit[1];
    }
    attribute dynamicViscosity : DynamicViscosityValue[*] nonunique :> scalarQuantities;
    attribute def DynamicViscosityUnit :> DerivedUnit {
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
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    alias ViscosityUnit for DynamicViscosityUnit;
    alias ViscosityValue for DynamicViscosityValue;
    alias viscosity for dynamicViscosity;
    /* ISO-80000-4 item 4-25 kinematic viscosity */
    attribute def KinematicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-25 kinematic viscosity
         * symbol(s): `v`
         * application domain: generic
         * name: KinematicViscosity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of dynamic viscosity `η` (item 4-24) and mass density `ρ` (item 4-2) of a fluid: `v = η/ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : KinematicViscosityUnit[1];
    }
    attribute kinematicViscosity : KinematicViscosityValue[*] nonunique :> scalarQuantities;
    attribute def KinematicViscosityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-26 surface tension */
    attribute def SurfaceTensionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-26 surface tension
         * symbol(s): `γ`, `σ`
         * application domain: generic
         * name: SurfaceTension
         * quantity dimension: M^1*T^-2
         * measurement unit(s): N*m^-1, kg*s^-2
         * tensor order: 0
         * definition: magnitude of a force acting against the enlargement of area portion of a surface separating a liquid from its surrounding
         * remarks: The concept of surface energy is closely related to surface tension and has the same dimension.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceTensionUnit[1];
    }
    attribute surfaceTension : SurfaceTensionValue[*] nonunique :> scalarQuantities;
    attribute def SurfaceTensionUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-27.1 power */
    attribute def PowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-27.1 power
         * symbol(s): `P`
         * application domain: generic
         * name: Power
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: quotient of energy (ISO 80000-5) and duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PowerUnit[1];
    }
    attribute power : PowerValue[*] nonunique :> scalarQuantities;
    attribute def PowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-27 mechanical power */
    attribute mechanicalPower : PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 4-27 mechanical power
         * symbol(s): `P`
         * application domain: mechanics
         * name: MechanicalPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, N*m*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: scalar product of force `vec(F)` (item 4-9.1) acting to a body and its velocity `vec(v)` (ISO 80000-3): `P = vec(F) * vec(v)`
         * remarks: None.
         */
    }
    /* ISO-80000-4 item 4-28.1 potential energy */
    attribute potentialEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.1 potential energy
         * symbol(s): `V`, `E_p`
         * application domain: generic
         * name: PotentialEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: for conservative force `vec(F)`, scalar additive quantity obeying condition `vec(F) = -nabla F`, if it exists
         * remarks: For the definition of energy, see ISO 80000-5. A force is conservative when the force field is irrotational, i.e. `rot(F) = 0` , or `vec(F)` is perpendicular to the speed of the body to ensure `vec(F) * d vec(r) = 0` .
         */
    }
    /* ISO-80000-4 item 4-28.2 kinetic energy */
    attribute kineticEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.2 kinetic energy
         * symbol(s): `T`, `E_k`
         * application domain: generic
         * name: KineticEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing a moving body expressed by: `T = 1/2 m v^2` where `m` is mass (item 4-1) of the body and `v` is its speed (ISO 80000-3)
         * remarks: For the definition of energy, see ISO 80000-5.
         */
    }
    /* ISO-80000-4 item 4-28.3 mechanical energy */
    attribute mechanicalEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.3 mechanical energy
         * symbol(s): `E`, `W`
         * application domain: generic
         * name: MechanicalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of kinetic energy `T` (item 4-28.2) and potential energy `V` (item 4-28.1): `E = T+V`
         * remarks: The symbols `E` and `W` are also used for other kinds of energy. This definition is understood in a classical way and it does not include thermal motion.
         */
    }
    /* ISO-80000-4 item 4-28.4 mechanical work, work */
    attribute mechanicalWork : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.4 mechanical work, work
         * symbol(s): `A`, `W`
         * application domain: generic
         * name: MechanicalWork (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: process quantity describing the total action of a force `vec(F)` (item 4-9.1) along a continuous curve `Γ` in three-dimensional space with infinitesimal displacement (ISO 80000-3) `dvec(r)`, as a line integral of their scalar product: `A = int_Γ vec(F) * d vec(r)`
         * remarks: The definition covers the case `A = -int_Γ p*dV` where `Γ` is a curve in the phase space and implies that work generally depends upon `Γ`, and that type of process must be defined (e.g. isentropic or isothermic).
         */
    }
    alias work for mechanicalWork;
    /* ISO-80000-4 item 4-29 mechanical efficiency */
    attribute def MechanicalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-29 mechanical efficiency
         * symbol(s): `η`
         * application domain: mechanics
         * name: MechanicalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of output power `P_"out"` (item 4-27) from a system and input power `P_"in"` (item 4-27) to this system: `η = P_"out"/P_"in"`
         * remarks: The system must be specified. This quantity is often expressed by the unit percent, symbol %.
         */
    }
    attribute mechanicalEfficiency : MechanicalEfficiencyValue :> scalarQuantities;
    /* ISO-80000-4 item 4-30.1 mass flow */
    attribute def MassFlowValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.1 mass flow (magnitude)
         * symbol(s): `j_m`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassFlowUnit[1];
    }
    attribute massFlow : MassFlowValue[*] nonunique :> scalarQuantities;
    attribute def MassFlowUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CartesianMassFlow3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-30.1 mass flow (vector)
         * symbol(s): `vec(j_m)`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMassFlow3dCoordinateFrame[1];
    }
    attribute cartesianMassFlow3dVector : CartesianMassFlow3dVector :> vectorQuantities;
    attribute def CartesianMassFlow3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MassFlowUnit[3];
    }
    /* ISO-80000-4 item 4-30.2 mass flow rate */
    attribute def MassFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.2 mass flow rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassFlowRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with normal vector `vec(e)_n` of a flowing fluid with mass flow `vec(j)_m` (item 4-30.1) as an integral: `q_m = int int_A vec(j)_m * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassFlowRateUnit[1];
    }
    attribute massFlowRate : MassFlowRateValue[*] nonunique :> scalarQuantities;
    attribute def MassFlowRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-30.3 mass change rate */
    attribute def MassChangeRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.3 mass change rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassChangeRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: rate of increment of mass `m` (item 4-1): `q_m = (dm)/(dt)` where `dm` is the infinitesimal mass (item 4-1) increment and `dt` is the infinitesimal duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassChangeRateUnit[1];
    }
    attribute massChangeRate : MassChangeRateValue[*] nonunique :> scalarQuantities;
    attribute def MassChangeRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-31 volume flow rate */
    attribute def VolumeFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-31 volume flow rate
         * symbol(s): `q_v`
         * application domain: generic
         * name: VolumeFlowRate
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with the normal vector `vec(e)_n` of a flowing fluid with velocity `vec(v)` (ISO 80000-3) as an integral: `q_v = int int_A vec(v) * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeFlowRateUnit[1];
    }
    attribute volumeFlowRate : VolumeFlowRateValue[*] nonunique :> scalarQuantities;
    attribute def VolumeFlowRateUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-4 item 4-32 action quantity */
    attribute def ActionQuantityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-32 action quantity
         * symbol(s): `S`
         * application domain: generic
         * name: ActionQuantity
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: time integral of energy `E` over a time interval `(t_1, t_2)`: `S = int_(t_1)^(t_2) E dt`
         * remarks: The energy may be expressed by a Lagrangian or Hamiltonian function. Note for SysML: the ISQ quantity "action" has been renamed to "action quantity" to avoid the name clash with the SysML action keyword.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ActionQuantityUnit[1];
    }
    attribute actionQuantity : ActionQuantityValue[*] nonunique :> scalarQuantities;
    attribute def ActionQuantityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 779) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 779) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 793) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 818) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 818) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 852) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 852) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 897) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 897) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 995) (line 21) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 995) (line 21) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1014) (line 21) (column 39) (len 11)))))
    (reference r5 (scope relative) (span (offset 1233) (line 27) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1233) (line 27) (column 39) (len 19)))))
    (reference r6 (scope relative) (span (offset 1854) (line 40) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1854) (line 40) (column 28) (len 4)))))
    (reference r7 (scope relative) (span (offset 1849) (line 40) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 1849) (line 40) (column 23) (len 3)))))
    (reference r8 (scope relative) (span (offset 1888) (line 41) (column 29) (len 15)) (segments (segment 0 (token "MassDensityUnit") (name "MassDensityUnit") (separator none) (span (offset 1888) (line 41) (column 29) (len 15)))))
    (reference r9 (scope relative) (span (offset 1882) (line 41) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1882) (line 41) (column 23) (len 4)))))
    (reference r10 (scope relative) (span (offset 2031) (line 46) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 2031) (line 46) (column 38) (len 11)))))
    (reference r11 (scope relative) (span (offset 2081) (line 47) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2081) (line 47) (column 37) (len 19)))))
    (reference r12 (scope relative) (span (offset 2110) (line 47) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2110) (line 47) (column 66) (len 8)))))
    (reference r13 (scope relative) (span (offset 2121) (line 47) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2121) (line 47) (column 77) (len 3)))))
    (reference r14 (scope relative) (span (offset 2125) (line 47) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 2125) (line 47) (column 81) (len 1)))))
    (reference r15 (scope relative) (span (offset 2132) (line 47) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2132) (line 47) (column 88) (len 8)))))
    (reference r16 (scope relative) (span (offset 2183) (line 48) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2183) (line 48) (column 35) (len 19)))))
    (reference r17 (scope relative) (span (offset 2212) (line 48) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2212) (line 48) (column 64) (len 8)))))
    (reference r18 (scope relative) (span (offset 2223) (line 48) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2223) (line 48) (column 75) (len 3)))))
    (reference r19 (scope relative) (span (offset 2227) (line 48) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 2227) (line 48) (column 79) (len 1)))))
    (reference r20 (scope relative) (span (offset 2234) (line 48) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2234) (line 48) (column 86) (len 8)))))
    (reference r21 (scope relative) (span (offset 2272) (line 49) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 2272) (line 49) (column 23) (len 17)))))
    (reference r22 (scope relative) (span (offset 2296) (line 49) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 2296) (line 49) (column 47) (len 20)))))
    (reference r23 (scope relative) (span (offset 2320) (line 49) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 2320) (line 49) (column 71) (len 8)))))
    (reference r24 (scope relative) (span (offset 2330) (line 49) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 2330) (line 49) (column 81) (len 6)))))
    (reference r25 (scope relative) (span (offset 2374) (line 52) (column 27) (len 15)) (segments (segment 0 (token "MassDensityUnit") (name "MassDensityUnit") (separator none) (span (offset 2374) (line 52) (column 27) (len 15)))))
    (reference r26 (scope relative) (span (offset 2418) (line 53) (column 28) (len 16)) (segments (segment 0 (token "MassDensityValue") (name "MassDensityValue") (separator none) (span (offset 2418) (line 53) (column 28) (len 16)))))
    (reference r27 (scope relative) (span (offset 2458) (line 54) (column 23) (len 11)) (segments (segment 0 (token "massDensity") (name "massDensity") (separator none) (span (offset 2458) (line 54) (column 23) (len 11)))))
    (reference r28 (scope relative) (span (offset 2560) (line 57) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 2560) (line 57) (column 42) (len 19)))))
    (reference r29 (scope relative) (span (offset 2998) (line 70) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 2998) (line 70) (column 28) (len 4)))))
    (reference r30 (scope relative) (span (offset 2993) (line 70) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 2993) (line 70) (column 23) (len 3)))))
    (reference r31 (scope relative) (span (offset 3032) (line 71) (column 29) (len 18)) (segments (segment 0 (token "SpecificVolumeUnit") (name "SpecificVolumeUnit") (separator none) (span (offset 3032) (line 71) (column 29) (len 18)))))
    (reference r32 (scope relative) (span (offset 3026) (line 71) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 3026) (line 71) (column 23) (len 4)))))
    (reference r33 (scope relative) (span (offset 3187) (line 76) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 3187) (line 76) (column 41) (len 11)))))
    (reference r34 (scope relative) (span (offset 3237) (line 77) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 3237) (line 77) (column 37) (len 19)))))
    (reference r35 (scope relative) (span (offset 3266) (line 77) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 3266) (line 77) (column 66) (len 8)))))
    (reference r36 (scope relative) (span (offset 3277) (line 77) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 3277) (line 77) (column 77) (len 3)))))
    (reference r37 (scope relative) (span (offset 3281) (line 77) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 3281) (line 77) (column 81) (len 1)))))
    (reference r38 (scope relative) (span (offset 3288) (line 77) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 3288) (line 77) (column 88) (len 8)))))
    (reference r39 (scope relative) (span (offset 3338) (line 78) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 3338) (line 78) (column 35) (len 19)))))
    (reference r40 (scope relative) (span (offset 3367) (line 78) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 3367) (line 78) (column 64) (len 8)))))
    (reference r41 (scope relative) (span (offset 3378) (line 78) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 3378) (line 78) (column 75) (len 3)))))
    (reference r42 (scope relative) (span (offset 3382) (line 78) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 3382) (line 78) (column 79) (len 1)))))
    (reference r43 (scope relative) (span (offset 3389) (line 78) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 3389) (line 78) (column 86) (len 8)))))
    (reference r44 (scope relative) (span (offset 3428) (line 79) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 3428) (line 79) (column 23) (len 17)))))
    (reference r45 (scope relative) (span (offset 3452) (line 79) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 3452) (line 79) (column 47) (len 20)))))
    (reference r46 (scope relative) (span (offset 3476) (line 79) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 3476) (line 79) (column 71) (len 8)))))
    (reference r47 (scope relative) (span (offset 3486) (line 79) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 3486) (line 79) (column 81) (len 6)))))
    (reference r48 (scope relative) (span (offset 3621) (line 83) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 3621) (line 83) (column 47) (len 17)))))
    (reference r49 (scope relative) (span (offset 4319) (line 99) (column 31) (len 19)) (segments (segment 0 (token "relativeMassDensity") (name "relativeMassDensity") (separator none) (span (offset 4319) (line 99) (column 31) (len 19)))))
    (reference r50 (scope relative) (span (offset 4455) (line 102) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 4455) (line 102) (column 46) (len 19)))))
    (reference r51 (scope relative) (span (offset 5098) (line 115) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 5098) (line 115) (column 28) (len 4)))))
    (reference r52 (scope relative) (span (offset 5093) (line 115) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 5093) (line 115) (column 23) (len 3)))))
    (reference r53 (scope relative) (span (offset 5132) (line 116) (column 29) (len 22)) (segments (segment 0 (token "SurfaceMassDensityUnit") (name "SurfaceMassDensityUnit") (separator none) (span (offset 5132) (line 116) (column 29) (len 22)))))
    (reference r54 (scope relative) (span (offset 5126) (line 116) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5126) (line 116) (column 23) (len 4)))))
    (reference r55 (scope relative) (span (offset 5303) (line 121) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 5303) (line 121) (column 45) (len 11)))))
    (reference r56 (scope relative) (span (offset 5353) (line 122) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5353) (line 122) (column 37) (len 19)))))
    (reference r57 (scope relative) (span (offset 5382) (line 122) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5382) (line 122) (column 66) (len 8)))))
    (reference r58 (scope relative) (span (offset 5393) (line 122) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5393) (line 122) (column 77) (len 3)))))
    (reference r59 (scope relative) (span (offset 5397) (line 122) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 5397) (line 122) (column 81) (len 1)))))
    (reference r60 (scope relative) (span (offset 5404) (line 122) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5404) (line 122) (column 88) (len 8)))))
    (reference r61 (scope relative) (span (offset 5455) (line 123) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5455) (line 123) (column 35) (len 19)))))
    (reference r62 (scope relative) (span (offset 5484) (line 123) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5484) (line 123) (column 64) (len 8)))))
    (reference r63 (scope relative) (span (offset 5495) (line 123) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5495) (line 123) (column 75) (len 3)))))
    (reference r64 (scope relative) (span (offset 5499) (line 123) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 5499) (line 123) (column 79) (len 1)))))
    (reference r65 (scope relative) (span (offset 5506) (line 123) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5506) (line 123) (column 86) (len 8)))))
    (reference r66 (scope relative) (span (offset 5544) (line 124) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 5544) (line 124) (column 23) (len 17)))))
    (reference r67 (scope relative) (span (offset 5568) (line 124) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 5568) (line 124) (column 47) (len 20)))))
    (reference r68 (scope relative) (span (offset 5592) (line 124) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 5592) (line 124) (column 71) (len 8)))))
    (reference r69 (scope relative) (span (offset 5602) (line 124) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 5602) (line 124) (column 81) (len 6)))))
    (reference r70 (scope relative) (span (offset 5653) (line 127) (column 34) (len 22)) (segments (segment 0 (token "SurfaceMassDensityUnit") (name "SurfaceMassDensityUnit") (separator none) (span (offset 5653) (line 127) (column 34) (len 22)))))
    (reference r71 (scope relative) (span (offset 5711) (line 128) (column 35) (len 23)) (segments (segment 0 (token "SurfaceMassDensityValue") (name "SurfaceMassDensityValue") (separator none) (span (offset 5711) (line 128) (column 35) (len 23)))))
    (reference r72 (scope relative) (span (offset 5765) (line 129) (column 30) (len 18)) (segments (segment 0 (token "surfaceMassDensity") (name "surfaceMassDensity") (separator none) (span (offset 5765) (line 129) (column 30) (len 18)))))
    (reference r73 (scope relative) (span (offset 5897) (line 132) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 5897) (line 132) (column 45) (len 19)))))
    (reference r74 (scope relative) (span (offset 6488) (line 145) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 6488) (line 145) (column 28) (len 4)))))
    (reference r75 (scope relative) (span (offset 6483) (line 145) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 6483) (line 145) (column 23) (len 3)))))
    (reference r76 (scope relative) (span (offset 6522) (line 146) (column 29) (len 21)) (segments (segment 0 (token "LinearMassDensityUnit") (name "LinearMassDensityUnit") (separator none) (span (offset 6522) (line 146) (column 29) (len 21)))))
    (reference r77 (scope relative) (span (offset 6516) (line 146) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 6516) (line 146) (column 23) (len 4)))))
    (reference r78 (scope relative) (span (offset 6689) (line 151) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 6689) (line 151) (column 44) (len 11)))))
    (reference r79 (scope relative) (span (offset 6739) (line 152) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6739) (line 152) (column 37) (len 19)))))
    (reference r80 (scope relative) (span (offset 6768) (line 152) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6768) (line 152) (column 66) (len 8)))))
    (reference r81 (scope relative) (span (offset 6779) (line 152) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6779) (line 152) (column 77) (len 3)))))
    (reference r82 (scope relative) (span (offset 6783) (line 152) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 6783) (line 152) (column 81) (len 1)))))
    (reference r83 (scope relative) (span (offset 6790) (line 152) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6790) (line 152) (column 88) (len 8)))))
    (reference r84 (scope relative) (span (offset 6841) (line 153) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6841) (line 153) (column 35) (len 19)))))
    (reference r85 (scope relative) (span (offset 6870) (line 153) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6870) (line 153) (column 64) (len 8)))))
    (reference r86 (scope relative) (span (offset 6881) (line 153) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6881) (line 153) (column 75) (len 3)))))
    (reference r87 (scope relative) (span (offset 6885) (line 153) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 6885) (line 153) (column 79) (len 1)))))
    (reference r88 (scope relative) (span (offset 6892) (line 153) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6892) (line 153) (column 86) (len 8)))))
    (reference r89 (scope relative) (span (offset 6930) (line 154) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 6930) (line 154) (column 23) (len 17)))))
    (reference r90 (scope relative) (span (offset 6954) (line 154) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 6954) (line 154) (column 47) (len 20)))))
    (reference r91 (scope relative) (span (offset 6978) (line 154) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 6978) (line 154) (column 71) (len 8)))))
    (reference r92 (scope relative) (span (offset 6988) (line 154) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 6988) (line 154) (column 81) (len 6)))))
    (reference r93 (scope relative) (span (offset 7038) (line 157) (column 33) (len 21)) (segments (segment 0 (token "LinearMassDensityUnit") (name "LinearMassDensityUnit") (separator none) (span (offset 7038) (line 157) (column 33) (len 21)))))
    (reference r94 (scope relative) (span (offset 7094) (line 158) (column 34) (len 22)) (segments (segment 0 (token "LinearMassDensityValue") (name "LinearMassDensityValue") (separator none) (span (offset 7094) (line 158) (column 34) (len 22)))))
    (reference r95 (scope relative) (span (offset 7146) (line 159) (column 29) (len 17)) (segments (segment 0 (token "linearMassDensity") (name "linearMassDensity") (separator none) (span (offset 7146) (line 159) (column 29) (len 17)))))
    (reference r96 (scope relative) (span (offset 7257) (line 162) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 7257) (line 162) (column 43) (len 19)))))
    (reference r97 (scope relative) (span (offset 8023) (line 175) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 8023) (line 175) (column 28) (len 4)))))
    (reference r98 (scope relative) (span (offset 8018) (line 175) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 8018) (line 175) (column 23) (len 3)))))
    (reference r99 (scope relative) (span (offset 8057) (line 176) (column 29) (len 19)) (segments (segment 0 (token "MomentOfInertiaUnit") (name "MomentOfInertiaUnit") (separator none) (span (offset 8057) (line 176) (column 29) (len 19)))))
    (reference r100 (scope relative) (span (offset 8051) (line 176) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 8051) (line 176) (column 23) (len 4)))))
    (reference r101 (scope relative) (span (offset 8216) (line 181) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 8216) (line 181) (column 42) (len 11)))))
    (reference r102 (scope relative) (span (offset 8266) (line 182) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8266) (line 182) (column 37) (len 19)))))
    (reference r103 (scope relative) (span (offset 8295) (line 182) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8295) (line 182) (column 66) (len 8)))))
    (reference r104 (scope relative) (span (offset 8306) (line 182) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8306) (line 182) (column 77) (len 3)))))
    (reference r105 (scope relative) (span (offset 8310) (line 182) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 8310) (line 182) (column 81) (len 1)))))
    (reference r106 (scope relative) (span (offset 8317) (line 182) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8317) (line 182) (column 88) (len 8)))))
    (reference r107 (scope relative) (span (offset 8367) (line 183) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8367) (line 183) (column 35) (len 19)))))
    (reference r108 (scope relative) (span (offset 8396) (line 183) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8396) (line 183) (column 64) (len 8)))))
    (reference r109 (scope relative) (span (offset 8407) (line 183) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8407) (line 183) (column 75) (len 3)))))
    (reference r110 (scope relative) (span (offset 8411) (line 183) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 8411) (line 183) (column 79) (len 1)))))
    (reference r111 (scope relative) (span (offset 8418) (line 183) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8418) (line 183) (column 86) (len 8)))))
    (reference r112 (scope relative) (span (offset 8456) (line 184) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 8456) (line 184) (column 23) (len 17)))))
    (reference r113 (scope relative) (span (offset 8480) (line 184) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 8480) (line 184) (column 47) (len 20)))))
    (reference r114 (scope relative) (span (offset 8504) (line 184) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 8504) (line 184) (column 71) (len 8)))))
    (reference r115 (scope relative) (span (offset 8514) (line 184) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 8514) (line 184) (column 81) (len 6)))))
    (reference r116 (scope relative) (span (offset 8586) (line 187) (column 55) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 8586) (line 187) (column 55) (len 19)))))
    (reference r117 (scope relative) (span (offset 9354) (line 200) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 9354) (line 200) (column 23) (len 7)))))
    (reference r118 (scope relative) (span (offset 9398) (line 201) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 9398) (line 201) (column 28) (len 4)))))
    (reference r119 (scope relative) (span (offset 9393) (line 201) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 9393) (line 201) (column 23) (len 3)))))
    (reference r120 (scope relative) (span (offset 9435) (line 202) (column 29) (len 46)) (segments (segment 0 (token "Cartesian3dMomentOfInertiaMeasurementReference") (name "Cartesian3dMomentOfInertiaMeasurementReference") (separator none) (span (offset 9435) (line 202) (column 29) (len 46)))))
    (reference r121 (scope relative) (span (offset 9429) (line 202) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 9429) (line 202) (column 23) (len 4)))))
    (reference r122 (scope relative) (span (offset 9653) (line 207) (column 69) (len 26)) (segments (segment 0 (token "TensorMeasurementReference") (name "TensorMeasurementReference") (separator none) (span (offset 9653) (line 207) (column 69) (len 26)))))
    (reference r123 (scope relative) (span (offset 9704) (line 208) (column 23) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 9704) (line 208) (column 23) (len 10)))))
    (reference r124 (scope relative) (span (offset 9747) (line 209) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 9747) (line 209) (column 23) (len 7)))))
    (reference r125 (scope relative) (span (offset 9793) (line 210) (column 30) (len 19)) (segments (segment 0 (token "MomentOfInertiaUnit") (name "MomentOfInertiaUnit") (separator none) (span (offset 9793) (line 210) (column 30) (len 19)))))
    (reference r126 (scope relative) (span (offset 9786) (line 210) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 9786) (line 210) (column 23) (len 5)))))
    (reference r127 (scope relative) (span (offset 9899) (line 214) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 9899) (line 214) (column 36) (len 19)))))
    (reference r128 (scope relative) (span (offset 10406) (line 227) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10406) (line 227) (column 28) (len 4)))))
    (reference r129 (scope relative) (span (offset 10401) (line 227) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 10401) (line 227) (column 23) (len 3)))))
    (reference r130 (scope relative) (span (offset 10440) (line 228) (column 29) (len 12)) (segments (segment 0 (token "MomentumUnit") (name "MomentumUnit") (separator none) (span (offset 10440) (line 228) (column 29) (len 12)))))
    (reference r131 (scope relative) (span (offset 10434) (line 228) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 10434) (line 228) (column 23) (len 4)))))
    (reference r132 (scope relative) (span (offset 10571) (line 233) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 10571) (line 233) (column 35) (len 11)))))
    (reference r133 (scope relative) (span (offset 10621) (line 234) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10621) (line 234) (column 37) (len 19)))))
    (reference r134 (scope relative) (span (offset 10650) (line 234) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10650) (line 234) (column 66) (len 8)))))
    (reference r135 (scope relative) (span (offset 10661) (line 234) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10661) (line 234) (column 77) (len 3)))))
    (reference r136 (scope relative) (span (offset 10665) (line 234) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 10665) (line 234) (column 81) (len 1)))))
    (reference r137 (scope relative) (span (offset 10672) (line 234) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10672) (line 234) (column 88) (len 8)))))
    (reference r138 (scope relative) (span (offset 10722) (line 235) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10722) (line 235) (column 35) (len 19)))))
    (reference r139 (scope relative) (span (offset 10751) (line 235) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10751) (line 235) (column 64) (len 8)))))
    (reference r140 (scope relative) (span (offset 10762) (line 235) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10762) (line 235) (column 75) (len 3)))))
    (reference r141 (scope relative) (span (offset 10766) (line 235) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 10766) (line 235) (column 79) (len 1)))))
    (reference r142 (scope relative) (span (offset 10773) (line 235) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10773) (line 235) (column 86) (len 8)))))
    (reference r143 (scope relative) (span (offset 10827) (line 236) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10827) (line 236) (column 39) (len 19)))))
    (reference r144 (scope relative) (span (offset 10856) (line 236) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10856) (line 236) (column 68) (len 8)))))
    (reference r145 (scope relative) (span (offset 10867) (line 236) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10867) (line 236) (column 79) (len 3)))))
    (reference r146 (scope relative) (span (offset 10871) (line 236) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 10871) (line 236) (column 83) (len 1)))))
    (reference r147 (scope relative) (span (offset 10878) (line 236) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10878) (line 236) (column 90) (len 8)))))
    (reference r148 (scope relative) (span (offset 10917) (line 237) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 10917) (line 237) (column 23) (len 17)))))
    (reference r149 (scope relative) (span (offset 10941) (line 237) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 10941) (line 237) (column 47) (len 20)))))
    (reference r150 (scope relative) (span (offset 10965) (line 237) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 10965) (line 237) (column 71) (len 8)))))
    (reference r151 (scope relative) (span (offset 10975) (line 237) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 10975) (line 237) (column 81) (len 6)))))
    (reference r152 (scope relative) (span (offset 10983) (line 237) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 10983) (line 237) (column 89) (len 10)))))
    (reference r153 (scope relative) (span (offset 11052) (line 240) (column 48) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 11052) (line 240) (column 48) (len 23)))))
    (reference r154 (scope relative) (span (offset 11560) (line 253) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 11560) (line 253) (column 23) (len 7)))))
    (reference r155 (scope relative) (span (offset 11605) (line 254) (column 29) (len 34)) (segments (segment 0 (token "CartesianMomentum3dCoordinateFrame") (name "CartesianMomentum3dCoordinateFrame") (separator none) (span (offset 11605) (line 254) (column 29) (len 34)))))
    (reference r156 (scope relative) (span (offset 11599) (line 254) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 11599) (line 254) (column 23) (len 4)))))
    (reference r157 (scope relative) (span (offset 11796) (line 259) (column 57) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 11796) (line 259) (column 57) (len 19)))))
    (reference r158 (scope relative) (span (offset 11840) (line 260) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 11840) (line 260) (column 23) (len 7)))))
    (reference r159 (scope relative) (span (offset 11879) (line 261) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 11879) (line 261) (column 23) (len 12)))))
    (reference r160 (scope relative) (span (offset 11929) (line 262) (column 30) (len 12)) (segments (segment 0 (token "MomentumUnit") (name "MomentumUnit") (separator none) (span (offset 11929) (line 262) (column 30) (len 12)))))
    (reference r161 (scope relative) (span (offset 11922) (line 262) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 11922) (line 262) (column 23) (len 5)))))
    (reference r162 (scope relative) (span (offset 12024) (line 266) (column 33) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 12024) (line 266) (column 33) (len 19)))))
    (reference r163 (scope relative) (span (offset 12490) (line 279) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 12490) (line 279) (column 28) (len 4)))))
    (reference r164 (scope relative) (span (offset 12485) (line 279) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 12485) (line 279) (column 23) (len 3)))))
    (reference r165 (scope relative) (span (offset 12524) (line 280) (column 29) (len 9)) (segments (segment 0 (token "ForceUnit") (name "ForceUnit") (separator none) (span (offset 12524) (line 280) (column 29) (len 9)))))
    (reference r166 (scope relative) (span (offset 12518) (line 280) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 12518) (line 280) (column 23) (len 4)))))
    (reference r167 (scope relative) (span (offset 12643) (line 285) (column 32) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 12643) (line 285) (column 32) (len 11)))))
    (reference r168 (scope relative) (span (offset 12693) (line 286) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12693) (line 286) (column 37) (len 19)))))
    (reference r169 (scope relative) (span (offset 12722) (line 286) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12722) (line 286) (column 66) (len 8)))))
    (reference r170 (scope relative) (span (offset 12733) (line 286) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12733) (line 286) (column 77) (len 3)))))
    (reference r171 (scope relative) (span (offset 12737) (line 286) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 12737) (line 286) (column 81) (len 1)))))
    (reference r172 (scope relative) (span (offset 12744) (line 286) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12744) (line 286) (column 88) (len 8)))))
    (reference r173 (scope relative) (span (offset 12794) (line 287) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12794) (line 287) (column 35) (len 19)))))
    (reference r174 (scope relative) (span (offset 12823) (line 287) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12823) (line 287) (column 64) (len 8)))))
    (reference r175 (scope relative) (span (offset 12834) (line 287) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12834) (line 287) (column 75) (len 3)))))
    (reference r176 (scope relative) (span (offset 12838) (line 287) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 12838) (line 287) (column 79) (len 1)))))
    (reference r177 (scope relative) (span (offset 12845) (line 287) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12845) (line 287) (column 86) (len 8)))))
    (reference r178 (scope relative) (span (offset 12899) (line 288) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12899) (line 288) (column 39) (len 19)))))
    (reference r179 (scope relative) (span (offset 12928) (line 288) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12928) (line 288) (column 68) (len 8)))))
    (reference r180 (scope relative) (span (offset 12939) (line 288) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12939) (line 288) (column 79) (len 3)))))
    (reference r181 (scope relative) (span (offset 12943) (line 288) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 12943) (line 288) (column 83) (len 1)))))
    (reference r182 (scope relative) (span (offset 12950) (line 288) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12950) (line 288) (column 90) (len 8)))))
    (reference r183 (scope relative) (span (offset 12989) (line 289) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 12989) (line 289) (column 23) (len 17)))))
    (reference r184 (scope relative) (span (offset 13013) (line 289) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 13013) (line 289) (column 47) (len 20)))))
    (reference r185 (scope relative) (span (offset 13037) (line 289) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 13037) (line 289) (column 71) (len 8)))))
    (reference r186 (scope relative) (span (offset 13047) (line 289) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 13047) (line 289) (column 81) (len 6)))))
    (reference r187 (scope relative) (span (offset 13055) (line 289) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 13055) (line 289) (column 89) (len 10)))))
    (reference r188 (scope relative) (span (offset 13121) (line 292) (column 45) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 13121) (line 292) (column 45) (len 23)))))
    (reference r189 (scope relative) (span (offset 13588) (line 305) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 13588) (line 305) (column 23) (len 7)))))
    (reference r190 (scope relative) (span (offset 13633) (line 306) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 13633) (line 306) (column 29) (len 31)))))
    (reference r191 (scope relative) (span (offset 13627) (line 306) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13627) (line 306) (column 23) (len 4)))))
    (reference r192 (scope relative) (span (offset 13812) (line 311) (column 54) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 13812) (line 311) (column 54) (len 19)))))
    (reference r193 (scope relative) (span (offset 13856) (line 312) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 13856) (line 312) (column 23) (len 7)))))
    (reference r194 (scope relative) (span (offset 13895) (line 313) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 13895) (line 313) (column 23) (len 12)))))
    (reference r195 (scope relative) (span (offset 13945) (line 314) (column 30) (len 9)) (segments (segment 0 (token "ForceUnit") (name "ForceUnit") (separator none) (span (offset 13945) (line 314) (column 30) (len 9)))))
    (reference r196 (scope relative) (span (offset 13938) (line 314) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 13938) (line 314) (column 23) (len 5)))))
    (reference r197 (scope relative) (span (offset 14051) (line 318) (column 46) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 14051) (line 318) (column 46) (len 23)))))
    (reference r198 (scope relative) (span (offset 14945) (line 331) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 14945) (line 331) (column 23) (len 7)))))
    (reference r199 (scope relative) (span (offset 14990) (line 332) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 14990) (line 332) (column 29) (len 31)))))
    (reference r200 (scope relative) (span (offset 14984) (line 332) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 14984) (line 332) (column 23) (len 4)))))
    (reference r201 (scope relative) (span (offset 15248) (line 338) (column 59) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 15248) (line 338) (column 59) (len 23)))))
    (reference r202 (scope relative) (span (offset 15825) (line 351) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 15825) (line 351) (column 23) (len 7)))))
    (reference r203 (scope relative) (span (offset 15870) (line 352) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 15870) (line 352) (column 29) (len 31)))))
    (reference r204 (scope relative) (span (offset 15864) (line 352) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15864) (line 352) (column 23) (len 4)))))
    (reference r205 (scope relative) (span (offset 16070) (line 357) (column 47) (len 36)) (segments (segment 0 (token "cartesianStaticFrictionForce3dVector") (name "cartesianStaticFrictionForce3dVector") (separator none) (span (offset 16070) (line 357) (column 47) (len 36)))))
    (reference r206 (scope relative) (span (offset 16248) (line 360) (column 60) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 16248) (line 360) (column 60) (len 23)))))
    (reference r207 (scope relative) (span (offset 16820) (line 373) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 16820) (line 373) (column 23) (len 7)))))
    (reference r208 (scope relative) (span (offset 16865) (line 374) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 16865) (line 374) (column 29) (len 31)))))
    (reference r209 (scope relative) (span (offset 16859) (line 374) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16859) (line 374) (column 23) (len 4)))))
    (reference r210 (scope relative) (span (offset 17073) (line 379) (column 53) (len 37)) (segments (segment 0 (token "cartesianKineticFrictionForce3dVector") (name "cartesianKineticFrictionForce3dVector") (separator none) (span (offset 17073) (line 379) (column 53) (len 37)))))
    (reference r211 (scope relative) (span (offset 17259) (line 382) (column 57) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 17259) (line 382) (column 57) (len 23)))))
    (reference r212 (scope relative) (span (offset 17841) (line 395) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 17841) (line 395) (column 23) (len 7)))))
    (reference r213 (scope relative) (span (offset 17886) (line 396) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 17886) (line 396) (column 29) (len 31)))))
    (reference r214 (scope relative) (span (offset 17880) (line 396) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17880) (line 396) (column 23) (len 4)))))
    (reference r215 (scope relative) (span (offset 18079) (line 401) (column 44) (len 34)) (segments (segment 0 (token "cartesianRollingResistance3dVector") (name "cartesianRollingResistance3dVector") (separator none) (span (offset 18079) (line 401) (column 44) (len 34)))))
    (reference r216 (scope relative) (span (offset 18168) (line 403) (column 53) (len 34)) (segments (segment 0 (token "cartesianRollingResistance3dVector") (name "cartesianRollingResistance3dVector") (separator none) (span (offset 18168) (line 403) (column 53) (len 34)))))
    (reference r217 (scope relative) (span (offset 18297) (line 406) (column 49) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 18297) (line 406) (column 49) (len 23)))))
    (reference r218 (scope relative) (span (offset 18803) (line 419) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 18803) (line 419) (column 23) (len 7)))))
    (reference r219 (scope relative) (span (offset 18848) (line 420) (column 29) (len 31)) (segments (segment 0 (token "CartesianForce3dCoordinateFrame") (name "CartesianForce3dCoordinateFrame") (separator none) (span (offset 18848) (line 420) (column 29) (len 31)))))
    (reference r220 (scope relative) (span (offset 18842) (line 420) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 18842) (line 420) (column 23) (len 4)))))
    (reference r221 (scope relative) (span (offset 19056) (line 426) (column 35) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 19056) (line 426) (column 35) (len 19)))))
    (reference r222 (scope relative) (span (offset 19809) (line 439) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 19809) (line 439) (column 28) (len 4)))))
    (reference r223 (scope relative) (span (offset 19804) (line 439) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19804) (line 439) (column 23) (len 3)))))
    (reference r224 (scope relative) (span (offset 19843) (line 440) (column 29) (len 11)) (segments (segment 0 (token "ImpulseUnit") (name "ImpulseUnit") (separator none) (span (offset 19843) (line 440) (column 29) (len 11)))))
    (reference r225 (scope relative) (span (offset 19837) (line 440) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19837) (line 440) (column 23) (len 4)))))
    (reference r226 (scope relative) (span (offset 19970) (line 445) (column 34) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19970) (line 445) (column 34) (len 11)))))
    (reference r227 (scope relative) (span (offset 20020) (line 446) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 20020) (line 446) (column 37) (len 19)))))
    (reference r228 (scope relative) (span (offset 20049) (line 446) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 20049) (line 446) (column 66) (len 8)))))
    (reference r229 (scope relative) (span (offset 20060) (line 446) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 20060) (line 446) (column 77) (len 3)))))
    (reference r230 (scope relative) (span (offset 20064) (line 446) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 20064) (line 446) (column 81) (len 1)))))
    (reference r231 (scope relative) (span (offset 20071) (line 446) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20071) (line 446) (column 88) (len 8)))))
    (reference r232 (scope relative) (span (offset 20121) (line 447) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 20121) (line 447) (column 35) (len 19)))))
    (reference r233 (scope relative) (span (offset 20150) (line 447) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 20150) (line 447) (column 64) (len 8)))))
    (reference r234 (scope relative) (span (offset 20161) (line 447) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 20161) (line 447) (column 75) (len 3)))))
    (reference r235 (scope relative) (span (offset 20165) (line 447) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 20165) (line 447) (column 79) (len 1)))))
    (reference r236 (scope relative) (span (offset 20172) (line 447) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20172) (line 447) (column 86) (len 8)))))
    (reference r237 (scope relative) (span (offset 20226) (line 448) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 20226) (line 448) (column 39) (len 19)))))
    (reference r238 (scope relative) (span (offset 20255) (line 448) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 20255) (line 448) (column 68) (len 8)))))
    (reference r239 (scope relative) (span (offset 20266) (line 448) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 20266) (line 448) (column 79) (len 3)))))
    (reference r240 (scope relative) (span (offset 20270) (line 448) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 20270) (line 448) (column 83) (len 1)))))
    (reference r241 (scope relative) (span (offset 20277) (line 448) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20277) (line 448) (column 90) (len 8)))))
    (reference r242 (scope relative) (span (offset 20316) (line 449) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 20316) (line 449) (column 23) (len 17)))))
    (reference r243 (scope relative) (span (offset 20340) (line 449) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 20340) (line 449) (column 47) (len 20)))))
    (reference r244 (scope relative) (span (offset 20364) (line 449) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 20364) (line 449) (column 71) (len 8)))))
    (reference r245 (scope relative) (span (offset 20374) (line 449) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 20374) (line 449) (column 81) (len 6)))))
    (reference r246 (scope relative) (span (offset 20382) (line 449) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 20382) (line 449) (column 89) (len 10)))))
    (reference r247 (scope relative) (span (offset 20450) (line 452) (column 47) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 20450) (line 452) (column 47) (len 23)))))
    (reference r248 (scope relative) (span (offset 21204) (line 465) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 21204) (line 465) (column 23) (len 7)))))
    (reference r249 (scope relative) (span (offset 21249) (line 466) (column 29) (len 33)) (segments (segment 0 (token "CartesianImpulse3dCoordinateFrame") (name "CartesianImpulse3dCoordinateFrame") (separator none) (span (offset 21249) (line 466) (column 29) (len 33)))))
    (reference r250 (scope relative) (span (offset 21243) (line 466) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 21243) (line 466) (column 23) (len 4)))))
    (reference r251 (scope relative) (span (offset 21436) (line 471) (column 56) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 21436) (line 471) (column 56) (len 19)))))
    (reference r252 (scope relative) (span (offset 21480) (line 472) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 21480) (line 472) (column 23) (len 7)))))
    (reference r253 (scope relative) (span (offset 21519) (line 473) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 21519) (line 473) (column 23) (len 12)))))
    (reference r254 (scope relative) (span (offset 21569) (line 474) (column 30) (len 11)) (segments (segment 0 (token "ImpulseUnit") (name "ImpulseUnit") (separator none) (span (offset 21569) (line 474) (column 30) (len 11)))))
    (reference r255 (scope relative) (span (offset 21562) (line 474) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 21562) (line 474) (column 23) (len 5)))))
    (reference r256 (scope relative) (span (offset 21683) (line 478) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21683) (line 478) (column 43) (len 19)))))
    (reference r257 (scope relative) (span (offset 22299) (line 491) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 22299) (line 491) (column 28) (len 4)))))
    (reference r258 (scope relative) (span (offset 22294) (line 491) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 22294) (line 491) (column 23) (len 3)))))
    (reference r259 (scope relative) (span (offset 22333) (line 492) (column 29) (len 19)) (segments (segment 0 (token "AngularMomentumUnit") (name "AngularMomentumUnit") (separator none) (span (offset 22333) (line 492) (column 29) (len 19)))))
    (reference r260 (scope relative) (span (offset 22327) (line 492) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22327) (line 492) (column 23) (len 4)))))
    (reference r261 (scope relative) (span (offset 22492) (line 497) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 22492) (line 497) (column 42) (len 11)))))
    (reference r262 (scope relative) (span (offset 22542) (line 498) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22542) (line 498) (column 37) (len 19)))))
    (reference r263 (scope relative) (span (offset 22571) (line 498) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22571) (line 498) (column 66) (len 8)))))
    (reference r264 (scope relative) (span (offset 22582) (line 498) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22582) (line 498) (column 77) (len 3)))))
    (reference r265 (scope relative) (span (offset 22586) (line 498) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 22586) (line 498) (column 81) (len 1)))))
    (reference r266 (scope relative) (span (offset 22593) (line 498) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22593) (line 498) (column 88) (len 8)))))
    (reference r267 (scope relative) (span (offset 22643) (line 499) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22643) (line 499) (column 35) (len 19)))))
    (reference r268 (scope relative) (span (offset 22672) (line 499) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22672) (line 499) (column 64) (len 8)))))
    (reference r269 (scope relative) (span (offset 22683) (line 499) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22683) (line 499) (column 75) (len 3)))))
    (reference r270 (scope relative) (span (offset 22687) (line 499) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 22687) (line 499) (column 79) (len 1)))))
    (reference r271 (scope relative) (span (offset 22694) (line 499) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22694) (line 499) (column 86) (len 8)))))
    (reference r272 (scope relative) (span (offset 22748) (line 500) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22748) (line 500) (column 39) (len 19)))))
    (reference r273 (scope relative) (span (offset 22777) (line 500) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22777) (line 500) (column 68) (len 8)))))
    (reference r274 (scope relative) (span (offset 22788) (line 500) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22788) (line 500) (column 79) (len 3)))))
    (reference r275 (scope relative) (span (offset 22792) (line 500) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 22792) (line 500) (column 83) (len 1)))))
    (reference r276 (scope relative) (span (offset 22799) (line 500) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22799) (line 500) (column 90) (len 8)))))
    (reference r277 (scope relative) (span (offset 22838) (line 501) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 22838) (line 501) (column 23) (len 17)))))
    (reference r278 (scope relative) (span (offset 22862) (line 501) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 22862) (line 501) (column 47) (len 20)))))
    (reference r279 (scope relative) (span (offset 22886) (line 501) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 22886) (line 501) (column 71) (len 8)))))
    (reference r280 (scope relative) (span (offset 22896) (line 501) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 22896) (line 501) (column 81) (len 6)))))
    (reference r281 (scope relative) (span (offset 22904) (line 501) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 22904) (line 501) (column 89) (len 10)))))
    (reference r282 (scope relative) (span (offset 22980) (line 504) (column 55) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 22980) (line 504) (column 55) (len 23)))))
    (reference r283 (scope relative) (span (offset 23597) (line 517) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 23597) (line 517) (column 23) (len 7)))))
    (reference r284 (scope relative) (span (offset 23642) (line 518) (column 29) (len 41)) (segments (segment 0 (token "CartesianAngularMomentum3dCoordinateFrame") (name "CartesianAngularMomentum3dCoordinateFrame") (separator none) (span (offset 23642) (line 518) (column 29) (len 41)))))
    (reference r285 (scope relative) (span (offset 23636) (line 518) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23636) (line 518) (column 23) (len 4)))))
    (reference r286 (scope relative) (span (offset 23861) (line 523) (column 64) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 23861) (line 523) (column 64) (len 19)))))
    (reference r287 (scope relative) (span (offset 23905) (line 524) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 23905) (line 524) (column 23) (len 7)))))
    (reference r288 (scope relative) (span (offset 23944) (line 525) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 23944) (line 525) (column 23) (len 12)))))
    (reference r289 (scope relative) (span (offset 23994) (line 526) (column 30) (len 19)) (segments (segment 0 (token "AngularMomentumUnit") (name "AngularMomentumUnit") (separator none) (span (offset 23994) (line 526) (column 30) (len 19)))))
    (reference r290 (scope relative) (span (offset 23987) (line 526) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 23987) (line 526) (column 23) (len 5)))))
    (reference r291 (scope relative) (span (offset 24115) (line 530) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 24115) (line 530) (column 41) (len 19)))))
    (reference r292 (scope relative) (span (offset 24782) (line 543) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 24782) (line 543) (column 28) (len 4)))))
    (reference r293 (scope relative) (span (offset 24777) (line 543) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 24777) (line 543) (column 23) (len 3)))))
    (reference r294 (scope relative) (span (offset 24816) (line 544) (column 29) (len 17)) (segments (segment 0 (token "MomentOfForceUnit") (name "MomentOfForceUnit") (separator none) (span (offset 24816) (line 544) (column 29) (len 17)))))
    (reference r295 (scope relative) (span (offset 24810) (line 544) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 24810) (line 544) (column 23) (len 4)))))
    (reference r296 (scope relative) (span (offset 24967) (line 549) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 24967) (line 549) (column 40) (len 11)))))
    (reference r297 (scope relative) (span (offset 25017) (line 550) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 25017) (line 550) (column 37) (len 19)))))
    (reference r298 (scope relative) (span (offset 25046) (line 550) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 25046) (line 550) (column 66) (len 8)))))
    (reference r299 (scope relative) (span (offset 25057) (line 550) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 25057) (line 550) (column 77) (len 3)))))
    (reference r300 (scope relative) (span (offset 25061) (line 550) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 25061) (line 550) (column 81) (len 1)))))
    (reference r301 (scope relative) (span (offset 25068) (line 550) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 25068) (line 550) (column 88) (len 8)))))
    (reference r302 (scope relative) (span (offset 25118) (line 551) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 25118) (line 551) (column 35) (len 19)))))
    (reference r303 (scope relative) (span (offset 25147) (line 551) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 25147) (line 551) (column 64) (len 8)))))
    (reference r304 (scope relative) (span (offset 25158) (line 551) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 25158) (line 551) (column 75) (len 3)))))
    (reference r305 (scope relative) (span (offset 25162) (line 551) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 25162) (line 551) (column 79) (len 1)))))
    (reference r306 (scope relative) (span (offset 25169) (line 551) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 25169) (line 551) (column 86) (len 8)))))
    (reference r307 (scope relative) (span (offset 25223) (line 552) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 25223) (line 552) (column 39) (len 19)))))
    (reference r308 (scope relative) (span (offset 25252) (line 552) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 25252) (line 552) (column 68) (len 8)))))
    (reference r309 (scope relative) (span (offset 25263) (line 552) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 25263) (line 552) (column 79) (len 3)))))
    (reference r310 (scope relative) (span (offset 25267) (line 552) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 25267) (line 552) (column 83) (len 1)))))
    (reference r311 (scope relative) (span (offset 25274) (line 552) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 25274) (line 552) (column 90) (len 8)))))
    (reference r312 (scope relative) (span (offset 25313) (line 553) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 25313) (line 553) (column 23) (len 17)))))
    (reference r313 (scope relative) (span (offset 25337) (line 553) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 25337) (line 553) (column 47) (len 20)))))
    (reference r314 (scope relative) (span (offset 25361) (line 553) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 25361) (line 553) (column 71) (len 8)))))
    (reference r315 (scope relative) (span (offset 25371) (line 553) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 25371) (line 553) (column 81) (len 6)))))
    (reference r316 (scope relative) (span (offset 25379) (line 553) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 25379) (line 553) (column 89) (len 10)))))
    (reference r317 (scope relative) (span (offset 25453) (line 556) (column 53) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 25453) (line 556) (column 53) (len 23)))))
    (reference r318 (scope relative) (span (offset 26121) (line 569) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 26121) (line 569) (column 23) (len 7)))))
    (reference r319 (scope relative) (span (offset 26166) (line 570) (column 29) (len 39)) (segments (segment 0 (token "CartesianMomentOfForce3dCoordinateFrame") (name "CartesianMomentOfForce3dCoordinateFrame") (separator none) (span (offset 26166) (line 570) (column 29) (len 39)))))
    (reference r320 (scope relative) (span (offset 26160) (line 570) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 26160) (line 570) (column 23) (len 4)))))
    (reference r321 (scope relative) (span (offset 26377) (line 575) (column 62) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 26377) (line 575) (column 62) (len 19)))))
    (reference r322 (scope relative) (span (offset 26421) (line 576) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 26421) (line 576) (column 23) (len 7)))))
    (reference r323 (scope relative) (span (offset 26460) (line 577) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 26460) (line 577) (column 23) (len 12)))))
    (reference r324 (scope relative) (span (offset 26510) (line 578) (column 30) (len 17)) (segments (segment 0 (token "MomentOfForceUnit") (name "MomentOfForceUnit") (separator none) (span (offset 26510) (line 578) (column 30) (len 17)))))
    (reference r325 (scope relative) (span (offset 26503) (line 578) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 26503) (line 578) (column 23) (len 5)))))
    (reference r326 (scope relative) (span (offset 26613) (line 582) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 26613) (line 582) (column 34) (len 19)))))
    (reference r327 (scope relative) (span (offset 27306) (line 595) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 27306) (line 595) (column 28) (len 4)))))
    (reference r328 (scope relative) (span (offset 27301) (line 595) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 27301) (line 595) (column 23) (len 3)))))
    (reference r329 (scope relative) (span (offset 27340) (line 596) (column 29) (len 10)) (segments (segment 0 (token "TorqueUnit") (name "TorqueUnit") (separator none) (span (offset 27340) (line 596) (column 29) (len 10)))))
    (reference r330 (scope relative) (span (offset 27334) (line 596) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 27334) (line 596) (column 23) (len 4)))))
    (reference r331 (scope relative) (span (offset 27463) (line 601) (column 33) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 27463) (line 601) (column 33) (len 11)))))
    (reference r332 (scope relative) (span (offset 27513) (line 602) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27513) (line 602) (column 37) (len 19)))))
    (reference r333 (scope relative) (span (offset 27542) (line 602) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27542) (line 602) (column 66) (len 8)))))
    (reference r334 (scope relative) (span (offset 27553) (line 602) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27553) (line 602) (column 77) (len 3)))))
    (reference r335 (scope relative) (span (offset 27557) (line 602) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 27557) (line 602) (column 81) (len 1)))))
    (reference r336 (scope relative) (span (offset 27564) (line 602) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27564) (line 602) (column 88) (len 8)))))
    (reference r337 (scope relative) (span (offset 27614) (line 603) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27614) (line 603) (column 35) (len 19)))))
    (reference r338 (scope relative) (span (offset 27643) (line 603) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27643) (line 603) (column 64) (len 8)))))
    (reference r339 (scope relative) (span (offset 27654) (line 603) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27654) (line 603) (column 75) (len 3)))))
    (reference r340 (scope relative) (span (offset 27658) (line 603) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 27658) (line 603) (column 79) (len 1)))))
    (reference r341 (scope relative) (span (offset 27665) (line 603) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27665) (line 603) (column 86) (len 8)))))
    (reference r342 (scope relative) (span (offset 27719) (line 604) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27719) (line 604) (column 39) (len 19)))))
    (reference r343 (scope relative) (span (offset 27748) (line 604) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27748) (line 604) (column 68) (len 8)))))
    (reference r344 (scope relative) (span (offset 27759) (line 604) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27759) (line 604) (column 79) (len 3)))))
    (reference r345 (scope relative) (span (offset 27763) (line 604) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 27763) (line 604) (column 83) (len 1)))))
    (reference r346 (scope relative) (span (offset 27770) (line 604) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27770) (line 604) (column 90) (len 8)))))
    (reference r347 (scope relative) (span (offset 27809) (line 605) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 27809) (line 605) (column 23) (len 17)))))
    (reference r348 (scope relative) (span (offset 27833) (line 605) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 27833) (line 605) (column 47) (len 20)))))
    (reference r349 (scope relative) (span (offset 27857) (line 605) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 27857) (line 605) (column 71) (len 8)))))
    (reference r350 (scope relative) (span (offset 27867) (line 605) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 27867) (line 605) (column 81) (len 6)))))
    (reference r351 (scope relative) (span (offset 27875) (line 605) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 27875) (line 605) (column 89) (len 10)))))
    (reference r352 (scope relative) (span (offset 27986) (line 609) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 27986) (line 609) (column 42) (len 19)))))
    (reference r353 (scope relative) (span (offset 28779) (line 622) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 28779) (line 622) (column 28) (len 4)))))
    (reference r354 (scope relative) (span (offset 28774) (line 622) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 28774) (line 622) (column 23) (len 3)))))
    (reference r355 (scope relative) (span (offset 28813) (line 623) (column 29) (len 18)) (segments (segment 0 (token "AngularImpulseUnit") (name "AngularImpulseUnit") (separator none) (span (offset 28813) (line 623) (column 29) (len 18)))))
    (reference r356 (scope relative) (span (offset 28807) (line 623) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 28807) (line 623) (column 23) (len 4)))))
    (reference r357 (scope relative) (span (offset 28968) (line 628) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 28968) (line 628) (column 41) (len 11)))))
    (reference r358 (scope relative) (span (offset 29018) (line 629) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29018) (line 629) (column 37) (len 19)))))
    (reference r359 (scope relative) (span (offset 29047) (line 629) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29047) (line 629) (column 66) (len 8)))))
    (reference r360 (scope relative) (span (offset 29058) (line 629) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29058) (line 629) (column 77) (len 3)))))
    (reference r361 (scope relative) (span (offset 29062) (line 629) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 29062) (line 629) (column 81) (len 1)))))
    (reference r362 (scope relative) (span (offset 29069) (line 629) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29069) (line 629) (column 88) (len 8)))))
    (reference r363 (scope relative) (span (offset 29119) (line 630) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29119) (line 630) (column 35) (len 19)))))
    (reference r364 (scope relative) (span (offset 29148) (line 630) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29148) (line 630) (column 64) (len 8)))))
    (reference r365 (scope relative) (span (offset 29159) (line 630) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29159) (line 630) (column 75) (len 3)))))
    (reference r366 (scope relative) (span (offset 29163) (line 630) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 29163) (line 630) (column 79) (len 1)))))
    (reference r367 (scope relative) (span (offset 29170) (line 630) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29170) (line 630) (column 86) (len 8)))))
    (reference r368 (scope relative) (span (offset 29224) (line 631) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29224) (line 631) (column 39) (len 19)))))
    (reference r369 (scope relative) (span (offset 29253) (line 631) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29253) (line 631) (column 68) (len 8)))))
    (reference r370 (scope relative) (span (offset 29264) (line 631) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29264) (line 631) (column 79) (len 3)))))
    (reference r371 (scope relative) (span (offset 29268) (line 631) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 29268) (line 631) (column 83) (len 1)))))
    (reference r372 (scope relative) (span (offset 29275) (line 631) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29275) (line 631) (column 90) (len 8)))))
    (reference r373 (scope relative) (span (offset 29314) (line 632) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 29314) (line 632) (column 23) (len 17)))))
    (reference r374 (scope relative) (span (offset 29338) (line 632) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 29338) (line 632) (column 47) (len 20)))))
    (reference r375 (scope relative) (span (offset 29362) (line 632) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 29362) (line 632) (column 71) (len 8)))))
    (reference r376 (scope relative) (span (offset 29372) (line 632) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 29372) (line 632) (column 81) (len 6)))))
    (reference r377 (scope relative) (span (offset 29380) (line 632) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 29380) (line 632) (column 89) (len 10)))))
    (reference r378 (scope relative) (span (offset 29455) (line 635) (column 54) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 29455) (line 635) (column 54) (len 23)))))
    (reference r379 (scope relative) (span (offset 30249) (line 648) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 30249) (line 648) (column 23) (len 7)))))
    (reference r380 (scope relative) (span (offset 30294) (line 649) (column 29) (len 40)) (segments (segment 0 (token "CartesianAngularImpulse3dCoordinateFrame") (name "CartesianAngularImpulse3dCoordinateFrame") (separator none) (span (offset 30294) (line 649) (column 29) (len 40)))))
    (reference r381 (scope relative) (span (offset 30288) (line 649) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 30288) (line 649) (column 23) (len 4)))))
    (reference r382 (scope relative) (span (offset 30509) (line 654) (column 63) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 30509) (line 654) (column 63) (len 19)))))
    (reference r383 (scope relative) (span (offset 30553) (line 655) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 30553) (line 655) (column 23) (len 7)))))
    (reference r384 (scope relative) (span (offset 30592) (line 656) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 30592) (line 656) (column 23) (len 12)))))
    (reference r385 (scope relative) (span (offset 30642) (line 657) (column 30) (len 18)) (segments (segment 0 (token "AngularImpulseUnit") (name "AngularImpulseUnit") (separator none) (span (offset 30642) (line 657) (column 30) (len 18)))))
    (reference r386 (scope relative) (span (offset 30635) (line 657) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 30635) (line 657) (column 23) (len 5)))))
    (reference r387 (scope relative) (span (offset 30750) (line 661) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 30750) (line 661) (column 36) (len 19)))))
    (reference r388 (scope relative) (span (offset 31360) (line 674) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 31360) (line 674) (column 28) (len 4)))))
    (reference r389 (scope relative) (span (offset 31355) (line 674) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 31355) (line 674) (column 23) (len 3)))))
    (reference r390 (scope relative) (span (offset 31394) (line 675) (column 29) (len 12)) (segments (segment 0 (token "PressureUnit") (name "PressureUnit") (separator none) (span (offset 31394) (line 675) (column 29) (len 12)))))
    (reference r391 (scope relative) (span (offset 31388) (line 675) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 31388) (line 675) (column 23) (len 4)))))
    (reference r392 (scope relative) (span (offset 31525) (line 680) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 31525) (line 680) (column 35) (len 11)))))
    (reference r393 (scope relative) (span (offset 31575) (line 681) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31575) (line 681) (column 37) (len 19)))))
    (reference r394 (scope relative) (span (offset 31604) (line 681) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31604) (line 681) (column 66) (len 8)))))
    (reference r395 (scope relative) (span (offset 31615) (line 681) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31615) (line 681) (column 77) (len 3)))))
    (reference r396 (scope relative) (span (offset 31619) (line 681) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 31619) (line 681) (column 81) (len 1)))))
    (reference r397 (scope relative) (span (offset 31626) (line 681) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31626) (line 681) (column 88) (len 8)))))
    (reference r398 (scope relative) (span (offset 31677) (line 682) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31677) (line 682) (column 35) (len 19)))))
    (reference r399 (scope relative) (span (offset 31706) (line 682) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31706) (line 682) (column 64) (len 8)))))
    (reference r400 (scope relative) (span (offset 31717) (line 682) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31717) (line 682) (column 75) (len 3)))))
    (reference r401 (scope relative) (span (offset 31721) (line 682) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 31721) (line 682) (column 79) (len 1)))))
    (reference r402 (scope relative) (span (offset 31728) (line 682) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31728) (line 682) (column 86) (len 8)))))
    (reference r403 (scope relative) (span (offset 31782) (line 683) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31782) (line 683) (column 39) (len 19)))))
    (reference r404 (scope relative) (span (offset 31811) (line 683) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31811) (line 683) (column 68) (len 8)))))
    (reference r405 (scope relative) (span (offset 31822) (line 683) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31822) (line 683) (column 79) (len 3)))))
    (reference r406 (scope relative) (span (offset 31826) (line 683) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 31826) (line 683) (column 83) (len 1)))))
    (reference r407 (scope relative) (span (offset 31833) (line 683) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31833) (line 683) (column 90) (len 8)))))
    (reference r408 (scope relative) (span (offset 31872) (line 684) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 31872) (line 684) (column 23) (len 17)))))
    (reference r409 (scope relative) (span (offset 31896) (line 684) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 31896) (line 684) (column 47) (len 20)))))
    (reference r410 (scope relative) (span (offset 31920) (line 684) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 31920) (line 684) (column 71) (len 8)))))
    (reference r411 (scope relative) (span (offset 31930) (line 684) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 31930) (line 684) (column 81) (len 6)))))
    (reference r412 (scope relative) (span (offset 31938) (line 684) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 31938) (line 684) (column 89) (len 10)))))
    (reference r413 (scope relative) (span (offset 32703) (line 704) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 32703) (line 704) (column 34) (len 19)))))
    (reference r414 (scope relative) (span (offset 33269) (line 717) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 33269) (line 717) (column 28) (len 4)))))
    (reference r415 (scope relative) (span (offset 33264) (line 717) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 33264) (line 717) (column 23) (len 3)))))
    (reference r416 (scope relative) (span (offset 33303) (line 718) (column 29) (len 10)) (segments (segment 0 (token "StressUnit") (name "StressUnit") (separator none) (span (offset 33303) (line 718) (column 29) (len 10)))))
    (reference r417 (scope relative) (span (offset 33297) (line 718) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 33297) (line 718) (column 23) (len 4)))))
    (reference r418 (scope relative) (span (offset 33426) (line 723) (column 33) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 33426) (line 723) (column 33) (len 11)))))
    (reference r419 (scope relative) (span (offset 33476) (line 724) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33476) (line 724) (column 37) (len 19)))))
    (reference r420 (scope relative) (span (offset 33505) (line 724) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33505) (line 724) (column 66) (len 8)))))
    (reference r421 (scope relative) (span (offset 33516) (line 724) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33516) (line 724) (column 77) (len 3)))))
    (reference r422 (scope relative) (span (offset 33520) (line 724) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 33520) (line 724) (column 81) (len 1)))))
    (reference r423 (scope relative) (span (offset 33527) (line 724) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33527) (line 724) (column 88) (len 8)))))
    (reference r424 (scope relative) (span (offset 33578) (line 725) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33578) (line 725) (column 35) (len 19)))))
    (reference r425 (scope relative) (span (offset 33607) (line 725) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33607) (line 725) (column 64) (len 8)))))
    (reference r426 (scope relative) (span (offset 33618) (line 725) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33618) (line 725) (column 75) (len 3)))))
    (reference r427 (scope relative) (span (offset 33622) (line 725) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 33622) (line 725) (column 79) (len 1)))))
    (reference r428 (scope relative) (span (offset 33629) (line 725) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33629) (line 725) (column 86) (len 8)))))
    (reference r429 (scope relative) (span (offset 33683) (line 726) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33683) (line 726) (column 39) (len 19)))))
    (reference r430 (scope relative) (span (offset 33712) (line 726) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33712) (line 726) (column 68) (len 8)))))
    (reference r431 (scope relative) (span (offset 33723) (line 726) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33723) (line 726) (column 79) (len 3)))))
    (reference r432 (scope relative) (span (offset 33727) (line 726) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 33727) (line 726) (column 83) (len 1)))))
    (reference r433 (scope relative) (span (offset 33734) (line 726) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33734) (line 726) (column 90) (len 8)))))
    (reference r434 (scope relative) (span (offset 33773) (line 727) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 33773) (line 727) (column 23) (len 17)))))
    (reference r435 (scope relative) (span (offset 33797) (line 727) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 33797) (line 727) (column 47) (len 20)))))
    (reference r436 (scope relative) (span (offset 33821) (line 727) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 33821) (line 727) (column 71) (len 8)))))
    (reference r437 (scope relative) (span (offset 33831) (line 727) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 33831) (line 727) (column 81) (len 6)))))
    (reference r438 (scope relative) (span (offset 33839) (line 727) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 33839) (line 727) (column 89) (len 10)))))
    (reference r439 (scope relative) (span (offset 33906) (line 730) (column 46) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 33906) (line 730) (column 46) (len 19)))))
    (reference r440 (scope relative) (span (offset 34474) (line 743) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 34474) (line 743) (column 23) (len 7)))))
    (reference r441 (scope relative) (span (offset 34518) (line 744) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 34518) (line 744) (column 28) (len 4)))))
    (reference r442 (scope relative) (span (offset 34513) (line 744) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 34513) (line 744) (column 23) (len 3)))))
    (reference r443 (scope relative) (span (offset 34555) (line 745) (column 29) (len 37)) (segments (segment 0 (token "Cartesian3dStressMeasurementReference") (name "Cartesian3dStressMeasurementReference") (separator none) (span (offset 34555) (line 745) (column 29) (len 37)))))
    (reference r444 (scope relative) (span (offset 34549) (line 745) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 34549) (line 745) (column 23) (len 4)))))
    (reference r445 (scope relative) (span (offset 34737) (line 750) (column 60) (len 26)) (segments (segment 0 (token "TensorMeasurementReference") (name "TensorMeasurementReference") (separator none) (span (offset 34737) (line 750) (column 60) (len 26)))))
    (reference r446 (scope relative) (span (offset 34788) (line 751) (column 23) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 34788) (line 751) (column 23) (len 10)))))
    (reference r447 (scope relative) (span (offset 34831) (line 752) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 34831) (line 752) (column 23) (len 7)))))
    (reference r448 (scope relative) (span (offset 34877) (line 753) (column 30) (len 10)) (segments (segment 0 (token "StressUnit") (name "StressUnit") (separator none) (span (offset 34877) (line 753) (column 30) (len 10)))))
    (reference r449 (scope relative) (span (offset 34870) (line 753) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 34870) (line 753) (column 23) (len 5)))))
    (reference r450 (scope relative) (span (offset 34986) (line 757) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 34986) (line 757) (column 40) (len 19)))))
    (reference r451 (scope relative) (span (offset 35853) (line 770) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 35853) (line 770) (column 28) (len 4)))))
    (reference r452 (scope relative) (span (offset 35848) (line 770) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 35848) (line 770) (column 23) (len 3)))))
    (reference r453 (scope relative) (span (offset 35887) (line 771) (column 29) (len 16)) (segments (segment 0 (token "NormalStressUnit") (name "NormalStressUnit") (separator none) (span (offset 35887) (line 771) (column 29) (len 16)))))
    (reference r454 (scope relative) (span (offset 35881) (line 771) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 35881) (line 771) (column 23) (len 4)))))
    (reference r455 (scope relative) (span (offset 36034) (line 776) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 36034) (line 776) (column 39) (len 11)))))
    (reference r456 (scope relative) (span (offset 36084) (line 777) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36084) (line 777) (column 37) (len 19)))))
    (reference r457 (scope relative) (span (offset 36113) (line 777) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36113) (line 777) (column 66) (len 8)))))
    (reference r458 (scope relative) (span (offset 36124) (line 777) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36124) (line 777) (column 77) (len 3)))))
    (reference r459 (scope relative) (span (offset 36128) (line 777) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 36128) (line 777) (column 81) (len 1)))))
    (reference r460 (scope relative) (span (offset 36135) (line 777) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36135) (line 777) (column 88) (len 8)))))
    (reference r461 (scope relative) (span (offset 36186) (line 778) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36186) (line 778) (column 35) (len 19)))))
    (reference r462 (scope relative) (span (offset 36215) (line 778) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36215) (line 778) (column 64) (len 8)))))
    (reference r463 (scope relative) (span (offset 36226) (line 778) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36226) (line 778) (column 75) (len 3)))))
    (reference r464 (scope relative) (span (offset 36230) (line 778) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 36230) (line 778) (column 79) (len 1)))))
    (reference r465 (scope relative) (span (offset 36237) (line 778) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36237) (line 778) (column 86) (len 8)))))
    (reference r466 (scope relative) (span (offset 36291) (line 779) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36291) (line 779) (column 39) (len 19)))))
    (reference r467 (scope relative) (span (offset 36320) (line 779) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36320) (line 779) (column 68) (len 8)))))
    (reference r468 (scope relative) (span (offset 36331) (line 779) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36331) (line 779) (column 79) (len 3)))))
    (reference r469 (scope relative) (span (offset 36335) (line 779) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 36335) (line 779) (column 83) (len 1)))))
    (reference r470 (scope relative) (span (offset 36342) (line 779) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36342) (line 779) (column 90) (len 8)))))
    (reference r471 (scope relative) (span (offset 36381) (line 780) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 36381) (line 780) (column 23) (len 17)))))
    (reference r472 (scope relative) (span (offset 36405) (line 780) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 36405) (line 780) (column 47) (len 20)))))
    (reference r473 (scope relative) (span (offset 36429) (line 780) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 36429) (line 780) (column 71) (len 8)))))
    (reference r474 (scope relative) (span (offset 36439) (line 780) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 36439) (line 780) (column 81) (len 6)))))
    (reference r475 (scope relative) (span (offset 36447) (line 780) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 36447) (line 780) (column 89) (len 10)))))
    (reference r476 (scope relative) (span (offset 36554) (line 784) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 36554) (line 784) (column 39) (len 19)))))
    (reference r477 (scope relative) (span (offset 37422) (line 797) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 37422) (line 797) (column 28) (len 4)))))
    (reference r478 (scope relative) (span (offset 37417) (line 797) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 37417) (line 797) (column 23) (len 3)))))
    (reference r479 (scope relative) (span (offset 37456) (line 798) (column 29) (len 15)) (segments (segment 0 (token "ShearStressUnit") (name "ShearStressUnit") (separator none) (span (offset 37456) (line 798) (column 29) (len 15)))))
    (reference r480 (scope relative) (span (offset 37450) (line 798) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 37450) (line 798) (column 23) (len 4)))))
    (reference r481 (scope relative) (span (offset 37599) (line 803) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 37599) (line 803) (column 38) (len 11)))))
    (reference r482 (scope relative) (span (offset 37649) (line 804) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37649) (line 804) (column 37) (len 19)))))
    (reference r483 (scope relative) (span (offset 37678) (line 804) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37678) (line 804) (column 66) (len 8)))))
    (reference r484 (scope relative) (span (offset 37689) (line 804) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37689) (line 804) (column 77) (len 3)))))
    (reference r485 (scope relative) (span (offset 37693) (line 804) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 37693) (line 804) (column 81) (len 1)))))
    (reference r486 (scope relative) (span (offset 37700) (line 804) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37700) (line 804) (column 88) (len 8)))))
    (reference r487 (scope relative) (span (offset 37751) (line 805) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37751) (line 805) (column 35) (len 19)))))
    (reference r488 (scope relative) (span (offset 37780) (line 805) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37780) (line 805) (column 64) (len 8)))))
    (reference r489 (scope relative) (span (offset 37791) (line 805) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37791) (line 805) (column 75) (len 3)))))
    (reference r490 (scope relative) (span (offset 37795) (line 805) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 37795) (line 805) (column 79) (len 1)))))
    (reference r491 (scope relative) (span (offset 37802) (line 805) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37802) (line 805) (column 86) (len 8)))))
    (reference r492 (scope relative) (span (offset 37856) (line 806) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37856) (line 806) (column 39) (len 19)))))
    (reference r493 (scope relative) (span (offset 37885) (line 806) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37885) (line 806) (column 68) (len 8)))))
    (reference r494 (scope relative) (span (offset 37896) (line 806) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37896) (line 806) (column 79) (len 3)))))
    (reference r495 (scope relative) (span (offset 37900) (line 806) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 37900) (line 806) (column 83) (len 1)))))
    (reference r496 (scope relative) (span (offset 37907) (line 806) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37907) (line 806) (column 90) (len 8)))))
    (reference r497 (scope relative) (span (offset 37946) (line 807) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 37946) (line 807) (column 23) (len 17)))))
    (reference r498 (scope relative) (span (offset 37970) (line 807) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 37970) (line 807) (column 47) (len 20)))))
    (reference r499 (scope relative) (span (offset 37994) (line 807) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 37994) (line 807) (column 71) (len 8)))))
    (reference r500 (scope relative) (span (offset 38004) (line 807) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 38004) (line 807) (column 81) (len 6)))))
    (reference r501 (scope relative) (span (offset 38012) (line 807) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 38012) (line 807) (column 89) (len 10)))))
    (reference r502 (scope relative) (span (offset 38108) (line 811) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 38108) (line 811) (column 34) (len 19)))))
    (reference r503 (scope relative) (span (offset 38657) (line 824) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 38657) (line 824) (column 28) (len 4)))))
    (reference r504 (scope relative) (span (offset 38652) (line 824) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 38652) (line 824) (column 23) (len 3)))))
    (reference r505 (scope relative) (span (offset 38691) (line 825) (column 29) (len 10)) (segments (segment 0 (token "StrainUnit") (name "StrainUnit") (separator none) (span (offset 38691) (line 825) (column 29) (len 10)))))
    (reference r506 (scope relative) (span (offset 38685) (line 825) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 38685) (line 825) (column 23) (len 4)))))
    (reference r507 (scope relative) (span (offset 38814) (line 830) (column 33) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 38814) (line 830) (column 33) (len 16)))))
    (reference r508 (scope relative) (span (offset 38885) (line 833) (column 46) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 38885) (line 833) (column 46) (len 19)))))
    (reference r509 (scope relative) (span (offset 39436) (line 846) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 39436) (line 846) (column 23) (len 7)))))
    (reference r510 (scope relative) (span (offset 39480) (line 847) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 39480) (line 847) (column 28) (len 4)))))
    (reference r511 (scope relative) (span (offset 39475) (line 847) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 39475) (line 847) (column 23) (len 3)))))
    (reference r512 (scope relative) (span (offset 39517) (line 848) (column 29) (len 37)) (segments (segment 0 (token "Cartesian3dStrainMeasurementReference") (name "Cartesian3dStrainMeasurementReference") (separator none) (span (offset 39517) (line 848) (column 29) (len 37)))))
    (reference r513 (scope relative) (span (offset 39511) (line 848) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 39511) (line 848) (column 23) (len 4)))))
    (reference r514 (scope relative) (span (offset 39699) (line 853) (column 60) (len 26)) (segments (segment 0 (token "TensorMeasurementReference") (name "TensorMeasurementReference") (separator none) (span (offset 39699) (line 853) (column 60) (len 26)))))
    (reference r515 (scope relative) (span (offset 39750) (line 854) (column 23) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 39750) (line 854) (column 23) (len 10)))))
    (reference r516 (scope relative) (span (offset 39793) (line 855) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 39793) (line 855) (column 23) (len 7)))))
    (reference r517 (scope relative) (span (offset 39839) (line 856) (column 30) (len 10)) (segments (segment 0 (token "StrainUnit") (name "StrainUnit") (separator none) (span (offset 39839) (line 856) (column 30) (len 10)))))
    (reference r518 (scope relative) (span (offset 39832) (line 856) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 39832) (line 856) (column 23) (len 5)))))
    (reference r519 (scope relative) (span (offset 39965) (line 860) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 39965) (line 860) (column 48) (len 17)))))
    (reference r520 (scope relative) (span (offset 40649) (line 877) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 40649) (line 877) (column 39) (len 17)))))
    (reference r521 (scope relative) (span (offset 41343) (line 894) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 41343) (line 894) (column 48) (len 17)))))
    (reference r522 (scope relative) (span (offset 42026) (line 911) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 42026) (line 911) (column 41) (len 17)))))
    (reference r523 (scope relative) (span (offset 42763) (line 928) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 42763) (line 928) (column 47) (len 19)))))
    (reference r524 (scope relative) (span (offset 43376) (line 941) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 43376) (line 941) (column 28) (len 4)))))
    (reference r525 (scope relative) (span (offset 43371) (line 941) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 43371) (line 941) (column 23) (len 3)))))
    (reference r526 (scope relative) (span (offset 43410) (line 942) (column 29) (len 23)) (segments (segment 0 (token "ModulusOfElasticityUnit") (name "ModulusOfElasticityUnit") (separator none) (span (offset 43410) (line 942) (column 29) (len 23)))))
    (reference r527 (scope relative) (span (offset 43404) (line 942) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 43404) (line 942) (column 23) (len 4)))))
    (reference r528 (scope relative) (span (offset 43585) (line 947) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 43585) (line 947) (column 46) (len 11)))))
    (reference r529 (scope relative) (span (offset 43635) (line 948) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43635) (line 948) (column 37) (len 19)))))
    (reference r530 (scope relative) (span (offset 43664) (line 948) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43664) (line 948) (column 66) (len 8)))))
    (reference r531 (scope relative) (span (offset 43675) (line 948) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43675) (line 948) (column 77) (len 3)))))
    (reference r532 (scope relative) (span (offset 43679) (line 948) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 43679) (line 948) (column 81) (len 1)))))
    (reference r533 (scope relative) (span (offset 43686) (line 948) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43686) (line 948) (column 88) (len 8)))))
    (reference r534 (scope relative) (span (offset 43737) (line 949) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43737) (line 949) (column 35) (len 19)))))
    (reference r535 (scope relative) (span (offset 43766) (line 949) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43766) (line 949) (column 64) (len 8)))))
    (reference r536 (scope relative) (span (offset 43777) (line 949) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43777) (line 949) (column 75) (len 3)))))
    (reference r537 (scope relative) (span (offset 43781) (line 949) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 43781) (line 949) (column 79) (len 1)))))
    (reference r538 (scope relative) (span (offset 43788) (line 949) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43788) (line 949) (column 86) (len 8)))))
    (reference r539 (scope relative) (span (offset 43842) (line 950) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43842) (line 950) (column 39) (len 19)))))
    (reference r540 (scope relative) (span (offset 43871) (line 950) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43871) (line 950) (column 68) (len 8)))))
    (reference r541 (scope relative) (span (offset 43882) (line 950) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43882) (line 950) (column 79) (len 3)))))
    (reference r542 (scope relative) (span (offset 43886) (line 950) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 43886) (line 950) (column 83) (len 1)))))
    (reference r543 (scope relative) (span (offset 43893) (line 950) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43893) (line 950) (column 90) (len 8)))))
    (reference r544 (scope relative) (span (offset 43932) (line 951) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 43932) (line 951) (column 23) (len 17)))))
    (reference r545 (scope relative) (span (offset 43956) (line 951) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 43956) (line 951) (column 47) (len 20)))))
    (reference r546 (scope relative) (span (offset 43980) (line 951) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 43980) (line 951) (column 71) (len 8)))))
    (reference r547 (scope relative) (span (offset 43990) (line 951) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 43990) (line 951) (column 81) (len 6)))))
    (reference r548 (scope relative) (span (offset 43998) (line 951) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 43998) (line 951) (column 89) (len 10)))))
    (reference r549 (scope relative) (span (offset 44051) (line 954) (column 32) (len 23)) (segments (segment 0 (token "ModulusOfElasticityUnit") (name "ModulusOfElasticityUnit") (separator none) (span (offset 44051) (line 954) (column 32) (len 23)))))
    (reference r550 (scope relative) (span (offset 44108) (line 955) (column 33) (len 24)) (segments (segment 0 (token "ModulusOfElasticityValue") (name "ModulusOfElasticityValue") (separator none) (span (offset 44108) (line 955) (column 33) (len 24)))))
    (reference r551 (scope relative) (span (offset 44161) (line 956) (column 28) (len 19)) (segments (segment 0 (token "modulusOfElasticity") (name "modulusOfElasticity") (separator none) (span (offset 44161) (line 956) (column 28) (len 19)))))
    (reference r552 (scope relative) (span (offset 44296) (line 959) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 44296) (line 959) (column 45) (len 19)))))
    (reference r553 (scope relative) (span (offset 44883) (line 972) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 44883) (line 972) (column 28) (len 4)))))
    (reference r554 (scope relative) (span (offset 44878) (line 972) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 44878) (line 972) (column 23) (len 3)))))
    (reference r555 (scope relative) (span (offset 44917) (line 973) (column 29) (len 21)) (segments (segment 0 (token "ModulusOfRigidityUnit") (name "ModulusOfRigidityUnit") (separator none) (span (offset 44917) (line 973) (column 29) (len 21)))))
    (reference r556 (scope relative) (span (offset 44911) (line 973) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 44911) (line 973) (column 23) (len 4)))))
    (reference r557 (scope relative) (span (offset 45084) (line 978) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 45084) (line 978) (column 44) (len 11)))))
    (reference r558 (scope relative) (span (offset 45134) (line 979) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45134) (line 979) (column 37) (len 19)))))
    (reference r559 (scope relative) (span (offset 45163) (line 979) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45163) (line 979) (column 66) (len 8)))))
    (reference r560 (scope relative) (span (offset 45174) (line 979) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45174) (line 979) (column 77) (len 3)))))
    (reference r561 (scope relative) (span (offset 45178) (line 979) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 45178) (line 979) (column 81) (len 1)))))
    (reference r562 (scope relative) (span (offset 45185) (line 979) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45185) (line 979) (column 88) (len 8)))))
    (reference r563 (scope relative) (span (offset 45236) (line 980) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45236) (line 980) (column 35) (len 19)))))
    (reference r564 (scope relative) (span (offset 45265) (line 980) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45265) (line 980) (column 64) (len 8)))))
    (reference r565 (scope relative) (span (offset 45276) (line 980) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45276) (line 980) (column 75) (len 3)))))
    (reference r566 (scope relative) (span (offset 45280) (line 980) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 45280) (line 980) (column 79) (len 1)))))
    (reference r567 (scope relative) (span (offset 45287) (line 980) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45287) (line 980) (column 86) (len 8)))))
    (reference r568 (scope relative) (span (offset 45341) (line 981) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45341) (line 981) (column 39) (len 19)))))
    (reference r569 (scope relative) (span (offset 45370) (line 981) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45370) (line 981) (column 68) (len 8)))))
    (reference r570 (scope relative) (span (offset 45381) (line 981) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45381) (line 981) (column 79) (len 3)))))
    (reference r571 (scope relative) (span (offset 45385) (line 981) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 45385) (line 981) (column 83) (len 1)))))
    (reference r572 (scope relative) (span (offset 45392) (line 981) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45392) (line 981) (column 90) (len 8)))))
    (reference r573 (scope relative) (span (offset 45431) (line 982) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 45431) (line 982) (column 23) (len 17)))))
    (reference r574 (scope relative) (span (offset 45455) (line 982) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 45455) (line 982) (column 47) (len 20)))))
    (reference r575 (scope relative) (span (offset 45479) (line 982) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 45479) (line 982) (column 71) (len 8)))))
    (reference r576 (scope relative) (span (offset 45489) (line 982) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 45489) (line 982) (column 81) (len 6)))))
    (reference r577 (scope relative) (span (offset 45497) (line 982) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 45497) (line 982) (column 89) (len 10)))))
    (reference r578 (scope relative) (span (offset 45550) (line 985) (column 32) (len 21)) (segments (segment 0 (token "ModulusOfRigidityUnit") (name "ModulusOfRigidityUnit") (separator none) (span (offset 45550) (line 985) (column 32) (len 21)))))
    (reference r579 (scope relative) (span (offset 45605) (line 986) (column 33) (len 22)) (segments (segment 0 (token "ModulusOfRigidityValue") (name "ModulusOfRigidityValue") (separator none) (span (offset 45605) (line 986) (column 33) (len 22)))))
    (reference r580 (scope relative) (span (offset 45656) (line 987) (column 28) (len 17)) (segments (segment 0 (token "modulusOfRigidity") (name "modulusOfRigidity") (separator none) (span (offset 45656) (line 987) (column 28) (len 17)))))
    (reference r581 (scope relative) (span (offset 45794) (line 990) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 45794) (line 990) (column 48) (len 19)))))
    (reference r582 (scope relative) (span (offset 46421) (line 1003) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 46421) (line 1003) (column 28) (len 4)))))
    (reference r583 (scope relative) (span (offset 46416) (line 1003) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 46416) (line 1003) (column 23) (len 3)))))
    (reference r584 (scope relative) (span (offset 46455) (line 1004) (column 29) (len 24)) (segments (segment 0 (token "ModulusOfCompressionUnit") (name "ModulusOfCompressionUnit") (separator none) (span (offset 46455) (line 1004) (column 29) (len 24)))))
    (reference r585 (scope relative) (span (offset 46449) (line 1004) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 46449) (line 1004) (column 23) (len 4)))))
    (reference r586 (scope relative) (span (offset 46634) (line 1009) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 46634) (line 1009) (column 47) (len 11)))))
    (reference r587 (scope relative) (span (offset 46684) (line 1010) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 46684) (line 1010) (column 37) (len 19)))))
    (reference r588 (scope relative) (span (offset 46713) (line 1010) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 46713) (line 1010) (column 66) (len 8)))))
    (reference r589 (scope relative) (span (offset 46724) (line 1010) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 46724) (line 1010) (column 77) (len 3)))))
    (reference r590 (scope relative) (span (offset 46728) (line 1010) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 46728) (line 1010) (column 81) (len 1)))))
    (reference r591 (scope relative) (span (offset 46735) (line 1010) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 46735) (line 1010) (column 88) (len 8)))))
    (reference r592 (scope relative) (span (offset 46786) (line 1011) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 46786) (line 1011) (column 35) (len 19)))))
    (reference r593 (scope relative) (span (offset 46815) (line 1011) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 46815) (line 1011) (column 64) (len 8)))))
    (reference r594 (scope relative) (span (offset 46826) (line 1011) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 46826) (line 1011) (column 75) (len 3)))))
    (reference r595 (scope relative) (span (offset 46830) (line 1011) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 46830) (line 1011) (column 79) (len 1)))))
    (reference r596 (scope relative) (span (offset 46837) (line 1011) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 46837) (line 1011) (column 86) (len 8)))))
    (reference r597 (scope relative) (span (offset 46891) (line 1012) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 46891) (line 1012) (column 39) (len 19)))))
    (reference r598 (scope relative) (span (offset 46920) (line 1012) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 46920) (line 1012) (column 68) (len 8)))))
    (reference r599 (scope relative) (span (offset 46931) (line 1012) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 46931) (line 1012) (column 79) (len 3)))))
    (reference r600 (scope relative) (span (offset 46935) (line 1012) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 46935) (line 1012) (column 83) (len 1)))))
    (reference r601 (scope relative) (span (offset 46942) (line 1012) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 46942) (line 1012) (column 90) (len 8)))))
    (reference r602 (scope relative) (span (offset 46981) (line 1013) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 46981) (line 1013) (column 23) (len 17)))))
    (reference r603 (scope relative) (span (offset 47005) (line 1013) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 47005) (line 1013) (column 47) (len 20)))))
    (reference r604 (scope relative) (span (offset 47029) (line 1013) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 47029) (line 1013) (column 71) (len 8)))))
    (reference r605 (scope relative) (span (offset 47039) (line 1013) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 47039) (line 1013) (column 81) (len 6)))))
    (reference r606 (scope relative) (span (offset 47047) (line 1013) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 47047) (line 1013) (column 89) (len 10)))))
    (reference r607 (scope relative) (span (offset 47099) (line 1016) (column 31) (len 24)) (segments (segment 0 (token "ModulusOfCompressionUnit") (name "ModulusOfCompressionUnit") (separator none) (span (offset 47099) (line 1016) (column 31) (len 24)))))
    (reference r608 (scope relative) (span (offset 47156) (line 1017) (column 32) (len 25)) (segments (segment 0 (token "ModulusOfCompressionValue") (name "ModulusOfCompressionValue") (separator none) (span (offset 47156) (line 1017) (column 32) (len 25)))))
    (reference r609 (scope relative) (span (offset 47209) (line 1018) (column 27) (len 20)) (segments (segment 0 (token "modulusOfCompression") (name "modulusOfCompression") (separator none) (span (offset 47209) (line 1018) (column 27) (len 20)))))
    (reference r610 (scope relative) (span (offset 47322) (line 1021) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 47322) (line 1021) (column 43) (len 19)))))
    (reference r611 (scope relative) (span (offset 47945) (line 1034) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 47945) (line 1034) (column 28) (len 4)))))
    (reference r612 (scope relative) (span (offset 47940) (line 1034) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 47940) (line 1034) (column 23) (len 3)))))
    (reference r613 (scope relative) (span (offset 47979) (line 1035) (column 29) (len 19)) (segments (segment 0 (token "CompressibilityUnit") (name "CompressibilityUnit") (separator none) (span (offset 47979) (line 1035) (column 29) (len 19)))))
    (reference r614 (scope relative) (span (offset 47973) (line 1035) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 47973) (line 1035) (column 23) (len 4)))))
    (reference r615 (scope relative) (span (offset 48138) (line 1040) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 48138) (line 1040) (column 42) (len 11)))))
    (reference r616 (scope relative) (span (offset 48188) (line 1041) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48188) (line 1041) (column 37) (len 19)))))
    (reference r617 (scope relative) (span (offset 48217) (line 1041) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48217) (line 1041) (column 66) (len 8)))))
    (reference r618 (scope relative) (span (offset 48228) (line 1041) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48228) (line 1041) (column 77) (len 3)))))
    (reference r619 (scope relative) (span (offset 48232) (line 1041) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 48232) (line 1041) (column 81) (len 1)))))
    (reference r620 (scope relative) (span (offset 48239) (line 1041) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48239) (line 1041) (column 88) (len 8)))))
    (reference r621 (scope relative) (span (offset 48289) (line 1042) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48289) (line 1042) (column 35) (len 19)))))
    (reference r622 (scope relative) (span (offset 48318) (line 1042) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48318) (line 1042) (column 64) (len 8)))))
    (reference r623 (scope relative) (span (offset 48329) (line 1042) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48329) (line 1042) (column 75) (len 3)))))
    (reference r624 (scope relative) (span (offset 48333) (line 1042) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 48333) (line 1042) (column 79) (len 1)))))
    (reference r625 (scope relative) (span (offset 48340) (line 1042) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48340) (line 1042) (column 86) (len 8)))))
    (reference r626 (scope relative) (span (offset 48395) (line 1043) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48395) (line 1043) (column 39) (len 19)))))
    (reference r627 (scope relative) (span (offset 48424) (line 1043) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48424) (line 1043) (column 68) (len 8)))))
    (reference r628 (scope relative) (span (offset 48435) (line 1043) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48435) (line 1043) (column 79) (len 3)))))
    (reference r629 (scope relative) (span (offset 48439) (line 1043) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 48439) (line 1043) (column 83) (len 1)))))
    (reference r630 (scope relative) (span (offset 48446) (line 1043) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48446) (line 1043) (column 90) (len 8)))))
    (reference r631 (scope relative) (span (offset 48484) (line 1044) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 48484) (line 1044) (column 23) (len 17)))))
    (reference r632 (scope relative) (span (offset 48508) (line 1044) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 48508) (line 1044) (column 47) (len 20)))))
    (reference r633 (scope relative) (span (offset 48532) (line 1044) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48532) (line 1044) (column 71) (len 8)))))
    (reference r634 (scope relative) (span (offset 48542) (line 1044) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 48542) (line 1044) (column 81) (len 6)))))
    (reference r635 (scope relative) (span (offset 48550) (line 1044) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 48550) (line 1044) (column 89) (len 10)))))
    (reference r636 (scope relative) (span (offset 48684) (line 1048) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 48684) (line 1048) (column 51) (len 19)))))
    (reference r637 (scope relative) (span (offset 49525) (line 1061) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 49525) (line 1061) (column 28) (len 4)))))
    (reference r638 (scope relative) (span (offset 49520) (line 1061) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 49520) (line 1061) (column 23) (len 3)))))
    (reference r639 (scope relative) (span (offset 49559) (line 1062) (column 29) (len 27)) (segments (segment 0 (token "SecondAxialMomentOfAreaUnit") (name "SecondAxialMomentOfAreaUnit") (separator none) (span (offset 49559) (line 1062) (column 29) (len 27)))))
    (reference r640 (scope relative) (span (offset 49553) (line 1062) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 49553) (line 1062) (column 23) (len 4)))))
    (reference r641 (scope relative) (span (offset 49750) (line 1067) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 49750) (line 1067) (column 50) (len 11)))))
    (reference r642 (scope relative) (span (offset 49800) (line 1068) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 49800) (line 1068) (column 37) (len 19)))))
    (reference r643 (scope relative) (span (offset 49829) (line 1068) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 49829) (line 1068) (column 66) (len 8)))))
    (reference r644 (scope relative) (span (offset 49840) (line 1068) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 49840) (line 1068) (column 77) (len 3)))))
    (reference r645 (scope relative) (span (offset 49844) (line 1068) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 49844) (line 1068) (column 81) (len 1)))))
    (reference r646 (scope relative) (span (offset 49851) (line 1068) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 49851) (line 1068) (column 88) (len 8)))))
    (reference r647 (scope relative) (span (offset 49889) (line 1069) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 49889) (line 1069) (column 23) (len 17)))))
    (reference r648 (scope relative) (span (offset 49913) (line 1069) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 49913) (line 1069) (column 47) (len 20)))))
    (reference r649 (scope relative) (span (offset 49936) (line 1069) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 49936) (line 1069) (column 70) (len 8)))))
    (reference r650 (scope relative) (span (offset 50067) (line 1073) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 50067) (line 1073) (column 51) (len 19)))))
    (reference r651 (scope relative) (span (offset 50924) (line 1086) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 50924) (line 1086) (column 28) (len 4)))))
    (reference r652 (scope relative) (span (offset 50919) (line 1086) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 50919) (line 1086) (column 23) (len 3)))))
    (reference r653 (scope relative) (span (offset 50958) (line 1087) (column 29) (len 27)) (segments (segment 0 (token "SecondPolarMomentOfAreaUnit") (name "SecondPolarMomentOfAreaUnit") (separator none) (span (offset 50958) (line 1087) (column 29) (len 27)))))
    (reference r654 (scope relative) (span (offset 50952) (line 1087) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 50952) (line 1087) (column 23) (len 4)))))
    (reference r655 (scope relative) (span (offset 51149) (line 1092) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 51149) (line 1092) (column 50) (len 11)))))
    (reference r656 (scope relative) (span (offset 51199) (line 1093) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51199) (line 1093) (column 37) (len 19)))))
    (reference r657 (scope relative) (span (offset 51228) (line 1093) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51228) (line 1093) (column 66) (len 8)))))
    (reference r658 (scope relative) (span (offset 51239) (line 1093) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51239) (line 1093) (column 77) (len 3)))))
    (reference r659 (scope relative) (span (offset 51243) (line 1093) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 51243) (line 1093) (column 81) (len 1)))))
    (reference r660 (scope relative) (span (offset 51250) (line 1093) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51250) (line 1093) (column 88) (len 8)))))
    (reference r661 (scope relative) (span (offset 51288) (line 1094) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 51288) (line 1094) (column 23) (len 17)))))
    (reference r662 (scope relative) (span (offset 51312) (line 1094) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 51312) (line 1094) (column 47) (len 20)))))
    (reference r663 (scope relative) (span (offset 51335) (line 1094) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 51335) (line 1094) (column 70) (len 8)))))
    (reference r664 (scope relative) (span (offset 51443) (line 1098) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 51443) (line 1098) (column 42) (len 19)))))
    (reference r665 (scope relative) (span (offset 52117) (line 1111) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 52117) (line 1111) (column 28) (len 4)))))
    (reference r666 (scope relative) (span (offset 52112) (line 1111) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 52112) (line 1111) (column 23) (len 3)))))
    (reference r667 (scope relative) (span (offset 52151) (line 1112) (column 29) (len 18)) (segments (segment 0 (token "SectionModulusUnit") (name "SectionModulusUnit") (separator none) (span (offset 52151) (line 1112) (column 29) (len 18)))))
    (reference r668 (scope relative) (span (offset 52145) (line 1112) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 52145) (line 1112) (column 23) (len 4)))))
    (reference r669 (scope relative) (span (offset 52306) (line 1117) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 52306) (line 1117) (column 41) (len 11)))))
    (reference r670 (scope relative) (span (offset 52356) (line 1118) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 52356) (line 1118) (column 37) (len 19)))))
    (reference r671 (scope relative) (span (offset 52385) (line 1118) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 52385) (line 1118) (column 66) (len 8)))))
    (reference r672 (scope relative) (span (offset 52396) (line 1118) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 52396) (line 1118) (column 77) (len 3)))))
    (reference r673 (scope relative) (span (offset 52400) (line 1118) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 52400) (line 1118) (column 81) (len 1)))))
    (reference r674 (scope relative) (span (offset 52407) (line 1118) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 52407) (line 1118) (column 88) (len 8)))))
    (reference r675 (scope relative) (span (offset 52445) (line 1119) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 52445) (line 1119) (column 23) (len 17)))))
    (reference r676 (scope relative) (span (offset 52469) (line 1119) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 52469) (line 1119) (column 47) (len 20)))))
    (reference r677 (scope relative) (span (offset 52492) (line 1119) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 52492) (line 1119) (column 70) (len 8)))))
    (reference r678 (scope relative) (span (offset 52681) (line 1123) (column 53) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 52681) (line 1123) (column 53) (len 17)))))
    (reference r679 (scope relative) (span (offset 53719) (line 1139) (column 36) (len 25)) (segments (segment 0 (token "staticFrictionCoefficient") (name "staticFrictionCoefficient") (separator none) (span (offset 53719) (line 1139) (column 36) (len 25)))))
    (reference r680 (scope relative) (span (offset 53789) (line 1141) (column 43) (len 25)) (segments (segment 0 (token "staticFrictionCoefficient") (name "staticFrictionCoefficient") (separator none) (span (offset 53789) (line 1141) (column 43) (len 25)))))
    (reference r681 (scope relative) (span (offset 53948) (line 1144) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 53948) (line 1144) (column 49) (len 17)))))
    (reference r682 (scope relative) (span (offset 54914) (line 1160) (column 37) (len 21)) (segments (segment 0 (token "kineticFrictionFactor") (name "kineticFrictionFactor") (separator none) (span (offset 54914) (line 1160) (column 37) (len 21)))))
    (reference r683 (scope relative) (span (offset 55048) (line 1163) (column 51) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 55048) (line 1163) (column 51) (len 17)))))
    (reference r684 (scope relative) (span (offset 55905) (line 1180) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 55905) (line 1180) (column 43) (len 17)))))
    (reference r685 (scope relative) (span (offset 56705) (line 1196) (column 26) (len 15)) (segments (segment 0 (token "dragCoefficient") (name "dragCoefficient") (separator none) (span (offset 56705) (line 1196) (column 26) (len 15)))))
    (reference r686 (scope relative) (span (offset 56827) (line 1199) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 56827) (line 1199) (column 44) (len 19)))))
    (reference r687 (scope relative) (span (offset 57475) (line 1212) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 57475) (line 1212) (column 28) (len 4)))))
    (reference r688 (scope relative) (span (offset 57470) (line 1212) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 57470) (line 1212) (column 23) (len 3)))))
    (reference r689 (scope relative) (span (offset 57509) (line 1213) (column 29) (len 20)) (segments (segment 0 (token "DynamicViscosityUnit") (name "DynamicViscosityUnit") (separator none) (span (offset 57509) (line 1213) (column 29) (len 20)))))
    (reference r690 (scope relative) (span (offset 57503) (line 1213) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 57503) (line 1213) (column 23) (len 4)))))
    (reference r691 (scope relative) (span (offset 57672) (line 1218) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 57672) (line 1218) (column 43) (len 11)))))
    (reference r692 (scope relative) (span (offset 57722) (line 1219) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57722) (line 1219) (column 37) (len 19)))))
    (reference r693 (scope relative) (span (offset 57751) (line 1219) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57751) (line 1219) (column 66) (len 8)))))
    (reference r694 (scope relative) (span (offset 57762) (line 1219) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57762) (line 1219) (column 77) (len 3)))))
    (reference r695 (scope relative) (span (offset 57766) (line 1219) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 57766) (line 1219) (column 81) (len 1)))))
    (reference r696 (scope relative) (span (offset 57773) (line 1219) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57773) (line 1219) (column 88) (len 8)))))
    (reference r697 (scope relative) (span (offset 57824) (line 1220) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57824) (line 1220) (column 35) (len 19)))))
    (reference r698 (scope relative) (span (offset 57853) (line 1220) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57853) (line 1220) (column 64) (len 8)))))
    (reference r699 (scope relative) (span (offset 57864) (line 1220) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57864) (line 1220) (column 75) (len 3)))))
    (reference r700 (scope relative) (span (offset 57868) (line 1220) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 57868) (line 1220) (column 79) (len 1)))))
    (reference r701 (scope relative) (span (offset 57875) (line 1220) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57875) (line 1220) (column 86) (len 8)))))
    (reference r702 (scope relative) (span (offset 57929) (line 1221) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57929) (line 1221) (column 39) (len 19)))))
    (reference r703 (scope relative) (span (offset 57958) (line 1221) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57958) (line 1221) (column 68) (len 8)))))
    (reference r704 (scope relative) (span (offset 57969) (line 1221) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57969) (line 1221) (column 79) (len 3)))))
    (reference r705 (scope relative) (span (offset 57973) (line 1221) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 57973) (line 1221) (column 83) (len 1)))))
    (reference r706 (scope relative) (span (offset 57980) (line 1221) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57980) (line 1221) (column 90) (len 8)))))
    (reference r707 (scope relative) (span (offset 58019) (line 1222) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 58019) (line 1222) (column 23) (len 17)))))
    (reference r708 (scope relative) (span (offset 58043) (line 1222) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 58043) (line 1222) (column 47) (len 20)))))
    (reference r709 (scope relative) (span (offset 58067) (line 1222) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 58067) (line 1222) (column 71) (len 8)))))
    (reference r710 (scope relative) (span (offset 58077) (line 1222) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 58077) (line 1222) (column 81) (len 6)))))
    (reference r711 (scope relative) (span (offset 58085) (line 1222) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 58085) (line 1222) (column 89) (len 10)))))
    (reference r712 (scope relative) (span (offset 58135) (line 1225) (column 29) (len 20)) (segments (segment 0 (token "DynamicViscosityUnit") (name "DynamicViscosityUnit") (separator none) (span (offset 58135) (line 1225) (column 29) (len 20)))))
    (reference r713 (scope relative) (span (offset 58186) (line 1226) (column 30) (len 21)) (segments (segment 0 (token "DynamicViscosityValue") (name "DynamicViscosityValue") (separator none) (span (offset 58186) (line 1226) (column 30) (len 21)))))
    (reference r714 (scope relative) (span (offset 58233) (line 1227) (column 25) (len 16)) (segments (segment 0 (token "dynamicViscosity") (name "dynamicViscosity") (separator none) (span (offset 58233) (line 1227) (column 25) (len 16)))))
    (reference r715 (scope relative) (span (offset 58349) (line 1230) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 58349) (line 1230) (column 46) (len 19)))))
    (reference r716 (scope relative) (span (offset 58844) (line 1243) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 58844) (line 1243) (column 28) (len 4)))))
    (reference r717 (scope relative) (span (offset 58839) (line 1243) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 58839) (line 1243) (column 23) (len 3)))))
    (reference r718 (scope relative) (span (offset 58878) (line 1244) (column 29) (len 22)) (segments (segment 0 (token "KinematicViscosityUnit") (name "KinematicViscosityUnit") (separator none) (span (offset 58878) (line 1244) (column 29) (len 22)))))
    (reference r719 (scope relative) (span (offset 58872) (line 1244) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 58872) (line 1244) (column 23) (len 4)))))
    (reference r720 (scope relative) (span (offset 59049) (line 1249) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 59049) (line 1249) (column 45) (len 11)))))
    (reference r721 (scope relative) (span (offset 59099) (line 1250) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 59099) (line 1250) (column 37) (len 19)))))
    (reference r722 (scope relative) (span (offset 59128) (line 1250) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59128) (line 1250) (column 66) (len 8)))))
    (reference r723 (scope relative) (span (offset 59139) (line 1250) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59139) (line 1250) (column 77) (len 3)))))
    (reference r724 (scope relative) (span (offset 59143) (line 1250) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 59143) (line 1250) (column 81) (len 1)))))
    (reference r725 (scope relative) (span (offset 59150) (line 1250) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59150) (line 1250) (column 88) (len 8)))))
    (reference r726 (scope relative) (span (offset 59204) (line 1251) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 59204) (line 1251) (column 39) (len 19)))))
    (reference r727 (scope relative) (span (offset 59233) (line 1251) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59233) (line 1251) (column 68) (len 8)))))
    (reference r728 (scope relative) (span (offset 59244) (line 1251) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59244) (line 1251) (column 79) (len 3)))))
    (reference r729 (scope relative) (span (offset 59248) (line 1251) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 59248) (line 1251) (column 83) (len 1)))))
    (reference r730 (scope relative) (span (offset 59255) (line 1251) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59255) (line 1251) (column 90) (len 8)))))
    (reference r731 (scope relative) (span (offset 59294) (line 1252) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 59294) (line 1252) (column 23) (len 17)))))
    (reference r732 (scope relative) (span (offset 59318) (line 1252) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 59318) (line 1252) (column 47) (len 20)))))
    (reference r733 (scope relative) (span (offset 59342) (line 1252) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 59342) (line 1252) (column 71) (len 8)))))
    (reference r734 (scope relative) (span (offset 59352) (line 1252) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 59352) (line 1252) (column 81) (len 10)))))
    (reference r735 (scope relative) (span (offset 59463) (line 1256) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 59463) (line 1256) (column 42) (len 19)))))
    (reference r736 (scope relative) (span (offset 60072) (line 1269) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 60072) (line 1269) (column 28) (len 4)))))
    (reference r737 (scope relative) (span (offset 60067) (line 1269) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 60067) (line 1269) (column 23) (len 3)))))
    (reference r738 (scope relative) (span (offset 60106) (line 1270) (column 29) (len 18)) (segments (segment 0 (token "SurfaceTensionUnit") (name "SurfaceTensionUnit") (separator none) (span (offset 60106) (line 1270) (column 29) (len 18)))))
    (reference r739 (scope relative) (span (offset 60100) (line 1270) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 60100) (line 1270) (column 23) (len 4)))))
    (reference r740 (scope relative) (span (offset 60261) (line 1275) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 60261) (line 1275) (column 41) (len 11)))))
    (reference r741 (scope relative) (span (offset 60309) (line 1276) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 60309) (line 1276) (column 35) (len 19)))))
    (reference r742 (scope relative) (span (offset 60338) (line 1276) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 60338) (line 1276) (column 64) (len 8)))))
    (reference r743 (scope relative) (span (offset 60349) (line 1276) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 60349) (line 1276) (column 75) (len 3)))))
    (reference r744 (scope relative) (span (offset 60353) (line 1276) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 60353) (line 1276) (column 79) (len 1)))))
    (reference r745 (scope relative) (span (offset 60360) (line 1276) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 60360) (line 1276) (column 86) (len 8)))))
    (reference r746 (scope relative) (span (offset 60414) (line 1277) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 60414) (line 1277) (column 39) (len 19)))))
    (reference r747 (scope relative) (span (offset 60443) (line 1277) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 60443) (line 1277) (column 68) (len 8)))))
    (reference r748 (scope relative) (span (offset 60454) (line 1277) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 60454) (line 1277) (column 79) (len 3)))))
    (reference r749 (scope relative) (span (offset 60458) (line 1277) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 60458) (line 1277) (column 83) (len 1)))))
    (reference r750 (scope relative) (span (offset 60465) (line 1277) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 60465) (line 1277) (column 90) (len 8)))))
    (reference r751 (scope relative) (span (offset 60504) (line 1278) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 60504) (line 1278) (column 23) (len 17)))))
    (reference r752 (scope relative) (span (offset 60528) (line 1278) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 60528) (line 1278) (column 47) (len 20)))))
    (reference r753 (scope relative) (span (offset 60552) (line 1278) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 60552) (line 1278) (column 71) (len 6)))))
    (reference r754 (scope relative) (span (offset 60560) (line 1278) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 60560) (line 1278) (column 79) (len 10)))))
    (reference r755 (scope relative) (span (offset 60654) (line 1282) (column 33) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 60654) (line 1282) (column 33) (len 19)))))
    (reference r756 (scope relative) (span (offset 61098) (line 1295) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 61098) (line 1295) (column 28) (len 4)))))
    (reference r757 (scope relative) (span (offset 61093) (line 1295) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 61093) (line 1295) (column 23) (len 3)))))
    (reference r758 (scope relative) (span (offset 61132) (line 1296) (column 29) (len 9)) (segments (segment 0 (token "PowerUnit") (name "PowerUnit") (separator none) (span (offset 61132) (line 1296) (column 29) (len 9)))))
    (reference r759 (scope relative) (span (offset 61126) (line 1296) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 61126) (line 1296) (column 23) (len 4)))))
    (reference r760 (scope relative) (span (offset 61251) (line 1301) (column 32) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 61251) (line 1301) (column 32) (len 11)))))
    (reference r761 (scope relative) (span (offset 61301) (line 1302) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 61301) (line 1302) (column 37) (len 19)))))
    (reference r762 (scope relative) (span (offset 61330) (line 1302) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 61330) (line 1302) (column 66) (len 8)))))
    (reference r763 (scope relative) (span (offset 61341) (line 1302) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 61341) (line 1302) (column 77) (len 3)))))
    (reference r764 (scope relative) (span (offset 61345) (line 1302) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 61345) (line 1302) (column 81) (len 1)))))
    (reference r765 (scope relative) (span (offset 61352) (line 1302) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 61352) (line 1302) (column 88) (len 8)))))
    (reference r766 (scope relative) (span (offset 61402) (line 1303) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 61402) (line 1303) (column 35) (len 19)))))
    (reference r767 (scope relative) (span (offset 61431) (line 1303) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 61431) (line 1303) (column 64) (len 8)))))
    (reference r768 (scope relative) (span (offset 61442) (line 1303) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 61442) (line 1303) (column 75) (len 3)))))
    (reference r769 (scope relative) (span (offset 61446) (line 1303) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 61446) (line 1303) (column 79) (len 1)))))
    (reference r770 (scope relative) (span (offset 61453) (line 1303) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 61453) (line 1303) (column 86) (len 8)))))
    (reference r771 (scope relative) (span (offset 61507) (line 1304) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 61507) (line 1304) (column 39) (len 19)))))
    (reference r772 (scope relative) (span (offset 61536) (line 1304) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 61536) (line 1304) (column 68) (len 8)))))
    (reference r773 (scope relative) (span (offset 61547) (line 1304) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 61547) (line 1304) (column 79) (len 3)))))
    (reference r774 (scope relative) (span (offset 61551) (line 1304) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 61551) (line 1304) (column 83) (len 1)))))
    (reference r775 (scope relative) (span (offset 61558) (line 1304) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 61558) (line 1304) (column 90) (len 8)))))
    (reference r776 (scope relative) (span (offset 61597) (line 1305) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 61597) (line 1305) (column 23) (len 17)))))
    (reference r777 (scope relative) (span (offset 61621) (line 1305) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 61621) (line 1305) (column 47) (len 20)))))
    (reference r778 (scope relative) (span (offset 61645) (line 1305) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 61645) (line 1305) (column 71) (len 8)))))
    (reference r779 (scope relative) (span (offset 61655) (line 1305) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 61655) (line 1305) (column 81) (len 6)))))
    (reference r780 (scope relative) (span (offset 61663) (line 1305) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 61663) (line 1305) (column 89) (len 10)))))
    (reference r781 (scope relative) (span (offset 65568) (line 1388) (column 20) (len 14)) (segments (segment 0 (token "mechanicalWork") (name "mechanicalWork") (separator none) (span (offset 65568) (line 1388) (column 20) (len 14)))))
    (reference r782 (scope relative) (span (offset 65686) (line 1391) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 65686) (line 1391) (column 48) (len 17)))))
    (reference r783 (scope relative) (span (offset 66469) (line 1408) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 66469) (line 1408) (column 36) (len 19)))))
    (reference r784 (scope relative) (span (offset 67049) (line 1421) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 67049) (line 1421) (column 28) (len 4)))))
    (reference r785 (scope relative) (span (offset 67044) (line 1421) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 67044) (line 1421) (column 23) (len 3)))))
    (reference r786 (scope relative) (span (offset 67083) (line 1422) (column 29) (len 12)) (segments (segment 0 (token "MassFlowUnit") (name "MassFlowUnit") (separator none) (span (offset 67083) (line 1422) (column 29) (len 12)))))
    (reference r787 (scope relative) (span (offset 67077) (line 1422) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 67077) (line 1422) (column 23) (len 4)))))
    (reference r788 (scope relative) (span (offset 67214) (line 1427) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 67214) (line 1427) (column 35) (len 11)))))
    (reference r789 (scope relative) (span (offset 67264) (line 1428) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 67264) (line 1428) (column 37) (len 19)))))
    (reference r790 (scope relative) (span (offset 67293) (line 1428) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 67293) (line 1428) (column 66) (len 8)))))
    (reference r791 (scope relative) (span (offset 67304) (line 1428) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 67304) (line 1428) (column 77) (len 3)))))
    (reference r792 (scope relative) (span (offset 67308) (line 1428) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 67308) (line 1428) (column 81) (len 1)))))
    (reference r793 (scope relative) (span (offset 67315) (line 1428) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 67315) (line 1428) (column 88) (len 8)))))
    (reference r794 (scope relative) (span (offset 67366) (line 1429) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 67366) (line 1429) (column 35) (len 19)))))
    (reference r795 (scope relative) (span (offset 67395) (line 1429) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 67395) (line 1429) (column 64) (len 8)))))
    (reference r796 (scope relative) (span (offset 67406) (line 1429) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 67406) (line 1429) (column 75) (len 3)))))
    (reference r797 (scope relative) (span (offset 67410) (line 1429) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 67410) (line 1429) (column 79) (len 1)))))
    (reference r798 (scope relative) (span (offset 67417) (line 1429) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 67417) (line 1429) (column 86) (len 8)))))
    (reference r799 (scope relative) (span (offset 67471) (line 1430) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 67471) (line 1430) (column 39) (len 19)))))
    (reference r800 (scope relative) (span (offset 67500) (line 1430) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 67500) (line 1430) (column 68) (len 8)))))
    (reference r801 (scope relative) (span (offset 67511) (line 1430) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 67511) (line 1430) (column 79) (len 3)))))
    (reference r802 (scope relative) (span (offset 67515) (line 1430) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 67515) (line 1430) (column 83) (len 1)))))
    (reference r803 (scope relative) (span (offset 67522) (line 1430) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 67522) (line 1430) (column 90) (len 8)))))
    (reference r804 (scope relative) (span (offset 67561) (line 1431) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 67561) (line 1431) (column 23) (len 17)))))
    (reference r805 (scope relative) (span (offset 67585) (line 1431) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 67585) (line 1431) (column 47) (len 20)))))
    (reference r806 (scope relative) (span (offset 67609) (line 1431) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 67609) (line 1431) (column 71) (len 8)))))
    (reference r807 (scope relative) (span (offset 67619) (line 1431) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 67619) (line 1431) (column 81) (len 6)))))
    (reference r808 (scope relative) (span (offset 67627) (line 1431) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 67627) (line 1431) (column 89) (len 10)))))
    (reference r809 (scope relative) (span (offset 67696) (line 1434) (column 48) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 67696) (line 1434) (column 48) (len 23)))))
    (reference r810 (scope relative) (span (offset 68277) (line 1447) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 68277) (line 1447) (column 23) (len 7)))))
    (reference r811 (scope relative) (span (offset 68322) (line 1448) (column 29) (len 34)) (segments (segment 0 (token "CartesianMassFlow3dCoordinateFrame") (name "CartesianMassFlow3dCoordinateFrame") (separator none) (span (offset 68322) (line 1448) (column 29) (len 34)))))
    (reference r812 (scope relative) (span (offset 68316) (line 1448) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 68316) (line 1448) (column 23) (len 4)))))
    (reference r813 (scope relative) (span (offset 68513) (line 1453) (column 57) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 68513) (line 1453) (column 57) (len 19)))))
    (reference r814 (scope relative) (span (offset 68557) (line 1454) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 68557) (line 1454) (column 23) (len 7)))))
    (reference r815 (scope relative) (span (offset 68596) (line 1455) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 68596) (line 1455) (column 23) (len 12)))))
    (reference r816 (scope relative) (span (offset 68646) (line 1456) (column 30) (len 12)) (segments (segment 0 (token "MassFlowUnit") (name "MassFlowUnit") (separator none) (span (offset 68646) (line 1456) (column 30) (len 12)))))
    (reference r817 (scope relative) (span (offset 68639) (line 1456) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 68639) (line 1456) (column 23) (len 5)))))
    (reference r818 (scope relative) (span (offset 68758) (line 1460) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 68758) (line 1460) (column 40) (len 19)))))
    (reference r819 (scope relative) (span (offset 69471) (line 1473) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 69471) (line 1473) (column 28) (len 4)))))
    (reference r820 (scope relative) (span (offset 69466) (line 1473) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 69466) (line 1473) (column 23) (len 3)))))
    (reference r821 (scope relative) (span (offset 69505) (line 1474) (column 29) (len 16)) (segments (segment 0 (token "MassFlowRateUnit") (name "MassFlowRateUnit") (separator none) (span (offset 69505) (line 1474) (column 29) (len 16)))))
    (reference r822 (scope relative) (span (offset 69499) (line 1474) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 69499) (line 1474) (column 23) (len 4)))))
    (reference r823 (scope relative) (span (offset 69652) (line 1479) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 69652) (line 1479) (column 39) (len 11)))))
    (reference r824 (scope relative) (span (offset 69700) (line 1480) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 69700) (line 1480) (column 35) (len 19)))))
    (reference r825 (scope relative) (span (offset 69729) (line 1480) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 69729) (line 1480) (column 64) (len 8)))))
    (reference r826 (scope relative) (span (offset 69740) (line 1480) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 69740) (line 1480) (column 75) (len 3)))))
    (reference r827 (scope relative) (span (offset 69744) (line 1480) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 69744) (line 1480) (column 79) (len 1)))))
    (reference r828 (scope relative) (span (offset 69751) (line 1480) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 69751) (line 1480) (column 86) (len 8)))))
    (reference r829 (scope relative) (span (offset 69805) (line 1481) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 69805) (line 1481) (column 39) (len 19)))))
    (reference r830 (scope relative) (span (offset 69834) (line 1481) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 69834) (line 1481) (column 68) (len 8)))))
    (reference r831 (scope relative) (span (offset 69845) (line 1481) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 69845) (line 1481) (column 79) (len 3)))))
    (reference r832 (scope relative) (span (offset 69849) (line 1481) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 69849) (line 1481) (column 83) (len 1)))))
    (reference r833 (scope relative) (span (offset 69856) (line 1481) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 69856) (line 1481) (column 90) (len 8)))))
    (reference r834 (scope relative) (span (offset 69895) (line 1482) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 69895) (line 1482) (column 23) (len 17)))))
    (reference r835 (scope relative) (span (offset 69919) (line 1482) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 69919) (line 1482) (column 47) (len 20)))))
    (reference r836 (scope relative) (span (offset 69943) (line 1482) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 69943) (line 1482) (column 71) (len 6)))))
    (reference r837 (scope relative) (span (offset 69951) (line 1482) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 69951) (line 1482) (column 79) (len 10)))))
    (reference r838 (scope relative) (span (offset 70065) (line 1486) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 70065) (line 1486) (column 42) (len 19)))))
    (reference r839 (scope relative) (span (offset 70623) (line 1499) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 70623) (line 1499) (column 28) (len 4)))))
    (reference r840 (scope relative) (span (offset 70618) (line 1499) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 70618) (line 1499) (column 23) (len 3)))))
    (reference r841 (scope relative) (span (offset 70657) (line 1500) (column 29) (len 18)) (segments (segment 0 (token "MassChangeRateUnit") (name "MassChangeRateUnit") (separator none) (span (offset 70657) (line 1500) (column 29) (len 18)))))
    (reference r842 (scope relative) (span (offset 70651) (line 1500) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 70651) (line 1500) (column 23) (len 4)))))
    (reference r843 (scope relative) (span (offset 70812) (line 1505) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 70812) (line 1505) (column 41) (len 11)))))
    (reference r844 (scope relative) (span (offset 70860) (line 1506) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 70860) (line 1506) (column 35) (len 19)))))
    (reference r845 (scope relative) (span (offset 70889) (line 1506) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 70889) (line 1506) (column 64) (len 8)))))
    (reference r846 (scope relative) (span (offset 70900) (line 1506) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 70900) (line 1506) (column 75) (len 3)))))
    (reference r847 (scope relative) (span (offset 70904) (line 1506) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 70904) (line 1506) (column 79) (len 1)))))
    (reference r848 (scope relative) (span (offset 70911) (line 1506) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 70911) (line 1506) (column 86) (len 8)))))
    (reference r849 (scope relative) (span (offset 70965) (line 1507) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 70965) (line 1507) (column 39) (len 19)))))
    (reference r850 (scope relative) (span (offset 70994) (line 1507) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 70994) (line 1507) (column 68) (len 8)))))
    (reference r851 (scope relative) (span (offset 71005) (line 1507) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 71005) (line 1507) (column 79) (len 3)))))
    (reference r852 (scope relative) (span (offset 71009) (line 1507) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 71009) (line 1507) (column 83) (len 1)))))
    (reference r853 (scope relative) (span (offset 71016) (line 1507) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 71016) (line 1507) (column 90) (len 8)))))
    (reference r854 (scope relative) (span (offset 71055) (line 1508) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 71055) (line 1508) (column 23) (len 17)))))
    (reference r855 (scope relative) (span (offset 71079) (line 1508) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 71079) (line 1508) (column 47) (len 20)))))
    (reference r856 (scope relative) (span (offset 71103) (line 1508) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 71103) (line 1508) (column 71) (len 6)))))
    (reference r857 (scope relative) (span (offset 71111) (line 1508) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 71111) (line 1508) (column 79) (len 10)))))
    (reference r858 (scope relative) (span (offset 71223) (line 1512) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 71223) (line 1512) (column 42) (len 19)))))
    (reference r859 (scope relative) (span (offset 71938) (line 1525) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 71938) (line 1525) (column 28) (len 4)))))
    (reference r860 (scope relative) (span (offset 71933) (line 1525) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 71933) (line 1525) (column 23) (len 3)))))
    (reference r861 (scope relative) (span (offset 71972) (line 1526) (column 29) (len 18)) (segments (segment 0 (token "VolumeFlowRateUnit") (name "VolumeFlowRateUnit") (separator none) (span (offset 71972) (line 1526) (column 29) (len 18)))))
    (reference r862 (scope relative) (span (offset 71966) (line 1526) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 71966) (line 1526) (column 23) (len 4)))))
    (reference r863 (scope relative) (span (offset 72127) (line 1531) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 72127) (line 1531) (column 41) (len 11)))))
    (reference r864 (scope relative) (span (offset 72177) (line 1532) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 72177) (line 1532) (column 37) (len 19)))))
    (reference r865 (scope relative) (span (offset 72206) (line 1532) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 72206) (line 1532) (column 66) (len 8)))))
    (reference r866 (scope relative) (span (offset 72217) (line 1532) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 72217) (line 1532) (column 77) (len 3)))))
    (reference r867 (scope relative) (span (offset 72221) (line 1532) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 72221) (line 1532) (column 81) (len 1)))))
    (reference r868 (scope relative) (span (offset 72228) (line 1532) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 72228) (line 1532) (column 88) (len 8)))))
    (reference r869 (scope relative) (span (offset 72282) (line 1533) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 72282) (line 1533) (column 39) (len 19)))))
    (reference r870 (scope relative) (span (offset 72311) (line 1533) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 72311) (line 1533) (column 68) (len 8)))))
    (reference r871 (scope relative) (span (offset 72322) (line 1533) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 72322) (line 1533) (column 79) (len 3)))))
    (reference r872 (scope relative) (span (offset 72326) (line 1533) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 72326) (line 1533) (column 83) (len 1)))))
    (reference r873 (scope relative) (span (offset 72333) (line 1533) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 72333) (line 1533) (column 90) (len 8)))))
    (reference r874 (scope relative) (span (offset 72372) (line 1534) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 72372) (line 1534) (column 23) (len 17)))))
    (reference r875 (scope relative) (span (offset 72396) (line 1534) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 72396) (line 1534) (column 47) (len 20)))))
    (reference r876 (scope relative) (span (offset 72420) (line 1534) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 72420) (line 1534) (column 71) (len 8)))))
    (reference r877 (scope relative) (span (offset 72430) (line 1534) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 72430) (line 1534) (column 81) (len 10)))))
    (reference r878 (scope relative) (span (offset 72541) (line 1538) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 72541) (line 1538) (column 42) (len 19)))))
    (reference r879 (scope relative) (span (offset 73224) (line 1551) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 73224) (line 1551) (column 28) (len 4)))))
    (reference r880 (scope relative) (span (offset 73219) (line 1551) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 73219) (line 1551) (column 23) (len 3)))))
    (reference r881 (scope relative) (span (offset 73258) (line 1552) (column 29) (len 18)) (segments (segment 0 (token "ActionQuantityUnit") (name "ActionQuantityUnit") (separator none) (span (offset 73258) (line 1552) (column 29) (len 18)))))
    (reference r882 (scope relative) (span (offset 73252) (line 1552) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 73252) (line 1552) (column 23) (len 4)))))
    (reference r883 (scope relative) (span (offset 73413) (line 1557) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 73413) (line 1557) (column 41) (len 11)))))
    (reference r884 (scope relative) (span (offset 73463) (line 1558) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 73463) (line 1558) (column 37) (len 19)))))
    (reference r885 (scope relative) (span (offset 73492) (line 1558) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 73492) (line 1558) (column 66) (len 8)))))
    (reference r886 (scope relative) (span (offset 73503) (line 1558) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 73503) (line 1558) (column 77) (len 3)))))
    (reference r887 (scope relative) (span (offset 73507) (line 1558) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 73507) (line 1558) (column 81) (len 1)))))
    (reference r888 (scope relative) (span (offset 73514) (line 1558) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 73514) (line 1558) (column 88) (len 8)))))
    (reference r889 (scope relative) (span (offset 73564) (line 1559) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 73564) (line 1559) (column 35) (len 19)))))
    (reference r890 (scope relative) (span (offset 73593) (line 1559) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 73593) (line 1559) (column 64) (len 8)))))
    (reference r891 (scope relative) (span (offset 73604) (line 1559) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 73604) (line 1559) (column 75) (len 3)))))
    (reference r892 (scope relative) (span (offset 73608) (line 1559) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 73608) (line 1559) (column 79) (len 1)))))
    (reference r893 (scope relative) (span (offset 73615) (line 1559) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 73615) (line 1559) (column 86) (len 8)))))
    (reference r894 (scope relative) (span (offset 73669) (line 1560) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 73669) (line 1560) (column 39) (len 19)))))
    (reference r895 (scope relative) (span (offset 73698) (line 1560) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 73698) (line 1560) (column 68) (len 8)))))
    (reference r896 (scope relative) (span (offset 73709) (line 1560) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 73709) (line 1560) (column 79) (len 3)))))
    (reference r897 (scope relative) (span (offset 73713) (line 1560) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 73713) (line 1560) (column 83) (len 1)))))
    (reference r898 (scope relative) (span (offset 73720) (line 1560) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 73720) (line 1560) (column 90) (len 8)))))
    (reference r899 (scope relative) (span (offset 73759) (line 1561) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 73759) (line 1561) (column 23) (len 17)))))
    (reference r900 (scope relative) (span (offset 73783) (line 1561) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 73783) (line 1561) (column 47) (len 20)))))
    (reference r901 (scope relative) (span (offset 73807) (line 1561) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 73807) (line 1561) (column 71) (len 8)))))
    (reference r902 (scope relative) (span (offset 73817) (line 1561) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 73817) (line 1561) (column 81) (len 6)))))
    (reference r903 (scope relative) (span (offset 73825) (line 1561) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 73825) (line 1561) (column 89) (len 10)))))
  )
  (root (library-package (name "ISQMechanics") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 54) (line 3) (column 7) (len 702)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO-80000-4:2019 \"Mechanics\"\nsee also https://www.iso.org/standard/64975.html\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\nNote 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \ndefined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \nor TensorMeasurementReference.\n"))) (import (target (span (span (offset 779) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 818) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 828) (line 16) (column 30) (len 3))) (separator (span (offset 828) (line 16) (column 30) (len 2))) (marker (span (offset 830) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 852) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 873) (line 17) (column 41) (len 3))) (separator (span (offset 873) (line 17) (column 41) (len 2))) (marker (span (offset 875) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 897) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 904) (line 18) (column 27) (len 3))) (separator (span (offset 904) (line 18) (column 27) (len 2))) (marker (span (offset 906) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 916) (line 20) (column 7) (len 57)) (normalized "Quantity definitions referenced from other ISQ packages "))) (import (target (span (span (offset 995) (line 21) (column 20) (len 30))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1034) (line 23) (column 7) (len 27)) (normalized "ISO-80000-4 item 4-1 mass "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1070) (line 24) (column 7) (len 68)) (normalized "See package ISQBase for the declarations of MassValue and MassUnit "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1148) (line 26) (column 7) (len 44)) (normalized "ISO-80000-4 item 4-2 mass density, density "))) (attribute-def (declaration-name "MassDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1277) (line 29) (column 11) (len 547)) (normalized "source: item 4-2 mass density, density\nsymbol(s): `ρ`, `ρ_m`\napplication domain: generic\nname: MassDensity\nquantity dimension: L^-3*M^1\nmeasurement unit(s): kg*m^-3\ntensor order: 0\ndefinition: quantity representing the spatial distribution of mass of a continuous material: `ρ(vec(r)) = (dm)/(dV)` where `m` is mass of the material contained in an infinitesimal domain at point `vec(r)` and `V` is volume of this domain\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2121) (line 47) (column 77) (len 5)) (member-access (base (expression (span (offset 2121) (line 47) (column 77) (len 3)) (ref r13))) (separator dot) (member (ref r14))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2143) (line 47) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 2144) (line 47) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2223) (line 48) (column 75) (len 5)) (member-access (base (expression (span (offset 2223) (line 48) (column 75) (len 3)) (ref r18))) (separator dot) (member (ref r19))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2245) (line 48) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2319) (line 49) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 2320) (line 49) (column 71) (len 8)) (ref r23))) (element comma (expression (span (offset 2330) (line 49) (column 81) (len 6)) (ref r24))))))))) (body semicolon)))))) (alias (name "DensityUnit") (target (ref r25)) (body semicolon)) (alias (name "DensityValue") (target (ref r26)) (body semicolon)) (alias (name "density") (target (ref r27)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2478) (line 56) (column 7) (len 38)) (normalized "ISO-80000-4 item 4-3 specific volume "))) (attribute-def (declaration-name "SpecificVolumeValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2604) (line 59) (column 11) (len 364)) (normalized "source: item 4-3 specific volume\nsymbol(s): `v`\napplication domain: generic\nname: SpecificVolume\nquantity dimension: L^3*M^-1\nmeasurement unit(s): kg^-1*m^3\ntensor order: 0\ndefinition: reciprocal of mass density `ρ` (item 4-2): `v = 1/ρ`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SpecificVolumeUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3277) (line 77) (column 77) (len 5)) (member-access (base (expression (span (offset 3277) (line 77) (column 77) (len 3)) (ref r36))) (separator dot) (member (ref r37))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3299) (line 77) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3378) (line 78) (column 75) (len 5)) (member-access (base (expression (span (offset 3378) (line 78) (column 75) (len 3)) (ref r41))) (separator dot) (member (ref r42))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3400) (line 78) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 3401) (line 78) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r44)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3475) (line 79) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 3476) (line 79) (column 71) (len 8)) (ref r46))) (element comma (expression (span (offset 3486) (line 79) (column 81) (len 6)) (ref r47))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3510) (line 82) (column 7) (len 62)) (normalized "ISO-80000-4 item 4-4 relative mass density, relative density "))) (attribute-def (declaration-name "RelativeMassDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r48)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3663) (line 85) (column 11) (len 535)) (normalized "source: item 4-4 relative mass density, relative density\nsymbol(s): `d`\napplication domain: generic\nname: RelativeMassDensity (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mass density of a substance `ρ` and mass density of a reference substance `ρ_0` : `d = ρ/ρ_0`\nremarks: Conditions and material should be specified for the reference substance.\n"))))) (attribute-usage) (alias (name "relativeDensity") (target (ref r49)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4347) (line 101) (column 7) (len 60)) (normalized "ISO-80000-4 item 4-5 surface mass density, surface density "))) (attribute-def (declaration-name "SurfaceMassDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r50)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4499) (line 104) (column 11) (len 569)) (normalized "source: item 4-5 surface mass density, surface density\nsymbol(s): `ρ_A`\napplication domain: generic\nname: SurfaceMassDensity\nquantity dimension: L^-2*M^1\nmeasurement unit(s): kg*m^-2\ntensor order: 0\ndefinition: quantity representing the areal distribution of mass of a continuous material: `ρ_A(vec(r)) = (dm)/(dA)` where `m` is the mass of the material at position `vec(r)` and `A` is area\nremarks: The name \"grammage\" should not be used for this quantity.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r54)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SurfaceMassDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r55)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r56)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r57)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5393) (line 122) (column 77) (len 5)) (member-access (base (expression (span (offset 5393) (line 122) (column 77) (len 3)) (ref r58))) (separator dot) (member (ref r59))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5415) (line 122) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 5416) (line 122) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r62)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5495) (line 123) (column 75) (len 5)) (member-access (base (expression (span (offset 5495) (line 123) (column 75) (len 3)) (ref r63))) (separator dot) (member (ref r64))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5517) (line 123) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r67)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5591) (line 124) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 5592) (line 124) (column 71) (len 8)) (ref r68))) (element comma (expression (span (offset 5602) (line 124) (column 81) (len 6)) (ref r69))))))))) (body semicolon)))))) (alias (name "SurfaceDensityUnit") (target (ref r70)) (body semicolon)) (alias (name "SurfaceDensityValue") (target (ref r71)) (body semicolon)) (alias (name "surfaceDensity") (target (ref r72)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5792) (line 131) (column 7) (len 58)) (normalized "ISO-80000-4 item 4-6 linear mass density, linear density "))) (attribute-def (declaration-name "LinearMassDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5941) (line 134) (column 11) (len 517)) (normalized "source: item 4-6 linear mass density, linear density\nsymbol(s): `ρ_I`\napplication domain: generic\nname: LinearMassDensity\nquantity dimension: L^-1*M^1\nmeasurement unit(s): kg*m^-1\ntensor order: 0\ndefinition: quantity representing the linear distribution of mass of a continuous material: `ρ_I(vec(r)) = (dm)/(dI)` where `m` is the mass of the material at position `vec(r)` and `l` is length\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r74)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r75)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r76)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r77)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LinearMassDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r78)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r80)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6779) (line 152) (column 77) (len 5)) (member-access (base (expression (span (offset 6779) (line 152) (column 77) (len 3)) (ref r81))) (separator dot) (member (ref r82))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r83)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6801) (line 152) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 6802) (line 152) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r84)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6881) (line 153) (column 75) (len 5)) (member-access (base (expression (span (offset 6881) (line 153) (column 75) (len 3)) (ref r86))) (separator dot) (member (ref r87))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r88)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6903) (line 153) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r90)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6977) (line 154) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 6978) (line 154) (column 71) (len 8)) (ref r91))) (element comma (expression (span (offset 6988) (line 154) (column 81) (len 6)) (ref r92))))))))) (body semicolon)))))) (alias (name "LinearDensityUnit") (target (ref r93)) (body semicolon)) (alias (name "LinearDensityValue") (target (ref r94)) (body semicolon)) (alias (name "linearDensity") (target (ref r95)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7172) (line 161) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-7 moment of inertia "))) (attribute-def (declaration-name "MomentOfInertiaValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r96)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7301) (line 164) (column 11) (len 692)) (normalized "source: item 4-7 moment of inertia (magnitude)\nsymbol(s): `J`\napplication domain: generic\nname: MomentOfInertia\nquantity dimension: L^2*M^1\nmeasurement unit(s): kg*m^2\ntensor order: 0\ndefinition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)\nremarks: The calculation of the value requires an integration.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r97)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r98)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r99)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MomentOfInertiaUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r101)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r102)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r103)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8306) (line 182) (column 77) (len 5)) (member-access (base (expression (span (offset 8306) (line 182) (column 77) (len 3)) (ref r104))) (separator dot) (member (ref r105))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r106)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8328) (line 182) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r107)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r108)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8407) (line 183) (column 75) (len 5)) (member-access (base (expression (span (offset 8407) (line 183) (column 75) (len 3)) (ref r109))) (separator dot) (member (ref r110))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r111)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8429) (line 183) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r112)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r113)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8503) (line 184) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 8504) (line 184) (column 71) (len 8)) (ref r114))) (element comma (expression (span (offset 8514) (line 184) (column 81) (len 6)) (ref r115))))))))) (body semicolon)))))) (attribute-def (declaration-name "Cartesian3dMomentOfInertiaTensor") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r116)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8630) (line 189) (column 11) (len 699)) (normalized "source: item 4-7 moment of inertia (tensor)\nsymbol(s): `vec(vec(J))`\napplication domain: generic\nname: MomentOfInertia\nquantity dimension: L^2*M^1\nmeasurement unit(s): kg*m^2\ntensor order: 2\ndefinition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)\nremarks: The calculation of the value requires an integration.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r117)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9364) (line 200) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r118)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r119)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r120)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r121)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "Cartesian3dMomentOfInertiaMeasurementReference") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r122)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r123)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9717) (line 208) (column 36) (len 6)) (sequence (sequence-list (element first (expression (span (offset 9718) (line 208) (column 37) (len 1)) (integer 3))) (element comma (expression (span (offset 9721) (line 208) (column 40) (len 1)) (integer 3))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r124)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9757) (line 209) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r125)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r126)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9830) (line 213) (column 7) (len 31)) (normalized "ISO-80000-4 item 4-8 momentum "))) (attribute-def (declaration-name "MomentumValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r127)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9943) (line 216) (column 11) (len 433)) (normalized "source: item 4-8 momentum (magnitude)\nsymbol(s): `p`\napplication domain: generic\nname: Momentum\nquantity dimension: L^1*M^1*T^-1\nmeasurement unit(s): kg*m*s^-1\ntensor order: 0\ndefinition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r128)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r129)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r130)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r131)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MomentumUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r132)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r133)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r134)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10661) (line 234) (column 77) (len 5)) (member-access (base (expression (span (offset 10661) (line 234) (column 77) (len 3)) (ref r135))) (separator dot) (member (ref r136))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r137)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10683) (line 234) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r138)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r139)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10762) (line 235) (column 75) (len 5)) (member-access (base (expression (span (offset 10762) (line 235) (column 75) (len 3)) (ref r140))) (separator dot) (member (ref r141))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r142)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10784) (line 235) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r143)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r144)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10867) (line 236) (column 79) (len 5)) (member-access (base (expression (span (offset 10867) (line 236) (column 79) (len 3)) (ref r145))) (separator dot) (member (ref r146))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10889) (line 236) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 10890) (line 236) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r148)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r149)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10964) (line 237) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 10965) (line 237) (column 71) (len 8)) (ref r150))) (element comma (expression (span (offset 10975) (line 237) (column 81) (len 6)) (ref r151))) (element comma (expression (span (offset 10983) (line 237) (column 89) (len 10)) (ref r152))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianMomentum3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r153)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 11100) (line 242) (column 11) (len 435)) (normalized "source: item 4-8 momentum (vector)\nsymbol(s): `vec(p)`\napplication domain: generic\nname: Momentum\nquantity dimension: L^1*M^1*T^-1\nmeasurement unit(s): kg*m*s^-1\ntensor order: 1\ndefinition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r154)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11570) (line 253) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r155)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r156)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianMomentum3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r157)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r158)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11850) (line 260) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r159)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11894) (line 261) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r160)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r161)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 11959) (line 265) (column 7) (len 30)) (normalized "ISO-80000-4 item 4-9.1 force "))) (attribute-def (declaration-name "ForceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r162)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12068) (line 268) (column 11) (len 392)) (normalized "source: item 4-9.1 force (magnitude)\nsymbol(s): `F`\napplication domain: generic\nname: Force\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity describing interaction between bodies or particles\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r163)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r164)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r165)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r166)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ForceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r167)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r168)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r169)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12733) (line 286) (column 77) (len 5)) (member-access (base (expression (span (offset 12733) (line 286) (column 77) (len 3)) (ref r170))) (separator dot) (member (ref r171))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r172)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12755) (line 286) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r173)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r174)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12834) (line 287) (column 75) (len 5)) (member-access (base (expression (span (offset 12834) (line 287) (column 75) (len 3)) (ref r175))) (separator dot) (member (ref r176))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r177)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12856) (line 287) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r178)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r179)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12939) (line 288) (column 79) (len 5)) (member-access (base (expression (span (offset 12939) (line 288) (column 79) (len 3)) (ref r180))) (separator dot) (member (ref r181))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r182)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12961) (line 288) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 12962) (line 288) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r183)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r184)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13036) (line 289) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 13037) (line 289) (column 71) (len 8)) (ref r185))) (element comma (expression (span (offset 13047) (line 289) (column 81) (len 6)) (ref r186))) (element comma (expression (span (offset 13055) (line 289) (column 89) (len 10)) (ref r187))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianForce3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r188)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 13169) (line 294) (column 11) (len 394)) (normalized "source: item 4-9.1 force (vector)\nsymbol(s): `vec(F)`\napplication domain: generic\nname: Force\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity describing interaction between bodies or particles\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r189)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13598) (line 305) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r190)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r191)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianForce3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r192)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r193)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13866) (line 312) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r194)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13910) (line 313) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r195)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r196)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 13972) (line 317) (column 7) (len 31)) (normalized "ISO-80000-4 item 4-9.2 weight "))) (attribute-def (declaration-name "CartesianWeight3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r197)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14099) (line 320) (column 11) (len 821)) (normalized "source: item 4-9.2 weight\nsymbol(s): `vec(F_g)`\napplication domain: generic\nname: Weight (specializes Force)\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: force (item 4-9.1) acting on a body in the gravitational field of Earth: `vec(F_g) = m vec(g)` where `m` (item 4-1) is the mass of the body and `vec(g)` is the local acceleration of free fall (ISO 80000-3)\nremarks: In colloquial language, the name \"weight\" continues to be used where \"mass\" is meant. This practice should be avoided. Weight is an example of a gravitational force. Weight comprises not only the local gravitational force but also the local centrifugal force due to the rotation of the Earth.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r198)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14955) (line 331) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r199)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r200)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 15124) (line 337) (column 7) (len 63)) (normalized "ISO-80000-4 item 4-9.3 static friction force, static friction "))) (attribute-def (declaration-name "CartesianStaticFrictionForce3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r201)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15296) (line 340) (column 11) (len 504)) (normalized "source: item 4-9.3 static friction force, static friction\nsymbol(s): `vec(F_s)`\napplication domain: generic\nname: StaticFrictionForce (specializes Force)\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: force (item 4-9.1) resisting the motion before a body starts to slide on a surface\nremarks: For the static friction coefficient, see item 4-23.1.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15835) (line 351) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r203)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r204)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (alias (name "cartesianStaticFriction3dVector") (target (ref r205)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 16115) (line 359) (column 7) (len 71)) (normalized "ISO-80000-4 item 4-9.4 kinetic friction force, dynamic friction force "))) (attribute-def (declaration-name "CartesianKineticFrictionForce3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r206)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16296) (line 362) (column 11) (len 499)) (normalized "source: item 4-9.4 kinetic friction force, dynamic friction force\nsymbol(s): `vec(F_μ)`\napplication domain: generic\nname: KineticFrictionForce (specializes Force)\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: force (item 4-9.1) resisting the motion when a body slides on a surface\nremarks: For the kinetic friction factor, see item 4-23.2.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r207)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16830) (line 373) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r208)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r209)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (alias (name "cartesianDynamicFrictionForce3dVector") (target (ref r210)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 17119) (line 381) (column 7) (len 81)) (normalized "ISO-80000-4 item 4-9.5 rolling resistance, rolling drag, rolling friction force "))) (attribute-def (declaration-name "CartesianRollingResistance3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r211)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 17307) (line 384) (column 11) (len 509)) (normalized "source: item 4-9.5 rolling resistance, rolling drag, rolling friction force\nsymbol(s): `vec(F_\"rr\")`\napplication domain: generic\nname: RollingResistance (specializes Force)\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: force (item 4-9.1) resisting the motion when a body rolls on a surface\nremarks: For the rolling resistance factor, see item 4-23.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r212)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17851) (line 395) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r213)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r214)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (alias (name "cartesianRollingDrag3dVector") (target (ref r215)) (body semicolon)) (alias (name "cartesianRollingFrictionForce3dVector") (target (ref r216)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 18211) (line 405) (column 7) (len 35)) (normalized "ISO-80000-4 item 4-9.6 drag force "))) (attribute-def (declaration-name "CartesianDragForce3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r217)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 18345) (line 408) (column 11) (len 433)) (normalized "source: item 4-9.6 drag force\nsymbol(s): `vec(F_D)`\napplication domain: generic\nname: DragForce (specializes Force)\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): N, kg*m*s^-2\ntensor order: 1\ndefinition: force (item 4-9.1) resisting the motion of a body in a fluid\nremarks: For the drag coefficient, see item 4-23.4.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r218)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18813) (line 419) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r219)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r220)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 18988) (line 425) (column 7) (len 31)) (normalized "ISO-80000-4 item 4-10 impulse "))) (attribute-def (declaration-name "ImpulseValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r221)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19100) (line 428) (column 11) (len 679)) (normalized "source: item 4-10 impulse (magnitude)\nsymbol(s): `I`\napplication domain: generic\nname: Impulse\nquantity dimension: L^1*M^1*T^-1\nmeasurement unit(s): N*s, kg*m*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval\nremarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r222)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r223)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r224)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r225)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ImpulseUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r226)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r227)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r228)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20060) (line 446) (column 77) (len 5)) (member-access (base (expression (span (offset 20060) (line 446) (column 77) (len 3)) (ref r229))) (separator dot) (member (ref r230))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r231)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20082) (line 446) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r232)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r233)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20161) (line 447) (column 75) (len 5)) (member-access (base (expression (span (offset 20161) (line 447) (column 75) (len 3)) (ref r234))) (separator dot) (member (ref r235))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r236)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20183) (line 447) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r237)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r238)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20266) (line 448) (column 79) (len 5)) (member-access (base (expression (span (offset 20266) (line 448) (column 79) (len 3)) (ref r239))) (separator dot) (member (ref r240))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r241)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20288) (line 448) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 20289) (line 448) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r242)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r243)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20363) (line 449) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 20364) (line 449) (column 71) (len 8)) (ref r244))) (element comma (expression (span (offset 20374) (line 449) (column 81) (len 6)) (ref r245))) (element comma (expression (span (offset 20382) (line 449) (column 89) (len 10)) (ref r246))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianImpulse3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r247)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 20498) (line 454) (column 11) (len 681)) (normalized "source: item 4-10 impulse (vector)\nsymbol(s): `vec(I)`\napplication domain: generic\nname: Impulse\nquantity dimension: L^1*M^1*T^-1\nmeasurement unit(s): N*s, kg*m*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval\nremarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r248)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21214) (line 465) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r249)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r250)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianImpulse3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r251)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r252)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21490) (line 472) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r253)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21534) (line 473) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r254)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r255)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 21598) (line 477) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-11 angular momentum "))) (attribute-def (declaration-name "AngularMomentumValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r256)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 21727) (line 480) (column 11) (len 542)) (normalized "source: item 4-11 angular momentum (magnitude)\nsymbol(s): `L`\napplication domain: generic\nname: AngularMomentum\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): kg*m^2*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r257)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r258)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r259)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r260)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AngularMomentumUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r261)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r262)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r263)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22582) (line 498) (column 77) (len 5)) (member-access (base (expression (span (offset 22582) (line 498) (column 77) (len 3)) (ref r264))) (separator dot) (member (ref r265))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r266)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22604) (line 498) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r267)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r268)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22683) (line 499) (column 75) (len 5)) (member-access (base (expression (span (offset 22683) (line 499) (column 75) (len 3)) (ref r269))) (separator dot) (member (ref r270))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r271)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22705) (line 499) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r272)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r273)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22788) (line 500) (column 79) (len 5)) (member-access (base (expression (span (offset 22788) (line 500) (column 79) (len 3)) (ref r274))) (separator dot) (member (ref r275))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22810) (line 500) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 22811) (line 500) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r277)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r278)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22885) (line 501) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 22886) (line 501) (column 71) (len 8)) (ref r279))) (element comma (expression (span (offset 22896) (line 501) (column 81) (len 6)) (ref r280))) (element comma (expression (span (offset 22904) (line 501) (column 89) (len 10)) (ref r281))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAngularMomentum3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r282)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 23028) (line 506) (column 11) (len 544)) (normalized "source: item 4-11 angular momentum (vector)\nsymbol(s): `vec(L)`\napplication domain: generic\nname: AngularMomentum\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): kg*m^2*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r283)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23607) (line 517) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r284)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r285)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianAngularMomentum3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r286)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r287)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23915) (line 524) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r288)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23959) (line 525) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r289)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r290)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 24031) (line 529) (column 7) (len 41)) (normalized "ISO-80000-4 item 4-12.1 moment of force "))) (attribute-def (declaration-name "MomentOfForceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r291)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 24159) (line 532) (column 11) (len 593)) (normalized "source: item 4-12.1 moment of force (magnitude)\nsymbol(s): `M`\napplication domain: generic\nname: MomentOfForce\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): N*m, kg*m^2*s^-2\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)\nremarks: The bending moment of force is denoted by `vec(M)_b`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r292)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r293)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r294)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r295)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MomentOfForceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r296)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r297)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r298)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25057) (line 550) (column 77) (len 5)) (member-access (base (expression (span (offset 25057) (line 550) (column 77) (len 3)) (ref r299))) (separator dot) (member (ref r300))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r301)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25079) (line 550) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r302)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r303)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25158) (line 551) (column 75) (len 5)) (member-access (base (expression (span (offset 25158) (line 551) (column 75) (len 3)) (ref r304))) (separator dot) (member (ref r305))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r306)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25180) (line 551) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r307)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r308)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25263) (line 552) (column 79) (len 5)) (member-access (base (expression (span (offset 25263) (line 552) (column 79) (len 3)) (ref r309))) (separator dot) (member (ref r310))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r311)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25285) (line 552) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 25286) (line 552) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r312)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r313)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25360) (line 553) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 25361) (line 553) (column 71) (len 8)) (ref r314))) (element comma (expression (span (offset 25371) (line 553) (column 81) (len 6)) (ref r315))) (element comma (expression (span (offset 25379) (line 553) (column 89) (len 10)) (ref r316))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianMomentOfForce3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r317)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 25501) (line 558) (column 11) (len 595)) (normalized "source: item 4-12.1 moment of force (vector)\nsymbol(s): `vec(M)`\napplication domain: generic\nname: MomentOfForce\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): N*m, kg*m^2*s^-2\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)\nremarks: The bending moment of force is denoted by `vec(M)_b`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r318)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26131) (line 569) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r319)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r320)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianMomentOfForce3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r321)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r322)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26431) (line 576) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r323)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26475) (line 577) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r324)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r325)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 26545) (line 581) (column 7) (len 32)) (normalized "ISO-80000-4 item 4-12.2 torque "))) (attribute-def (declaration-name "TorqueValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r326)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 26657) (line 584) (column 11) (len 619)) (normalized "source: item 4-12.2 torque\nsymbol(s): `T`, `M_Q`\napplication domain: generic\nname: Torque\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): N*m, kg*m^2*s^-2\ntensor order: 0\ndefinition: quantity described by the scalar product: `T = vec(M)*vec(e_Q)` where `vec(M)` is moment of force (item 4-12.1) and `vec(e_Q)` is unit vector of direction with respect to which the torque is considered\nremarks: For example, torque is the twisting moment of force with respect to the longitudinal axis of a beam or shaft.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r327)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r328)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r329)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r330)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "TorqueUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r331)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r332)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r333)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27553) (line 602) (column 77) (len 5)) (member-access (base (expression (span (offset 27553) (line 602) (column 77) (len 3)) (ref r334))) (separator dot) (member (ref r335))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r336)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27575) (line 602) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r337)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r338)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27654) (line 603) (column 75) (len 5)) (member-access (base (expression (span (offset 27654) (line 603) (column 75) (len 3)) (ref r339))) (separator dot) (member (ref r340))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r341)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27676) (line 603) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r342)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r343)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27759) (line 604) (column 79) (len 5)) (member-access (base (expression (span (offset 27759) (line 604) (column 79) (len 3)) (ref r344))) (separator dot) (member (ref r345))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r346)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27781) (line 604) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 27782) (line 604) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r347)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r348)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27856) (line 605) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 27857) (line 605) (column 71) (len 8)) (ref r349))) (element comma (expression (span (offset 27867) (line 605) (column 81) (len 6)) (ref r350))) (element comma (expression (span (offset 27875) (line 605) (column 89) (len 10)) (ref r351))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 27903) (line 608) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-13 angular impulse "))) (attribute-def (declaration-name "AngularImpulseValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r352)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 28030) (line 611) (column 11) (len 719)) (normalized "source: item 4-13 angular impulse (magnitude)\nsymbol(s): `H`\napplication domain: generic\nname: AngularImpulse\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): N*m*s, kg*m^2*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval\nremarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r353)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r354)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r355)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r356)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AngularImpulseUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r357)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r358)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r359)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29058) (line 629) (column 77) (len 5)) (member-access (base (expression (span (offset 29058) (line 629) (column 77) (len 3)) (ref r360))) (separator dot) (member (ref r361))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r362)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29080) (line 629) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r363)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r364)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29159) (line 630) (column 75) (len 5)) (member-access (base (expression (span (offset 29159) (line 630) (column 75) (len 3)) (ref r365))) (separator dot) (member (ref r366))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r367)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29181) (line 630) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r368)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r369)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29264) (line 631) (column 79) (len 5)) (member-access (base (expression (span (offset 29264) (line 631) (column 79) (len 3)) (ref r370))) (separator dot) (member (ref r371))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r372)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29286) (line 631) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 29287) (line 631) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r373)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r374)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29361) (line 632) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 29362) (line 632) (column 71) (len 8)) (ref r375))) (element comma (expression (span (offset 29372) (line 632) (column 81) (len 6)) (ref r376))) (element comma (expression (span (offset 29380) (line 632) (column 89) (len 10)) (ref r377))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAngularImpulse3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r378)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 29503) (line 637) (column 11) (len 721)) (normalized "source: item 4-13 angular impulse (vector)\nsymbol(s): `vec(H)`\napplication domain: generic\nname: AngularImpulse\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): N*m*s, kg*m^2*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval\nremarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r379)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30259) (line 648) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r380)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r381)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianAngularImpulse3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r382)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30563) (line 655) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r384)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30607) (line 656) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r385)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r386)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 30678) (line 660) (column 7) (len 34)) (normalized "ISO-80000-4 item 4-14.1 pressure "))) (attribute-def (declaration-name "PressureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r387)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 30794) (line 663) (column 11) (len 536)) (normalized "source: item 4-14.1 pressure\nsymbol(s): `p`\napplication domain: generic\nname: Pressure\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: quotient of the component of a force normal to a surface and its area: `p = (vec(e_n) * vec(F)) / A` where `vec(e_n)` is unit vector of the surface normal, `vec(F)` is force (item 4-9.1) and `A` is area (ISO 80000-3)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r388)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r389)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r390)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r391)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "PressureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r392)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r393)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r394)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31615) (line 681) (column 77) (len 5)) (member-access (base (expression (span (offset 31615) (line 681) (column 77) (len 3)) (ref r395))) (separator dot) (member (ref r396))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r397)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31637) (line 681) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 31638) (line 681) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r398)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r399)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31717) (line 682) (column 75) (len 5)) (member-access (base (expression (span (offset 31717) (line 682) (column 75) (len 3)) (ref r400))) (separator dot) (member (ref r401))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r402)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31739) (line 682) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r403)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r404)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31822) (line 683) (column 79) (len 5)) (member-access (base (expression (span (offset 31822) (line 683) (column 79) (len 3)) (ref r405))) (separator dot) (member (ref r406))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r407)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31844) (line 683) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 31845) (line 683) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r409)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31919) (line 684) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 31920) (line 684) (column 71) (len 8)) (ref r410))) (element comma (expression (span (offset 31930) (line 684) (column 81) (len 6)) (ref r411))) (element comma (expression (span (offset 31938) (line 684) (column 89) (len 10)) (ref r412))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 31966) (line 687) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-14.2 gauge pressure "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 32637) (line 703) (column 7) (len 30)) (normalized "ISO-80000-4 item 4-15 stress "))) (attribute-def (declaration-name "StressValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r413)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 32747) (line 706) (column 11) (len 492)) (normalized "source: item 4-15 stress (magnitude)\nsymbol(s): `σ`\napplication domain: generic\nname: Stress\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: tensor (ISO 80000-2) quantity representing state of tension of matter\nremarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r414)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r415)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r416)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r417)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "StressUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r418)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r419)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r420)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33516) (line 724) (column 77) (len 5)) (member-access (base (expression (span (offset 33516) (line 724) (column 77) (len 3)) (ref r421))) (separator dot) (member (ref r422))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r423)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33538) (line 724) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 33539) (line 724) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r424)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r425)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33618) (line 725) (column 75) (len 5)) (member-access (base (expression (span (offset 33618) (line 725) (column 75) (len 3)) (ref r426))) (separator dot) (member (ref r427))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r428)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33640) (line 725) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r429)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r430)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33723) (line 726) (column 79) (len 5)) (member-access (base (expression (span (offset 33723) (line 726) (column 79) (len 3)) (ref r431))) (separator dot) (member (ref r432))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r433)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33745) (line 726) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 33746) (line 726) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r434)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r435)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33820) (line 727) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 33821) (line 727) (column 71) (len 8)) (ref r436))) (element comma (expression (span (offset 33831) (line 727) (column 81) (len 6)) (ref r437))) (element comma (expression (span (offset 33839) (line 727) (column 89) (len 10)) (ref r438))))))))) (body semicolon)))))) (attribute-def (declaration-name "Cartesian3dStressTensor") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r439)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 33950) (line 732) (column 11) (len 499)) (normalized "source: item 4-15 stress (tensor)\nsymbol(s): `vec(vec(σ))`\napplication domain: generic\nname: Stress\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 2\ndefinition: tensor (ISO 80000-2) quantity representing state of tension of matter\nremarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r440)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34484) (line 743) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r441)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r442)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r443)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r444)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "Cartesian3dStressMeasurementReference") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r445)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r446)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34801) (line 751) (column 36) (len 6)) (sequence (sequence-list (element first (expression (span (offset 34802) (line 751) (column 37) (len 1)) (integer 3))) (element comma (expression (span (offset 34805) (line 751) (column 40) (len 1)) (integer 3))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r447)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34841) (line 752) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r448)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r449)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 34905) (line 756) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-16.1 normal stress "))) (attribute-def (declaration-name "NormalStressValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r450)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 35030) (line 759) (column 11) (len 793)) (normalized "source: item 4-16.1 normal stress\nsymbol(s): `σ_n`, `σ`\napplication domain: generic\nname: NormalStress\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `σ_n = (d F_n)/(dA)` where `F_n` is the normal component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element\nremarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter normal to it, and evenly distributed, cause a constant normal stress `σ_n = F A` in the slice (layer).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r451)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r452)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r453)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r454)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "NormalStressUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r455)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r456)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r457)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36124) (line 777) (column 77) (len 5)) (member-access (base (expression (span (offset 36124) (line 777) (column 77) (len 3)) (ref r458))) (separator dot) (member (ref r459))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r460)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36146) (line 777) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 36147) (line 777) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r461)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r462)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36226) (line 778) (column 75) (len 5)) (member-access (base (expression (span (offset 36226) (line 778) (column 75) (len 3)) (ref r463))) (separator dot) (member (ref r464))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r465)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36248) (line 778) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r466)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r467)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36331) (line 779) (column 79) (len 5)) (member-access (base (expression (span (offset 36331) (line 779) (column 79) (len 3)) (ref r468))) (separator dot) (member (ref r469))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r470)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36353) (line 779) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 36354) (line 779) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r471)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r472)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36428) (line 780) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 36429) (line 780) (column 71) (len 8)) (ref r473))) (element comma (expression (span (offset 36439) (line 780) (column 81) (len 6)) (ref r474))) (element comma (expression (span (offset 36447) (line 780) (column 89) (len 10)) (ref r475))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 36475) (line 783) (column 7) (len 38)) (normalized "ISO-80000-4 item 4-16.2 shear stress "))) (attribute-def (declaration-name "ShearStressValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r476)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36598) (line 786) (column 11) (len 794)) (normalized "source: item 4-16.2 shear stress\nsymbol(s): `τ_s`, `τ`\napplication domain: generic\nname: ShearStress\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `τ_s = (d F_t)/(dA)` where `F_t` is the tangential component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element\nremarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter parallel to it, and evenly distributed, cause a constant shear stress `τ = F/A` in the slice (layer).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r477)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r478)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r479)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r480)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ShearStressUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r481)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r482)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r483)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37689) (line 804) (column 77) (len 5)) (member-access (base (expression (span (offset 37689) (line 804) (column 77) (len 3)) (ref r484))) (separator dot) (member (ref r485))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r486)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37711) (line 804) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 37712) (line 804) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r487)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r488)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37791) (line 805) (column 75) (len 5)) (member-access (base (expression (span (offset 37791) (line 805) (column 75) (len 3)) (ref r489))) (separator dot) (member (ref r490))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r491)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37813) (line 805) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r492)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r493)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37896) (line 806) (column 79) (len 5)) (member-access (base (expression (span (offset 37896) (line 806) (column 79) (len 3)) (ref r494))) (separator dot) (member (ref r495))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r496)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37918) (line 806) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 37919) (line 806) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r497)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r498)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37993) (line 807) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 37994) (line 807) (column 71) (len 8)) (ref r499))) (element comma (expression (span (offset 38004) (line 807) (column 81) (len 6)) (ref r500))) (element comma (expression (span (offset 38012) (line 807) (column 89) (len 10)) (ref r501))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38040) (line 810) (column 7) (len 32)) (normalized "ISO-80000-4 item 4-17.1 strain "))) (attribute-def (declaration-name "StrainValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r502)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38152) (line 813) (column 11) (len 475)) (normalized "source: item 4-17.1 strain (magnitude)\nsymbol(s): `ε`\napplication domain: generic\nname: Strain\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress\nremarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r503)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r504)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r505)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r506)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "StrainUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r507)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "Cartesian3dStrainTensor") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r508)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38929) (line 835) (column 11) (len 482)) (normalized "source: item 4-17.1 strain (tensor)\nsymbol(s): `vec(vec(ε))`\napplication domain: generic\nname: Strain\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 2\ndefinition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress\nremarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r509)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39446) (line 846) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r510)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r511)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r512)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r513)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "Cartesian3dStrainMeasurementReference") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r514)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r515)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39763) (line 854) (column 36) (len 6)) (sequence (sequence-list (element first (expression (span (offset 39764) (line 854) (column 37) (len 1)) (integer 3))) (element comma (expression (span (offset 39767) (line 854) (column 40) (len 1)) (integer 3))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r516)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39803) (line 855) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r517)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r518)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 39867) (line 859) (column 7) (len 48)) (normalized "ISO-80000-4 item 4-17.2 relative linear strain "))) (attribute-def (declaration-name "RelativeLinearStrainValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r519)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40007) (line 862) (column 11) (len 464)) (normalized "source: item 4-17.2 relative linear strain\nsymbol(s): `ε`, `(e)`\napplication domain: generic\nname: RelativeLinearStrain (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of change in length `Δl` (ISO 80000-3) of an object and its length `l` (ISO 80000-3): `ε = (Δl)/l`\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 40570) (line 876) (column 7) (len 38)) (normalized "ISO-80000-4 item 4-17.3 shear strain "))) (attribute-def (declaration-name "ShearStrainValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r520)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40691) (line 879) (column 11) (len 473)) (normalized "source: item 4-17.3 shear strain\nsymbol(s): `γ`\napplication domain: generic\nname: ShearStrain (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of parallel displacement `Δx` (ISO 80000-3) of two surfaces of a layer and the thickness `d` (ISO 80000-3) of the layer: `γ = (Δx)/d`\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 41245) (line 893) (column 7) (len 48)) (normalized "ISO-80000-4 item 4-17.4 relative volume strain "))) (attribute-def (declaration-name "RelativeVolumeStrainValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r521)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 41385) (line 896) (column 11) (len 461)) (normalized "source: item 4-17.4 relative volume strain\nsymbol(s): `θ`\napplication domain: generic\nname: RelativeVolumeStrain (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of change in volume `ΔV` (ISO 80000-3) of an object and its volume `V_0` (ISO 80000-3): `θ = (ΔV)/V_0`\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 41945) (line 910) (column 7) (len 38)) (normalized "ISO-80000-4 item 4-18 Poisson number "))) (attribute-def (declaration-name "PoissonNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r522)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 42068) (line 913) (column 11) (len 499)) (normalized "source: item 4-18 Poisson number\nsymbol(s): `μ`, `(v)`\napplication domain: generic\nname: PoissonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of change in width `Δb` (width is defined in ISO 80000-3) and change in length `Δl` (length is defined in ISO 80000-3) of an object: `μ = (Δb)/(Δl)`\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 42652) (line 927) (column 7) (len 62)) (normalized "ISO-80000-4 item 4-19.1 modulus of elasticity, Young modulus "))) (attribute-def (declaration-name "ModulusOfElasticityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r523)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 42807) (line 930) (column 11) (len 539)) (normalized "source: item 4-19.1 modulus of elasticity, Young modulus\nsymbol(s): `E`, `E_m`, `Y`\napplication domain: generic\nname: ModulusOfElasticity\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: quotient of normal stress `σ` (item 4-16.1) and relative linear strain `ε` (item 4-17.2): `E = σ/ε`\nremarks: Conditions should be specified (e.g. adiabatic or isothermal process).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r524)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r525)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r526)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r527)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ModulusOfElasticityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r528)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r529)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r530)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43675) (line 948) (column 77) (len 5)) (member-access (base (expression (span (offset 43675) (line 948) (column 77) (len 3)) (ref r531))) (separator dot) (member (ref r532))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r533)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43697) (line 948) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 43698) (line 948) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r534)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r535)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43777) (line 949) (column 75) (len 5)) (member-access (base (expression (span (offset 43777) (line 949) (column 75) (len 3)) (ref r536))) (separator dot) (member (ref r537))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r538)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43799) (line 949) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r539)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r540)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43882) (line 950) (column 79) (len 5)) (member-access (base (expression (span (offset 43882) (line 950) (column 79) (len 3)) (ref r541))) (separator dot) (member (ref r542))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r543)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43904) (line 950) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 43905) (line 950) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r544)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r545)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43979) (line 951) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 43980) (line 951) (column 71) (len 8)) (ref r546))) (element comma (expression (span (offset 43990) (line 951) (column 81) (len 6)) (ref r547))) (element comma (expression (span (offset 43998) (line 951) (column 89) (len 10)) (ref r548))))))))) (body semicolon)))))) (alias (name "YoungModulusUnit") (target (ref r549)) (body semicolon)) (alias (name "YoungModulusValue") (target (ref r550)) (body semicolon)) (alias (name "youngModulus") (target (ref r551)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 44189) (line 958) (column 7) (len 60)) (normalized "ISO-80000-4 item 4-19.2 modulus of rigidity, shear modulus "))) (attribute-def (declaration-name "ModulusOfRigidityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r552)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 44340) (line 961) (column 11) (len 513)) (normalized "source: item 4-19.2 modulus of rigidity, shear modulus\nsymbol(s): `G`\napplication domain: generic\nname: ModulusOfRigidity\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: quotient of shear stress `τ` (item 4-16.2) and shear strain `γ` (item 4-17.3): `G = τ/γ`\nremarks: Conditions should be specified (e.g. isentropic or isothermal process).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r553)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r554)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r555)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r556)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ModulusOfRigidityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r557)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r558)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r559)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45174) (line 979) (column 77) (len 5)) (member-access (base (expression (span (offset 45174) (line 979) (column 77) (len 3)) (ref r560))) (separator dot) (member (ref r561))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r562)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45196) (line 979) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 45197) (line 979) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r563)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r564)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45276) (line 980) (column 75) (len 5)) (member-access (base (expression (span (offset 45276) (line 980) (column 75) (len 3)) (ref r565))) (separator dot) (member (ref r566))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r567)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45298) (line 980) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r568)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r569)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45381) (line 981) (column 79) (len 5)) (member-access (base (expression (span (offset 45381) (line 981) (column 79) (len 3)) (ref r570))) (separator dot) (member (ref r571))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r572)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45403) (line 981) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 45404) (line 981) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r573)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r574)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45478) (line 982) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 45479) (line 982) (column 71) (len 8)) (ref r575))) (element comma (expression (span (offset 45489) (line 982) (column 81) (len 6)) (ref r576))) (element comma (expression (span (offset 45497) (line 982) (column 89) (len 10)) (ref r577))))))))) (body semicolon)))))) (alias (name "ShearModulusUnit") (target (ref r578)) (body semicolon)) (alias (name "ShearModulusValue") (target (ref r579)) (body semicolon)) (alias (name "shearModulus") (target (ref r580)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45682) (line 989) (column 7) (len 62)) (normalized "ISO-80000-4 item 4-19.3 modulus of compression, bulk modulus "))) (attribute-def (declaration-name "ModulusOfCompressionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r581)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 45838) (line 992) (column 11) (len 553)) (normalized "source: item 4-19.3 modulus of compression, bulk modulus\nsymbol(s): `K`, `K_m`, `B`\napplication domain: generic\nname: ModulusOfCompression\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2\ntensor order: 0\ndefinition: negative of the quotient of pressure `p` (item 4-14.1) and relative volume strain `θ` (item 4-17.4): `K = -(p/θ)`\nremarks: Conditions should be specified (e.g. isentropic or isothermal process).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r582)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r583)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r584)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r585)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ModulusOfCompressionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r586)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r587)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r588)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46724) (line 1010) (column 77) (len 5)) (member-access (base (expression (span (offset 46724) (line 1010) (column 77) (len 3)) (ref r589))) (separator dot) (member (ref r590))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r591)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46746) (line 1010) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 46747) (line 1010) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r592)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r593)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46826) (line 1011) (column 75) (len 5)) (member-access (base (expression (span (offset 46826) (line 1011) (column 75) (len 3)) (ref r594))) (separator dot) (member (ref r595))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r596)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46848) (line 1011) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r597)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r598)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46931) (line 1012) (column 79) (len 5)) (member-access (base (expression (span (offset 46931) (line 1012) (column 79) (len 3)) (ref r599))) (separator dot) (member (ref r600))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r601)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46953) (line 1012) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 46954) (line 1012) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r602)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r603)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47028) (line 1013) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 47029) (line 1013) (column 71) (len 8)) (ref r604))) (element comma (expression (span (offset 47039) (line 1013) (column 81) (len 6)) (ref r605))) (element comma (expression (span (offset 47047) (line 1013) (column 89) (len 10)) (ref r606))))))))) (body semicolon)))))) (alias (name "BulkModulusUnit") (target (ref r607)) (body semicolon)) (alias (name "BulkModulusValue") (target (ref r608)) (body semicolon)) (alias (name "bulkModulus") (target (ref r609)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 47238) (line 1020) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-20 compressibility "))) (attribute-def (declaration-name "CompressibilityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r610)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 47366) (line 1023) (column 11) (len 549)) (normalized "source: item 4-20 compressibility\nsymbol(s): `ϰ`\napplication domain: generic\nname: Compressibility\nquantity dimension: L^1*M^-1*T^2\nmeasurement unit(s): Pa^-1, kg^-1*m*s^2\ntensor order: 0\ndefinition: negative relative change of volume `V` (ISO 80000-3) of an object under pressure `p` (item 4-14.1) expressed by: `ϰ = -(1/V)(dV)/(dp)`\nremarks: Conditions should be specified (e.g. isentropic or isothermal process). See also ISO 80000-5.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r611)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r612)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r613)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r614)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CompressibilityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r615)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r616)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r617)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48228) (line 1041) (column 77) (len 5)) (member-access (base (expression (span (offset 48228) (line 1041) (column 77) (len 3)) (ref r618))) (separator dot) (member (ref r619))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r620)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48250) (line 1041) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r621)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r622)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48329) (line 1042) (column 75) (len 5)) (member-access (base (expression (span (offset 48329) (line 1042) (column 75) (len 3)) (ref r623))) (separator dot) (member (ref r624))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r625)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48351) (line 1042) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 48352) (line 1042) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r626)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r627)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48435) (line 1043) (column 79) (len 5)) (member-access (base (expression (span (offset 48435) (line 1043) (column 79) (len 3)) (ref r628))) (separator dot) (member (ref r629))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r630)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48457) (line 1043) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r631)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r632)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48531) (line 1044) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 48532) (line 1044) (column 71) (len 8)) (ref r633))) (element comma (expression (span (offset 48542) (line 1044) (column 81) (len 6)) (ref r634))) (element comma (expression (span (offset 48550) (line 1044) (column 89) (len 10)) (ref r635))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 48578) (line 1047) (column 7) (len 53)) (normalized "ISO-80000-4 item 4-21.1 second axial moment of area "))) (attribute-def (declaration-name "SecondAxialMomentOfAreaValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r636)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 48728) (line 1050) (column 11) (len 767)) (normalized "source: item 4-21.1 second axial moment of area\nsymbol(s): `I_a`\napplication domain: generic\nname: SecondAxialMomentOfArea\nquantity dimension: L^4\nmeasurement unit(s): m^4\ntensor order: 0\ndefinition: geometrical characteristic of a shape of a body equal to: `I_a = int int_M r_Q^2 dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis in the plane of the surface considered and `A` is area (ISO 80000-3)\nremarks: This quantity is often referred to wrongly as \"moment of inertia\" (item 4-7). The subscript, `a`, may be omitted when there is no risk of confusion.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r637)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r638)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r639)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r640)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SecondAxialMomentOfAreaUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r641)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r642)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r643)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49840) (line 1068) (column 77) (len 5)) (member-access (base (expression (span (offset 49840) (line 1068) (column 77) (len 3)) (ref r644))) (separator dot) (member (ref r645))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r646)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49862) (line 1068) (column 99) (len 1)) (integer 4))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r647)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r648)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49936) (line 1069) (column 70) (len 8)) (ref r649))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49961) (line 1072) (column 7) (len 53)) (normalized "ISO-80000-4 item 4-21.2 second polar moment of area "))) (attribute-def (declaration-name "SecondPolarMomentOfAreaValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r650)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50111) (line 1075) (column 11) (len 783)) (normalized "source: item 4-21.2 second polar moment of area\nsymbol(s): `I_p`\napplication domain: generic\nname: SecondPolarMomentOfArea\nquantity dimension: L^4\nmeasurement unit(s): m^4\ntensor order: 0\ndefinition: geometrical characteristic of a shape of a body equal to: `I_p = int int_M r_Q^2 * dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis perpendicular to the plane of the surface considered and `A` is area (ISO 80000-3)\nremarks: This quantity is often referred to wrongly as \"moment of inertia\" (item 4-7). The subscript, `p`, may be omitted when there is no risk of confusion.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r651)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r652)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r653)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r654)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SecondPolarMomentOfAreaUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r655)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r656)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r657)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51239) (line 1093) (column 77) (len 5)) (member-access (base (expression (span (offset 51239) (line 1093) (column 77) (len 3)) (ref r658))) (separator dot) (member (ref r659))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r660)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51261) (line 1093) (column 99) (len 1)) (integer 4))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r661)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r662)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51335) (line 1094) (column 70) (len 8)) (ref r663))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 51360) (line 1097) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-22 section modulus "))) (attribute-def (declaration-name "SectionModulusValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r664)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 51487) (line 1100) (column 11) (len 600)) (normalized "source: item 4-22 section modulus\nsymbol(s): `Z`, `(W)`\napplication domain: generic\nname: SectionModulus\nquantity dimension: L^3\nmeasurement unit(s): m^3\ntensor order: 0\ndefinition: geometrical characteristic of a shape of a body equal to: `Z = I_a/r_(Q_max)` where `I_a` is the second axial moment of area (item 4-21.1) and `r_(Q,max)` is the maximum radial distance (ISO 80000-3) of any point in the surface considered from the Q-axis with respect to which `I_a` is defined\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r665)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r666)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r667)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r668)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SectionModulusUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r669)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r670)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r671)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52396) (line 1118) (column 77) (len 5)) (member-access (base (expression (span (offset 52396) (line 1118) (column 77) (len 3)) (ref r672))) (separator dot) (member (ref r673))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r674)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52418) (line 1118) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r675)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r676)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52492) (line 1119) (column 70) (len 8)) (ref r677))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 52517) (line 1122) (column 7) (len 109)) (normalized "ISO-80000-4 item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction "))) (attribute-def (declaration-name "StaticFrictionCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r678)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 52723) (line 1125) (column 11) (len 858)) (normalized "source: item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction\nsymbol(s): `μ_s`, `(f_s)`\napplication domain: generic\nname: StaticFrictionCoefficient (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: proportionality factor between the maximum magnitude of the tangential component `F_max` of the static friction force (item 4-9.3) and the magnitude of the normal component `N` of the contact force (item 4-9.1) between two bodies at relative rest with respect to each other: `F_max = μ_s * N`\nremarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both.\n"))))) (attribute-usage) (alias (name "staticFrictionFactor") (target (ref r679)) (body semicolon)) (alias (name "coefficientOfStaticFriction") (target (ref r680)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 53823) (line 1143) (column 7) (len 74)) (normalized "ISO-80000-4 item 4-23.2 kinetic friction factor, dynamic friction factor "))) (attribute-def (declaration-name "KineticFrictionFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r681)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 53990) (line 1146) (column 11) (len 793)) (normalized "source: item 4-23.2 kinetic friction factor, dynamic friction factor\nsymbol(s): `μ`, `(f)`\napplication domain: generic\nname: KineticFrictionFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: proportionality factor between the magnitudes of the kinetic friction force, `F_μ` (item 4-9.4) and the normal component `N` of the contact force (item 4-9.1): `F_μ = μ * N`\nremarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both. The dynamic friction factor `µ` is independent in first approximation of the contact surface.\n"))))) (attribute-usage) (alias (name "dynamicFrictionFactor") (target (ref r682)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 54944) (line 1162) (column 7) (len 51)) (normalized "ISO-80000-4 item 4-23.3 rolling resistance factor "))) (attribute-def (declaration-name "RollingResistanceFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r683)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 55090) (line 1165) (column 11) (len 610)) (normalized "source: item 4-23.3 rolling resistance factor\nsymbol(s): `C_\"rr\"`\napplication domain: generic\nname: RollingResistanceFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: proportionality factor between the magnitude of the tangential component `F` and the magnitude of the normal component `N` of the force applied to a body rolling on a surface at constant speed: `F = C_(rr)*N`\nremarks: Also known as rolling resistance coefficient, RRC.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 55805) (line 1179) (column 7) (len 55)) (normalized "ISO-80000-4 item 4-23.4 drag coefficient, drag factor "))) (attribute-def (declaration-name "DragCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r684)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 55947) (line 1182) (column 11) (len 650)) (normalized "source: item 4-23.4 drag coefficient, drag factor\nsymbol(s): `C_D`\napplication domain: generic\nname: DragCoefficient (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: factor proportional to magnitude `F_D` of the drag force (item 4-9.6) of a body moving in a fluid, dependent on the shape and speed `v` (ISO 80000-3) of a body: `F_D = 1/2 * C_D * ρ * v^2 * A` where `ρ` is mass density (item 4-2) of the fluid and `A` is cross-section area (ISO 80000-3) of the body\nremarks: None.\n"))))) (attribute-usage) (alias (name "dragFactor") (target (ref r685)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 56729) (line 1198) (column 7) (len 52)) (normalized "ISO-80000-4 item 4-24 dynamic viscosity, viscosity "))) (attribute-def (declaration-name "DynamicViscosityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r686)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 56871) (line 1201) (column 11) (len 574)) (normalized "source: item 4-24 dynamic viscosity, viscosity\nsymbol(s): `η`\napplication domain: generic\nname: DynamicViscosity\nquantity dimension: L^-1*M^1*T^-1\nmeasurement unit(s): Pa*s, kg*m^-1*s^-1\ntensor order: 0\ndefinition: for laminar flows, proportionality constant between shear stress `τ_(xz)` (item 4-16.2) in a fluid moving with a velocity `v_x` (ISO 80000-3) and gradient `(d v_x)/dz` perpendicular to the plane of shear: `τ_(xz) = η (d v_x)/(dz)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r687)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r688)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r689)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r690)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DynamicViscosityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r691)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r692)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r693)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57762) (line 1219) (column 77) (len 5)) (member-access (base (expression (span (offset 57762) (line 1219) (column 77) (len 3)) (ref r694))) (separator dot) (member (ref r695))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r696)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57784) (line 1219) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 57785) (line 1219) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r697)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r698)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57864) (line 1220) (column 75) (len 5)) (member-access (base (expression (span (offset 57864) (line 1220) (column 75) (len 3)) (ref r699))) (separator dot) (member (ref r700))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r701)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57886) (line 1220) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r702)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r703)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57969) (line 1221) (column 79) (len 5)) (member-access (base (expression (span (offset 57969) (line 1221) (column 79) (len 3)) (ref r704))) (separator dot) (member (ref r705))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r706)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57991) (line 1221) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 57992) (line 1221) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r707)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r708)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58066) (line 1222) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 58067) (line 1222) (column 71) (len 8)) (ref r709))) (element comma (expression (span (offset 58077) (line 1222) (column 81) (len 6)) (ref r710))) (element comma (expression (span (offset 58085) (line 1222) (column 89) (len 10)) (ref r711))))))))) (body semicolon)))))) (alias (name "ViscosityUnit") (target (ref r712)) (body semicolon)) (alias (name "ViscosityValue") (target (ref r713)) (body semicolon)) (alias (name "viscosity") (target (ref r714)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 58258) (line 1229) (column 7) (len 43)) (normalized "ISO-80000-4 item 4-25 kinematic viscosity "))) (attribute-def (declaration-name "KinematicViscosityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r715)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 58393) (line 1232) (column 11) (len 421)) (normalized "source: item 4-25 kinematic viscosity\nsymbol(s): `v`\napplication domain: generic\nname: KinematicViscosity\nquantity dimension: L^2*T^-1\nmeasurement unit(s): m^2*s^-1\ntensor order: 0\ndefinition: quotient of dynamic viscosity `η` (item 4-24) and mass density `ρ` (item 4-2) of a fluid: `v = η/ρ`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r716)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r717)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r718)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r719)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "KinematicViscosityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r720)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r721)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r722)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59139) (line 1250) (column 77) (len 5)) (member-access (base (expression (span (offset 59139) (line 1250) (column 77) (len 3)) (ref r723))) (separator dot) (member (ref r724))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r725)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59161) (line 1250) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r726)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r727)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59244) (line 1251) (column 79) (len 5)) (member-access (base (expression (span (offset 59244) (line 1251) (column 79) (len 3)) (ref r728))) (separator dot) (member (ref r729))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r730)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59266) (line 1251) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 59267) (line 1251) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r731)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r732)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59341) (line 1252) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 59342) (line 1252) (column 71) (len 8)) (ref r733))) (element comma (expression (span (offset 59352) (line 1252) (column 81) (len 10)) (ref r734))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 59380) (line 1255) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-26 surface tension "))) (attribute-def (declaration-name "SurfaceTensionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r735)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 59507) (line 1258) (column 11) (len 535)) (normalized "source: item 4-26 surface tension\nsymbol(s): `γ`, `σ`\napplication domain: generic\nname: SurfaceTension\nquantity dimension: M^1*T^-2\nmeasurement unit(s): N*m^-1, kg*s^-2\ntensor order: 0\ndefinition: magnitude of a force acting against the enlargement of area portion of a surface separating a liquid from its surrounding\nremarks: The concept of surface energy is closely related to surface tension and has the same dimension.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r736)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r737)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r738)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r739)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SurfaceTensionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r740)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r741)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r742)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60349) (line 1276) (column 75) (len 5)) (member-access (base (expression (span (offset 60349) (line 1276) (column 75) (len 3)) (ref r743))) (separator dot) (member (ref r744))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r745)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60371) (line 1276) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r746)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r747)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60454) (line 1277) (column 79) (len 5)) (member-access (base (expression (span (offset 60454) (line 1277) (column 79) (len 3)) (ref r748))) (separator dot) (member (ref r749))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r750)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60476) (line 1277) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 60477) (line 1277) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r751)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r752)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60551) (line 1278) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 60552) (line 1278) (column 71) (len 6)) (ref r753))) (element comma (expression (span (offset 60560) (line 1278) (column 79) (len 10)) (ref r754))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 60588) (line 1281) (column 7) (len 31)) (normalized "ISO-80000-4 item 4-27.1 power "))) (attribute-def (declaration-name "PowerValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r755)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 60698) (line 1284) (column 11) (len 370)) (normalized "source: item 4-27.1 power\nsymbol(s): `P`\napplication domain: generic\nname: Power\nquantity dimension: L^2*M^1*T^-3\nmeasurement unit(s): W, J*s^-1, kg*m^2*s^-3\ntensor order: 0\ndefinition: quotient of energy (ISO 80000-5) and duration (ISO 80000-3)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r756)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r757)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r758)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r759)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "PowerUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r760)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r761)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r762)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61341) (line 1302) (column 77) (len 5)) (member-access (base (expression (span (offset 61341) (line 1302) (column 77) (len 3)) (ref r763))) (separator dot) (member (ref r764))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r765)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61363) (line 1302) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r766)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r767)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61442) (line 1303) (column 75) (len 5)) (member-access (base (expression (span (offset 61442) (line 1303) (column 75) (len 3)) (ref r768))) (separator dot) (member (ref r769))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r770)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61464) (line 1303) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r771)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r772)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61547) (line 1304) (column 79) (len 5)) (member-access (base (expression (span (offset 61547) (line 1304) (column 79) (len 3)) (ref r773))) (separator dot) (member (ref r774))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r775)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61569) (line 1304) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 61570) (line 1304) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r776)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r777)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61644) (line 1305) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 61645) (line 1305) (column 71) (len 8)) (ref r778))) (element comma (expression (span (offset 61655) (line 1305) (column 81) (len 6)) (ref r779))) (element comma (expression (span (offset 61663) (line 1305) (column 89) (len 10)) (ref r780))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 61691) (line 1308) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-27 mechanical power "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 62315) (line 1324) (column 7) (len 42)) (normalized "ISO-80000-4 item 4-28.1 potential energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 63137) (line 1340) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-28.2 kinetic energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 63834) (line 1356) (column 7) (len 43)) (normalized "ISO-80000-4 item 4-28.3 mechanical energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 64573) (line 1372) (column 7) (len 47)) (normalized "ISO-80000-4 item 4-28.4 mechanical work, work "))) (attribute-usage) (alias (name "work") (target (ref r781)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 65591) (line 1390) (column 7) (len 45)) (normalized "ISO-80000-4 item 4-29 mechanical efficiency "))) (attribute-def (declaration-name "MechanicalEfficiencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r782)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 65728) (line 1393) (column 11) (len 569)) (normalized "source: item 4-29 mechanical efficiency\nsymbol(s): `η`\napplication domain: mechanics\nname: MechanicalEfficiency (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of output power `P_\"out\"` (item 4-27) from a system and input power `P_\"in\"` (item 4-27) to this system: `η = P_\"out\"/P_\"in\"`\nremarks: The system must be specified. This quantity is often expressed by the unit percent, symbol %.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 66396) (line 1407) (column 7) (len 35)) (normalized "ISO-80000-4 item 4-30.1 mass flow "))) (attribute-def (declaration-name "MassFlowValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r783)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 66513) (line 1410) (column 11) (len 506)) (normalized "source: item 4-30.1 mass flow (magnitude)\nsymbol(s): `j_m`\napplication domain: generic\nname: MassFlow\nquantity dimension: L^-2*M^1*T^-1\nmeasurement unit(s): kg*m^-2*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r784)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r785)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r786)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r787)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassFlowUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r788)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r789)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r790)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67304) (line 1428) (column 77) (len 5)) (member-access (base (expression (span (offset 67304) (line 1428) (column 77) (len 3)) (ref r791))) (separator dot) (member (ref r792))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r793)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67326) (line 1428) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 67327) (line 1428) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r794)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r795)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67406) (line 1429) (column 75) (len 5)) (member-access (base (expression (span (offset 67406) (line 1429) (column 75) (len 3)) (ref r796))) (separator dot) (member (ref r797))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r798)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67428) (line 1429) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r799)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r800)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67511) (line 1430) (column 79) (len 5)) (member-access (base (expression (span (offset 67511) (line 1430) (column 79) (len 3)) (ref r801))) (separator dot) (member (ref r802))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r803)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67533) (line 1430) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 67534) (line 1430) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r804)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r805)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67608) (line 1431) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 67609) (line 1431) (column 71) (len 8)) (ref r806))) (element comma (expression (span (offset 67619) (line 1431) (column 81) (len 6)) (ref r807))) (element comma (expression (span (offset 67627) (line 1431) (column 89) (len 10)) (ref r808))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianMassFlow3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r809)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 67744) (line 1436) (column 11) (len 508)) (normalized "source: item 4-30.1 mass flow (vector)\nsymbol(s): `vec(j_m)`\napplication domain: generic\nname: MassFlow\nquantity dimension: L^-2*M^1*T^-1\nmeasurement unit(s): kg*m^-2*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r810)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 68287) (line 1447) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r811)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r812)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianMassFlow3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r813)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r814)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 68567) (line 1454) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r815)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 68611) (line 1455) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r816)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r817)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 68676) (line 1459) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-30.2 mass flow rate "))) (attribute-def (declaration-name "MassFlowRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r818)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 68802) (line 1462) (column 11) (len 639)) (normalized "source: item 4-30.2 mass flow rate\nsymbol(s): `q_m`\napplication domain: generic\nname: MassFlowRate\nquantity dimension: M^1*T^-1\nmeasurement unit(s): kg*s^-1\ntensor order: 0\ndefinition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with normal vector `vec(e)_n` of a flowing fluid with mass flow `vec(j)_m` (item 4-30.1) as an integral: `q_m = int int_A vec(j)_m * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r819)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r820)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r821)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r822)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassFlowRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r823)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r824)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r825)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69740) (line 1480) (column 75) (len 5)) (member-access (base (expression (span (offset 69740) (line 1480) (column 75) (len 3)) (ref r826))) (separator dot) (member (ref r827))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r828)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69762) (line 1480) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r829)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r830)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69845) (line 1481) (column 79) (len 5)) (member-access (base (expression (span (offset 69845) (line 1481) (column 79) (len 3)) (ref r831))) (separator dot) (member (ref r832))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r833)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69867) (line 1481) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 69868) (line 1481) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r834)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r835)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69942) (line 1482) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 69943) (line 1482) (column 71) (len 6)) (ref r836))) (element comma (expression (span (offset 69951) (line 1482) (column 79) (len 10)) (ref r837))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 69979) (line 1485) (column 7) (len 42)) (normalized "ISO-80000-4 item 4-30.3 mass change rate "))) (attribute-def (declaration-name "MassChangeRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r838)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 70109) (line 1488) (column 11) (len 484)) (normalized "source: item 4-30.3 mass change rate\nsymbol(s): `q_m`\napplication domain: generic\nname: MassChangeRate\nquantity dimension: M^1*T^-1\nmeasurement unit(s): kg*s^-1\ntensor order: 0\ndefinition: rate of increment of mass `m` (item 4-1): `q_m = (dm)/(dt)` where `dm` is the infinitesimal mass (item 4-1) increment and `dt` is the infinitesimal duration (ISO 80000-3)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r839)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r840)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r841)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r842)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassChangeRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r843)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r844)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r845)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70900) (line 1506) (column 75) (len 5)) (member-access (base (expression (span (offset 70900) (line 1506) (column 75) (len 3)) (ref r846))) (separator dot) (member (ref r847))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r848)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70922) (line 1506) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r849)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r850)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71005) (line 1507) (column 79) (len 5)) (member-access (base (expression (span (offset 71005) (line 1507) (column 79) (len 3)) (ref r851))) (separator dot) (member (ref r852))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r853)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71027) (line 1507) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 71028) (line 1507) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r854)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r855)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71102) (line 1508) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 71103) (line 1508) (column 71) (len 6)) (ref r856))) (element comma (expression (span (offset 71111) (line 1508) (column 79) (len 10)) (ref r857))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 71139) (line 1511) (column 7) (len 40)) (normalized "ISO-80000-4 item 4-31 volume flow rate "))) (attribute-def (declaration-name "VolumeFlowRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r858)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 71267) (line 1514) (column 11) (len 641)) (normalized "source: item 4-31 volume flow rate\nsymbol(s): `q_v`\napplication domain: generic\nname: VolumeFlowRate\nquantity dimension: L^3*T^-1\nmeasurement unit(s): m^3*s^-1\ntensor order: 0\ndefinition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with the normal vector `vec(e)_n` of a flowing fluid with velocity `vec(v)` (ISO 80000-3) as an integral: `q_v = int int_A vec(v) * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r859)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r860)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r861)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r862)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "VolumeFlowRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r863)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r864)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r865)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 72217) (line 1532) (column 77) (len 5)) (member-access (base (expression (span (offset 72217) (line 1532) (column 77) (len 3)) (ref r866))) (separator dot) (member (ref r867))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r868)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 72239) (line 1532) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r869)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r870)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 72322) (line 1533) (column 79) (len 5)) (member-access (base (expression (span (offset 72322) (line 1533) (column 79) (len 3)) (ref r871))) (separator dot) (member (ref r872))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r873)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 72344) (line 1533) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 72345) (line 1533) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r874)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r875)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 72419) (line 1534) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 72420) (line 1534) (column 71) (len 8)) (ref r876))) (element comma (expression (span (offset 72430) (line 1534) (column 81) (len 10)) (ref r877))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 72458) (line 1537) (column 7) (len 39)) (normalized "ISO-80000-4 item 4-32 action quantity "))) (attribute-def (declaration-name "ActionQuantityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r878)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 72585) (line 1540) (column 11) (len 609)) (normalized "source: item 4-32 action quantity\nsymbol(s): `S`\napplication domain: generic\nname: ActionQuantity\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): J*s, kg*m^2*s^-1\ntensor order: 0\ndefinition: time integral of energy `E` over a time interval `(t_1, t_2)`: `S = int_(t_1)^(t_2) E dt`\nremarks: The energy may be expressed by a Lagrangian or Hamiltonian function. Note for SysML: the ISQ quantity \"action\" has been renamed to \"action quantity\" to avoid the name clash with the SysML action keyword.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r879)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r880)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r881)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r882)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ActionQuantityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r883)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r884)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r885)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73503) (line 1558) (column 77) (len 5)) (member-access (base (expression (span (offset 73503) (line 1558) (column 77) (len 3)) (ref r886))) (separator dot) (member (ref r887))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r888)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73525) (line 1558) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r889)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r890)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73604) (line 1559) (column 75) (len 5)) (member-access (base (expression (span (offset 73604) (line 1559) (column 75) (len 3)) (ref r891))) (separator dot) (member (ref r892))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r893)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73626) (line 1559) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r894)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r895)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73709) (line 1560) (column 79) (len 5)) (member-access (base (expression (span (offset 73709) (line 1560) (column 79) (len 3)) (ref r896))) (separator dot) (member (ref r897))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r898)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73731) (line 1560) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 73732) (line 1560) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r899)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r900)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73806) (line 1561) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 73807) (line 1561) (column 71) (len 8)) (ref r901))) (element comma (expression (span (offset 73817) (line 1561) (column 81) (len 6)) (ref r902))) (element comma (expression (span (offset 73825) (line 1561) (column 89) (len 10)) (ref r903))))))))) (body semicolon)))))))))
)
~~~
