# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQAtomicNuclear"))
~~~
# SOURCE
~~~sysml
standard library package ISQAtomicNuclear {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-10:2019 "Atomic and nuclear physics"
     * see also https://www.iso.org/standard/64980.html
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
    private import ISQChemistryMolecular::DiffusionCoefficientUnit;
    private import ISQChemistryMolecular::DiffusionCoefficientValue;
    private import ISQChemistryMolecular::diffusionCoefficient;    
    private import ISQElectromagnetism::ElectricChargeValue;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AreaValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-10 item 10-1.1 atomic number, proton number */
    attribute atomicNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.1 atomic number, proton number
         * symbol(s): `Z`
         * application domain: generic
         * name: AtomicNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of protons in an atomic nucleus
         * remarks: A nuclide is a species of atom with specified numbers of protons and neutrons. Nuclides with the same value of `Z` but different values of `N` are called isotopes of an element. The ordinal number of an element in the periodic table is equal to the atomic number. The atomic number equals the quotient of the charge (IEC 80000-6) of the nucleus and the elementary charge (ISO 80000-1).
         */
    }

    alias protonNumber for atomicNumber;

    /* ISO-80000-10 item 10-1.2 neutron number */
    attribute neutronNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.2 neutron number
         * symbol(s): `N`
         * application domain: generic
         * name: NeutronNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of neutrons in an atomic nucleus
         * remarks: Nuclides with the same value of `N` but different values of `Z` are called isotones. `N - Z` is called the neutron excess number.
         */
    }

    /* ISO-80000-10 item 10-1.3 nucleon number, mass number */
    attribute nucleonNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.3 nucleon number, mass number
         * symbol(s): `A`
         * application domain: generic
         * name: NucleonNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of nucleons in an atomic nucleus
         * remarks: `A` = `Z` + `N` Nuclides with the same value of `A` are called isobars.
         */
    }

    alias massNumber for nucleonNumber;

    /* ISO-80000-10 item 10-2 rest mass, proper mass */
    attribute restMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-2 rest mass, proper mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: RestMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: for particle X, mass (ISO 80000-4) of that particle at rest in an inertial frame
         * remarks: EXAMPLE `m(H_2O)` for a water molecule, `m_e` for an electron. Rest mass is often denoted `m_0`. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    alias properMass for restMass;

    /* ISO-80000-10 item 10-3 rest energy */
    attribute restEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-3 rest energy
         * symbol(s): `E_0`
         * application domain: generic
         * name: RestEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy `E_0` (ISO 80000-5) of a particle at rest: `E_0 = m_0 c_0^2` where `m_0` is the rest mass (item 10-2) of that particle, and `c_0` is speed of light in vacuum (ISO 80000-1)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-4.1 atomic mass */
    attribute atomicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.1 atomic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: AtomicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of an atom X in the ground state
         * remarks: `m(X)/m_u` is called the relative atomic mass. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.2 nuclidic mass */
    attribute nuclidicMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.2 nuclidic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: NuclidicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of a nuclide X in the ground state
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-4.3 unified atomic mass constant */
    attribute unifiedAtomicMassConstant: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.3 unified atomic mass constant
         * symbol(s): `m_u`
         * application domain: generic
         * name: UnifiedAtomicMassConstant (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: 1/12 of the mass (ISO 80000-4) of an atom of the nuclide ^(12)C in the ground state at rest
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }

    /* ISO-80000-10 item 10-5.1 elementary charge */
    attribute elementaryCharge: ElectricChargeValue :> scalarQuantities {
        doc
        /*
         * source: item 10-5.1 elementary charge
         * symbol(s): `e`
         * application domain: generic
         * name: ElementaryCharge (specializes ElectricCharge)
         * quantity dimension: T^1*I^1
         * measurement unit(s): C, s*A
         * tensor order: 0
         * definition: one of the fundamental constants in the SI system (ISO 80000-1), equal to the charge of the proton and opposite to the charge of the electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-5.2 charge number, ionization number */
    attribute def ChargeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-5.2 charge number, ionization number
         * symbol(s): `c`
         * application domain: generic
         * name: ChargeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a particle, quotient of the electric charge (IEC 80000-6) and the elementary charge (ISO 80000-1)
         * remarks: A particle is said to be electrically neutral if its charge number is equal to zero. The charge number of a particle can be positive, negative, or zero. The state of charge of a particle may be presented as a superscript to the symbol of that particle, e.g. `H^+, He^(++), Al^(3+), Cl^-, S^(--), N^(3-)`.
         */
    }
    attribute chargeNumber: ChargeNumberValue :> scalarQuantities;

    alias ionizationNumber for chargeNumber;

    /* ISO-80000-10 item 10-6 Bohr radius */
    attribute bohrRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-6 Bohr radius
         * symbol(s): `a_0`
         * application domain: generic
         * name: BohrRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m, Å
         * tensor order: 0
         * definition: radius (ISO 80000-3) of the electron orbital in the hydrogen atom in its ground state in the Bohr model of the atom: `a_0 = (4 π ε_0 ℏ^2)/(m_e e^2)` where `ε_0` is the electric constant (IEC 80000-6), `ℏ` is the reduced Planck constant (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `e` is the elementary charge (ISO 80000-1)
         * remarks: The radius of the electron orbital in the H atom in its ground state is `a_0` in the Bohr model of the atom. ångström (Å), `1 Å := 10^-10 m`.
         */
    }

    /* ISO-80000-10 item 10-7 Rydberg constant */
    attribute def RydbergConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-7 Rydberg constant
         * symbol(s): `R_∞`
         * application domain: generic
         * name: RydbergConstant
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: spectroscopic constant that determines the wave numbers of the lines in the spectrum of hydrogen: `R_(oo) = e^2/(8 π ε_0 a_0 h c_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `a_0` is the Bohr radius (item 10-6), `h` is the Planck constant (ISO 80000-1), and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The quantity `R_y = R_∞ h c_0` is called the Rydberg energy.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RydbergConstantUnit[1];
    }

    attribute rydbergConstant: RydbergConstantValue[*] nonunique :> scalarQuantities;

    attribute def RydbergConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-8 Hartree energy */
    attribute def HartreeEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-8 Hartree energy
         * symbol(s): `E_H`, `E_h`
         * application domain: generic
         * name: HartreeEnergy
         * quantity dimension: L^6*M^3*T^-6
         * measurement unit(s): eV*J*kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) of the electron in a hydrogen atom in its ground state: `E_H = e^2/(4 π ε_0 a_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), and `a_0` is the Bohr radius (item 10-6)
         * remarks: The energy of the electron in an H atom in its ground state is `E_H`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HartreeEnergyUnit[1];
    }

    attribute hartreeEnergy: HartreeEnergyValue[*] nonunique :> scalarQuantities;

    attribute def HartreeEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 6; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -6; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-9.1 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (magnitude)
         * symbol(s): `μ`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticDipoleMomentUnit[1];
    }

    attribute magneticDipoleMoment: MagneticDipoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (vector)
         * symbol(s): `vec(μ)`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 1
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticDipoleMoment3dCoordinateFrame[1];
    }

    attribute cartesianMagneticDipoleMoment3dVector: CartesianMagneticDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticDipoleMomentUnit[3];
    }

    /* ISO-80000-10 item 10-9.2 Bohr magneton */
    attribute bohrMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.2 Bohr magneton
         * symbol(s): `μ_B`
         * application domain: generic
         * name: BohrMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: magnitude of the magnetic moment of an electron in a state with orbital angular momentum quantum number `l`=1 (item 10-13.3) due to its orbital motion: `μ_B = (e ℏ)/(2 m_e)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_e` is the rest mass (item 10-2) of electron
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-9.3 nuclear magneton */
    attribute nuclearMagneton: MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.3 nuclear magneton
         * symbol(s): `μ_N`
         * application domain: generic
         * name: NuclearMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: absolute value of the magnetic moment of a nucleus: `μ_N = (e ℏ)/(2 m_p)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_p` is the rest mass (item 10-2) of proton
         * remarks: Subscript N stands for nucleus. For the neutron magnetic moment, subscript n is used. The magnetic moments of protons and neutrons differ from this quantity by their specific `g` factors (item 10-14.2).
         */
    }

    /* ISO-80000-10 item 10-10 spin */
    attribute def SpinValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-10 spin (magnitude)
         * symbol(s): `s`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpinUnit[1];
    }

    attribute spin: SpinValue[*] nonunique :> scalarQuantities;

    attribute def SpinUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianSpin3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-10 spin (vector)
         * symbol(s): `vec(s)`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpin3dCoordinateFrame[1];
    }

    attribute cartesianSpin3dVector: CartesianSpin3dVector :> vectorQuantities;

    attribute def CartesianSpin3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SpinUnit[3];
    }

    /* ISO-80000-10 item 10-11 total angular momentum */
    attribute def TotalAngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-11 total angular momentum (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalAngularMomentumUnit[1];
    }

    attribute totalAngularMomentum: TotalAngularMomentumValue[*] nonunique :> scalarQuantities;

    attribute def TotalAngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianTotalAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-11 total angular momentum (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianTotalAngularMomentum3dCoordinateFrame[1];
    }

    attribute cartesianTotalAngularMomentum3dVector: CartesianTotalAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianTotalAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: TotalAngularMomentumUnit[3];
    }

    /* ISO-80000-10 item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient */
    attribute def GyromagneticRatioValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: GyromagneticRatio
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum: `vec(μ)` = `γ` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1 The systematic name is "gyromagnetic coefficient", but "gyromagnetic ratio" is more usual. The gyromagnetic ratio of the proton is denoted by `γ_p`. The gyromagnetic ratio of the neutron is denoted by `γ_n`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioUnit[1];
    }

    attribute gyromagneticRatio: GyromagneticRatioValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioUnit for GyromagneticRatioUnit;
    alias MagnetogyricRatioValue for GyromagneticRatioValue;
    alias magnetogyricRatio for gyromagneticRatio;

    alias GyromagneticCoefficientUnit for GyromagneticRatioUnit;
    alias GyromagneticCoefficientValue for GyromagneticRatioValue;
    alias gyromagneticCoefficient for gyromagneticRatio;

    /* ISO-80000-10 item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron */
    attribute def GyromagneticRatioOfTheElectronValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron
         * symbol(s): `γ_e`
         * application domain: generic
         * name: GyromagneticRatioOfTheElectron
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum of the electron `vec(μ)` = `γ_e` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1
         */
        attribute :>> num: Real;
        attribute :>> mRef: GyromagneticRatioOfTheElectronUnit[1];
    }

    attribute gyromagneticRatioOfTheElectron: GyromagneticRatioOfTheElectronValue[*] nonunique :> scalarQuantities;

    attribute def GyromagneticRatioOfTheElectronUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    alias MagnetogyricRatioOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias MagnetogyricRatioOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias magnetogyricRatioOfTheElectron for gyromagneticRatioOfTheElectron;

    alias GyromagneticCoefficientOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias GyromagneticCoefficientOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias gyromagneticCoefficientOfTheElectron for gyromagneticRatioOfTheElectron;

    /* ISO-80000-10 item 10-13.1 quantum number */
    attribute def QuantumNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-13.1 quantum number
         * symbol(s): `N`, `L`, `M`, `j`, `s`, `F`
         * application domain: generic
         * name: QuantumNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number describing a particular state of a quantum system
         * remarks: Electron states determine the binding energy `E = E(n,l,m,j,s,f)` in an atom. Upper case letters `N, L, M, J, S, F` are usually used for the whole system. The spatial probability distribution of an electron is given by `|Ψ|^2`, where `Ψ` is its wave function. For an electron in an H atom in a non-relativistic approximation, the wave function can be presented as: `Ψ(r,θ,φ) = R_(nl)(r)*Y_l^m(θ,φ)`, where `r,θ,φ` are spherical coordinates (ISO 80000-2) with respect to the nucleus and to a given (quantization) axis, `R_(nl)(r)` is the radial distribution function, and `Y_l^m(θ,φ)` are spherical harmonics. In the Bohr model of one-electron atoms, `n`, `l`, and `m` define the possible orbits of an electron about the nucleus.
         */
    }
    attribute quantumNumber: QuantumNumberValue :> scalarQuantities;

    /* ISO-80000-10 item 10-13.2 principal quantum number */
    attribute principalQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.2 principal quantum number
         * symbol(s): `n`
         * application domain: generic
         * name: PrincipalQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the number `n`-1 of radial nodes of one-electron wave functions
         * remarks: In the Bohr model, `n = 1,2,…,∞` is related to the binding energy of an electron and the radius of spherical orbits (principal axis of the elliptic orbits). For an electron in an H atom, the semi-classical radius of its orbit is `r_n = a_0 n^2` and its binding energy is `E_n = E_H/n^2`.
         */
    }

    /* ISO-80000-10 item 10-13.3 orbital angular momentum quantum number */
    attribute orbitalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.3 orbital angular momentum quantum number
         * symbol(s): `l`, `l_i`, `L`
         * application domain: generic
         * name: OrbitalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the orbital angular momentum `l` of a one-electron state
         * remarks: `abs(l)^2 = ℏ^2 l (l-1)` , `l = 0, 1, …, n-1` where `vec(l)` is the orbital angular momentum and `ℏ` is the reduced Planck constant (ISO 80000-1). If reference is made to a specific particle `i`, the symbol `l_i` is used instead of `l`; if reference is made to the whole system, the symbol `L` is used instead of `l`. An electron in an H atom for `l = 0` appears as a spherical cloud. In the Bohr model, it is related to the form of the orbit.
         */
    }

    /* ISO-80000-10 item 10-13.4 magnetic quantum number */
    attribute magneticQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.4 magnetic quantum number
         * symbol(s): `m`, `m_i`, `M`
         * application domain: generic
         * name: MagneticQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the `z` component `l_z`, `j_z` or `s_z`, of the orbital, total, or spin angular momentum
         * remarks: `l_z = m_l ℏ` , `j_z = m_j ℏ` , and `s_z = m_s ℏ` , with the ranges from `-l` to `l`, from `-j` to `j`, and `±1/2`, respectively. `m_i` refers to a specific particle `i`. `M` is used for the whole system. Subscripts `l`, `s`, `j`, etc., as appropriate, indicate the angular momentum involved. `ℏ` is the reduced Planck constant (ISO 80000-1).
         */
    }

    /* ISO-80000-10 item 10-13.5 spin quantum number */
    attribute spinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.5 spin quantum number
         * symbol(s): `s`
         * application domain: generic
         * name: SpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characteristic quantum number `s` of a particle, related to its spin (item 10-10), `vec(s)`: `s^2 = ℏ^2 s (s+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Spin quantum numbers of fermions are odd multiples of 1/2, and those of bosons are integers.
         */
    }

    /* ISO-80000-10 item 10-13.6 total angular momentum quantum number */
    attribute totalAngularMomentumQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.6 total angular momentum quantum number
         * symbol(s): `j`, `j_i`, `J`
         * application domain: generic
         * name: TotalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number in an atom describing the magnitude of total angular momentum `vec(J)` (item 10-11)
         * remarks: `j_i` refers to a specific particle `i`; `J` is used for the whole system. The quantum number `J` and the magnitude of total angular momentum `vec(J)` (item 10-11) are different quantities. The two values of `j` are `l`±1/2. (See item 10-13.3.)
         */
    }

    /* ISO-80000-10 item 10-13.7 nuclear spin quantum number */
    attribute nuclearSpinQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.7 nuclear spin quantum number
         * symbol(s): `I`
         * application domain: generic
         * name: NuclearSpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number related to the total angular momentum (item 10-11), `vec(J)`, of a nucleus in any specified state, normally called nuclear spin: `vec(J)^2 = ℏ^2 I (I+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Nuclear spin is composed of spins of the nucleons (protons and neutrons) and their (orbital) motions. In principle there is no upper limit for the nuclear spin quantum number. It has possible values `I` = 0,1,2,… for even `A` and `I = 1/2, 3/2, …` for odd `A`. In nuclear and particle physics, `vec(J)` is often used.
         */
    }

    /* ISO-80000-10 item 10-13.8 hyperfine structure quantum number */
    attribute hyperfineStructureQuantumNumber: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.8 hyperfine structure quantum number
         * symbol(s): `F`
         * application domain: generic
         * name: HyperfineStructureQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number of an atom describing the inclination of the nuclear spin with respect to a quantization axis given by the magnetic field produced by the orbital electrons
         * remarks: The interval of `F` is │`I`-`J`│, │`I`-`J`│+1, ..., `I`-`J`. This is related to the hyperfine splitting of the atomic energy levels due to the interaction between the electron and nuclear magnetic moments.
         */
    }

    /* ISO-80000-10 item 10-14.1 Lande factor, g factor of atom */
    attribute def LandeFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.1 Lande factor, g factor of atom
         * symbol(s): `g`
         * application domain: generic
         * name: LandeFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the total angular momentum quantum number and the Bohr magneton: `g = μ/(J*μ_B)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `J` is total angular momentum quantum number (item 10-13.6), and `μ_B` is the Bohr magneton (item 10-9.2)
         * remarks: These quantities are also called `g` values. The Landé factor can be calculated from the expression: `g(L, S, J) = 1 + (g_e -1) xx (J(J+1) + S(S+1) - L(L+1))/(2J(J+1))` where `g_e` is the` g` factor of the electron.
         */
    }
    attribute landeFactor: LandeFactorValue :> scalarQuantities;

    alias gFactorOfAtom for landeFactor;

    /* ISO-80000-10 item 10-14.2 g factor of nucleus or nuclear particle */
    attribute def GFactorOfNucleusOrNuclearParticleValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.2 g factor of nucleus or nuclear particle
         * symbol(s): `g`
         * application domain: generic
         * name: GFactorOfNucleusOrNuclearParticle (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the nuclear spin quantum number and the nuclear magneton: `g = μ/(I*μ_N)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `I` is nuclear spin quantum number (item 10-13.7), and `μ_N` is the nuclear magneton (item 10-9.3)
         * remarks: The `g` factors for nuclei or nucleons are known from measurements.
         */
    }
    attribute gFactorOfNucleusOrNuclearParticle: GFactorOfNucleusOrNuclearParticleValue :> scalarQuantities;

    /* ISO-80000-10 item 10-15.1 Larmor angular frequency */
    attribute larmorAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.1 Larmor angular frequency
         * symbol(s): `ω_L`
         * application domain: generic
         * name: LarmorAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: angular frequency (ISO 80000-3) of the electron angular momentum (ISO 80000-4) vector precession about the axis of an external magnetic field: `ω_L = e/(2 m_e) B` where `e` is the elementary charge (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-15.2 Larmor frequency */
    attribute def LarmorFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-15.2 Larmor frequency
         * symbol(s): `ν_L`
         * application domain: generic
         * name: LarmorFrequency
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of Larmor angular frequency (ISO 80000-3) and 2π
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LarmorFrequencyUnit[1];
    }

    attribute larmorFrequency: LarmorFrequencyValue[*] nonunique :> scalarQuantities;

    attribute def LarmorFrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-15.3 nuclear precession angular frequency */
    attribute nuclearPrecessionAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.3 nuclear precession angular frequency
         * symbol(s): `ω_N`
         * application domain: generic
         * name: NuclearPrecessionAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: frequency (ISO 80000-3) by which the nucleus angular momentum vector (ISO 80000-4) precesses about the axis of an external magnetic field: `ω_N` = `γ` `B` where `γ` is the gyromagnetic ratio (item 10-12.1), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-16 cyclotron angular frequency */
    attribute cyclotronAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-16 cyclotron angular frequency
         * symbol(s): `ω_c`
         * application domain: generic
         * name: CyclotronAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: quotient of the product of the electric charge of a particle and the magnitude of the magnetic flux density of the magnetic field, and the particle mass: `ω_c = abs(q)/m B` where `q` is the electric charge (IEC 80000-6) of the particle, `m` is the mass (ISO 80000-4) of the particle, and `B` is the absolute value of the magnetic flux density (IEC 80000-6)
         * remarks: The quantity `v_c` = `ω_c`/2π is called the cyclotron frequency.
         */
    }

    /* ISO-80000-10 item 10-17 gyroradius, Larmor radius */
    attribute gyroradius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-17 gyroradius, Larmor radius
         * symbol(s): `r_g`, `r_L`
         * application domain: generic
         * name: Gyroradius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (ISO 80000-3) of circular movement of a particle with mass (ISO 80000-4), velocity `vec(v)` (ISO 80000-3), and electric charge `q` (IEC 80000-6), moving in a magnetic field with magnetic flux density `vec(B)` (IEC 80000-6): `r_g = (m abs(vec(v) xx vec(B)))/(q B^2)`
         * remarks: None.
         */
    }

    alias larmorRadius for gyroradius;

    /* ISO-80000-10 item 10-18 nuclear quadrupole moment */
    attribute def NuclearQuadrupoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-18 nuclear quadrupole moment
         * symbol(s): `Q`
         * application domain: generic
         * name: NuclearQuadrupoleMoment
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: `z` component of the diagonalized tensor of nuclear quadrupole moment: `Q = (1/e) int (3z^2 - r^2) ρ(x, y, z) dV` in the quantum state with the nuclear spin in the field direction (`z`), where `e` is the elementary charge (ISO 80000-1), `r^2 = x^2 + y^2 + z^2`, `ρ(x,y,z)` is the nuclear electric charge density (IEC 80000-6), and `dV` is the volume element `dx dy dz`
         * remarks: The electric nuclear quadrupole moment is `eQ`. This value is equal to the `z` component of the diagonalized tensor of quadrupole moment.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearQuadrupoleMomentUnit[1];
    }

    attribute nuclearQuadrupoleMoment: NuclearQuadrupoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def NuclearQuadrupoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-19.1 nuclear radius */
    attribute nuclearRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.1 nuclear radius
         * symbol(s): `R`
         * application domain: generic
         * name: NuclearRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: conventional radius (ISO 80000-3) of sphere in which the nuclear matter is included
         * remarks: This quantity is not exactly defined. It is given approximately for nuclei in their ground state by: `R = r_0 A^(1//3)` where `r_0 ~~ 1.2 * 10^-15` m, and `A` is the nucleon number (item 10-1.3). Nuclear radius is usually expressed in femtometres, 1 fm = 10^(-15) m.
         */
    }

    /* ISO-80000-10 item 10-19.2 electron radius */
    attribute electronRadius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.2 electron radius
         * symbol(s): `r_e`
         * application domain: generic
         * name: ElectronRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius of a sphere such that the relativistic electron energy is distributed uniformly: `r_e = e^2/(4 π ε_0 m_e c_0^2)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `m_e` is the rest mass (item 10-2) of electron, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: This quantity corresponds to the electrostatic energy `E` of a charge distributed inside a sphere of radius `r_e` as if all the rest energy (item 10-3) of the electron were attributed to the energy of electromagnetic origin, using the relation `E = m_e c_0^2`.
         */
    }

    /* ISO-80000-10 item 10-20 Compton wavelength */
    attribute comptonWavelength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-20 Compton wavelength
         * symbol(s): `λ_C`
         * application domain: generic
         * name: ComptonWavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: quotient of the Planck constant and the product of the mass of the particle and the speed of light in vacuum: `λ_C = h / (m c_0)` where `h` is the Planck constant (ISO 80000-1), `m` is the rest mass (item 10-2) of a particle, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The wavelength of electromagnetic radiation scattered from free electrons (Compton scattering) is larger than that of the incident radiation by a maximum of 2`λ_C`.
         */
    }

    /* ISO-80000-10 item 10-21.1 mass excess */
    attribute massExcess: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.1 mass excess
         * symbol(s): `Δ`
         * application domain: generic
         * name: MassExcess (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: difference between the mass of an atom, and the product of its mass number and the unified mass constant: `Δ = m_a - A*m_u`, where `m_a` is the rest mass (item 10-2) of the atom, `A` is its nucleon number (item 10-1.3), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. See item 10-2.
         */
    }

    /* ISO-80000-10 item 10-21.2 mass defect */
    attribute massDefect: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.2 mass defect
         * symbol(s): `B`
         * application domain: generic
         * name: MassDefect (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: sum of the product of the proton number and the hydrogen atomic mass, and the neutron rest mass, minus the rest mass of the atom: `B = Z*m(⁢^1"H") + N*m_n - m_a` where `Z` is the proton number (item 10-1.1) of the atom, `m(⁢^1"H")` is atomic mass (item 10-4.1) of `⁢^1"H"`, `N` is neutron number (item 10-1.2), `m_n` is the rest mass (item 10-2) of the neutron, and `m_a` is the rest mass (item 10-2) of the atom
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. If the binding energy of the orbital electrons is neglected, `B c_0^2` is equal to the binding energy of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-22.1 relative mass excess */
    attribute def RelativeMassExcessValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.1 relative mass excess
         * symbol(s): `Δ_r`
         * application domain: generic
         * name: RelativeMassExcess (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass excess and the unified atomic mass constant: `Δ_r = Δ/m_u` where `Δ` is mass excess (item 10-21.1), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassExcess: RelativeMassExcessValue :> scalarQuantities;

    /* ISO-80000-10 item 10-22.2 relative mass defect */
    attribute def RelativeMassDefectValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.2 relative mass defect
         * symbol(s): `B_r`
         * application domain: generic
         * name: RelativeMassDefect (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass defect and the unified atomic mass constant: `B_r = B/m_u` where `B` is mass defect (item 10-21.2), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassDefect: RelativeMassDefectValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.1 packing fraction */
    attribute def PackingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.1 packing fraction
         * symbol(s): `f`
         * application domain: generic
         * name: PackingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass excess and the nucleon number: `f` = Δ_r/A` where `Δ_r` is relative mass excess (item 10-22.1), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute packingFraction: PackingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-23.2 binding fraction */
    attribute def BindingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.2 binding fraction
         * symbol(s): `b`
         * application domain: generic
         * name: BindingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass defect and the nucleon number: `b = B_r/A` where `B_r` is relative mass defect (item 10-22.2), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute bindingFraction: BindingFractionValue :> scalarQuantities;

    /* ISO-80000-10 item 10-24 decay constant, disintegration constant */
    attribute def DecayConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-24 decay constant, disintegration constant
         * symbol(s): `λ`
         * application domain: generic
         * name: DecayConstant
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of `(-dN)/N` and `dt`, where `(dN)/N` is the mean fractional change in the number of nuclei in a particular energy state due to spontaneous transformations in a time interval of duration (ISO 80000-3) `dt`: `λ = -1/N (dN)/(dt)`
         * remarks: For exponential decay, this quantity is constant. For more than one decay channel, `λ = sum λ_a` where `λ_a` denotes the decay constant for a specified final state and the sum is taken over all final states.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DecayConstantUnit[1];
    }

    attribute decayConstant: DecayConstantValue[*] nonunique :> scalarQuantities;

    attribute def DecayConstantUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    alias DisintegrationConstantUnit for DecayConstantUnit;
    alias DisintegrationConstantValue for DecayConstantValue;
    alias disintegrationConstant for decayConstant;

    /* ISO-80000-10 item 10-25 mean duration of life, mean life time */
    attribute meanDurationOfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-25 mean duration of life, mean life time
         * symbol(s): `τ`
         * application domain: atomic and nuclear physics
         * name: MeanDurationOfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: reciprocal of the decay constant `λ` (item 10-24): `τ = 1/λ`
         * remarks: Mean duration of life is the expected value of the duration of life of an unstable particle or an excited state of a particle when the number of decay events in a short time interval follows a Poisson distribution.
         */
    }

    alias meanLifeTime for meanDurationOfLife;

    /* ISO-80000-10 item 10-26 level width */
    attribute levelWidth: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-26 level width
         * symbol(s): `Γ`
         * application domain: generic
         * name: LevelWidth (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the reduced Planck constant and the mean life: `Γ = ℏ/τ` where `ℏ` is the reduced Planck constant (ISO 80000-1), and `τ` is mean duration of life (item 10-25)
         * remarks: Level width is the uncertainty of the energy of an unstable particle or an excited state of a system due to the Heisenberg principle. The term energy level refers to the configuration of the distribution function of the density of states. Energy levels may be considered as discrete, like those in an atom, or may have a finite width, like e.g. this item or like e.g. the valence or conduction band in solid state physics. Energy levels are applicable to both real and virtual particles, e.g. electrons and phonons, respectively.
         */
    }

    /* ISO-80000-10 item 10-27 nuclear activity */
    attribute def NuclearActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-27 nuclear activity
         * symbol(s): `A`
         * application domain: generic
         * name: NuclearActivity
         * quantity dimension: T^-1
         * measurement unit(s): Bq, s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the mean change in the number of nuclei in a particular energy state due to spontaneous nuclear transformations in a time interval of duration (ISO 80000-3) `dt`: `A = -(dN)/(dt)`
         * remarks: For exponential decay, `A = λN`, where `λ` is the decay constant (item 10-24). The becquerel (Bq) is a special name for second to the power minus one, to be used as the coherent SI unit of activity. In report 85a of the ICRU a definition with an equivalent meaning is given as: The activity, `A`, of an amount of a radionuclide in a particular energy state at a given time is the quotient of `-dN` by `dt`, where `dN` is the mean change in the number of nuclei in that energy state due to spontaneous nuclear transformations in the time interval `dt`: `A = -(dN)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NuclearActivityUnit[1];
    }

    attribute nuclearActivity: NuclearActivityValue[*] nonunique :> scalarQuantities;

    attribute def NuclearActivityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-28 specific activity, massic activity */
    attribute def SpecificActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-28 specific activity, massic activity
         * symbol(s): `a`
         * application domain: generic
         * name: SpecificActivity
         * quantity dimension: M^-1*T^-1
         * measurement unit(s): Bq/kg, kg^-1*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificActivityUnit[1];
    }

    attribute specificActivity: SpecificActivityValue[*] nonunique :> scalarQuantities;

    attribute def SpecificActivityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias MassicActivityUnit for SpecificActivityUnit;
    alias MassicActivityValue for SpecificActivityValue;
    alias massicActivity for specificActivity;

    /* ISO-80000-10 item 10-29 activity density, volumic activity, activity concentration */
    attribute def ActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-29 activity density, volumic activity, activity concentration
         * symbol(s): `c_A`
         * application domain: generic
         * name: ActivityDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): Bq/m^3, m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ActivityDensityUnit[1];
    }

    attribute activityDensity: ActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def ActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias VolumicActivityUnit for ActivityDensityUnit;
    alias VolumicActivityValue for ActivityDensityValue;
    alias volumicActivity for activityDensity;

    alias ActivityConcentrationUnit for ActivityDensityUnit;
    alias ActivityConcentrationValue for ActivityDensityValue;
    alias activityConcentration for activityDensity;

    /* ISO-80000-10 item 10-30 surface-activity density */
    attribute def SurfaceActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-30 surface-activity density
         * symbol(s): `a_S`
         * application domain: generic
         * name: SurfaceActivityDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): Bq/m^2, m^-2*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the total area `S` (ISO 80000-3) of the surface of that sample: `a_S` = `A`/`S`
         * remarks: This value is usually defined for flat sources, where `S` corresponds to the total area of surface of one side of the source.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceActivityDensityUnit[1];
    }

    attribute surfaceActivityDensity: SurfaceActivityDensityValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-31 half life */
    attribute halfLife: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-31 half life
         * symbol(s): `T_(1/2)`
         * application domain: generic
         * name: HalfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: mean duration (ISO 80000-3) required for the decay of one half of the atoms or nuclei
         * remarks: For exponential decay, `T_(1/2) = (ln2)/λ`, where `λ` is the decay constant (item 10-24).
         */
    }

    /* ISO-80000-10 item 10-32 alpha disintegration energy */
    attribute alphaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-32 alpha disintegration energy
         * symbol(s): `Q_α`
         * application domain: generic
         * name: AlphaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the kinetic energy (ISO 80000-4) of the α-particle produced in the disintegration process and the recoil energy (ISO 80000-5) of the product atom in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: The ground-state alpha disintegration energy, `Q_(α,0)`, also includes the energy of any nuclear transitions that take place in the daughter produced.
         */
    }

    /* ISO-80000-10 item 10-33 maximum beta-particle energy */
    attribute maximumBetaParticleEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-33 maximum beta-particle energy
         * symbol(s): `E_β`
         * application domain: generic
         * name: MaximumBetaParticleEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: maximum kinetic energy (ISO 80000-4) of the emitted beta particle produced in the nuclear disintegration process
         * remarks: The maximum kinetic energy corresponds to the highest energy of the beta spectrum.
         */
    }

    /* ISO-80000-10 item 10-34 beta disintegration energy */
    attribute betaDisintegrationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-34 beta disintegration energy
         * symbol(s): `Q_β`
         * application domain: generic
         * name: BetaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the maximum beta-particle kinetic energy (item 10-33) and the recoil energy (ISO 80000-5) of the atom produced in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: For positron emitters, the energy for the production of the annihilation radiation created in the combination of an electron with the positron is part of the beta disintegration energy. The ground-state beta disintegration energy, `Q_(β,0)`, also includes the energy of any nuclear transitions that take place in the daughter product.
         */
    }

    /* ISO-80000-10 item 10-35 internal conversion factor */
    attribute def InternalConversionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-35 internal conversion factor
         * symbol(s): `α`
         * application domain: generic
         * name: InternalConversionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the number of internal conversion electrons and the number of gamma quanta emitted by the radioactive atom in a given transition, where a conversion electron represents an orbital electron emitted through the radioactive decay
         * remarks: The quantity `α/(α+1)` is also used and called the internal-conversion fraction. Partial conversion fractions referring to the various electron shells `K, L, ...` are indicated by `α_K`, `α_L`, ... `α_K/α_L` is called the K-to-L internal conversion ratio.
         */
    }
    attribute internalConversionFactor: InternalConversionFactorValue :> scalarQuantities;

    /* ISO-80000-10 item 10-36 particle emission rate */
    attribute def ParticleEmissionRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-36 particle emission rate
         * symbol(s): `dot(N)`
         * application domain: generic
         * name: ParticleEmissionRate
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the number of particles being emitted from an infinitesimally small volume element in the time interval of duration `dt` (ISO 80000-3), and `dt`: `dot(N) = (dN)/(dt)`
         * remarks: Usually the kind of particles is specified, e.g. neutron emission rate or alpha particle emission rate.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleEmissionRateUnit[1];
    }

    attribute particleEmissionRate: ParticleEmissionRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleEmissionRateUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-10 item 10-37.1 reaction energy */
    attribute reactionEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.1 reaction energy
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactionEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a nuclear reaction, sum of the kinetic energies (ISO 80000-4) and photon energies (ISO 80000-5) of the reaction products minus the sum of the kinetic and photon energies of the reactants
         * remarks: For exothermic nuclear reactions, `Q>0`. For endothermic nuclear reactions, `Q<0`.
         */
    }

    /* ISO-80000-10 item 10-37.2 resonance energy */
    attribute resonanceEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.2 resonance energy
         * symbol(s): `E_r`, `E_"res"`
         * application domain: generic
         * name: ResonanceEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: kinetic energy (ISO 80000-4) of an incident particle, in the reference frame of the target, corresponding to a resonance in a nuclear reaction
         * remarks: The energy of the resonance corresponds to the difference of the energy levels involved of the nucleus.
         */
    }

    /* ISO-80000-10 item 10-38.1 cross section */
    attribute crossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.1 cross section
         * symbol(s): `σ`
         * application domain: atomic physics
         * name: CrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: for a specified target entity and for a specified reaction or process produced by incident charged or uncharged particles of a given type and energy, the quotient of the mean number of such reactions or processes and the incident-particle fluence (item 10-43)
         * remarks: The type of process is indicated by subscripts, e.g. absorption cross section `σ_a`, scattering cross section `σ_s`, fission cross section `σ_f`. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-38.2 total cross section */
    attribute totalCrossSection: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.2 total cross section
         * symbol(s): `σ_"tot"`, `σ_"T"`
         * application domain: atomic physics
         * name: TotalCrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: sum of all cross sections (item 10-38.1) corresponding to the various reactions or processes between an incident particle of specified type and energy (ISO 80000-5) and a target entity
         * remarks: In the case of a narrow unidirectional beam of incident particles, this is the effective cross section for the removal of an incident particle from the beam. See the Remarks for item 10-52. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }

    /* ISO-80000-10 item 10-39 direction distribution of cross section */
    attribute def DirectionDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-39 direction distribution of cross section
         * symbol(s): `σ_Ω`
         * application domain: atomic physics
         * name: DirectionDistributionOfCrossSection
         * quantity dimension: L^2
         * measurement unit(s): m^2*sr^-1, m^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to `Ω`, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a specified direction, and `Ω` is the solid angle (ISO 80000-3) around that direction: `σ_Ω = (dσ)/(dΩ)`
         * remarks: Quantities listed under items 10-39, 10-40 and 10-41 are sometimes called differential cross sections. The type of interaction needs to be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionDistributionOfCrossSectionUnit[1];
    }

    attribute directionDistributionOfCrossSection: DirectionDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-40 energy distribution of cross section */
    attribute def EnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-40 energy distribution of cross section
         * symbol(s): `σ_E`
         * application domain: atomic physics
         * name: EnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/J, kg^-1*s^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to energy, where `σ` is the cross section (item 10-38.1) for a process in which the energy `E` (ISO 80000-5) of the ejected or scattered particle is between `E` and `E + dE`: `σ_E = (dσ)/(dE)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyDistributionOfCrossSectionUnit[1];
    }

    attribute energyDistributionOfCrossSection: EnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def EnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-41 direction and energy distribution of cross section */
    attribute def DirectionAndEnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-41 direction and energy distribution of cross section
         * symbol(s): `σ_(Ω,E)`
         * application domain: atomic physics
         * name: DirectionAndEnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/(J*sr), kg^-1*s^2
         * tensor order: 0
         * definition: partial differential quotient of `σ` with respect to solid angle and energy, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a solid angle `dΩ` around a specified direction and with an energy between `E` and `E+dE`: `σ_(Ω,E) = (del^2 σ) / (del Ω del E)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DirectionAndEnergyDistributionOfCrossSectionUnit[1];
    }

    attribute directionAndEnergyDistributionOfCrossSection: DirectionAndEnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def DirectionAndEnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-42.1 volumic cross section, macroscopic cross section */
    attribute def VolumicCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.1 volumic cross section, macroscopic cross section
         * symbol(s): `Σ`
         * application domain: atomic physics
         * name: VolumicCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and of the cross section (item 10-38.1) `σ_a` for a given type of atoms: `Σ = n_a σ_a`
         * remarks: When the target particles of the medium are at rest, `Σ = 1/l`, where `l` is the mean free path (item 10-71).
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicCrossSectionUnit[1];
    }

    attribute volumicCrossSection: VolumicCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicCrossSectionUnit for VolumicCrossSectionUnit;
    alias MacroscopicCrossSectionValue for VolumicCrossSectionValue;
    alias macroscopicCrossSection for volumicCrossSection;

    /* ISO-80000-10 item 10-42.2 volumic total cross section, macroscopic total cross section */
    attribute def VolumicTotalCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.2 volumic total cross section, macroscopic total cross section
         * symbol(s): `Σ_"tot"`, `Σ_"T"`
         * application domain: atomic physics
         * name: VolumicTotalCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and the cross section (item 10-38.1) `σ_"tot"` for a given type of atoms: `Σ_"tot" = n_a*σ_"tot"`
         * remarks: See the Remarks for item 10-49.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumicTotalCrossSectionUnit[1];
    }

    attribute volumicTotalCrossSection: VolumicTotalCrossSectionValue[*] nonunique :> scalarQuantities;

    attribute def VolumicTotalCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias MacroscopicTotalCrossSectionUnit for VolumicTotalCrossSectionUnit;
    alias MacroscopicTotalCrossSectionValue for VolumicTotalCrossSectionValue;
    alias macroscopicTotalCrossSection for volumicTotalCrossSection;

    /* ISO-80000-10 item 10-43 particle fluence */
    attribute def ParticleFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-43 particle fluence
         * symbol(s): `Φ`
         * application domain: generic
         * name: ParticleFluence
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: differential quotient of `N` with respect to `a`, where `N` is the number of particles incident on a sphere of cross-sectional area `a` (item 10-38.1): `Φ = (dN)/(da)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` fluence. If a flat area of size `dA` is passed perpendicularly by a number of `dN` particles, the corresponding particle fluence is: `Φ = (dN)/(dA)`. A plane area of size `dA` crossed at an angle `α` with respect to the surface normal by a number of `dN` particles results in the particle fluence: `Φ = (dN)/(cos(α) dA)` In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence, `Φ` , is the quotient of `dN` and `da`, where `dN` is the number of particles incident on a sphere of cross-sectional area `da`: `Φ = (dN)/(dA)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceUnit[1];
    }

    attribute particleFluence: ParticleFluenceValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-44 particle fluence rate */
    attribute def ParticleFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-44 particle fluence rate
         * symbol(s): `dot(Φ)`
         * application domain: generic
         * name: ParticleFluenceRate
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: differential quotient of fluence `Φ` (item 10-43) with respect to time (ISO 80000-3): `dot(Φ) = (dΦ)/(dA)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example proton fluence rate. The distribution function expressed in terms of speed and energy, `dot(Φ)_v` and `dot(Φ)_E` , are related to by: `dot(Φ) = int dot(Φ)_v dv = int dot(Φ)_E dE`. This quantity has also been termed particle flux density. Because the word "density" has several connotations, the term "fluence rate" is preferred. For a radiation field composed of particles of velocity `v`, the fluence rate is equal to `n`·`v` where `n` is the particle number density. See Remarks for item 10-43. In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence rate, `dot(Φ)` , is the quotient of `d Φ` and `dt`, where `d Φ` is the increment of the fluence in the time interval `dt`: `dot(Φ) = (dΦ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleFluenceRateUnit[1];
    }

    attribute particleFluenceRate: ParticleFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def ParticleFluenceRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-45 radiant energy */
    attribute radiantEnergyForIonizingRadiation: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-45 radiant energy
         * symbol(s): `R`
         * application domain: ionizing radiation
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: mean energy (ISO 80000-5), excluding rest energy (item 10-3), of the particles that are emitted, transferred, or received
         * remarks: For particles of energy `E` (excluding rest energy), the radiant energy, `R`, is equal to the product `N·E` where `N` is the number of the particles that are emitted, transferred, or received The distributions, `N_E` and `R_E`, of the particle number and the radiant energy with respect to energy are given by `N_E` = `dN`/d`E` and `R_E` = `dR`/d`E`, respectively, where `dN` is the number of particles with energy between `E` and `E`+d`E`, and `dR` is their radiant energy. The two distributions are related by `R_E` = `E`·`N_E`.
         */
    }

    /* ISO-80000-10 item 10-46 energy fluence */
    attribute def EnergyFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-46 energy fluence
         * symbol(s): `Ψ`
         * application domain: generic
         * name: EnergyFluence
         * quantity dimension: M^1*T^-2
         * measurement unit(s): eV/m^2, J/m^2, kg*s^-2
         * tensor order: 0
         * definition: differential quotient of radiant energy `R` (item 10-45) incident on a sphere of cross-sectional area (item 10-38.1) `a` with respect to that area: `Ψ = (dR)/(da)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy fluence, `Ψ` is the quotient of `dR` and `da`, where `dR` is the radiant energy incident on a sphere of cross-sectional area `da`: `Ψ = (dR)/(da)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceUnit[1];
    }

    attribute energyFluence: EnergyFluenceValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-47 energy fluence rate */
    attribute def EnergyFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-47 energy fluence rate
         * symbol(s): `dot(Ψ)`
         * application domain: generic
         * name: EnergyFluenceRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: differential quotient of the energy fluence `Ψ` (item 10-46) with respect to time (ISO 80000-3): `dot(Ψ) = (d Ψ)/(dt)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy-fluence rate, `dot(Ψ)` , is the quotient of `d Ψ` by `dt`, where `d Ψ` is the increment of the energy fluence in the time interval `dt`: `dot(Ψ) = (d Ψ)/(dt)`. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyFluenceRateUnit[1];
    }

    attribute energyFluenceRate: EnergyFluenceRateValue[*] nonunique :> scalarQuantities;

    attribute def EnergyFluenceRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-48 particle current density */
    attribute def ParticleCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-48 particle current density (magnitude)
         * symbol(s): `J`, `S`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleCurrentDensityUnit[1];
    }

    attribute particleCurrentDensity: ParticleCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    attribute def CartesianParticleCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-48 particle current density (vector)
         * symbol(s): `vec(J)`, `vec(S)`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 1
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianParticleCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianParticleCurrentDensity3dVector: CartesianParticleCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianParticleCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ParticleCurrentDensityUnit[3];
    }

    /* ISO-80000-10 item 10-49 linear attenuation coefficient */
    attribute def LinearAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-49 linear attenuation coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: ionizing radiation
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: for uncharged particles of a given type and energy the differential quotient `n` with respect to `l,` where `n` is the fraction of `N` incoming particles that experience interactions in traversing a distance (ISO 80000-3) `l` in a given material: `μ = (dn)/(dl) = 1/N (dN)/(dl)` where `dN` is the number of particles that experience interactions in traversing `dl`
         * remarks: `μ` is equal to the macroscopic total cross section `Σ_"tot"` for the removal of particles from the beam. Using the relation `μ_m = μ/ρ` between the linear attenuation coefficient `μ`, the mass attenuation coefficient `μ_m` (item 10-50) and the density `ρ`, the definition given for the mass attenuation coefficient in report 85a of the ICRU can be applied to the linear attenuation coefficient resulting in: The linear attenuation coefficient, `μ`, of a material, for uncharged particles of a given type and energy, is the quotient of `(dN)/N` by `dl`, where `(dN)/N` is the mean fraction of the particles that experience interactions in traversing a distance `dl` in the material: `μ = 1/(dl) (dN)/(N)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute linearAttenuationCoefficientForIonizingRadiation: LinearAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-50 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-50 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: ionizing radiation
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the mass density `ρ` (ISO 80000-4) of the medium: `μ_m = μ/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientForIonizingRadiationUnit[1];
    }

    attribute massAttenuationCoefficientForIonizingRadiation: MassAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-51 molar attenuation coefficient */
    attribute def MolarAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-51 molar attenuation coefficient
         * symbol(s): `μ_c`
         * application domain: generic
         * name: MolarAttenuationCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: quotient of linear attenuation coefficient `µ` (item 10-49) and the amount c (ISO 80000-9) of the medium: `μ_c = μ/c`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAttenuationCoefficientUnit[1];
    }

    attribute molarAttenuationCoefficient: MolarAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-10 item 10-52 atomic attenuation coefficient */
    attribute def AtomicAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-52 atomic attenuation coefficient
         * symbol(s): `μ_a`
         * application domain: generic
         * name: AtomicAttenuationCoefficient
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the number density (item 10-62.1), `n`, of atoms in the substance: `μ_a = μ/n`
         * remarks: `μ` is equal to the total cross section `σ_"tot"` for the removal of particles from the beam. See also item 10-38.2.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AtomicAttenuationCoefficientUnit[1];
    }

    attribute atomicAttenuationCoefficient: AtomicAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def AtomicAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-53 half-value thickness */
    attribute halfValueThickness: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-53 half-value thickness
         * symbol(s): `d_(1//2)`
         * application domain: generic
         * name: HalfValueThickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: thickness (ISO 80000-3) of the attenuating layer that reduces the quantity of interest of a unidirectional beam of infinitesimal width to half of its initial value
         * remarks: For exponential attenuation, `d_(1/2) = ln(2)/μ`. The quantity of interest is often the air kerma or exposure.
         */
    }

    /* ISO-80000-10 item 10-54 total linear stopping power, linear stopping power */
    attribute def TotalLinearStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-54 total linear stopping power, linear stopping power
         * symbol(s): `S`, `S_l`
         * application domain: generic
         * name: TotalLinearStoppingPower
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: for charged particles of a given type and energy `E_0` the differential quotient of `E` with respect to `x,` where `E` is the mean energy (ISO 80000-4) lost by the charged particles in traversing a distance (ISO 80000-3) `x` in the given material: `S = -(dE)/(dx)`
         * remarks: The total linear stopping power is sometimes also called stopping power. Both electronic losses and radiative losses are included. The quotient of the total linear stopping power of a substance and that of a reference substance is called the relative linear stopping power. See also item 10-85. Using the relation `S_m = S/ρ` between the total mass stopping power `S_m` (item 10-55), the total linear stopping power `S`, and the density `ρ`, the definition given for the mass stopping in report 85a of the ICRU can be applied to that of the total linear stopping power resulting in: The linear stopping power, `S`, of a material, for charged particles of a given type and energy, is the quotient of `dE` by `dl`, where `dE` is the mean energy lost by the charged particles in traversing a distance `dl` in the material: `S = -(dE)/(dx)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalLinearStoppingPowerUnit[1];
    }

    attribute totalLinearStoppingPower: TotalLinearStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalLinearStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias LinearStoppingPowerUnit for TotalLinearStoppingPowerUnit;
    alias LinearStoppingPowerValue for TotalLinearStoppingPowerValue;
    alias linearStoppingPower for totalLinearStoppingPower;

    /* ISO-80000-10 item 10-55 total mass stopping power, mass stopping power */
    attribute def TotalMassStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-55 total mass stopping power, mass stopping power
         * symbol(s): `S_m`
         * application domain: generic
         * name: TotalMassStoppingPower
         * quantity dimension: L^4*T^-2
         * measurement unit(s): eV*m^-2/kg, J*m^2/kg, m^4*s^-2
         * tensor order: 0
         * definition: quotient of the total linear stopping power `S` (item 10-54) and the mass density `ρ` (ISO 80000-4) of the material: `S_m = S/ρ`
         * remarks: The quotient of total mass stopping power of a material and that of a reference material is called relative mass stopping power.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalMassStoppingPowerUnit[1];
    }

    attribute totalMassStoppingPower: TotalMassStoppingPowerValue[*] nonunique :> scalarQuantities;

    attribute def TotalMassStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias MassStoppingPowerUnit for TotalMassStoppingPowerUnit;
    alias MassStoppingPowerValue for TotalMassStoppingPowerValue;
    alias massStoppingPower for totalMassStoppingPower;

    /* ISO-80000-10 item 10-56 mean linear range */
    attribute meanLinearRange: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-56 mean linear range
         * symbol(s): `R`, `R_l`
         * application domain: generic
         * name: MeanLinearRange (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: mean total rectified path length (ISO 80000-3) travelled by a particle in the course of slowing down to rest in a given material averaged over a group of particles having the same initial energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-57 mean mass range */
    attribute def MeanMassRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-57 mean mass range
         * symbol(s): `R_ρ`, `R_m`
         * application domain: generic
         * name: MeanMassRange
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: product of the mean linear range (item 10-56) `R` and the mass density `ρ` (ISO 80000-4) of the material: `R_ρ = R*ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MeanMassRangeUnit[1];
    }

    attribute meanMassRange: MeanMassRangeValue[*] nonunique :> scalarQuantities;

    attribute def MeanMassRangeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-58 linear ionization */
    attribute def LinearIonizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-58 linear ionization
         * symbol(s): `N_{i_l}`
         * application domain: generic
         * name: LinearIonization
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: differential quotient of `q` with respect to `l`, where `q` is the average total charge (IEC 80000-6) of all positive ions produced by an ionizing charged particle over a path `l` (ISO 80000-3), divided by the elementary charge, `e` (ISO 80000-1): `N_{i_l} = 1/e*(dq)/(dl)`
         * remarks: Ionization due to secondary ionizing particles is included.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearIonizationUnit[1];
    }

    attribute linearIonization: LinearIonizationValue[*] nonunique :> scalarQuantities;

    attribute def LinearIonizationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-59 total ionization */
    attribute def TotalIonizationValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-59 total ionization
         * symbol(s): `N_i`
         * application domain: generic
         * name: TotalIonization (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total mean charge of all positive ions produced by an ionizing charged particle along its entire path and along the paths of any secondary charged particles, and the elementary charge, `e` (ISO 80000-1)
         * remarks: `N_i = int N_(il) dl` See item 10-58.
         */
    }
    attribute totalIonization: TotalIonizationValue :> scalarQuantities;

    /* ISO-80000-10 item 10-60 average energy loss per elementary charge produced */
    attribute def AverageEnergyLossPerElementaryChargeProducedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-60 average energy loss per elementary charge produced
         * symbol(s): `W_i`
         * application domain: generic
         * name: AverageEnergyLossPerElementaryChargeProduced
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the initial kinetic energy `E_k` (ISO 80000-4) of an ionizing charged particle and the total ionization `N_i` (item 10-59) produced by that particle: `W_i = E_k/N_i`
         * remarks: The name "average energy loss per ion pair formed" is usually used, although it is ambiguous. In the practical dosimetry of ionizing radiation the term `W`/`e`, the quotient of `W`, the average energy deposited in dry air per ion pair formed, and `e`, the elementary charge, is used as the factor which, when multiplied with the electric charge of one sign carried by all ion pairs formed in dry air of given mass, gives the energy deposited in this amount of dry air in the form of excitations and ionizations. In ICRU Report 85a, the mean energy expended in a gas per ion pair formed, `W`, is the quotient of `E` by `N,` where `N` is the mean total liberated charge of either sign, divided by the elementary charge when the initial kinetic energy `E` of a charged particle introduced into the gas is completely dissipated in the gas. Thus, `W` = `E`/`N`. It follows from the definition of `W` that the ions produced by bremsstrahlung or other secondary radiation emitted by the initial and secondary charged particles are included in `N`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AverageEnergyLossPerElementaryChargeProducedUnit[1];
    }

    attribute averageEnergyLossPerElementaryChargeProduced: AverageEnergyLossPerElementaryChargeProducedValue[*] nonunique :> scalarQuantities;

    attribute def AverageEnergyLossPerElementaryChargeProducedUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-61 mobility */
    attribute def MobilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-61 mobility
         * symbol(s): `μ`, `μ_m`
         * application domain: generic
         * name: Mobility
         * quantity dimension: M^-1*T^2*I^1
         * measurement unit(s): m^2/(V*s), kg^-1*s^2*A
         * tensor order: 0
         * definition: quotient of average drift speed (ISO 80000-3) imparted to a charged particle in a medium by an electric field, and the electric field strength (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MobilityUnit[1];
    }

    attribute mobility: MobilityValue[*] nonunique :> scalarQuantities;

    attribute def MobilityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-62.1 particle number density */
    attribute def ParticleNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.1 particle number density
         * symbol(s): `n`
         * application domain: generic
         * name: ParticleNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the mean number `N` of particles in the volume (ISO 80000-3) `V` and volume: `n = N/V`
         * remarks: `n` is the general symbol for the number density of particles. The distribution functions expressed in terms of speed and energy, `n_v` and `n_E`, are related to `n` by: `n = int n_v dv = int n_E dE`. The word "particle" is usually replaced by the name of a specific particle, for example `neutron` number density.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleNumberDensityUnit[1];
    }

    attribute particleNumberDensity: ParticleNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-10 item 10-62.2 ion number density, ion density */
    attribute def IonNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.2 ion number density, ion density
         * symbol(s): `n^"+"`, `n^"-"`
         * application domain: generic
         * name: IonNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the number of positive and negative ions, `N^"+"` and `N^"-"`, respectively, in the volume `V` (ISO 80000-3), and that volume: `n^"+" = N^"+" / V`, `n^"-" = N^"-" / V`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IonNumberDensityUnit[1];
    }

    attribute ionNumberDensity: IonNumberDensityValue[*] nonunique :> scalarQuantities;

    attribute def IonNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias IonDensityUnit for IonNumberDensityUnit;
    alias IonDensityValue for IonNumberDensityValue;
    alias ionDensity for ionNumberDensity;

    /* ISO-80000-10 item 10-63 Recombination coefficient */
    attribute def RecombinationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-63 Recombination coefficient
         * symbol(s): `α`
         * application domain: generic
         * name: RecombinationCoefficient
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: coefficient in the law of recombination: `-(dn^"+")/(dt) = -(dn^"-")/(dt) = α*n^"+"*n^"-"`, where `n^"+"` and `n^"-"` are the ion number densities (item 10-62.2) of positive and negative ions, respectively, recombined during a time interval of duration `dt` (ISO 80000-3)
         * remarks: The widely used term "recombination factor" is not correct because "factor" should only be used for quantities with dimension 1. The terms `(dn^"+")/(dt)` , `(dn^"-")/(dt)` are differential quotients.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RecombinationCoefficientUnit[1];
    }

    attribute recombinationCoefficient: RecombinationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def RecombinationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-64 diffusion coefficient, diffusion coefficient for particle number density */
    /* Refer to declaration for DiffusionCoefficient in ISQChemistryMolecular item 9-39 diffusion coefficient */

    alias DiffusionCoefficientForParticleNumberDensityUnit for DiffusionCoefficientUnit;
    alias DiffusionCoefficientForParticleNumberDensityValue for DiffusionCoefficientValue;
    alias diffusionCoefficientForParticleNumberDensity for diffusionCoefficient;

    /* ISO-80000-10 item 10-65 diffusion coefficient for fluence rate */
    attribute diffusionCoefficientForFluenceRate: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-65 diffusion coefficient for fluence rate
         * symbol(s): `D_ϕ`, `D`
         * application domain: generic
         * name: DiffusionCoefficientForFluenceRate (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: proportionality constant between the particle current density `vec(J )`(item 10-48) and the gradient of the particle fluence rate `dot(Φ)` (item 10-44): `vec(J) = -vec(D) * nabla Φ`
         * remarks: For a particle of a given speed `v`: `D_Ψ(v) = -J_{v,x}/(partial Ψ // partial x)` and `vec(v) * vec(D_Ψ)(v) = -vec(D_n)(v)`
         */
    }

    /* ISO-80000-10 item 10-66 particle source density */
    attribute def ParticleSourceDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-66 particle source density
         * symbol(s): `S`
         * application domain: generic
         * name: ParticleSourceDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the mean rate of production of particles in a volume, and that volume (ISO 80000-3)
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` source density. The distribution functions expressed in terms of speed and energy, `S_v` and `S_E`, are related to `S` by: `S = int S_v dv = int S_E dE`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleSourceDensityUnit[1];
    }

    attribute particleSourceDensity: ParticleSourceDensityValue[*] nonunique :> scalarQuantities;

    attribute def ParticleSourceDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-67 slowing-down density */
    attribute def SlowingDownDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-67 slowing-down density
         * symbol(s): `q`
         * application domain: generic
         * name: SlowingDownDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: differential quotient of `n` with respect to time, where `n` is the number density of particles that are slowed down in a time interval of duration (ISO 80000-3) `t`: `q = -(dn)/(dt)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SlowingDownDensityUnit[1];
    }

    attribute slowingDownDensity: SlowingDownDensityValue[*] nonunique :> scalarQuantities;

    attribute def SlowingDownDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-68 resonance escape probability */
    attribute def ResonanceEscapeProbabilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-68 resonance escape probability
         * symbol(s): `p`
         * application domain: generic
         * name: ResonanceEscapeProbability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the probability that a neutron slowing down will traverse all or some specified portion of the range of resonance energies (item 10-37.2) without being absorbed
         * remarks: None.
         */
    }
    attribute resonanceEscapeProbability: ResonanceEscapeProbabilityValue :> scalarQuantities;

    /* ISO-80000-10 item 10-69 lethargy */
    attribute def LethargyValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-69 lethargy
         * symbol(s): `u`
         * application domain: generic
         * name: Lethargy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a neutron of kinetic energy `E` (ISO 80000-4) : `u = ln(E_0/E)`, where `E_0` is a reference energy
         * remarks: Lethargy is also referred to as logarithmic energy decrement.
         */
    }
    attribute lethargy: LethargyValue :> scalarQuantities;

    /* ISO-80000-10 item 10-70 average logarithmic energy decrement */
    attribute def AverageLogarithmicEnergyDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-70 average logarithmic energy decrement
         * symbol(s): `ζ`
         * application domain: generic
         * name: AverageLogarithmicEnergyDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average value of the increase in lethargy (item 10-69) in elastic collisions between neutrons and nuclei whose kinetic energy (ISO 80000-4) is negligible compared with that of the neutrons
         * remarks: None.
         */
    }
    attribute averageLogarithmicEnergyDecrement: AverageLogarithmicEnergyDecrementValue :> scalarQuantities;

    /* ISO-80000-10 item 10-71 mean free path */
    attribute meanFreePathForAtomicPhysics: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-71 mean free path
         * symbol(s): `l`, `λ`
         * application domain: atomic physics
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that particles travel between two successive specified reactions or processes
         * remarks: See the Remarks for item 10-42.1.
         */
    }

    /* ISO-80000-10 item 10-72.1 slowing-down area */
    attribute slowingDownArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.1 slowing-down area
         * symbol(s): `L_s^2`, `L_"sl"^2`
         * application domain: generic
         * name: SlowingDownArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square of the distance (ISO 80000-3) between the neutron source and the point where a neutron reaches a given energy (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-72.2 diffusion area */
    attribute diffusionArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.2 diffusion area
         * symbol(s): `L^2`
         * application domain: generic
         * name: DiffusionArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square distance (ISO 80000-3) between the point where a neutron enters a specified class and the point where it leaves this class
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
    }

    /* ISO-80000-10 item 10-72.3 migration area */
    attribute migrationArea: AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.3 migration area
         * symbol(s): `M^2`
         * application domain: generic
         * name: MigrationArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: sum of the slowing-down area (item 10-72.1) from fission energy to thermal energy (ISO 80000-5) and the diffusion area (item 10-72.2) for thermal neutrons
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.1 slowing-down length */
    attribute slowingDownLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.1 slowing-down length
         * symbol(s): `L_s`, `L_"sl"`
         * application domain: generic
         * name: SlowingDownLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the slowing down area `L_s^2` (item 10-72.1): `L_s = sqrt(L_s^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.2 diffusion length */
    attribute diffusionLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.2 diffusion length
         * symbol(s): `L`
         * application domain: atomic physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the diffusion area `L^2` (item 10-72.2): `L = sqrt(L^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-73.3 migration length */
    attribute migrationLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.3 migration length
         * symbol(s): `M`
         * application domain: generic
         * name: MigrationLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the migration area `M^2` (item 10-72.3): `M = sqrt(M^2)`
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.1 neutron yield per fission */
    attribute neutronYieldPerFission: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.1 neutron yield per fission
         * symbol(s): `ν`
         * application domain: generic
         * name: NeutronYieldPerFission (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per fission event
         * remarks: None.
         */
    }

    /* ISO-80000-10 item 10-74.2 neutron yield per absorption */
    attribute neutronYieldPerAbsorption: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.2 neutron yield per absorption
         * symbol(s): `η`
         * application domain: generic
         * name: NeutronYieldPerAbsorption (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per neutron absorbed in a fissionable nuclide or in a nuclear fuel, as specified
         * remarks: `ν/η` is equal to the quotient of the macroscopic cross section for fission and that for absorption, both for neutrons in the fuel material.
         */
    }

    /* ISO-80000-10 item 10-75 fast fission factor */
    attribute def FastFissionFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-75 fast fission factor
         * symbol(s): `φ`
         * application domain: generic
         * name: FastFissionFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the mean number of neutrons produced by fission due to neutrons of all energies (ISO 80000-5) and the mean number of neutrons produced by fissions due to thermal neutrons only
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FastFissionFactorUnit[1];
    }

    attribute fastFissionFactor: FastFissionFactorValue[*] nonunique :> scalarQuantities;

    attribute def FastFissionFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-76 thermal utilization factor */
    attribute def ThermalUtilizationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-76 thermal utilization factor
         * symbol(s): `f`
         * application domain: generic
         * name: ThermalUtilizationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the number of thermal neutrons absorbed in a fissionable nuclide or in a nuclear fuel, as specified, and the total number of thermal neutrons absorbed
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalUtilizationFactorUnit[1];
    }

    attribute thermalUtilizationFactor: ThermalUtilizationFactorValue[*] nonunique :> scalarQuantities;

    attribute def ThermalUtilizationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-77 non-leakage probability */
    attribute def NonLeakageProbabilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-77 non-leakage probability
         * symbol(s): `Λ`
         * application domain: generic
         * name: NonLeakageProbability
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: probability that a neutron will not escape from the reactor during the slowing-down process or while it diffuses as a thermal neutron
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: NonLeakageProbabilityUnit[1];
    }

    attribute nonLeakageProbability: NonLeakageProbabilityValue[*] nonunique :> scalarQuantities;

    attribute def NonLeakageProbabilityUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.1 multiplication factor */
    attribute def MultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.1 multiplication factor
         * symbol(s): `k`
         * application domain: generic
         * name: MultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total number of fission or fission-dependent neutrons produced in the duration of a time interval and the total number of neutrons lost by absorption and leakage in that duration
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MultiplicationFactorUnit[1];
    }

    attribute multiplicationFactor: MultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def MultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-78.2 infinite multiplication factor */
    attribute def InfiniteMultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.2 infinite multiplication factor
         * symbol(s): `k_∞`
         * application domain: generic
         * name: InfiniteMultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: multiplication factor (item 10-78.1) for an infinite medium or for an infinite repeating lattice
         * remarks: For a thermal reactor, `k_∞ = η*ε*p*f`
         */
        attribute :>> num: Real;
        attribute :>> mRef: InfiniteMultiplicationFactorUnit[1];
    }

    attribute infiniteMultiplicationFactor: InfiniteMultiplicationFactorValue[*] nonunique :> scalarQuantities;

    attribute def InfiniteMultiplicationFactorUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-79 reactor time constant */
    attribute reactorTimeConstant: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-79 reactor time constant
         * symbol(s): `T`
         * application domain: generic
         * name: ReactorTimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (ISO 80000-3) required for the neutron fluence rate (item 10-44) in a reactor to change by the factor e when the fluence rate is rising or falling exponentially
         * remarks: Also called reactor period.
         */
    }

    /* ISO-80000-10 item 10-80.1 energy imparted */
    attribute energyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.1 energy imparted
         * symbol(s): `ε`
         * application domain: generic
         * name: EnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of all energy deposits in a given volume: `ε = sum_i ε_i` where the summation is performed over all energy (ISO 80000-5) deposits `ε_i` of interaction `i` in that volume
         * remarks: Energy imparted is a stochastic quantity. `ε_i` is given by: `ε_i = ε_(i n) - ε_"out" + Q` where `ε_(i n)` is the energy (ISO 80000-5) of the incident ionizing particle, excluding rest energy (item 10-3), `ε_"out"` is the sum of the energies (ISO 80000-5) of all ionizing particles leaving the interaction, excluding rest energy (item 10-3), and `Q` is the change in the rest energies (item 10-3) of the nucleus and of all particles involved in the interaction. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy. Stochastic quantities such as the energy imparted and the specific energy imparted (item 10-81.2) and their probability distributions have been introduced as they describe the discontinuous nature of the ionizing radiations as a determinant of radiochemical and radiobiological effects. In radiation applications involving large numbers of ionizing particles, e.g. in medicine, radiation protection and materials testing and processing, these fluctuations are adequately represented by the expectation values of the probability distributions. Non-stochastic quantities such as particle fluence (item 10-43), absorbed dose (item 10-81.1) and kerma (item 10-86.1) are based on these expectation values.
         */
    }

    /* ISO-80000-10 item 10-80.2 mean energy imparted */
    attribute meanEnergyImparted: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.2 mean energy imparted
         * symbol(s): `bar(ε)`
         * application domain: generic
         * name: MeanEnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: expectation value of the energy imparted (item 10-80.1): `bar(ε) = R_"in" - R_"out" + sum Q` where `R_"in"` is the radiant energy (item 10-45) of all those charged and uncharged ionizing particles that enter the volume, `R_"out"` is the radiant energy of all those charged and uncharged ionizing particles that leave the volume, and `sum Q` is the sum of all changes of the rest energy (item 10-3) of nuclei and elementary particles that occur in that volume
         * remarks: Sometimes, it has been called the integral absorbed dose. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy.
         */
    }

    /* ISO-80000-10 item 10-81.1 absorbed dose */
    attribute def AbsorbedDoseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-81.1 absorbed dose
         * symbol(s): `D`
         * application domain: generic
         * name: AbsorbedDose
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: differential quotient of `bar(ε)` with respect to `m`, where `bar(ε)` is the mean energy (ISO 80000-5) imparted by ionizing radiation to matter of mass (ISO 80000-4) `m`: `D = (d bar(ε))/(dm)`
         * remarks: The gray is a special name for joule per kilogram, to be used as the coherent SI unit for absorbed dose. `1 "Gy" = 1 "J"/"kg"`. `bar(ε) = int D dm` where `dm` is the element of mass of the irradiated matter. In the limit of a small domain, the mean specific energy `bar(z) = (Δ bar(ε))/(Δ m)` is equal to the absorbed dose `D`. The absorbed dose can also be expressed in terms of the volume of the mass element by: `D = (d bar(ε))/(dm) = (d bar(ε))/(ρ dV)` where `ρ` is the mass density of the mass element. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed dose, `D`, is the quotient of `d bar(ε)` by dm, where `d bar(ε)` is the mean energy imparted by ionizing radiation to matter of mass `dm`: `D = (d bar(ε))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseUnit[1];
    }

    attribute absorbedDose: AbsorbedDoseValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-81.2 specific energy imparted */
    attribute specificEnergyImparted: AbsorbedDoseValue :> scalarQuantities {
        doc
        /*
         * source: item 10-81.2 specific energy imparted
         * symbol(s): `z`
         * application domain: generic
         * name: SpecificEnergyImparted (specializes AbsorbedDose)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of the energy imparted `ε` (item 10-80.1) and the mass `m` (ISO 80000-4) of the matter in a given volume element: `z = ε / m`
         * remarks: `z` is a stochastic quantity. In the limit of a small domain, the mean specific energy `bar(z)` is equal to the absorbed dose `D`. The specific energy imparted can be due to one or more (energy-deposition) events.
         */
    }

    /* ISO-80000-10 item 10-82 quality factor */
    attribute def QualityFactorForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-82 quality factor
         * symbol(s): `Q`
         * application domain: ionizing radiation
         * name: QualityFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor in the calculation and measurement of dose equivalent (item 10-83.1), by which the absorbed dose (item 10-81.1) is to be weighted in order to account for different biological effectiveness of radiations, for radiation protection purposes
         * remarks: `Q` is determined by the linear energy transfer (item 10-85) for `Δ -> ∞` , `L_∞` (often denoted as `L` or LET), of charged particles passing through a small volume element at this point (the value of `L_∞` refers to water, not to tissue; the difference, however, is small). The relationship between `L` and `Q` is given in ICRP Publication 103 (ICRP, 2007).
         */
        attribute :>> num: Real;
        attribute :>> mRef: QualityFactorForIonizingRadiationUnit[1];
    }

    attribute qualityFactorForIonizingRadiation: QualityFactorForIonizingRadiationValue[*] nonunique :> scalarQuantities;

    attribute def QualityFactorForIonizingRadiationUnit :> DimensionOneUnit {
    }

    /* ISO-80000-10 item 10-83.1 dose equivalent */
    attribute def DoseEquivalentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-83.1 dose equivalent
         * symbol(s): `H`
         * application domain: generic
         * name: DoseEquivalent
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Sv, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: product of the absorbed dose `D` (item 10-81.1) to tissue at the point of interest and the quality factor `Q` (item 10-82) at that point: `H = DQ`
         * remarks: The sievert (Sv) is a special name for joule per kilogram, and is the coherent SI unit for dose equivalent. `1 "Sv" = 1 "J/kg"`. The dose equivalent at a point in tissue is given by: `H = int_0^∞ Q(L) D_L dL` where `D_L = (dD)/(dL)` is the distribution of `D` in `L` at the point of interest. See ICRP Publication 103 (ICRP, 2007). The quantities measured with radiation protection dosimeters are based on the definition `H = Q*D`. If various radiation qualities `i` have to be simultaneously accounted for, the definition is: `H = sum_i Q_i*D_i`. In ICRU 51 this quantity is denoted as "dose equivalent". In order to quantify the radiation exposition of the human body and to specify dose limits, use is made of a quantity defined in ICRP 103, the "equivalent dose to a tissue or organ": `H_T = w_T*sum_R w_R*D_{T,R}`. The weighting factors `w_T` for various tissues and organs `T` and `w_R` for various radiation qualities `R` have been numerically laid down in ICRP 103. `D_{T,R}` is the mean absorbed dose to tissue within a tissue or organ `T`, imparted by radiation with radiation quality `R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DoseEquivalentUnit[1];
    }

    attribute doseEquivalent: DoseEquivalentValue[*] nonunique :> scalarQuantities;

    attribute def DoseEquivalentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-83.2 dose equivalent rate */
    attribute doseEquivalentRate: DoseEquivalentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-83.2 dose equivalent rate
         * symbol(s): `dot(H)`
         * application domain: generic
         * name: DoseEquivalentRate (specializes DoseEquivalent)
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Sv/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of dose equivalent `H` (item 10-83.1) with respect to time (ISO 80000-3): `dot(H) = (dH)/(dt)`
         * remarks: `1 "Sv/s" = 1 "W/kg"`. See the remarks for item 10-83.1.
         */
    }

    /* ISO-80000-10 item 10-84 absorbed-dose rate */
    attribute def AbsorbedDoseRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-84 absorbed-dose rate
         * symbol(s): `dot(D)`
         * application domain: generic
         * name: AbsorbedDoseRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of the absorbed dose `D` (item 10-81.1) with respect to time (ISO 80000-3): `dot(D) = (dD)/(dt)`
         * remarks: `1 "Gy/s"  = 1 "W/kg"` See the remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed-does rate, `dot(D)` , is the quotient of `dD` by `dt`, where `dD` is the increment of absorbed does in the time interval `dt`: `dot(D) = (dD)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AbsorbedDoseRateUnit[1];
    }

    attribute absorbedDoseRate: AbsorbedDoseRateValue[*] nonunique :> scalarQuantities;

    attribute def AbsorbedDoseRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-85 linear energy transfer */
    attribute def LinearEnergyTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-85 linear energy transfer
         * symbol(s): `L_Δ`
         * application domain: generic
         * name: LinearEnergyTransfer
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: quotient of the mean energy (ISO 80000-4) `dE_Δ` lost by the charged particles due to electronic interactions in traversing a distance (ISO 80000-3) `dl`, minus the mean sum of the kinetic energies in excess of `Δ` of all the electrons released by the charged particles and `dl`: `L_Δ = (dE_Δ)/(dl)`
         * remarks: This quantity is not completely defined unless `Δ` is specified, i.e. the maximum kinetic energy of secondary electrons whose energy is considered to be "locally deposited". `Δ` may be expressed in `"eV"`. Note that the abbreviation LET specifically refers to the quantity `L_∞` mentioned in the remark to 10-82.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearEnergyTransferUnit[1];
    }

    attribute linearEnergyTransfer: LinearEnergyTransferValue[*] nonunique :> scalarQuantities;

    attribute def LinearEnergyTransferUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.1 kerma */
    attribute def KermaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.1 kerma
         * symbol(s): `K`
         * application domain: generic
         * name: Kerma
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: for uncharged ionizing radiation, differential quotient of `E_(`tr) with respect to `m`, where `E_(`tr) is the mean sum of the initial kinetic energies (ISO 80000-4) of all the charged ionizing particles liberated in a mass (ISO 80000-4) `m` of a material: `K = (dE_tr)/(dm)`
         * remarks: `1 "Gy" = 1 "J/kg"` See the remarks for item 10-81.1. The name "kerma" is derived from Kinetic Energy Released in MAtter (or MAss or MAterial). The quantity `dE_(tr)` includes also the kinetic energy of the charged particles emitted in the decay of excited atoms, molecules, or nuclei. When the mass element `dm` consists of air the term air kerma is used. It can be convenient to refer to a value of air kerma in free space or at a point inside a material different from air, e.g. to the air kerma at a point inside a water phantom. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma, `K`, for ionizing uncharged particles, is the quotient of `dE_(tr)` by `dm`, where `dE_(tr)` is the mean sum of the initial kinetic energies of all the charged particles liberated in a mass `dm` of a material by the uncharged particles incident on `dm`: `K = (dE_(tr))/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaUnit[1];
    }

    attribute kerma: KermaValue[*] nonunique :> scalarQuantities;

    attribute def KermaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-86.2 kerma rate */
    attribute def KermaRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.2 kerma rate
         * symbol(s): `dot(K)`
         * application domain: generic
         * name: KermaRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of kerma (item 10-86.1) with respect to time (ISO 80000-3): `dot(K) = (dK)/(dt)`
         * remarks: `1 "Gy/s" = 1 "W/kg"`. See the Remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma rate, `dot(K)` , is the quotient of `dK` by `dt`, where `dK` is the increment of kerma in the time interval `dt`: `dot(K) = (dK)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KermaRateUnit[1];
    }

    attribute kermaRate: KermaRateValue[*] nonunique :> scalarQuantities;

    attribute def KermaRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-10 item 10-87 mass energy-transfer coefficient */
    attribute def MassEnergyTransferCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-87 mass energy-transfer coefficient
         * symbol(s): `μ_"tr"/ρ`
         * application domain: generic
         * name: MassEnergyTransferCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: for ionizing uncharged particles of a given type and energy, the differential quotient of `R_"tr"` with respect to `l`: `m_"tr"/ρ = 1/ρ 1/R (dR_"tr")/(dl)` where `R_"tr"` is the mean energy (ISO 80000-5) that is transferred to kinetic energy (ISO 80000-4) of charged particles by interactions of the uncharged particles of incident radiant energy `R` (item 10-45) in traversing a distance (ISO 80000-3) `l` in the material of density (ISO 80000-4) `ρ`, divided by `ρ` and `R`
         * remarks: `m_(tr)/ρ = (dot(K))/ψ` , where `dot(K)` is kerma rate (item 10-86.2) and `ψ` is energy fluence rate (item 10-47). The quantity: `μ_(en)/ρ = μ_(tr)/ρ(1-g)` where `g` is mean fraction of the kinetic energy of the liberated charged particles that is lost in radiative processes in the material, is called mass energy-absorption coefficient. The mass energy-absorption coefficient of a compound material depends on the stopping power of the material. Thus, its evaluation cannot, in principle, be reduced to a simple summation of the mass energy-absorption coefficient of the atomic constituents. Such a summation can provide an adequate approximation when the value of `g` is sufficiently small. In report 85a of the ICRU a definition with an equivalent meaning is given as: The mass energy-transfer coefficient, `μ_(tr)/ρ` , of a material, for uncharged particles of a given type and energy, is the quotient of `(dR_(tr))/R` by `ρ dl`, where `dR_(tr)` is the mean energy that is transferred to kinetic energy of charged particles by interactions of the uncharged particles of incident radiant energy `R` in traversing a distance `dl` in the material of density `ρ` : `μ_(tr)/ρ = 1/(ρ dl) (d R_(tr))/R`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassEnergyTransferCoefficientUnit[1];
    }

    attribute massEnergyTransferCoefficient: MassEnergyTransferCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassEnergyTransferCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-10 item 10-88 exposure */
    attribute def ExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-88 exposure
         * symbol(s): `X`
         * application domain: ionizing radiation
         * name: Exposure
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): C/kg, kg^-1*s*A
         * tensor order: 0
         * definition: for X- or gamma radiation the differential quotient of `q` with respect to `m`, where `q` is the absolute value of the mean total electric charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on an element of dry air with mass `m` (ISO 80000-4) are completely stopped in dry air: `X = (dq)/(dm)`
         * remarks: The ionization produced by electrons emitted in atomic or molecular relaxation is included in `dq`. The ionization due to photons emitted by radiative processes (i.e. bremsstrahlung and fluorescence photons) is not included in `dq`. This quantity should not be confused with the quantity photon exposure (ISO 80000-7), radiation exposure (ISO 80000-7), or the quantity luminous exposure (ISO 80000-7). It can be convenient to refer to a value of exposure in free space or at a point inside a material different from air, e.g. to the exposure at a point inside a water phantom. The exposure is related to the air kerma, `K_a`, (see item 10-86.1) by: `X = (e (1-g))/W K_a` , where `e` is the elementary charge (ISO 80000-1), `W` the average energy loss per elementary charge produced (item 10-60), and `g` is the fraction of the kinetic energy of liberated charged particles that is lost in radiative processes. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure, `X`, is the quotient of `dq` by `dm`, where `dq` is the absolute value of the mean total charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on a mass `dm` of dry air are completely stopped in dry air: `X = (dq)/(dm)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureUnit[1];
    }

    attribute exposure: ExposureValue[*] nonunique :> scalarQuantities;

    attribute def ExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-10 item 10-89 exposure rate */
    attribute def ExposureRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-89 exposure rate
         * symbol(s): `dot(X)`
         * application domain: generic
         * name: ExposureRate
         * quantity dimension: M^-1*I^1
         * measurement unit(s): C/(kg*s), kg^-1*A
         * tensor order: 0
         * definition: differential quotient of the exposure `X` (item 10-88) with respect to time (ISO 80000-3): `dot(X) = (dX)/(dt)`
         * remarks: `1 "C/(kg s)" = 1 "A/kg"`. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure rate, `dot(X)` , is the quotient of `dX` by `dt`, where `dX` is the increment of exposure in the time interval `dt`: `dot(X) = (dX)/(dt)`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ExposureRateUnit[1];
    }

    attribute exposureRate: ExposureRateValue[*] nonunique :> scalarQuantities;

    attribute def ExposureRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, electricCurrentPF); }
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_atomic_nuclear.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQAtomicNuclear {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-10:2019 "Atomic and nuclear physics"
     * see also https://www.iso.org/standard/64980.html
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
    private import ISQChemistryMolecular::DiffusionCoefficientUnit;
    private import ISQChemistryMolecular::DiffusionCoefficientValue;
    private import ISQChemistryMolecular::diffusionCoefficient;
    private import ISQElectromagnetism::ElectricChargeValue;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AreaValue;
    private import ISQThermodynamics::EnergyValue;
    /* ISO-80000-10 item 10-1.1 atomic number, proton number */
    attribute atomicNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.1 atomic number, proton number
         * symbol(s): `Z`
         * application domain: generic
         * name: AtomicNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of protons in an atomic nucleus
         * remarks: A nuclide is a species of atom with specified numbers of protons and neutrons. Nuclides with the same value of `Z` but different values of `N` are called isotopes of an element. The ordinal number of an element in the periodic table is equal to the atomic number. The atomic number equals the quotient of the charge (IEC 80000-6) of the nucleus and the elementary charge (ISO 80000-1).
         */
    }
    alias protonNumber for atomicNumber;
    /* ISO-80000-10 item 10-1.2 neutron number */
    attribute neutronNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.2 neutron number
         * symbol(s): `N`
         * application domain: generic
         * name: NeutronNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of neutrons in an atomic nucleus
         * remarks: Nuclides with the same value of `N` but different values of `Z` are called isotones. `N - Z` is called the neutron excess number.
         */
    }
    /* ISO-80000-10 item 10-1.3 nucleon number, mass number */
    attribute nucleonNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-1.3 nucleon number, mass number
         * symbol(s): `A`
         * application domain: generic
         * name: NucleonNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of nucleons in an atomic nucleus
         * remarks: `A` = `Z` + `N` Nuclides with the same value of `A` are called isobars.
         */
    }
    alias massNumber for nucleonNumber;
    /* ISO-80000-10 item 10-2 rest mass, proper mass */
    attribute restMass : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-2 rest mass, proper mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: RestMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: for particle X, mass (ISO 80000-4) of that particle at rest in an inertial frame
         * remarks: EXAMPLE `m(H_2O)` for a water molecule, `m_e` for an electron. Rest mass is often denoted `m_0`. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }
    alias properMass for restMass;
    /* ISO-80000-10 item 10-3 rest energy */
    attribute restEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-3 rest energy
         * symbol(s): `E_0`
         * application domain: generic
         * name: RestEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy `E_0` (ISO 80000-5) of a particle at rest: `E_0 = m_0 c_0^2` where `m_0` is the rest mass (item 10-2) of that particle, and `c_0` is speed of light in vacuum (ISO 80000-1)
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-4.1 atomic mass */
    attribute atomicMass : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.1 atomic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: AtomicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of an atom X in the ground state
         * remarks: `m(X)/m_u` is called the relative atomic mass. 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }
    /* ISO-80000-10 item 10-4.2 nuclidic mass */
    attribute nuclidicMass : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.2 nuclidic mass
         * symbol(s): `m(X)`, `m_X`
         * application domain: generic
         * name: NuclidicMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: rest mass (item 10-2) of a nuclide X in the ground state
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }
    /* ISO-80000-10 item 10-4.3 unified atomic mass constant */
    attribute unifiedAtomicMassConstant : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-4.3 unified atomic mass constant
         * symbol(s): `m_u`
         * application domain: generic
         * name: UnifiedAtomicMassConstant (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: 1/12 of the mass (ISO 80000-4) of an atom of the nuclide ^(12)C in the ground state at rest
         * remarks: 1 u is equal to 1/12 times the mass of a free carbon 12 atom, at rest and in its ground state. 1 Da = 1 u
         */
    }
    /* ISO-80000-10 item 10-5.1 elementary charge */
    attribute elementaryCharge : ElectricChargeValue :> scalarQuantities {
        doc
        /*
         * source: item 10-5.1 elementary charge
         * symbol(s): `e`
         * application domain: generic
         * name: ElementaryCharge (specializes ElectricCharge)
         * quantity dimension: T^1*I^1
         * measurement unit(s): C, s*A
         * tensor order: 0
         * definition: one of the fundamental constants in the SI system (ISO 80000-1), equal to the charge of the proton and opposite to the charge of the electron
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-5.2 charge number, ionization number */
    attribute def ChargeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-5.2 charge number, ionization number
         * symbol(s): `c`
         * application domain: generic
         * name: ChargeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a particle, quotient of the electric charge (IEC 80000-6) and the elementary charge (ISO 80000-1)
         * remarks: A particle is said to be electrically neutral if its charge number is equal to zero. The charge number of a particle can be positive, negative, or zero. The state of charge of a particle may be presented as a superscript to the symbol of that particle, e.g. `H^+, He^(++), Al^(3+), Cl^-, S^(--), N^(3-)`.
         */
    }
    attribute chargeNumber : ChargeNumberValue :> scalarQuantities;
    alias ionizationNumber for chargeNumber;
    /* ISO-80000-10 item 10-6 Bohr radius */
    attribute bohrRadius : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-6 Bohr radius
         * symbol(s): `a_0`
         * application domain: generic
         * name: BohrRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m, Å
         * tensor order: 0
         * definition: radius (ISO 80000-3) of the electron orbital in the hydrogen atom in its ground state in the Bohr model of the atom: `a_0 = (4 π ε_0 ℏ^2)/(m_e e^2)` where `ε_0` is the electric constant (IEC 80000-6), `ℏ` is the reduced Planck constant (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `e` is the elementary charge (ISO 80000-1)
         * remarks: The radius of the electron orbital in the H atom in its ground state is `a_0` in the Bohr model of the atom. ångström (Å), `1 Å := 10^-10 m`.
         */
    }
    /* ISO-80000-10 item 10-7 Rydberg constant */
    attribute def RydbergConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-7 Rydberg constant
         * symbol(s): `R_∞`
         * application domain: generic
         * name: RydbergConstant
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: spectroscopic constant that determines the wave numbers of the lines in the spectrum of hydrogen: `R_(oo) = e^2/(8 π ε_0 a_0 h c_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `a_0` is the Bohr radius (item 10-6), `h` is the Planck constant (ISO 80000-1), and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The quantity `R_y = R_∞ h c_0` is called the Rydberg energy.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RydbergConstantUnit[1];
    }
    attribute rydbergConstant : RydbergConstantValue[*] nonunique :> scalarQuantities;
    attribute def RydbergConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-8 Hartree energy */
    attribute def HartreeEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-8 Hartree energy
         * symbol(s): `E_H`, `E_h`
         * application domain: generic
         * name: HartreeEnergy
         * quantity dimension: L^6*M^3*T^-6
         * measurement unit(s): eV*J*kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) of the electron in a hydrogen atom in its ground state: `E_H = e^2/(4 π ε_0 a_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), and `a_0` is the Bohr radius (item 10-6)
         * remarks: The energy of the electron in an H atom in its ground state is `E_H`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HartreeEnergyUnit[1];
    }
    attribute hartreeEnergy : HartreeEnergyValue[*] nonunique :> scalarQuantities;
    attribute def HartreeEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 6;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -6;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-9.1 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (magnitude)
         * symbol(s): `μ`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticDipoleMomentUnit[1];
    }
    attribute magneticDipoleMoment : MagneticDipoleMomentValue[*] nonunique :> scalarQuantities;
    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }
    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-9.1 magnetic dipole moment (vector)
         * symbol(s): `vec(μ)`
         * application domain: atomic physics
         * name: MagneticDipoleMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 1
         * definition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`
         * remarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticDipoleMoment3dCoordinateFrame[1];
    }
    attribute cartesianMagneticDipoleMoment3dVector : CartesianMagneticDipoleMoment3dVector :> vectorQuantities;
    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticDipoleMomentUnit[3];
    }
    /* ISO-80000-10 item 10-9.2 Bohr magneton */
    attribute bohrMagneton : MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.2 Bohr magneton
         * symbol(s): `μ_B`
         * application domain: generic
         * name: BohrMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: magnitude of the magnetic moment of an electron in a state with orbital angular momentum quantum number `l`=1 (item 10-13.3) due to its orbital motion: `μ_B = (e ℏ)/(2 m_e)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_e` is the rest mass (item 10-2) of electron
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-9.3 nuclear magneton */
    attribute nuclearMagneton : MagneticDipoleMomentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-9.3 nuclear magneton
         * symbol(s): `μ_N`
         * application domain: generic
         * name: NuclearMagneton (specializes MagneticDipoleMoment)
         * quantity dimension: L^2*I^1
         * measurement unit(s): m^2*A
         * tensor order: 0
         * definition: absolute value of the magnetic moment of a nucleus: `μ_N = (e ℏ)/(2 m_p)` where `e` is the elementary charge (ISO 80000-1), `ℏ` is the reduced Planck constant (ISO 80000-1), and `m_p` is the rest mass (item 10-2) of proton
         * remarks: Subscript N stands for nucleus. For the neutron magnetic moment, subscript n is used. The magnetic moments of protons and neutrons differ from this quantity by their specific `g` factors (item 10-14.2).
         */
    }
    /* ISO-80000-10 item 10-10 spin */
    attribute def SpinValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-10 spin (magnitude)
         * symbol(s): `s`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpinUnit[1];
    }
    attribute spin : SpinValue[*] nonunique :> scalarQuantities;
    attribute def SpinUnit :> DerivedUnit {
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
    attribute def CartesianSpin3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-10 spin (vector)
         * symbol(s): `vec(s)`
         * application domain: generic
         * name: Spin
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system
         * remarks: Spin is an additive vector quantity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpin3dCoordinateFrame[1];
    }
    attribute cartesianSpin3dVector : CartesianSpin3dVector :> vectorQuantities;
    attribute def CartesianSpin3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : SpinUnit[3];
    }
    /* ISO-80000-10 item 10-11 total angular momentum */
    attribute def TotalAngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-11 total angular momentum (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TotalAngularMomentumUnit[1];
    }
    attribute totalAngularMomentum : TotalAngularMomentumValue[*] nonunique :> scalarQuantities;
    attribute def TotalAngularMomentumUnit :> DerivedUnit {
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
    attribute def CartesianTotalAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-11 total angular momentum (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: TotalAngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s*eV*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)
         * remarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianTotalAngularMomentum3dCoordinateFrame[1];
    }
    attribute cartesianTotalAngularMomentum3dVector : CartesianTotalAngularMomentum3dVector :> vectorQuantities;
    attribute def CartesianTotalAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : TotalAngularMomentumUnit[3];
    }
    /* ISO-80000-10 item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient */
    attribute def GyromagneticRatioValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: GyromagneticRatio
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum: `vec(μ)` = `γ` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1 The systematic name is "gyromagnetic coefficient", but "gyromagnetic ratio" is more usual. The gyromagnetic ratio of the proton is denoted by `γ_p`. The gyromagnetic ratio of the neutron is denoted by `γ_n`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : GyromagneticRatioUnit[1];
    }
    attribute gyromagneticRatio : GyromagneticRatioValue[*] nonunique :> scalarQuantities;
    attribute def GyromagneticRatioUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }
    alias MagnetogyricRatioUnit for GyromagneticRatioUnit;
    alias MagnetogyricRatioValue for GyromagneticRatioValue;
    alias magnetogyricRatio for gyromagneticRatio;
    alias GyromagneticCoefficientUnit for GyromagneticRatioUnit;
    alias GyromagneticCoefficientValue for GyromagneticRatioValue;
    alias gyromagneticCoefficient for gyromagneticRatio;
    /* ISO-80000-10 item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron */
    attribute def GyromagneticRatioOfTheElectronValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron
         * symbol(s): `γ_e`
         * application domain: generic
         * name: GyromagneticRatioOfTheElectron
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A
         * tensor order: 0
         * definition: proportionality constant between the magnetic dipole moment and the angular momentum of the electron `vec(μ)` = `γ_e` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)
         * remarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1
         */
        attribute :>> num : Real;
        attribute :>> mRef : GyromagneticRatioOfTheElectronUnit[1];
    }
    attribute gyromagneticRatioOfTheElectron : GyromagneticRatioOfTheElectronValue[*] nonunique :> scalarQuantities;
    attribute def GyromagneticRatioOfTheElectronUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }
    alias MagnetogyricRatioOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias MagnetogyricRatioOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias magnetogyricRatioOfTheElectron for gyromagneticRatioOfTheElectron;
    alias GyromagneticCoefficientOfTheElectronUnit for GyromagneticRatioOfTheElectronUnit;
    alias GyromagneticCoefficientOfTheElectronValue for GyromagneticRatioOfTheElectronValue;
    alias gyromagneticCoefficientOfTheElectron for gyromagneticRatioOfTheElectron;
    /* ISO-80000-10 item 10-13.1 quantum number */
    attribute def QuantumNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-13.1 quantum number
         * symbol(s): `N`, `L`, `M`, `j`, `s`, `F`
         * application domain: generic
         * name: QuantumNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number describing a particular state of a quantum system
         * remarks: Electron states determine the binding energy `E = E(n,l,m,j,s,f)` in an atom. Upper case letters `N, L, M, J, S, F` are usually used for the whole system. The spatial probability distribution of an electron is given by `|Ψ|^2`, where `Ψ` is its wave function. For an electron in an H atom in a non-relativistic approximation, the wave function can be presented as: `Ψ(r,θ,φ) = R_(nl)(r)*Y_l^m(θ,φ)`, where `r,θ,φ` are spherical coordinates (ISO 80000-2) with respect to the nucleus and to a given (quantization) axis, `R_(nl)(r)` is the radial distribution function, and `Y_l^m(θ,φ)` are spherical harmonics. In the Bohr model of one-electron atoms, `n`, `l`, and `m` define the possible orbits of an electron about the nucleus.
         */
    }
    attribute quantumNumber : QuantumNumberValue :> scalarQuantities;
    /* ISO-80000-10 item 10-13.2 principal quantum number */
    attribute principalQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.2 principal quantum number
         * symbol(s): `n`
         * application domain: generic
         * name: PrincipalQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the number `n`-1 of radial nodes of one-electron wave functions
         * remarks: In the Bohr model, `n = 1,2,…,∞` is related to the binding energy of an electron and the radius of spherical orbits (principal axis of the elliptic orbits). For an electron in an H atom, the semi-classical radius of its orbit is `r_n = a_0 n^2` and its binding energy is `E_n = E_H/n^2`.
         */
    }
    /* ISO-80000-10 item 10-13.3 orbital angular momentum quantum number */
    attribute orbitalAngularMomentumQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.3 orbital angular momentum quantum number
         * symbol(s): `l`, `l_i`, `L`
         * application domain: generic
         * name: OrbitalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the orbital angular momentum `l` of a one-electron state
         * remarks: `abs(l)^2 = ℏ^2 l (l-1)` , `l = 0, 1, …, n-1` where `vec(l)` is the orbital angular momentum and `ℏ` is the reduced Planck constant (ISO 80000-1). If reference is made to a specific particle `i`, the symbol `l_i` is used instead of `l`; if reference is made to the whole system, the symbol `L` is used instead of `l`. An electron in an H atom for `l = 0` appears as a spherical cloud. In the Bohr model, it is related to the form of the orbit.
         */
    }
    /* ISO-80000-10 item 10-13.4 magnetic quantum number */
    attribute magneticQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.4 magnetic quantum number
         * symbol(s): `m`, `m_i`, `M`
         * application domain: generic
         * name: MagneticQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: atomic quantum number related to the `z` component `l_z`, `j_z` or `s_z`, of the orbital, total, or spin angular momentum
         * remarks: `l_z = m_l ℏ` , `j_z = m_j ℏ` , and `s_z = m_s ℏ` , with the ranges from `-l` to `l`, from `-j` to `j`, and `±1/2`, respectively. `m_i` refers to a specific particle `i`. `M` is used for the whole system. Subscripts `l`, `s`, `j`, etc., as appropriate, indicate the angular momentum involved. `ℏ` is the reduced Planck constant (ISO 80000-1).
         */
    }
    /* ISO-80000-10 item 10-13.5 spin quantum number */
    attribute spinQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.5 spin quantum number
         * symbol(s): `s`
         * application domain: generic
         * name: SpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characteristic quantum number `s` of a particle, related to its spin (item 10-10), `vec(s)`: `s^2 = ℏ^2 s (s+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Spin quantum numbers of fermions are odd multiples of 1/2, and those of bosons are integers.
         */
    }
    /* ISO-80000-10 item 10-13.6 total angular momentum quantum number */
    attribute totalAngularMomentumQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.6 total angular momentum quantum number
         * symbol(s): `j`, `j_i`, `J`
         * application domain: generic
         * name: TotalAngularMomentumQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number in an atom describing the magnitude of total angular momentum `vec(J)` (item 10-11)
         * remarks: `j_i` refers to a specific particle `i`; `J` is used for the whole system. The quantum number `J` and the magnitude of total angular momentum `vec(J)` (item 10-11) are different quantities. The two values of `j` are `l`±1/2. (See item 10-13.3.)
         */
    }
    /* ISO-80000-10 item 10-13.7 nuclear spin quantum number */
    attribute nuclearSpinQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.7 nuclear spin quantum number
         * symbol(s): `I`
         * application domain: generic
         * name: NuclearSpinQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number related to the total angular momentum (item 10-11), `vec(J)`, of a nucleus in any specified state, normally called nuclear spin: `vec(J)^2 = ℏ^2 I (I+1)` where `ℏ` is the reduced Planck constant (ISO 80000-1)
         * remarks: Nuclear spin is composed of spins of the nucleons (protons and neutrons) and their (orbital) motions. In principle there is no upper limit for the nuclear spin quantum number. It has possible values `I` = 0,1,2,… for even `A` and `I = 1/2, 3/2, …` for odd `A`. In nuclear and particle physics, `vec(J)` is often used.
         */
    }
    /* ISO-80000-10 item 10-13.8 hyperfine structure quantum number */
    attribute hyperfineStructureQuantumNumber : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-13.8 hyperfine structure quantum number
         * symbol(s): `F`
         * application domain: generic
         * name: HyperfineStructureQuantumNumber (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantum number of an atom describing the inclination of the nuclear spin with respect to a quantization axis given by the magnetic field produced by the orbital electrons
         * remarks: The interval of `F` is │`I`-`J`│, │`I`-`J`│+1, ..., `I`-`J`. This is related to the hyperfine splitting of the atomic energy levels due to the interaction between the electron and nuclear magnetic moments.
         */
    }
    /* ISO-80000-10 item 10-14.1 Lande factor, g factor of atom */
    attribute def LandeFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.1 Lande factor, g factor of atom
         * symbol(s): `g`
         * application domain: generic
         * name: LandeFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the total angular momentum quantum number and the Bohr magneton: `g = μ/(J*μ_B)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `J` is total angular momentum quantum number (item 10-13.6), and `μ_B` is the Bohr magneton (item 10-9.2)
         * remarks: These quantities are also called `g` values. The Landé factor can be calculated from the expression: `g(L, S, J) = 1 + (g_e -1) xx (J(J+1) + S(S+1) - L(L+1))/(2J(J+1))` where `g_e` is the` g` factor of the electron.
         */
    }
    attribute landeFactor : LandeFactorValue :> scalarQuantities;
    alias gFactorOfAtom for landeFactor;
    /* ISO-80000-10 item 10-14.2 g factor of nucleus or nuclear particle */
    attribute def GFactorOfNucleusOrNuclearParticleValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-14.2 g factor of nucleus or nuclear particle
         * symbol(s): `g`
         * application domain: generic
         * name: GFactorOfNucleusOrNuclearParticle (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the magnetic dipole moment of an atom, and the product of the nuclear spin quantum number and the nuclear magneton: `g = μ/(I*μ_N)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `I` is nuclear spin quantum number (item 10-13.7), and `μ_N` is the nuclear magneton (item 10-9.3)
         * remarks: The `g` factors for nuclei or nucleons are known from measurements.
         */
    }
    attribute gFactorOfNucleusOrNuclearParticle : GFactorOfNucleusOrNuclearParticleValue :> scalarQuantities;
    /* ISO-80000-10 item 10-15.1 Larmor angular frequency */
    attribute larmorAngularFrequency : AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.1 Larmor angular frequency
         * symbol(s): `ω_L`
         * application domain: generic
         * name: LarmorAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: angular frequency (ISO 80000-3) of the electron angular momentum (ISO 80000-4) vector precession about the axis of an external magnetic field: `ω_L = e/(2 m_e) B` where `e` is the elementary charge (ISO 80000-1), `m_e` is the rest mass (item 10-2) of electron, and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-15.2 Larmor frequency */
    attribute def LarmorFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-15.2 Larmor frequency
         * symbol(s): `ν_L`
         * application domain: generic
         * name: LarmorFrequency
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of Larmor angular frequency (ISO 80000-3) and 2π
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LarmorFrequencyUnit[1];
    }
    attribute larmorFrequency : LarmorFrequencyValue[*] nonunique :> scalarQuantities;
    attribute def LarmorFrequencyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-10 item 10-15.3 nuclear precession angular frequency */
    attribute nuclearPrecessionAngularFrequency : AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-15.3 nuclear precession angular frequency
         * symbol(s): `ω_N`
         * application domain: generic
         * name: NuclearPrecessionAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: frequency (ISO 80000-3) by which the nucleus angular momentum vector (ISO 80000-4) precesses about the axis of an external magnetic field: `ω_N` = `γ` `B` where `γ` is the gyromagnetic ratio (item 10-12.1), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-16 cyclotron angular frequency */
    attribute cyclotronAngularFrequency : AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-16 cyclotron angular frequency
         * symbol(s): `ω_c`
         * application domain: generic
         * name: CyclotronAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: quotient of the product of the electric charge of a particle and the magnitude of the magnetic flux density of the magnetic field, and the particle mass: `ω_c = abs(q)/m B` where `q` is the electric charge (IEC 80000-6) of the particle, `m` is the mass (ISO 80000-4) of the particle, and `B` is the absolute value of the magnetic flux density (IEC 80000-6)
         * remarks: The quantity `v_c` = `ω_c`/2π is called the cyclotron frequency.
         */
    }
    /* ISO-80000-10 item 10-17 gyroradius, Larmor radius */
    attribute gyroradius : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-17 gyroradius, Larmor radius
         * symbol(s): `r_g`, `r_L`
         * application domain: generic
         * name: Gyroradius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (ISO 80000-3) of circular movement of a particle with mass (ISO 80000-4), velocity `vec(v)` (ISO 80000-3), and electric charge `q` (IEC 80000-6), moving in a magnetic field with magnetic flux density `vec(B)` (IEC 80000-6): `r_g = (m abs(vec(v) xx vec(B)))/(q B^2)`
         * remarks: None.
         */
    }
    alias larmorRadius for gyroradius;
    /* ISO-80000-10 item 10-18 nuclear quadrupole moment */
    attribute def NuclearQuadrupoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-18 nuclear quadrupole moment
         * symbol(s): `Q`
         * application domain: generic
         * name: NuclearQuadrupoleMoment
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: `z` component of the diagonalized tensor of nuclear quadrupole moment: `Q = (1/e) int (3z^2 - r^2) ρ(x, y, z) dV` in the quantum state with the nuclear spin in the field direction (`z`), where `e` is the elementary charge (ISO 80000-1), `r^2 = x^2 + y^2 + z^2`, `ρ(x,y,z)` is the nuclear electric charge density (IEC 80000-6), and `dV` is the volume element `dx dy dz`
         * remarks: The electric nuclear quadrupole moment is `eQ`. This value is equal to the `z` component of the diagonalized tensor of quadrupole moment.
         */
        attribute :>> num : Real;
        attribute :>> mRef : NuclearQuadrupoleMomentUnit[1];
    }
    attribute nuclearQuadrupoleMoment : NuclearQuadrupoleMomentValue[*] nonunique :> scalarQuantities;
    attribute def NuclearQuadrupoleMomentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-19.1 nuclear radius */
    attribute nuclearRadius : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.1 nuclear radius
         * symbol(s): `R`
         * application domain: generic
         * name: NuclearRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: conventional radius (ISO 80000-3) of sphere in which the nuclear matter is included
         * remarks: This quantity is not exactly defined. It is given approximately for nuclei in their ground state by: `R = r_0 A^(1//3)` where `r_0 ~~ 1.2 * 10^-15` m, and `A` is the nucleon number (item 10-1.3). Nuclear radius is usually expressed in femtometres, 1 fm = 10^(-15) m.
         */
    }
    /* ISO-80000-10 item 10-19.2 electron radius */
    attribute electronRadius : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-19.2 electron radius
         * symbol(s): `r_e`
         * application domain: generic
         * name: ElectronRadius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius of a sphere such that the relativistic electron energy is distributed uniformly: `r_e = e^2/(4 π ε_0 m_e c_0^2)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `m_e` is the rest mass (item 10-2) of electron, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: This quantity corresponds to the electrostatic energy `E` of a charge distributed inside a sphere of radius `r_e` as if all the rest energy (item 10-3) of the electron were attributed to the energy of electromagnetic origin, using the relation `E = m_e c_0^2`.
         */
    }
    /* ISO-80000-10 item 10-20 Compton wavelength */
    attribute comptonWavelength : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-20 Compton wavelength
         * symbol(s): `λ_C`
         * application domain: generic
         * name: ComptonWavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: quotient of the Planck constant and the product of the mass of the particle and the speed of light in vacuum: `λ_C = h / (m c_0)` where `h` is the Planck constant (ISO 80000-1), `m` is the rest mass (item 10-2) of a particle, and `c_0` is the speed of light in vacuum (ISO 80000-1)
         * remarks: The wavelength of electromagnetic radiation scattered from free electrons (Compton scattering) is larger than that of the incident radiation by a maximum of 2`λ_C`.
         */
    }
    /* ISO-80000-10 item 10-21.1 mass excess */
    attribute massExcess : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.1 mass excess
         * symbol(s): `Δ`
         * application domain: generic
         * name: MassExcess (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: difference between the mass of an atom, and the product of its mass number and the unified mass constant: `Δ = m_a - A*m_u`, where `m_a` is the rest mass (item 10-2) of the atom, `A` is its nucleon number (item 10-1.3), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. See item 10-2.
         */
    }
    /* ISO-80000-10 item 10-21.2 mass defect */
    attribute massDefect : MassValue :> scalarQuantities {
        doc
        /*
         * source: item 10-21.2 mass defect
         * symbol(s): `B`
         * application domain: generic
         * name: MassDefect (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg, Da, u
         * tensor order: 0
         * definition: sum of the product of the proton number and the hydrogen atomic mass, and the neutron rest mass, minus the rest mass of the atom: `B = Z*m(⁢^1"H") + N*m_n - m_a` where `Z` is the proton number (item 10-1.1) of the atom, `m(⁢^1"H")` is atomic mass (item 10-4.1) of `⁢^1"H"`, `N` is neutron number (item 10-1.2), `m_n` is the rest mass (item 10-2) of the neutron, and `m_a` is the rest mass (item 10-2) of the atom
         * remarks: The mass excess is usually expressed in daltons, 1 Da = 1 u. If the binding energy of the orbital electrons is neglected, `B c_0^2` is equal to the binding energy of the nucleus.
         */
    }
    /* ISO-80000-10 item 10-22.1 relative mass excess */
    attribute def RelativeMassExcessValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.1 relative mass excess
         * symbol(s): `Δ_r`
         * application domain: generic
         * name: RelativeMassExcess (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass excess and the unified atomic mass constant: `Δ_r = Δ/m_u` where `Δ` is mass excess (item 10-21.1), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassExcess : RelativeMassExcessValue :> scalarQuantities;
    /* ISO-80000-10 item 10-22.2 relative mass defect */
    attribute def RelativeMassDefectValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-22.2 relative mass defect
         * symbol(s): `B_r`
         * application domain: generic
         * name: RelativeMassDefect (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass defect and the unified atomic mass constant: `B_r = B/m_u` where `B` is mass defect (item 10-21.2), and `m_u` is the unified atomic mass constant (item 10-4.3)
         * remarks: None.
         */
    }
    attribute relativeMassDefect : RelativeMassDefectValue :> scalarQuantities;
    /* ISO-80000-10 item 10-23.1 packing fraction */
    attribute def PackingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.1 packing fraction
         * symbol(s): `f`
         * application domain: generic
         * name: PackingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass excess and the nucleon number: `f` = Δ_r/A` where `Δ_r` is relative mass excess (item 10-22.1), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute packingFraction : PackingFractionValue :> scalarQuantities;
    /* ISO-80000-10 item 10-23.2 binding fraction */
    attribute def BindingFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-23.2 binding fraction
         * symbol(s): `b`
         * application domain: generic
         * name: BindingFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relative mass defect and the nucleon number: `b = B_r/A` where `B_r` is relative mass defect (item 10-22.2), and `A` is the nucleon number (item 10-1.3)
         * remarks: None.
         */
    }
    attribute bindingFraction : BindingFractionValue :> scalarQuantities;
    /* ISO-80000-10 item 10-24 decay constant, disintegration constant */
    attribute def DecayConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-24 decay constant, disintegration constant
         * symbol(s): `λ`
         * application domain: generic
         * name: DecayConstant
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: quotient of `(-dN)/N` and `dt`, where `(dN)/N` is the mean fractional change in the number of nuclei in a particular energy state due to spontaneous transformations in a time interval of duration (ISO 80000-3) `dt`: `λ = -1/N (dN)/(dt)`
         * remarks: For exponential decay, this quantity is constant. For more than one decay channel, `λ = sum λ_a` where `λ_a` denotes the decay constant for a specified final state and the sum is taken over all final states.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DecayConstantUnit[1];
    }
    attribute decayConstant : DecayConstantValue[*] nonunique :> scalarQuantities;
    attribute def DecayConstantUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    alias DisintegrationConstantUnit for DecayConstantUnit;
    alias DisintegrationConstantValue for DecayConstantValue;
    alias disintegrationConstant for decayConstant;
    /* ISO-80000-10 item 10-25 mean duration of life, mean life time */
    attribute meanDurationOfLife : DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-25 mean duration of life, mean life time
         * symbol(s): `τ`
         * application domain: atomic and nuclear physics
         * name: MeanDurationOfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: reciprocal of the decay constant `λ` (item 10-24): `τ = 1/λ`
         * remarks: Mean duration of life is the expected value of the duration of life of an unstable particle or an excited state of a particle when the number of decay events in a short time interval follows a Poisson distribution.
         */
    }
    alias meanLifeTime for meanDurationOfLife;
    /* ISO-80000-10 item 10-26 level width */
    attribute levelWidth : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-26 level width
         * symbol(s): `Γ`
         * application domain: generic
         * name: LevelWidth (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the reduced Planck constant and the mean life: `Γ = ℏ/τ` where `ℏ` is the reduced Planck constant (ISO 80000-1), and `τ` is mean duration of life (item 10-25)
         * remarks: Level width is the uncertainty of the energy of an unstable particle or an excited state of a system due to the Heisenberg principle. The term energy level refers to the configuration of the distribution function of the density of states. Energy levels may be considered as discrete, like those in an atom, or may have a finite width, like e.g. this item or like e.g. the valence or conduction band in solid state physics. Energy levels are applicable to both real and virtual particles, e.g. electrons and phonons, respectively.
         */
    }
    /* ISO-80000-10 item 10-27 nuclear activity */
    attribute def NuclearActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-27 nuclear activity
         * symbol(s): `A`
         * application domain: generic
         * name: NuclearActivity
         * quantity dimension: T^-1
         * measurement unit(s): Bq, s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the mean change in the number of nuclei in a particular energy state due to spontaneous nuclear transformations in a time interval of duration (ISO 80000-3) `dt`: `A = -(dN)/(dt)`
         * remarks: For exponential decay, `A = λN`, where `λ` is the decay constant (item 10-24). The becquerel (Bq) is a special name for second to the power minus one, to be used as the coherent SI unit of activity. In report 85a of the ICRU a definition with an equivalent meaning is given as: The activity, `A`, of an amount of a radionuclide in a particular energy state at a given time is the quotient of `-dN` by `dt`, where `dN` is the mean change in the number of nuclei in that energy state due to spontaneous nuclear transformations in the time interval `dt`: `A = -(dN)/(dt)`. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : NuclearActivityUnit[1];
    }
    attribute nuclearActivity : NuclearActivityValue[*] nonunique :> scalarQuantities;
    attribute def NuclearActivityUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-10 item 10-28 specific activity, massic activity */
    attribute def SpecificActivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-28 specific activity, massic activity
         * symbol(s): `a`
         * application domain: generic
         * name: SpecificActivity
         * quantity dimension: M^-1*T^-1
         * measurement unit(s): Bq/kg, kg^-1*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificActivityUnit[1];
    }
    attribute specificActivity : SpecificActivityValue[*] nonunique :> scalarQuantities;
    attribute def SpecificActivityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    alias MassicActivityUnit for SpecificActivityUnit;
    alias MassicActivityValue for SpecificActivityValue;
    alias massicActivity for specificActivity;
    /* ISO-80000-10 item 10-29 activity density, volumic activity, activity concentration */
    attribute def ActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-29 activity density, volumic activity, activity concentration
         * symbol(s): `c_A`
         * application domain: generic
         * name: ActivityDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): Bq/m^3, m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ActivityDensityUnit[1];
    }
    attribute activityDensity : ActivityDensityValue[*] nonunique :> scalarQuantities;
    attribute def ActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    alias VolumicActivityUnit for ActivityDensityUnit;
    alias VolumicActivityValue for ActivityDensityValue;
    alias volumicActivity for activityDensity;
    alias ActivityConcentrationUnit for ActivityDensityUnit;
    alias ActivityConcentrationValue for ActivityDensityValue;
    alias activityConcentration for activityDensity;
    /* ISO-80000-10 item 10-30 surface-activity density */
    attribute def SurfaceActivityDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-30 surface-activity density
         * symbol(s): `a_S`
         * application domain: generic
         * name: SurfaceActivityDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): Bq/m^2, m^-2*s^-1
         * tensor order: 0
         * definition: quotient of the activity `A` (item 10-27) of a sample and the total area `S` (ISO 80000-3) of the surface of that sample: `a_S` = `A`/`S`
         * remarks: This value is usually defined for flat sources, where `S` corresponds to the total area of surface of one side of the source.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceActivityDensityUnit[1];
    }
    attribute surfaceActivityDensity : SurfaceActivityDensityValue[*] nonunique :> scalarQuantities;
    attribute def SurfaceActivityDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-31 half life */
    attribute halfLife : DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-31 half life
         * symbol(s): `T_(1/2)`
         * application domain: generic
         * name: HalfLife (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: mean duration (ISO 80000-3) required for the decay of one half of the atoms or nuclei
         * remarks: For exponential decay, `T_(1/2) = (ln2)/λ`, where `λ` is the decay constant (item 10-24).
         */
    }
    /* ISO-80000-10 item 10-32 alpha disintegration energy */
    attribute alphaDisintegrationEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-32 alpha disintegration energy
         * symbol(s): `Q_α`
         * application domain: generic
         * name: AlphaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the kinetic energy (ISO 80000-4) of the α-particle produced in the disintegration process and the recoil energy (ISO 80000-5) of the product atom in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: The ground-state alpha disintegration energy, `Q_(α,0)`, also includes the energy of any nuclear transitions that take place in the daughter produced.
         */
    }
    /* ISO-80000-10 item 10-33 maximum beta-particle energy */
    attribute maximumBetaParticleEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-33 maximum beta-particle energy
         * symbol(s): `E_β`
         * application domain: generic
         * name: MaximumBetaParticleEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: maximum kinetic energy (ISO 80000-4) of the emitted beta particle produced in the nuclear disintegration process
         * remarks: The maximum kinetic energy corresponds to the highest energy of the beta spectrum.
         */
    }
    /* ISO-80000-10 item 10-34 beta disintegration energy */
    attribute betaDisintegrationEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-34 beta disintegration energy
         * symbol(s): `Q_β`
         * application domain: generic
         * name: BetaDisintegrationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of the maximum beta-particle kinetic energy (item 10-33) and the recoil energy (ISO 80000-5) of the atom produced in a reference frame in which the emitting nucleus is at rest before its disintegration
         * remarks: For positron emitters, the energy for the production of the annihilation radiation created in the combination of an electron with the positron is part of the beta disintegration energy. The ground-state beta disintegration energy, `Q_(β,0)`, also includes the energy of any nuclear transitions that take place in the daughter product.
         */
    }
    /* ISO-80000-10 item 10-35 internal conversion factor */
    attribute def InternalConversionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-35 internal conversion factor
         * symbol(s): `α`
         * application domain: generic
         * name: InternalConversionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the number of internal conversion electrons and the number of gamma quanta emitted by the radioactive atom in a given transition, where a conversion electron represents an orbital electron emitted through the radioactive decay
         * remarks: The quantity `α/(α+1)` is also used and called the internal-conversion fraction. Partial conversion fractions referring to the various electron shells `K, L, ...` are indicated by `α_K`, `α_L`, ... `α_K/α_L` is called the K-to-L internal conversion ratio.
         */
    }
    attribute internalConversionFactor : InternalConversionFactorValue :> scalarQuantities;
    /* ISO-80000-10 item 10-36 particle emission rate */
    attribute def ParticleEmissionRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-36 particle emission rate
         * symbol(s): `dot(N)`
         * application domain: generic
         * name: ParticleEmissionRate
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: differential quotient of `N` with respect to time, where `N` is the number of particles being emitted from an infinitesimally small volume element in the time interval of duration `dt` (ISO 80000-3), and `dt`: `dot(N) = (dN)/(dt)`
         * remarks: Usually the kind of particles is specified, e.g. neutron emission rate or alpha particle emission rate.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleEmissionRateUnit[1];
    }
    attribute particleEmissionRate : ParticleEmissionRateValue[*] nonunique :> scalarQuantities;
    attribute def ParticleEmissionRateUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-10 item 10-37.1 reaction energy */
    attribute reactionEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.1 reaction energy
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactionEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a nuclear reaction, sum of the kinetic energies (ISO 80000-4) and photon energies (ISO 80000-5) of the reaction products minus the sum of the kinetic and photon energies of the reactants
         * remarks: For exothermic nuclear reactions, `Q>0`. For endothermic nuclear reactions, `Q<0`.
         */
    }
    /* ISO-80000-10 item 10-37.2 resonance energy */
    attribute resonanceEnergy : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-37.2 resonance energy
         * symbol(s): `E_r`, `E_"res"`
         * application domain: generic
         * name: ResonanceEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: kinetic energy (ISO 80000-4) of an incident particle, in the reference frame of the target, corresponding to a resonance in a nuclear reaction
         * remarks: The energy of the resonance corresponds to the difference of the energy levels involved of the nucleus.
         */
    }
    /* ISO-80000-10 item 10-38.1 cross section */
    attribute crossSection : AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.1 cross section
         * symbol(s): `σ`
         * application domain: atomic physics
         * name: CrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: for a specified target entity and for a specified reaction or process produced by incident charged or uncharged particles of a given type and energy, the quotient of the mean number of such reactions or processes and the incident-particle fluence (item 10-43)
         * remarks: The type of process is indicated by subscripts, e.g. absorption cross section `σ_a`, scattering cross section `σ_s`, fission cross section `σ_f`. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }
    /* ISO-80000-10 item 10-38.2 total cross section */
    attribute totalCrossSection : AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-38.2 total cross section
         * symbol(s): `σ_"tot"`, `σ_"T"`
         * application domain: atomic physics
         * name: TotalCrossSection (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2, b
         * tensor order: 0
         * definition: sum of all cross sections (item 10-38.1) corresponding to the various reactions or processes between an incident particle of specified type and energy (ISO 80000-5) and a target entity
         * remarks: In the case of a narrow unidirectional beam of incident particles, this is the effective cross section for the removal of an incident particle from the beam. See the Remarks for item 10-52. `1 "barn" ("b") = 10^(-28) "m"^2`.
         */
    }
    /* ISO-80000-10 item 10-39 direction distribution of cross section */
    attribute def DirectionDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-39 direction distribution of cross section
         * symbol(s): `σ_Ω`
         * application domain: atomic physics
         * name: DirectionDistributionOfCrossSection
         * quantity dimension: L^2
         * measurement unit(s): m^2*sr^-1, m^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to `Ω`, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a specified direction, and `Ω` is the solid angle (ISO 80000-3) around that direction: `σ_Ω = (dσ)/(dΩ)`
         * remarks: Quantities listed under items 10-39, 10-40 and 10-41 are sometimes called differential cross sections. The type of interaction needs to be specified.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DirectionDistributionOfCrossSectionUnit[1];
    }
    attribute directionDistributionOfCrossSection : DirectionDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;
    attribute def DirectionDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-40 energy distribution of cross section */
    attribute def EnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-40 energy distribution of cross section
         * symbol(s): `σ_E`
         * application domain: atomic physics
         * name: EnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/J, kg^-1*s^2
         * tensor order: 0
         * definition: differential quotient of `σ` with respect to energy, where `σ` is the cross section (item 10-38.1) for a process in which the energy `E` (ISO 80000-5) of the ejected or scattered particle is between `E` and `E + dE`: `σ_E = (dσ)/(dE)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyDistributionOfCrossSectionUnit[1];
    }
    attribute energyDistributionOfCrossSection : EnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;
    attribute def EnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-41 direction and energy distribution of cross section */
    attribute def DirectionAndEnergyDistributionOfCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-41 direction and energy distribution of cross section
         * symbol(s): `σ_(Ω,E)`
         * application domain: atomic physics
         * name: DirectionAndEnergyDistributionOfCrossSection
         * quantity dimension: M^-1*T^2
         * measurement unit(s): m^2/(J*sr), kg^-1*s^2
         * tensor order: 0
         * definition: partial differential quotient of `σ` with respect to solid angle and energy, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a solid angle `dΩ` around a specified direction and with an energy between `E` and `E+dE`: `σ_(Ω,E) = (del^2 σ) / (del Ω del E)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DirectionAndEnergyDistributionOfCrossSectionUnit[1];
    }
    attribute directionAndEnergyDistributionOfCrossSection : DirectionAndEnergyDistributionOfCrossSectionValue[*] nonunique :> scalarQuantities;
    attribute def DirectionAndEnergyDistributionOfCrossSectionUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-42.1 volumic cross section, macroscopic cross section */
    attribute def VolumicCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.1 volumic cross section, macroscopic cross section
         * symbol(s): `Σ`
         * application domain: atomic physics
         * name: VolumicCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and of the cross section (item 10-38.1) `σ_a` for a given type of atoms: `Σ = n_a σ_a`
         * remarks: When the target particles of the medium are at rest, `Σ = 1/l`, where `l` is the mean free path (item 10-71).
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumicCrossSectionUnit[1];
    }
    attribute volumicCrossSection : VolumicCrossSectionValue[*] nonunique :> scalarQuantities;
    attribute def VolumicCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias MacroscopicCrossSectionUnit for VolumicCrossSectionUnit;
    alias MacroscopicCrossSectionValue for VolumicCrossSectionValue;
    alias macroscopicCrossSection for volumicCrossSection;
    /* ISO-80000-10 item 10-42.2 volumic total cross section, macroscopic total cross section */
    attribute def VolumicTotalCrossSectionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-42.2 volumic total cross section, macroscopic total cross section
         * symbol(s): `Σ_"tot"`, `Σ_"T"`
         * application domain: atomic physics
         * name: VolumicTotalCrossSection
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: product of the number density `n_a` of the atoms and the cross section (item 10-38.1) `σ_"tot"` for a given type of atoms: `Σ_"tot" = n_a*σ_"tot"`
         * remarks: See the Remarks for item 10-49.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumicTotalCrossSectionUnit[1];
    }
    attribute volumicTotalCrossSection : VolumicTotalCrossSectionValue[*] nonunique :> scalarQuantities;
    attribute def VolumicTotalCrossSectionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias MacroscopicTotalCrossSectionUnit for VolumicTotalCrossSectionUnit;
    alias MacroscopicTotalCrossSectionValue for VolumicTotalCrossSectionValue;
    alias macroscopicTotalCrossSection for volumicTotalCrossSection;
    /* ISO-80000-10 item 10-43 particle fluence */
    attribute def ParticleFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-43 particle fluence
         * symbol(s): `Φ`
         * application domain: generic
         * name: ParticleFluence
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: differential quotient of `N` with respect to `a`, where `N` is the number of particles incident on a sphere of cross-sectional area `a` (item 10-38.1): `Φ = (dN)/(da)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` fluence. If a flat area of size `dA` is passed perpendicularly by a number of `dN` particles, the corresponding particle fluence is: `Φ = (dN)/(dA)`. A plane area of size `dA` crossed at an angle `α` with respect to the surface normal by a number of `dN` particles results in the particle fluence: `Φ = (dN)/(cos(α) dA)` In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence, `Φ` , is the quotient of `dN` and `da`, where `dN` is the number of particles incident on a sphere of cross-sectional area `da`: `Φ = (dN)/(dA)`. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleFluenceUnit[1];
    }
    attribute particleFluence : ParticleFluenceValue[*] nonunique :> scalarQuantities;
    attribute def ParticleFluenceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-44 particle fluence rate */
    attribute def ParticleFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-44 particle fluence rate
         * symbol(s): `dot(Φ)`
         * application domain: generic
         * name: ParticleFluenceRate
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: differential quotient of fluence `Φ` (item 10-43) with respect to time (ISO 80000-3): `dot(Φ) = (dΦ)/(dA)`
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example proton fluence rate. The distribution function expressed in terms of speed and energy, `dot(Φ)_v` and `dot(Φ)_E` , are related to by: `dot(Φ) = int dot(Φ)_v dv = int dot(Φ)_E dE`. This quantity has also been termed particle flux density. Because the word "density" has several connotations, the term "fluence rate" is preferred. For a radiation field composed of particles of velocity `v`, the fluence rate is equal to `n`·`v` where `n` is the particle number density. See Remarks for item 10-43. In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence rate, `dot(Φ)` , is the quotient of `d Φ` and `dt`, where `d Φ` is the increment of the fluence in the time interval `dt`: `dot(Φ) = (dΦ)/(dt)`. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleFluenceRateUnit[1];
    }
    attribute particleFluenceRate : ParticleFluenceRateValue[*] nonunique :> scalarQuantities;
    attribute def ParticleFluenceRateUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-45 radiant energy */
    attribute radiantEnergyForIonizingRadiation : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-45 radiant energy
         * symbol(s): `R`
         * application domain: ionizing radiation
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: mean energy (ISO 80000-5), excluding rest energy (item 10-3), of the particles that are emitted, transferred, or received
         * remarks: For particles of energy `E` (excluding rest energy), the radiant energy, `R`, is equal to the product `N·E` where `N` is the number of the particles that are emitted, transferred, or received The distributions, `N_E` and `R_E`, of the particle number and the radiant energy with respect to energy are given by `N_E` = `dN`/d`E` and `R_E` = `dR`/d`E`, respectively, where `dN` is the number of particles with energy between `E` and `E`+d`E`, and `dR` is their radiant energy. The two distributions are related by `R_E` = `E`·`N_E`.
         */
    }
    /* ISO-80000-10 item 10-46 energy fluence */
    attribute def EnergyFluenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-46 energy fluence
         * symbol(s): `Ψ`
         * application domain: generic
         * name: EnergyFluence
         * quantity dimension: M^1*T^-2
         * measurement unit(s): eV/m^2, J/m^2, kg*s^-2
         * tensor order: 0
         * definition: differential quotient of radiant energy `R` (item 10-45) incident on a sphere of cross-sectional area (item 10-38.1) `a` with respect to that area: `Ψ = (dR)/(da)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy fluence, `Ψ` is the quotient of `dR` and `da`, where `dR` is the radiant energy incident on a sphere of cross-sectional area `da`: `Ψ = (dR)/(da)`. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyFluenceUnit[1];
    }
    attribute energyFluence : EnergyFluenceValue[*] nonunique :> scalarQuantities;
    attribute def EnergyFluenceUnit :> DerivedUnit {
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
    /* ISO-80000-10 item 10-47 energy fluence rate */
    attribute def EnergyFluenceRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-47 energy fluence rate
         * symbol(s): `dot(Ψ)`
         * application domain: generic
         * name: EnergyFluenceRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: differential quotient of the energy fluence `Ψ` (item 10-46) with respect to time (ISO 80000-3): `dot(Ψ) = (d Ψ)/(dt)`
         * remarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy-fluence rate, `dot(Ψ)` , is the quotient of `d Ψ` by `dt`, where `d Ψ` is the increment of the energy fluence in the time interval `dt`: `dot(Ψ) = (d Ψ)/(dt)`. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyFluenceRateUnit[1];
    }
    attribute energyFluenceRate : EnergyFluenceRateValue[*] nonunique :> scalarQuantities;
    attribute def EnergyFluenceRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-48 particle current density */
    attribute def ParticleCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-48 particle current density (magnitude)
         * symbol(s): `J`, `S`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleCurrentDensityUnit[1];
    }
    attribute particleCurrentDensity : ParticleCurrentDensityValue[*] nonunique :> scalarQuantities;
    attribute def ParticleCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def CartesianParticleCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 10-48 particle current density (vector)
         * symbol(s): `vec(J)`, `vec(S)`
         * application domain: generic
         * name: ParticleCurrentDensity
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 1
         * definition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively
         * remarks: Usually the word "particle" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianParticleCurrentDensity3dCoordinateFrame[1];
    }
    attribute cartesianParticleCurrentDensity3dVector : CartesianParticleCurrentDensity3dVector :> vectorQuantities;
    attribute def CartesianParticleCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ParticleCurrentDensityUnit[3];
    }
    /* ISO-80000-10 item 10-49 linear attenuation coefficient */
    attribute def LinearAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-49 linear attenuation coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: ionizing radiation
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: for uncharged particles of a given type and energy the differential quotient `n` with respect to `l,` where `n` is the fraction of `N` incoming particles that experience interactions in traversing a distance (ISO 80000-3) `l` in a given material: `μ = (dn)/(dl) = 1/N (dN)/(dl)` where `dN` is the number of particles that experience interactions in traversing `dl`
         * remarks: `μ` is equal to the macroscopic total cross section `Σ_"tot"` for the removal of particles from the beam. Using the relation `μ_m = μ/ρ` between the linear attenuation coefficient `μ`, the mass attenuation coefficient `μ_m` (item 10-50) and the density `ρ`, the definition given for the mass attenuation coefficient in report 85a of the ICRU can be applied to the linear attenuation coefficient resulting in: The linear attenuation coefficient, `μ`, of a material, for uncharged particles of a given type and energy, is the quotient of `(dN)/N` by `dl`, where `(dN)/N` is the mean fraction of the particles that experience interactions in traversing a distance `dl` in the material: `μ = 1/(dl) (dN)/(N)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearAttenuationCoefficientForIonizingRadiationUnit[1];
    }
    attribute linearAttenuationCoefficientForIonizingRadiation : LinearAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;
    attribute def LinearAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-50 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-50 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: ionizing radiation
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the mass density `ρ` (ISO 80000-4) of the medium: `μ_m = μ/ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassAttenuationCoefficientForIonizingRadiationUnit[1];
    }
    attribute massAttenuationCoefficientForIonizingRadiation : MassAttenuationCoefficientForIonizingRadiationValue[*] nonunique :> scalarQuantities;
    attribute def MassAttenuationCoefficientForIonizingRadiationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    /* ISO-80000-10 item 10-51 molar attenuation coefficient */
    attribute def MolarAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-51 molar attenuation coefficient
         * symbol(s): `μ_c`
         * application domain: generic
         * name: MolarAttenuationCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: quotient of linear attenuation coefficient `µ` (item 10-49) and the amount c (ISO 80000-9) of the medium: `μ_c = μ/c`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarAttenuationCoefficientUnit[1];
    }
    attribute molarAttenuationCoefficient : MolarAttenuationCoefficientValue[*] nonunique :> scalarQuantities;
    attribute def MolarAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }
    /* ISO-80000-10 item 10-52 atomic attenuation coefficient */
    attribute def AtomicAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-52 atomic attenuation coefficient
         * symbol(s): `μ_a`
         * application domain: generic
         * name: AtomicAttenuationCoefficient
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the number density (item 10-62.1), `n`, of atoms in the substance: `μ_a = μ/n`
         * remarks: `μ` is equal to the total cross section `σ_"tot"` for the removal of particles from the beam. See also item 10-38.2.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AtomicAttenuationCoefficientUnit[1];
    }
    attribute atomicAttenuationCoefficient : AtomicAttenuationCoefficientValue[*] nonunique :> scalarQuantities;
    attribute def AtomicAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-53 half-value thickness */
    attribute halfValueThickness : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-53 half-value thickness
         * symbol(s): `d_(1//2)`
         * application domain: generic
         * name: HalfValueThickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: thickness (ISO 80000-3) of the attenuating layer that reduces the quantity of interest of a unidirectional beam of infinitesimal width to half of its initial value
         * remarks: For exponential attenuation, `d_(1/2) = ln(2)/μ`. The quantity of interest is often the air kerma or exposure.
         */
    }
    /* ISO-80000-10 item 10-54 total linear stopping power, linear stopping power */
    attribute def TotalLinearStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-54 total linear stopping power, linear stopping power
         * symbol(s): `S`, `S_l`
         * application domain: generic
         * name: TotalLinearStoppingPower
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: for charged particles of a given type and energy `E_0` the differential quotient of `E` with respect to `x,` where `E` is the mean energy (ISO 80000-4) lost by the charged particles in traversing a distance (ISO 80000-3) `x` in the given material: `S = -(dE)/(dx)`
         * remarks: The total linear stopping power is sometimes also called stopping power. Both electronic losses and radiative losses are included. The quotient of the total linear stopping power of a substance and that of a reference substance is called the relative linear stopping power. See also item 10-85. Using the relation `S_m = S/ρ` between the total mass stopping power `S_m` (item 10-55), the total linear stopping power `S`, and the density `ρ`, the definition given for the mass stopping in report 85a of the ICRU can be applied to that of the total linear stopping power resulting in: The linear stopping power, `S`, of a material, for charged particles of a given type and energy, is the quotient of `dE` by `dl`, where `dE` is the mean energy lost by the charged particles in traversing a distance `dl` in the material: `S = -(dE)/(dx)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TotalLinearStoppingPowerUnit[1];
    }
    attribute totalLinearStoppingPower : TotalLinearStoppingPowerValue[*] nonunique :> scalarQuantities;
    attribute def TotalLinearStoppingPowerUnit :> DerivedUnit {
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
    alias LinearStoppingPowerUnit for TotalLinearStoppingPowerUnit;
    alias LinearStoppingPowerValue for TotalLinearStoppingPowerValue;
    alias linearStoppingPower for totalLinearStoppingPower;
    /* ISO-80000-10 item 10-55 total mass stopping power, mass stopping power */
    attribute def TotalMassStoppingPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-55 total mass stopping power, mass stopping power
         * symbol(s): `S_m`
         * application domain: generic
         * name: TotalMassStoppingPower
         * quantity dimension: L^4*T^-2
         * measurement unit(s): eV*m^-2/kg, J*m^2/kg, m^4*s^-2
         * tensor order: 0
         * definition: quotient of the total linear stopping power `S` (item 10-54) and the mass density `ρ` (ISO 80000-4) of the material: `S_m = S/ρ`
         * remarks: The quotient of total mass stopping power of a material and that of a reference material is called relative mass stopping power.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TotalMassStoppingPowerUnit[1];
    }
    attribute totalMassStoppingPower : TotalMassStoppingPowerValue[*] nonunique :> scalarQuantities;
    attribute def TotalMassStoppingPowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 4;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    alias MassStoppingPowerUnit for TotalMassStoppingPowerUnit;
    alias MassStoppingPowerValue for TotalMassStoppingPowerValue;
    alias massStoppingPower for totalMassStoppingPower;
    /* ISO-80000-10 item 10-56 mean linear range */
    attribute meanLinearRange : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-56 mean linear range
         * symbol(s): `R`, `R_l`
         * application domain: generic
         * name: MeanLinearRange (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: mean total rectified path length (ISO 80000-3) travelled by a particle in the course of slowing down to rest in a given material averaged over a group of particles having the same initial energy (ISO 80000-5)
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-57 mean mass range */
    attribute def MeanMassRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-57 mean mass range
         * symbol(s): `R_ρ`, `R_m`
         * application domain: generic
         * name: MeanMassRange
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: product of the mean linear range (item 10-56) `R` and the mass density `ρ` (ISO 80000-4) of the material: `R_ρ = R*ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MeanMassRangeUnit[1];
    }
    attribute meanMassRange : MeanMassRangeValue[*] nonunique :> scalarQuantities;
    attribute def MeanMassRangeUnit :> DerivedUnit {
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
    /* ISO-80000-10 item 10-58 linear ionization */
    attribute def LinearIonizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-58 linear ionization
         * symbol(s): `N_{i_l}`
         * application domain: generic
         * name: LinearIonization
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: differential quotient of `q` with respect to `l`, where `q` is the average total charge (IEC 80000-6) of all positive ions produced by an ionizing charged particle over a path `l` (ISO 80000-3), divided by the elementary charge, `e` (ISO 80000-1): `N_{i_l} = 1/e*(dq)/(dl)`
         * remarks: Ionization due to secondary ionizing particles is included.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearIonizationUnit[1];
    }
    attribute linearIonization : LinearIonizationValue[*] nonunique :> scalarQuantities;
    attribute def LinearIonizationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-59 total ionization */
    attribute def TotalIonizationValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-59 total ionization
         * symbol(s): `N_i`
         * application domain: generic
         * name: TotalIonization (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total mean charge of all positive ions produced by an ionizing charged particle along its entire path and along the paths of any secondary charged particles, and the elementary charge, `e` (ISO 80000-1)
         * remarks: `N_i = int N_(il) dl` See item 10-58.
         */
    }
    attribute totalIonization : TotalIonizationValue :> scalarQuantities;
    /* ISO-80000-10 item 10-60 average energy loss per elementary charge produced */
    attribute def AverageEnergyLossPerElementaryChargeProducedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-60 average energy loss per elementary charge produced
         * symbol(s): `W_i`
         * application domain: generic
         * name: AverageEnergyLossPerElementaryChargeProduced
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: quotient of the initial kinetic energy `E_k` (ISO 80000-4) of an ionizing charged particle and the total ionization `N_i` (item 10-59) produced by that particle: `W_i = E_k/N_i`
         * remarks: The name "average energy loss per ion pair formed" is usually used, although it is ambiguous. In the practical dosimetry of ionizing radiation the term `W`/`e`, the quotient of `W`, the average energy deposited in dry air per ion pair formed, and `e`, the elementary charge, is used as the factor which, when multiplied with the electric charge of one sign carried by all ion pairs formed in dry air of given mass, gives the energy deposited in this amount of dry air in the form of excitations and ionizations. In ICRU Report 85a, the mean energy expended in a gas per ion pair formed, `W`, is the quotient of `E` by `N,` where `N` is the mean total liberated charge of either sign, divided by the elementary charge when the initial kinetic energy `E` of a charged particle introduced into the gas is completely dissipated in the gas. Thus, `W` = `E`/`N`. It follows from the definition of `W` that the ions produced by bremsstrahlung or other secondary radiation emitted by the initial and secondary charged particles are included in `N`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AverageEnergyLossPerElementaryChargeProducedUnit[1];
    }
    attribute averageEnergyLossPerElementaryChargeProduced : AverageEnergyLossPerElementaryChargeProducedValue[*] nonunique :> scalarQuantities;
    attribute def AverageEnergyLossPerElementaryChargeProducedUnit :> DerivedUnit {
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
    /* ISO-80000-10 item 10-61 mobility */
    attribute def MobilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-61 mobility
         * symbol(s): `μ`, `μ_m`
         * application domain: generic
         * name: Mobility
         * quantity dimension: M^-1*T^2*I^1
         * measurement unit(s): m^2/(V*s), kg^-1*s^2*A
         * tensor order: 0
         * definition: quotient of average drift speed (ISO 80000-3) imparted to a charged particle in a medium by an electric field, and the electric field strength (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MobilityUnit[1];
    }
    attribute mobility : MobilityValue[*] nonunique :> scalarQuantities;
    attribute def MobilityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }
    /* ISO-80000-10 item 10-62.1 particle number density */
    attribute def ParticleNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.1 particle number density
         * symbol(s): `n`
         * application domain: generic
         * name: ParticleNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the mean number `N` of particles in the volume (ISO 80000-3) `V` and volume: `n = N/V`
         * remarks: `n` is the general symbol for the number density of particles. The distribution functions expressed in terms of speed and energy, `n_v` and `n_E`, are related to `n` by: `n = int n_v dv = int n_E dE`. The word "particle" is usually replaced by the name of a specific particle, for example `neutron` number density.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleNumberDensityUnit[1];
    }
    attribute particleNumberDensity : ParticleNumberDensityValue[*] nonunique :> scalarQuantities;
    attribute def ParticleNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-10 item 10-62.2 ion number density, ion density */
    attribute def IonNumberDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-62.2 ion number density, ion density
         * symbol(s): `n^"+"`, `n^"-"`
         * application domain: generic
         * name: IonNumberDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of the number of positive and negative ions, `N^"+"` and `N^"-"`, respectively, in the volume `V` (ISO 80000-3), and that volume: `n^"+" = N^"+" / V`, `n^"-" = N^"-" / V`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IonNumberDensityUnit[1];
    }
    attribute ionNumberDensity : IonNumberDensityValue[*] nonunique :> scalarQuantities;
    attribute def IonNumberDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias IonDensityUnit for IonNumberDensityUnit;
    alias IonDensityValue for IonNumberDensityValue;
    alias ionDensity for ionNumberDensity;
    /* ISO-80000-10 item 10-63 Recombination coefficient */
    attribute def RecombinationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-63 Recombination coefficient
         * symbol(s): `α`
         * application domain: generic
         * name: RecombinationCoefficient
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: coefficient in the law of recombination: `-(dn^"+")/(dt) = -(dn^"-")/(dt) = α*n^"+"*n^"-"`, where `n^"+"` and `n^"-"` are the ion number densities (item 10-62.2) of positive and negative ions, respectively, recombined during a time interval of duration `dt` (ISO 80000-3)
         * remarks: The widely used term "recombination factor" is not correct because "factor" should only be used for quantities with dimension 1. The terms `(dn^"+")/(dt)` , `(dn^"-")/(dt)` are differential quotients.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RecombinationCoefficientUnit[1];
    }
    attribute recombinationCoefficient : RecombinationCoefficientValue[*] nonunique :> scalarQuantities;
    attribute def RecombinationCoefficientUnit :> DerivedUnit {
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
    /* ISO-80000-10 item 10-64 diffusion coefficient, diffusion coefficient for particle number density */
    /* Refer to declaration for DiffusionCoefficient in ISQChemistryMolecular item 9-39 diffusion coefficient */
    alias DiffusionCoefficientForParticleNumberDensityUnit for DiffusionCoefficientUnit;
    alias DiffusionCoefficientForParticleNumberDensityValue for DiffusionCoefficientValue;
    alias diffusionCoefficientForParticleNumberDensity for diffusionCoefficient;
    /* ISO-80000-10 item 10-65 diffusion coefficient for fluence rate */
    attribute diffusionCoefficientForFluenceRate : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-65 diffusion coefficient for fluence rate
         * symbol(s): `D_ϕ`, `D`
         * application domain: generic
         * name: DiffusionCoefficientForFluenceRate (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: proportionality constant between the particle current density `vec(J )`(item 10-48) and the gradient of the particle fluence rate `dot(Φ)` (item 10-44): `vec(J) = -vec(D) * nabla Φ`
         * remarks: For a particle of a given speed `v`: `D_Ψ(v) = -J_{v,x}/(partial Ψ // partial x)` and `vec(v) * vec(D_Ψ)(v) = -vec(D_n)(v)`
         */
    }
    /* ISO-80000-10 item 10-66 particle source density */
    attribute def ParticleSourceDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-66 particle source density
         * symbol(s): `S`
         * application domain: generic
         * name: ParticleSourceDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: quotient of the mean rate of production of particles in a volume, and that volume (ISO 80000-3)
         * remarks: The word "particle" is usually replaced by the name of a specific particle, for example `proton` source density. The distribution functions expressed in terms of speed and energy, `S_v` and `S_E`, are related to `S` by: `S = int S_v dv = int S_E dE`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleSourceDensityUnit[1];
    }
    attribute particleSourceDensity : ParticleSourceDensityValue[*] nonunique :> scalarQuantities;
    attribute def ParticleSourceDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-67 slowing-down density */
    attribute def SlowingDownDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-67 slowing-down density
         * symbol(s): `q`
         * application domain: generic
         * name: SlowingDownDensity
         * quantity dimension: L^-3*T^-1
         * measurement unit(s): m^-3*s^-1
         * tensor order: 0
         * definition: differential quotient of `n` with respect to time, where `n` is the number density of particles that are slowed down in a time interval of duration (ISO 80000-3) `t`: `q = -(dn)/(dt)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SlowingDownDensityUnit[1];
    }
    attribute slowingDownDensity : SlowingDownDensityValue[*] nonunique :> scalarQuantities;
    attribute def SlowingDownDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-68 resonance escape probability */
    attribute def ResonanceEscapeProbabilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-68 resonance escape probability
         * symbol(s): `p`
         * application domain: generic
         * name: ResonanceEscapeProbability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the probability that a neutron slowing down will traverse all or some specified portion of the range of resonance energies (item 10-37.2) without being absorbed
         * remarks: None.
         */
    }
    attribute resonanceEscapeProbability : ResonanceEscapeProbabilityValue :> scalarQuantities;
    /* ISO-80000-10 item 10-69 lethargy */
    attribute def LethargyValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-69 lethargy
         * symbol(s): `u`
         * application domain: generic
         * name: Lethargy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a neutron of kinetic energy `E` (ISO 80000-4) : `u = ln(E_0/E)`, where `E_0` is a reference energy
         * remarks: Lethargy is also referred to as logarithmic energy decrement.
         */
    }
    attribute lethargy : LethargyValue :> scalarQuantities;
    /* ISO-80000-10 item 10-70 average logarithmic energy decrement */
    attribute def AverageLogarithmicEnergyDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 10-70 average logarithmic energy decrement
         * symbol(s): `ζ`
         * application domain: generic
         * name: AverageLogarithmicEnergyDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average value of the increase in lethargy (item 10-69) in elastic collisions between neutrons and nuclei whose kinetic energy (ISO 80000-4) is negligible compared with that of the neutrons
         * remarks: None.
         */
    }
    attribute averageLogarithmicEnergyDecrement : AverageLogarithmicEnergyDecrementValue :> scalarQuantities;
    /* ISO-80000-10 item 10-71 mean free path */
    attribute meanFreePathForAtomicPhysics : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-71 mean free path
         * symbol(s): `l`, `λ`
         * application domain: atomic physics
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that particles travel between two successive specified reactions or processes
         * remarks: See the Remarks for item 10-42.1.
         */
    }
    /* ISO-80000-10 item 10-72.1 slowing-down area */
    attribute slowingDownArea : AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.1 slowing-down area
         * symbol(s): `L_s^2`, `L_"sl"^2`
         * application domain: generic
         * name: SlowingDownArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square of the distance (ISO 80000-3) between the neutron source and the point where a neutron reaches a given energy (ISO 80000-5)
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-72.2 diffusion area */
    attribute diffusionArea : AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.2 diffusion area
         * symbol(s): `L^2`
         * application domain: generic
         * name: DiffusionArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: in an infinite homogenous medium, one-sixth of the mean square distance (ISO 80000-3) between the point where a neutron enters a specified class and the point where it leaves this class
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
    }
    /* ISO-80000-10 item 10-72.3 migration area */
    attribute migrationArea : AreaValue :> scalarQuantities {
        doc
        /*
         * source: item 10-72.3 migration area
         * symbol(s): `M^2`
         * application domain: generic
         * name: MigrationArea (specializes Area)
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: sum of the slowing-down area (item 10-72.1) from fission energy to thermal energy (ISO 80000-5) and the diffusion area (item 10-72.2) for thermal neutrons
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-73.1 slowing-down length */
    attribute slowingDownLength : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.1 slowing-down length
         * symbol(s): `L_s`, `L_"sl"`
         * application domain: generic
         * name: SlowingDownLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the slowing down area `L_s^2` (item 10-72.1): `L_s = sqrt(L_s^2)`
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-73.2 diffusion length */
    attribute diffusionLength : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.2 diffusion length
         * symbol(s): `L`
         * application domain: atomic physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the diffusion area `L^2` (item 10-72.2): `L = sqrt(L^2)`
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-73.3 migration length */
    attribute migrationLength : LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 10-73.3 migration length
         * symbol(s): `M`
         * application domain: generic
         * name: MigrationLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the migration area `M^2` (item 10-72.3): `M = sqrt(M^2)`
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-74.1 neutron yield per fission */
    attribute neutronYieldPerFission : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.1 neutron yield per fission
         * symbol(s): `ν`
         * application domain: generic
         * name: NeutronYieldPerFission (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per fission event
         * remarks: None.
         */
    }
    /* ISO-80000-10 item 10-74.2 neutron yield per absorption */
    attribute neutronYieldPerAbsorption : CountValue :> scalarQuantities {
        doc
        /*
         * source: item 10-74.2 neutron yield per absorption
         * symbol(s): `η`
         * application domain: generic
         * name: NeutronYieldPerAbsorption (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: average number of fission neutrons, both prompt and delayed, emitted per neutron absorbed in a fissionable nuclide or in a nuclear fuel, as specified
         * remarks: `ν/η` is equal to the quotient of the macroscopic cross section for fission and that for absorption, both for neutrons in the fuel material.
         */
    }
    /* ISO-80000-10 item 10-75 fast fission factor */
    attribute def FastFissionFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-75 fast fission factor
         * symbol(s): `φ`
         * application domain: generic
         * name: FastFissionFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the mean number of neutrons produced by fission due to neutrons of all energies (ISO 80000-5) and the mean number of neutrons produced by fissions due to thermal neutrons only
         * remarks: The class of neutrons must be specified, e.g. thermal.
         */
        attribute :>> num : Real;
        attribute :>> mRef : FastFissionFactorUnit[1];
    }
    attribute fastFissionFactor : FastFissionFactorValue[*] nonunique :> scalarQuantities;
    attribute def FastFissionFactorUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-76 thermal utilization factor */
    attribute def ThermalUtilizationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-76 thermal utilization factor
         * symbol(s): `f`
         * application domain: generic
         * name: ThermalUtilizationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in an infinite medium, the quotient of the number of thermal neutrons absorbed in a fissionable nuclide or in a nuclear fuel, as specified, and the total number of thermal neutrons absorbed
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalUtilizationFactorUnit[1];
    }
    attribute thermalUtilizationFactor : ThermalUtilizationFactorValue[*] nonunique :> scalarQuantities;
    attribute def ThermalUtilizationFactorUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-77 non-leakage probability */
    attribute def NonLeakageProbabilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-77 non-leakage probability
         * symbol(s): `Λ`
         * application domain: generic
         * name: NonLeakageProbability
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: probability that a neutron will not escape from the reactor during the slowing-down process or while it diffuses as a thermal neutron
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : NonLeakageProbabilityUnit[1];
    }
    attribute nonLeakageProbability : NonLeakageProbabilityValue[*] nonunique :> scalarQuantities;
    attribute def NonLeakageProbabilityUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-78.1 multiplication factor */
    attribute def MultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.1 multiplication factor
         * symbol(s): `k`
         * application domain: generic
         * name: MultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the total number of fission or fission-dependent neutrons produced in the duration of a time interval and the total number of neutrons lost by absorption and leakage in that duration
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MultiplicationFactorUnit[1];
    }
    attribute multiplicationFactor : MultiplicationFactorValue[*] nonunique :> scalarQuantities;
    attribute def MultiplicationFactorUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-78.2 infinite multiplication factor */
    attribute def InfiniteMultiplicationFactorValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-78.2 infinite multiplication factor
         * symbol(s): `k_∞`
         * application domain: generic
         * name: InfiniteMultiplicationFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: multiplication factor (item 10-78.1) for an infinite medium or for an infinite repeating lattice
         * remarks: For a thermal reactor, `k_∞ = η*ε*p*f`
         */
        attribute :>> num : Real;
        attribute :>> mRef : InfiniteMultiplicationFactorUnit[1];
    }
    attribute infiniteMultiplicationFactor : InfiniteMultiplicationFactorValue[*] nonunique :> scalarQuantities;
    attribute def InfiniteMultiplicationFactorUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-79 reactor time constant */
    attribute reactorTimeConstant : DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 10-79 reactor time constant
         * symbol(s): `T`
         * application domain: generic
         * name: ReactorTimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (ISO 80000-3) required for the neutron fluence rate (item 10-44) in a reactor to change by the factor e when the fluence rate is rising or falling exponentially
         * remarks: Also called reactor period.
         */
    }
    /* ISO-80000-10 item 10-80.1 energy imparted */
    attribute energyImparted : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.1 energy imparted
         * symbol(s): `ε`
         * application domain: generic
         * name: EnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of all energy deposits in a given volume: `ε = sum_i ε_i` where the summation is performed over all energy (ISO 80000-5) deposits `ε_i` of interaction `i` in that volume
         * remarks: Energy imparted is a stochastic quantity. `ε_i` is given by: `ε_i = ε_(i n) - ε_"out" + Q` where `ε_(i n)` is the energy (ISO 80000-5) of the incident ionizing particle, excluding rest energy (item 10-3), `ε_"out"` is the sum of the energies (ISO 80000-5) of all ionizing particles leaving the interaction, excluding rest energy (item 10-3), and `Q` is the change in the rest energies (item 10-3) of the nucleus and of all particles involved in the interaction. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy. Stochastic quantities such as the energy imparted and the specific energy imparted (item 10-81.2) and their probability distributions have been introduced as they describe the discontinuous nature of the ionizing radiations as a determinant of radiochemical and radiobiological effects. In radiation applications involving large numbers of ionizing particles, e.g. in medicine, radiation protection and materials testing and processing, these fluctuations are adequately represented by the expectation values of the probability distributions. Non-stochastic quantities such as particle fluence (item 10-43), absorbed dose (item 10-81.1) and kerma (item 10-86.1) are based on these expectation values.
         */
    }
    /* ISO-80000-10 item 10-80.2 mean energy imparted */
    attribute meanEnergyImparted : EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 10-80.2 mean energy imparted
         * symbol(s): `bar(ε)`
         * application domain: generic
         * name: MeanEnergyImparted (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): eV, J, kg*m^2*s^-2
         * tensor order: 0
         * definition: expectation value of the energy imparted (item 10-80.1): `bar(ε) = R_"in" - R_"out" + sum Q` where `R_"in"` is the radiant energy (item 10-45) of all those charged and uncharged ionizing particles that enter the volume, `R_"out"` is the radiant energy of all those charged and uncharged ionizing particles that leave the volume, and `sum Q` is the sum of all changes of the rest energy (item 10-3) of nuclei and elementary particles that occur in that volume
         * remarks: Sometimes, it has been called the integral absorbed dose. `Q > 0` means decrease of rest energy; `Q < 0` means increase of rest energy.
         */
    }
    /* ISO-80000-10 item 10-81.1 absorbed dose */
    attribute def AbsorbedDoseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-81.1 absorbed dose
         * symbol(s): `D`
         * application domain: generic
         * name: AbsorbedDose
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: differential quotient of `bar(ε)` with respect to `m`, where `bar(ε)` is the mean energy (ISO 80000-5) imparted by ionizing radiation to matter of mass (ISO 80000-4) `m`: `D = (d bar(ε))/(dm)`
         * remarks: The gray is a special name for joule per kilogram, to be used as the coherent SI unit for absorbed dose. `1 "Gy" = 1 "J"/"kg"`. `bar(ε) = int D dm` where `dm` is the element of mass of the irradiated matter. In the limit of a small domain, the mean specific energy `bar(z) = (Δ bar(ε))/(Δ m)` is equal to the absorbed dose `D`. The absorbed dose can also be expressed in terms of the volume of the mass element by: `D = (d bar(ε))/(dm) = (d bar(ε))/(ρ dV)` where `ρ` is the mass density of the mass element. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed dose, `D`, is the quotient of `d bar(ε)` by dm, where `d bar(ε)` is the mean energy imparted by ionizing radiation to matter of mass `dm`: `D = (d bar(ε))/(dm)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AbsorbedDoseUnit[1];
    }
    attribute absorbedDose : AbsorbedDoseValue[*] nonunique :> scalarQuantities;
    attribute def AbsorbedDoseUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-81.2 specific energy imparted */
    attribute specificEnergyImparted : AbsorbedDoseValue :> scalarQuantities {
        doc
        /*
         * source: item 10-81.2 specific energy imparted
         * symbol(s): `z`
         * application domain: generic
         * name: SpecificEnergyImparted (specializes AbsorbedDose)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of the energy imparted `ε` (item 10-80.1) and the mass `m` (ISO 80000-4) of the matter in a given volume element: `z = ε / m`
         * remarks: `z` is a stochastic quantity. In the limit of a small domain, the mean specific energy `bar(z)` is equal to the absorbed dose `D`. The specific energy imparted can be due to one or more (energy-deposition) events.
         */
    }
    /* ISO-80000-10 item 10-82 quality factor */
    attribute def QualityFactorForIonizingRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-82 quality factor
         * symbol(s): `Q`
         * application domain: ionizing radiation
         * name: QualityFactor
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor in the calculation and measurement of dose equivalent (item 10-83.1), by which the absorbed dose (item 10-81.1) is to be weighted in order to account for different biological effectiveness of radiations, for radiation protection purposes
         * remarks: `Q` is determined by the linear energy transfer (item 10-85) for `Δ -> ∞` , `L_∞` (often denoted as `L` or LET), of charged particles passing through a small volume element at this point (the value of `L_∞` refers to water, not to tissue; the difference, however, is small). The relationship between `L` and `Q` is given in ICRP Publication 103 (ICRP, 2007).
         */
        attribute :>> num : Real;
        attribute :>> mRef : QualityFactorForIonizingRadiationUnit[1];
    }
    attribute qualityFactorForIonizingRadiation : QualityFactorForIonizingRadiationValue[*] nonunique :> scalarQuantities;
    attribute def QualityFactorForIonizingRadiationUnit :> DimensionOneUnit {
    }
    /* ISO-80000-10 item 10-83.1 dose equivalent */
    attribute def DoseEquivalentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-83.1 dose equivalent
         * symbol(s): `H`
         * application domain: generic
         * name: DoseEquivalent
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Sv, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: product of the absorbed dose `D` (item 10-81.1) to tissue at the point of interest and the quality factor `Q` (item 10-82) at that point: `H = DQ`
         * remarks: The sievert (Sv) is a special name for joule per kilogram, and is the coherent SI unit for dose equivalent. `1 "Sv" = 1 "J/kg"`. The dose equivalent at a point in tissue is given by: `H = int_0^∞ Q(L) D_L dL` where `D_L = (dD)/(dL)` is the distribution of `D` in `L` at the point of interest. See ICRP Publication 103 (ICRP, 2007). The quantities measured with radiation protection dosimeters are based on the definition `H = Q*D`. If various radiation qualities `i` have to be simultaneously accounted for, the definition is: `H = sum_i Q_i*D_i`. In ICRU 51 this quantity is denoted as "dose equivalent". In order to quantify the radiation exposition of the human body and to specify dose limits, use is made of a quantity defined in ICRP 103, the "equivalent dose to a tissue or organ": `H_T = w_T*sum_R w_R*D_{T,R}`. The weighting factors `w_T` for various tissues and organs `T` and `w_R` for various radiation qualities `R` have been numerically laid down in ICRP 103. `D_{T,R}` is the mean absorbed dose to tissue within a tissue or organ `T`, imparted by radiation with radiation quality `R`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DoseEquivalentUnit[1];
    }
    attribute doseEquivalent : DoseEquivalentValue[*] nonunique :> scalarQuantities;
    attribute def DoseEquivalentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-83.2 dose equivalent rate */
    attribute doseEquivalentRate : DoseEquivalentValue :> scalarQuantities {
        doc
        /*
         * source: item 10-83.2 dose equivalent rate
         * symbol(s): `dot(H)`
         * application domain: generic
         * name: DoseEquivalentRate (specializes DoseEquivalent)
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Sv/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of dose equivalent `H` (item 10-83.1) with respect to time (ISO 80000-3): `dot(H) = (dH)/(dt)`
         * remarks: `1 "Sv/s" = 1 "W/kg"`. See the remarks for item 10-83.1.
         */
    }
    /* ISO-80000-10 item 10-84 absorbed-dose rate */
    attribute def AbsorbedDoseRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-84 absorbed-dose rate
         * symbol(s): `dot(D)`
         * application domain: generic
         * name: AbsorbedDoseRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of the absorbed dose `D` (item 10-81.1) with respect to time (ISO 80000-3): `dot(D) = (dD)/(dt)`
         * remarks: `1 "Gy/s"  = 1 "W/kg"` See the remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed-does rate, `dot(D)` , is the quotient of `dD` by `dt`, where `dD` is the increment of absorbed does in the time interval `dt`: `dot(D) = (dD)/(dt)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AbsorbedDoseRateUnit[1];
    }
    attribute absorbedDoseRate : AbsorbedDoseRateValue[*] nonunique :> scalarQuantities;
    attribute def AbsorbedDoseRateUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-85 linear energy transfer */
    attribute def LinearEnergyTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-85 linear energy transfer
         * symbol(s): `L_Δ`
         * application domain: generic
         * name: LinearEnergyTransfer
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): eV/m, J/m, kg*m*s^-2
         * tensor order: 0
         * definition: quotient of the mean energy (ISO 80000-4) `dE_Δ` lost by the charged particles due to electronic interactions in traversing a distance (ISO 80000-3) `dl`, minus the mean sum of the kinetic energies in excess of `Δ` of all the electrons released by the charged particles and `dl`: `L_Δ = (dE_Δ)/(dl)`
         * remarks: This quantity is not completely defined unless `Δ` is specified, i.e. the maximum kinetic energy of secondary electrons whose energy is considered to be "locally deposited". `Δ` may be expressed in `"eV"`. Note that the abbreviation LET specifically refers to the quantity `L_∞` mentioned in the remark to 10-82.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearEnergyTransferUnit[1];
    }
    attribute linearEnergyTransfer : LinearEnergyTransferValue[*] nonunique :> scalarQuantities;
    attribute def LinearEnergyTransferUnit :> DerivedUnit {
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
    /* ISO-80000-10 item 10-86.1 kerma */
    attribute def KermaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.1 kerma
         * symbol(s): `K`
         * application domain: generic
         * name: Kerma
         * quantity dimension: L^2*T^-2
         * measurement unit(s): Gy, J/kg, m^2*s^-2
         * tensor order: 0
         * definition: for uncharged ionizing radiation, differential quotient of `E_(`tr) with respect to `m`, where `E_(`tr) is the mean sum of the initial kinetic energies (ISO 80000-4) of all the charged ionizing particles liberated in a mass (ISO 80000-4) `m` of a material: `K = (dE_tr)/(dm)`
         * remarks: `1 "Gy" = 1 "J/kg"` See the remarks for item 10-81.1. The name "kerma" is derived from Kinetic Energy Released in MAtter (or MAss or MAterial). The quantity `dE_(tr)` includes also the kinetic energy of the charged particles emitted in the decay of excited atoms, molecules, or nuclei. When the mass element `dm` consists of air the term air kerma is used. It can be convenient to refer to a value of air kerma in free space or at a point inside a material different from air, e.g. to the air kerma at a point inside a water phantom. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma, `K`, for ionizing uncharged particles, is the quotient of `dE_(tr)` by `dm`, where `dE_(tr)` is the mean sum of the initial kinetic energies of all the charged particles liberated in a mass `dm` of a material by the uncharged particles incident on `dm`: `K = (dE_(tr))/(dm)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : KermaUnit[1];
    }
    attribute kerma : KermaValue[*] nonunique :> scalarQuantities;
    attribute def KermaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-86.2 kerma rate */
    attribute def KermaRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-86.2 kerma rate
         * symbol(s): `dot(K)`
         * application domain: generic
         * name: KermaRate
         * quantity dimension: L^2*T^-3
         * measurement unit(s): Gy/s, W/kg, m^2*s^-3
         * tensor order: 0
         * definition: differential quotient of kerma (item 10-86.1) with respect to time (ISO 80000-3): `dot(K) = (dK)/(dt)`
         * remarks: `1 "Gy/s" = 1 "W/kg"`. See the Remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma rate, `dot(K)` , is the quotient of `dK` by `dt`, where `dK` is the increment of kerma in the time interval `dt`: `dot(K) = (dK)/(dt)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : KermaRateUnit[1];
    }
    attribute kermaRate : KermaRateValue[*] nonunique :> scalarQuantities;
    attribute def KermaRateUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-10 item 10-87 mass energy-transfer coefficient */
    attribute def MassEnergyTransferCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-87 mass energy-transfer coefficient
         * symbol(s): `μ_"tr"/ρ`
         * application domain: generic
         * name: MassEnergyTransferCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: for ionizing uncharged particles of a given type and energy, the differential quotient of `R_"tr"` with respect to `l`: `m_"tr"/ρ = 1/ρ 1/R (dR_"tr")/(dl)` where `R_"tr"` is the mean energy (ISO 80000-5) that is transferred to kinetic energy (ISO 80000-4) of charged particles by interactions of the uncharged particles of incident radiant energy `R` (item 10-45) in traversing a distance (ISO 80000-3) `l` in the material of density (ISO 80000-4) `ρ`, divided by `ρ` and `R`
         * remarks: `m_(tr)/ρ = (dot(K))/ψ` , where `dot(K)` is kerma rate (item 10-86.2) and `ψ` is energy fluence rate (item 10-47). The quantity: `μ_(en)/ρ = μ_(tr)/ρ(1-g)` where `g` is mean fraction of the kinetic energy of the liberated charged particles that is lost in radiative processes in the material, is called mass energy-absorption coefficient. The mass energy-absorption coefficient of a compound material depends on the stopping power of the material. Thus, its evaluation cannot, in principle, be reduced to a simple summation of the mass energy-absorption coefficient of the atomic constituents. Such a summation can provide an adequate approximation when the value of `g` is sufficiently small. In report 85a of the ICRU a definition with an equivalent meaning is given as: The mass energy-transfer coefficient, `μ_(tr)/ρ` , of a material, for uncharged particles of a given type and energy, is the quotient of `(dR_(tr))/R` by `ρ dl`, where `dR_(tr)` is the mean energy that is transferred to kinetic energy of charged particles by interactions of the uncharged particles of incident radiant energy `R` in traversing a distance `dl` in the material of density `ρ` : `μ_(tr)/ρ = 1/(ρ dl) (d R_(tr))/R`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassEnergyTransferCoefficientUnit[1];
    }
    attribute massEnergyTransferCoefficient : MassEnergyTransferCoefficientValue[*] nonunique :> scalarQuantities;
    attribute def MassEnergyTransferCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    /* ISO-80000-10 item 10-88 exposure */
    attribute def ExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-88 exposure
         * symbol(s): `X`
         * application domain: ionizing radiation
         * name: Exposure
         * quantity dimension: M^-1*T^1*I^1
         * measurement unit(s): C/kg, kg^-1*s*A
         * tensor order: 0
         * definition: for X- or gamma radiation the differential quotient of `q` with respect to `m`, where `q` is the absolute value of the mean total electric charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on an element of dry air with mass `m` (ISO 80000-4) are completely stopped in dry air: `X = (dq)/(dm)`
         * remarks: The ionization produced by electrons emitted in atomic or molecular relaxation is included in `dq`. The ionization due to photons emitted by radiative processes (i.e. bremsstrahlung and fluorescence photons) is not included in `dq`. This quantity should not be confused with the quantity photon exposure (ISO 80000-7), radiation exposure (ISO 80000-7), or the quantity luminous exposure (ISO 80000-7). It can be convenient to refer to a value of exposure in free space or at a point inside a material different from air, e.g. to the exposure at a point inside a water phantom. The exposure is related to the air kerma, `K_a`, (see item 10-86.1) by: `X = (e (1-g))/W K_a` , where `e` is the elementary charge (ISO 80000-1), `W` the average energy loss per elementary charge produced (item 10-60), and `g` is the fraction of the kinetic energy of liberated charged particles that is lost in radiative processes. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure, `X`, is the quotient of `dq` by `dm`, where `dq` is the absolute value of the mean total charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on a mass `dm` of dry air are completely stopped in dry air: `X = (dq)/(dm)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ExposureUnit[1];
    }
    attribute exposure : ExposureValue[*] nonunique :> scalarQuantities;
    attribute def ExposureUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }
    /* ISO-80000-10 item 10-89 exposure rate */
    attribute def ExposureRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 10-89 exposure rate
         * symbol(s): `dot(X)`
         * application domain: generic
         * name: ExposureRate
         * quantity dimension: M^-1*I^1
         * measurement unit(s): C/(kg*s), kg^-1*A
         * tensor order: 0
         * definition: differential quotient of the exposure `X` (item 10-88) with respect to time (ISO 80000-3): `dot(X) = (dX)/(dt)`
         * remarks: `1 "C/(kg s)" = 1 "A/kg"`. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure rate, `dot(X)` , is the quotient of `dX` by `dt`, where `dX` is the increment of exposure in the time interval `dt`: `dot(X) = (dX)/(dt)`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ExposureRateUnit[1];
    }
    attribute exposureRate : ExposureRateValue[*] nonunique :> scalarQuantities;
    attribute def ExposureRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, electricCurrentPF);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 801) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 801) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 815) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 840) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 840) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 874) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 874) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 919) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 919) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1017) (line 21) (column 20) (len 47)) (segments (segment 0 (token "ISQChemistryMolecular") (name "ISQChemistryMolecular") (separator none) (span (offset 1017) (line 21) (column 20) (len 21))) (segment 1 (token "DiffusionCoefficientUnit") (name "DiffusionCoefficientUnit") (separator colon-colon) (span (offset 1040) (line 21) (column 43) (len 24)))))
    (reference r5 (scope relative) (span (offset 1085) (line 22) (column 20) (len 48)) (segments (segment 0 (token "ISQChemistryMolecular") (name "ISQChemistryMolecular") (separator none) (span (offset 1085) (line 22) (column 20) (len 21))) (segment 1 (token "DiffusionCoefficientValue") (name "DiffusionCoefficientValue") (separator colon-colon) (span (offset 1108) (line 22) (column 43) (len 25)))))
    (reference r6 (scope relative) (span (offset 1154) (line 23) (column 20) (len 43)) (segments (segment 0 (token "ISQChemistryMolecular") (name "ISQChemistryMolecular") (separator none) (span (offset 1154) (line 23) (column 20) (len 21))) (segment 1 (token "diffusionCoefficient") (name "diffusionCoefficient") (separator colon-colon) (span (offset 1177) (line 23) (column 43) (len 20)))))
    (reference r7 (scope relative) (span (offset 1222) (line 24) (column 20) (len 40)) (segments (segment 0 (token "ISQElectromagnetism") (name "ISQElectromagnetism") (separator none) (span (offset 1222) (line 24) (column 20) (len 19))) (segment 1 (token "ElectricChargeValue") (name "ElectricChargeValue") (separator colon-colon) (span (offset 1243) (line 24) (column 41) (len 19)))))
    (reference r8 (scope relative) (span (offset 1283) (line 25) (column 20) (len 35)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1283) (line 25) (column 20) (len 12))) (segment 1 (token "AngularFrequencyValue") (name "AngularFrequencyValue") (separator colon-colon) (span (offset 1297) (line 25) (column 34) (len 21)))))
    (reference r9 (scope relative) (span (offset 1339) (line 26) (column 20) (len 23)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1339) (line 26) (column 20) (len 12))) (segment 1 (token "AreaValue") (name "AreaValue") (separator colon-colon) (span (offset 1353) (line 26) (column 34) (len 9)))))
    (reference r10 (scope relative) (span (offset 1383) (line 27) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 1383) (line 27) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1402) (line 27) (column 39) (len 11)))))
    (reference r11 (scope relative) (span (offset 2347) (line 45) (column 28) (len 12)) (segments (segment 0 (token "atomicNumber") (name "atomicNumber") (separator none) (span (offset 2347) (line 45) (column 28) (len 12)))))
    (reference r12 (scope relative) (span (offset 3601) (line 79) (column 26) (len 13)) (segments (segment 0 (token "nucleonNumber") (name "nucleonNumber") (separator none) (span (offset 3601) (line 79) (column 26) (len 13)))))
    (reference r13 (scope relative) (span (offset 4399) (line 97) (column 26) (len 8)) (segments (segment 0 (token "restMass") (name "restMass") (separator none) (span (offset 4399) (line 97) (column 26) (len 8)))))
    (reference r14 (scope relative) (span (offset 7824) (line 180) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 7824) (line 180) (column 40) (len 17)))))
    (reference r15 (scope relative) (span (offset 8722) (line 196) (column 32) (len 12)) (segments (segment 0 (token "chargeNumber") (name "chargeNumber") (separator none) (span (offset 8722) (line 196) (column 32) (len 12)))))
    (reference r16 (scope relative) (span (offset 9775) (line 215) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 9775) (line 215) (column 43) (len 19)))))
    (reference r17 (scope relative) (span (offset 10581) (line 228) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10581) (line 228) (column 28) (len 4)))))
    (reference r18 (scope relative) (span (offset 10576) (line 228) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 10576) (line 228) (column 23) (len 3)))))
    (reference r19 (scope relative) (span (offset 10615) (line 229) (column 29) (len 19)) (segments (segment 0 (token "RydbergConstantUnit") (name "RydbergConstantUnit") (separator none) (span (offset 10615) (line 229) (column 29) (len 19)))))
    (reference r20 (scope relative) (span (offset 10609) (line 229) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 10609) (line 229) (column 23) (len 4)))))
    (reference r21 (scope relative) (span (offset 10774) (line 234) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 10774) (line 234) (column 42) (len 11)))))
    (reference r22 (scope relative) (span (offset 10824) (line 235) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10824) (line 235) (column 37) (len 19)))))
    (reference r23 (scope relative) (span (offset 10853) (line 235) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10853) (line 235) (column 66) (len 8)))))
    (reference r24 (scope relative) (span (offset 10864) (line 235) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10864) (line 235) (column 77) (len 3)))))
    (reference r25 (scope relative) (span (offset 10868) (line 235) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 10868) (line 235) (column 81) (len 1)))))
    (reference r26 (scope relative) (span (offset 10875) (line 235) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10875) (line 235) (column 88) (len 8)))))
    (reference r27 (scope relative) (span (offset 10914) (line 236) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 10914) (line 236) (column 23) (len 17)))))
    (reference r28 (scope relative) (span (offset 10938) (line 236) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 10938) (line 236) (column 47) (len 20)))))
    (reference r29 (scope relative) (span (offset 10961) (line 236) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 10961) (line 236) (column 70) (len 8)))))
    (reference r30 (scope relative) (span (offset 11068) (line 240) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 11068) (line 240) (column 41) (len 19)))))
    (reference r31 (scope relative) (span (offset 11777) (line 253) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 11777) (line 253) (column 28) (len 4)))))
    (reference r32 (scope relative) (span (offset 11772) (line 253) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 11772) (line 253) (column 23) (len 3)))))
    (reference r33 (scope relative) (span (offset 11811) (line 254) (column 29) (len 17)) (segments (segment 0 (token "HartreeEnergyUnit") (name "HartreeEnergyUnit") (separator none) (span (offset 11811) (line 254) (column 29) (len 17)))))
    (reference r34 (scope relative) (span (offset 11805) (line 254) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 11805) (line 254) (column 23) (len 4)))))
    (reference r35 (scope relative) (span (offset 11962) (line 259) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 11962) (line 259) (column 40) (len 11)))))
    (reference r36 (scope relative) (span (offset 12012) (line 260) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12012) (line 260) (column 37) (len 19)))))
    (reference r37 (scope relative) (span (offset 12041) (line 260) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12041) (line 260) (column 66) (len 8)))))
    (reference r38 (scope relative) (span (offset 12052) (line 260) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12052) (line 260) (column 77) (len 3)))))
    (reference r39 (scope relative) (span (offset 12056) (line 260) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 12056) (line 260) (column 81) (len 1)))))
    (reference r40 (scope relative) (span (offset 12063) (line 260) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12063) (line 260) (column 88) (len 8)))))
    (reference r41 (scope relative) (span (offset 12113) (line 261) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12113) (line 261) (column 35) (len 19)))))
    (reference r42 (scope relative) (span (offset 12142) (line 261) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12142) (line 261) (column 64) (len 8)))))
    (reference r43 (scope relative) (span (offset 12153) (line 261) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12153) (line 261) (column 75) (len 3)))))
    (reference r44 (scope relative) (span (offset 12157) (line 261) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 12157) (line 261) (column 79) (len 1)))))
    (reference r45 (scope relative) (span (offset 12164) (line 261) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12164) (line 261) (column 86) (len 8)))))
    (reference r46 (scope relative) (span (offset 12218) (line 262) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12218) (line 262) (column 39) (len 19)))))
    (reference r47 (scope relative) (span (offset 12247) (line 262) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12247) (line 262) (column 68) (len 8)))))
    (reference r48 (scope relative) (span (offset 12258) (line 262) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12258) (line 262) (column 79) (len 3)))))
    (reference r49 (scope relative) (span (offset 12262) (line 262) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 12262) (line 262) (column 83) (len 1)))))
    (reference r50 (scope relative) (span (offset 12269) (line 262) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12269) (line 262) (column 90) (len 8)))))
    (reference r51 (scope relative) (span (offset 12308) (line 263) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 12308) (line 263) (column 23) (len 17)))))
    (reference r52 (scope relative) (span (offset 12332) (line 263) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 12332) (line 263) (column 47) (len 20)))))
    (reference r53 (scope relative) (span (offset 12356) (line 263) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 12356) (line 263) (column 71) (len 8)))))
    (reference r54 (scope relative) (span (offset 12366) (line 263) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 12366) (line 263) (column 81) (len 6)))))
    (reference r55 (scope relative) (span (offset 12374) (line 263) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 12374) (line 263) (column 89) (len 10)))))
    (reference r56 (scope relative) (span (offset 12501) (line 267) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 12501) (line 267) (column 48) (len 19)))))
    (reference r57 (scope relative) (span (offset 13481) (line 280) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 13481) (line 280) (column 28) (len 4)))))
    (reference r58 (scope relative) (span (offset 13476) (line 280) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 13476) (line 280) (column 23) (len 3)))))
    (reference r59 (scope relative) (span (offset 13515) (line 281) (column 29) (len 24)) (segments (segment 0 (token "MagneticDipoleMomentUnit") (name "MagneticDipoleMomentUnit") (separator none) (span (offset 13515) (line 281) (column 29) (len 24)))))
    (reference r60 (scope relative) (span (offset 13509) (line 281) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13509) (line 281) (column 23) (len 4)))))
    (reference r61 (scope relative) (span (offset 13694) (line 286) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 13694) (line 286) (column 47) (len 11)))))
    (reference r62 (scope relative) (span (offset 13744) (line 287) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13744) (line 287) (column 37) (len 19)))))
    (reference r63 (scope relative) (span (offset 13773) (line 287) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13773) (line 287) (column 66) (len 8)))))
    (reference r64 (scope relative) (span (offset 13784) (line 287) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13784) (line 287) (column 77) (len 3)))))
    (reference r65 (scope relative) (span (offset 13788) (line 287) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 13788) (line 287) (column 81) (len 1)))))
    (reference r66 (scope relative) (span (offset 13795) (line 287) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13795) (line 287) (column 88) (len 8)))))
    (reference r67 (scope relative) (span (offset 13856) (line 288) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13856) (line 288) (column 46) (len 19)))))
    (reference r68 (scope relative) (span (offset 13885) (line 288) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13885) (line 288) (column 75) (len 8)))))
    (reference r69 (scope relative) (span (offset 13896) (line 288) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13896) (line 288) (column 86) (len 3)))))
    (reference r70 (scope relative) (span (offset 13900) (line 288) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 13900) (line 288) (column 90) (len 1)))))
    (reference r71 (scope relative) (span (offset 13907) (line 288) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13907) (line 288) (column 97) (len 8)))))
    (reference r72 (scope relative) (span (offset 13945) (line 289) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 13945) (line 289) (column 23) (len 17)))))
    (reference r73 (scope relative) (span (offset 13969) (line 289) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 13969) (line 289) (column 47) (len 20)))))
    (reference r74 (scope relative) (span (offset 13993) (line 289) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 13993) (line 289) (column 71) (len 8)))))
    (reference r75 (scope relative) (span (offset 14003) (line 289) (column 81) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 14003) (line 289) (column 81) (len 17)))))
    (reference r76 (scope relative) (span (offset 14091) (line 292) (column 60) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 14091) (line 292) (column 60) (len 23)))))
    (reference r77 (scope relative) (span (offset 15072) (line 305) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 15072) (line 305) (column 23) (len 7)))))
    (reference r78 (scope relative) (span (offset 15117) (line 306) (column 29) (len 46)) (segments (segment 0 (token "CartesianMagneticDipoleMoment3dCoordinateFrame") (name "CartesianMagneticDipoleMoment3dCoordinateFrame") (separator none) (span (offset 15117) (line 306) (column 29) (len 46)))))
    (reference r79 (scope relative) (span (offset 15111) (line 306) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15111) (line 306) (column 23) (len 4)))))
    (reference r80 (scope relative) (span (offset 15356) (line 311) (column 69) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 15356) (line 311) (column 69) (len 19)))))
    (reference r81 (scope relative) (span (offset 15400) (line 312) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 15400) (line 312) (column 23) (len 7)))))
    (reference r82 (scope relative) (span (offset 15439) (line 313) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 15439) (line 313) (column 23) (len 12)))))
    (reference r83 (scope relative) (span (offset 15489) (line 314) (column 30) (len 24)) (segments (segment 0 (token "MagneticDipoleMomentUnit") (name "MagneticDipoleMomentUnit") (separator none) (span (offset 15489) (line 314) (column 30) (len 24)))))
    (reference r84 (scope relative) (span (offset 15482) (line 314) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 15482) (line 314) (column 23) (len 5)))))
    (reference r85 (scope relative) (span (offset 17358) (line 350) (column 32) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 17358) (line 350) (column 32) (len 19)))))
    (reference r86 (scope relative) (span (offset 17891) (line 363) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 17891) (line 363) (column 28) (len 4)))))
    (reference r87 (scope relative) (span (offset 17886) (line 363) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17886) (line 363) (column 23) (len 3)))))
    (reference r88 (scope relative) (span (offset 17925) (line 364) (column 29) (len 8)) (segments (segment 0 (token "SpinUnit") (name "SpinUnit") (separator none) (span (offset 17925) (line 364) (column 29) (len 8)))))
    (reference r89 (scope relative) (span (offset 17919) (line 364) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17919) (line 364) (column 23) (len 4)))))
    (reference r90 (scope relative) (span (offset 18040) (line 369) (column 31) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 18040) (line 369) (column 31) (len 11)))))
    (reference r91 (scope relative) (span (offset 18090) (line 370) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18090) (line 370) (column 37) (len 19)))))
    (reference r92 (scope relative) (span (offset 18119) (line 370) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18119) (line 370) (column 66) (len 8)))))
    (reference r93 (scope relative) (span (offset 18130) (line 370) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18130) (line 370) (column 77) (len 3)))))
    (reference r94 (scope relative) (span (offset 18134) (line 370) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 18134) (line 370) (column 81) (len 1)))))
    (reference r95 (scope relative) (span (offset 18141) (line 370) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18141) (line 370) (column 88) (len 8)))))
    (reference r96 (scope relative) (span (offset 18191) (line 371) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18191) (line 371) (column 35) (len 19)))))
    (reference r97 (scope relative) (span (offset 18220) (line 371) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18220) (line 371) (column 64) (len 8)))))
    (reference r98 (scope relative) (span (offset 18231) (line 371) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18231) (line 371) (column 75) (len 3)))))
    (reference r99 (scope relative) (span (offset 18235) (line 371) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 18235) (line 371) (column 79) (len 1)))))
    (reference r100 (scope relative) (span (offset 18242) (line 371) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18242) (line 371) (column 86) (len 8)))))
    (reference r101 (scope relative) (span (offset 18296) (line 372) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18296) (line 372) (column 39) (len 19)))))
    (reference r102 (scope relative) (span (offset 18325) (line 372) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18325) (line 372) (column 68) (len 8)))))
    (reference r103 (scope relative) (span (offset 18336) (line 372) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18336) (line 372) (column 79) (len 3)))))
    (reference r104 (scope relative) (span (offset 18340) (line 372) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 18340) (line 372) (column 83) (len 1)))))
    (reference r105 (scope relative) (span (offset 18347) (line 372) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18347) (line 372) (column 90) (len 8)))))
    (reference r106 (scope relative) (span (offset 18386) (line 373) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 18386) (line 373) (column 23) (len 17)))))
    (reference r107 (scope relative) (span (offset 18410) (line 373) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 18410) (line 373) (column 47) (len 20)))))
    (reference r108 (scope relative) (span (offset 18434) (line 373) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 18434) (line 373) (column 71) (len 8)))))
    (reference r109 (scope relative) (span (offset 18444) (line 373) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 18444) (line 373) (column 81) (len 6)))))
    (reference r110 (scope relative) (span (offset 18452) (line 373) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 18452) (line 373) (column 89) (len 10)))))
    (reference r111 (scope relative) (span (offset 18517) (line 376) (column 44) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 18517) (line 376) (column 44) (len 23)))))
    (reference r112 (scope relative) (span (offset 19051) (line 389) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 19051) (line 389) (column 23) (len 7)))))
    (reference r113 (scope relative) (span (offset 19096) (line 390) (column 29) (len 30)) (segments (segment 0 (token "CartesianSpin3dCoordinateFrame") (name "CartesianSpin3dCoordinateFrame") (separator none) (span (offset 19096) (line 390) (column 29) (len 30)))))
    (reference r114 (scope relative) (span (offset 19090) (line 390) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19090) (line 390) (column 23) (len 4)))))
    (reference r115 (scope relative) (span (offset 19271) (line 395) (column 53) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 19271) (line 395) (column 53) (len 19)))))
    (reference r116 (scope relative) (span (offset 19315) (line 396) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 19315) (line 396) (column 23) (len 7)))))
    (reference r117 (scope relative) (span (offset 19354) (line 397) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 19354) (line 397) (column 23) (len 12)))))
    (reference r118 (scope relative) (span (offset 19404) (line 398) (column 30) (len 8)) (segments (segment 0 (token "SpinUnit") (name "SpinUnit") (separator none) (span (offset 19404) (line 398) (column 30) (len 8)))))
    (reference r119 (scope relative) (span (offset 19397) (line 398) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 19397) (line 398) (column 23) (len 5)))))
    (reference r120 (scope relative) (span (offset 19528) (line 402) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 19528) (line 402) (column 48) (len 19)))))
    (reference r121 (scope relative) (span (offset 20547) (line 415) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 20547) (line 415) (column 28) (len 4)))))
    (reference r122 (scope relative) (span (offset 20542) (line 415) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 20542) (line 415) (column 23) (len 3)))))
    (reference r123 (scope relative) (span (offset 20581) (line 416) (column 29) (len 24)) (segments (segment 0 (token "TotalAngularMomentumUnit") (name "TotalAngularMomentumUnit") (separator none) (span (offset 20581) (line 416) (column 29) (len 24)))))
    (reference r124 (scope relative) (span (offset 20575) (line 416) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 20575) (line 416) (column 23) (len 4)))))
    (reference r125 (scope relative) (span (offset 20760) (line 421) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 20760) (line 421) (column 47) (len 11)))))
    (reference r126 (scope relative) (span (offset 20810) (line 422) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 20810) (line 422) (column 37) (len 19)))))
    (reference r127 (scope relative) (span (offset 20839) (line 422) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 20839) (line 422) (column 66) (len 8)))))
    (reference r128 (scope relative) (span (offset 20850) (line 422) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 20850) (line 422) (column 77) (len 3)))))
    (reference r129 (scope relative) (span (offset 20854) (line 422) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 20854) (line 422) (column 81) (len 1)))))
    (reference r130 (scope relative) (span (offset 20861) (line 422) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20861) (line 422) (column 88) (len 8)))))
    (reference r131 (scope relative) (span (offset 20911) (line 423) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 20911) (line 423) (column 35) (len 19)))))
    (reference r132 (scope relative) (span (offset 20940) (line 423) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 20940) (line 423) (column 64) (len 8)))))
    (reference r133 (scope relative) (span (offset 20951) (line 423) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 20951) (line 423) (column 75) (len 3)))))
    (reference r134 (scope relative) (span (offset 20955) (line 423) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 20955) (line 423) (column 79) (len 1)))))
    (reference r135 (scope relative) (span (offset 20962) (line 423) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20962) (line 423) (column 86) (len 8)))))
    (reference r136 (scope relative) (span (offset 21016) (line 424) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21016) (line 424) (column 39) (len 19)))))
    (reference r137 (scope relative) (span (offset 21045) (line 424) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21045) (line 424) (column 68) (len 8)))))
    (reference r138 (scope relative) (span (offset 21056) (line 424) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21056) (line 424) (column 79) (len 3)))))
    (reference r139 (scope relative) (span (offset 21060) (line 424) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 21060) (line 424) (column 83) (len 1)))))
    (reference r140 (scope relative) (span (offset 21067) (line 424) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21067) (line 424) (column 90) (len 8)))))
    (reference r141 (scope relative) (span (offset 21106) (line 425) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 21106) (line 425) (column 23) (len 17)))))
    (reference r142 (scope relative) (span (offset 21130) (line 425) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 21130) (line 425) (column 47) (len 20)))))
    (reference r143 (scope relative) (span (offset 21154) (line 425) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 21154) (line 425) (column 71) (len 8)))))
    (reference r144 (scope relative) (span (offset 21164) (line 425) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 21164) (line 425) (column 81) (len 6)))))
    (reference r145 (scope relative) (span (offset 21172) (line 425) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 21172) (line 425) (column 89) (len 10)))))
    (reference r146 (scope relative) (span (offset 21253) (line 428) (column 60) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 21253) (line 428) (column 60) (len 23)))))
    (reference r147 (scope relative) (span (offset 22273) (line 441) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 22273) (line 441) (column 23) (len 7)))))
    (reference r148 (scope relative) (span (offset 22318) (line 442) (column 29) (len 46)) (segments (segment 0 (token "CartesianTotalAngularMomentum3dCoordinateFrame") (name "CartesianTotalAngularMomentum3dCoordinateFrame") (separator none) (span (offset 22318) (line 442) (column 29) (len 46)))))
    (reference r149 (scope relative) (span (offset 22312) (line 442) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22312) (line 442) (column 23) (len 4)))))
    (reference r150 (scope relative) (span (offset 22557) (line 447) (column 69) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 22557) (line 447) (column 69) (len 19)))))
    (reference r151 (scope relative) (span (offset 22601) (line 448) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 22601) (line 448) (column 23) (len 7)))))
    (reference r152 (scope relative) (span (offset 22640) (line 449) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 22640) (line 449) (column 23) (len 12)))))
    (reference r153 (scope relative) (span (offset 22690) (line 450) (column 30) (len 24)) (segments (segment 0 (token "TotalAngularMomentumUnit") (name "TotalAngularMomentumUnit") (separator none) (span (offset 22690) (line 450) (column 30) (len 24)))))
    (reference r154 (scope relative) (span (offset 22683) (line 450) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 22683) (line 450) (column 23) (len 5)))))
    (reference r155 (scope relative) (span (offset 22871) (line 454) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 22871) (line 454) (column 45) (len 19)))))
    (reference r156 (scope relative) (span (offset 23816) (line 467) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23816) (line 467) (column 28) (len 4)))))
    (reference r157 (scope relative) (span (offset 23811) (line 467) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23811) (line 467) (column 23) (len 3)))))
    (reference r158 (scope relative) (span (offset 23850) (line 468) (column 29) (len 21)) (segments (segment 0 (token "GyromagneticRatioUnit") (name "GyromagneticRatioUnit") (separator none) (span (offset 23850) (line 468) (column 29) (len 21)))))
    (reference r159 (scope relative) (span (offset 23844) (line 468) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23844) (line 468) (column 23) (len 4)))))
    (reference r160 (scope relative) (span (offset 24017) (line 473) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 24017) (line 473) (column 44) (len 11)))))
    (reference r161 (scope relative) (span (offset 24065) (line 474) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24065) (line 474) (column 35) (len 19)))))
    (reference r162 (scope relative) (span (offset 24094) (line 474) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24094) (line 474) (column 64) (len 8)))))
    (reference r163 (scope relative) (span (offset 24105) (line 474) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24105) (line 474) (column 75) (len 3)))))
    (reference r164 (scope relative) (span (offset 24109) (line 474) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 24109) (line 474) (column 79) (len 1)))))
    (reference r165 (scope relative) (span (offset 24116) (line 474) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24116) (line 474) (column 86) (len 8)))))
    (reference r166 (scope relative) (span (offset 24171) (line 475) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24171) (line 475) (column 39) (len 19)))))
    (reference r167 (scope relative) (span (offset 24200) (line 475) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24200) (line 475) (column 68) (len 8)))))
    (reference r168 (scope relative) (span (offset 24211) (line 475) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24211) (line 475) (column 79) (len 3)))))
    (reference r169 (scope relative) (span (offset 24215) (line 475) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 24215) (line 475) (column 83) (len 1)))))
    (reference r170 (scope relative) (span (offset 24222) (line 475) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24222) (line 475) (column 90) (len 8)))))
    (reference r171 (scope relative) (span (offset 24283) (line 476) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24283) (line 476) (column 46) (len 19)))))
    (reference r172 (scope relative) (span (offset 24312) (line 476) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24312) (line 476) (column 75) (len 8)))))
    (reference r173 (scope relative) (span (offset 24323) (line 476) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24323) (line 476) (column 86) (len 3)))))
    (reference r174 (scope relative) (span (offset 24327) (line 476) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 24327) (line 476) (column 90) (len 1)))))
    (reference r175 (scope relative) (span (offset 24334) (line 476) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24334) (line 476) (column 97) (len 8)))))
    (reference r176 (scope relative) (span (offset 24372) (line 477) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 24372) (line 477) (column 23) (len 17)))))
    (reference r177 (scope relative) (span (offset 24396) (line 477) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 24396) (line 477) (column 47) (len 20)))))
    (reference r178 (scope relative) (span (offset 24420) (line 477) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 24420) (line 477) (column 71) (len 6)))))
    (reference r179 (scope relative) (span (offset 24428) (line 477) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 24428) (line 477) (column 79) (len 10)))))
    (reference r180 (scope relative) (span (offset 24440) (line 477) (column 91) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 24440) (line 477) (column 91) (len 17)))))
    (reference r181 (scope relative) (span (offset 24505) (line 480) (column 37) (len 21)) (segments (segment 0 (token "GyromagneticRatioUnit") (name "GyromagneticRatioUnit") (separator none) (span (offset 24505) (line 480) (column 37) (len 21)))))
    (reference r182 (scope relative) (span (offset 24565) (line 481) (column 38) (len 22)) (segments (segment 0 (token "GyromagneticRatioValue") (name "GyromagneticRatioValue") (separator none) (span (offset 24565) (line 481) (column 38) (len 22)))))
    (reference r183 (scope relative) (span (offset 24621) (line 482) (column 33) (len 17)) (segments (segment 0 (token "gyromagneticRatio") (name "gyromagneticRatio") (separator none) (span (offset 24621) (line 482) (column 33) (len 17)))))
    (reference r184 (scope relative) (span (offset 24683) (line 484) (column 43) (len 21)) (segments (segment 0 (token "GyromagneticRatioUnit") (name "GyromagneticRatioUnit") (separator none) (span (offset 24683) (line 484) (column 43) (len 21)))))
    (reference r185 (scope relative) (span (offset 24749) (line 485) (column 44) (len 22)) (segments (segment 0 (token "GyromagneticRatioValue") (name "GyromagneticRatioValue") (separator none) (span (offset 24749) (line 485) (column 44) (len 22)))))
    (reference r186 (scope relative) (span (offset 24811) (line 486) (column 39) (len 17)) (segments (segment 0 (token "gyromagneticRatio") (name "gyromagneticRatio") (separator none) (span (offset 24811) (line 486) (column 39) (len 17)))))
    (reference r187 (scope relative) (span (offset 25037) (line 489) (column 58) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 25037) (line 489) (column 58) (len 19)))))
    (reference r188 (scope relative) (span (offset 25852) (line 502) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 25852) (line 502) (column 28) (len 4)))))
    (reference r189 (scope relative) (span (offset 25847) (line 502) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 25847) (line 502) (column 23) (len 3)))))
    (reference r190 (scope relative) (span (offset 25886) (line 503) (column 29) (len 34)) (segments (segment 0 (token "GyromagneticRatioOfTheElectronUnit") (name "GyromagneticRatioOfTheElectronUnit") (separator none) (span (offset 25886) (line 503) (column 29) (len 34)))))
    (reference r191 (scope relative) (span (offset 25880) (line 503) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 25880) (line 503) (column 23) (len 4)))))
    (reference r192 (scope relative) (span (offset 26105) (line 508) (column 57) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 26105) (line 508) (column 57) (len 11)))))
    (reference r193 (scope relative) (span (offset 26153) (line 509) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26153) (line 509) (column 35) (len 19)))))
    (reference r194 (scope relative) (span (offset 26182) (line 509) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26182) (line 509) (column 64) (len 8)))))
    (reference r195 (scope relative) (span (offset 26193) (line 509) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26193) (line 509) (column 75) (len 3)))))
    (reference r196 (scope relative) (span (offset 26197) (line 509) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 26197) (line 509) (column 79) (len 1)))))
    (reference r197 (scope relative) (span (offset 26204) (line 509) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26204) (line 509) (column 86) (len 8)))))
    (reference r198 (scope relative) (span (offset 26259) (line 510) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26259) (line 510) (column 39) (len 19)))))
    (reference r199 (scope relative) (span (offset 26288) (line 510) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26288) (line 510) (column 68) (len 8)))))
    (reference r200 (scope relative) (span (offset 26299) (line 510) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26299) (line 510) (column 79) (len 3)))))
    (reference r201 (scope relative) (span (offset 26303) (line 510) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 26303) (line 510) (column 83) (len 1)))))
    (reference r202 (scope relative) (span (offset 26310) (line 510) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26310) (line 510) (column 90) (len 8)))))
    (reference r203 (scope relative) (span (offset 26371) (line 511) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26371) (line 511) (column 46) (len 19)))))
    (reference r204 (scope relative) (span (offset 26400) (line 511) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26400) (line 511) (column 75) (len 8)))))
    (reference r205 (scope relative) (span (offset 26411) (line 511) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26411) (line 511) (column 86) (len 3)))))
    (reference r206 (scope relative) (span (offset 26415) (line 511) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 26415) (line 511) (column 90) (len 1)))))
    (reference r207 (scope relative) (span (offset 26422) (line 511) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26422) (line 511) (column 97) (len 8)))))
    (reference r208 (scope relative) (span (offset 26460) (line 512) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 26460) (line 512) (column 23) (len 17)))))
    (reference r209 (scope relative) (span (offset 26484) (line 512) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 26484) (line 512) (column 47) (len 20)))))
    (reference r210 (scope relative) (span (offset 26508) (line 512) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 26508) (line 512) (column 71) (len 6)))))
    (reference r211 (scope relative) (span (offset 26516) (line 512) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 26516) (line 512) (column 79) (len 10)))))
    (reference r212 (scope relative) (span (offset 26528) (line 512) (column 91) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 26528) (line 512) (column 91) (len 17)))))
    (reference r213 (scope relative) (span (offset 26606) (line 515) (column 50) (len 34)) (segments (segment 0 (token "GyromagneticRatioOfTheElectronUnit") (name "GyromagneticRatioOfTheElectronUnit") (separator none) (span (offset 26606) (line 515) (column 50) (len 34)))))
    (reference r214 (scope relative) (span (offset 26692) (line 516) (column 51) (len 35)) (segments (segment 0 (token "GyromagneticRatioOfTheElectronValue") (name "GyromagneticRatioOfTheElectronValue") (separator none) (span (offset 26692) (line 516) (column 51) (len 35)))))
    (reference r215 (scope relative) (span (offset 26774) (line 517) (column 46) (len 30)) (segments (segment 0 (token "gyromagneticRatioOfTheElectron") (name "gyromagneticRatioOfTheElectron") (separator none) (span (offset 26774) (line 517) (column 46) (len 30)))))
    (reference r216 (scope relative) (span (offset 26862) (line 519) (column 56) (len 34)) (segments (segment 0 (token "GyromagneticRatioOfTheElectronUnit") (name "GyromagneticRatioOfTheElectronUnit") (separator none) (span (offset 26862) (line 519) (column 56) (len 34)))))
    (reference r217 (scope relative) (span (offset 26954) (line 520) (column 57) (len 35)) (segments (segment 0 (token "GyromagneticRatioOfTheElectronValue") (name "GyromagneticRatioOfTheElectronValue") (separator none) (span (offset 26954) (line 520) (column 57) (len 35)))))
    (reference r218 (scope relative) (span (offset 27042) (line 521) (column 52) (len 30)) (segments (segment 0 (token "gyromagneticRatioOfTheElectron") (name "gyromagneticRatioOfTheElectron") (separator none) (span (offset 27042) (line 521) (column 52) (len 30)))))
    (reference r219 (scope relative) (span (offset 27166) (line 524) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 27166) (line 524) (column 41) (len 17)))))
    (reference r220 (scope relative) (span (offset 35102) (line 653) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 35102) (line 653) (column 39) (len 17)))))
    (reference r221 (scope relative) (span (offset 36128) (line 669) (column 29) (len 11)) (segments (segment 0 (token "landeFactor") (name "landeFactor") (separator none) (span (offset 36128) (line 669) (column 29) (len 11)))))
    (reference r222 (scope relative) (span (offset 36278) (line 672) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 36278) (line 672) (column 61) (len 17)))))
    (reference r223 (scope relative) (span (offset 38132) (line 705) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 38132) (line 705) (column 43) (len 19)))))
    (reference r224 (scope relative) (span (offset 38574) (line 718) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 38574) (line 718) (column 28) (len 4)))))
    (reference r225 (scope relative) (span (offset 38569) (line 718) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 38569) (line 718) (column 23) (len 3)))))
    (reference r226 (scope relative) (span (offset 38608) (line 719) (column 29) (len 19)) (segments (segment 0 (token "LarmorFrequencyUnit") (name "LarmorFrequencyUnit") (separator none) (span (offset 38608) (line 719) (column 29) (len 19)))))
    (reference r227 (scope relative) (span (offset 38602) (line 719) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 38602) (line 719) (column 23) (len 4)))))
    (reference r228 (scope relative) (span (offset 38767) (line 724) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 38767) (line 724) (column 42) (len 11)))))
    (reference r229 (scope relative) (span (offset 38819) (line 725) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 38819) (line 725) (column 39) (len 19)))))
    (reference r230 (scope relative) (span (offset 38848) (line 725) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 38848) (line 725) (column 68) (len 8)))))
    (reference r231 (scope relative) (span (offset 38859) (line 725) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 38859) (line 725) (column 79) (len 3)))))
    (reference r232 (scope relative) (span (offset 38863) (line 725) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 38863) (line 725) (column 83) (len 1)))))
    (reference r233 (scope relative) (span (offset 38870) (line 725) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 38870) (line 725) (column 90) (len 8)))))
    (reference r234 (scope relative) (span (offset 38909) (line 726) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 38909) (line 726) (column 23) (len 17)))))
    (reference r235 (scope relative) (span (offset 38933) (line 726) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 38933) (line 726) (column 47) (len 20)))))
    (reference r236 (scope relative) (span (offset 38956) (line 726) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 38956) (line 726) (column 70) (len 10)))))
    (reference r237 (scope relative) (span (offset 41573) (line 777) (column 28) (len 10)) (segments (segment 0 (token "gyroradius") (name "gyroradius") (separator none) (span (offset 41573) (line 777) (column 28) (len 10)))))
    (reference r238 (scope relative) (span (offset 41696) (line 780) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 41696) (line 780) (column 51) (len 19)))))
    (reference r239 (scope relative) (span (offset 42592) (line 793) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 42592) (line 793) (column 28) (len 4)))))
    (reference r240 (scope relative) (span (offset 42587) (line 793) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 42587) (line 793) (column 23) (len 3)))))
    (reference r241 (scope relative) (span (offset 42626) (line 794) (column 29) (len 27)) (segments (segment 0 (token "NuclearQuadrupoleMomentUnit") (name "NuclearQuadrupoleMomentUnit") (separator none) (span (offset 42626) (line 794) (column 29) (len 27)))))
    (reference r242 (scope relative) (span (offset 42620) (line 794) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 42620) (line 794) (column 23) (len 4)))))
    (reference r243 (scope relative) (span (offset 42817) (line 799) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 42817) (line 799) (column 50) (len 11)))))
    (reference r244 (scope relative) (span (offset 42867) (line 800) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 42867) (line 800) (column 37) (len 19)))))
    (reference r245 (scope relative) (span (offset 42896) (line 800) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 42896) (line 800) (column 66) (len 8)))))
    (reference r246 (scope relative) (span (offset 42907) (line 800) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 42907) (line 800) (column 77) (len 3)))))
    (reference r247 (scope relative) (span (offset 42911) (line 800) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 42911) (line 800) (column 81) (len 1)))))
    (reference r248 (scope relative) (span (offset 42918) (line 800) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 42918) (line 800) (column 88) (len 8)))))
    (reference r249 (scope relative) (span (offset 42956) (line 801) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 42956) (line 801) (column 23) (len 17)))))
    (reference r250 (scope relative) (span (offset 42980) (line 801) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 42980) (line 801) (column 47) (len 20)))))
    (reference r251 (scope relative) (span (offset 43003) (line 801) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 43003) (line 801) (column 70) (len 8)))))
    (reference r252 (scope relative) (span (offset 47764) (line 885) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 47764) (line 885) (column 46) (len 17)))))
    (reference r253 (scope relative) (span (offset 48522) (line 902) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 48522) (line 902) (column 46) (len 17)))))
    (reference r254 (scope relative) (span (offset 49269) (line 919) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 49269) (line 919) (column 43) (len 17)))))
    (reference r255 (scope relative) (span (offset 49992) (line 936) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 49992) (line 936) (column 43) (len 17)))))
    (reference r256 (scope relative) (span (offset 50731) (line 953) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 50731) (line 953) (column 41) (len 19)))))
    (reference r257 (scope relative) (span (offset 51574) (line 966) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 51574) (line 966) (column 28) (len 4)))))
    (reference r258 (scope relative) (span (offset 51569) (line 966) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 51569) (line 966) (column 23) (len 3)))))
    (reference r259 (scope relative) (span (offset 51608) (line 967) (column 29) (len 17)) (segments (segment 0 (token "DecayConstantUnit") (name "DecayConstantUnit") (separator none) (span (offset 51608) (line 967) (column 29) (len 17)))))
    (reference r260 (scope relative) (span (offset 51602) (line 967) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 51602) (line 967) (column 23) (len 4)))))
    (reference r261 (scope relative) (span (offset 51759) (line 972) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 51759) (line 972) (column 40) (len 11)))))
    (reference r262 (scope relative) (span (offset 51811) (line 973) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51811) (line 973) (column 39) (len 19)))))
    (reference r263 (scope relative) (span (offset 51840) (line 973) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51840) (line 973) (column 68) (len 8)))))
    (reference r264 (scope relative) (span (offset 51851) (line 973) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51851) (line 973) (column 79) (len 3)))))
    (reference r265 (scope relative) (span (offset 51855) (line 973) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 51855) (line 973) (column 83) (len 1)))))
    (reference r266 (scope relative) (span (offset 51862) (line 973) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51862) (line 973) (column 90) (len 8)))))
    (reference r267 (scope relative) (span (offset 51901) (line 974) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 51901) (line 974) (column 23) (len 17)))))
    (reference r268 (scope relative) (span (offset 51925) (line 974) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 51925) (line 974) (column 47) (len 20)))))
    (reference r269 (scope relative) (span (offset 51948) (line 974) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 51948) (line 974) (column 70) (len 10)))))
    (reference r270 (scope relative) (span (offset 52010) (line 977) (column 42) (len 17)) (segments (segment 0 (token "DecayConstantUnit") (name "DecayConstantUnit") (separator none) (span (offset 52010) (line 977) (column 42) (len 17)))))
    (reference r271 (scope relative) (span (offset 52071) (line 978) (column 43) (len 18)) (segments (segment 0 (token "DecayConstantValue") (name "DecayConstantValue") (separator none) (span (offset 52071) (line 978) (column 43) (len 18)))))
    (reference r272 (scope relative) (span (offset 52128) (line 979) (column 38) (len 13)) (segments (segment 0 (token "decayConstant") (name "decayConstant") (separator none) (span (offset 52128) (line 979) (column 38) (len 13)))))
    (reference r273 (scope relative) (span (offset 52985) (line 997) (column 28) (len 18)) (segments (segment 0 (token "meanDurationOfLife") (name "meanDurationOfLife") (separator none) (span (offset 52985) (line 997) (column 28) (len 18)))))
    (reference r274 (scope relative) (span (offset 54277) (line 1016) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 54277) (line 1016) (column 43) (len 19)))))
    (reference r275 (scope relative) (span (offset 55491) (line 1029) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 55491) (line 1029) (column 28) (len 4)))))
    (reference r276 (scope relative) (span (offset 55486) (line 1029) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 55486) (line 1029) (column 23) (len 3)))))
    (reference r277 (scope relative) (span (offset 55525) (line 1030) (column 29) (len 19)) (segments (segment 0 (token "NuclearActivityUnit") (name "NuclearActivityUnit") (separator none) (span (offset 55525) (line 1030) (column 29) (len 19)))))
    (reference r278 (scope relative) (span (offset 55519) (line 1030) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 55519) (line 1030) (column 23) (len 4)))))
    (reference r279 (scope relative) (span (offset 55684) (line 1035) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 55684) (line 1035) (column 42) (len 11)))))
    (reference r280 (scope relative) (span (offset 55736) (line 1036) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 55736) (line 1036) (column 39) (len 19)))))
    (reference r281 (scope relative) (span (offset 55765) (line 1036) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 55765) (line 1036) (column 68) (len 8)))))
    (reference r282 (scope relative) (span (offset 55776) (line 1036) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 55776) (line 1036) (column 79) (len 3)))))
    (reference r283 (scope relative) (span (offset 55780) (line 1036) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 55780) (line 1036) (column 83) (len 1)))))
    (reference r284 (scope relative) (span (offset 55787) (line 1036) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 55787) (line 1036) (column 90) (len 8)))))
    (reference r285 (scope relative) (span (offset 55826) (line 1037) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 55826) (line 1037) (column 23) (len 17)))))
    (reference r286 (scope relative) (span (offset 55850) (line 1037) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 55850) (line 1037) (column 47) (len 20)))))
    (reference r287 (scope relative) (span (offset 55873) (line 1037) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 55873) (line 1037) (column 70) (len 10)))))
    (reference r288 (scope relative) (span (offset 56006) (line 1041) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 56006) (line 1041) (column 44) (len 19)))))
    (reference r289 (scope relative) (span (offset 56532) (line 1054) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 56532) (line 1054) (column 28) (len 4)))))
    (reference r290 (scope relative) (span (offset 56527) (line 1054) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 56527) (line 1054) (column 23) (len 3)))))
    (reference r291 (scope relative) (span (offset 56566) (line 1055) (column 29) (len 20)) (segments (segment 0 (token "SpecificActivityUnit") (name "SpecificActivityUnit") (separator none) (span (offset 56566) (line 1055) (column 29) (len 20)))))
    (reference r292 (scope relative) (span (offset 56560) (line 1055) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 56560) (line 1055) (column 23) (len 4)))))
    (reference r293 (scope relative) (span (offset 56729) (line 1060) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 56729) (line 1060) (column 43) (len 11)))))
    (reference r294 (scope relative) (span (offset 56777) (line 1061) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56777) (line 1061) (column 35) (len 19)))))
    (reference r295 (scope relative) (span (offset 56806) (line 1061) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56806) (line 1061) (column 64) (len 8)))))
    (reference r296 (scope relative) (span (offset 56817) (line 1061) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56817) (line 1061) (column 75) (len 3)))))
    (reference r297 (scope relative) (span (offset 56821) (line 1061) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 56821) (line 1061) (column 79) (len 1)))))
    (reference r298 (scope relative) (span (offset 56828) (line 1061) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56828) (line 1061) (column 86) (len 8)))))
    (reference r299 (scope relative) (span (offset 56883) (line 1062) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56883) (line 1062) (column 39) (len 19)))))
    (reference r300 (scope relative) (span (offset 56912) (line 1062) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56912) (line 1062) (column 68) (len 8)))))
    (reference r301 (scope relative) (span (offset 56923) (line 1062) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56923) (line 1062) (column 79) (len 3)))))
    (reference r302 (scope relative) (span (offset 56927) (line 1062) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 56927) (line 1062) (column 83) (len 1)))))
    (reference r303 (scope relative) (span (offset 56934) (line 1062) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56934) (line 1062) (column 90) (len 8)))))
    (reference r304 (scope relative) (span (offset 56973) (line 1063) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 56973) (line 1063) (column 23) (len 17)))))
    (reference r305 (scope relative) (span (offset 56997) (line 1063) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 56997) (line 1063) (column 47) (len 20)))))
    (reference r306 (scope relative) (span (offset 57021) (line 1063) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 57021) (line 1063) (column 71) (len 6)))))
    (reference r307 (scope relative) (span (offset 57029) (line 1063) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 57029) (line 1063) (column 79) (len 10)))))
    (reference r308 (scope relative) (span (offset 57084) (line 1066) (column 34) (len 20)) (segments (segment 0 (token "SpecificActivityUnit") (name "SpecificActivityUnit") (separator none) (span (offset 57084) (line 1066) (column 34) (len 20)))))
    (reference r309 (scope relative) (span (offset 57140) (line 1067) (column 35) (len 21)) (segments (segment 0 (token "SpecificActivityValue") (name "SpecificActivityValue") (separator none) (span (offset 57140) (line 1067) (column 35) (len 21)))))
    (reference r310 (scope relative) (span (offset 57192) (line 1068) (column 30) (len 16)) (segments (segment 0 (token "specificActivity") (name "specificActivity") (separator none) (span (offset 57192) (line 1068) (column 30) (len 16)))))
    (reference r311 (scope relative) (span (offset 57346) (line 1071) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 57346) (line 1071) (column 43) (len 19)))))
    (reference r312 (scope relative) (span (offset 57897) (line 1084) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 57897) (line 1084) (column 28) (len 4)))))
    (reference r313 (scope relative) (span (offset 57892) (line 1084) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 57892) (line 1084) (column 23) (len 3)))))
    (reference r314 (scope relative) (span (offset 57931) (line 1085) (column 29) (len 19)) (segments (segment 0 (token "ActivityDensityUnit") (name "ActivityDensityUnit") (separator none) (span (offset 57931) (line 1085) (column 29) (len 19)))))
    (reference r315 (scope relative) (span (offset 57925) (line 1085) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 57925) (line 1085) (column 23) (len 4)))))
    (reference r316 (scope relative) (span (offset 58090) (line 1090) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 58090) (line 1090) (column 42) (len 11)))))
    (reference r317 (scope relative) (span (offset 58140) (line 1091) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 58140) (line 1091) (column 37) (len 19)))))
    (reference r318 (scope relative) (span (offset 58169) (line 1091) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 58169) (line 1091) (column 66) (len 8)))))
    (reference r319 (scope relative) (span (offset 58180) (line 1091) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 58180) (line 1091) (column 77) (len 3)))))
    (reference r320 (scope relative) (span (offset 58184) (line 1091) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 58184) (line 1091) (column 81) (len 1)))))
    (reference r321 (scope relative) (span (offset 58191) (line 1091) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 58191) (line 1091) (column 88) (len 8)))))
    (reference r322 (scope relative) (span (offset 58246) (line 1092) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 58246) (line 1092) (column 39) (len 19)))))
    (reference r323 (scope relative) (span (offset 58275) (line 1092) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 58275) (line 1092) (column 68) (len 8)))))
    (reference r324 (scope relative) (span (offset 58286) (line 1092) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 58286) (line 1092) (column 79) (len 3)))))
    (reference r325 (scope relative) (span (offset 58290) (line 1092) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 58290) (line 1092) (column 83) (len 1)))))
    (reference r326 (scope relative) (span (offset 58297) (line 1092) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 58297) (line 1092) (column 90) (len 8)))))
    (reference r327 (scope relative) (span (offset 58336) (line 1093) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 58336) (line 1093) (column 23) (len 17)))))
    (reference r328 (scope relative) (span (offset 58360) (line 1093) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 58360) (line 1093) (column 47) (len 20)))))
    (reference r329 (scope relative) (span (offset 58384) (line 1093) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 58384) (line 1093) (column 71) (len 8)))))
    (reference r330 (scope relative) (span (offset 58394) (line 1093) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 58394) (line 1093) (column 81) (len 10)))))
    (reference r331 (scope relative) (span (offset 58450) (line 1096) (column 35) (len 19)) (segments (segment 0 (token "ActivityDensityUnit") (name "ActivityDensityUnit") (separator none) (span (offset 58450) (line 1096) (column 35) (len 19)))))
    (reference r332 (scope relative) (span (offset 58506) (line 1097) (column 36) (len 20)) (segments (segment 0 (token "ActivityDensityValue") (name "ActivityDensityValue") (separator none) (span (offset 58506) (line 1097) (column 36) (len 20)))))
    (reference r333 (scope relative) (span (offset 58558) (line 1098) (column 31) (len 15)) (segments (segment 0 (token "activityDensity") (name "activityDensity") (separator none) (span (offset 58558) (line 1098) (column 31) (len 15)))))
    (reference r334 (scope relative) (span (offset 58616) (line 1100) (column 41) (len 19)) (segments (segment 0 (token "ActivityDensityUnit") (name "ActivityDensityUnit") (separator none) (span (offset 58616) (line 1100) (column 41) (len 19)))))
    (reference r335 (scope relative) (span (offset 58678) (line 1101) (column 42) (len 20)) (segments (segment 0 (token "ActivityDensityValue") (name "ActivityDensityValue") (separator none) (span (offset 58678) (line 1101) (column 42) (len 20)))))
    (reference r336 (scope relative) (span (offset 58736) (line 1102) (column 37) (len 15)) (segments (segment 0 (token "activityDensity") (name "activityDensity") (separator none) (span (offset 58736) (line 1102) (column 37) (len 15)))))
    (reference r337 (scope relative) (span (offset 58862) (line 1105) (column 50) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 58862) (line 1105) (column 50) (len 19)))))
    (reference r338 (scope relative) (span (offset 59533) (line 1118) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 59533) (line 1118) (column 28) (len 4)))))
    (reference r339 (scope relative) (span (offset 59528) (line 1118) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 59528) (line 1118) (column 23) (len 3)))))
    (reference r340 (scope relative) (span (offset 59567) (line 1119) (column 29) (len 26)) (segments (segment 0 (token "SurfaceActivityDensityUnit") (name "SurfaceActivityDensityUnit") (separator none) (span (offset 59567) (line 1119) (column 29) (len 26)))))
    (reference r341 (scope relative) (span (offset 59561) (line 1119) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 59561) (line 1119) (column 23) (len 4)))))
    (reference r342 (scope relative) (span (offset 59754) (line 1124) (column 49) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 59754) (line 1124) (column 49) (len 11)))))
    (reference r343 (scope relative) (span (offset 59804) (line 1125) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 59804) (line 1125) (column 37) (len 19)))))
    (reference r344 (scope relative) (span (offset 59833) (line 1125) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59833) (line 1125) (column 66) (len 8)))))
    (reference r345 (scope relative) (span (offset 59844) (line 1125) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59844) (line 1125) (column 77) (len 3)))))
    (reference r346 (scope relative) (span (offset 59848) (line 1125) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 59848) (line 1125) (column 81) (len 1)))))
    (reference r347 (scope relative) (span (offset 59855) (line 1125) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59855) (line 1125) (column 88) (len 8)))))
    (reference r348 (scope relative) (span (offset 59910) (line 1126) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 59910) (line 1126) (column 39) (len 19)))))
    (reference r349 (scope relative) (span (offset 59939) (line 1126) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59939) (line 1126) (column 68) (len 8)))))
    (reference r350 (scope relative) (span (offset 59950) (line 1126) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59950) (line 1126) (column 79) (len 3)))))
    (reference r351 (scope relative) (span (offset 59954) (line 1126) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 59954) (line 1126) (column 83) (len 1)))))
    (reference r352 (scope relative) (span (offset 59961) (line 1126) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59961) (line 1126) (column 90) (len 8)))))
    (reference r353 (scope relative) (span (offset 60000) (line 1127) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 60000) (line 1127) (column 23) (len 17)))))
    (reference r354 (scope relative) (span (offset 60024) (line 1127) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 60024) (line 1127) (column 47) (len 20)))))
    (reference r355 (scope relative) (span (offset 60048) (line 1127) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 60048) (line 1127) (column 71) (len 8)))))
    (reference r356 (scope relative) (span (offset 60058) (line 1127) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 60058) (line 1127) (column 81) (len 10)))))
    (reference r357 (scope relative) (span (offset 63547) (line 1195) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 63547) (line 1195) (column 52) (len 17)))))
    (reference r358 (scope relative) (span (offset 64642) (line 1212) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 64642) (line 1212) (column 48) (len 19)))))
    (reference r359 (scope relative) (span (offset 65365) (line 1225) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 65365) (line 1225) (column 28) (len 4)))))
    (reference r360 (scope relative) (span (offset 65360) (line 1225) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 65360) (line 1225) (column 23) (len 3)))))
    (reference r361 (scope relative) (span (offset 65399) (line 1226) (column 29) (len 24)) (segments (segment 0 (token "ParticleEmissionRateUnit") (name "ParticleEmissionRateUnit") (separator none) (span (offset 65399) (line 1226) (column 29) (len 24)))))
    (reference r362 (scope relative) (span (offset 65393) (line 1226) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 65393) (line 1226) (column 23) (len 4)))))
    (reference r363 (scope relative) (span (offset 65578) (line 1231) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 65578) (line 1231) (column 47) (len 11)))))
    (reference r364 (scope relative) (span (offset 65630) (line 1232) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 65630) (line 1232) (column 39) (len 19)))))
    (reference r365 (scope relative) (span (offset 65659) (line 1232) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 65659) (line 1232) (column 68) (len 8)))))
    (reference r366 (scope relative) (span (offset 65670) (line 1232) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 65670) (line 1232) (column 79) (len 3)))))
    (reference r367 (scope relative) (span (offset 65674) (line 1232) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 65674) (line 1232) (column 83) (len 1)))))
    (reference r368 (scope relative) (span (offset 65681) (line 1232) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 65681) (line 1232) (column 90) (len 8)))))
    (reference r369 (scope relative) (span (offset 65720) (line 1233) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 65720) (line 1233) (column 23) (len 17)))))
    (reference r370 (scope relative) (span (offset 65744) (line 1233) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 65744) (line 1233) (column 47) (len 20)))))
    (reference r371 (scope relative) (span (offset 65767) (line 1233) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 65767) (line 1233) (column 70) (len 10)))))
    (reference r372 (scope relative) (span (offset 69259) (line 1301) (column 63) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 69259) (line 1301) (column 63) (len 19)))))
    (reference r373 (scope relative) (span (offset 70096) (line 1314) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 70096) (line 1314) (column 28) (len 4)))))
    (reference r374 (scope relative) (span (offset 70091) (line 1314) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 70091) (line 1314) (column 23) (len 3)))))
    (reference r375 (scope relative) (span (offset 70130) (line 1315) (column 29) (len 39)) (segments (segment 0 (token "DirectionDistributionOfCrossSectionUnit") (name "DirectionDistributionOfCrossSectionUnit") (separator none) (span (offset 70130) (line 1315) (column 29) (len 39)))))
    (reference r376 (scope relative) (span (offset 70124) (line 1315) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 70124) (line 1315) (column 23) (len 4)))))
    (reference r377 (scope relative) (span (offset 70369) (line 1320) (column 62) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 70369) (line 1320) (column 62) (len 11)))))
    (reference r378 (scope relative) (span (offset 70419) (line 1321) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 70419) (line 1321) (column 37) (len 19)))))
    (reference r379 (scope relative) (span (offset 70448) (line 1321) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 70448) (line 1321) (column 66) (len 8)))))
    (reference r380 (scope relative) (span (offset 70459) (line 1321) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 70459) (line 1321) (column 77) (len 3)))))
    (reference r381 (scope relative) (span (offset 70463) (line 1321) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 70463) (line 1321) (column 81) (len 1)))))
    (reference r382 (scope relative) (span (offset 70470) (line 1321) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 70470) (line 1321) (column 88) (len 8)))))
    (reference r383 (scope relative) (span (offset 70508) (line 1322) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 70508) (line 1322) (column 23) (len 17)))))
    (reference r384 (scope relative) (span (offset 70532) (line 1322) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 70532) (line 1322) (column 47) (len 20)))))
    (reference r385 (scope relative) (span (offset 70555) (line 1322) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 70555) (line 1322) (column 70) (len 8)))))
    (reference r386 (scope relative) (span (offset 70704) (line 1326) (column 60) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 70704) (line 1326) (column 60) (len 19)))))
    (reference r387 (scope relative) (span (offset 71384) (line 1339) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 71384) (line 1339) (column 28) (len 4)))))
    (reference r388 (scope relative) (span (offset 71379) (line 1339) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 71379) (line 1339) (column 23) (len 3)))))
    (reference r389 (scope relative) (span (offset 71418) (line 1340) (column 29) (len 36)) (segments (segment 0 (token "EnergyDistributionOfCrossSectionUnit") (name "EnergyDistributionOfCrossSectionUnit") (separator none) (span (offset 71418) (line 1340) (column 29) (len 36)))))
    (reference r390 (scope relative) (span (offset 71412) (line 1340) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 71412) (line 1340) (column 23) (len 4)))))
    (reference r391 (scope relative) (span (offset 71645) (line 1345) (column 59) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 71645) (line 1345) (column 59) (len 11)))))
    (reference r392 (scope relative) (span (offset 71693) (line 1346) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 71693) (line 1346) (column 35) (len 19)))))
    (reference r393 (scope relative) (span (offset 71722) (line 1346) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 71722) (line 1346) (column 64) (len 8)))))
    (reference r394 (scope relative) (span (offset 71733) (line 1346) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 71733) (line 1346) (column 75) (len 3)))))
    (reference r395 (scope relative) (span (offset 71737) (line 1346) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 71737) (line 1346) (column 79) (len 1)))))
    (reference r396 (scope relative) (span (offset 71744) (line 1346) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 71744) (line 1346) (column 86) (len 8)))))
    (reference r397 (scope relative) (span (offset 71799) (line 1347) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 71799) (line 1347) (column 39) (len 19)))))
    (reference r398 (scope relative) (span (offset 71828) (line 1347) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 71828) (line 1347) (column 68) (len 8)))))
    (reference r399 (scope relative) (span (offset 71839) (line 1347) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 71839) (line 1347) (column 79) (len 3)))))
    (reference r400 (scope relative) (span (offset 71843) (line 1347) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 71843) (line 1347) (column 83) (len 1)))))
    (reference r401 (scope relative) (span (offset 71850) (line 1347) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 71850) (line 1347) (column 90) (len 8)))))
    (reference r402 (scope relative) (span (offset 71888) (line 1348) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 71888) (line 1348) (column 23) (len 17)))))
    (reference r403 (scope relative) (span (offset 71912) (line 1348) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 71912) (line 1348) (column 47) (len 20)))))
    (reference r404 (scope relative) (span (offset 71936) (line 1348) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 71936) (line 1348) (column 71) (len 6)))))
    (reference r405 (scope relative) (span (offset 71944) (line 1348) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 71944) (line 1348) (column 79) (len 10)))))
    (reference r406 (scope relative) (span (offset 72122) (line 1352) (column 72) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 72122) (line 1352) (column 72) (len 19)))))
    (reference r407 (scope relative) (span (offset 72901) (line 1365) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 72901) (line 1365) (column 28) (len 4)))))
    (reference r408 (scope relative) (span (offset 72896) (line 1365) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 72896) (line 1365) (column 23) (len 3)))))
    (reference r409 (scope relative) (span (offset 72935) (line 1366) (column 29) (len 48)) (segments (segment 0 (token "DirectionAndEnergyDistributionOfCrossSectionUnit") (name "DirectionAndEnergyDistributionOfCrossSectionUnit") (separator none) (span (offset 72935) (line 1366) (column 29) (len 48)))))
    (reference r410 (scope relative) (span (offset 72929) (line 1366) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 72929) (line 1366) (column 23) (len 4)))))
    (reference r411 (scope relative) (span (offset 73210) (line 1371) (column 71) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 73210) (line 1371) (column 71) (len 11)))))
    (reference r412 (scope relative) (span (offset 73258) (line 1372) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 73258) (line 1372) (column 35) (len 19)))))
    (reference r413 (scope relative) (span (offset 73287) (line 1372) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 73287) (line 1372) (column 64) (len 8)))))
    (reference r414 (scope relative) (span (offset 73298) (line 1372) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 73298) (line 1372) (column 75) (len 3)))))
    (reference r415 (scope relative) (span (offset 73302) (line 1372) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 73302) (line 1372) (column 79) (len 1)))))
    (reference r416 (scope relative) (span (offset 73309) (line 1372) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 73309) (line 1372) (column 86) (len 8)))))
    (reference r417 (scope relative) (span (offset 73364) (line 1373) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 73364) (line 1373) (column 39) (len 19)))))
    (reference r418 (scope relative) (span (offset 73393) (line 1373) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 73393) (line 1373) (column 68) (len 8)))))
    (reference r419 (scope relative) (span (offset 73404) (line 1373) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 73404) (line 1373) (column 79) (len 3)))))
    (reference r420 (scope relative) (span (offset 73408) (line 1373) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 73408) (line 1373) (column 83) (len 1)))))
    (reference r421 (scope relative) (span (offset 73415) (line 1373) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 73415) (line 1373) (column 90) (len 8)))))
    (reference r422 (scope relative) (span (offset 73453) (line 1374) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 73453) (line 1374) (column 23) (len 17)))))
    (reference r423 (scope relative) (span (offset 73477) (line 1374) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 73477) (line 1374) (column 47) (len 20)))))
    (reference r424 (scope relative) (span (offset 73501) (line 1374) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 73501) (line 1374) (column 71) (len 6)))))
    (reference r425 (scope relative) (span (offset 73509) (line 1374) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 73509) (line 1374) (column 79) (len 10)))))
    (reference r426 (scope relative) (span (offset 73662) (line 1378) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 73662) (line 1378) (column 47) (len 19)))))
    (reference r427 (scope relative) (span (offset 74330) (line 1391) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 74330) (line 1391) (column 28) (len 4)))))
    (reference r428 (scope relative) (span (offset 74325) (line 1391) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 74325) (line 1391) (column 23) (len 3)))))
    (reference r429 (scope relative) (span (offset 74364) (line 1392) (column 29) (len 23)) (segments (segment 0 (token "VolumicCrossSectionUnit") (name "VolumicCrossSectionUnit") (separator none) (span (offset 74364) (line 1392) (column 29) (len 23)))))
    (reference r430 (scope relative) (span (offset 74358) (line 1392) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 74358) (line 1392) (column 23) (len 4)))))
    (reference r431 (scope relative) (span (offset 74539) (line 1397) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 74539) (line 1397) (column 46) (len 11)))))
    (reference r432 (scope relative) (span (offset 74589) (line 1398) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 74589) (line 1398) (column 37) (len 19)))))
    (reference r433 (scope relative) (span (offset 74618) (line 1398) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 74618) (line 1398) (column 66) (len 8)))))
    (reference r434 (scope relative) (span (offset 74629) (line 1398) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 74629) (line 1398) (column 77) (len 3)))))
    (reference r435 (scope relative) (span (offset 74633) (line 1398) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 74633) (line 1398) (column 81) (len 1)))))
    (reference r436 (scope relative) (span (offset 74640) (line 1398) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 74640) (line 1398) (column 88) (len 8)))))
    (reference r437 (scope relative) (span (offset 74679) (line 1399) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 74679) (line 1399) (column 23) (len 17)))))
    (reference r438 (scope relative) (span (offset 74703) (line 1399) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 74703) (line 1399) (column 47) (len 20)))))
    (reference r439 (scope relative) (span (offset 74726) (line 1399) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 74726) (line 1399) (column 70) (len 8)))))
    (reference r440 (scope relative) (span (offset 74787) (line 1402) (column 43) (len 23)) (segments (segment 0 (token "VolumicCrossSectionUnit") (name "VolumicCrossSectionUnit") (separator none) (span (offset 74787) (line 1402) (column 43) (len 23)))))
    (reference r441 (scope relative) (span (offset 74855) (line 1403) (column 44) (len 24)) (segments (segment 0 (token "VolumicCrossSectionValue") (name "VolumicCrossSectionValue") (separator none) (span (offset 74855) (line 1403) (column 44) (len 24)))))
    (reference r442 (scope relative) (span (offset 74919) (line 1404) (column 39) (len 19)) (segments (segment 0 (token "volumicCrossSection") (name "volumicCrossSection") (separator none) (span (offset 74919) (line 1404) (column 39) (len 19)))))
    (reference r443 (scope relative) (span (offset 75089) (line 1407) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 75089) (line 1407) (column 52) (len 19)))))
    (reference r444 (scope relative) (span (offset 75722) (line 1420) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 75722) (line 1420) (column 28) (len 4)))))
    (reference r445 (scope relative) (span (offset 75717) (line 1420) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 75717) (line 1420) (column 23) (len 3)))))
    (reference r446 (scope relative) (span (offset 75756) (line 1421) (column 29) (len 28)) (segments (segment 0 (token "VolumicTotalCrossSectionUnit") (name "VolumicTotalCrossSectionUnit") (separator none) (span (offset 75756) (line 1421) (column 29) (len 28)))))
    (reference r447 (scope relative) (span (offset 75750) (line 1421) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 75750) (line 1421) (column 23) (len 4)))))
    (reference r448 (scope relative) (span (offset 75951) (line 1426) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 75951) (line 1426) (column 51) (len 11)))))
    (reference r449 (scope relative) (span (offset 76001) (line 1427) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 76001) (line 1427) (column 37) (len 19)))))
    (reference r450 (scope relative) (span (offset 76030) (line 1427) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 76030) (line 1427) (column 66) (len 8)))))
    (reference r451 (scope relative) (span (offset 76041) (line 1427) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 76041) (line 1427) (column 77) (len 3)))))
    (reference r452 (scope relative) (span (offset 76045) (line 1427) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 76045) (line 1427) (column 81) (len 1)))))
    (reference r453 (scope relative) (span (offset 76052) (line 1427) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 76052) (line 1427) (column 88) (len 8)))))
    (reference r454 (scope relative) (span (offset 76091) (line 1428) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 76091) (line 1428) (column 23) (len 17)))))
    (reference r455 (scope relative) (span (offset 76115) (line 1428) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 76115) (line 1428) (column 47) (len 20)))))
    (reference r456 (scope relative) (span (offset 76138) (line 1428) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 76138) (line 1428) (column 70) (len 8)))))
    (reference r457 (scope relative) (span (offset 76204) (line 1431) (column 48) (len 28)) (segments (segment 0 (token "VolumicTotalCrossSectionUnit") (name "VolumicTotalCrossSectionUnit") (separator none) (span (offset 76204) (line 1431) (column 48) (len 28)))))
    (reference r458 (scope relative) (span (offset 76282) (line 1432) (column 49) (len 29)) (segments (segment 0 (token "VolumicTotalCrossSectionValue") (name "VolumicTotalCrossSectionValue") (separator none) (span (offset 76282) (line 1432) (column 49) (len 29)))))
    (reference r459 (scope relative) (span (offset 76356) (line 1433) (column 44) (len 24)) (segments (segment 0 (token "volumicTotalCrossSection") (name "volumicTotalCrossSection") (separator none) (span (offset 76356) (line 1433) (column 44) (len 24)))))
    (reference r460 (scope relative) (span (offset 76476) (line 1436) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 76476) (line 1436) (column 43) (len 19)))))
    (reference r461 (scope relative) (span (offset 77702) (line 1449) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 77702) (line 1449) (column 28) (len 4)))))
    (reference r462 (scope relative) (span (offset 77697) (line 1449) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 77697) (line 1449) (column 23) (len 3)))))
    (reference r463 (scope relative) (span (offset 77736) (line 1450) (column 29) (len 19)) (segments (segment 0 (token "ParticleFluenceUnit") (name "ParticleFluenceUnit") (separator none) (span (offset 77736) (line 1450) (column 29) (len 19)))))
    (reference r464 (scope relative) (span (offset 77730) (line 1450) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 77730) (line 1450) (column 23) (len 4)))))
    (reference r465 (scope relative) (span (offset 77895) (line 1455) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 77895) (line 1455) (column 42) (len 11)))))
    (reference r466 (scope relative) (span (offset 77945) (line 1456) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 77945) (line 1456) (column 37) (len 19)))))
    (reference r467 (scope relative) (span (offset 77974) (line 1456) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 77974) (line 1456) (column 66) (len 8)))))
    (reference r468 (scope relative) (span (offset 77985) (line 1456) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 77985) (line 1456) (column 77) (len 3)))))
    (reference r469 (scope relative) (span (offset 77989) (line 1456) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 77989) (line 1456) (column 81) (len 1)))))
    (reference r470 (scope relative) (span (offset 77996) (line 1456) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 77996) (line 1456) (column 88) (len 8)))))
    (reference r471 (scope relative) (span (offset 78035) (line 1457) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 78035) (line 1457) (column 23) (len 17)))))
    (reference r472 (scope relative) (span (offset 78059) (line 1457) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 78059) (line 1457) (column 47) (len 20)))))
    (reference r473 (scope relative) (span (offset 78082) (line 1457) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 78082) (line 1457) (column 70) (len 8)))))
    (reference r474 (scope relative) (span (offset 78203) (line 1461) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 78203) (line 1461) (column 47) (len 19)))))
    (reference r475 (scope relative) (span (offset 79564) (line 1474) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 79564) (line 1474) (column 28) (len 4)))))
    (reference r476 (scope relative) (span (offset 79559) (line 1474) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 79559) (line 1474) (column 23) (len 3)))))
    (reference r477 (scope relative) (span (offset 79598) (line 1475) (column 29) (len 23)) (segments (segment 0 (token "ParticleFluenceRateUnit") (name "ParticleFluenceRateUnit") (separator none) (span (offset 79598) (line 1475) (column 29) (len 23)))))
    (reference r478 (scope relative) (span (offset 79592) (line 1475) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 79592) (line 1475) (column 23) (len 4)))))
    (reference r479 (scope relative) (span (offset 79773) (line 1480) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 79773) (line 1480) (column 46) (len 11)))))
    (reference r480 (scope relative) (span (offset 79823) (line 1481) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 79823) (line 1481) (column 37) (len 19)))))
    (reference r481 (scope relative) (span (offset 79852) (line 1481) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 79852) (line 1481) (column 66) (len 8)))))
    (reference r482 (scope relative) (span (offset 79863) (line 1481) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 79863) (line 1481) (column 77) (len 3)))))
    (reference r483 (scope relative) (span (offset 79867) (line 1481) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 79867) (line 1481) (column 81) (len 1)))))
    (reference r484 (scope relative) (span (offset 79874) (line 1481) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 79874) (line 1481) (column 88) (len 8)))))
    (reference r485 (scope relative) (span (offset 79929) (line 1482) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 79929) (line 1482) (column 39) (len 19)))))
    (reference r486 (scope relative) (span (offset 79958) (line 1482) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 79958) (line 1482) (column 68) (len 8)))))
    (reference r487 (scope relative) (span (offset 79969) (line 1482) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 79969) (line 1482) (column 79) (len 3)))))
    (reference r488 (scope relative) (span (offset 79973) (line 1482) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 79973) (line 1482) (column 83) (len 1)))))
    (reference r489 (scope relative) (span (offset 79980) (line 1482) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 79980) (line 1482) (column 90) (len 8)))))
    (reference r490 (scope relative) (span (offset 80019) (line 1483) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 80019) (line 1483) (column 23) (len 17)))))
    (reference r491 (scope relative) (span (offset 80043) (line 1483) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 80043) (line 1483) (column 47) (len 20)))))
    (reference r492 (scope relative) (span (offset 80067) (line 1483) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 80067) (line 1483) (column 71) (len 8)))))
    (reference r493 (scope relative) (span (offset 80077) (line 1483) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 80077) (line 1483) (column 81) (len 10)))))
    (reference r494 (scope relative) (span (offset 81355) (line 1503) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 81355) (line 1503) (column 41) (len 19)))))
    (reference r495 (scope relative) (span (offset 82173) (line 1516) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 82173) (line 1516) (column 28) (len 4)))))
    (reference r496 (scope relative) (span (offset 82168) (line 1516) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 82168) (line 1516) (column 23) (len 3)))))
    (reference r497 (scope relative) (span (offset 82207) (line 1517) (column 29) (len 17)) (segments (segment 0 (token "EnergyFluenceUnit") (name "EnergyFluenceUnit") (separator none) (span (offset 82207) (line 1517) (column 29) (len 17)))))
    (reference r498 (scope relative) (span (offset 82201) (line 1517) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 82201) (line 1517) (column 23) (len 4)))))
    (reference r499 (scope relative) (span (offset 82358) (line 1522) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 82358) (line 1522) (column 40) (len 11)))))
    (reference r500 (scope relative) (span (offset 82406) (line 1523) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 82406) (line 1523) (column 35) (len 19)))))
    (reference r501 (scope relative) (span (offset 82435) (line 1523) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 82435) (line 1523) (column 64) (len 8)))))
    (reference r502 (scope relative) (span (offset 82446) (line 1523) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 82446) (line 1523) (column 75) (len 3)))))
    (reference r503 (scope relative) (span (offset 82450) (line 1523) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 82450) (line 1523) (column 79) (len 1)))))
    (reference r504 (scope relative) (span (offset 82457) (line 1523) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 82457) (line 1523) (column 86) (len 8)))))
    (reference r505 (scope relative) (span (offset 82511) (line 1524) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 82511) (line 1524) (column 39) (len 19)))))
    (reference r506 (scope relative) (span (offset 82540) (line 1524) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 82540) (line 1524) (column 68) (len 8)))))
    (reference r507 (scope relative) (span (offset 82551) (line 1524) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 82551) (line 1524) (column 79) (len 3)))))
    (reference r508 (scope relative) (span (offset 82555) (line 1524) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 82555) (line 1524) (column 83) (len 1)))))
    (reference r509 (scope relative) (span (offset 82562) (line 1524) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 82562) (line 1524) (column 90) (len 8)))))
    (reference r510 (scope relative) (span (offset 82601) (line 1525) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 82601) (line 1525) (column 23) (len 17)))))
    (reference r511 (scope relative) (span (offset 82625) (line 1525) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 82625) (line 1525) (column 47) (len 20)))))
    (reference r512 (scope relative) (span (offset 82649) (line 1525) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 82649) (line 1525) (column 71) (len 6)))))
    (reference r513 (scope relative) (span (offset 82657) (line 1525) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 82657) (line 1525) (column 79) (len 10)))))
    (reference r514 (scope relative) (span (offset 82777) (line 1529) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 82777) (line 1529) (column 45) (len 19)))))
    (reference r515 (scope relative) (span (offset 83573) (line 1542) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 83573) (line 1542) (column 28) (len 4)))))
    (reference r516 (scope relative) (span (offset 83568) (line 1542) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 83568) (line 1542) (column 23) (len 3)))))
    (reference r517 (scope relative) (span (offset 83607) (line 1543) (column 29) (len 21)) (segments (segment 0 (token "EnergyFluenceRateUnit") (name "EnergyFluenceRateUnit") (separator none) (span (offset 83607) (line 1543) (column 29) (len 21)))))
    (reference r518 (scope relative) (span (offset 83601) (line 1543) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 83601) (line 1543) (column 23) (len 4)))))
    (reference r519 (scope relative) (span (offset 83774) (line 1548) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 83774) (line 1548) (column 44) (len 11)))))
    (reference r520 (scope relative) (span (offset 83822) (line 1549) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 83822) (line 1549) (column 35) (len 19)))))
    (reference r521 (scope relative) (span (offset 83851) (line 1549) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 83851) (line 1549) (column 64) (len 8)))))
    (reference r522 (scope relative) (span (offset 83862) (line 1549) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 83862) (line 1549) (column 75) (len 3)))))
    (reference r523 (scope relative) (span (offset 83866) (line 1549) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 83866) (line 1549) (column 79) (len 1)))))
    (reference r524 (scope relative) (span (offset 83873) (line 1549) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 83873) (line 1549) (column 86) (len 8)))))
    (reference r525 (scope relative) (span (offset 83927) (line 1550) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 83927) (line 1550) (column 39) (len 19)))))
    (reference r526 (scope relative) (span (offset 83956) (line 1550) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 83956) (line 1550) (column 68) (len 8)))))
    (reference r527 (scope relative) (span (offset 83967) (line 1550) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 83967) (line 1550) (column 79) (len 3)))))
    (reference r528 (scope relative) (span (offset 83971) (line 1550) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 83971) (line 1550) (column 83) (len 1)))))
    (reference r529 (scope relative) (span (offset 83978) (line 1550) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 83978) (line 1550) (column 90) (len 8)))))
    (reference r530 (scope relative) (span (offset 84017) (line 1551) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 84017) (line 1551) (column 23) (len 17)))))
    (reference r531 (scope relative) (span (offset 84041) (line 1551) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 84041) (line 1551) (column 47) (len 20)))))
    (reference r532 (scope relative) (span (offset 84065) (line 1551) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 84065) (line 1551) (column 71) (len 6)))))
    (reference r533 (scope relative) (span (offset 84073) (line 1551) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 84073) (line 1551) (column 79) (len 10)))))
    (reference r534 (scope relative) (span (offset 84203) (line 1555) (column 50) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 84203) (line 1555) (column 50) (len 19)))))
    (reference r535 (scope relative) (span (offset 85443) (line 1568) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 85443) (line 1568) (column 28) (len 4)))))
    (reference r536 (scope relative) (span (offset 85438) (line 1568) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 85438) (line 1568) (column 23) (len 3)))))
    (reference r537 (scope relative) (span (offset 85477) (line 1569) (column 29) (len 26)) (segments (segment 0 (token "ParticleCurrentDensityUnit") (name "ParticleCurrentDensityUnit") (separator none) (span (offset 85477) (line 1569) (column 29) (len 26)))))
    (reference r538 (scope relative) (span (offset 85471) (line 1569) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 85471) (line 1569) (column 23) (len 4)))))
    (reference r539 (scope relative) (span (offset 85664) (line 1574) (column 49) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 85664) (line 1574) (column 49) (len 11)))))
    (reference r540 (scope relative) (span (offset 85714) (line 1575) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 85714) (line 1575) (column 37) (len 19)))))
    (reference r541 (scope relative) (span (offset 85743) (line 1575) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 85743) (line 1575) (column 66) (len 8)))))
    (reference r542 (scope relative) (span (offset 85754) (line 1575) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 85754) (line 1575) (column 77) (len 3)))))
    (reference r543 (scope relative) (span (offset 85758) (line 1575) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 85758) (line 1575) (column 81) (len 1)))))
    (reference r544 (scope relative) (span (offset 85765) (line 1575) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 85765) (line 1575) (column 88) (len 8)))))
    (reference r545 (scope relative) (span (offset 85820) (line 1576) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 85820) (line 1576) (column 39) (len 19)))))
    (reference r546 (scope relative) (span (offset 85849) (line 1576) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 85849) (line 1576) (column 68) (len 8)))))
    (reference r547 (scope relative) (span (offset 85860) (line 1576) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 85860) (line 1576) (column 79) (len 3)))))
    (reference r548 (scope relative) (span (offset 85864) (line 1576) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 85864) (line 1576) (column 83) (len 1)))))
    (reference r549 (scope relative) (span (offset 85871) (line 1576) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 85871) (line 1576) (column 90) (len 8)))))
    (reference r550 (scope relative) (span (offset 85910) (line 1577) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 85910) (line 1577) (column 23) (len 17)))))
    (reference r551 (scope relative) (span (offset 85934) (line 1577) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 85934) (line 1577) (column 47) (len 20)))))
    (reference r552 (scope relative) (span (offset 85958) (line 1577) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 85958) (line 1577) (column 71) (len 8)))))
    (reference r553 (scope relative) (span (offset 85968) (line 1577) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 85968) (line 1577) (column 81) (len 10)))))
    (reference r554 (scope relative) (span (offset 86051) (line 1580) (column 62) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 86051) (line 1580) (column 62) (len 23)))))
    (reference r555 (scope relative) (span (offset 87297) (line 1593) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 87297) (line 1593) (column 23) (len 7)))))
    (reference r556 (scope relative) (span (offset 87342) (line 1594) (column 29) (len 48)) (segments (segment 0 (token "CartesianParticleCurrentDensity3dCoordinateFrame") (name "CartesianParticleCurrentDensity3dCoordinateFrame") (separator none) (span (offset 87342) (line 1594) (column 29) (len 48)))))
    (reference r557 (scope relative) (span (offset 87336) (line 1594) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 87336) (line 1594) (column 23) (len 4)))))
    (reference r558 (scope relative) (span (offset 87589) (line 1599) (column 71) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 87589) (line 1599) (column 71) (len 19)))))
    (reference r559 (scope relative) (span (offset 87633) (line 1600) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 87633) (line 1600) (column 23) (len 7)))))
    (reference r560 (scope relative) (span (offset 87672) (line 1601) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 87672) (line 1601) (column 23) (len 12)))))
    (reference r561 (scope relative) (span (offset 87722) (line 1602) (column 30) (len 26)) (segments (segment 0 (token "ParticleCurrentDensityUnit") (name "ParticleCurrentDensityUnit") (separator none) (span (offset 87722) (line 1602) (column 30) (len 26)))))
    (reference r562 (scope relative) (span (offset 87715) (line 1602) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 87715) (line 1602) (column 23) (len 5)))))
    (reference r563 (scope relative) (span (offset 87900) (line 1606) (column 76) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 87900) (line 1606) (column 76) (len 19)))))
    (reference r564 (scope relative) (span (offset 89508) (line 1619) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 89508) (line 1619) (column 28) (len 4)))))
    (reference r565 (scope relative) (span (offset 89503) (line 1619) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 89503) (line 1619) (column 23) (len 3)))))
    (reference r566 (scope relative) (span (offset 89542) (line 1620) (column 29) (len 52)) (segments (segment 0 (token "LinearAttenuationCoefficientForIonizingRadiationUnit") (name "LinearAttenuationCoefficientForIonizingRadiationUnit") (separator none) (span (offset 89542) (line 1620) (column 29) (len 52)))))
    (reference r567 (scope relative) (span (offset 89536) (line 1620) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 89536) (line 1620) (column 23) (len 4)))))
    (reference r568 (scope relative) (span (offset 89833) (line 1625) (column 75) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 89833) (line 1625) (column 75) (len 11)))))
    (reference r569 (scope relative) (span (offset 89883) (line 1626) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 89883) (line 1626) (column 37) (len 19)))))
    (reference r570 (scope relative) (span (offset 89912) (line 1626) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 89912) (line 1626) (column 66) (len 8)))))
    (reference r571 (scope relative) (span (offset 89923) (line 1626) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 89923) (line 1626) (column 77) (len 3)))))
    (reference r572 (scope relative) (span (offset 89927) (line 1626) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 89927) (line 1626) (column 81) (len 1)))))
    (reference r573 (scope relative) (span (offset 89934) (line 1626) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 89934) (line 1626) (column 88) (len 8)))))
    (reference r574 (scope relative) (span (offset 89973) (line 1627) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 89973) (line 1627) (column 23) (len 17)))))
    (reference r575 (scope relative) (span (offset 89997) (line 1627) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 89997) (line 1627) (column 47) (len 20)))))
    (reference r576 (scope relative) (span (offset 90020) (line 1627) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 90020) (line 1627) (column 70) (len 8)))))
    (reference r577 (scope relative) (span (offset 90175) (line 1631) (column 74) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 90175) (line 1631) (column 74) (len 19)))))
    (reference r578 (scope relative) (span (offset 90734) (line 1644) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 90734) (line 1644) (column 28) (len 4)))))
    (reference r579 (scope relative) (span (offset 90729) (line 1644) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 90729) (line 1644) (column 23) (len 3)))))
    (reference r580 (scope relative) (span (offset 90768) (line 1645) (column 29) (len 50)) (segments (segment 0 (token "MassAttenuationCoefficientForIonizingRadiationUnit") (name "MassAttenuationCoefficientForIonizingRadiationUnit") (separator none) (span (offset 90768) (line 1645) (column 29) (len 50)))))
    (reference r581 (scope relative) (span (offset 90762) (line 1645) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 90762) (line 1645) (column 23) (len 4)))))
    (reference r582 (scope relative) (span (offset 91051) (line 1650) (column 73) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 91051) (line 1650) (column 73) (len 11)))))
    (reference r583 (scope relative) (span (offset 91101) (line 1651) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 91101) (line 1651) (column 37) (len 19)))))
    (reference r584 (scope relative) (span (offset 91130) (line 1651) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 91130) (line 1651) (column 66) (len 8)))))
    (reference r585 (scope relative) (span (offset 91141) (line 1651) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 91141) (line 1651) (column 77) (len 3)))))
    (reference r586 (scope relative) (span (offset 91145) (line 1651) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 91145) (line 1651) (column 81) (len 1)))))
    (reference r587 (scope relative) (span (offset 91152) (line 1651) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 91152) (line 1651) (column 88) (len 8)))))
    (reference r588 (scope relative) (span (offset 91202) (line 1652) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 91202) (line 1652) (column 35) (len 19)))))
    (reference r589 (scope relative) (span (offset 91231) (line 1652) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 91231) (line 1652) (column 64) (len 8)))))
    (reference r590 (scope relative) (span (offset 91242) (line 1652) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 91242) (line 1652) (column 75) (len 3)))))
    (reference r591 (scope relative) (span (offset 91246) (line 1652) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 91246) (line 1652) (column 79) (len 1)))))
    (reference r592 (scope relative) (span (offset 91253) (line 1652) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 91253) (line 1652) (column 86) (len 8)))))
    (reference r593 (scope relative) (span (offset 91292) (line 1653) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 91292) (line 1653) (column 23) (len 17)))))
    (reference r594 (scope relative) (span (offset 91316) (line 1653) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 91316) (line 1653) (column 47) (len 20)))))
    (reference r595 (scope relative) (span (offset 91340) (line 1653) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 91340) (line 1653) (column 71) (len 8)))))
    (reference r596 (scope relative) (span (offset 91350) (line 1653) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 91350) (line 1653) (column 81) (len 6)))))
    (reference r597 (scope relative) (span (offset 91486) (line 1657) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 91486) (line 1657) (column 55) (len 19)))))
    (reference r598 (scope relative) (span (offset 92023) (line 1670) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 92023) (line 1670) (column 28) (len 4)))))
    (reference r599 (scope relative) (span (offset 92018) (line 1670) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 92018) (line 1670) (column 23) (len 3)))))
    (reference r600 (scope relative) (span (offset 92057) (line 1671) (column 29) (len 31)) (segments (segment 0 (token "MolarAttenuationCoefficientUnit") (name "MolarAttenuationCoefficientUnit") (separator none) (span (offset 92057) (line 1671) (column 29) (len 31)))))
    (reference r601 (scope relative) (span (offset 92051) (line 1671) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 92051) (line 1671) (column 23) (len 4)))))
    (reference r602 (scope relative) (span (offset 92264) (line 1676) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 92264) (line 1676) (column 54) (len 11)))))
    (reference r603 (scope relative) (span (offset 92314) (line 1677) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 92314) (line 1677) (column 37) (len 19)))))
    (reference r604 (scope relative) (span (offset 92343) (line 1677) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 92343) (line 1677) (column 66) (len 8)))))
    (reference r605 (scope relative) (span (offset 92354) (line 1677) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 92354) (line 1677) (column 77) (len 3)))))
    (reference r606 (scope relative) (span (offset 92358) (line 1677) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 92358) (line 1677) (column 81) (len 1)))))
    (reference r607 (scope relative) (span (offset 92365) (line 1677) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 92365) (line 1677) (column 88) (len 8)))))
    (reference r608 (scope relative) (span (offset 92428) (line 1678) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 92428) (line 1678) (column 48) (len 19)))))
    (reference r609 (scope relative) (span (offset 92457) (line 1678) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 92457) (line 1678) (column 77) (len 8)))))
    (reference r610 (scope relative) (span (offset 92468) (line 1678) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 92468) (line 1678) (column 88) (len 3)))))
    (reference r611 (scope relative) (span (offset 92472) (line 1678) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 92472) (line 1678) (column 92) (len 1)))))
    (reference r612 (scope relative) (span (offset 92479) (line 1678) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 92479) (line 1678) (column 99) (len 8)))))
    (reference r613 (scope relative) (span (offset 92518) (line 1679) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 92518) (line 1679) (column 23) (len 17)))))
    (reference r614 (scope relative) (span (offset 92542) (line 1679) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 92542) (line 1679) (column 47) (len 20)))))
    (reference r615 (scope relative) (span (offset 92566) (line 1679) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 92566) (line 1679) (column 71) (len 8)))))
    (reference r616 (scope relative) (span (offset 92576) (line 1679) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 92576) (line 1679) (column 81) (len 19)))))
    (reference r617 (scope relative) (span (offset 92727) (line 1683) (column 56) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 92727) (line 1683) (column 56) (len 19)))))
    (reference r618 (scope relative) (span (offset 93396) (line 1696) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 93396) (line 1696) (column 28) (len 4)))))
    (reference r619 (scope relative) (span (offset 93391) (line 1696) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 93391) (line 1696) (column 23) (len 3)))))
    (reference r620 (scope relative) (span (offset 93430) (line 1697) (column 29) (len 32)) (segments (segment 0 (token "AtomicAttenuationCoefficientUnit") (name "AtomicAttenuationCoefficientUnit") (separator none) (span (offset 93430) (line 1697) (column 29) (len 32)))))
    (reference r621 (scope relative) (span (offset 93424) (line 1697) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 93424) (line 1697) (column 23) (len 4)))))
    (reference r622 (scope relative) (span (offset 93641) (line 1702) (column 55) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 93641) (line 1702) (column 55) (len 11)))))
    (reference r623 (scope relative) (span (offset 93691) (line 1703) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 93691) (line 1703) (column 37) (len 19)))))
    (reference r624 (scope relative) (span (offset 93720) (line 1703) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 93720) (line 1703) (column 66) (len 8)))))
    (reference r625 (scope relative) (span (offset 93731) (line 1703) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 93731) (line 1703) (column 77) (len 3)))))
    (reference r626 (scope relative) (span (offset 93735) (line 1703) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 93735) (line 1703) (column 81) (len 1)))))
    (reference r627 (scope relative) (span (offset 93742) (line 1703) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 93742) (line 1703) (column 88) (len 8)))))
    (reference r628 (scope relative) (span (offset 93780) (line 1704) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 93780) (line 1704) (column 23) (len 17)))))
    (reference r629 (scope relative) (span (offset 93804) (line 1704) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 93804) (line 1704) (column 47) (len 20)))))
    (reference r630 (scope relative) (span (offset 93827) (line 1704) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 93827) (line 1704) (column 70) (len 8)))))
    (reference r631 (scope relative) (span (offset 94742) (line 1724) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 94742) (line 1724) (column 52) (len 19)))))
    (reference r632 (scope relative) (span (offset 96400) (line 1737) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 96400) (line 1737) (column 28) (len 4)))))
    (reference r633 (scope relative) (span (offset 96395) (line 1737) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 96395) (line 1737) (column 23) (len 3)))))
    (reference r634 (scope relative) (span (offset 96434) (line 1738) (column 29) (len 28)) (segments (segment 0 (token "TotalLinearStoppingPowerUnit") (name "TotalLinearStoppingPowerUnit") (separator none) (span (offset 96434) (line 1738) (column 29) (len 28)))))
    (reference r635 (scope relative) (span (offset 96428) (line 1738) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 96428) (line 1738) (column 23) (len 4)))))
    (reference r636 (scope relative) (span (offset 96629) (line 1743) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 96629) (line 1743) (column 51) (len 11)))))
    (reference r637 (scope relative) (span (offset 96679) (line 1744) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 96679) (line 1744) (column 37) (len 19)))))
    (reference r638 (scope relative) (span (offset 96708) (line 1744) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 96708) (line 1744) (column 66) (len 8)))))
    (reference r639 (scope relative) (span (offset 96719) (line 1744) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 96719) (line 1744) (column 77) (len 3)))))
    (reference r640 (scope relative) (span (offset 96723) (line 1744) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 96723) (line 1744) (column 81) (len 1)))))
    (reference r641 (scope relative) (span (offset 96730) (line 1744) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 96730) (line 1744) (column 88) (len 8)))))
    (reference r642 (scope relative) (span (offset 96780) (line 1745) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 96780) (line 1745) (column 35) (len 19)))))
    (reference r643 (scope relative) (span (offset 96809) (line 1745) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 96809) (line 1745) (column 64) (len 8)))))
    (reference r644 (scope relative) (span (offset 96820) (line 1745) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 96820) (line 1745) (column 75) (len 3)))))
    (reference r645 (scope relative) (span (offset 96824) (line 1745) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 96824) (line 1745) (column 79) (len 1)))))
    (reference r646 (scope relative) (span (offset 96831) (line 1745) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 96831) (line 1745) (column 86) (len 8)))))
    (reference r647 (scope relative) (span (offset 96885) (line 1746) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 96885) (line 1746) (column 39) (len 19)))))
    (reference r648 (scope relative) (span (offset 96914) (line 1746) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 96914) (line 1746) (column 68) (len 8)))))
    (reference r649 (scope relative) (span (offset 96925) (line 1746) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 96925) (line 1746) (column 79) (len 3)))))
    (reference r650 (scope relative) (span (offset 96929) (line 1746) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 96929) (line 1746) (column 83) (len 1)))))
    (reference r651 (scope relative) (span (offset 96936) (line 1746) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 96936) (line 1746) (column 90) (len 8)))))
    (reference r652 (scope relative) (span (offset 96975) (line 1747) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 96975) (line 1747) (column 23) (len 17)))))
    (reference r653 (scope relative) (span (offset 96999) (line 1747) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 96999) (line 1747) (column 47) (len 20)))))
    (reference r654 (scope relative) (span (offset 97023) (line 1747) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 97023) (line 1747) (column 71) (len 8)))))
    (reference r655 (scope relative) (span (offset 97033) (line 1747) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 97033) (line 1747) (column 81) (len 6)))))
    (reference r656 (scope relative) (span (offset 97041) (line 1747) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 97041) (line 1747) (column 89) (len 10)))))
    (reference r657 (scope relative) (span (offset 97101) (line 1750) (column 39) (len 28)) (segments (segment 0 (token "TotalLinearStoppingPowerUnit") (name "TotalLinearStoppingPowerUnit") (separator none) (span (offset 97101) (line 1750) (column 39) (len 28)))))
    (reference r658 (scope relative) (span (offset 97170) (line 1751) (column 40) (len 29)) (segments (segment 0 (token "TotalLinearStoppingPowerValue") (name "TotalLinearStoppingPowerValue") (separator none) (span (offset 97170) (line 1751) (column 40) (len 29)))))
    (reference r659 (scope relative) (span (offset 97235) (line 1752) (column 35) (len 24)) (segments (segment 0 (token "totalLinearStoppingPower") (name "totalLinearStoppingPower") (separator none) (span (offset 97235) (line 1752) (column 35) (len 24)))))
    (reference r660 (scope relative) (span (offset 97392) (line 1755) (column 50) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 97392) (line 1755) (column 50) (len 19)))))
    (reference r661 (scope relative) (span (offset 98093) (line 1768) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 98093) (line 1768) (column 28) (len 4)))))
    (reference r662 (scope relative) (span (offset 98088) (line 1768) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 98088) (line 1768) (column 23) (len 3)))))
    (reference r663 (scope relative) (span (offset 98127) (line 1769) (column 29) (len 26)) (segments (segment 0 (token "TotalMassStoppingPowerUnit") (name "TotalMassStoppingPowerUnit") (separator none) (span (offset 98127) (line 1769) (column 29) (len 26)))))
    (reference r664 (scope relative) (span (offset 98121) (line 1769) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 98121) (line 1769) (column 23) (len 4)))))
    (reference r665 (scope relative) (span (offset 98314) (line 1774) (column 49) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 98314) (line 1774) (column 49) (len 11)))))
    (reference r666 (scope relative) (span (offset 98364) (line 1775) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 98364) (line 1775) (column 37) (len 19)))))
    (reference r667 (scope relative) (span (offset 98393) (line 1775) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 98393) (line 1775) (column 66) (len 8)))))
    (reference r668 (scope relative) (span (offset 98404) (line 1775) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 98404) (line 1775) (column 77) (len 3)))))
    (reference r669 (scope relative) (span (offset 98408) (line 1775) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 98408) (line 1775) (column 81) (len 1)))))
    (reference r670 (scope relative) (span (offset 98415) (line 1775) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 98415) (line 1775) (column 88) (len 8)))))
    (reference r671 (scope relative) (span (offset 98469) (line 1776) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 98469) (line 1776) (column 39) (len 19)))))
    (reference r672 (scope relative) (span (offset 98498) (line 1776) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 98498) (line 1776) (column 68) (len 8)))))
    (reference r673 (scope relative) (span (offset 98509) (line 1776) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 98509) (line 1776) (column 79) (len 3)))))
    (reference r674 (scope relative) (span (offset 98513) (line 1776) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 98513) (line 1776) (column 83) (len 1)))))
    (reference r675 (scope relative) (span (offset 98520) (line 1776) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 98520) (line 1776) (column 90) (len 8)))))
    (reference r676 (scope relative) (span (offset 98559) (line 1777) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 98559) (line 1777) (column 23) (len 17)))))
    (reference r677 (scope relative) (span (offset 98583) (line 1777) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 98583) (line 1777) (column 47) (len 20)))))
    (reference r678 (scope relative) (span (offset 98607) (line 1777) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 98607) (line 1777) (column 71) (len 8)))))
    (reference r679 (scope relative) (span (offset 98617) (line 1777) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 98617) (line 1777) (column 81) (len 10)))))
    (reference r680 (scope relative) (span (offset 98675) (line 1780) (column 37) (len 26)) (segments (segment 0 (token "TotalMassStoppingPowerUnit") (name "TotalMassStoppingPowerUnit") (separator none) (span (offset 98675) (line 1780) (column 37) (len 26)))))
    (reference r681 (scope relative) (span (offset 98740) (line 1781) (column 38) (len 27)) (segments (segment 0 (token "TotalMassStoppingPowerValue") (name "TotalMassStoppingPowerValue") (separator none) (span (offset 98740) (line 1781) (column 38) (len 27)))))
    (reference r682 (scope relative) (span (offset 98801) (line 1782) (column 33) (len 22)) (segments (segment 0 (token "totalMassStoppingPower") (name "totalMassStoppingPower") (separator none) (span (offset 98801) (line 1782) (column 33) (len 22)))))
    (reference r683 (scope relative) (span (offset 99603) (line 1801) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 99603) (line 1801) (column 41) (len 19)))))
    (reference r684 (scope relative) (span (offset 100116) (line 1814) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 100116) (line 1814) (column 28) (len 4)))))
    (reference r685 (scope relative) (span (offset 100111) (line 1814) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 100111) (line 1814) (column 23) (len 3)))))
    (reference r686 (scope relative) (span (offset 100150) (line 1815) (column 29) (len 17)) (segments (segment 0 (token "MeanMassRangeUnit") (name "MeanMassRangeUnit") (separator none) (span (offset 100150) (line 1815) (column 29) (len 17)))))
    (reference r687 (scope relative) (span (offset 100144) (line 1815) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 100144) (line 1815) (column 23) (len 4)))))
    (reference r688 (scope relative) (span (offset 100301) (line 1820) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 100301) (line 1820) (column 40) (len 11)))))
    (reference r689 (scope relative) (span (offset 100351) (line 1821) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 100351) (line 1821) (column 37) (len 19)))))
    (reference r690 (scope relative) (span (offset 100380) (line 1821) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 100380) (line 1821) (column 66) (len 8)))))
    (reference r691 (scope relative) (span (offset 100391) (line 1821) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 100391) (line 1821) (column 77) (len 3)))))
    (reference r692 (scope relative) (span (offset 100395) (line 1821) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 100395) (line 1821) (column 81) (len 1)))))
    (reference r693 (scope relative) (span (offset 100402) (line 1821) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 100402) (line 1821) (column 88) (len 8)))))
    (reference r694 (scope relative) (span (offset 100453) (line 1822) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 100453) (line 1822) (column 35) (len 19)))))
    (reference r695 (scope relative) (span (offset 100482) (line 1822) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 100482) (line 1822) (column 64) (len 8)))))
    (reference r696 (scope relative) (span (offset 100493) (line 1822) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 100493) (line 1822) (column 75) (len 3)))))
    (reference r697 (scope relative) (span (offset 100497) (line 1822) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 100497) (line 1822) (column 79) (len 1)))))
    (reference r698 (scope relative) (span (offset 100504) (line 1822) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 100504) (line 1822) (column 86) (len 8)))))
    (reference r699 (scope relative) (span (offset 100542) (line 1823) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 100542) (line 1823) (column 23) (len 17)))))
    (reference r700 (scope relative) (span (offset 100566) (line 1823) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 100566) (line 1823) (column 47) (len 20)))))
    (reference r701 (scope relative) (span (offset 100590) (line 1823) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 100590) (line 1823) (column 71) (len 8)))))
    (reference r702 (scope relative) (span (offset 100600) (line 1823) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 100600) (line 1823) (column 81) (len 6)))))
    (reference r703 (scope relative) (span (offset 100713) (line 1827) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 100713) (line 1827) (column 44) (len 19)))))
    (reference r704 (scope relative) (span (offset 101427) (line 1840) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 101427) (line 1840) (column 28) (len 4)))))
    (reference r705 (scope relative) (span (offset 101422) (line 1840) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 101422) (line 1840) (column 23) (len 3)))))
    (reference r706 (scope relative) (span (offset 101461) (line 1841) (column 29) (len 20)) (segments (segment 0 (token "LinearIonizationUnit") (name "LinearIonizationUnit") (separator none) (span (offset 101461) (line 1841) (column 29) (len 20)))))
    (reference r707 (scope relative) (span (offset 101455) (line 1841) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 101455) (line 1841) (column 23) (len 4)))))
    (reference r708 (scope relative) (span (offset 101624) (line 1846) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 101624) (line 1846) (column 43) (len 11)))))
    (reference r709 (scope relative) (span (offset 101674) (line 1847) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 101674) (line 1847) (column 37) (len 19)))))
    (reference r710 (scope relative) (span (offset 101703) (line 1847) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 101703) (line 1847) (column 66) (len 8)))))
    (reference r711 (scope relative) (span (offset 101714) (line 1847) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 101714) (line 1847) (column 77) (len 3)))))
    (reference r712 (scope relative) (span (offset 101718) (line 1847) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 101718) (line 1847) (column 81) (len 1)))))
    (reference r713 (scope relative) (span (offset 101725) (line 1847) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 101725) (line 1847) (column 88) (len 8)))))
    (reference r714 (scope relative) (span (offset 101764) (line 1848) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 101764) (line 1848) (column 23) (len 17)))))
    (reference r715 (scope relative) (span (offset 101788) (line 1848) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 101788) (line 1848) (column 47) (len 20)))))
    (reference r716 (scope relative) (span (offset 101811) (line 1848) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 101811) (line 1848) (column 70) (len 8)))))
    (reference r717 (scope relative) (span (offset 101923) (line 1852) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 101923) (line 1852) (column 43) (len 17)))))
    (reference r718 (scope relative) (span (offset 102790) (line 1869) (column 72) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 102790) (line 1869) (column 72) (len 19)))))
    (reference r719 (scope relative) (span (offset 104468) (line 1882) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 104468) (line 1882) (column 28) (len 4)))))
    (reference r720 (scope relative) (span (offset 104463) (line 1882) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 104463) (line 1882) (column 23) (len 3)))))
    (reference r721 (scope relative) (span (offset 104502) (line 1883) (column 29) (len 48)) (segments (segment 0 (token "AverageEnergyLossPerElementaryChargeProducedUnit") (name "AverageEnergyLossPerElementaryChargeProducedUnit") (separator none) (span (offset 104502) (line 1883) (column 29) (len 48)))))
    (reference r722 (scope relative) (span (offset 104496) (line 1883) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 104496) (line 1883) (column 23) (len 4)))))
    (reference r723 (scope relative) (span (offset 104777) (line 1888) (column 71) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 104777) (line 1888) (column 71) (len 11)))))
    (reference r724 (scope relative) (span (offset 104827) (line 1889) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 104827) (line 1889) (column 37) (len 19)))))
    (reference r725 (scope relative) (span (offset 104856) (line 1889) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 104856) (line 1889) (column 66) (len 8)))))
    (reference r726 (scope relative) (span (offset 104867) (line 1889) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 104867) (line 1889) (column 77) (len 3)))))
    (reference r727 (scope relative) (span (offset 104871) (line 1889) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 104871) (line 1889) (column 81) (len 1)))))
    (reference r728 (scope relative) (span (offset 104878) (line 1889) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 104878) (line 1889) (column 88) (len 8)))))
    (reference r729 (scope relative) (span (offset 104928) (line 1890) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 104928) (line 1890) (column 35) (len 19)))))
    (reference r730 (scope relative) (span (offset 104957) (line 1890) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 104957) (line 1890) (column 64) (len 8)))))
    (reference r731 (scope relative) (span (offset 104968) (line 1890) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 104968) (line 1890) (column 75) (len 3)))))
    (reference r732 (scope relative) (span (offset 104972) (line 1890) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 104972) (line 1890) (column 79) (len 1)))))
    (reference r733 (scope relative) (span (offset 104979) (line 1890) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 104979) (line 1890) (column 86) (len 8)))))
    (reference r734 (scope relative) (span (offset 105033) (line 1891) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 105033) (line 1891) (column 39) (len 19)))))
    (reference r735 (scope relative) (span (offset 105062) (line 1891) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 105062) (line 1891) (column 68) (len 8)))))
    (reference r736 (scope relative) (span (offset 105073) (line 1891) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 105073) (line 1891) (column 79) (len 3)))))
    (reference r737 (scope relative) (span (offset 105077) (line 1891) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 105077) (line 1891) (column 83) (len 1)))))
    (reference r738 (scope relative) (span (offset 105084) (line 1891) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 105084) (line 1891) (column 90) (len 8)))))
    (reference r739 (scope relative) (span (offset 105123) (line 1892) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 105123) (line 1892) (column 23) (len 17)))))
    (reference r740 (scope relative) (span (offset 105147) (line 1892) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 105147) (line 1892) (column 47) (len 20)))))
    (reference r741 (scope relative) (span (offset 105171) (line 1892) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 105171) (line 1892) (column 71) (len 8)))))
    (reference r742 (scope relative) (span (offset 105181) (line 1892) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 105181) (line 1892) (column 81) (len 6)))))
    (reference r743 (scope relative) (span (offset 105189) (line 1892) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 105189) (line 1892) (column 89) (len 10)))))
    (reference r744 (scope relative) (span (offset 105289) (line 1896) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 105289) (line 1896) (column 36) (len 19)))))
    (reference r745 (scope relative) (span (offset 105844) (line 1909) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 105844) (line 1909) (column 28) (len 4)))))
    (reference r746 (scope relative) (span (offset 105839) (line 1909) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 105839) (line 1909) (column 23) (len 3)))))
    (reference r747 (scope relative) (span (offset 105878) (line 1910) (column 29) (len 12)) (segments (segment 0 (token "MobilityUnit") (name "MobilityUnit") (separator none) (span (offset 105878) (line 1910) (column 29) (len 12)))))
    (reference r748 (scope relative) (span (offset 105872) (line 1910) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 105872) (line 1910) (column 23) (len 4)))))
    (reference r749 (scope relative) (span (offset 106009) (line 1915) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 106009) (line 1915) (column 35) (len 11)))))
    (reference r750 (scope relative) (span (offset 106057) (line 1916) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 106057) (line 1916) (column 35) (len 19)))))
    (reference r751 (scope relative) (span (offset 106086) (line 1916) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 106086) (line 1916) (column 64) (len 8)))))
    (reference r752 (scope relative) (span (offset 106097) (line 1916) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 106097) (line 1916) (column 75) (len 3)))))
    (reference r753 (scope relative) (span (offset 106101) (line 1916) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 106101) (line 1916) (column 79) (len 1)))))
    (reference r754 (scope relative) (span (offset 106108) (line 1916) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 106108) (line 1916) (column 86) (len 8)))))
    (reference r755 (scope relative) (span (offset 106163) (line 1917) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 106163) (line 1917) (column 39) (len 19)))))
    (reference r756 (scope relative) (span (offset 106192) (line 1917) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 106192) (line 1917) (column 68) (len 8)))))
    (reference r757 (scope relative) (span (offset 106203) (line 1917) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 106203) (line 1917) (column 79) (len 3)))))
    (reference r758 (scope relative) (span (offset 106207) (line 1917) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 106207) (line 1917) (column 83) (len 1)))))
    (reference r759 (scope relative) (span (offset 106214) (line 1917) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 106214) (line 1917) (column 90) (len 8)))))
    (reference r760 (scope relative) (span (offset 106275) (line 1918) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 106275) (line 1918) (column 46) (len 19)))))
    (reference r761 (scope relative) (span (offset 106304) (line 1918) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 106304) (line 1918) (column 75) (len 8)))))
    (reference r762 (scope relative) (span (offset 106315) (line 1918) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 106315) (line 1918) (column 86) (len 3)))))
    (reference r763 (scope relative) (span (offset 106319) (line 1918) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 106319) (line 1918) (column 90) (len 1)))))
    (reference r764 (scope relative) (span (offset 106326) (line 1918) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 106326) (line 1918) (column 97) (len 8)))))
    (reference r765 (scope relative) (span (offset 106364) (line 1919) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 106364) (line 1919) (column 23) (len 17)))))
    (reference r766 (scope relative) (span (offset 106388) (line 1919) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 106388) (line 1919) (column 47) (len 20)))))
    (reference r767 (scope relative) (span (offset 106412) (line 1919) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 106412) (line 1919) (column 71) (len 6)))))
    (reference r768 (scope relative) (span (offset 106420) (line 1919) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 106420) (line 1919) (column 79) (len 10)))))
    (reference r769 (scope relative) (span (offset 106432) (line 1919) (column 91) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 106432) (line 1919) (column 91) (len 17)))))
    (reference r770 (scope relative) (span (offset 106569) (line 1923) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 106569) (line 1923) (column 49) (len 19)))))
    (reference r771 (scope relative) (span (offset 107370) (line 1936) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 107370) (line 1936) (column 28) (len 4)))))
    (reference r772 (scope relative) (span (offset 107365) (line 1936) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 107365) (line 1936) (column 23) (len 3)))))
    (reference r773 (scope relative) (span (offset 107404) (line 1937) (column 29) (len 25)) (segments (segment 0 (token "ParticleNumberDensityUnit") (name "ParticleNumberDensityUnit") (separator none) (span (offset 107404) (line 1937) (column 29) (len 25)))))
    (reference r774 (scope relative) (span (offset 107398) (line 1937) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 107398) (line 1937) (column 23) (len 4)))))
    (reference r775 (scope relative) (span (offset 107587) (line 1942) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 107587) (line 1942) (column 48) (len 11)))))
    (reference r776 (scope relative) (span (offset 107637) (line 1943) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 107637) (line 1943) (column 37) (len 19)))))
    (reference r777 (scope relative) (span (offset 107666) (line 1943) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 107666) (line 1943) (column 66) (len 8)))))
    (reference r778 (scope relative) (span (offset 107677) (line 1943) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 107677) (line 1943) (column 77) (len 3)))))
    (reference r779 (scope relative) (span (offset 107681) (line 1943) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 107681) (line 1943) (column 81) (len 1)))))
    (reference r780 (scope relative) (span (offset 107688) (line 1943) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 107688) (line 1943) (column 88) (len 8)))))
    (reference r781 (scope relative) (span (offset 107727) (line 1944) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 107727) (line 1944) (column 23) (len 17)))))
    (reference r782 (scope relative) (span (offset 107751) (line 1944) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 107751) (line 1944) (column 47) (len 20)))))
    (reference r783 (scope relative) (span (offset 107774) (line 1944) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 107774) (line 1944) (column 70) (len 8)))))
    (reference r784 (scope relative) (span (offset 107904) (line 1948) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 107904) (line 1948) (column 44) (len 19)))))
    (reference r785 (scope relative) (span (offset 108493) (line 1961) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 108493) (line 1961) (column 28) (len 4)))))
    (reference r786 (scope relative) (span (offset 108488) (line 1961) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 108488) (line 1961) (column 23) (len 3)))))
    (reference r787 (scope relative) (span (offset 108527) (line 1962) (column 29) (len 20)) (segments (segment 0 (token "IonNumberDensityUnit") (name "IonNumberDensityUnit") (separator none) (span (offset 108527) (line 1962) (column 29) (len 20)))))
    (reference r788 (scope relative) (span (offset 108521) (line 1962) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 108521) (line 1962) (column 23) (len 4)))))
    (reference r789 (scope relative) (span (offset 108690) (line 1967) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 108690) (line 1967) (column 43) (len 11)))))
    (reference r790 (scope relative) (span (offset 108740) (line 1968) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 108740) (line 1968) (column 37) (len 19)))))
    (reference r791 (scope relative) (span (offset 108769) (line 1968) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 108769) (line 1968) (column 66) (len 8)))))
    (reference r792 (scope relative) (span (offset 108780) (line 1968) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 108780) (line 1968) (column 77) (len 3)))))
    (reference r793 (scope relative) (span (offset 108784) (line 1968) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 108784) (line 1968) (column 81) (len 1)))))
    (reference r794 (scope relative) (span (offset 108791) (line 1968) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 108791) (line 1968) (column 88) (len 8)))))
    (reference r795 (scope relative) (span (offset 108830) (line 1969) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 108830) (line 1969) (column 23) (len 17)))))
    (reference r796 (scope relative) (span (offset 108854) (line 1969) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 108854) (line 1969) (column 47) (len 20)))))
    (reference r797 (scope relative) (span (offset 108877) (line 1969) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 108877) (line 1969) (column 70) (len 8)))))
    (reference r798 (scope relative) (span (offset 108925) (line 1972) (column 30) (len 20)) (segments (segment 0 (token "IonNumberDensityUnit") (name "IonNumberDensityUnit") (separator none) (span (offset 108925) (line 1972) (column 30) (len 20)))))
    (reference r799 (scope relative) (span (offset 108977) (line 1973) (column 31) (len 21)) (segments (segment 0 (token "IonNumberDensityValue") (name "IonNumberDensityValue") (separator none) (span (offset 108977) (line 1973) (column 31) (len 21)))))
    (reference r800 (scope relative) (span (offset 109025) (line 1974) (column 26) (len 16)) (segments (segment 0 (token "ionNumberDensity") (name "ionNumberDensity") (separator none) (span (offset 109025) (line 1974) (column 26) (len 16)))))
    (reference r801 (scope relative) (span (offset 109155) (line 1977) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 109155) (line 1977) (column 52) (len 19)))))
    (reference r802 (scope relative) (span (offset 110028) (line 1990) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 110028) (line 1990) (column 28) (len 4)))))
    (reference r803 (scope relative) (span (offset 110023) (line 1990) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 110023) (line 1990) (column 23) (len 3)))))
    (reference r804 (scope relative) (span (offset 110062) (line 1991) (column 29) (len 28)) (segments (segment 0 (token "RecombinationCoefficientUnit") (name "RecombinationCoefficientUnit") (separator none) (span (offset 110062) (line 1991) (column 29) (len 28)))))
    (reference r805 (scope relative) (span (offset 110056) (line 1991) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 110056) (line 1991) (column 23) (len 4)))))
    (reference r806 (scope relative) (span (offset 110257) (line 1996) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 110257) (line 1996) (column 51) (len 11)))))
    (reference r807 (scope relative) (span (offset 110307) (line 1997) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 110307) (line 1997) (column 37) (len 19)))))
    (reference r808 (scope relative) (span (offset 110336) (line 1997) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 110336) (line 1997) (column 66) (len 8)))))
    (reference r809 (scope relative) (span (offset 110347) (line 1997) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 110347) (line 1997) (column 77) (len 3)))))
    (reference r810 (scope relative) (span (offset 110351) (line 1997) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 110351) (line 1997) (column 81) (len 1)))))
    (reference r811 (scope relative) (span (offset 110358) (line 1997) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 110358) (line 1997) (column 88) (len 8)))))
    (reference r812 (scope relative) (span (offset 110412) (line 1998) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 110412) (line 1998) (column 39) (len 19)))))
    (reference r813 (scope relative) (span (offset 110441) (line 1998) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 110441) (line 1998) (column 68) (len 8)))))
    (reference r814 (scope relative) (span (offset 110452) (line 1998) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 110452) (line 1998) (column 79) (len 3)))))
    (reference r815 (scope relative) (span (offset 110456) (line 1998) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 110456) (line 1998) (column 83) (len 1)))))
    (reference r816 (scope relative) (span (offset 110463) (line 1998) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 110463) (line 1998) (column 90) (len 8)))))
    (reference r817 (scope relative) (span (offset 110502) (line 1999) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 110502) (line 1999) (column 23) (len 17)))))
    (reference r818 (scope relative) (span (offset 110526) (line 1999) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 110526) (line 1999) (column 47) (len 20)))))
    (reference r819 (scope relative) (span (offset 110550) (line 1999) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 110550) (line 1999) (column 71) (len 8)))))
    (reference r820 (scope relative) (span (offset 110560) (line 1999) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 110560) (line 1999) (column 81) (len 10)))))
    (reference r821 (scope relative) (span (offset 110866) (line 2005) (column 64) (len 24)) (segments (segment 0 (token "DiffusionCoefficientUnit") (name "DiffusionCoefficientUnit") (separator none) (span (offset 110866) (line 2005) (column 64) (len 24)))))
    (reference r822 (scope relative) (span (offset 110956) (line 2006) (column 65) (len 25)) (segments (segment 0 (token "DiffusionCoefficientValue") (name "DiffusionCoefficientValue") (separator none) (span (offset 110956) (line 2006) (column 65) (len 25)))))
    (reference r823 (scope relative) (span (offset 111042) (line 2007) (column 60) (len 20)) (segments (segment 0 (token "diffusionCoefficient") (name "diffusionCoefficient") (separator none) (span (offset 111042) (line 2007) (column 60) (len 20)))))
    (reference r824 (scope relative) (span (offset 112035) (line 2026) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 112035) (line 2026) (column 49) (len 19)))))
    (reference r825 (scope relative) (span (offset 112777) (line 2039) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 112777) (line 2039) (column 28) (len 4)))))
    (reference r826 (scope relative) (span (offset 112772) (line 2039) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 112772) (line 2039) (column 23) (len 3)))))
    (reference r827 (scope relative) (span (offset 112811) (line 2040) (column 29) (len 25)) (segments (segment 0 (token "ParticleSourceDensityUnit") (name "ParticleSourceDensityUnit") (separator none) (span (offset 112811) (line 2040) (column 29) (len 25)))))
    (reference r828 (scope relative) (span (offset 112805) (line 2040) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 112805) (line 2040) (column 23) (len 4)))))
    (reference r829 (scope relative) (span (offset 112994) (line 2045) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 112994) (line 2045) (column 48) (len 11)))))
    (reference r830 (scope relative) (span (offset 113044) (line 2046) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 113044) (line 2046) (column 37) (len 19)))))
    (reference r831 (scope relative) (span (offset 113073) (line 2046) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 113073) (line 2046) (column 66) (len 8)))))
    (reference r832 (scope relative) (span (offset 113084) (line 2046) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 113084) (line 2046) (column 77) (len 3)))))
    (reference r833 (scope relative) (span (offset 113088) (line 2046) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 113088) (line 2046) (column 81) (len 1)))))
    (reference r834 (scope relative) (span (offset 113095) (line 2046) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 113095) (line 2046) (column 88) (len 8)))))
    (reference r835 (scope relative) (span (offset 113150) (line 2047) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 113150) (line 2047) (column 39) (len 19)))))
    (reference r836 (scope relative) (span (offset 113179) (line 2047) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 113179) (line 2047) (column 68) (len 8)))))
    (reference r837 (scope relative) (span (offset 113190) (line 2047) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 113190) (line 2047) (column 79) (len 3)))))
    (reference r838 (scope relative) (span (offset 113194) (line 2047) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 113194) (line 2047) (column 83) (len 1)))))
    (reference r839 (scope relative) (span (offset 113201) (line 2047) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 113201) (line 2047) (column 90) (len 8)))))
    (reference r840 (scope relative) (span (offset 113240) (line 2048) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 113240) (line 2048) (column 23) (len 17)))))
    (reference r841 (scope relative) (span (offset 113264) (line 2048) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 113264) (line 2048) (column 47) (len 20)))))
    (reference r842 (scope relative) (span (offset 113288) (line 2048) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 113288) (line 2048) (column 71) (len 8)))))
    (reference r843 (scope relative) (span (offset 113298) (line 2048) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 113298) (line 2048) (column 81) (len 10)))))
    (reference r844 (scope relative) (span (offset 113420) (line 2052) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 113420) (line 2052) (column 46) (len 19)))))
    (reference r845 (scope relative) (span (offset 113999) (line 2065) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 113999) (line 2065) (column 28) (len 4)))))
    (reference r846 (scope relative) (span (offset 113994) (line 2065) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 113994) (line 2065) (column 23) (len 3)))))
    (reference r847 (scope relative) (span (offset 114033) (line 2066) (column 29) (len 22)) (segments (segment 0 (token "SlowingDownDensityUnit") (name "SlowingDownDensityUnit") (separator none) (span (offset 114033) (line 2066) (column 29) (len 22)))))
    (reference r848 (scope relative) (span (offset 114027) (line 2066) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 114027) (line 2066) (column 23) (len 4)))))
    (reference r849 (scope relative) (span (offset 114204) (line 2071) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 114204) (line 2071) (column 45) (len 11)))))
    (reference r850 (scope relative) (span (offset 114254) (line 2072) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 114254) (line 2072) (column 37) (len 19)))))
    (reference r851 (scope relative) (span (offset 114283) (line 2072) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 114283) (line 2072) (column 66) (len 8)))))
    (reference r852 (scope relative) (span (offset 114294) (line 2072) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 114294) (line 2072) (column 77) (len 3)))))
    (reference r853 (scope relative) (span (offset 114298) (line 2072) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 114298) (line 2072) (column 81) (len 1)))))
    (reference r854 (scope relative) (span (offset 114305) (line 2072) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 114305) (line 2072) (column 88) (len 8)))))
    (reference r855 (scope relative) (span (offset 114360) (line 2073) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 114360) (line 2073) (column 39) (len 19)))))
    (reference r856 (scope relative) (span (offset 114389) (line 2073) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 114389) (line 2073) (column 68) (len 8)))))
    (reference r857 (scope relative) (span (offset 114400) (line 2073) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 114400) (line 2073) (column 79) (len 3)))))
    (reference r858 (scope relative) (span (offset 114404) (line 2073) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 114404) (line 2073) (column 83) (len 1)))))
    (reference r859 (scope relative) (span (offset 114411) (line 2073) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 114411) (line 2073) (column 90) (len 8)))))
    (reference r860 (scope relative) (span (offset 114450) (line 2074) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 114450) (line 2074) (column 23) (len 17)))))
    (reference r861 (scope relative) (span (offset 114474) (line 2074) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 114474) (line 2074) (column 47) (len 20)))))
    (reference r862 (scope relative) (span (offset 114498) (line 2074) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 114498) (line 2074) (column 71) (len 8)))))
    (reference r863 (scope relative) (span (offset 114508) (line 2074) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 114508) (line 2074) (column 81) (len 10)))))
    (reference r864 (scope relative) (span (offset 114646) (line 2078) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 114646) (line 2078) (column 54) (len 17)))))
    (reference r865 (scope relative) (span (offset 115411) (line 2095) (column 36) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 115411) (line 2095) (column 36) (len 17)))))
    (reference r866 (scope relative) (span (offset 116130) (line 2112) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 116130) (line 2112) (column 61) (len 17)))))
    (reference r867 (scope relative) (span (offset 122634) (line 2273) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 122634) (line 2273) (column 45) (len 19)))))
    (reference r868 (scope relative) (span (offset 123276) (line 2286) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 123276) (line 2286) (column 28) (len 4)))))
    (reference r869 (scope relative) (span (offset 123271) (line 2286) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 123271) (line 2286) (column 23) (len 3)))))
    (reference r870 (scope relative) (span (offset 123310) (line 2287) (column 29) (len 21)) (segments (segment 0 (token "FastFissionFactorUnit") (name "FastFissionFactorUnit") (separator none) (span (offset 123310) (line 2287) (column 29) (len 21)))))
    (reference r871 (scope relative) (span (offset 123304) (line 2287) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 123304) (line 2287) (column 23) (len 4)))))
    (reference r872 (scope relative) (span (offset 123477) (line 2292) (column 44) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 123477) (line 2292) (column 44) (len 16)))))
    (reference r873 (scope relative) (span (offset 123615) (line 2296) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 123615) (line 2296) (column 52) (len 19)))))
    (reference r874 (scope relative) (span (offset 124196) (line 2309) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 124196) (line 2309) (column 28) (len 4)))))
    (reference r875 (scope relative) (span (offset 124191) (line 2309) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 124191) (line 2309) (column 23) (len 3)))))
    (reference r876 (scope relative) (span (offset 124230) (line 2310) (column 29) (len 28)) (segments (segment 0 (token "ThermalUtilizationFactorUnit") (name "ThermalUtilizationFactorUnit") (separator none) (span (offset 124230) (line 2310) (column 29) (len 28)))))
    (reference r877 (scope relative) (span (offset 124224) (line 2310) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 124224) (line 2310) (column 23) (len 4)))))
    (reference r878 (scope relative) (span (offset 124425) (line 2315) (column 51) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 124425) (line 2315) (column 51) (len 16)))))
    (reference r879 (scope relative) (span (offset 124557) (line 2319) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 124557) (line 2319) (column 49) (len 19)))))
    (reference r880 (scope relative) (span (offset 125077) (line 2332) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 125077) (line 2332) (column 28) (len 4)))))
    (reference r881 (scope relative) (span (offset 125072) (line 2332) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 125072) (line 2332) (column 23) (len 3)))))
    (reference r882 (scope relative) (span (offset 125111) (line 2333) (column 29) (len 25)) (segments (segment 0 (token "NonLeakageProbabilityUnit") (name "NonLeakageProbabilityUnit") (separator none) (span (offset 125111) (line 2333) (column 29) (len 25)))))
    (reference r883 (scope relative) (span (offset 125105) (line 2333) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 125105) (line 2333) (column 23) (len 4)))))
    (reference r884 (scope relative) (span (offset 125294) (line 2338) (column 48) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 125294) (line 2338) (column 48) (len 16)))))
    (reference r885 (scope relative) (span (offset 125425) (line 2342) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 125425) (line 2342) (column 48) (len 19)))))
    (reference r886 (scope relative) (span (offset 126004) (line 2355) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 126004) (line 2355) (column 28) (len 4)))))
    (reference r887 (scope relative) (span (offset 125999) (line 2355) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 125999) (line 2355) (column 23) (len 3)))))
    (reference r888 (scope relative) (span (offset 126038) (line 2356) (column 29) (len 24)) (segments (segment 0 (token "MultiplicationFactorUnit") (name "MultiplicationFactorUnit") (separator none) (span (offset 126038) (line 2356) (column 29) (len 24)))))
    (reference r889 (scope relative) (span (offset 126032) (line 2356) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 126032) (line 2356) (column 23) (len 4)))))
    (reference r890 (scope relative) (span (offset 126217) (line 2361) (column 47) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 126217) (line 2361) (column 47) (len 16)))))
    (reference r891 (scope relative) (span (offset 126365) (line 2365) (column 56) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 126365) (line 2365) (column 56) (len 19)))))
    (reference r892 (scope relative) (span (offset 126904) (line 2378) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 126904) (line 2378) (column 28) (len 4)))))
    (reference r893 (scope relative) (span (offset 126899) (line 2378) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 126899) (line 2378) (column 23) (len 3)))))
    (reference r894 (scope relative) (span (offset 126938) (line 2379) (column 29) (len 32)) (segments (segment 0 (token "InfiniteMultiplicationFactorUnit") (name "InfiniteMultiplicationFactorUnit") (separator none) (span (offset 126938) (line 2379) (column 29) (len 32)))))
    (reference r895 (scope relative) (span (offset 126932) (line 2379) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 126932) (line 2379) (column 23) (len 4)))))
    (reference r896 (scope relative) (span (offset 127149) (line 2384) (column 55) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 127149) (line 2384) (column 55) (len 16)))))
    (reference r897 (scope relative) (span (offset 130970) (line 2436) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 130970) (line 2436) (column 40) (len 19)))))
    (reference r898 (scope relative) (span (offset 132325) (line 2449) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 132325) (line 2449) (column 28) (len 4)))))
    (reference r899 (scope relative) (span (offset 132320) (line 2449) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 132320) (line 2449) (column 23) (len 3)))))
    (reference r900 (scope relative) (span (offset 132359) (line 2450) (column 29) (len 16)) (segments (segment 0 (token "AbsorbedDoseUnit") (name "AbsorbedDoseUnit") (separator none) (span (offset 132359) (line 2450) (column 29) (len 16)))))
    (reference r901 (scope relative) (span (offset 132353) (line 2450) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 132353) (line 2450) (column 23) (len 4)))))
    (reference r902 (scope relative) (span (offset 132506) (line 2455) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 132506) (line 2455) (column 39) (len 11)))))
    (reference r903 (scope relative) (span (offset 132556) (line 2456) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 132556) (line 2456) (column 37) (len 19)))))
    (reference r904 (scope relative) (span (offset 132585) (line 2456) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 132585) (line 2456) (column 66) (len 8)))))
    (reference r905 (scope relative) (span (offset 132596) (line 2456) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 132596) (line 2456) (column 77) (len 3)))))
    (reference r906 (scope relative) (span (offset 132600) (line 2456) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 132600) (line 2456) (column 81) (len 1)))))
    (reference r907 (scope relative) (span (offset 132607) (line 2456) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 132607) (line 2456) (column 88) (len 8)))))
    (reference r908 (scope relative) (span (offset 132661) (line 2457) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 132661) (line 2457) (column 39) (len 19)))))
    (reference r909 (scope relative) (span (offset 132690) (line 2457) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 132690) (line 2457) (column 68) (len 8)))))
    (reference r910 (scope relative) (span (offset 132701) (line 2457) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 132701) (line 2457) (column 79) (len 3)))))
    (reference r911 (scope relative) (span (offset 132705) (line 2457) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 132705) (line 2457) (column 83) (len 1)))))
    (reference r912 (scope relative) (span (offset 132712) (line 2457) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 132712) (line 2457) (column 90) (len 8)))))
    (reference r913 (scope relative) (span (offset 132751) (line 2458) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 132751) (line 2458) (column 23) (len 17)))))
    (reference r914 (scope relative) (span (offset 132775) (line 2458) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 132775) (line 2458) (column 47) (len 20)))))
    (reference r915 (scope relative) (span (offset 132799) (line 2458) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 132799) (line 2458) (column 71) (len 8)))))
    (reference r916 (scope relative) (span (offset 132809) (line 2458) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 132809) (line 2458) (column 81) (len 10)))))
    (reference r917 (scope relative) (span (offset 133822) (line 2478) (column 61) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 133822) (line 2478) (column 61) (len 19)))))
    (reference r918 (scope relative) (span (offset 134806) (line 2491) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 134806) (line 2491) (column 28) (len 4)))))
    (reference r919 (scope relative) (span (offset 134801) (line 2491) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 134801) (line 2491) (column 23) (len 3)))))
    (reference r920 (scope relative) (span (offset 134840) (line 2492) (column 29) (len 37)) (segments (segment 0 (token "QualityFactorForIonizingRadiationUnit") (name "QualityFactorForIonizingRadiationUnit") (separator none) (span (offset 134840) (line 2492) (column 29) (len 37)))))
    (reference r921 (scope relative) (span (offset 134834) (line 2492) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 134834) (line 2492) (column 23) (len 4)))))
    (reference r922 (scope relative) (span (offset 135071) (line 2497) (column 60) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 135071) (line 2497) (column 60) (len 16)))))
    (reference r923 (scope relative) (span (offset 135190) (line 2501) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 135190) (line 2501) (column 42) (len 19)))))
    (reference r924 (scope relative) (span (offset 136829) (line 2514) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 136829) (line 2514) (column 28) (len 4)))))
    (reference r925 (scope relative) (span (offset 136824) (line 2514) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 136824) (line 2514) (column 23) (len 3)))))
    (reference r926 (scope relative) (span (offset 136863) (line 2515) (column 29) (len 18)) (segments (segment 0 (token "DoseEquivalentUnit") (name "DoseEquivalentUnit") (separator none) (span (offset 136863) (line 2515) (column 29) (len 18)))))
    (reference r927 (scope relative) (span (offset 136857) (line 2515) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 136857) (line 2515) (column 23) (len 4)))))
    (reference r928 (scope relative) (span (offset 137018) (line 2520) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 137018) (line 2520) (column 41) (len 11)))))
    (reference r929 (scope relative) (span (offset 137068) (line 2521) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 137068) (line 2521) (column 37) (len 19)))))
    (reference r930 (scope relative) (span (offset 137097) (line 2521) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 137097) (line 2521) (column 66) (len 8)))))
    (reference r931 (scope relative) (span (offset 137108) (line 2521) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 137108) (line 2521) (column 77) (len 3)))))
    (reference r932 (scope relative) (span (offset 137112) (line 2521) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 137112) (line 2521) (column 81) (len 1)))))
    (reference r933 (scope relative) (span (offset 137119) (line 2521) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 137119) (line 2521) (column 88) (len 8)))))
    (reference r934 (scope relative) (span (offset 137173) (line 2522) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 137173) (line 2522) (column 39) (len 19)))))
    (reference r935 (scope relative) (span (offset 137202) (line 2522) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 137202) (line 2522) (column 68) (len 8)))))
    (reference r936 (scope relative) (span (offset 137213) (line 2522) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 137213) (line 2522) (column 79) (len 3)))))
    (reference r937 (scope relative) (span (offset 137217) (line 2522) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 137217) (line 2522) (column 83) (len 1)))))
    (reference r938 (scope relative) (span (offset 137224) (line 2522) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 137224) (line 2522) (column 90) (len 8)))))
    (reference r939 (scope relative) (span (offset 137263) (line 2523) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 137263) (line 2523) (column 23) (len 17)))))
    (reference r940 (scope relative) (span (offset 137287) (line 2523) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 137287) (line 2523) (column 47) (len 20)))))
    (reference r941 (scope relative) (span (offset 137311) (line 2523) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 137311) (line 2523) (column 71) (len 8)))))
    (reference r942 (scope relative) (span (offset 137321) (line 2523) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 137321) (line 2523) (column 81) (len 10)))))
    (reference r943 (scope relative) (span (offset 138139) (line 2543) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 138139) (line 2543) (column 44) (len 19)))))
    (reference r944 (scope relative) (span (offset 138956) (line 2556) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 138956) (line 2556) (column 28) (len 4)))))
    (reference r945 (scope relative) (span (offset 138951) (line 2556) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 138951) (line 2556) (column 23) (len 3)))))
    (reference r946 (scope relative) (span (offset 138990) (line 2557) (column 29) (len 20)) (segments (segment 0 (token "AbsorbedDoseRateUnit") (name "AbsorbedDoseRateUnit") (separator none) (span (offset 138990) (line 2557) (column 29) (len 20)))))
    (reference r947 (scope relative) (span (offset 138984) (line 2557) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 138984) (line 2557) (column 23) (len 4)))))
    (reference r948 (scope relative) (span (offset 139153) (line 2562) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 139153) (line 2562) (column 43) (len 11)))))
    (reference r949 (scope relative) (span (offset 139203) (line 2563) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 139203) (line 2563) (column 37) (len 19)))))
    (reference r950 (scope relative) (span (offset 139232) (line 2563) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 139232) (line 2563) (column 66) (len 8)))))
    (reference r951 (scope relative) (span (offset 139243) (line 2563) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 139243) (line 2563) (column 77) (len 3)))))
    (reference r952 (scope relative) (span (offset 139247) (line 2563) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 139247) (line 2563) (column 81) (len 1)))))
    (reference r953 (scope relative) (span (offset 139254) (line 2563) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 139254) (line 2563) (column 88) (len 8)))))
    (reference r954 (scope relative) (span (offset 139308) (line 2564) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 139308) (line 2564) (column 39) (len 19)))))
    (reference r955 (scope relative) (span (offset 139337) (line 2564) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 139337) (line 2564) (column 68) (len 8)))))
    (reference r956 (scope relative) (span (offset 139348) (line 2564) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 139348) (line 2564) (column 79) (len 3)))))
    (reference r957 (scope relative) (span (offset 139352) (line 2564) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 139352) (line 2564) (column 83) (len 1)))))
    (reference r958 (scope relative) (span (offset 139359) (line 2564) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 139359) (line 2564) (column 90) (len 8)))))
    (reference r959 (scope relative) (span (offset 139398) (line 2565) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 139398) (line 2565) (column 23) (len 17)))))
    (reference r960 (scope relative) (span (offset 139422) (line 2565) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 139422) (line 2565) (column 47) (len 20)))))
    (reference r961 (scope relative) (span (offset 139446) (line 2565) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 139446) (line 2565) (column 71) (len 8)))))
    (reference r962 (scope relative) (span (offset 139456) (line 2565) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 139456) (line 2565) (column 81) (len 10)))))
    (reference r963 (scope relative) (span (offset 139582) (line 2569) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 139582) (line 2569) (column 48) (len 19)))))
    (reference r964 (scope relative) (span (offset 140613) (line 2582) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 140613) (line 2582) (column 28) (len 4)))))
    (reference r965 (scope relative) (span (offset 140608) (line 2582) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 140608) (line 2582) (column 23) (len 3)))))
    (reference r966 (scope relative) (span (offset 140647) (line 2583) (column 29) (len 24)) (segments (segment 0 (token "LinearEnergyTransferUnit") (name "LinearEnergyTransferUnit") (separator none) (span (offset 140647) (line 2583) (column 29) (len 24)))))
    (reference r967 (scope relative) (span (offset 140641) (line 2583) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 140641) (line 2583) (column 23) (len 4)))))
    (reference r968 (scope relative) (span (offset 140826) (line 2588) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 140826) (line 2588) (column 47) (len 11)))))
    (reference r969 (scope relative) (span (offset 140876) (line 2589) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 140876) (line 2589) (column 37) (len 19)))))
    (reference r970 (scope relative) (span (offset 140905) (line 2589) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 140905) (line 2589) (column 66) (len 8)))))
    (reference r971 (scope relative) (span (offset 140916) (line 2589) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 140916) (line 2589) (column 77) (len 3)))))
    (reference r972 (scope relative) (span (offset 140920) (line 2589) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 140920) (line 2589) (column 81) (len 1)))))
    (reference r973 (scope relative) (span (offset 140927) (line 2589) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 140927) (line 2589) (column 88) (len 8)))))
    (reference r974 (scope relative) (span (offset 140977) (line 2590) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 140977) (line 2590) (column 35) (len 19)))))
    (reference r975 (scope relative) (span (offset 141006) (line 2590) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 141006) (line 2590) (column 64) (len 8)))))
    (reference r976 (scope relative) (span (offset 141017) (line 2590) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 141017) (line 2590) (column 75) (len 3)))))
    (reference r977 (scope relative) (span (offset 141021) (line 2590) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 141021) (line 2590) (column 79) (len 1)))))
    (reference r978 (scope relative) (span (offset 141028) (line 2590) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 141028) (line 2590) (column 86) (len 8)))))
    (reference r979 (scope relative) (span (offset 141082) (line 2591) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 141082) (line 2591) (column 39) (len 19)))))
    (reference r980 (scope relative) (span (offset 141111) (line 2591) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 141111) (line 2591) (column 68) (len 8)))))
    (reference r981 (scope relative) (span (offset 141122) (line 2591) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 141122) (line 2591) (column 79) (len 3)))))
    (reference r982 (scope relative) (span (offset 141126) (line 2591) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 141126) (line 2591) (column 83) (len 1)))))
    (reference r983 (scope relative) (span (offset 141133) (line 2591) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 141133) (line 2591) (column 90) (len 8)))))
    (reference r984 (scope relative) (span (offset 141172) (line 2592) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 141172) (line 2592) (column 23) (len 17)))))
    (reference r985 (scope relative) (span (offset 141196) (line 2592) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 141196) (line 2592) (column 47) (len 20)))))
    (reference r986 (scope relative) (span (offset 141220) (line 2592) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 141220) (line 2592) (column 71) (len 8)))))
    (reference r987 (scope relative) (span (offset 141230) (line 2592) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 141230) (line 2592) (column 81) (len 6)))))
    (reference r988 (scope relative) (span (offset 141238) (line 2592) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 141238) (line 2592) (column 89) (len 10)))))
    (reference r989 (scope relative) (span (offset 141334) (line 2596) (column 33) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 141334) (line 2596) (column 33) (len 19)))))
    (reference r990 (scope relative) (span (offset 142881) (line 2609) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 142881) (line 2609) (column 28) (len 4)))))
    (reference r991 (scope relative) (span (offset 142876) (line 2609) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 142876) (line 2609) (column 23) (len 3)))))
    (reference r992 (scope relative) (span (offset 142915) (line 2610) (column 29) (len 9)) (segments (segment 0 (token "KermaUnit") (name "KermaUnit") (separator none) (span (offset 142915) (line 2610) (column 29) (len 9)))))
    (reference r993 (scope relative) (span (offset 142909) (line 2610) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 142909) (line 2610) (column 23) (len 4)))))
    (reference r994 (scope relative) (span (offset 143034) (line 2615) (column 32) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 143034) (line 2615) (column 32) (len 11)))))
    (reference r995 (scope relative) (span (offset 143084) (line 2616) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 143084) (line 2616) (column 37) (len 19)))))
    (reference r996 (scope relative) (span (offset 143113) (line 2616) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 143113) (line 2616) (column 66) (len 8)))))
    (reference r997 (scope relative) (span (offset 143124) (line 2616) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 143124) (line 2616) (column 77) (len 3)))))
    (reference r998 (scope relative) (span (offset 143128) (line 2616) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 143128) (line 2616) (column 81) (len 1)))))
    (reference r999 (scope relative) (span (offset 143135) (line 2616) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 143135) (line 2616) (column 88) (len 8)))))
    (reference r1000 (scope relative) (span (offset 143189) (line 2617) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 143189) (line 2617) (column 39) (len 19)))))
    (reference r1001 (scope relative) (span (offset 143218) (line 2617) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 143218) (line 2617) (column 68) (len 8)))))
    (reference r1002 (scope relative) (span (offset 143229) (line 2617) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 143229) (line 2617) (column 79) (len 3)))))
    (reference r1003 (scope relative) (span (offset 143233) (line 2617) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 143233) (line 2617) (column 83) (len 1)))))
    (reference r1004 (scope relative) (span (offset 143240) (line 2617) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 143240) (line 2617) (column 90) (len 8)))))
    (reference r1005 (scope relative) (span (offset 143279) (line 2618) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 143279) (line 2618) (column 23) (len 17)))))
    (reference r1006 (scope relative) (span (offset 143303) (line 2618) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 143303) (line 2618) (column 47) (len 20)))))
    (reference r1007 (scope relative) (span (offset 143327) (line 2618) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 143327) (line 2618) (column 71) (len 8)))))
    (reference r1008 (scope relative) (span (offset 143337) (line 2618) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 143337) (line 2618) (column 81) (len 10)))))
    (reference r1009 (scope relative) (span (offset 143442) (line 2622) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 143442) (line 2622) (column 37) (len 19)))))
    (reference r1010 (scope relative) (span (offset 144214) (line 2635) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 144214) (line 2635) (column 28) (len 4)))))
    (reference r1011 (scope relative) (span (offset 144209) (line 2635) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 144209) (line 2635) (column 23) (len 3)))))
    (reference r1012 (scope relative) (span (offset 144248) (line 2636) (column 29) (len 13)) (segments (segment 0 (token "KermaRateUnit") (name "KermaRateUnit") (separator none) (span (offset 144248) (line 2636) (column 29) (len 13)))))
    (reference r1013 (scope relative) (span (offset 144242) (line 2636) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 144242) (line 2636) (column 23) (len 4)))))
    (reference r1014 (scope relative) (span (offset 144383) (line 2641) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 144383) (line 2641) (column 36) (len 11)))))
    (reference r1015 (scope relative) (span (offset 144433) (line 2642) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 144433) (line 2642) (column 37) (len 19)))))
    (reference r1016 (scope relative) (span (offset 144462) (line 2642) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 144462) (line 2642) (column 66) (len 8)))))
    (reference r1017 (scope relative) (span (offset 144473) (line 2642) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 144473) (line 2642) (column 77) (len 3)))))
    (reference r1018 (scope relative) (span (offset 144477) (line 2642) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 144477) (line 2642) (column 81) (len 1)))))
    (reference r1019 (scope relative) (span (offset 144484) (line 2642) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 144484) (line 2642) (column 88) (len 8)))))
    (reference r1020 (scope relative) (span (offset 144538) (line 2643) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 144538) (line 2643) (column 39) (len 19)))))
    (reference r1021 (scope relative) (span (offset 144567) (line 2643) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 144567) (line 2643) (column 68) (len 8)))))
    (reference r1022 (scope relative) (span (offset 144578) (line 2643) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 144578) (line 2643) (column 79) (len 3)))))
    (reference r1023 (scope relative) (span (offset 144582) (line 2643) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 144582) (line 2643) (column 83) (len 1)))))
    (reference r1024 (scope relative) (span (offset 144589) (line 2643) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 144589) (line 2643) (column 90) (len 8)))))
    (reference r1025 (scope relative) (span (offset 144628) (line 2644) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 144628) (line 2644) (column 23) (len 17)))))
    (reference r1026 (scope relative) (span (offset 144652) (line 2644) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 144652) (line 2644) (column 47) (len 20)))))
    (reference r1027 (scope relative) (span (offset 144676) (line 2644) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 144676) (line 2644) (column 71) (len 8)))))
    (reference r1028 (scope relative) (span (offset 144686) (line 2644) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 144686) (line 2644) (column 81) (len 10)))))
    (reference r1029 (scope relative) (span (offset 144831) (line 2648) (column 57) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 144831) (line 2648) (column 57) (len 19)))))
    (reference r1030 (scope relative) (span (offset 146947) (line 2661) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 146947) (line 2661) (column 28) (len 4)))))
    (reference r1031 (scope relative) (span (offset 146942) (line 2661) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 146942) (line 2661) (column 23) (len 3)))))
    (reference r1032 (scope relative) (span (offset 146981) (line 2662) (column 29) (len 33)) (segments (segment 0 (token "MassEnergyTransferCoefficientUnit") (name "MassEnergyTransferCoefficientUnit") (separator none) (span (offset 146981) (line 2662) (column 29) (len 33)))))
    (reference r1033 (scope relative) (span (offset 146975) (line 2662) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 146975) (line 2662) (column 23) (len 4)))))
    (reference r1034 (scope relative) (span (offset 147196) (line 2667) (column 56) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 147196) (line 2667) (column 56) (len 11)))))
    (reference r1035 (scope relative) (span (offset 147246) (line 2668) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 147246) (line 2668) (column 37) (len 19)))))
    (reference r1036 (scope relative) (span (offset 147275) (line 2668) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 147275) (line 2668) (column 66) (len 8)))))
    (reference r1037 (scope relative) (span (offset 147286) (line 2668) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 147286) (line 2668) (column 77) (len 3)))))
    (reference r1038 (scope relative) (span (offset 147290) (line 2668) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 147290) (line 2668) (column 81) (len 1)))))
    (reference r1039 (scope relative) (span (offset 147297) (line 2668) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 147297) (line 2668) (column 88) (len 8)))))
    (reference r1040 (scope relative) (span (offset 147347) (line 2669) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 147347) (line 2669) (column 35) (len 19)))))
    (reference r1041 (scope relative) (span (offset 147376) (line 2669) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 147376) (line 2669) (column 64) (len 8)))))
    (reference r1042 (scope relative) (span (offset 147387) (line 2669) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 147387) (line 2669) (column 75) (len 3)))))
    (reference r1043 (scope relative) (span (offset 147391) (line 2669) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 147391) (line 2669) (column 79) (len 1)))))
    (reference r1044 (scope relative) (span (offset 147398) (line 2669) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 147398) (line 2669) (column 86) (len 8)))))
    (reference r1045 (scope relative) (span (offset 147437) (line 2670) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 147437) (line 2670) (column 23) (len 17)))))
    (reference r1046 (scope relative) (span (offset 147461) (line 2670) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 147461) (line 2670) (column 47) (len 20)))))
    (reference r1047 (scope relative) (span (offset 147485) (line 2670) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 147485) (line 2670) (column 71) (len 8)))))
    (reference r1048 (scope relative) (span (offset 147495) (line 2670) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 147495) (line 2670) (column 81) (len 6)))))
    (reference r1049 (scope relative) (span (offset 147591) (line 2674) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 147591) (line 2674) (column 36) (len 19)))))
    (reference r1050 (scope relative) (span (offset 149627) (line 2687) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 149627) (line 2687) (column 28) (len 4)))))
    (reference r1051 (scope relative) (span (offset 149622) (line 2687) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 149622) (line 2687) (column 23) (len 3)))))
    (reference r1052 (scope relative) (span (offset 149661) (line 2688) (column 29) (len 12)) (segments (segment 0 (token "ExposureUnit") (name "ExposureUnit") (separator none) (span (offset 149661) (line 2688) (column 29) (len 12)))))
    (reference r1053 (scope relative) (span (offset 149655) (line 2688) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 149655) (line 2688) (column 23) (len 4)))))
    (reference r1054 (scope relative) (span (offset 149792) (line 2693) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 149792) (line 2693) (column 35) (len 11)))))
    (reference r1055 (scope relative) (span (offset 149840) (line 2694) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 149840) (line 2694) (column 35) (len 19)))))
    (reference r1056 (scope relative) (span (offset 149869) (line 2694) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 149869) (line 2694) (column 64) (len 8)))))
    (reference r1057 (scope relative) (span (offset 149880) (line 2694) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 149880) (line 2694) (column 75) (len 3)))))
    (reference r1058 (scope relative) (span (offset 149884) (line 2694) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 149884) (line 2694) (column 79) (len 1)))))
    (reference r1059 (scope relative) (span (offset 149891) (line 2694) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 149891) (line 2694) (column 86) (len 8)))))
    (reference r1060 (scope relative) (span (offset 149946) (line 2695) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 149946) (line 2695) (column 39) (len 19)))))
    (reference r1061 (scope relative) (span (offset 149975) (line 2695) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 149975) (line 2695) (column 68) (len 8)))))
    (reference r1062 (scope relative) (span (offset 149986) (line 2695) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 149986) (line 2695) (column 79) (len 3)))))
    (reference r1063 (scope relative) (span (offset 149990) (line 2695) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 149990) (line 2695) (column 83) (len 1)))))
    (reference r1064 (scope relative) (span (offset 149997) (line 2695) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 149997) (line 2695) (column 90) (len 8)))))
    (reference r1065 (scope relative) (span (offset 150058) (line 2696) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 150058) (line 2696) (column 46) (len 19)))))
    (reference r1066 (scope relative) (span (offset 150087) (line 2696) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 150087) (line 2696) (column 75) (len 8)))))
    (reference r1067 (scope relative) (span (offset 150098) (line 2696) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 150098) (line 2696) (column 86) (len 3)))))
    (reference r1068 (scope relative) (span (offset 150102) (line 2696) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 150102) (line 2696) (column 90) (len 1)))))
    (reference r1069 (scope relative) (span (offset 150109) (line 2696) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 150109) (line 2696) (column 97) (len 8)))))
    (reference r1070 (scope relative) (span (offset 150147) (line 2697) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 150147) (line 2697) (column 23) (len 17)))))
    (reference r1071 (scope relative) (span (offset 150171) (line 2697) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 150171) (line 2697) (column 47) (len 20)))))
    (reference r1072 (scope relative) (span (offset 150195) (line 2697) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 150195) (line 2697) (column 71) (len 6)))))
    (reference r1073 (scope relative) (span (offset 150203) (line 2697) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 150203) (line 2697) (column 79) (len 10)))))
    (reference r1074 (scope relative) (span (offset 150215) (line 2697) (column 91) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 150215) (line 2697) (column 91) (len 17)))))
    (reference r1075 (scope relative) (span (offset 150331) (line 2701) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 150331) (line 2701) (column 40) (len 19)))))
    (reference r1076 (scope relative) (span (offset 151089) (line 2714) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 151089) (line 2714) (column 28) (len 4)))))
    (reference r1077 (scope relative) (span (offset 151084) (line 2714) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 151084) (line 2714) (column 23) (len 3)))))
    (reference r1078 (scope relative) (span (offset 151123) (line 2715) (column 29) (len 16)) (segments (segment 0 (token "ExposureRateUnit") (name "ExposureRateUnit") (separator none) (span (offset 151123) (line 2715) (column 29) (len 16)))))
    (reference r1079 (scope relative) (span (offset 151117) (line 2715) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 151117) (line 2715) (column 23) (len 4)))))
    (reference r1080 (scope relative) (span (offset 151270) (line 2720) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 151270) (line 2720) (column 39) (len 11)))))
    (reference r1081 (scope relative) (span (offset 151318) (line 2721) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 151318) (line 2721) (column 35) (len 19)))))
    (reference r1082 (scope relative) (span (offset 151347) (line 2721) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 151347) (line 2721) (column 64) (len 8)))))
    (reference r1083 (scope relative) (span (offset 151358) (line 2721) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 151358) (line 2721) (column 75) (len 3)))))
    (reference r1084 (scope relative) (span (offset 151362) (line 2721) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 151362) (line 2721) (column 79) (len 1)))))
    (reference r1085 (scope relative) (span (offset 151369) (line 2721) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 151369) (line 2721) (column 86) (len 8)))))
    (reference r1086 (scope relative) (span (offset 151431) (line 2722) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 151431) (line 2722) (column 46) (len 19)))))
    (reference r1087 (scope relative) (span (offset 151460) (line 2722) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 151460) (line 2722) (column 75) (len 8)))))
    (reference r1088 (scope relative) (span (offset 151471) (line 2722) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 151471) (line 2722) (column 86) (len 3)))))
    (reference r1089 (scope relative) (span (offset 151475) (line 2722) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 151475) (line 2722) (column 90) (len 1)))))
    (reference r1090 (scope relative) (span (offset 151482) (line 2722) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 151482) (line 2722) (column 97) (len 8)))))
    (reference r1091 (scope relative) (span (offset 151520) (line 2723) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 151520) (line 2723) (column 23) (len 17)))))
    (reference r1092 (scope relative) (span (offset 151544) (line 2723) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 151544) (line 2723) (column 47) (len 20)))))
    (reference r1093 (scope relative) (span (offset 151568) (line 2723) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 151568) (line 2723) (column 71) (len 6)))))
    (reference r1094 (scope relative) (span (offset 151576) (line 2723) (column 79) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 151576) (line 2723) (column 79) (len 17)))))
  )
  (root (library-package (name "ISQAtomicNuclear") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 58) (line 3) (column 7) (len 720)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO-80000-10:2019 \"Atomic and nuclear physics\"\nsee also https://www.iso.org/standard/64980.html\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\nNote 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \ndefined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \nor TensorMeasurementReference.\n"))) (import (target (span (span (offset 801) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 840) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 850) (line 16) (column 30) (len 3))) (separator (span (offset 850) (line 16) (column 30) (len 2))) (marker (span (offset 852) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 874) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 895) (line 17) (column 41) (len 3))) (separator (span (offset 895) (line 17) (column 41) (len 2))) (marker (span (offset 897) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 919) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 926) (line 18) (column 27) (len 3))) (separator (span (offset 926) (line 18) (column 27) (len 2))) (marker (span (offset 928) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 938) (line 20) (column 7) (len 57)) (normalized "Quantity definitions referenced from other ISQ packages "))) (import (target (span (span (offset 1017) (line 21) (column 20) (len 47))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1085) (line 22) (column 20) (len 48))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1154) (line 23) (column 20) (len 43))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1222) (line 24) (column 20) (len 40))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1283) (line 25) (column 20) (len 35))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1339) (line 26) (column 20) (len 23))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1383) (line 27) (column 20) (len 30))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1422) (line 29) (column 7) (len 55)) (normalized "ISO-80000-10 item 10-1.1 atomic number, proton number "))) (attribute-usage) (alias (name "protonNumber") (target (ref r11)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2368) (line 47) (column 7) (len 41)) (normalized "ISO-80000-10 item 10-1.2 neutron number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2991) (line 63) (column 7) (len 54)) (normalized "ISO-80000-10 item 10-1.3 nucleon number, mass number "))) (attribute-usage) (alias (name "massNumber") (target (ref r12)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3623) (line 81) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-2 rest mass, proper mass "))) (attribute-usage) (alias (name "properMass") (target (ref r13)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4416) (line 99) (column 7) (len 36)) (normalized "ISO-80000-10 item 10-3 rest energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5071) (line 115) (column 7) (len 38)) (normalized "ISO-80000-10 item 10-4.1 atomic mass "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5738) (line 131) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-4.2 nuclidic mass "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 6368) (line 147) (column 7) (len 55)) (normalized "ISO-80000-10 item 10-4.3 unified atomic mass constant "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7081) (line 163) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-5.1 elementary charge "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7723) (line 179) (column 7) (len 59)) (normalized "ISO-80000-10 item 10-5.2 charge number, ionization number "))) (attribute-def (declaration-name "ChargeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7866) (line 182) (column 11) (len 748)) (normalized "source: item 10-5.2 charge number, ionization number\nsymbol(s): `c`\napplication domain: generic\nname: ChargeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: for a particle, quotient of the electric charge (IEC 80000-6) and the elementary charge (ISO 80000-1)\nremarks: A particle is said to be electrically neutral if its charge number is equal to zero. The charge number of a particle can be positive, negative, or zero. The state of charge of a particle may be presented as a superscript to the symbol of that particle, e.g. `H^+, He^(++), Al^(3+), Cl^-, S^(--), N^(3-)`.\n"))))) (attribute-usage) (alias (name "ionizationNumber") (target (ref r15)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 8743) (line 198) (column 7) (len 36)) (normalized "ISO-80000-10 item 10-6 Bohr radius "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9689) (line 214) (column 7) (len 41)) (normalized "ISO-80000-10 item 10-7 Rydberg constant "))) (attribute-def (declaration-name "RydbergConstantValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9819) (line 217) (column 11) (len 732)) (normalized "source: item 10-7 Rydberg constant\nsymbol(s): `R_∞`\napplication domain: generic\nname: RydbergConstant\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: spectroscopic constant that determines the wave numbers of the lines in the spectrum of hydrogen: `R_(oo) = e^2/(8 π ε_0 a_0 h c_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), `a_0` is the Bohr radius (item 10-6), `h` is the Planck constant (ISO 80000-1), and `c_0` is the speed of light in vacuum (ISO 80000-1)\nremarks: The quantity `R_y = R_∞ h c_0` is called the Rydberg energy.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "RydbergConstantUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10864) (line 235) (column 77) (len 5)) (member-access (base (expression (span (offset 10864) (line 235) (column 77) (len 3)) (ref r24))) (separator dot) (member (ref r25))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10886) (line 235) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 10887) (line 235) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10961) (line 236) (column 70) (len 8)) (ref r29))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 10986) (line 239) (column 7) (len 39)) (normalized "ISO-80000-10 item 10-8 Hartree energy "))) (attribute-def (declaration-name "HartreeEnergyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 11112) (line 242) (column 11) (len 635)) (normalized "source: item 10-8 Hartree energy\nsymbol(s): `E_H`, `E_h`\napplication domain: generic\nname: HartreeEnergy\nquantity dimension: L^6*M^3*T^-6\nmeasurement unit(s): eV*J*kg*m^2*s^-2\ntensor order: 0\ndefinition: energy (ISO 80000-5) of the electron in a hydrogen atom in its ground state: `E_H = e^2/(4 π ε_0 a_0)` where `e` is the elementary charge (ISO 80000-1), `ε_0` is the electric constant (IEC 80000-6), and `a_0` is the Bohr radius (item 10-6)\nremarks: The energy of the electron in an H atom in its ground state is `E_H`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "HartreeEnergyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r37)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12052) (line 260) (column 77) (len 5)) (member-access (base (expression (span (offset 12052) (line 260) (column 77) (len 3)) (ref r38))) (separator dot) (member (ref r39))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12074) (line 260) (column 99) (len 1)) (integer 6))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r41)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12153) (line 261) (column 75) (len 5)) (member-access (base (expression (span (offset 12153) (line 261) (column 75) (len 3)) (ref r43))) (separator dot) (member (ref r44))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12175) (line 261) (column 97) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12258) (line 262) (column 79) (len 5)) (member-access (base (expression (span (offset 12258) (line 262) (column 79) (len 3)) (ref r48))) (separator dot) (member (ref r49))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12280) (line 262) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 12281) (line 262) (column 102) (len 1)) (integer 6)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12355) (line 263) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 12356) (line 263) (column 71) (len 8)) (ref r53))) (element comma (expression (span (offset 12366) (line 263) (column 81) (len 6)) (ref r54))) (element comma (expression (span (offset 12374) (line 263) (column 89) (len 10)) (ref r55))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 12402) (line 266) (column 7) (len 49)) (normalized "ISO-80000-10 item 10-9.1 magnetic dipole moment "))) (attribute-def (declaration-name "MagneticDipoleMomentValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r56)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12545) (line 269) (column 11) (len 906)) (normalized "source: item 10-9.1 magnetic dipole moment (magnitude)\nsymbol(s): `μ`\napplication domain: atomic physics\nname: MagneticDipoleMoment\nquantity dimension: L^2*I^1\nmeasurement unit(s): m^2*A\ntensor order: 0\ndefinition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`\nremarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r59)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MagneticDipoleMomentUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r61)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r62)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r63)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13784) (line 287) (column 77) (len 5)) (member-access (base (expression (span (offset 13784) (line 287) (column 77) (len 3)) (ref r64))) (separator dot) (member (ref r65))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13806) (line 287) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r67)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r68)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13896) (line 288) (column 86) (len 5)) (member-access (base (expression (span (offset 13896) (line 288) (column 86) (len 3)) (ref r69))) (separator dot) (member (ref r70))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13918) (line 288) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r72)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r73)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13992) (line 289) (column 70) (len 29)) (sequence (sequence-list (element first (expression (span (offset 13993) (line 289) (column 71) (len 8)) (ref r74))) (element comma (expression (span (offset 14003) (line 289) (column 81) (len 17)) (ref r75))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianMagneticDipoleMoment3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r76)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14139) (line 294) (column 11) (len 908)) (normalized "source: item 10-9.1 magnetic dipole moment (vector)\nsymbol(s): `vec(μ)`\napplication domain: atomic physics\nname: MagneticDipoleMoment\nquantity dimension: L^2*I^1\nmeasurement unit(s): m^2*A\ntensor order: 1\ndefinition: for a particle, vector (ISO 80000-2) quantity causing a change to its energy (ISO 80000-5) `ΔW` in an external magnetic field of field flux density `vec(B)` (IEC 80000-6): `ΔW` = -`vec(μ)` · `vec(B)`\nremarks: For an atom or nucleus, this energy is quantized and can be written as: `W` = `g μ_x M B` where `g` is the appropriate `g` factor (item 10-14.1 or item 10-14.2), `μ_x` is mostly the Bohr magneton or nuclear magneton (item 10-9.2 or item 10-9.3), `M` is magnetic quantum number (item 10-13.4), and `B` is magnitude of the magnetic flux density. See also IEC 80000-6.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r77)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15082) (line 305) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r78)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r79)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianMagneticDipoleMoment3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r80)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r81)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15410) (line 312) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r82)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15454) (line 313) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r83)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r84)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 15531) (line 317) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-9.2 Bohr magneton "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 16359) (line 333) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-9.3 nuclear magneton "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 17294) (line 349) (column 7) (len 30)) (normalized "ISO-80000-10 item 10-10 spin "))) (attribute-def (declaration-name "SpinValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r85)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 17402) (line 352) (column 11) (len 459)) (normalized "source: item 10-10 spin (magnitude)\nsymbol(s): `s`\napplication domain: generic\nname: Spin\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): kg*m^2*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system\nremarks: Spin is an additive vector quantity.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r86)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r87)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r88)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SpinUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r90)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r91)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r92)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18130) (line 370) (column 77) (len 5)) (member-access (base (expression (span (offset 18130) (line 370) (column 77) (len 3)) (ref r93))) (separator dot) (member (ref r94))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r95)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18152) (line 370) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r96)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r97)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18231) (line 371) (column 75) (len 5)) (member-access (base (expression (span (offset 18231) (line 371) (column 75) (len 3)) (ref r98))) (separator dot) (member (ref r99))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18253) (line 371) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r101)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r102)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18336) (line 372) (column 79) (len 5)) (member-access (base (expression (span (offset 18336) (line 372) (column 79) (len 3)) (ref r103))) (separator dot) (member (ref r104))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r105)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18358) (line 372) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 18359) (line 372) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r106)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r107)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18433) (line 373) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 18434) (line 373) (column 71) (len 8)) (ref r108))) (element comma (expression (span (offset 18444) (line 373) (column 81) (len 6)) (ref r109))) (element comma (expression (span (offset 18452) (line 373) (column 89) (len 10)) (ref r110))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianSpin3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r111)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 18565) (line 378) (column 11) (len 461)) (normalized "source: item 10-10 spin (vector)\nsymbol(s): `vec(s)`\napplication domain: generic\nname: Spin\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): kg*m^2*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity expressing the internal angular momentum (ISO 80000-4) of a particle or a particle system\nremarks: Spin is an additive vector quantity.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r112)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19061) (line 389) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r113)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r114)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianSpin3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r115)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r116)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19325) (line 396) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r117)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19369) (line 397) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r118)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r119)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 19430) (line 401) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-11 total angular momentum "))) (attribute-def (declaration-name "TotalAngularMomentumValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r120)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19572) (line 404) (column 11) (len 945)) (normalized "source: item 10-11 total angular momentum (magnitude)\nsymbol(s): `J`\napplication domain: generic\nname: TotalAngularMomentum\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): J*s*eV*s, kg*m^2*s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)\nremarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r121)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r122)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r123)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r124)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "TotalAngularMomentumUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r125)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r126)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r127)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20850) (line 422) (column 77) (len 5)) (member-access (base (expression (span (offset 20850) (line 422) (column 77) (len 3)) (ref r128))) (separator dot) (member (ref r129))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r130)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20872) (line 422) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r131)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r132)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20951) (line 423) (column 75) (len 5)) (member-access (base (expression (span (offset 20951) (line 423) (column 75) (len 3)) (ref r133))) (separator dot) (member (ref r134))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r135)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20973) (line 423) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r136)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r137)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21056) (line 424) (column 79) (len 5)) (member-access (base (expression (span (offset 21056) (line 424) (column 79) (len 3)) (ref r138))) (separator dot) (member (ref r139))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r140)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21078) (line 424) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 21079) (line 424) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r141)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r142)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21153) (line 425) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 21154) (line 425) (column 71) (len 8)) (ref r143))) (element comma (expression (span (offset 21164) (line 425) (column 81) (len 6)) (ref r144))) (element comma (expression (span (offset 21172) (line 425) (column 89) (len 10)) (ref r145))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianTotalAngularMomentum3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r146)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 21301) (line 430) (column 11) (len 947)) (normalized "source: item 10-11 total angular momentum (vector)\nsymbol(s): `vec(J)`\napplication domain: generic\nname: TotalAngularMomentum\nquantity dimension: L^2*M^1*T^-1\nmeasurement unit(s): J*s*eV*s, kg*m^2*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity in a quantum system composed of the vectorial sum of angular momentum `vec(L)` (ISO 80000-4) and spin `vec(s)` (item 10-10)\nremarks: In atomic and nuclear physics, orbital angular momentum is usually denoted by `vec(l)` or `vec(L)`. The magnitude of `vec(J)` is quantized so that: `J^2 = ℏ^2 j (j+1)` where `j` is the total angular momentum quantum number (item 10-13.6). Total angular momentum and magnetic dipole moment have the same direction. `j` is not the magnitude of the total angular momentum `vec(J)` but its projection onto the quantization axis, divided by `ℏ`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22283) (line 441) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r148)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r149)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianTotalAngularMomentum3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r150)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r151)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22611) (line 448) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r152)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22655) (line 449) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r153)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r154)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 22732) (line 453) (column 7) (len 92)) (normalized "ISO-80000-10 item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient "))) (attribute-def (declaration-name "GyromagneticRatioValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r155)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 22915) (line 456) (column 11) (len 871)) (normalized "source: item 10-12.1 gyromagnetic ratio, magnetogyric ratio, gyromagnetic coefficient\nsymbol(s): `γ`\napplication domain: generic\nname: GyromagneticRatio\nquantity dimension: M^-1*T^1*I^1\nmeasurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A\ntensor order: 0\ndefinition: proportionality constant between the magnetic dipole moment and the angular momentum: `vec(μ)` = `γ` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)\nremarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1 The systematic name is \"gyromagnetic coefficient\", but \"gyromagnetic ratio\" is more usual. The gyromagnetic ratio of the proton is denoted by `γ_p`. The gyromagnetic ratio of the neutron is denoted by `γ_n`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r156)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r157)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r158)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r159)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "GyromagneticRatioUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r160)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r161)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r162)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24105) (line 474) (column 75) (len 5)) (member-access (base (expression (span (offset 24105) (line 474) (column 75) (len 3)) (ref r163))) (separator dot) (member (ref r164))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r165)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24127) (line 474) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 24128) (line 474) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r166)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r167)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24211) (line 475) (column 79) (len 5)) (member-access (base (expression (span (offset 24211) (line 475) (column 79) (len 3)) (ref r168))) (separator dot) (member (ref r169))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r170)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24233) (line 475) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r171)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r172)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24323) (line 476) (column 86) (len 5)) (member-access (base (expression (span (offset 24323) (line 476) (column 86) (len 3)) (ref r173))) (separator dot) (member (ref r174))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r175)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24345) (line 476) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r176)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r177)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24419) (line 477) (column 70) (len 39)) (sequence (sequence-list (element first (expression (span (offset 24420) (line 477) (column 71) (len 6)) (ref r178))) (element comma (expression (span (offset 24428) (line 477) (column 79) (len 10)) (ref r179))) (element comma (expression (span (offset 24440) (line 477) (column 91) (len 17)) (ref r180))))))))) (body semicolon)))))) (alias (name "MagnetogyricRatioUnit") (target (ref r181)) (body semicolon)) (alias (name "MagnetogyricRatioValue") (target (ref r182)) (body semicolon)) (alias (name "magnetogyricRatio") (target (ref r183)) (body semicolon)) (alias (name "GyromagneticCoefficientUnit") (target (ref r184)) (body semicolon)) (alias (name "GyromagneticCoefficientValue") (target (ref r185)) (body semicolon)) (alias (name "gyromagneticCoefficient") (target (ref r186)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 24837) (line 488) (column 7) (len 140)) (normalized "ISO-80000-10 item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron "))) (attribute-def (declaration-name "GyromagneticRatioOfTheElectronValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r187)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 25081) (line 491) (column 11) (len 741)) (normalized "source: item 10-12.2 gyromagnetic ratio of the electron, magnetogyric ratio of the electron, gyromagnetic coefficient of the electron\nsymbol(s): `γ_e`\napplication domain: generic\nname: GyromagneticRatioOfTheElectron\nquantity dimension: M^-1*T^1*I^1\nmeasurement unit(s): A*m^2*J^-1*s^-1, A*s/kg, kg^-1*s*A\ntensor order: 0\ndefinition: proportionality constant between the magnetic dipole moment and the angular momentum of the electron `vec(μ)` = `γ_e` `vec(J)` where `vec(μ)` is the magnetic dipole moment (item 10-9.1), and `vec(J)` is the total angular momentum (item 10-11)\nremarks: 1 A·m^2/(J·s) = 1 A·s/kg = 1 T^-1·s^-1\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r188)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r189)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r190)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r191)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "GyromagneticRatioOfTheElectronUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r192)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r193)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r194)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26193) (line 509) (column 75) (len 5)) (member-access (base (expression (span (offset 26193) (line 509) (column 75) (len 3)) (ref r195))) (separator dot) (member (ref r196))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r197)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26215) (line 509) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 26216) (line 509) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r198)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r199)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26299) (line 510) (column 79) (len 5)) (member-access (base (expression (span (offset 26299) (line 510) (column 79) (len 3)) (ref r200))) (separator dot) (member (ref r201))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26321) (line 510) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r203)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r204)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26411) (line 511) (column 86) (len 5)) (member-access (base (expression (span (offset 26411) (line 511) (column 86) (len 3)) (ref r205))) (separator dot) (member (ref r206))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r207)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26433) (line 511) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r208)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r209)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26507) (line 512) (column 70) (len 39)) (sequence (sequence-list (element first (expression (span (offset 26508) (line 512) (column 71) (len 6)) (ref r210))) (element comma (expression (span (offset 26516) (line 512) (column 79) (len 10)) (ref r211))) (element comma (expression (span (offset 26528) (line 512) (column 91) (len 17)) (ref r212))))))))) (body semicolon)))))) (alias (name "MagnetogyricRatioOfTheElectronUnit") (target (ref r213)) (body semicolon)) (alias (name "MagnetogyricRatioOfTheElectronValue") (target (ref r214)) (body semicolon)) (alias (name "magnetogyricRatioOfTheElectron") (target (ref r215)) (body semicolon)) (alias (name "GyromagneticCoefficientOfTheElectronUnit") (target (ref r216)) (body semicolon)) (alias (name "GyromagneticCoefficientOfTheElectronValue") (target (ref r217)) (body semicolon)) (alias (name "gyromagneticCoefficientOfTheElectron") (target (ref r218)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 27081) (line 523) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-13.1 quantum number "))) (attribute-def (declaration-name "QuantumNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r219)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 27208) (line 526) (column 11) (len 1147)) (normalized "source: item 10-13.1 quantum number\nsymbol(s): `N`, `L`, `M`, `j`, `s`, `F`\napplication domain: generic\nname: QuantumNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: number describing a particular state of a quantum system\nremarks: Electron states determine the binding energy `E = E(n,l,m,j,s,f)` in an atom. Upper case letters `N, L, M, J, S, F` are usually used for the whole system. The spatial probability distribution of an electron is given by `|Ψ|^2`, where `Ψ` is its wave function. For an electron in an H atom in a non-relativistic approximation, the wave function can be presented as: `Ψ(r,θ,φ) = R_(nl)(r)*Y_l^m(θ,φ)`, where `r,θ,φ` are spherical coordinates (ISO 80000-2) with respect to the nucleus and to a given (quantization) axis, `R_(nl)(r)` is the radial distribution function, and `Y_l^m(θ,φ)` are spherical harmonics. In the Bohr model of one-electron atoms, `n`, `l`, and `m` define the possible orbits of an electron about the nucleus.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28440) (line 540) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-13.2 principal quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 29322) (line 556) (column 7) (len 67)) (normalized "ISO-80000-10 item 10-13.3 orbital angular momentum quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 30423) (line 572) (column 7) (len 51)) (normalized "ISO-80000-10 item 10-13.4 magnetic quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 31398) (line 588) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-13.5 spin quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 32136) (line 604) (column 7) (len 65)) (normalized "ISO-80000-10 item 10-13.6 total angular momentum quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 33034) (line 620) (column 7) (len 55)) (normalized "ISO-80000-10 item 10-13.7 nuclear spin quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 34087) (line 636) (column 7) (len 62)) (normalized "ISO-80000-10 item 10-13.8 hyperfine structure quantum number "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 35003) (line 652) (column 7) (len 58)) (normalized "ISO-80000-10 item 10-14.1 Lande factor, g factor of atom "))) (attribute-def (declaration-name "LandeFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r220)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 35144) (line 655) (column 11) (len 881)) (normalized "source: item 10-14.1 Lande factor, g factor of atom\nsymbol(s): `g`\napplication domain: generic\nname: LandeFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the magnetic dipole moment of an atom, and the product of the total angular momentum quantum number and the Bohr magneton: `g = μ/(J*μ_B)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `J` is total angular momentum quantum number (item 10-13.6), and `μ_B` is the Bohr magneton (item 10-9.2)\nremarks: These quantities are also called `g` values. The Landé factor can be calculated from the expression: `g(L, S, J) = 1 + (g_e -1) xx (J(J+1) + S(S+1) - L(L+1))/(2J(J+1))` where `g_e` is the` g` factor of the electron.\n"))))) (attribute-usage) (alias (name "gFactorOfAtom") (target (ref r221)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 36148) (line 671) (column 7) (len 67)) (normalized "ISO-80000-10 item 10-14.2 g factor of nucleus or nuclear particle "))) (attribute-def (declaration-name "GFactorOfNucleusOrNuclearParticleValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r222)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36320) (line 674) (column 11) (len 749)) (normalized "source: item 10-14.2 g factor of nucleus or nuclear particle\nsymbol(s): `g`\napplication domain: generic\nname: GFactorOfNucleusOrNuclearParticle (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the magnetic dipole moment of an atom, and the product of the nuclear spin quantum number and the nuclear magneton: `g = μ/(I*μ_N)` where `μ` is magnitude of magnetic dipole moment (item 10-9.1), `I` is nuclear spin quantum number (item 10-13.7), and `μ_N` is the nuclear magneton (item 10-9.3)\nremarks: The `g` factors for nuclei or nucleons are known from measurements.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 37194) (line 688) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-15.1 Larmor angular frequency "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38043) (line 704) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-15.2 Larmor frequency "))) (attribute-def (declaration-name "LarmorFrequencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r223)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38176) (line 707) (column 11) (len 368)) (normalized "source: item 10-15.2 Larmor frequency\nsymbol(s): `ν_L`\napplication domain: generic\nname: LarmorFrequency\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: quotient of Larmor angular frequency (ISO 80000-3) and 2π\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r224)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r225)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r226)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r227)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LarmorFrequencyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r228)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r229)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r230)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38859) (line 725) (column 79) (len 5)) (member-access (base (expression (span (offset 38859) (line 725) (column 79) (len 3)) (ref r231))) (separator dot) (member (ref r232))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r233)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38881) (line 725) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 38882) (line 725) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r234)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r235)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38956) (line 726) (column 70) (len 10)) (ref r236))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38983) (line 729) (column 7) (len 64)) (normalized "ISO-80000-10 item 10-15.3 nuclear precession angular frequency "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 39826) (line 745) (column 7) (len 53)) (normalized "ISO-80000-10 item 10-16 cyclotron angular frequency "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 40793) (line 761) (column 7) (len 51)) (normalized "ISO-80000-10 item 10-17 gyroradius, Larmor radius "))) (attribute-usage) (alias (name "larmorRadius") (target (ref r237)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 41592) (line 779) (column 7) (len 51)) (normalized "ISO-80000-10 item 10-18 nuclear quadrupole moment "))) (attribute-def (declaration-name "NuclearQuadrupoleMomentValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r238)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 41740) (line 782) (column 11) (len 822)) (normalized "source: item 10-18 nuclear quadrupole moment\nsymbol(s): `Q`\napplication domain: generic\nname: NuclearQuadrupoleMoment\nquantity dimension: L^2\nmeasurement unit(s): m^2\ntensor order: 0\ndefinition: `z` component of the diagonalized tensor of nuclear quadrupole moment: `Q = (1/e) int (3z^2 - r^2) ρ(x, y, z) dV` in the quantum state with the nuclear spin in the field direction (`z`), where `e` is the elementary charge (ISO 80000-1), `r^2 = x^2 + y^2 + z^2`, `ρ(x,y,z)` is the nuclear electric charge density (IEC 80000-6), and `dV` is the volume element `dx dy dz`\nremarks: The electric nuclear quadrupole moment is `eQ`. This value is equal to the `z` component of the diagonalized tensor of quadrupole moment.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r239)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r240)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r241)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r242)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "NuclearQuadrupoleMomentUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r243)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r244)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r245)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42907) (line 800) (column 77) (len 5)) (member-access (base (expression (span (offset 42907) (line 800) (column 77) (len 3)) (ref r246))) (separator dot) (member (ref r247))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r248)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42929) (line 800) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r249)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r250)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43003) (line 801) (column 70) (len 8)) (ref r251))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 43028) (line 804) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-19.1 nuclear radius "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 43838) (line 820) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-19.2 electron radius "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 44887) (line 836) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-20 Compton wavelength "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45810) (line 852) (column 7) (len 39)) (normalized "ISO-80000-10 item 10-21.1 mass excess "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 46619) (line 868) (column 7) (len 39)) (normalized "ISO-80000-10 item 10-21.2 mass defect "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 47668) (line 884) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-22.1 relative mass excess "))) (attribute-def (declaration-name "RelativeMassExcessValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r252)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 47806) (line 887) (column 11) (len 525)) (normalized "source: item 10-22.1 relative mass excess\nsymbol(s): `Δ_r`\napplication domain: generic\nname: RelativeMassExcess (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mass excess and the unified atomic mass constant: `Δ_r = Δ/m_u` where `Δ` is mass excess (item 10-21.1), and `m_u` is the unified atomic mass constant (item 10-4.3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 48426) (line 901) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-22.2 relative mass defect "))) (attribute-def (declaration-name "RelativeMassDefectValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r253)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 48564) (line 904) (column 11) (len 521)) (normalized "source: item 10-22.2 relative mass defect\nsymbol(s): `B_r`\napplication domain: generic\nname: RelativeMassDefect (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mass defect and the unified atomic mass constant: `B_r = B/m_u` where `B` is mass defect (item 10-21.2), and `m_u` is the unified atomic mass constant (item 10-4.3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49180) (line 918) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-23.1 packing fraction "))) (attribute-def (declaration-name "PackingFractionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r254)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 49311) (line 921) (column 11) (len 503)) (normalized "source: item 10-23.1 packing fraction\nsymbol(s): `f`\napplication domain: generic\nname: PackingFraction (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of relative mass excess and the nucleon number: `f` = Δ_r/A` where `Δ_r` is relative mass excess (item 10-22.1), and `A` is the nucleon number (item 10-1.3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49903) (line 935) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-23.2 binding fraction "))) (attribute-def (declaration-name "BindingFractionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r255)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50034) (line 938) (column 11) (len 500)) (normalized "source: item 10-23.2 binding fraction\nsymbol(s): `b`\napplication domain: generic\nname: BindingFraction (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of relative mass defect and the nucleon number: `b = B_r/A` where `B_r` is relative mass defect (item 10-22.2), and `A` is the nucleon number (item 10-1.3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 50623) (line 952) (column 7) (len 65)) (normalized "ISO-80000-10 item 10-24 decay constant, disintegration constant "))) (attribute-def (declaration-name "DecayConstantValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r256)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50775) (line 955) (column 11) (len 769)) (normalized "source: item 10-24 decay constant, disintegration constant\nsymbol(s): `λ`\napplication domain: generic\nname: DecayConstant\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: quotient of `(-dN)/N` and `dt`, where `(dN)/N` is the mean fractional change in the number of nuclei in a particular energy state due to spontaneous transformations in a time interval of duration (ISO 80000-3) `dt`: `λ = -1/N (dN)/(dt)`\nremarks: For exponential decay, this quantity is constant. For more than one decay channel, `λ = sum λ_a` where `λ_a` denotes the decay constant for a specified final state and the sum is taken over all final states.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r257)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r258)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r259)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r260)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DecayConstantUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r261)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r262)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r263)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51851) (line 973) (column 79) (len 5)) (member-access (base (expression (span (offset 51851) (line 973) (column 79) (len 3)) (ref r264))) (separator dot) (member (ref r265))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r266)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51873) (line 973) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 51874) (line 973) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r267)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r268)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51948) (line 974) (column 70) (len 10)) (ref r269))))) (body semicolon)))))) (alias (name "DisintegrationConstantUnit") (target (ref r270)) (body semicolon)) (alias (name "DisintegrationConstantValue") (target (ref r271)) (body semicolon)) (alias (name "disintegrationConstant") (target (ref r272)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 52150) (line 981) (column 7) (len 63)) (normalized "ISO-80000-10 item 10-25 mean duration of life, mean life time "))) (attribute-usage) (alias (name "meanLifeTime") (target (ref r273)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 53012) (line 999) (column 7) (len 37)) (normalized "ISO-80000-10 item 10-26 level width "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 54190) (line 1015) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-27 nuclear activity "))) (attribute-def (declaration-name "NuclearActivityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r274)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 54321) (line 1018) (column 11) (len 1140)) (normalized "source: item 10-27 nuclear activity\nsymbol(s): `A`\napplication domain: generic\nname: NuclearActivity\nquantity dimension: T^-1\nmeasurement unit(s): Bq, s^-1\ntensor order: 0\ndefinition: differential quotient of `N` with respect to time, where `N` is the mean change in the number of nuclei in a particular energy state due to spontaneous nuclear transformations in a time interval of duration (ISO 80000-3) `dt`: `A = -(dN)/(dt)`\nremarks: For exponential decay, `A = λN`, where `λ` is the decay constant (item 10-24). The becquerel (Bq) is a special name for second to the power minus one, to be used as the coherent SI unit of activity. In report 85a of the ICRU a definition with an equivalent meaning is given as: The activity, `A`, of an amount of a radionuclide in a particular energy state at a given time is the quotient of `-dN` by `dt`, where `dN` is the mean change in the number of nuclei in that energy state due to spontaneous nuclear transformations in the time interval `dt`: `A = -(dN)/(dt)`. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r275)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r277)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r278)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "NuclearActivityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r279)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r280)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r281)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 55776) (line 1036) (column 79) (len 5)) (member-access (base (expression (span (offset 55776) (line 1036) (column 79) (len 3)) (ref r282))) (separator dot) (member (ref r283))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r284)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 55798) (line 1036) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 55799) (line 1036) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r285)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r286)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 55873) (line 1037) (column 70) (len 10)) (ref r287))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 55900) (line 1040) (column 7) (len 60)) (normalized "ISO-80000-10 item 10-28 specific activity, massic activity "))) (attribute-def (declaration-name "SpecificActivityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r288)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 56050) (line 1043) (column 11) (len 452)) (normalized "source: item 10-28 specific activity, massic activity\nsymbol(s): `a`\napplication domain: generic\nname: SpecificActivity\nquantity dimension: M^-1*T^-1\nmeasurement unit(s): Bq/kg, kg^-1*s^-1\ntensor order: 0\ndefinition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r289)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r290)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r291)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r292)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SpecificActivityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r293)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r294)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r295)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56817) (line 1061) (column 75) (len 5)) (member-access (base (expression (span (offset 56817) (line 1061) (column 75) (len 3)) (ref r296))) (separator dot) (member (ref r297))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r298)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56839) (line 1061) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 56840) (line 1061) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r299)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r300)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56923) (line 1062) (column 79) (len 5)) (member-access (base (expression (span (offset 56923) (line 1062) (column 79) (len 3)) (ref r301))) (separator dot) (member (ref r302))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r303)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56945) (line 1062) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 56946) (line 1062) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r304)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r305)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57020) (line 1063) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 57021) (line 1063) (column 71) (len 6)) (ref r306))) (element comma (expression (span (offset 57029) (line 1063) (column 79) (len 10)) (ref r307))))))))) (body semicolon)))))) (alias (name "MassicActivityUnit") (target (ref r308)) (body semicolon)) (alias (name "MassicActivityValue") (target (ref r309)) (body semicolon)) (alias (name "massicActivity") (target (ref r310)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 57217) (line 1070) (column 7) (len 84)) (normalized "ISO-80000-10 item 10-29 activity density, volumic activity, activity concentration "))) (attribute-def (declaration-name "ActivityDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r311)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 57390) (line 1073) (column 11) (len 477)) (normalized "source: item 10-29 activity density, volumic activity, activity concentration\nsymbol(s): `c_A`\napplication domain: generic\nname: ActivityDensity\nquantity dimension: L^-3*T^-1\nmeasurement unit(s): Bq/m^3, m^-3*s^-1\ntensor order: 0\ndefinition: quotient of the activity `A` (item 10-27) of a sample and the mass `m` (ISO 80000-4) of that sample: `a = A/m`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r312)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r313)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r314)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r315)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ActivityDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r316)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r317)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r318)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58180) (line 1091) (column 77) (len 5)) (member-access (base (expression (span (offset 58180) (line 1091) (column 77) (len 3)) (ref r319))) (separator dot) (member (ref r320))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r321)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58202) (line 1091) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 58203) (line 1091) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r322)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r323)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58286) (line 1092) (column 79) (len 5)) (member-access (base (expression (span (offset 58286) (line 1092) (column 79) (len 3)) (ref r324))) (separator dot) (member (ref r325))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r326)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58308) (line 1092) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 58309) (line 1092) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r327)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r328)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58383) (line 1093) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 58384) (line 1093) (column 71) (len 8)) (ref r329))) (element comma (expression (span (offset 58394) (line 1093) (column 81) (len 10)) (ref r330))))))))) (body semicolon)))))) (alias (name "VolumicActivityUnit") (target (ref r331)) (body semicolon)) (alias (name "VolumicActivityValue") (target (ref r332)) (body semicolon)) (alias (name "volumicActivity") (target (ref r333)) (body semicolon)) (alias (name "ActivityConcentrationUnit") (target (ref r334)) (body semicolon)) (alias (name "ActivityConcentrationValue") (target (ref r335)) (body semicolon)) (alias (name "activityConcentration") (target (ref r336)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 58760) (line 1104) (column 7) (len 50)) (normalized "ISO-80000-10 item 10-30 surface-activity density "))) (attribute-def (declaration-name "SurfaceActivityDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r337)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 58906) (line 1107) (column 11) (len 597)) (normalized "source: item 10-30 surface-activity density\nsymbol(s): `a_S`\napplication domain: generic\nname: SurfaceActivityDensity\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): Bq/m^2, m^-2*s^-1\ntensor order: 0\ndefinition: quotient of the activity `A` (item 10-27) of a sample and the total area `S` (ISO 80000-3) of the surface of that sample: `a_S` = `A`/`S`\nremarks: This value is usually defined for flat sources, where `S` corresponds to the total area of surface of one side of the source.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r338)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r339)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r340)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r341)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SurfaceActivityDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r342)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r343)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r344)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59844) (line 1125) (column 77) (len 5)) (member-access (base (expression (span (offset 59844) (line 1125) (column 77) (len 3)) (ref r345))) (separator dot) (member (ref r346))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r347)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59866) (line 1125) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 59867) (line 1125) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r348)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r349)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59950) (line 1126) (column 79) (len 5)) (member-access (base (expression (span (offset 59950) (line 1126) (column 79) (len 3)) (ref r350))) (separator dot) (member (ref r351))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r352)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59972) (line 1126) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 59973) (line 1126) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r353)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r354)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60047) (line 1127) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 60048) (line 1127) (column 71) (len 8)) (ref r355))) (element comma (expression (span (offset 60058) (line 1127) (column 81) (len 10)) (ref r356))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 60086) (line 1130) (column 7) (len 35)) (normalized "ISO-80000-10 item 10-31 half life "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 60709) (line 1146) (column 7) (len 53)) (normalized "ISO-80000-10 item 10-32 alpha disintegration energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 61637) (line 1162) (column 7) (len 54)) (normalized "ISO-80000-10 item 10-33 maximum beta-particle energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 62369) (line 1178) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-34 beta disintegration energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 63441) (line 1194) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-35 internal conversion factor "))) (attribute-def (declaration-name "InternalConversionFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r357)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 63589) (line 1197) (column 11) (len 848)) (normalized "source: item 10-35 internal conversion factor\nsymbol(s): `α`\napplication domain: generic\nname: InternalConversionFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the number of internal conversion electrons and the number of gamma quanta emitted by the radioactive atom in a given transition, where a conversion electron represents an orbital electron emitted through the radioactive decay\nremarks: The quantity `α/(α+1)` is also used and called the internal-conversion fraction. Partial conversion fractions referring to the various electron shells `K, L, ...` are indicated by `α_K`, `α_L`, ... `α_K/α_L` is called the K-to-L internal conversion ratio.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 64544) (line 1211) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-36 particle emission rate "))) (attribute-def (declaration-name "ParticleEmissionRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r358)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 64686) (line 1214) (column 11) (len 649)) (normalized "source: item 10-36 particle emission rate\nsymbol(s): `dot(N)`\napplication domain: generic\nname: ParticleEmissionRate\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: differential quotient of `N` with respect to time, where `N` is the number of particles being emitted from an infinitesimally small volume element in the time interval of duration `dt` (ISO 80000-3), and `dt`: `dot(N) = (dN)/(dt)`\nremarks: Usually the kind of particles is specified, e.g. neutron emission rate or alpha particle emission rate.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r359)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r360)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r361)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r362)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleEmissionRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r363)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r364)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r365)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65670) (line 1232) (column 79) (len 5)) (member-access (base (expression (span (offset 65670) (line 1232) (column 79) (len 3)) (ref r366))) (separator dot) (member (ref r367))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r368)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65692) (line 1232) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 65693) (line 1232) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r369)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r370)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65767) (line 1233) (column 70) (len 10)) (ref r371))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 65794) (line 1236) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-37.1 reaction energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 66556) (line 1252) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-37.2 resonance energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 67309) (line 1268) (column 7) (len 41)) (normalized "ISO-80000-10 item 10-38.1 cross section "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 68217) (line 1284) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-38.2 total cross section "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 69129) (line 1300) (column 7) (len 65)) (normalized "ISO-80000-10 item 10-39 direction distribution of cross section "))) (attribute-def (declaration-name "DirectionDistributionOfCrossSectionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r372)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 69303) (line 1303) (column 11) (len 763)) (normalized "source: item 10-39 direction distribution of cross section\nsymbol(s): `σ_Ω`\napplication domain: atomic physics\nname: DirectionDistributionOfCrossSection\nquantity dimension: L^2\nmeasurement unit(s): m^2*sr^-1, m^2\ntensor order: 0\ndefinition: differential quotient of `σ` with respect to `Ω`, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a specified direction, and `Ω` is the solid angle (ISO 80000-3) around that direction: `σ_Ω = (dσ)/(dΩ)`\nremarks: Quantities listed under items 10-39, 10-40 and 10-41 are sometimes called differential cross sections. The type of interaction needs to be specified.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r373)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r374)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r375)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r376)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DirectionDistributionOfCrossSectionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r377)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r378)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r379)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70459) (line 1321) (column 77) (len 5)) (member-access (base (expression (span (offset 70459) (line 1321) (column 77) (len 3)) (ref r380))) (separator dot) (member (ref r381))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r382)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70481) (line 1321) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r384)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70555) (line 1322) (column 70) (len 8)) (ref r385))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 70580) (line 1325) (column 7) (len 62)) (normalized "ISO-80000-10 item 10-40 energy distribution of cross section "))) (attribute-def (declaration-name "EnergyDistributionOfCrossSectionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r386)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 70748) (line 1328) (column 11) (len 606)) (normalized "source: item 10-40 energy distribution of cross section\nsymbol(s): `σ_E`\napplication domain: atomic physics\nname: EnergyDistributionOfCrossSection\nquantity dimension: M^-1*T^2\nmeasurement unit(s): m^2/J, kg^-1*s^2\ntensor order: 0\ndefinition: differential quotient of `σ` with respect to energy, where `σ` is the cross section (item 10-38.1) for a process in which the energy `E` (ISO 80000-5) of the ejected or scattered particle is between `E` and `E + dE`: `σ_E = (dσ)/(dE)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r387)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r388)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r389)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r390)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "EnergyDistributionOfCrossSectionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r391)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r392)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r393)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71733) (line 1346) (column 75) (len 5)) (member-access (base (expression (span (offset 71733) (line 1346) (column 75) (len 3)) (ref r394))) (separator dot) (member (ref r395))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r396)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71755) (line 1346) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 71756) (line 1346) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r397)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r398)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71839) (line 1347) (column 79) (len 5)) (member-access (base (expression (span (offset 71839) (line 1347) (column 79) (len 3)) (ref r399))) (separator dot) (member (ref r400))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r401)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71861) (line 1347) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r402)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r403)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71935) (line 1348) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 71936) (line 1348) (column 71) (len 6)) (ref r404))) (element comma (expression (span (offset 71944) (line 1348) (column 79) (len 10)) (ref r405))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 71972) (line 1351) (column 7) (len 76)) (normalized "ISO-80000-10 item 10-41 direction and energy distribution of cross section "))) (attribute-def (declaration-name "DirectionAndEnergyDistributionOfCrossSectionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r406)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 72166) (line 1354) (column 11) (len 705)) (normalized "source: item 10-41 direction and energy distribution of cross section\nsymbol(s): `σ_(Ω,E)`\napplication domain: atomic physics\nname: DirectionAndEnergyDistributionOfCrossSection\nquantity dimension: M^-1*T^2\nmeasurement unit(s): m^2/(J*sr), kg^-1*s^2\ntensor order: 0\ndefinition: partial differential quotient of `σ` with respect to solid angle and energy, where `σ` is the cross section (item 10-38.1) for ejecting or scattering a particle into a solid angle `dΩ` around a specified direction and with an energy between `E` and `E+dE`: `σ_(Ω,E) = (del^2 σ) / (del Ω del E)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r407)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r409)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r410)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DirectionAndEnergyDistributionOfCrossSectionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r411)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r412)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r413)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73298) (line 1372) (column 75) (len 5)) (member-access (base (expression (span (offset 73298) (line 1372) (column 75) (len 3)) (ref r414))) (separator dot) (member (ref r415))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r416)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73320) (line 1372) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 73321) (line 1372) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r417)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r418)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73404) (line 1373) (column 79) (len 5)) (member-access (base (expression (span (offset 73404) (line 1373) (column 79) (len 3)) (ref r419))) (separator dot) (member (ref r420))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r421)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73426) (line 1373) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r422)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r423)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 73500) (line 1374) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 73501) (line 1374) (column 71) (len 6)) (ref r424))) (element comma (expression (span (offset 73509) (line 1374) (column 79) (len 10)) (ref r425))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 73537) (line 1377) (column 7) (len 76)) (normalized "ISO-80000-10 item 10-42.1 volumic cross section, macroscopic cross section "))) (attribute-def (declaration-name "VolumicCrossSectionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r426)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 73706) (line 1380) (column 11) (len 594)) (normalized "source: item 10-42.1 volumic cross section, macroscopic cross section\nsymbol(s): `Σ`\napplication domain: atomic physics\nname: VolumicCrossSection\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: product of the number density `n_a` of the atoms and of the cross section (item 10-38.1) `σ_a` for a given type of atoms: `Σ = n_a σ_a`\nremarks: When the target particles of the medium are at rest, `Σ = 1/l`, where `l` is the mean free path (item 10-71).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r427)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r428)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r429)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r430)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "VolumicCrossSectionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r431)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r432)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r433)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 74629) (line 1398) (column 77) (len 5)) (member-access (base (expression (span (offset 74629) (line 1398) (column 77) (len 3)) (ref r434))) (separator dot) (member (ref r435))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r436)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 74651) (line 1398) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 74652) (line 1398) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r437)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r438)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 74726) (line 1399) (column 70) (len 8)) (ref r439))))) (body semicolon)))))) (alias (name "MacroscopicCrossSectionUnit") (target (ref r440)) (body semicolon)) (alias (name "MacroscopicCrossSectionValue") (target (ref r441)) (body semicolon)) (alias (name "macroscopicCrossSection") (target (ref r442)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 74947) (line 1406) (column 7) (len 88)) (normalized "ISO-80000-10 item 10-42.2 volumic total cross section, macroscopic total cross section "))) (attribute-def (declaration-name "VolumicTotalCrossSectionValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r443)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 75133) (line 1409) (column 11) (len 559)) (normalized "source: item 10-42.2 volumic total cross section, macroscopic total cross section\nsymbol(s): `Σ_\"tot\"`, `Σ_\"T\"`\napplication domain: atomic physics\nname: VolumicTotalCrossSection\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: product of the number density `n_a` of the atoms and the cross section (item 10-38.1) `σ_\"tot\"` for a given type of atoms: `Σ_\"tot\" = n_a*σ_\"tot\"`\nremarks: See the Remarks for item 10-49.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r444)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r445)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r446)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r447)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "VolumicTotalCrossSectionUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r448)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r449)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r450)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 76041) (line 1427) (column 77) (len 5)) (member-access (base (expression (span (offset 76041) (line 1427) (column 77) (len 3)) (ref r451))) (separator dot) (member (ref r452))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r453)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 76063) (line 1427) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 76064) (line 1427) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r454)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r455)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 76138) (line 1428) (column 70) (len 8)) (ref r456))))) (body semicolon)))))) (alias (name "MacroscopicTotalCrossSectionUnit") (target (ref r457)) (body semicolon)) (alias (name "MacroscopicTotalCrossSectionValue") (target (ref r458)) (body semicolon)) (alias (name "macroscopicTotalCrossSection") (target (ref r459)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 76389) (line 1435) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-43 particle fluence "))) (attribute-def (declaration-name "ParticleFluenceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r460)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 76520) (line 1438) (column 11) (len 1152)) (normalized "source: item 10-43 particle fluence\nsymbol(s): `Φ`\napplication domain: generic\nname: ParticleFluence\nquantity dimension: L^-2\nmeasurement unit(s): m^-2\ntensor order: 0\ndefinition: differential quotient of `N` with respect to `a`, where `N` is the number of particles incident on a sphere of cross-sectional area `a` (item 10-38.1): `Φ = (dN)/(da)`\nremarks: The word \"particle\" is usually replaced by the name of a specific particle, for example `proton` fluence. If a flat area of size `dA` is passed perpendicularly by a number of `dN` particles, the corresponding particle fluence is: `Φ = (dN)/(dA)`. A plane area of size `dA` crossed at an angle `α` with respect to the surface normal by a number of `dN` particles results in the particle fluence: `Φ = (dN)/(cos(α) dA)` In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence, `Φ` , is the quotient of `dN` and `da`, where `dN` is the number of particles incident on a sphere of cross-sectional area `da`: `Φ = (dN)/(dA)`. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r461)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r462)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r463)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r464)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleFluenceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r465)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r466)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r467)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 77985) (line 1456) (column 77) (len 5)) (member-access (base (expression (span (offset 77985) (line 1456) (column 77) (len 3)) (ref r468))) (separator dot) (member (ref r469))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r470)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 78007) (line 1456) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 78008) (line 1456) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r471)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r472)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 78082) (line 1457) (column 70) (len 8)) (ref r473))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 78107) (line 1460) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-44 particle fluence rate "))) (attribute-def (declaration-name "ParticleFluenceRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r474)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 78247) (line 1463) (column 11) (len 1287)) (normalized "source: item 10-44 particle fluence rate\nsymbol(s): `dot(Φ)`\napplication domain: generic\nname: ParticleFluenceRate\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1\ntensor order: 0\ndefinition: differential quotient of fluence `Φ` (item 10-43) with respect to time (ISO 80000-3): `dot(Φ) = (dΦ)/(dA)`\nremarks: The word \"particle\" is usually replaced by the name of a specific particle, for example proton fluence rate. The distribution function expressed in terms of speed and energy, `dot(Φ)_v` and `dot(Φ)_E` , are related to by: `dot(Φ) = int dot(Φ)_v dv = int dot(Φ)_E dE`. This quantity has also been termed particle flux density. Because the word \"density\" has several connotations, the term \"fluence rate\" is preferred. For a radiation field composed of particles of velocity `v`, the fluence rate is equal to `n`·`v` where `n` is the particle number density. See Remarks for item 10-43. In report 85a of the ICRU a definition with an equivalent meaning is given as: The fluence rate, `dot(Φ)` , is the quotient of `d Φ` and `dt`, where `d Φ` is the increment of the fluence in the time interval `dt`: `dot(Φ) = (dΦ)/(dt)`. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r475)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r476)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r477)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r478)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleFluenceRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r479)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r480)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r481)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 79863) (line 1481) (column 77) (len 5)) (member-access (base (expression (span (offset 79863) (line 1481) (column 77) (len 3)) (ref r482))) (separator dot) (member (ref r483))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r484)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 79885) (line 1481) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 79886) (line 1481) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r485)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r486)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 79969) (line 1482) (column 79) (len 5)) (member-access (base (expression (span (offset 79969) (line 1482) (column 79) (len 3)) (ref r487))) (separator dot) (member (ref r488))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r489)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 79991) (line 1482) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 79992) (line 1482) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r490)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r491)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 80066) (line 1483) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 80067) (line 1483) (column 71) (len 8)) (ref r492))) (element comma (expression (span (offset 80077) (line 1483) (column 81) (len 10)) (ref r493))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 80105) (line 1486) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-45 radiant energy "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 81272) (line 1502) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-46 energy fluence "))) (attribute-def (declaration-name "EnergyFluenceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r494)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 81399) (line 1505) (column 11) (len 744)) (normalized "source: item 10-46 energy fluence\nsymbol(s): `Ψ`\napplication domain: generic\nname: EnergyFluence\nquantity dimension: M^1*T^-2\nmeasurement unit(s): eV/m^2, J/m^2, kg*s^-2\ntensor order: 0\ndefinition: differential quotient of radiant energy `R` (item 10-45) incident on a sphere of cross-sectional area (item 10-38.1) `a` with respect to that area: `Ψ = (dR)/(da)`\nremarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy fluence, `Ψ` is the quotient of `dR` and `da`, where `dR` is the radiant energy incident on a sphere of cross-sectional area `da`: `Ψ = (dR)/(da)`. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r495)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r496)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r497)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r498)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "EnergyFluenceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r499)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r500)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r501)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82446) (line 1523) (column 75) (len 5)) (member-access (base (expression (span (offset 82446) (line 1523) (column 75) (len 3)) (ref r502))) (separator dot) (member (ref r503))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r504)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82468) (line 1523) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r505)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r506)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82551) (line 1524) (column 79) (len 5)) (member-access (base (expression (span (offset 82551) (line 1524) (column 79) (len 3)) (ref r507))) (separator dot) (member (ref r508))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r509)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82573) (line 1524) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 82574) (line 1524) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r510)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r511)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82648) (line 1525) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 82649) (line 1525) (column 71) (len 6)) (ref r512))) (element comma (expression (span (offset 82657) (line 1525) (column 79) (len 10)) (ref r513))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 82685) (line 1528) (column 7) (len 45)) (normalized "ISO-80000-10 item 10-47 energy fluence rate "))) (attribute-def (declaration-name "EnergyFluenceRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r514)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 82821) (line 1531) (column 11) (len 722)) (normalized "source: item 10-47 energy fluence rate\nsymbol(s): `dot(Ψ)`\napplication domain: generic\nname: EnergyFluenceRate\nquantity dimension: M^1*T^-3\nmeasurement unit(s): W/m^2, kg*s^-3\ntensor order: 0\ndefinition: differential quotient of the energy fluence `Ψ` (item 10-46) with respect to time (ISO 80000-3): `dot(Ψ) = (d Ψ)/(dt)`\nremarks: In report 85a of the ICRU a definition with an equivalent meaning is given as: The energy-fluence rate, `dot(Ψ)` , is the quotient of `d Ψ` by `dt`, where `d Ψ` is the increment of the energy fluence in the time interval `dt`: `dot(Ψ) = (d Ψ)/(dt)`. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r515)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r516)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r517)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r518)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "EnergyFluenceRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r519)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r520)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r521)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 83862) (line 1549) (column 75) (len 5)) (member-access (base (expression (span (offset 83862) (line 1549) (column 75) (len 3)) (ref r522))) (separator dot) (member (ref r523))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r524)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 83884) (line 1549) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r525)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r526)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 83967) (line 1550) (column 79) (len 5)) (member-access (base (expression (span (offset 83967) (line 1550) (column 79) (len 3)) (ref r527))) (separator dot) (member (ref r528))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r529)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 83989) (line 1550) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 83990) (line 1550) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r530)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r531)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 84064) (line 1551) (column 70) (len 20)) (sequence (sequence-list (element first (expression (span (offset 84065) (line 1551) (column 71) (len 6)) (ref r532))) (element comma (expression (span (offset 84073) (line 1551) (column 79) (len 10)) (ref r533))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 84101) (line 1554) (column 7) (len 50)) (normalized "ISO-80000-10 item 10-48 particle current density "))) (attribute-def (declaration-name "ParticleCurrentDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r534)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 84247) (line 1557) (column 11) (len 1166)) (normalized "source: item 10-48 particle current density (magnitude)\nsymbol(s): `J`, `S`\napplication domain: generic\nname: ParticleCurrentDensity\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1\ntensor order: 0\ndefinition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively\nremarks: Usually the word \"particle\" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r535)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r536)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r537)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r538)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleCurrentDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r539)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r540)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r541)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85754) (line 1575) (column 77) (len 5)) (member-access (base (expression (span (offset 85754) (line 1575) (column 77) (len 3)) (ref r542))) (separator dot) (member (ref r543))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r544)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85776) (line 1575) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 85777) (line 1575) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r545)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r546)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85860) (line 1576) (column 79) (len 5)) (member-access (base (expression (span (offset 85860) (line 1576) (column 79) (len 3)) (ref r547))) (separator dot) (member (ref r548))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r549)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85882) (line 1576) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 85883) (line 1576) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r550)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r551)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 85957) (line 1577) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 85958) (line 1577) (column 71) (len 8)) (ref r552))) (element comma (expression (span (offset 85968) (line 1577) (column 81) (len 10)) (ref r553))))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianParticleCurrentDensity3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r554)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 86099) (line 1582) (column 11) (len 1173)) (normalized "source: item 10-48 particle current density (vector)\nsymbol(s): `vec(J)`, `vec(S)`\napplication domain: generic\nname: ParticleCurrentDensity\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1\ntensor order: 1\ndefinition: vector whose component in direction of an area normal is given by: `vec(J_n) = int Φ_Ω(θ, α) cos(θ) dΩ` where `Φ_Ω(θ, α)` is the directional distribution of the particle fluence rate (item 10-44), and ` θ` and `α` are polar and azimuthal angles, respectively\nremarks: Usually the word \"particle\" is replaced by the name of a specific particle, for example proton current. Symbol `vec(S)` is recommended when there is a possibility of confusion with the symbol `vec(J)` for electric current density. For neutron current, the symbol `vec(J)` is generally used. The distribution functions expressed in terms of speed and energy, `vec(J_v)` and `vec(J_E)`, are related to `vec(J)` by: `vec(J) = int vec(J_v) dv = int vec(J_E) dE`. The directional distribution of the particle fluence rate is also denoted as particle radiance.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r555)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 87307) (line 1593) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r556)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r557)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "CartesianParticleCurrentDensity3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r558)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r559)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 87643) (line 1600) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r560)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 87687) (line 1601) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r561)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r562)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 87766) (line 1605) (column 7) (len 56)) (normalized "ISO-80000-10 item 10-49 linear attenuation coefficient "))) (attribute-def (declaration-name "LinearAttenuationCoefficientForIonizingRadiationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r563)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 87944) (line 1608) (column 11) (len 1534)) (normalized "source: item 10-49 linear attenuation coefficient\nsymbol(s): `μ`, `μ_l`\napplication domain: ionizing radiation\nname: LinearAttenuationCoefficient\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: for uncharged particles of a given type and energy the differential quotient `n` with respect to `l,` where `n` is the fraction of `N` incoming particles that experience interactions in traversing a distance (ISO 80000-3) `l` in a given material: `μ = (dn)/(dl) = 1/N (dN)/(dl)` where `dN` is the number of particles that experience interactions in traversing `dl`\nremarks: `μ` is equal to the macroscopic total cross section `Σ_\"tot\"` for the removal of particles from the beam. Using the relation `μ_m = μ/ρ` between the linear attenuation coefficient `μ`, the mass attenuation coefficient `μ_m` (item 10-50) and the density `ρ`, the definition given for the mass attenuation coefficient in report 85a of the ICRU can be applied to the linear attenuation coefficient resulting in: The linear attenuation coefficient, `μ`, of a material, for uncharged particles of a given type and energy, is the quotient of `(dN)/N` by `dl`, where `(dN)/N` is the mean fraction of the particles that experience interactions in traversing a distance `dl` in the material: `μ = 1/(dl) (dN)/(N)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r564)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r565)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r566)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r567)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LinearAttenuationCoefficientForIonizingRadiationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r568)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r569)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r570)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 89923) (line 1626) (column 77) (len 5)) (member-access (base (expression (span (offset 89923) (line 1626) (column 77) (len 3)) (ref r571))) (separator dot) (member (ref r572))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r573)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 89945) (line 1626) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 89946) (line 1626) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r574)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r575)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 90020) (line 1627) (column 70) (len 8)) (ref r576))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 90045) (line 1630) (column 7) (len 54)) (normalized "ISO-80000-10 item 10-50 mass attenuation coefficient "))) (attribute-def (declaration-name "MassAttenuationCoefficientForIonizingRadiationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r577)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 90219) (line 1633) (column 11) (len 485)) (normalized "source: item 10-50 mass attenuation coefficient\nsymbol(s): `μ_m`\napplication domain: ionizing radiation\nname: MassAttenuationCoefficient\nquantity dimension: L^2*M^-1\nmeasurement unit(s): kg^-1*m^2\ntensor order: 0\ndefinition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the mass density `ρ` (ISO 80000-4) of the medium: `μ_m = μ/ρ`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r578)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r579)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r580)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r581)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassAttenuationCoefficientForIonizingRadiationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r582)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r583)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r584)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91141) (line 1651) (column 77) (len 5)) (member-access (base (expression (span (offset 91141) (line 1651) (column 77) (len 3)) (ref r585))) (separator dot) (member (ref r586))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r587)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91163) (line 1651) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r588)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r589)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91242) (line 1652) (column 75) (len 5)) (member-access (base (expression (span (offset 91242) (line 1652) (column 75) (len 3)) (ref r590))) (separator dot) (member (ref r591))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r592)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91264) (line 1652) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 91265) (line 1652) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r593)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r594)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91339) (line 1653) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 91340) (line 1653) (column 71) (len 8)) (ref r595))) (element comma (expression (span (offset 91350) (line 1653) (column 81) (len 6)) (ref r596))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 91374) (line 1656) (column 7) (len 55)) (normalized "ISO-80000-10 item 10-51 molar attenuation coefficient "))) (attribute-def (declaration-name "MolarAttenuationCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r597)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 91530) (line 1659) (column 11) (len 463)) (normalized "source: item 10-51 molar attenuation coefficient\nsymbol(s): `μ_c`\napplication domain: generic\nname: MolarAttenuationCoefficient\nquantity dimension: L^2*N^-1\nmeasurement unit(s): m^2*mol^-1\ntensor order: 0\ndefinition: quotient of linear attenuation coefficient `µ` (item 10-49) and the amount c (ISO 80000-9) of the medium: `μ_c = μ/c`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r598)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r599)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r600)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r601)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MolarAttenuationCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r602)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r603)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r604)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 92354) (line 1677) (column 77) (len 5)) (member-access (base (expression (span (offset 92354) (line 1677) (column 77) (len 3)) (ref r605))) (separator dot) (member (ref r606))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r607)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 92376) (line 1677) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r608)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r609)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 92468) (line 1678) (column 88) (len 5)) (member-access (base (expression (span (offset 92468) (line 1678) (column 88) (len 3)) (ref r610))) (separator dot) (member (ref r611))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r612)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 92490) (line 1678) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 92491) (line 1678) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r613)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r614)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 92565) (line 1679) (column 70) (len 31)) (sequence (sequence-list (element first (expression (span (offset 92566) (line 1679) (column 71) (len 8)) (ref r615))) (element comma (expression (span (offset 92576) (line 1679) (column 81) (len 19)) (ref r616))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 92613) (line 1682) (column 7) (len 56)) (normalized "ISO-80000-10 item 10-52 atomic attenuation coefficient "))) (attribute-def (declaration-name "AtomicAttenuationCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r617)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 92771) (line 1685) (column 11) (len 595)) (normalized "source: item 10-52 atomic attenuation coefficient\nsymbol(s): `μ_a`\napplication domain: generic\nname: AtomicAttenuationCoefficient\nquantity dimension: L^2\nmeasurement unit(s): m^2\ntensor order: 0\ndefinition: quotient of the linear attenuation coefficient `µ` (item 10-49) and the number density (item 10-62.1), `n`, of atoms in the substance: `μ_a = μ/n`\nremarks: `μ` is equal to the total cross section `σ_\"tot\"` for the removal of particles from the beam. See also item 10-38.2.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r618)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r619)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r620)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r621)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AtomicAttenuationCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r622)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r623)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r624)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93731) (line 1703) (column 77) (len 5)) (member-access (base (expression (span (offset 93731) (line 1703) (column 77) (len 3)) (ref r625))) (separator dot) (member (ref r626))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r627)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93753) (line 1703) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r628)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r629)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93827) (line 1704) (column 70) (len 8)) (ref r630))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 93852) (line 1707) (column 7) (len 46)) (normalized "ISO-80000-10 item 10-53 half-value thickness "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 94612) (line 1723) (column 7) (len 76)) (normalized "ISO-80000-10 item 10-54 total linear stopping power, linear stopping power "))) (attribute-def (declaration-name "TotalLinearStoppingPowerValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r631)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 94786) (line 1726) (column 11) (len 1584)) (normalized "source: item 10-54 total linear stopping power, linear stopping power\nsymbol(s): `S`, `S_l`\napplication domain: generic\nname: TotalLinearStoppingPower\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): eV/m, J/m, kg*m*s^-2\ntensor order: 0\ndefinition: for charged particles of a given type and energy `E_0` the differential quotient of `E` with respect to `x,` where `E` is the mean energy (ISO 80000-4) lost by the charged particles in traversing a distance (ISO 80000-3) `x` in the given material: `S = -(dE)/(dx)`\nremarks: The total linear stopping power is sometimes also called stopping power. Both electronic losses and radiative losses are included. The quotient of the total linear stopping power of a substance and that of a reference substance is called the relative linear stopping power. See also item 10-85. Using the relation `S_m = S/ρ` between the total mass stopping power `S_m` (item 10-55), the total linear stopping power `S`, and the density `ρ`, the definition given for the mass stopping in report 85a of the ICRU can be applied to that of the total linear stopping power resulting in: The linear stopping power, `S`, of a material, for charged particles of a given type and energy, is the quotient of `dE` by `dl`, where `dE` is the mean energy lost by the charged particles in traversing a distance `dl` in the material: `S = -(dE)/(dx)`. This definition has an equivalent meaning as the one given in column 4 of this item. See also section 0.3.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r632)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r633)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r634)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r635)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "TotalLinearStoppingPowerUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r636)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r637)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r638)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96719) (line 1744) (column 77) (len 5)) (member-access (base (expression (span (offset 96719) (line 1744) (column 77) (len 3)) (ref r639))) (separator dot) (member (ref r640))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r641)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96741) (line 1744) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r642)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r643)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96820) (line 1745) (column 75) (len 5)) (member-access (base (expression (span (offset 96820) (line 1745) (column 75) (len 3)) (ref r644))) (separator dot) (member (ref r645))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r646)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96842) (line 1745) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r647)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r648)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96925) (line 1746) (column 79) (len 5)) (member-access (base (expression (span (offset 96925) (line 1746) (column 79) (len 3)) (ref r649))) (separator dot) (member (ref r650))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r651)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 96947) (line 1746) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 96948) (line 1746) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r652)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r653)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97022) (line 1747) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 97023) (line 1747) (column 71) (len 8)) (ref r654))) (element comma (expression (span (offset 97033) (line 1747) (column 81) (len 6)) (ref r655))) (element comma (expression (span (offset 97041) (line 1747) (column 89) (len 10)) (ref r656))))))))) (body semicolon)))))) (alias (name "LinearStoppingPowerUnit") (target (ref r657)) (body semicolon)) (alias (name "LinearStoppingPowerValue") (target (ref r658)) (body semicolon)) (alias (name "linearStoppingPower") (target (ref r659)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 97268) (line 1754) (column 7) (len 72)) (normalized "ISO-80000-10 item 10-55 total mass stopping power, mass stopping power "))) (attribute-def (declaration-name "TotalMassStoppingPowerValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r660)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 97436) (line 1757) (column 11) (len 627)) (normalized "source: item 10-55 total mass stopping power, mass stopping power\nsymbol(s): `S_m`\napplication domain: generic\nname: TotalMassStoppingPower\nquantity dimension: L^4*T^-2\nmeasurement unit(s): eV*m^-2/kg, J*m^2/kg, m^4*s^-2\ntensor order: 0\ndefinition: quotient of the total linear stopping power `S` (item 10-54) and the mass density `ρ` (ISO 80000-4) of the material: `S_m = S/ρ`\nremarks: The quotient of total mass stopping power of a material and that of a reference material is called relative mass stopping power.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r661)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r662)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r663)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r664)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "TotalMassStoppingPowerUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r665)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r666)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r667)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98404) (line 1775) (column 77) (len 5)) (member-access (base (expression (span (offset 98404) (line 1775) (column 77) (len 3)) (ref r668))) (separator dot) (member (ref r669))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r670)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98426) (line 1775) (column 99) (len 1)) (integer 4))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r671)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r672)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98509) (line 1776) (column 79) (len 5)) (member-access (base (expression (span (offset 98509) (line 1776) (column 79) (len 3)) (ref r673))) (separator dot) (member (ref r674))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r675)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98531) (line 1776) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 98532) (line 1776) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r676)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r677)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98606) (line 1777) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 98607) (line 1777) (column 71) (len 8)) (ref r678))) (element comma (expression (span (offset 98617) (line 1777) (column 81) (len 10)) (ref r679))))))))) (body semicolon)))))) (alias (name "MassStoppingPowerUnit") (target (ref r680)) (body semicolon)) (alias (name "MassStoppingPowerValue") (target (ref r681)) (body semicolon)) (alias (name "massStoppingPower") (target (ref r682)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 98832) (line 1784) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-56 mean linear range "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 99519) (line 1800) (column 7) (len 41)) (normalized "ISO-80000-10 item 10-57 mean mass range "))) (attribute-def (declaration-name "MeanMassRangeValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r683)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 99647) (line 1803) (column 11) (len 439)) (normalized "source: item 10-57 mean mass range\nsymbol(s): `R_ρ`, `R_m`\napplication domain: generic\nname: MeanMassRange\nquantity dimension: L^-2*M^1\nmeasurement unit(s): kg*m^-2\ntensor order: 0\ndefinition: product of the mean linear range (item 10-56) `R` and the mass density `ρ` (ISO 80000-4) of the material: `R_ρ = R*ρ`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r684)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r685)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r686)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r687)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MeanMassRangeUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r688)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r689)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r690)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100391) (line 1821) (column 77) (len 5)) (member-access (base (expression (span (offset 100391) (line 1821) (column 77) (len 3)) (ref r691))) (separator dot) (member (ref r692))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r693)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100413) (line 1821) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 100414) (line 1821) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r694)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r695)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100493) (line 1822) (column 75) (len 5)) (member-access (base (expression (span (offset 100493) (line 1822) (column 75) (len 3)) (ref r696))) (separator dot) (member (ref r697))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r698)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100515) (line 1822) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r699)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r700)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100589) (line 1823) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 100590) (line 1823) (column 71) (len 8)) (ref r701))) (element comma (expression (span (offset 100600) (line 1823) (column 81) (len 6)) (ref r702))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 100624) (line 1826) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-58 linear ionization "))) (attribute-def (declaration-name "LinearIonizationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r703)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 100757) (line 1829) (column 11) (len 640)) (normalized "source: item 10-58 linear ionization\nsymbol(s): `N_{i_l}`\napplication domain: generic\nname: LinearIonization\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: differential quotient of `q` with respect to `l`, where `q` is the average total charge (IEC 80000-6) of all positive ions produced by an ionizing charged particle over a path `l` (ISO 80000-3), divided by the elementary charge, `e` (ISO 80000-1): `N_{i_l} = 1/e*(dq)/(dl)`\nremarks: Ionization due to secondary ionizing particles is included.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r704)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r705)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r706)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r707)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LinearIonizationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r708)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r709)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r710)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 101714) (line 1847) (column 77) (len 5)) (member-access (base (expression (span (offset 101714) (line 1847) (column 77) (len 3)) (ref r711))) (separator dot) (member (ref r712))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r713)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 101736) (line 1847) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 101737) (line 1847) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r714)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r715)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 101811) (line 1848) (column 70) (len 8)) (ref r716))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 101836) (line 1851) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-59 total ionization "))) (attribute-def (declaration-name "TotalIonizationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r717)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 101965) (line 1854) (column 11) (len 586)) (normalized "source: item 10-59 total ionization\nsymbol(s): `N_i`\napplication domain: generic\nname: TotalIonization (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the total mean charge of all positive ions produced by an ionizing charged particle along its entire path and along the paths of any secondary charged particles, and the elementary charge, `e` (ISO 80000-1)\nremarks: `N_i = int N_(il) dl` See item 10-58.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 102640) (line 1868) (column 7) (len 76)) (normalized "ISO-80000-10 item 10-60 average energy loss per elementary charge produced "))) (attribute-def (declaration-name "AverageEnergyLossPerElementaryChargeProducedValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r718)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 102834) (line 1871) (column 11) (len 1604)) (normalized "source: item 10-60 average energy loss per elementary charge produced\nsymbol(s): `W_i`\napplication domain: generic\nname: AverageEnergyLossPerElementaryChargeProduced\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): eV, J, kg*m^2*s^-2\ntensor order: 0\ndefinition: quotient of the initial kinetic energy `E_k` (ISO 80000-4) of an ionizing charged particle and the total ionization `N_i` (item 10-59) produced by that particle: `W_i = E_k/N_i`\nremarks: The name \"average energy loss per ion pair formed\" is usually used, although it is ambiguous. In the practical dosimetry of ionizing radiation the term `W`/`e`, the quotient of `W`, the average energy deposited in dry air per ion pair formed, and `e`, the elementary charge, is used as the factor which, when multiplied with the electric charge of one sign carried by all ion pairs formed in dry air of given mass, gives the energy deposited in this amount of dry air in the form of excitations and ionizations. In ICRU Report 85a, the mean energy expended in a gas per ion pair formed, `W`, is the quotient of `E` by `N,` where `N` is the mean total liberated charge of either sign, divided by the elementary charge when the initial kinetic energy `E` of a charged particle introduced into the gas is completely dissipated in the gas. Thus, `W` = `E`/`N`. It follows from the definition of `W` that the ions produced by bremsstrahlung or other secondary radiation emitted by the initial and secondary charged particles are included in `N`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r719)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r720)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r721)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r722)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AverageEnergyLossPerElementaryChargeProducedUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r723)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r724)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r725)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 104867) (line 1889) (column 77) (len 5)) (member-access (base (expression (span (offset 104867) (line 1889) (column 77) (len 3)) (ref r726))) (separator dot) (member (ref r727))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r728)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 104889) (line 1889) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r729)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r730)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 104968) (line 1890) (column 75) (len 5)) (member-access (base (expression (span (offset 104968) (line 1890) (column 75) (len 3)) (ref r731))) (separator dot) (member (ref r732))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r733)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 104990) (line 1890) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r734)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r735)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 105073) (line 1891) (column 79) (len 5)) (member-access (base (expression (span (offset 105073) (line 1891) (column 79) (len 3)) (ref r736))) (separator dot) (member (ref r737))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r738)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 105095) (line 1891) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 105096) (line 1891) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r739)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r740)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 105170) (line 1892) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 105171) (line 1892) (column 71) (len 8)) (ref r741))) (element comma (expression (span (offset 105181) (line 1892) (column 81) (len 6)) (ref r742))) (element comma (expression (span (offset 105189) (line 1892) (column 89) (len 10)) (ref r743))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 105217) (line 1895) (column 7) (len 34)) (normalized "ISO-80000-10 item 10-61 mobility "))) (attribute-def (declaration-name "MobilityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r744)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 105333) (line 1898) (column 11) (len 481)) (normalized "source: item 10-61 mobility\nsymbol(s): `μ`, `μ_m`\napplication domain: generic\nname: Mobility\nquantity dimension: M^-1*T^2*I^1\nmeasurement unit(s): m^2/(V*s), kg^-1*s^2*A\ntensor order: 0\ndefinition: quotient of average drift speed (ISO 80000-3) imparted to a charged particle in a medium by an electric field, and the electric field strength (IEC 80000-6)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r745)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r746)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r747)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r748)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MobilityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r749)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r750)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r751)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106097) (line 1916) (column 75) (len 5)) (member-access (base (expression (span (offset 106097) (line 1916) (column 75) (len 3)) (ref r752))) (separator dot) (member (ref r753))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r754)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106119) (line 1916) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 106120) (line 1916) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r755)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r756)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106203) (line 1917) (column 79) (len 5)) (member-access (base (expression (span (offset 106203) (line 1917) (column 79) (len 3)) (ref r757))) (separator dot) (member (ref r758))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r759)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106225) (line 1917) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r760)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r761)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106315) (line 1918) (column 86) (len 5)) (member-access (base (expression (span (offset 106315) (line 1918) (column 86) (len 3)) (ref r762))) (separator dot) (member (ref r763))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r764)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106337) (line 1918) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r765)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r766)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 106411) (line 1919) (column 70) (len 39)) (sequence (sequence-list (element first (expression (span (offset 106412) (line 1919) (column 71) (len 6)) (ref r767))) (element comma (expression (span (offset 106420) (line 1919) (column 79) (len 10)) (ref r768))) (element comma (expression (span (offset 106432) (line 1919) (column 91) (len 17)) (ref r769))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 106467) (line 1922) (column 7) (len 51)) (normalized "ISO-80000-10 item 10-62.1 particle number density "))) (attribute-def (declaration-name "ParticleNumberDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r770)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 106613) (line 1925) (column 11) (len 727)) (normalized "source: item 10-62.1 particle number density\nsymbol(s): `n`\napplication domain: generic\nname: ParticleNumberDensity\nquantity dimension: L^-3\nmeasurement unit(s): m^-3\ntensor order: 0\ndefinition: quotient of the mean number `N` of particles in the volume (ISO 80000-3) `V` and volume: `n = N/V`\nremarks: `n` is the general symbol for the number density of particles. The distribution functions expressed in terms of speed and energy, `n_v` and `n_E`, are related to `n` by: `n = int n_v dv = int n_E dE`. The word \"particle\" is usually replaced by the name of a specific particle, for example `neutron` number density.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r771)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r772)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r773)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r774)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleNumberDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r775)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r776)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r777)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 107677) (line 1943) (column 77) (len 5)) (member-access (base (expression (span (offset 107677) (line 1943) (column 77) (len 3)) (ref r778))) (separator dot) (member (ref r779))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r780)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 107699) (line 1943) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 107700) (line 1943) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r781)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r782)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 107774) (line 1944) (column 70) (len 8)) (ref r783))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 107799) (line 1947) (column 7) (len 59)) (normalized "ISO-80000-10 item 10-62.2 ion number density, ion density "))) (attribute-def (declaration-name "IonNumberDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r784)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 107948) (line 1950) (column 11) (len 515)) (normalized "source: item 10-62.2 ion number density, ion density\nsymbol(s): `n^\"+\"`, `n^\"-\"`\napplication domain: generic\nname: IonNumberDensity\nquantity dimension: L^-3\nmeasurement unit(s): m^-3\ntensor order: 0\ndefinition: quotient of the number of positive and negative ions, `N^\"+\"` and `N^\"-\"`, respectively, in the volume `V` (ISO 80000-3), and that volume: `n^\"+\" = N^\"+\" / V`, `n^\"-\" = N^\"-\" / V`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r785)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r786)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r787)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r788)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "IonNumberDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r789)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r790)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r791)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 108780) (line 1968) (column 77) (len 5)) (member-access (base (expression (span (offset 108780) (line 1968) (column 77) (len 3)) (ref r792))) (separator dot) (member (ref r793))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r794)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 108802) (line 1968) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 108803) (line 1968) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r795)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r796)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 108877) (line 1969) (column 70) (len 8)) (ref r797))))) (body semicolon)))))) (alias (name "IonDensityUnit") (target (ref r798)) (body semicolon)) (alias (name "IonDensityValue") (target (ref r799)) (body semicolon)) (alias (name "ionDensity") (target (ref r800)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 109050) (line 1976) (column 7) (len 51)) (normalized "ISO-80000-10 item 10-63 Recombination coefficient "))) (attribute-def (declaration-name "RecombinationCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r801)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 109199) (line 1979) (column 11) (len 799)) (normalized "source: item 10-63 Recombination coefficient\nsymbol(s): `α`\napplication domain: generic\nname: RecombinationCoefficient\nquantity dimension: L^3*T^-1\nmeasurement unit(s): m^3*s^-1\ntensor order: 0\ndefinition: coefficient in the law of recombination: `-(dn^\"+\")/(dt) = -(dn^\"-\")/(dt) = α*n^\"+\"*n^\"-\"`, where `n^\"+\"` and `n^\"-\"` are the ion number densities (item 10-62.2) of positive and negative ions, respectively, recombined during a time interval of duration `dt` (ISO 80000-3)\nremarks: The widely used term \"recombination factor\" is not correct because \"factor\" should only be used for quantities with dimension 1. The terms `(dn^\"+\")/(dt)` , `(dn^\"-\")/(dt)` are differential quotients.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r802)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r803)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r804)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r805)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "RecombinationCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r806)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r807)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r808)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 110347) (line 1997) (column 77) (len 5)) (member-access (base (expression (span (offset 110347) (line 1997) (column 77) (len 3)) (ref r809))) (separator dot) (member (ref r810))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r811)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 110369) (line 1997) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r812)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r813)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 110452) (line 1998) (column 79) (len 5)) (member-access (base (expression (span (offset 110452) (line 1998) (column 79) (len 3)) (ref r814))) (separator dot) (member (ref r815))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r816)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 110474) (line 1998) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 110475) (line 1998) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r817)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r818)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 110549) (line 1999) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 110550) (line 1999) (column 71) (len 8)) (ref r819))) (element comma (expression (span (offset 110560) (line 1999) (column 81) (len 10)) (ref r820))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 110588) (line 2002) (column 7) (len 98)) (normalized "ISO-80000-10 item 10-64 diffusion coefficient, diffusion coefficient for particle number density "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 110695) (line 2003) (column 7) (len 104)) (normalized "Refer to declaration for DiffusionCoefficient in ISQChemistryMolecular item 9-39 diffusion coefficient "))) (alias (name "DiffusionCoefficientForParticleNumberDensityUnit") (target (ref r821)) (body semicolon)) (alias (name "DiffusionCoefficientForParticleNumberDensityValue") (target (ref r822)) (body semicolon)) (alias (name "diffusionCoefficientForParticleNumberDensity") (target (ref r823)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 111071) (line 2009) (column 7) (len 64)) (normalized "ISO-80000-10 item 10-65 diffusion coefficient for fluence rate "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 111935) (line 2025) (column 7) (len 49)) (normalized "ISO-80000-10 item 10-66 particle source density "))) (attribute-def (declaration-name "ParticleSourceDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r824)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 112079) (line 2028) (column 11) (len 668)) (normalized "source: item 10-66 particle source density\nsymbol(s): `S`\napplication domain: generic\nname: ParticleSourceDensity\nquantity dimension: L^-3*T^-1\nmeasurement unit(s): m^-3*s^-1\ntensor order: 0\ndefinition: quotient of the mean rate of production of particles in a volume, and that volume (ISO 80000-3)\nremarks: The word \"particle\" is usually replaced by the name of a specific particle, for example `proton` source density. The distribution functions expressed in terms of speed and energy, `S_v` and `S_E`, are related to `S` by: `S = int S_v dv = int S_E dE`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r825)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r826)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r827)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r828)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ParticleSourceDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r829)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r830)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r831)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113084) (line 2046) (column 77) (len 5)) (member-access (base (expression (span (offset 113084) (line 2046) (column 77) (len 3)) (ref r832))) (separator dot) (member (ref r833))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r834)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113106) (line 2046) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 113107) (line 2046) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r835)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r836)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113190) (line 2047) (column 79) (len 5)) (member-access (base (expression (span (offset 113190) (line 2047) (column 79) (len 3)) (ref r837))) (separator dot) (member (ref r838))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r839)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113212) (line 2047) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 113213) (line 2047) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r840)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r841)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113287) (line 2048) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 113288) (line 2048) (column 71) (len 8)) (ref r842))) (element comma (expression (span (offset 113298) (line 2048) (column 81) (len 10)) (ref r843))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 113326) (line 2051) (column 7) (len 46)) (normalized "ISO-80000-10 item 10-67 slowing-down density "))) (attribute-def (declaration-name "SlowingDownDensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r844)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 113464) (line 2054) (column 11) (len 505)) (normalized "source: item 10-67 slowing-down density\nsymbol(s): `q`\napplication domain: generic\nname: SlowingDownDensity\nquantity dimension: L^-3*T^-1\nmeasurement unit(s): m^-3*s^-1\ntensor order: 0\ndefinition: differential quotient of `n` with respect to time, where `n` is the number density of particles that are slowed down in a time interval of duration (ISO 80000-3) `t`: `q = -(dn)/(dt)`\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r845)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r846)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r847)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r848)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "SlowingDownDensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r849)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r850)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r851)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 114294) (line 2072) (column 77) (len 5)) (member-access (base (expression (span (offset 114294) (line 2072) (column 77) (len 3)) (ref r852))) (separator dot) (member (ref r853))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r854)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 114316) (line 2072) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 114317) (line 2072) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r855)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r856)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 114400) (line 2073) (column 79) (len 5)) (member-access (base (expression (span (offset 114400) (line 2073) (column 79) (len 3)) (ref r857))) (separator dot) (member (ref r858))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r859)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 114422) (line 2073) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 114423) (line 2073) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r860)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r861)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 114497) (line 2074) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 114498) (line 2074) (column 71) (len 8)) (ref r862))) (element comma (expression (span (offset 114508) (line 2074) (column 81) (len 10)) (ref r863))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 114536) (line 2077) (column 7) (len 54)) (normalized "ISO-80000-10 item 10-68 resonance escape probability "))) (attribute-def (declaration-name "ResonanceEscapeProbabilityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r864)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 114688) (line 2080) (column 11) (len 540)) (normalized "source: item 10-68 resonance escape probability\nsymbol(s): `p`\napplication domain: generic\nname: ResonanceEscapeProbability (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: in an infinite medium, the probability that a neutron slowing down will traverse all or some specified portion of the range of resonance energies (item 10-37.2) without being absorbed\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 115339) (line 2094) (column 7) (len 34)) (normalized "ISO-80000-10 item 10-69 lethargy "))) (attribute-def (declaration-name "LethargyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r865)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 115453) (line 2097) (column 11) (len 477)) (normalized "source: item 10-69 lethargy\nsymbol(s): `u`\napplication domain: generic\nname: Lethargy (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: for a neutron of kinetic energy `E` (ISO 80000-4) : `u = ln(E_0/E)`, where `E_0` is a reference energy\nremarks: Lethargy is also referred to as logarithmic energy decrement.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 116005) (line 2111) (column 7) (len 62)) (normalized "ISO-80000-10 item 10-70 average logarithmic energy decrement "))) (attribute-def (declaration-name "AverageLogarithmicEnergyDecrementValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r866)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 116172) (line 2114) (column 11) (len 561)) (normalized "source: item 10-70 average logarithmic energy decrement\nsymbol(s): `ζ`\napplication domain: generic\nname: AverageLogarithmicEnergyDecrement (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: average value of the increase in lethargy (item 10-69) in elastic collisions between neutrons and nuclei whose kinetic energy (ISO 80000-4) is negligible compared with that of the neutrons\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 116858) (line 2128) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-71 mean free path "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 117483) (line 2144) (column 7) (len 45)) (normalized "ISO-80000-10 item 10-72.1 slowing-down area "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 118159) (line 2160) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-72.2 diffusion area "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 118859) (line 2176) (column 7) (len 42)) (normalized "ISO-80000-10 item 10-72.3 migration area "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 119479) (line 2192) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-73.1 slowing-down length "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 120055) (line 2208) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-73.2 diffusion length "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 120607) (line 2224) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-73.3 migration length "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 121152) (line 2240) (column 7) (len 53)) (normalized "ISO-80000-10 item 10-74.1 neutron yield per fission "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 121741) (line 2256) (column 7) (len 56)) (normalized "ISO-80000-10 item 10-74.2 neutron yield per absorption "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 122542) (line 2272) (column 7) (len 45)) (normalized "ISO-80000-10 item 10-75 fast fission factor "))) (attribute-def (declaration-name "FastFissionFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r867)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 122678) (line 2275) (column 11) (len 568)) (normalized "source: item 10-75 fast fission factor\nsymbol(s): `φ`\napplication domain: generic\nname: FastFissionFactor\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: in an infinite medium, the quotient of the mean number of neutrons produced by fission due to neutrons of all energies (ISO 80000-5) and the mean number of neutrons produced by fissions due to thermal neutrons only\nremarks: The class of neutrons must be specified, e.g. thermal.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r868)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r869)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r870)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r871)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "FastFissionFactorUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r872)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 123509) (line 2295) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-76 thermal utilization factor "))) (attribute-def (declaration-name "ThermalUtilizationFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r873)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 123659) (line 2298) (column 11) (len 507)) (normalized "source: item 10-76 thermal utilization factor\nsymbol(s): `f`\napplication domain: generic\nname: ThermalUtilizationFactor\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: in an infinite medium, the quotient of the number of thermal neutrons absorbed in a fissionable nuclide or in a nuclear fuel, as specified, and the total number of thermal neutrons absorbed\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r874)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r875)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r876)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r877)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ThermalUtilizationFactorUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r878)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 124457) (line 2318) (column 7) (len 49)) (normalized "ISO-80000-10 item 10-77 non-leakage probability "))) (attribute-def (declaration-name "NonLeakageProbabilityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r879)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 124601) (line 2321) (column 11) (len 446)) (normalized "source: item 10-77 non-leakage probability\nsymbol(s): `Λ`\napplication domain: generic\nname: NonLeakageProbability\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: probability that a neutron will not escape from the reactor during the slowing-down process or while it diffuses as a thermal neutron\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r880)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r881)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r882)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r883)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "NonLeakageProbabilityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r884)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 125326) (line 2341) (column 7) (len 49)) (normalized "ISO-80000-10 item 10-78.1 multiplication factor "))) (attribute-def (declaration-name "MultiplicationFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r885)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 125469) (line 2344) (column 11) (len 505)) (normalized "source: item 10-78.1 multiplication factor\nsymbol(s): `k`\napplication domain: generic\nname: MultiplicationFactor\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the total number of fission or fission-dependent neutrons produced in the duration of a time interval and the total number of neutrons lost by absorption and leakage in that duration\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r886)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r887)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r888)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r889)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MultiplicationFactorUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r890)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 126249) (line 2364) (column 7) (len 58)) (normalized "ISO-80000-10 item 10-78.2 infinite multiplication factor "))) (attribute-def (declaration-name "InfiniteMultiplicationFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r891)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 126409) (line 2367) (column 11) (len 465)) (normalized "source: item 10-78.2 infinite multiplication factor\nsymbol(s): `k_∞`\napplication domain: generic\nname: InfiniteMultiplicationFactor\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: multiplication factor (item 10-78.1) for an infinite medium or for an infinite repeating lattice\nremarks: For a thermal reactor, `k_∞ = η*ε*p*f`\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r892)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r893)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r894)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r895)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "InfiniteMultiplicationFactorUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r896)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 127181) (line 2387) (column 7) (len 47)) (normalized "ISO-80000-10 item 10-79 reactor time constant "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 127864) (line 2403) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-80.1 energy imparted "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 129778) (line 2419) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-80.2 mean energy imparted "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 130887) (line 2435) (column 7) (len 41)) (normalized "ISO-80000-10 item 10-81.1 absorbed dose "))) (attribute-def (declaration-name "AbsorbedDoseValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r897)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 131014) (line 2438) (column 11) (len 1281)) (normalized "source: item 10-81.1 absorbed dose\nsymbol(s): `D`\napplication domain: generic\nname: AbsorbedDose\nquantity dimension: L^2*T^-2\nmeasurement unit(s): Gy, J/kg, m^2*s^-2\ntensor order: 0\ndefinition: differential quotient of `bar(ε)` with respect to `m`, where `bar(ε)` is the mean energy (ISO 80000-5) imparted by ionizing radiation to matter of mass (ISO 80000-4) `m`: `D = (d bar(ε))/(dm)`\nremarks: The gray is a special name for joule per kilogram, to be used as the coherent SI unit for absorbed dose. `1 \"Gy\" = 1 \"J\"/\"kg\"`. `bar(ε) = int D dm` where `dm` is the element of mass of the irradiated matter. In the limit of a small domain, the mean specific energy `bar(z) = (Δ bar(ε))/(Δ m)` is equal to the absorbed dose `D`. The absorbed dose can also be expressed in terms of the volume of the mass element by: `D = (d bar(ε))/(dm) = (d bar(ε))/(ρ dV)` where `ρ` is the mass density of the mass element. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed dose, `D`, is the quotient of `d bar(ε)` by dm, where `d bar(ε)` is the mean energy imparted by ionizing radiation to matter of mass `dm`: `D = (d bar(ε))/(dm)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r898)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r899)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r900)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r901)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AbsorbedDoseUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r902)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r903)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r904)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 132596) (line 2456) (column 77) (len 5)) (member-access (base (expression (span (offset 132596) (line 2456) (column 77) (len 3)) (ref r905))) (separator dot) (member (ref r906))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r907)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 132618) (line 2456) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r908)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r909)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 132701) (line 2457) (column 79) (len 5)) (member-access (base (expression (span (offset 132701) (line 2457) (column 79) (len 3)) (ref r910))) (separator dot) (member (ref r911))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r912)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 132723) (line 2457) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 132724) (line 2457) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r913)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r914)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 132798) (line 2458) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 132799) (line 2458) (column 71) (len 8)) (ref r915))) (element comma (expression (span (offset 132809) (line 2458) (column 81) (len 10)) (ref r916))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 132837) (line 2461) (column 7) (len 52)) (normalized "ISO-80000-10 item 10-81.2 specific energy imparted "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 133719) (line 2477) (column 7) (len 40)) (normalized "ISO-80000-10 item 10-82 quality factor "))) (attribute-def (declaration-name "QualityFactorForIonizingRadiationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r917)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 133866) (line 2480) (column 11) (len 910)) (normalized "source: item 10-82 quality factor\nsymbol(s): `Q`\napplication domain: ionizing radiation\nname: QualityFactor\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: factor in the calculation and measurement of dose equivalent (item 10-83.1), by which the absorbed dose (item 10-81.1) is to be weighted in order to account for different biological effectiveness of radiations, for radiation protection purposes\nremarks: `Q` is determined by the linear energy transfer (item 10-85) for `Δ -> ∞` , `L_∞` (often denoted as `L` or LET), of charged particles passing through a small volume element at this point (the value of `L_∞` refers to water, not to tissue; the difference, however, is small). The relationship between `L` and `Q` is given in ICRP Publication 103 (ICRP, 2007).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r918)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r919)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r920)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r921)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "QualityFactorForIonizingRadiationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r922)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 135103) (line 2500) (column 7) (len 43)) (normalized "ISO-80000-10 item 10-83.1 dose equivalent "))) (attribute-def (declaration-name "DoseEquivalentValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r923)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 135234) (line 2503) (column 11) (len 1565)) (normalized "source: item 10-83.1 dose equivalent\nsymbol(s): `H`\napplication domain: generic\nname: DoseEquivalent\nquantity dimension: L^2*T^-2\nmeasurement unit(s): Sv, J/kg, m^2*s^-2\ntensor order: 0\ndefinition: product of the absorbed dose `D` (item 10-81.1) to tissue at the point of interest and the quality factor `Q` (item 10-82) at that point: `H = DQ`\nremarks: The sievert (Sv) is a special name for joule per kilogram, and is the coherent SI unit for dose equivalent. `1 \"Sv\" = 1 \"J/kg\"`. The dose equivalent at a point in tissue is given by: `H = int_0^∞ Q(L) D_L dL` where `D_L = (dD)/(dL)` is the distribution of `D` in `L` at the point of interest. See ICRP Publication 103 (ICRP, 2007). The quantities measured with radiation protection dosimeters are based on the definition `H = Q*D`. If various radiation qualities `i` have to be simultaneously accounted for, the definition is: `H = sum_i Q_i*D_i`. In ICRU 51 this quantity is denoted as \"dose equivalent\". In order to quantify the radiation exposition of the human body and to specify dose limits, use is made of a quantity defined in ICRP 103, the \"equivalent dose to a tissue or organ\": `H_T = w_T*sum_R w_R*D_{T,R}`. The weighting factors `w_T` for various tissues and organs `T` and `w_R` for various radiation qualities `R` have been numerically laid down in ICRP 103. `D_{T,R}` is the mean absorbed dose to tissue within a tissue or organ `T`, imparted by radiation with radiation quality `R`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r924)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r925)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r926)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r927)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DoseEquivalentUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r928)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r929)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r930)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 137108) (line 2521) (column 77) (len 5)) (member-access (base (expression (span (offset 137108) (line 2521) (column 77) (len 3)) (ref r931))) (separator dot) (member (ref r932))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r933)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 137130) (line 2521) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r934)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r935)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 137213) (line 2522) (column 79) (len 5)) (member-access (base (expression (span (offset 137213) (line 2522) (column 79) (len 3)) (ref r936))) (separator dot) (member (ref r937))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r938)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 137235) (line 2522) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 137236) (line 2522) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r939)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r940)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 137310) (line 2523) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 137311) (line 2523) (column 71) (len 8)) (ref r941))) (element comma (expression (span (offset 137321) (line 2523) (column 81) (len 10)) (ref r942))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 137349) (line 2526) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-83.2 dose equivalent rate "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 138049) (line 2542) (column 7) (len 44)) (normalized "ISO-80000-10 item 10-84 absorbed-dose rate "))) (attribute-def (declaration-name "AbsorbedDoseRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r943)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 138183) (line 2545) (column 11) (len 743)) (normalized "source: item 10-84 absorbed-dose rate\nsymbol(s): `dot(D)`\napplication domain: generic\nname: AbsorbedDoseRate\nquantity dimension: L^2*T^-3\nmeasurement unit(s): Gy/s, W/kg, m^2*s^-3\ntensor order: 0\ndefinition: differential quotient of the absorbed dose `D` (item 10-81.1) with respect to time (ISO 80000-3): `dot(D) = (dD)/(dt)`\nremarks: `1 \"Gy/s\"  = 1 \"W/kg\"` See the remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The absorbed-does rate, `dot(D)` , is the quotient of `dD` by `dt`, where `dD` is the increment of absorbed does in the time interval `dt`: `dot(D) = (dD)/(dt)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r944)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r945)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r946)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r947)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AbsorbedDoseRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r948)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r949)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r950)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139243) (line 2563) (column 77) (len 5)) (member-access (base (expression (span (offset 139243) (line 2563) (column 77) (len 3)) (ref r951))) (separator dot) (member (ref r952))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r953)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139265) (line 2563) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r954)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r955)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139348) (line 2564) (column 79) (len 5)) (member-access (base (expression (span (offset 139348) (line 2564) (column 79) (len 3)) (ref r956))) (separator dot) (member (ref r957))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r958)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139370) (line 2564) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 139371) (line 2564) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r959)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r960)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 139445) (line 2565) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 139446) (line 2565) (column 71) (len 8)) (ref r961))) (element comma (expression (span (offset 139456) (line 2565) (column 81) (len 10)) (ref r962))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 139484) (line 2568) (column 7) (len 48)) (normalized "ISO-80000-10 item 10-85 linear energy transfer "))) (attribute-def (declaration-name "LinearEnergyTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r963)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 139626) (line 2571) (column 11) (len 957)) (normalized "source: item 10-85 linear energy transfer\nsymbol(s): `L_Δ`\napplication domain: generic\nname: LinearEnergyTransfer\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): eV/m, J/m, kg*m*s^-2\ntensor order: 0\ndefinition: quotient of the mean energy (ISO 80000-4) `dE_Δ` lost by the charged particles due to electronic interactions in traversing a distance (ISO 80000-3) `dl`, minus the mean sum of the kinetic energies in excess of `Δ` of all the electrons released by the charged particles and `dl`: `L_Δ = (dE_Δ)/(dl)`\nremarks: This quantity is not completely defined unless `Δ` is specified, i.e. the maximum kinetic energy of secondary electrons whose energy is considered to be \"locally deposited\". `Δ` may be expressed in `\"eV\"`. Note that the abbreviation LET specifically refers to the quantity `L_∞` mentioned in the remark to 10-82.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r964)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r965)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r966)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r967)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LinearEnergyTransferUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r968)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r969)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r970)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 140916) (line 2589) (column 77) (len 5)) (member-access (base (expression (span (offset 140916) (line 2589) (column 77) (len 3)) (ref r971))) (separator dot) (member (ref r972))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r973)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 140938) (line 2589) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r974)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r975)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 141017) (line 2590) (column 75) (len 5)) (member-access (base (expression (span (offset 141017) (line 2590) (column 75) (len 3)) (ref r976))) (separator dot) (member (ref r977))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r978)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 141039) (line 2590) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r979)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r980)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 141122) (line 2591) (column 79) (len 5)) (member-access (base (expression (span (offset 141122) (line 2591) (column 79) (len 3)) (ref r981))) (separator dot) (member (ref r982))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r983)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 141144) (line 2591) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 141145) (line 2591) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r984)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r985)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 141219) (line 2592) (column 70) (len 30)) (sequence (sequence-list (element first (expression (span (offset 141220) (line 2592) (column 71) (len 8)) (ref r986))) (element comma (expression (span (offset 141230) (line 2592) (column 81) (len 6)) (ref r987))) (element comma (expression (span (offset 141238) (line 2592) (column 89) (len 10)) (ref r988))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 141266) (line 2595) (column 7) (len 33)) (normalized "ISO-80000-10 item 10-86.1 kerma "))) (attribute-def (declaration-name "KermaValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r989)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 141378) (line 2598) (column 11) (len 1473)) (normalized "source: item 10-86.1 kerma\nsymbol(s): `K`\napplication domain: generic\nname: Kerma\nquantity dimension: L^2*T^-2\nmeasurement unit(s): Gy, J/kg, m^2*s^-2\ntensor order: 0\ndefinition: for uncharged ionizing radiation, differential quotient of `E_(`tr) with respect to `m`, where `E_(`tr) is the mean sum of the initial kinetic energies (ISO 80000-4) of all the charged ionizing particles liberated in a mass (ISO 80000-4) `m` of a material: `K = (dE_tr)/(dm)`\nremarks: `1 \"Gy\" = 1 \"J/kg\"` See the remarks for item 10-81.1. The name \"kerma\" is derived from Kinetic Energy Released in MAtter (or MAss or MAterial). The quantity `dE_(tr)` includes also the kinetic energy of the charged particles emitted in the decay of excited atoms, molecules, or nuclei. When the mass element `dm` consists of air the term air kerma is used. It can be convenient to refer to a value of air kerma in free space or at a point inside a material different from air, e.g. to the air kerma at a point inside a water phantom. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma, `K`, for ionizing uncharged particles, is the quotient of `dE_(tr)` by `dm`, where `dE_(tr)` is the mean sum of the initial kinetic energies of all the charged particles liberated in a mass `dm` of a material by the uncharged particles incident on `dm`: `K = (dE_(tr))/(dm)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r990)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r991)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r992)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r993)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "KermaUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r994)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r995)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r996)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 143124) (line 2616) (column 77) (len 5)) (member-access (base (expression (span (offset 143124) (line 2616) (column 77) (len 3)) (ref r997))) (separator dot) (member (ref r998))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r999)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 143146) (line 2616) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1000)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1001)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 143229) (line 2617) (column 79) (len 5)) (member-access (base (expression (span (offset 143229) (line 2617) (column 79) (len 3)) (ref r1002))) (separator dot) (member (ref r1003))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1004)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 143251) (line 2617) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 143252) (line 2617) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1005)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1006)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 143326) (line 2618) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 143327) (line 2618) (column 71) (len 8)) (ref r1007))) (element comma (expression (span (offset 143337) (line 2618) (column 81) (len 10)) (ref r1008))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 143365) (line 2621) (column 7) (len 38)) (normalized "ISO-80000-10 item 10-86.2 kerma rate "))) (attribute-def (declaration-name "KermaRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1009)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 143486) (line 2624) (column 11) (len 698)) (normalized "source: item 10-86.2 kerma rate\nsymbol(s): `dot(K)`\napplication domain: generic\nname: KermaRate\nquantity dimension: L^2*T^-3\nmeasurement unit(s): Gy/s, W/kg, m^2*s^-3\ntensor order: 0\ndefinition: differential quotient of kerma (item 10-86.1) with respect to time (ISO 80000-3): `dot(K) = (dK)/(dt)`\nremarks: `1 \"Gy/s\" = 1 \"W/kg\"`. See the Remarks for item 10-81.1. In report 85a of the ICRU a definition with an equivalent meaning is given as: The kerma rate, `dot(K)` , is the quotient of `dK` by `dt`, where `dK` is the increment of kerma in the time interval `dt`: `dot(K) = (dK)/(dt)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1010)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1011)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1012)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1013)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "KermaRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1014)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1015)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1016)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144473) (line 2642) (column 77) (len 5)) (member-access (base (expression (span (offset 144473) (line 2642) (column 77) (len 3)) (ref r1017))) (separator dot) (member (ref r1018))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1019)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144495) (line 2642) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1020)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1021)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144578) (line 2643) (column 79) (len 5)) (member-access (base (expression (span (offset 144578) (line 2643) (column 79) (len 3)) (ref r1022))) (separator dot) (member (ref r1023))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1024)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144600) (line 2643) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 144601) (line 2643) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1025)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1026)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144675) (line 2644) (column 70) (len 22)) (sequence (sequence-list (element first (expression (span (offset 144676) (line 2644) (column 71) (len 8)) (ref r1027))) (element comma (expression (span (offset 144686) (line 2644) (column 81) (len 10)) (ref r1028))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 144714) (line 2647) (column 7) (len 58)) (normalized "ISO-80000-10 item 10-87 mass energy-transfer coefficient "))) (attribute-def (declaration-name "MassEnergyTransferCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1029)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 144875) (line 2650) (column 11) (len 2042)) (normalized "source: item 10-87 mass energy-transfer coefficient\nsymbol(s): `μ_\"tr\"/ρ`\napplication domain: generic\nname: MassEnergyTransferCoefficient\nquantity dimension: L^2*M^-1\nmeasurement unit(s): kg^-1*m^2\ntensor order: 0\ndefinition: for ionizing uncharged particles of a given type and energy, the differential quotient of `R_\"tr\"` with respect to `l`: `m_\"tr\"/ρ = 1/ρ 1/R (dR_\"tr\")/(dl)` where `R_\"tr\"` is the mean energy (ISO 80000-5) that is transferred to kinetic energy (ISO 80000-4) of charged particles by interactions of the uncharged particles of incident radiant energy `R` (item 10-45) in traversing a distance (ISO 80000-3) `l` in the material of density (ISO 80000-4) `ρ`, divided by `ρ` and `R`\nremarks: `m_(tr)/ρ = (dot(K))/ψ` , where `dot(K)` is kerma rate (item 10-86.2) and `ψ` is energy fluence rate (item 10-47). The quantity: `μ_(en)/ρ = μ_(tr)/ρ(1-g)` where `g` is mean fraction of the kinetic energy of the liberated charged particles that is lost in radiative processes in the material, is called mass energy-absorption coefficient. The mass energy-absorption coefficient of a compound material depends on the stopping power of the material. Thus, its evaluation cannot, in principle, be reduced to a simple summation of the mass energy-absorption coefficient of the atomic constituents. Such a summation can provide an adequate approximation when the value of `g` is sufficiently small. In report 85a of the ICRU a definition with an equivalent meaning is given as: The mass energy-transfer coefficient, `μ_(tr)/ρ` , of a material, for uncharged particles of a given type and energy, is the quotient of `(dR_(tr))/R` by `ρ dl`, where `dR_(tr)` is the mean energy that is transferred to kinetic energy of charged particles by interactions of the uncharged particles of incident radiant energy `R` in traversing a distance `dl` in the material of density `ρ` : `μ_(tr)/ρ = 1/(ρ dl) (d R_(tr))/R`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1030)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1031)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1032)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1033)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassEnergyTransferCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1034)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1035)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1036)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 147286) (line 2668) (column 77) (len 5)) (member-access (base (expression (span (offset 147286) (line 2668) (column 77) (len 3)) (ref r1037))) (separator dot) (member (ref r1038))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1039)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 147308) (line 2668) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1040)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1041)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 147387) (line 2669) (column 75) (len 5)) (member-access (base (expression (span (offset 147387) (line 2669) (column 75) (len 3)) (ref r1042))) (separator dot) (member (ref r1043))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1044)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 147409) (line 2669) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 147410) (line 2669) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1045)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1046)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 147484) (line 2670) (column 70) (len 18)) (sequence (sequence-list (element first (expression (span (offset 147485) (line 2670) (column 71) (len 8)) (ref r1047))) (element comma (expression (span (offset 147495) (line 2670) (column 81) (len 6)) (ref r1048))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 147519) (line 2673) (column 7) (len 34)) (normalized "ISO-80000-10 item 10-88 exposure "))) (attribute-def (declaration-name "ExposureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1049)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 147635) (line 2676) (column 11) (len 1962)) (normalized "source: item 10-88 exposure\nsymbol(s): `X`\napplication domain: ionizing radiation\nname: Exposure\nquantity dimension: M^-1*T^1*I^1\nmeasurement unit(s): C/kg, kg^-1*s*A\ntensor order: 0\ndefinition: for X- or gamma radiation the differential quotient of `q` with respect to `m`, where `q` is the absolute value of the mean total electric charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on an element of dry air with mass `m` (ISO 80000-4) are completely stopped in dry air: `X = (dq)/(dm)`\nremarks: The ionization produced by electrons emitted in atomic or molecular relaxation is included in `dq`. The ionization due to photons emitted by radiative processes (i.e. bremsstrahlung and fluorescence photons) is not included in `dq`. This quantity should not be confused with the quantity photon exposure (ISO 80000-7), radiation exposure (ISO 80000-7), or the quantity luminous exposure (ISO 80000-7). It can be convenient to refer to a value of exposure in free space or at a point inside a material different from air, e.g. to the exposure at a point inside a water phantom. The exposure is related to the air kerma, `K_a`, (see item 10-86.1) by: `X = (e (1-g))/W K_a` , where `e` is the elementary charge (ISO 80000-1), `W` the average energy loss per elementary charge produced (item 10-60), and `g` is the fraction of the kinetic energy of liberated charged particles that is lost in radiative processes. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure, `X`, is the quotient of `dq` by `dm`, where `dq` is the absolute value of the mean total charge of the ions of one sign produced when all the electrons and positrons liberated or created by photons incident on a mass `dm` of dry air are completely stopped in dry air: `X = (dq)/(dm)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1050)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1051)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1052)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1053)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ExposureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1054)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1055)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1056)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149880) (line 2694) (column 75) (len 5)) (member-access (base (expression (span (offset 149880) (line 2694) (column 75) (len 3)) (ref r1057))) (separator dot) (member (ref r1058))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1059)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149902) (line 2694) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 149903) (line 2694) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1060)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1061)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 149986) (line 2695) (column 79) (len 5)) (member-access (base (expression (span (offset 149986) (line 2695) (column 79) (len 3)) (ref r1062))) (separator dot) (member (ref r1063))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1064)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 150008) (line 2695) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1065)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1066)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 150098) (line 2696) (column 86) (len 5)) (member-access (base (expression (span (offset 150098) (line 2696) (column 86) (len 3)) (ref r1067))) (separator dot) (member (ref r1068))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1069)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 150120) (line 2696) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1070)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1071)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 150194) (line 2697) (column 70) (len 39)) (sequence (sequence-list (element first (expression (span (offset 150195) (line 2697) (column 71) (len 6)) (ref r1072))) (element comma (expression (span (offset 150203) (line 2697) (column 79) (len 10)) (ref r1073))) (element comma (expression (span (offset 150215) (line 2697) (column 91) (len 17)) (ref r1074))))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 150250) (line 2700) (column 7) (len 39)) (normalized "ISO-80000-10 item 10-89 exposure rate "))) (attribute-def (declaration-name "ExposureRateValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1075)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 150375) (line 2703) (column 11) (len 684)) (normalized "source: item 10-89 exposure rate\nsymbol(s): `dot(X)`\napplication domain: generic\nname: ExposureRate\nquantity dimension: M^-1*I^1\nmeasurement unit(s): C/(kg*s), kg^-1*A\ntensor order: 0\ndefinition: differential quotient of the exposure `X` (item 10-88) with respect to time (ISO 80000-3): `dot(X) = (dX)/(dt)`\nremarks: `1 \"C/(kg s)\" = 1 \"A/kg\"`. In report 85a of the ICRU a definition with an equivalent meaning is given as: The exposure rate, `dot(X)` , is the quotient of `dX` by `dt`, where `dX` is the increment of exposure in the time interval `dt`: `dot(X) = (dX)/(dt)`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1076)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1077)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1078)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1079)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ExposureRateUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1080)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1081)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1082)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 151358) (line 2721) (column 75) (len 5)) (member-access (base (expression (span (offset 151358) (line 2721) (column 75) (len 3)) (ref r1083))) (separator dot) (member (ref r1084))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1085)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 151380) (line 2721) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 151381) (line 2721) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1086)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1087)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 151471) (line 2722) (column 86) (len 5)) (member-access (base (expression (span (offset 151471) (line 2722) (column 86) (len 3)) (ref r1088))) (separator dot) (member (ref r1089))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1090)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 151493) (line 2722) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1091)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1092)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 151567) (line 2723) (column 70) (len 27)) (sequence (sequence-list (element first (expression (span (offset 151568) (line 2723) (column 71) (len 6)) (ref r1093))) (element comma (expression (span (offset 151576) (line 2723) (column 79) (len 17)) (ref r1094))))))))) (body semicolon)))))))))
)
~~~
