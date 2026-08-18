# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQCondensedMatter"))
~~~
# SOURCE
~~~sysml
standard library package ISQCondensedMatter {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-12:2019 "Condensed matter physics"
     * see also https://www.iso.org/standard/63480.html
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
    private import ISQElectromagnetism::ElectricPotentialDifferenceValue;
    private import ISQElectromagnetism::MagneticFluxDensityValue;
    private import ISQElectromagnetism::ResistivityValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQSpaceTime::RepetencyValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-12 item 12-1.1 lattice vector */
    attribute def CartesianLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.1 lattice vector
         * symbol(s): `vec(R)`
         * application domain: generic
         * name: LatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: translation vector that maps the crystal lattice on itself
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianLattice3dVector: CartesianLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-1.2 fundamental lattice vector */
    attribute def CartesianFundamentalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.2 fundamental lattice vector
         * symbol(s): `vec(a_1),vec(a_2),vec(a_3)`, `vec(a),vec(b),vec(c)`
         * application domain: generic
         * name: FundamentalLatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: fundamental translation vectors for the crystal lattice
         * remarks: The lattice vector (item 12-1.1) can be given as `vec(R) = n_1 vec(a_1) + n_2 vec(a_2) + n_3 vec(a_3)` where `n_1`, `n_2` and `n_3` are integers.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianFundamentalLattice3dVector: CartesianFundamentalLattice3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-2.1 angular reciprocal lattice vector */
    attribute def AngularReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector (magnitude)
         * symbol(s): `G`
         * application domain: generic
         * name: AngularReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularReciprocalLatticeVectorMagnitudeUnit[1];
    }

    attribute angularReciprocalLatticeVectorMagnitude: AngularReciprocalLatticeVectorMagnitudeValue[*] nonunique :> scalarQuantities;

    attribute def AngularReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    attribute def CartesianAngularReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector
         * symbol(s): `vec(G)`
         * application domain: generic
         * name: AngularReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularReciprocalLattice3dCoordinateFrame[1];
    }

    attribute cartesianAngularReciprocalLattice3dVector: CartesianAngularReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianAngularReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularReciprocalLatticeVectorMagnitudeUnit[3];
    }

    /* ISO-80000-12 item 12-2.2 fundamental reciprocal lattice vector */
    attribute def FundamentalReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector (magnitude)
         * symbol(s): `b_1,b_2,b_3`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FundamentalReciprocalLatticeVectorMagnitudeUnit[1];
    }

    attribute fundamentalReciprocalLatticeVectorMagnitude: FundamentalReciprocalLatticeVectorMagnitudeValue[*] nonunique :> scalarQuantities;

    attribute def FundamentalReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    attribute def CartesianFundamentalReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector
         * symbol(s): `vec(b_1),vec(b_2),vec(b_3)`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianFundamentalReciprocalLattice3dCoordinateFrame[1];
    }

    attribute cartesianFundamentalReciprocalLattice3dVector: CartesianFundamentalReciprocalLattice3dVector :> vectorQuantities;

    attribute def CartesianFundamentalReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: FundamentalReciprocalLatticeVectorMagnitudeUnit[3];
    }

    /* ISO-80000-12 item 12-3 lattice plane spacing */
    attribute latticePlaneSpacing: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-3 lattice plane spacing
         * symbol(s): `d`
         * application domain: generic
         * name: LatticePlaneSpacing (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) between successive lattice planes
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
    }

    /* ISO-80000-12 item 12-4 Bragg angle */
    attribute braggAngle: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-4 Bragg angle
         * symbol(s): `ϑ`
         * application domain: generic
         * name: BraggAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): °, 1
         * tensor order: 0
         * definition: angle between the scattered ray and the lattice plane
         * remarks: Bragg angle `ϑ` is given by `2d sin ϑ = nλ`, where `d` is the lattice plane spacing (item 12-3), `λ` is the wavelength (ISO 80000-7) of the radiation, and `n` is the order of reflexion which is an integer.
         */
    }

    /* ISO-80000-12 item 12-5.1 short-range order parameter */
    attribute def ShortRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.1 short-range order parameter
         * symbol(s): `r`, `σ`
         * application domain: generic
         * name: ShortRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of nearest-neighbour atom pairs in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute shortRangeOrderParameter: ShortRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.2 long-range order parameter */
    attribute def LongRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.2 long-range order parameter
         * symbol(s): `R`, `s`
         * application domain: generic
         * name: LongRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of atoms in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute longRangeOrderParameter: LongRangeOrderParameterValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.3 atomic scattering factor */
    attribute def AtomicScatteringFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.3 atomic scattering factor
         * symbol(s): `f`
         * application domain: generic
         * name: AtomicScatteringFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiation amplitude scattered by the atom and radiation amplitude scattered by a single electron
         * remarks: The atomic scattering factor can be expressed by: `f = E_a/(E_e`, where `E_a` is the radiation amplitude scattered by the atom and `E_e` is the radiation amplitude scattered by a single electron.
         */
    }
    attribute atomicScatteringFactor: AtomicScatteringFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-5.4 structure factor */
    attribute def StructureFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.4 structure factor
         * symbol(s): `F(h,k,l)`
         * application domain: generic
         * name: StructureFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `F(h,k,l) = sum_(n=1)^N f_n exp[2π i (h x_n + k y_n + l z_n)]`, where `f_n` is the atomic scattering factor (item 12-5.3) for atom `n`, `x_n`, `y_n`, `z_n` are fractional coordinates of its position, `N` is the total number of atoms in the unit cell and `h`, `k`, `l` are the Miller indices
         * remarks: For the Miller indices `h`, `k`, `l`, see Annex A.
         */
    }
    attribute structureFactor: StructureFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-6 Burgers vector */
    attribute def CartesianBurgers3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-6 Burgers vector
         * symbol(s): `vec(b)`
         * application domain: generic
         * name: BurgersVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: closing vector in a sequence of vectors encircling a dislocation
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianBurgers3dVector: CartesianBurgers3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.1 particle position vector */
    attribute def CartesianParticlePosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.1 particle position vector
         * symbol(s): `vec(r)`, `vec(R)`
         * application domain: generic
         * name: ParticlePositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of a particle
         * remarks: Often, `r` is used for electrons and `R` is used for atoms and other heavier particles.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianParticlePosition3dVector: CartesianParticlePosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.2 equilibrium position vector */
    attribute def CartesianEquilibriumPosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.2 equilibrium position vector
         * symbol(s): `vec(R_0)`
         * application domain: condensed matter physics
         * name: EquilibriumPositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of an ion or atom in equilibrium
         * remarks: None.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianEquilibriumPosition3dVector: CartesianEquilibriumPosition3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-7.3 displacement vector */
    attribute def CartesianDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.3 displacement vector
         * symbol(s): `vec(u)`
         * application domain: condensed matter physics
         * name: DisplacementVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: difference between the position vector (ISO 80000-3) of an ion or atom and its position vector in equilibrium
         * remarks: The displacement vector can be expressed by: `vec(u) = vec(R) − vec(R_0)`, where `vec(R)` is particle position vector (item 12-7.1) and `vec(R_0)` is position vector of an ion or atom in equilibrium (item 12-7.2).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianDisplacement3dVector: CartesianDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-12 item 12-8 Debye-Waller factor */
    attribute def DebyeWallerFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-8 Debye-Waller factor
         * symbol(s): `D`, `B`
         * application domain: generic
         * name: DebyeWallerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor by which the intensity of a diffraction line is reduced because of the lattice vibrations
         * remarks: `D` is sometimes expressed as `D = exp(−2W)`; in Mössbauer spectroscopy, it is also called the `f` factor and denoted by `f`.
         */
    }
    attribute debyeWallerFactor: DebyeWallerFactorValue :> scalarQuantities;

    /* ISO-80000-12 item 12-9.1 angular wavenumber, angular repetency */
    attribute angularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.1 angular wavenumber, angular repetency
         * symbol(s): `k`, `q`
         * application domain: condensed matter physics
         * name: AngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: quotient of momentum (ISO 80000-4) and the reduced Planck constant (ISO 80000-1)
         * remarks: The corresponding vector (ISO 80000-2) quantity is called wave vector (ISO 80000-3), expressed by: `vec(k) = vec(p)/ħ`, where `vec(p)` is the momentum (ISO 80000-4) of quasi free electrons in an electron gas, and `ħ` is the reduced Planck constant (ISO 80000-1); for phonons, its magnitude is `k = 2π/λ`, where `λ` is the wavelength (ISO 80000-3) of the lattice vibrations. When a distinction is needed between `k` and the symbol for the Boltzmann constant (ISO 80000-1), `k_B` can be used for the latter. When a distinction is needed, `q` should be used for phonons, and `k` for particles such as electrons and neutrons. The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias angularRepetency for angularWavenumber;

    /* ISO-80000-12 item 12-9.2 Fermi angular wavenumber, Fermi angular repetency */
    attribute fermiAngularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.2 Fermi angular wavenumber, Fermi angular repetency
         * symbol(s): `k_F`
         * application domain: generic
         * name: FermiAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: angular wavenumber (item 12-9.1) of electrons in states on the Fermi sphere
         * remarks: In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias fermiAngularRepetency for fermiAngularWavenumber;

    /* ISO-80000-12 item 12-9.3 Debye angular wavenumber, Debye angular repetency */
    attribute debyeAngularWavenumber: RepetencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-9.3 Debye angular wavenumber, Debye angular repetency
         * symbol(s): `q_D`
         * application domain: generic
         * name: DebyeAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: cut-off angular wavenumber (item 12-9.1) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }

    alias debyeAngularRepetency for debyeAngularWavenumber;

    /* ISO-80000-12 item 12-10 Debye angular frequency */
    attribute debyeAngularFrequency: AngularFrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-10 Debye angular frequency
         * symbol(s): `ω_D`
         * application domain: generic
         * name: DebyeAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: cut-off angular frequency (ISO 80000-3) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified.
         */
    }

    /* ISO-80000-12 item 12-11 Debye temperature */
    attribute debyeTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-11 Debye temperature
         * symbol(s): `Θ_D`
         * application domain: generic
         * name: DebyeTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the Debye model, quantity given by: `Θ_D = ħ*ω_D/k`, where `k` is the Boltzmann constant, (ISO 80000-1), `ħ` is the reduced Planck constant (ISO 80000-1), and `ω_D` is Debye angular frequency (item 12-10)
         * remarks: A Debye temperature can also be defined by fitting a Debye model result to a certain quantity, for instance, the heat capacity at a certain temperature.
         */
    }

    /* ISO-80000-12 item 12-12 density of vibrational states */
    attribute def DensityOfVibrationalStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-12 density of vibrational states
         * symbol(s): `g`
         * application domain: angular frequency
         * name: DensityOfVibrationalStates
         * quantity dimension: L^-3*T^1
         * measurement unit(s): m^-3*s
         * tensor order: 0
         * definition: quotient of the number of vibrational modes in an infinitesimal interval of angular frequency (ISO 80000-3), and the product of the width of that interval and volume (ISO 80000-3)
         * remarks: `g(ω) = n_ω = (dn(ω))/(dω)`, where `n(ω)` is the total number of vibrational modes per volume with angular frequency less than `ω`. The density of states may also be normalized in other ways instead of with respect to volume. See also item 12-16.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DensityOfVibrationalStatesUnit[1];
    }

    attribute densityOfVibrationalStates: DensityOfVibrationalStatesValue[*] nonunique :> scalarQuantities;

    attribute def DensityOfVibrationalStatesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-12 item 12-13 thermodynamic Grüneisen parameter */
    attribute def 'ThermodynamicGrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-13 thermodynamic Grüneisen parameter
         * symbol(s): `γ_G`, `Γ_G`
         * application domain: generic
         * name: ThermodynamicGrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `γ_G = (α_V)/(κ_T c_V ρ)`, where `α_V` is cubic expansion coefficient (ISO 80000-5), `κ_T` is isothermal compressibility (ISO 80000-5), `c_V` is specific heat capacity at constant volume (ISO 80000-5), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'thermodynamicGrüneisenParameter': 'ThermodynamicGrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-14 Grüneisen parameter */
    attribute def 'GrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-14 Grüneisen parameter
         * symbol(s): `γ`
         * application domain: generic
         * name: GrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by minus the partial differential quotient: `γ = -(del ln ω)/(del ln V)`, where `ω` is a lattice vibration frequency (ISO 80000-3), and `V` is volume (ISO 80000-3)
         * remarks: `ω` can also refer to an average of the vibrational spectrum, for instance as represented by a Debye angular frequency (item 12-10).
         */
    }
    attribute 'grüneisenParameter': 'GrüneisenParameterValue' :> scalarQuantities;

    /* ISO-80000-12 item 12-15.1 mean free path of phonons */
    attribute meanFreePathOfPhonons: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-15.1 mean free path of phonons
         * symbol(s): `l_p`
         * application domain: generic
         * name: MeanFreePathOfPhonons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that phonons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-15.2 mean free path of electrons */
    attribute meanFreePathOfElectrons: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-15.2 mean free path of electrons
         * symbol(s): `l_e`
         * application domain: generic
         * name: MeanFreePathOfElectrons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that electrons travel between two successive interactions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-16 energy density of states */
    attribute def EnergyDensityOfStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-16 energy density of states
         * symbol(s): `n_E(E)`, `ρ(E)`
         * application domain: generic
         * name: EnergyDensityOfStates
         * quantity dimension: L^-5*M^-1*T^2
         * measurement unit(s): J^-1*m^-3*eV^-1*m^-3, kg^-1*m^-5*s^2
         * tensor order: 0
         * definition: quantity given by the differential quotient with respect to energy: `n_E(E) = (dn(E))/(dE)`, where `n_E(E)` is the total number of one-electron states per volume (ISO 80000-3) with energy less than `E` (ISO 80000-5)
         * remarks: Density of states refers to electrons or other entities, e.g. phonons. It may be normalized in other ways instead of with respect to volume, e.g. with respect to amount of substance. See also item 12-12.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyDensityOfStatesUnit[1];
    }

    attribute energyDensityOfStates: EnergyDensityOfStatesValue[*] nonunique :> scalarQuantities;

    attribute def EnergyDensityOfStatesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -5; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-12 item 12-17 residual resistivity */
    attribute residualResistivity: ResistivityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-17 residual resistivity
         * symbol(s): `ρ_0`
         * application domain: generic
         * name: ResidualResistivity (specializes Resistivity)
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m, kg*m^3*s^-3*A^-2
         * tensor order: 0
         * definition: for metals, the resistivity (IEC 80000-6) extrapolated to zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-18 Lorenz coefficient */
    attribute def LorenzCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-18 Lorenz coefficient
         * symbol(s): `L`
         * application domain: generic
         * name: LorenzCoefficient
         * quantity dimension: L^4*M^2*T^-6*I^-2*Θ^-2
         * measurement unit(s): V^2/K^2, kg^2*m^4*s^-6*A^-2*K^-2
         * tensor order: 0
         * definition: quotient of thermal conductivity (ISO 80000-5), and the product of electric conductivity (IEC 80000-6) and thermodynamic temperature (ISO 80000-3)
         * remarks: The Lorenz coefficient can be expressed by `L = λ/(σT)`, where `λ` is thermal conductivity (ISO 80000-5), `σ` is electric conductivity (IEC 80000-6), and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LorenzCoefficientUnit[1];
    }

    attribute lorenzCoefficient: LorenzCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LorenzCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -6; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-19 Hall coefficient */
    attribute def HallCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-19 Hall coefficient
         * symbol(s): `R_H`, `A_H`
         * application domain: generic
         * name: HallCoefficient
         * quantity dimension: L^3*T^-1*I^-1
         * measurement unit(s): m^3/C, m^3*s^-1*A^-1
         * tensor order: 0
         * definition: in an isotropic conductor, relation between electric field strength, `vec(E)`, (IEC 80000-6) and electric current density, `vec(J)`, (IEC 80000-6) expressed as: `vec(E) = ρ vec(J) + R_H (vec(B) xx vec(J))`, where `ρ` is resistivity (IEC 80000-6), and `vec(B)` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HallCoefficientUnit[1];
    }

    attribute hallCoefficient: HallCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def HallCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-12 item 12-20 thermoelectric voltage (between substances a and b) */
    attribute thermoelectricVoltageBetweenSubstancesAAndB: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 12-20 thermoelectric voltage (between substances a and b)
         * symbol(s): `E_(ab)`
         * application domain: generic
         * name: ThermoelectricVoltageBetweenSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: voltage (IEC 80000-6) between substances `a` and `b` caused by the thermoelectric effect
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-21 Seebeck coefficient (for substances a and b) */
    attribute def SeebeckCoefficientForSubstancesAAndBValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-21 Seebeck coefficient (for substances a and b)
         * symbol(s): `S_(ab)`
         * application domain: generic
         * name: SeebeckCoefficientForSubstancesAAndB
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: differential quotient of thermoelectric voltage with respect to thermodynamic temperature: `S_(ab) =      (dE_(ab))/(dT)`, where `E_(ab)` is the thermoelectric voltage between substances `a` and `b` (item 12-20) and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: This term is also called "thermoelectric power".
         */
        attribute :>> num: Real;
        attribute :>> mRef: SeebeckCoefficientForSubstancesAAndBUnit[1];
    }

    attribute seebeckCoefficientForSubstancesAAndB: SeebeckCoefficientForSubstancesAAndBValue[*] nonunique :> scalarQuantities;

    attribute def SeebeckCoefficientForSubstancesAAndBUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-22 Peltier coefficient (for substances a and b) */
    attribute peltierCoefficientForSubstancesAAndB: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 12-22 Peltier coefficient (for substances a and b)
         * symbol(s): `Π_(ab)`
         * application domain: generic
         * name: PeltierCoefficientForSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: quotient of Peltier heat power (ISO 80000-5) developed at a junction, and the electric current (IEC 80000-6) flowing from substance `a` to substance `b`
         * remarks: `Π_(ab) = Π_a - Π_b`, where `Π_a` and `Π_b` are the Peltier coefficients of substances `a` and `b`, respectively.
         */
    }

    /* ISO-80000-12 item 12-23 Thomson coefficient */
    attribute def ThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-23 Thomson coefficient
         * symbol(s): `μ`
         * application domain: generic
         * name: ThomsonCoefficient
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: quotient of Thomson heat power (ISO 80000-5) developed, and the electric current (IEC 80000-6) and temperature (ISO 80000-5) difference
         * remarks: `μ` is positive if heat is developed when the temperature decreases in the direction of the electric current.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThomsonCoefficientUnit[1];
    }

    attribute thomsonCoefficient: ThomsonCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def ThomsonCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-24.1 work function */
    attribute workFunction: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-24.1 work function
         * symbol(s): `ϕ`
         * application domain: generic
         * name: WorkFunction (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and the Fermi energy (item 12-27.1)
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. The contact potential difference between substances `a` and `b` is given by `V_a - V_b = (ϕ_a - ϕ_b)/e`, where `e` is the elementary charge (ISO 80000-1). A set of energy levels, the energies of which occupy an interval practically continuously, is called an energy band. In semi-conductors `E_d` and `E_a` are used for donors and acceptors, respectively.
         */
    }

    /* ISO-80000-12 item 12-24.2 ionization energy */
    attribute ionizationEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-24.2 ionization energy
         * symbol(s): `E_i`
         * application domain: generic
         * name: IonizationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and a certain energy level which is the energy of an electron in the interior of a substance
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-25 electron affinity */
    attribute electronAffinity: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-25 electron affinity
         * symbol(s): `χ`
         * application domain: condensed matter physics
         * name: ElectronAffinity (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) difference between an electron at rest at infinity and an electron at the lowest level of the conduction band in an insulator or semiconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-26 Richardson constant */
    attribute def RichardsonConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-26 Richardson constant
         * symbol(s): `A`
         * application domain: generic
         * name: RichardsonConstant
         * quantity dimension: L^-2*I^1*Θ^-2
         * measurement unit(s): A*m^-2*K^-2
         * tensor order: 0
         * definition: parameter in the expression for the thermionic emission current density `J` (IEC 80000-6) for a metal in terms of the thermodynamic temperature `T` (ISO 80000-5) and work function `ϕ`, (item 12-24.1): `J = AT^2 exp(ϕ/(kT))`, where `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RichardsonConstantUnit[1];
    }

    attribute richardsonConstant: RichardsonConstantValue[*] nonunique :> scalarQuantities;

    attribute def RichardsonConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-12 item 12-27.1 Fermi energy */
    attribute fermiEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-27.1 Fermi energy
         * symbol(s): `E_F`
         * application domain: generic
         * name: FermiEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a metal, highest occupied energy level at zero thermodynamic temperature (ISO 80000-5), where energy level means the energy (ISO 80000-5) of an electron in the interior of a substance
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. At `T = 0 [K]`, `E_F` is equal to the chemical potential per electron. In condensed matter physics, the reference level for the energy is sometimes chosen so that, for instance, `E_F = 0`.
         */
    }

    /* ISO-80000-12 item 12-27.2 gap energy */
    attribute gapEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-27.2 gap energy
         * symbol(s): `E_g`
         * application domain: generic
         * name: GapEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference in energy (ISO 80000-5) between the lowest level of conduction band and the highest level of valence band at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-28 Fermi temperature */
    attribute fermiTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-28 Fermi temperature
         * symbol(s): `T_F`
         * application domain: generic
         * name: FermiTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the free electron model, the Fermi energy (item 12-27.1) divided by the Boltzmann constant (ISO 80000-1)
         * remarks: The Fermi temperature is expressed by: `T_F = E_F/k`, where `E_F` is Fermi energy (item 12-27.1) and `k` is the Boltzmann constant (ISO 80000-1). `E_F` is relative to the lowest occupied state.
         */
    }

    /* ISO-80000-12 item 12-29.1 electron density */
    attribute def ElectronDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.1 electron density
         * symbol(s): `n`
         * application domain: generic
         * name: ElectronDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of electrons in conduction band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectronDensityUnit[1];
    }

    attribute electronDensity: ElectronDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectronDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.2 hole density */
    attribute def HoleDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.2 hole density
         * symbol(s): `p`
         * application domain: generic
         * name: HoleDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of holes in valence band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HoleDensityUnit[1];
    }

    attribute holeDensity: HoleDensityValue[*] nonunique :> scalarQuantities;

    attribute def HoleDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.3 intrinsic carrier density */
    attribute def IntrinsicCarrierDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.3 intrinsic carrier density
         * symbol(s): `n_i`
         * application domain: generic
         * name: IntrinsicCarrierDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quantity given by: `n_i = sqrt(n p)`, where `n` is electron density (item 12-29.1), and `p` is hole
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IntrinsicCarrierDensityUnit[1];
    }

    attribute intrinsicCarrierDensity: IntrinsicCarrierDensityValue[*] nonunique :> scalarQuantities;

    attribute def IntrinsicCarrierDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.4 donor density */
    attribute def DonorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.4 donor density
         * symbol(s): `n_d`
         * application domain: generic
         * name: DonorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of donor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DonorDensityUnit[1];
    }

    attribute donorDensity: DonorDensityValue[*] nonunique :> scalarQuantities;

    attribute def DonorDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-29.5 acceptor density */
    attribute def AcceptorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.5 acceptor density
         * symbol(s): `n_a`
         * application domain: generic
         * name: AcceptorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of acceptor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AcceptorDensityUnit[1];
    }

    attribute acceptorDensity: AcceptorDensityValue[*] nonunique :> scalarQuantities;

    attribute def AcceptorDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-12 item 12-30 effective mass */
    attribute effectiveMass: MassValue :> scalarQuantities {
        doc
        /*
         * source: item 12-30 effective mass
         * symbol(s): `m"*"`
         * application domain: generic
         * name: EffectiveMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: quantity given by: `m^"*" = (ħ^2 k) / ((dε)/(dk))`, where `k` is wavenumber (ISO 80000-3), `ε` is the energy (ISO 80000-5) of an electron in the interior of a substance, and `ħ` is the reduced Planck constant (ISO 80000-1)
         * remarks: When `k` refers to a state where `ε` has an extremum, `m"*" = (ħ^2 k) / ((d^2ε)/(dk^2))`. The effective mass can be generalized to refer to an anisotropic system with `ε = ε(k)`.
         */
    }

    /* ISO-80000-12 item 12-31 mobility ratio */
    attribute def MobilityRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-31 mobility ratio
         * symbol(s): `b`
         * application domain: generic
         * name: MobilityRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mobilities (ISO 80000-10) of electrons and holes, respectively
         * remarks: The mobility ratio can be expressed by: `b = μ_n/μ_p`, where `μ_n` and `μ_p` are mobilities (ISO 80000-10) for electrons and holes, respectively.
         */
    }
    attribute mobilityRatio: MobilityRatioValue :> scalarQuantities;

    /* ISO-80000-12 item 12-32.1 relaxation time */
    attribute relaxationTime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 12-32.1 relaxation time
         * symbol(s): `τ`
         * application domain: condensed matter physics
         * name: RelaxationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for scattering, trapping or annihilation of charge carriers, phonons or other quasiparticles
         * remarks: For electrons in metals, `τ = l/v_F`, where `l` is mean free path (item 12-15.2) and `v_F` is speed (ISO 80000-3) of electrons on the Fermi surface.
         */
    }

    /* ISO-80000-12 item 12-32.2 carrier lifetime */
    attribute carrierLifetime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 12-32.2 carrier lifetime
         * symbol(s): `τ`, `τ_n`, `τ_p`
         * application domain: semiconductors
         * name: CarrierLifetime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for recombination or trapping of minority charge carriers in semiconductors
         * remarks: Indices "n" and "p" denote negative and positive charge carriers, respectively. Positive charge carriers can also be holes.
         */
    }

    /* ISO-80000-12 item 12-33 diffusion length */
    attribute diffusionLengthForCondensedMatterPhysics: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-33 diffusion length
         * symbol(s): `L`, `L_n`, `L_p`
         * application domain: condensed matter physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the product of diffusion coefficient (ISO 80000-10) and lifetime (ISO 80000-10)
         * remarks: The diffusion length can be expressed by: `L = sqrt(Dτ)`, where `D` is the diffusion coefficient (ISO 80000-9) and `τ` is lifetime (ISO 80000-3).
         */
    }

    /* ISO-80000-12 item 12-34 exchange integral */
    attribute exchangeIntegral: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-34 exchange integral
         * symbol(s): `K`, `J`
         * application domain: generic
         * name: ExchangeIntegral (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: constituent of the interaction energy (ISO 80000-5) between the spins of adjacent electrons in matter arising from the overlap of electron state functions
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.1 Curie temperature */
    attribute curieTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.1 Curie temperature
         * symbol(s): `T_C`
         * application domain: generic
         * name: CurieTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a ferromagnet
         * remarks: `T_(cr)` is used for critical thermodynamic temperature in general.
         */
    }

    /* ISO-80000-12 item 12-35.2 Néel temperature */
    attribute 'néelTemperature': ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.2 Néel temperature
         * symbol(s): `T_N`
         * application domain: generic
         * name: NéelTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of an antiferromagnet
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-35.3 superconduction transition temperature */
    attribute superconductionTransitionTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 12-35.3 superconduction transition temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: SuperconductionTransitionTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.1 thermodynamic critical magnetic flux density */
    attribute thermodynamicCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.1 thermodynamic critical magnetic flux density
         * symbol(s): `B_c`
         * application domain: generic
         * name: ThermodynamicCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: quantity given by: `B_c = sqrt((2μ_0 (G_n - G_s))/V)`, where `G_n` and `G_s` are the Gibbs energies (ISO 80000-5) at zero magnetic flux density (IEC 80000-6) in a normal conductor and superconductor, respectively, `μ_0` is the magnetic constant (IEC 80000-6), and `V` is volume (ISO 80000-3)
         * remarks: In type I superconductors, `B_c` is the critical magnetic flux density for disappearance of superconductivity. The symbol `B_(c3)` is used for the critical magnetic flux density for disappearance of surface superconductivity.
         */
    }

    /* ISO-80000-12 item 12-36.2 lower critical magnetic flux density */
    attribute lowerCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.2 lower critical magnetic flux density
         * symbol(s): `B_(c1)`
         * application domain: generic
         * name: LowerCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for magnetic flux (IEC 80000-6) entering the superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-36.3 upper critical magnetic flux density */
    attribute upperCriticalMagneticFluxDensity: MagneticFluxDensityValue :> scalarQuantities {
        doc
        /*
         * source: item 12-36.3 upper critical magnetic flux density
         * symbol(s): `B_(c2)`
         * application domain: generic
         * name: UpperCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for disappearance of bulk superconductivity
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-37 superconductor energy gap */
    attribute superconductorEnergyGap: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 12-37 superconductor energy gap
         * symbol(s): `Δ`
         * application domain: generic
         * name: SuperconductorEnergyGap (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: width of the forbidden energy band (item 12-24.2) in a superconductor
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.1 London penetration depth */
    attribute londonPenetrationDepth: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-38.1 London penetration depth
         * symbol(s): `λ_L`
         * application domain: generic
         * name: LondonPenetrationDepth (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) a magnetic field penetrates the plane surface of a semi-finite superconductor according to the expression: `B(x) = B(0) exp(-x/λ_L)`, where `B` is magnetic flux density (IEC 80000-6) and `x` is distance (ISO 80000-3) from the surface
         * remarks: None.
         */
    }

    /* ISO-80000-12 item 12-38.2 coherence length */
    attribute coherenceLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 12-38.2 coherence length
         * symbol(s): `ξ`
         * application domain: generic
         * name: CoherenceLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) in a superconductor over which the effect of a perturbation is appreciable at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_condensed_matter.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQCondensedMatter {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-12:2019 "Condensed matter physics"
     * see also https://www.iso.org/standard/63480.html
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
    private import ISQElectromagnetism::ElectricPotentialDifferenceValue;
    private import ISQElectromagnetism::MagneticFluxDensityValue;
    private import ISQElectromagnetism::ResistivityValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::AngularFrequencyValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQSpaceTime::RepetencyValue;
    private import ISQThermodynamics::EnergyValue;
    attribute def CartesianLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.1 lattice vector
         * symbol(s): `vec(R)`
         * application domain: generic
         * name: LatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: translation vector that maps the crystal lattice on itself
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianLattice3dVector : CartesianLattice3dVector;
    attribute def CartesianFundamentalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-1.2 fundamental lattice vector
         * symbol(s): `vec(a_1),vec(a_2),vec(a_3)`, `vec(a),vec(b),vec(c)`
         * application domain: generic
         * name: FundamentalLatticeVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: fundamental translation vectors for the crystal lattice
         * remarks: The lattice vector (item 12-1.1) can be given as `vec(R) = n_1 vec(a_1) + n_2 vec(a_2) + n_3 vec(a_3)` where `n_1`, `n_2` and `n_3` are integers.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianFundamentalLattice3dVector : CartesianFundamentalLattice3dVector;
    attribute def AngularReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector (magnitude)
         * symbol(s): `G`
         * application domain: generic
         * name: AngularReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularReciprocalLatticeVectorMagnitudeUnit[1];
    }
    attribute def angularReciprocalLatticeVectorMagnitude : AngularReciprocalLatticeVectorMagnitudeValue[*] nonunique;
    attribute def AngularReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def CartesianAngularReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.1 angular reciprocal lattice vector
         * symbol(s): `vec(G)`
         * application domain: generic
         * name: AngularReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector whose scalar products with all fundamental lattice vectors are integral multiples of  `2π`
         * remarks: In crystallography, however, the quantity `G/(2π)` is sometimes used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularReciprocalLattice3dCoordinateFrame[1];
    }
    attribute def cartesianAngularReciprocalLattice3dVector : CartesianAngularReciprocalLattice3dVector;
    attribute def CartesianAngularReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularReciprocalLatticeVectorMagnitudeUnit[3];
    }
    attribute def FundamentalReciprocalLatticeVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector (magnitude)
         * symbol(s): `b_1,b_2,b_3`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVectorMagnitude
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : FundamentalReciprocalLatticeVectorMagnitudeUnit[1];
    }
    attribute def fundamentalReciprocalLatticeVectorMagnitude : FundamentalReciprocalLatticeVectorMagnitudeValue[*] nonunique;
    attribute def FundamentalReciprocalLatticeVectorMagnitudeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def CartesianFundamentalReciprocalLattice3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-2.2 fundamental reciprocal lattice vector
         * symbol(s): `vec(b_1),vec(b_2),vec(b_3)`
         * application domain: generic
         * name: FundamentalReciprocalLatticeVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: fundamental translation vectors for the reciprocal lattice
         * remarks: `vec(a_i) * vec(b_i) = 2π δ_(ij)`. In crystallography, however, the quantities `vec(b_j)/(2π)` are also often used.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianFundamentalReciprocalLattice3dCoordinateFrame[1];
    }
    attribute def cartesianFundamentalReciprocalLattice3dVector : CartesianFundamentalReciprocalLattice3dVector;
    attribute def CartesianFundamentalReciprocalLattice3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : FundamentalReciprocalLatticeVectorMagnitudeUnit[3];
    }
    attribute def latticePlaneSpacing : LengthValue {
        doc
        /*
         * source: item 12-3 lattice plane spacing
         * symbol(s): `d`
         * application domain: generic
         * name: LatticePlaneSpacing (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) between successive lattice planes
         * remarks: The non-SI unit ångström (Å) is widely used by x-ray crystallographers and structural chemists.
         */
    }
    attribute def braggAngle : AngularMeasureValue {
        doc
        /*
         * source: item 12-4 Bragg angle
         * symbol(s): `ϑ`
         * application domain: generic
         * name: BraggAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): °, 1
         * tensor order: 0
         * definition: angle between the scattered ray and the lattice plane
         * remarks: Bragg angle `ϑ` is given by `2d sin ϑ = nλ`, where `d` is the lattice plane spacing (item 12-3), `λ` is the wavelength (ISO 80000-7) of the radiation, and `n` is the order of reflexion which is an integer.
         */
    }
    attribute def ShortRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.1 short-range order parameter
         * symbol(s): `r`, `σ`
         * application domain: generic
         * name: ShortRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of nearest-neighbour atom pairs in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute def shortRangeOrderParameter : ShortRangeOrderParameterValue;
    attribute def LongRangeOrderParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.2 long-range order parameter
         * symbol(s): `R`, `s`
         * application domain: generic
         * name: LongRangeOrderParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: fraction of atoms in an Ising ferromagnet having magnetic moments in one direction, minus the fraction having magnetic moments in the opposite direction
         * remarks: Similar definitions apply to other order-disorder phenomena. Other symbols are frequently used.
         */
    }
    attribute def longRangeOrderParameter : LongRangeOrderParameterValue;
    attribute def AtomicScatteringFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.3 atomic scattering factor
         * symbol(s): `f`
         * application domain: generic
         * name: AtomicScatteringFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiation amplitude scattered by the atom and radiation amplitude scattered by a single electron
         * remarks: The atomic scattering factor can be expressed by: `f = E_a/(E_e`, where `E_a` is the radiation amplitude scattered by the atom and `E_e` is the radiation amplitude scattered by a single electron.
         */
    }
    attribute def atomicScatteringFactor : AtomicScatteringFactorValue;
    attribute def StructureFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-5.4 structure factor
         * symbol(s): `F(h,k,l)`
         * application domain: generic
         * name: StructureFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `F(h,k,l) = sum_(n=1)^N f_n exp[2π i (h x_n + k y_n + l z_n)]`, where `f_n` is the atomic scattering factor (item 12-5.3) for atom `n`, `x_n`, `y_n`, `z_n` are fractional coordinates of its position, `N` is the total number of atoms in the unit cell and `h`, `k`, `l` are the Miller indices
         * remarks: For the Miller indices `h`, `k`, `l`, see Annex A.
         */
    }
    attribute def structureFactor : StructureFactorValue;
    attribute def CartesianBurgers3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-6 Burgers vector
         * symbol(s): `vec(b)`
         * application domain: generic
         * name: BurgersVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: closing vector in a sequence of vectors encircling a dislocation
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianBurgers3dVector : CartesianBurgers3dVector;
    attribute def CartesianParticlePosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.1 particle position vector
         * symbol(s): `vec(r)`, `vec(R)`
         * application domain: generic
         * name: ParticlePositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of a particle
         * remarks: Often, `r` is used for electrons and `R` is used for atoms and other heavier particles.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianParticlePosition3dVector : CartesianParticlePosition3dVector;
    attribute def CartesianEquilibriumPosition3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.2 equilibrium position vector
         * symbol(s): `vec(R_0)`
         * application domain: condensed matter physics
         * name: EquilibriumPositionVector (specializes PositionVector)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: position vector (ISO 80000-3) of an ion or atom in equilibrium
         * remarks: None.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianEquilibriumPosition3dVector : CartesianEquilibriumPosition3dVector;
    attribute def CartesianDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 12-7.3 displacement vector
         * symbol(s): `vec(u)`
         * application domain: condensed matter physics
         * name: DisplacementVector (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: difference between the position vector (ISO 80000-3) of an ion or atom and its position vector in equilibrium
         * remarks: The displacement vector can be expressed by: `vec(u) = vec(R) − vec(R_0)`, where `vec(R)` is particle position vector (item 12-7.1) and `vec(R_0)` is position vector of an ion or atom in equilibrium (item 12-7.2).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianDisplacement3dVector : CartesianDisplacement3dVector;
    attribute def DebyeWallerFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-8 Debye-Waller factor
         * symbol(s): `D`, `B`
         * application domain: generic
         * name: DebyeWallerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor by which the intensity of a diffraction line is reduced because of the lattice vibrations
         * remarks: `D` is sometimes expressed as `D = exp(−2W)`; in Mössbauer spectroscopy, it is also called the `f` factor and denoted by `f`.
         */
    }
    attribute def debyeWallerFactor : DebyeWallerFactorValue;
    attribute def angularWavenumber : RepetencyValue {
        doc
        /*
         * source: item 12-9.1 angular wavenumber, angular repetency
         * symbol(s): `k`, `q`
         * application domain: condensed matter physics
         * name: AngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: quotient of momentum (ISO 80000-4) and the reduced Planck constant (ISO 80000-1)
         * remarks: The corresponding vector (ISO 80000-2) quantity is called wave vector (ISO 80000-3), expressed by: `vec(k) = vec(p)/ħ`, where `vec(p)` is the momentum (ISO 80000-4) of quasi free electrons in an electron gas, and `ħ` is the reduced Planck constant (ISO 80000-1); for phonons, its magnitude is `k = 2π/λ`, where `λ` is the wavelength (ISO 80000-3) of the lattice vibrations. When a distinction is needed between `k` and the symbol for the Boltzmann constant (ISO 80000-1), `k_B` can be used for the latter. When a distinction is needed, `q` should be used for phonons, and `k` for particles such as electrons and neutrons. The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }
    alias angularRepetency for angularWavenumber;
    attribute def fermiAngularWavenumber : RepetencyValue {
        doc
        /*
         * source: item 12-9.2 Fermi angular wavenumber, Fermi angular repetency
         * symbol(s): `k_F`
         * application domain: generic
         * name: FermiAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: angular wavenumber (item 12-9.1) of electrons in states on the Fermi sphere
         * remarks: In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }
    alias fermiAngularRepetency for fermiAngularWavenumber;
    attribute def debyeAngularWavenumber : RepetencyValue {
        doc
        /*
         * source: item 12-9.3 Debye angular wavenumber, Debye angular repetency
         * symbol(s): `q_D`
         * application domain: generic
         * name: DebyeAngularWavenumber (specializes Repetency)
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: cut-off angular wavenumber (item 12-9.1) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified. In condensed matter physics, angular wavenumber is often called wavenumber.
         */
    }
    alias debyeAngularRepetency for debyeAngularWavenumber;
    attribute def debyeAngularFrequency : AngularFrequencyValue {
        doc
        /*
         * source: item 12-10 Debye angular frequency
         * symbol(s): `ω_D`
         * application domain: generic
         * name: DebyeAngularFrequency (specializes AngularFrequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: cut-off angular frequency (ISO 80000-3) in the Debye model of the vibrational spectrum of a solid
         * remarks: The method of cut-off must be specified.
         */
    }
    attribute def debyeTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 12-11 Debye temperature
         * symbol(s): `Θ_D`
         * application domain: generic
         * name: DebyeTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the Debye model, quantity given by: `Θ_D = ħ*ω_D/k`, where `k` is the Boltzmann constant, (ISO 80000-1), `ħ` is the reduced Planck constant (ISO 80000-1), and `ω_D` is Debye angular frequency (item 12-10)
         * remarks: A Debye temperature can also be defined by fitting a Debye model result to a certain quantity, for instance, the heat capacity at a certain temperature.
         */
    }
    attribute def DensityOfVibrationalStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-12 density of vibrational states
         * symbol(s): `g`
         * application domain: angular frequency
         * name: DensityOfVibrationalStates
         * quantity dimension: L^-3*T^1
         * measurement unit(s): m^-3*s
         * tensor order: 0
         * definition: quotient of the number of vibrational modes in an infinitesimal interval of angular frequency (ISO 80000-3), and the product of the width of that interval and volume (ISO 80000-3)
         * remarks: `g(ω) = n_ω = (dn(ω))/(dω)`, where `n(ω)` is the total number of vibrational modes per volume with angular frequency less than `ω`. The density of states may also be normalized in other ways instead of with respect to volume. See also item 12-16.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DensityOfVibrationalStatesUnit[1];
    }
    attribute def densityOfVibrationalStates : DensityOfVibrationalStatesValue[*] nonunique;
    attribute def DensityOfVibrationalStatesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def 'ThermodynamicGrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-13 thermodynamic Grüneisen parameter
         * symbol(s): `γ_G`, `Γ_G`
         * application domain: generic
         * name: ThermodynamicGrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `γ_G = (α_V)/(κ_T c_V ρ)`, where `α_V` is cubic expansion coefficient (ISO 80000-5), `κ_T` is isothermal compressibility (ISO 80000-5), `c_V` is specific heat capacity at constant volume (ISO 80000-5), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute def 'thermodynamicGrüneisenParameter' : 'ThermodynamicGrüneisenParameterValue';
    attribute def 'GrüneisenParameterValue' :> DimensionOneValue {
        doc
        /*
         * source: item 12-14 Grüneisen parameter
         * symbol(s): `γ`
         * application domain: generic
         * name: GrüneisenParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by minus the partial differential quotient: `γ = -(del ln ω)/(del ln V)`, where `ω` is a lattice vibration frequency (ISO 80000-3), and `V` is volume (ISO 80000-3)
         * remarks: `ω` can also refer to an average of the vibrational spectrum, for instance as represented by a Debye angular frequency (item 12-10).
         */
    }
    attribute def 'grüneisenParameter' : 'GrüneisenParameterValue';
    attribute def meanFreePathOfPhonons : LengthValue {
        doc
        /*
         * source: item 12-15.1 mean free path of phonons
         * symbol(s): `l_p`
         * application domain: generic
         * name: MeanFreePathOfPhonons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that phonons travel between two successive interactions
         * remarks: None.
         */
    }
    attribute def meanFreePathOfElectrons : LengthValue {
        doc
        /*
         * source: item 12-15.2 mean free path of electrons
         * symbol(s): `l_e`
         * application domain: generic
         * name: MeanFreePathOfElectrons (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: average distance (ISO 80000-3) that electrons travel between two successive interactions
         * remarks: None.
         */
    }
    attribute def EnergyDensityOfStatesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-16 energy density of states
         * symbol(s): `n_E(E)`, `ρ(E)`
         * application domain: generic
         * name: EnergyDensityOfStates
         * quantity dimension: L^-5*M^-1*T^2
         * measurement unit(s): J^-1*m^-3*eV^-1*m^-3, kg^-1*m^-5*s^2
         * tensor order: 0
         * definition: quantity given by the differential quotient with respect to energy: `n_E(E) = (dn(E))/(dE)`, where `n_E(E)` is the total number of one-electron states per volume (ISO 80000-3) with energy less than `E` (ISO 80000-5)
         * remarks: Density of states refers to electrons or other entities, e.g. phonons. It may be normalized in other ways instead of with respect to volume, e.g. with respect to amount of substance. See also item 12-12.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyDensityOfStatesUnit[1];
    }
    attribute def energyDensityOfStates : EnergyDensityOfStatesValue[*] nonunique;
    attribute def EnergyDensityOfStatesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -5;
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
    attribute def residualResistivity : ResistivityValue {
        doc
        /*
         * source: item 12-17 residual resistivity
         * symbol(s): `ρ_0`
         * application domain: generic
         * name: ResidualResistivity (specializes Resistivity)
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m, kg*m^3*s^-3*A^-2
         * tensor order: 0
         * definition: for metals, the resistivity (IEC 80000-6) extrapolated to zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute def LorenzCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-18 Lorenz coefficient
         * symbol(s): `L`
         * application domain: generic
         * name: LorenzCoefficient
         * quantity dimension: L^4*M^2*T^-6*I^-2*Θ^-2
         * measurement unit(s): V^2/K^2, kg^2*m^4*s^-6*A^-2*K^-2
         * tensor order: 0
         * definition: quotient of thermal conductivity (ISO 80000-5), and the product of electric conductivity (IEC 80000-6) and thermodynamic temperature (ISO 80000-3)
         * remarks: The Lorenz coefficient can be expressed by `L = λ/(σT)`, where `λ` is thermal conductivity (ISO 80000-5), `σ` is electric conductivity (IEC 80000-6), and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LorenzCoefficientUnit[1];
    }
    attribute def lorenzCoefficient : LorenzCoefficientValue[*] nonunique;
    attribute def LorenzCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 4;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -6;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }
    attribute def HallCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-19 Hall coefficient
         * symbol(s): `R_H`, `A_H`
         * application domain: generic
         * name: HallCoefficient
         * quantity dimension: L^3*T^-1*I^-1
         * measurement unit(s): m^3/C, m^3*s^-1*A^-1
         * tensor order: 0
         * definition: in an isotropic conductor, relation between electric field strength, `vec(E)`, (IEC 80000-6) and electric current density, `vec(J)`, (IEC 80000-6) expressed as: `vec(E) = ρ vec(J) + R_H (vec(B) xx vec(J))`, where `ρ` is resistivity (IEC 80000-6), and `vec(B)` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HallCoefficientUnit[1];
    }
    attribute def hallCoefficient : HallCoefficientValue[*] nonunique;
    attribute def HallCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }
    attribute def thermoelectricVoltageBetweenSubstancesAAndB : ElectricPotentialDifferenceValue {
        doc
        /*
         * source: item 12-20 thermoelectric voltage (between substances a and b)
         * symbol(s): `E_(ab)`
         * application domain: generic
         * name: ThermoelectricVoltageBetweenSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: voltage (IEC 80000-6) between substances `a` and `b` caused by the thermoelectric effect
         * remarks: None.
         */
    }
    attribute def SeebeckCoefficientForSubstancesAAndBValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-21 Seebeck coefficient (for substances a and b)
         * symbol(s): `S_(ab)`
         * application domain: generic
         * name: SeebeckCoefficientForSubstancesAAndB
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: differential quotient of thermoelectric voltage with respect to thermodynamic temperature: `S_(ab) =      (dE_(ab))/(dT)`, where `E_(ab)` is the thermoelectric voltage between substances `a` and `b` (item 12-20) and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: This term is also called "thermoelectric power".
         */
        attribute :>> num : Real;
        attribute :>> mRef : SeebeckCoefficientForSubstancesAAndBUnit[1];
    }
    attribute def seebeckCoefficientForSubstancesAAndB : SeebeckCoefficientForSubstancesAAndBValue[*] nonunique;
    attribute def SeebeckCoefficientForSubstancesAAndBUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = -1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }
    attribute def peltierCoefficientForSubstancesAAndB : ElectricPotentialDifferenceValue {
        doc
        /*
         * source: item 12-22 Peltier coefficient (for substances a and b)
         * symbol(s): `Π_(ab)`
         * application domain: generic
         * name: PeltierCoefficientForSubstancesAAndB (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V, kg*m^2*s^-3*A^-1
         * tensor order: 0
         * definition: quotient of Peltier heat power (ISO 80000-5) developed at a junction, and the electric current (IEC 80000-6) flowing from substance `a` to substance `b`
         * remarks: `Π_(ab) = Π_a - Π_b`, where `Π_a` and `Π_b` are the Peltier coefficients of substances `a` and `b`, respectively.
         */
    }
    attribute def ThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-23 Thomson coefficient
         * symbol(s): `μ`
         * application domain: generic
         * name: ThomsonCoefficient
         * quantity dimension: L^2*M^1*T^-3*I^-1*Θ^-1
         * measurement unit(s): V/K, kg*m^2*s^-3*A^-1*K^-1
         * tensor order: 0
         * definition: quotient of Thomson heat power (ISO 80000-5) developed, and the electric current (IEC 80000-6) and temperature (ISO 80000-5) difference
         * remarks: `μ` is positive if heat is developed when the temperature decreases in the direction of the electric current.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThomsonCoefficientUnit[1];
    }
    attribute def thomsonCoefficient : ThomsonCoefficientValue[*] nonunique;
    attribute def ThomsonCoefficientUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = -1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }
    attribute def workFunction : EnergyValue {
        doc
        /*
         * source: item 12-24.1 work function
         * symbol(s): `ϕ`
         * application domain: generic
         * name: WorkFunction (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and the Fermi energy (item 12-27.1)
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. The contact potential difference between substances `a` and `b` is given by `V_a - V_b = (ϕ_a - ϕ_b)/e`, where `e` is the elementary charge (ISO 80000-1). A set of energy levels, the energies of which occupy an interval practically continuously, is called an energy band. In semi-conductors `E_d` and `E_a` are used for donors and acceptors, respectively.
         */
    }
    attribute def ionizationEnergy : EnergyValue {
        doc
        /*
         * source: item 12-24.2 ionization energy
         * symbol(s): `E_i`
         * application domain: generic
         * name: IonizationEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between energy (ISO 80000-5) of an electron at rest at infinity and a certain energy level which is the energy of an electron in the interior of a substance
         * remarks: None.
         */
    }
    attribute def electronAffinity : EnergyValue {
        doc
        /*
         * source: item 12-25 electron affinity
         * symbol(s): `χ`
         * application domain: condensed matter physics
         * name: ElectronAffinity (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) difference between an electron at rest at infinity and an electron at the lowest level of the conduction band in an insulator or semiconductor
         * remarks: None.
         */
    }
    attribute def RichardsonConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-26 Richardson constant
         * symbol(s): `A`
         * application domain: generic
         * name: RichardsonConstant
         * quantity dimension: L^-2*I^1*Θ^-2
         * measurement unit(s): A*m^-2*K^-2
         * tensor order: 0
         * definition: parameter in the expression for the thermionic emission current density `J` (IEC 80000-6) for a metal in terms of the thermodynamic temperature `T` (ISO 80000-5) and work function `ϕ`, (item 12-24.1): `J = AT^2 exp(ϕ/(kT))`, where `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RichardsonConstantUnit[1];
    }
    attribute def richardsonConstant : RichardsonConstantValue[*] nonunique;
    attribute def RichardsonConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 1;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, electricCurrentPF, thermodynamicTemperaturePF);
        }
    }
    attribute def fermiEnergy : EnergyValue {
        doc
        /*
         * source: item 12-27.1 Fermi energy
         * symbol(s): `E_F`
         * application domain: generic
         * name: FermiEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: in a metal, highest occupied energy level at zero thermodynamic temperature (ISO 80000-5), where energy level means the energy (ISO 80000-5) of an electron in the interior of a substance
         * remarks: The term "energy level" is often used for the state of the electron, not only for its energy. At `T = 0 [K]`, `E_F` is equal to the chemical potential per electron. In condensed matter physics, the reference level for the energy is sometimes chosen so that, for instance, `E_F = 0`.
         */
    }
    attribute def gapEnergy : EnergyValue {
        doc
        /*
         * source: item 12-27.2 gap energy
         * symbol(s): `E_g`
         * application domain: generic
         * name: GapEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference in energy (ISO 80000-5) between the lowest level of conduction band and the highest level of valence band at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute def fermiTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 12-28 Fermi temperature
         * symbol(s): `T_F`
         * application domain: generic
         * name: FermiTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: in the free electron model, the Fermi energy (item 12-27.1) divided by the Boltzmann constant (ISO 80000-1)
         * remarks: The Fermi temperature is expressed by: `T_F = E_F/k`, where `E_F` is Fermi energy (item 12-27.1) and `k` is the Boltzmann constant (ISO 80000-1). `E_F` is relative to the lowest occupied state.
         */
    }
    attribute def ElectronDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.1 electron density
         * symbol(s): `n`
         * application domain: generic
         * name: ElectronDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of electrons in conduction band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectronDensityUnit[1];
    }
    attribute def electronDensity : ElectronDensityValue[*] nonunique;
    attribute def ElectronDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def HoleDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.2 hole density
         * symbol(s): `p`
         * application domain: generic
         * name: HoleDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of holes in valence band and volume (ISO 80000-3)
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HoleDensityUnit[1];
    }
    attribute def holeDensity : HoleDensityValue[*] nonunique;
    attribute def HoleDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def IntrinsicCarrierDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.3 intrinsic carrier density
         * symbol(s): `n_i`
         * application domain: generic
         * name: IntrinsicCarrierDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quantity given by: `n_i = sqrt(n p)`, where `n` is electron density (item 12-29.1), and `p` is hole
         * remarks: Subscripts `n` and `p` or `-` and `+` are often used to denote electrons and holes, respectively. `n_n` and `n_p` are also used for electron densities, and `p_n` and `p_p` for hole densities, in `n`-type and `p`-type regions, respectively, of a `n`-`p` junction.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IntrinsicCarrierDensityUnit[1];
    }
    attribute def intrinsicCarrierDensity : IntrinsicCarrierDensityValue[*] nonunique;
    attribute def IntrinsicCarrierDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def DonorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.4 donor density
         * symbol(s): `n_d`
         * application domain: generic
         * name: DonorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of donor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DonorDensityUnit[1];
    }
    attribute def donorDensity : DonorDensityValue[*] nonunique;
    attribute def DonorDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def AcceptorDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 12-29.5 acceptor density
         * symbol(s): `n_a`
         * application domain: generic
         * name: AcceptorDensity
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number of acceptor levels and volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AcceptorDensityUnit[1];
    }
    attribute def acceptorDensity : AcceptorDensityValue[*] nonunique;
    attribute def AcceptorDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def effectiveMass : MassValue {
        doc
        /*
         * source: item 12-30 effective mass
         * symbol(s): `m"*"`
         * application domain: generic
         * name: EffectiveMass (specializes Mass)
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: quantity given by: `m^"*" = (ħ^2 k) / ((dε)/(dk))`, where `k` is wavenumber (ISO 80000-3), `ε` is the energy (ISO 80000-5) of an electron in the interior of a substance, and `ħ` is the reduced Planck constant (ISO 80000-1)
         * remarks: When `k` refers to a state where `ε` has an extremum, `m"*" = (ħ^2 k) / ((d^2ε)/(dk^2))`. The effective mass can be generalized to refer to an anisotropic system with `ε = ε(k)`.
         */
    }
    attribute def MobilityRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 12-31 mobility ratio
         * symbol(s): `b`
         * application domain: generic
         * name: MobilityRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mobilities (ISO 80000-10) of electrons and holes, respectively
         * remarks: The mobility ratio can be expressed by: `b = μ_n/μ_p`, where `μ_n` and `μ_p` are mobilities (ISO 80000-10) for electrons and holes, respectively.
         */
    }
    attribute def mobilityRatio : MobilityRatioValue;
    attribute def relaxationTime : DurationValue {
        doc
        /*
         * source: item 12-32.1 relaxation time
         * symbol(s): `τ`
         * application domain: condensed matter physics
         * name: RelaxationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for scattering, trapping or annihilation of charge carriers, phonons or other quasiparticles
         * remarks: For electrons in metals, `τ = l/v_F`, where `l` is mean free path (item 12-15.2) and `v_F` is speed (ISO 80000-3) of electrons on the Fermi surface.
         */
    }
    attribute def carrierLifetime : DurationValue {
        doc
        /*
         * source: item 12-32.2 carrier lifetime
         * symbol(s): `τ`, `τ_n`, `τ_p`
         * application domain: semiconductors
         * name: CarrierLifetime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time constant (ISO 80000-3) for recombination or trapping of minority charge carriers in semiconductors
         * remarks: Indices "n" and "p" denote negative and positive charge carriers, respectively. Positive charge carriers can also be holes.
         */
    }
    attribute def diffusionLengthForCondensedMatterPhysics : LengthValue {
        doc
        /*
         * source: item 12-33 diffusion length
         * symbol(s): `L`, `L_n`, `L_p`
         * application domain: condensed matter physics
         * name: DiffusionLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: square root of the product of diffusion coefficient (ISO 80000-10) and lifetime (ISO 80000-10)
         * remarks: The diffusion length can be expressed by: `L = sqrt(Dτ)`, where `D` is the diffusion coefficient (ISO 80000-9) and `τ` is lifetime (ISO 80000-3).
         */
    }
    attribute def exchangeIntegral : EnergyValue {
        doc
        /*
         * source: item 12-34 exchange integral
         * symbol(s): `K`, `J`
         * application domain: generic
         * name: ExchangeIntegral (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: constituent of the interaction energy (ISO 80000-5) between the spins of adjacent electrons in matter arising from the overlap of electron state functions
         * remarks: None.
         */
    }
    attribute def curieTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 12-35.1 Curie temperature
         * symbol(s): `T_C`
         * application domain: generic
         * name: CurieTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a ferromagnet
         * remarks: `T_(cr)` is used for critical thermodynamic temperature in general.
         */
    }
    attribute def 'néelTemperature' : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 12-35.2 Néel temperature
         * symbol(s): `T_N`
         * application domain: generic
         * name: NéelTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of an antiferromagnet
         * remarks: None.
         */
    }
    attribute def superconductionTransitionTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 12-35.3 superconduction transition temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: SuperconductionTransitionTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: critical thermodynamic temperature (ISO 80000-5) of a superconductor
         * remarks: None.
         */
    }
    attribute def thermodynamicCriticalMagneticFluxDensity : MagneticFluxDensityValue {
        doc
        /*
         * source: item 12-36.1 thermodynamic critical magnetic flux density
         * symbol(s): `B_c`
         * application domain: generic
         * name: ThermodynamicCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: quantity given by: `B_c = sqrt((2μ_0 (G_n - G_s))/V)`, where `G_n` and `G_s` are the Gibbs energies (ISO 80000-5) at zero magnetic flux density (IEC 80000-6) in a normal conductor and superconductor, respectively, `μ_0` is the magnetic constant (IEC 80000-6), and `V` is volume (ISO 80000-3)
         * remarks: In type I superconductors, `B_c` is the critical magnetic flux density for disappearance of superconductivity. The symbol `B_(c3)` is used for the critical magnetic flux density for disappearance of surface superconductivity.
         */
    }
    attribute def lowerCriticalMagneticFluxDensity : MagneticFluxDensityValue {
        doc
        /*
         * source: item 12-36.2 lower critical magnetic flux density
         * symbol(s): `B_(c1)`
         * application domain: generic
         * name: LowerCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for magnetic flux (IEC 80000-6) entering the superconductor
         * remarks: None.
         */
    }
    attribute def upperCriticalMagneticFluxDensity : MagneticFluxDensityValue {
        doc
        /*
         * source: item 12-36.3 upper critical magnetic flux density
         * symbol(s): `B_(c2)`
         * application domain: generic
         * name: UpperCriticalMagneticFluxDensity (specializes MagneticFluxDensity)
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T, kg*s^-2*A^-1
         * tensor order: 0
         * definition: for type II superconductors, the threshold magnetic flux density (IEC 80000-6) for disappearance of bulk superconductivity
         * remarks: None.
         */
    }
    attribute def superconductorEnergyGap : EnergyValue {
        doc
        /*
         * source: item 12-37 superconductor energy gap
         * symbol(s): `Δ`
         * application domain: generic
         * name: SuperconductorEnergyGap (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, eV, kg*m^2*s^-2
         * tensor order: 0
         * definition: width of the forbidden energy band (item 12-24.2) in a superconductor
         * remarks: None.
         */
    }
    attribute def londonPenetrationDepth : LengthValue {
        doc
        /*
         * source: item 12-38.1 London penetration depth
         * symbol(s): `λ_L`
         * application domain: generic
         * name: LondonPenetrationDepth (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) a magnetic field penetrates the plane surface of a semi-finite superconductor according to the expression: `B(x) = B(0) exp(-x/λ_L)`, where `B` is magnetic flux density (IEC 80000-6) and `x` is distance (ISO 80000-3) from the surface
         * remarks: None.
         */
    }
    attribute def coherenceLength : LengthValue {
        doc
        /*
         * source: item 12-38.2 coherence length
         * symbol(s): `ξ`
         * application domain: generic
         * name: CoherenceLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (ISO 80000-3) in a superconductor over which the effect of a perturbation is appreciable at zero thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
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
    (reference r4 (scope relative) (span (offset 1017) (line 21) (column 20) (len 53)) (segments (segment 0 (token "ISQElectromagnetism") (name "ISQElectromagnetism") (separator none) (span (offset 1017) (line 21) (column 20) (len 19))) (segment 1 (token "ElectricPotentialDifferenceValue") (name "ElectricPotentialDifferenceValue") (separator colon-colon) (span (offset 1038) (line 21) (column 41) (len 32)))))
    (reference r5 (scope relative) (span (offset 1091) (line 22) (column 20) (len 45)) (segments (segment 0 (token "ISQElectromagnetism") (name "ISQElectromagnetism") (separator none) (span (offset 1091) (line 22) (column 20) (len 19))) (segment 1 (token "MagneticFluxDensityValue") (name "MagneticFluxDensityValue") (separator colon-colon) (span (offset 1112) (line 22) (column 41) (len 24)))))
    (reference r6 (scope relative) (span (offset 1157) (line 23) (column 20) (len 37)) (segments (segment 0 (token "ISQElectromagnetism") (name "ISQElectromagnetism") (separator none) (span (offset 1157) (line 23) (column 20) (len 19))) (segment 1 (token "ResistivityValue") (name "ResistivityValue") (separator colon-colon) (span (offset 1178) (line 23) (column 41) (len 16)))))
    (reference r7 (scope relative) (span (offset 1215) (line 24) (column 20) (len 47)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1215) (line 24) (column 20) (len 12))) (segment 1 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator colon-colon) (span (offset 1229) (line 24) (column 34) (len 33)))))
    (reference r8 (scope relative) (span (offset 1283) (line 25) (column 20) (len 35)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1283) (line 25) (column 20) (len 12))) (segment 1 (token "AngularFrequencyValue") (name "AngularFrequencyValue") (separator colon-colon) (span (offset 1297) (line 25) (column 34) (len 21)))))
    (reference r9 (scope relative) (span (offset 1339) (line 26) (column 20) (len 33)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1339) (line 26) (column 20) (len 12))) (segment 1 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator colon-colon) (span (offset 1353) (line 26) (column 34) (len 19)))))
    (reference r10 (scope relative) (span (offset 1393) (line 27) (column 20) (len 28)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1393) (line 27) (column 20) (len 12))) (segment 1 (token "RepetencyValue") (name "RepetencyValue") (separator colon-colon) (span (offset 1407) (line 27) (column 34) (len 14)))))
    (reference r11 (scope relative) (span (offset 1442) (line 28) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 1442) (line 28) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1461) (line 28) (column 39) (len 11)))))
    (reference r12 (scope relative) (span (offset 1571) (line 31) (column 47) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 1571) (line 31) (column 47) (len 23)))))
    (reference r13 (scope relative) (span (offset 2125) (line 44) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 2125) (line 44) (column 23) (len 7)))))
    (reference r14 (scope relative) (span (offset 2170) (line 45) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 2170) (line 45) (column 29) (len 33)))))
    (reference r15 (scope relative) (span (offset 2164) (line 45) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 2164) (line 45) (column 23) (len 4)))))
    (reference r16 (scope relative) (span (offset 2255) (line 48) (column 41) (len 24)) (segments (segment 0 (token "CartesianLattice3dVector") (name "CartesianLattice3dVector") (separator none) (span (offset 2255) (line 48) (column 41) (len 24)))))
    (reference r17 (scope relative) (span (offset 2421) (line 51) (column 58) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 2421) (line 51) (column 58) (len 23)))))
    (reference r18 (scope relative) (span (offset 3086) (line 64) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 3086) (line 64) (column 23) (len 7)))))
    (reference r19 (scope relative) (span (offset 3131) (line 65) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 3131) (line 65) (column 29) (len 33)))))
    (reference r20 (scope relative) (span (offset 3125) (line 65) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 3125) (line 65) (column 23) (len 4)))))
    (reference r21 (scope relative) (span (offset 3227) (line 68) (column 52) (len 35)) (segments (segment 0 (token "CartesianFundamentalLattice3dVector") (name "CartesianFundamentalLattice3dVector") (separator none) (span (offset 3227) (line 68) (column 52) (len 35)))))
    (reference r22 (scope relative) (span (offset 3420) (line 71) (column 67) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 3420) (line 71) (column 67) (len 19)))))
    (reference r23 (scope relative) (span (offset 4016) (line 84) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 4016) (line 84) (column 28) (len 4)))))
    (reference r24 (scope relative) (span (offset 4011) (line 84) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 4011) (line 84) (column 23) (len 3)))))
    (reference r25 (scope relative) (span (offset 4050) (line 85) (column 29) (len 43)) (segments (segment 0 (token "AngularReciprocalLatticeVectorMagnitudeUnit") (name "AngularReciprocalLatticeVectorMagnitudeUnit") (separator none) (span (offset 4050) (line 85) (column 29) (len 43)))))
    (reference r26 (scope relative) (span (offset 4044) (line 85) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 4044) (line 85) (column 23) (len 4)))))
    (reference r27 (scope relative) (span (offset 4160) (line 88) (column 56) (len 44)) (segments (segment 0 (token "AngularReciprocalLatticeVectorMagnitudeValue") (name "AngularReciprocalLatticeVectorMagnitudeValue") (separator none) (span (offset 4160) (line 88) (column 56) (len 44)))))
    (reference r28 (scope relative) (span (offset 4305) (line 90) (column 66) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 4305) (line 90) (column 66) (len 11)))))
    (reference r29 (scope relative) (span (offset 4355) (line 91) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 4355) (line 91) (column 37) (len 19)))))
    (reference r30 (scope relative) (span (offset 4384) (line 91) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 4384) (line 91) (column 66) (len 8)))))
    (reference r31 (scope relative) (span (offset 4395) (line 91) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 4395) (line 91) (column 77) (len 3)))))
    (reference r32 (scope relative) (span (offset 4399) (line 91) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 4399) (line 91) (column 81) (len 1)))))
    (reference r33 (scope relative) (span (offset 4406) (line 91) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 4406) (line 91) (column 88) (len 8)))))
    (reference r34 (scope relative) (span (offset 4445) (line 92) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 4445) (line 92) (column 23) (len 17)))))
    (reference r35 (scope relative) (span (offset 4469) (line 92) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 4469) (line 92) (column 47) (len 20)))))
    (reference r36 (scope relative) (span (offset 4492) (line 92) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 4492) (line 92) (column 70) (len 8)))))
    (reference r37 (scope relative) (span (offset 4574) (line 95) (column 64) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 4574) (line 95) (column 64) (len 23)))))
    (reference r38 (scope relative) (span (offset 5153) (line 108) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 5153) (line 108) (column 23) (len 7)))))
    (reference r39 (scope relative) (span (offset 5198) (line 109) (column 29) (len 50)) (segments (segment 0 (token "CartesianAngularReciprocalLattice3dCoordinateFrame") (name "CartesianAngularReciprocalLattice3dCoordinateFrame") (separator none) (span (offset 5198) (line 109) (column 29) (len 50)))))
    (reference r40 (scope relative) (span (offset 5192) (line 109) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5192) (line 109) (column 23) (len 4)))))
    (reference r41 (scope relative) (span (offset 5317) (line 112) (column 58) (len 41)) (segments (segment 0 (token "CartesianAngularReciprocalLattice3dVector") (name "CartesianAngularReciprocalLattice3dVector") (separator none) (span (offset 5317) (line 112) (column 58) (len 41)))))
    (reference r42 (scope relative) (span (offset 5453) (line 114) (column 73) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 5453) (line 114) (column 73) (len 19)))))
    (reference r43 (scope relative) (span (offset 5497) (line 115) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 5497) (line 115) (column 23) (len 7)))))
    (reference r44 (scope relative) (span (offset 5536) (line 116) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 5536) (line 116) (column 23) (len 12)))))
    (reference r45 (scope relative) (span (offset 5586) (line 117) (column 30) (len 43)) (segments (segment 0 (token "AngularReciprocalLatticeVectorMagnitudeUnit") (name "AngularReciprocalLatticeVectorMagnitudeUnit") (separator none) (span (offset 5586) (line 117) (column 30) (len 43)))))
    (reference r46 (scope relative) (span (offset 5579) (line 117) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 5579) (line 117) (column 23) (len 5)))))
    (reference r47 (scope relative) (span (offset 5784) (line 121) (column 71) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 5784) (line 121) (column 71) (len 19)))))
    (reference r48 (scope relative) (span (offset 6406) (line 134) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 6406) (line 134) (column 28) (len 4)))))
    (reference r49 (scope relative) (span (offset 6401) (line 134) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 6401) (line 134) (column 23) (len 3)))))
    (reference r50 (scope relative) (span (offset 6440) (line 135) (column 29) (len 47)) (segments (segment 0 (token "FundamentalReciprocalLatticeVectorMagnitudeUnit") (name "FundamentalReciprocalLatticeVectorMagnitudeUnit") (separator none) (span (offset 6440) (line 135) (column 29) (len 47)))))
    (reference r51 (scope relative) (span (offset 6434) (line 135) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 6434) (line 135) (column 23) (len 4)))))
    (reference r52 (scope relative) (span (offset 6558) (line 138) (column 60) (len 48)) (segments (segment 0 (token "FundamentalReciprocalLatticeVectorMagnitudeValue") (name "FundamentalReciprocalLatticeVectorMagnitudeValue") (separator none) (span (offset 6558) (line 138) (column 60) (len 48)))))
    (reference r53 (scope relative) (span (offset 6711) (line 140) (column 70) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 6711) (line 140) (column 70) (len 11)))))
    (reference r54 (scope relative) (span (offset 6761) (line 141) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6761) (line 141) (column 37) (len 19)))))
    (reference r55 (scope relative) (span (offset 6790) (line 141) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6790) (line 141) (column 66) (len 8)))))
    (reference r56 (scope relative) (span (offset 6801) (line 141) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6801) (line 141) (column 77) (len 3)))))
    (reference r57 (scope relative) (span (offset 6805) (line 141) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 6805) (line 141) (column 81) (len 1)))))
    (reference r58 (scope relative) (span (offset 6812) (line 141) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6812) (line 141) (column 88) (len 8)))))
    (reference r59 (scope relative) (span (offset 6851) (line 142) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 6851) (line 142) (column 23) (len 17)))))
    (reference r60 (scope relative) (span (offset 6875) (line 142) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 6875) (line 142) (column 47) (len 20)))))
    (reference r61 (scope relative) (span (offset 6898) (line 142) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 6898) (line 142) (column 70) (len 8)))))
    (reference r62 (scope relative) (span (offset 6984) (line 145) (column 68) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 6984) (line 145) (column 68) (len 23)))))
    (reference r63 (scope relative) (span (offset 7599) (line 158) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 7599) (line 158) (column 23) (len 7)))))
    (reference r64 (scope relative) (span (offset 7644) (line 159) (column 29) (len 54)) (segments (segment 0 (token "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (name "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (separator none) (span (offset 7644) (line 159) (column 29) (len 54)))))
    (reference r65 (scope relative) (span (offset 7638) (line 159) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7638) (line 159) (column 23) (len 4)))))
    (reference r66 (scope relative) (span (offset 7771) (line 162) (column 62) (len 45)) (segments (segment 0 (token "CartesianFundamentalReciprocalLattice3dVector") (name "CartesianFundamentalReciprocalLattice3dVector") (separator none) (span (offset 7771) (line 162) (column 62) (len 45)))))
    (reference r67 (scope relative) (span (offset 7915) (line 164) (column 77) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 7915) (line 164) (column 77) (len 19)))))
    (reference r68 (scope relative) (span (offset 7959) (line 165) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 7959) (line 165) (column 23) (len 7)))))
    (reference r69 (scope relative) (span (offset 7998) (line 166) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 7998) (line 166) (column 23) (len 12)))))
    (reference r70 (scope relative) (span (offset 8048) (line 167) (column 30) (len 47)) (segments (segment 0 (token "FundamentalReciprocalLatticeVectorMagnitudeUnit") (name "FundamentalReciprocalLatticeVectorMagnitudeUnit") (separator none) (span (offset 8048) (line 167) (column 30) (len 47)))))
    (reference r71 (scope relative) (span (offset 8041) (line 167) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 8041) (line 167) (column 23) (len 5)))))
    (reference r72 (scope relative) (span (offset 8197) (line 171) (column 36) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 8197) (line 171) (column 36) (len 11)))))
    (reference r73 (scope relative) (span (offset 8813) (line 187) (column 27) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 8813) (line 187) (column 27) (len 19)))))
    (reference r74 (scope relative) (span (offset 9580) (line 203) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 9580) (line 203) (column 52) (len 17)))))
    (reference r75 (scope relative) (span (offset 10297) (line 217) (column 41) (len 29)) (segments (segment 0 (token "ShortRangeOrderParameterValue") (name "ShortRangeOrderParameterValue") (separator none) (span (offset 10297) (line 217) (column 41) (len 29)))))
    (reference r76 (scope relative) (span (offset 10461) (line 220) (column 51) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 10461) (line 220) (column 51) (len 17)))))
    (reference r77 (scope relative) (span (offset 11151) (line 234) (column 40) (len 28)) (segments (segment 0 (token "LongRangeOrderParameterValue") (name "LongRangeOrderParameterValue") (separator none) (span (offset 11151) (line 234) (column 40) (len 28)))))
    (reference r78 (scope relative) (span (offset 11311) (line 237) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 11311) (line 237) (column 50) (len 17)))))
    (reference r79 (scope relative) (span (offset 12048) (line 251) (column 39) (len 27)) (segments (segment 0 (token "AtomicScatteringFactorValue") (name "AtomicScatteringFactorValue") (separator none) (span (offset 12048) (line 251) (column 39) (len 27)))))
    (reference r80 (scope relative) (span (offset 12192) (line 254) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 12192) (line 254) (column 43) (len 17)))))
    (reference r81 (scope relative) (span (offset 12971) (line 268) (column 32) (len 20)) (segments (segment 0 (token "StructureFactorValue") (name "StructureFactorValue") (separator none) (span (offset 12971) (line 268) (column 32) (len 20)))))
    (reference r82 (scope relative) (span (offset 13108) (line 271) (column 47) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 13108) (line 271) (column 47) (len 23)))))
    (reference r83 (scope relative) (span (offset 13573) (line 284) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 13573) (line 284) (column 23) (len 7)))))
    (reference r84 (scope relative) (span (offset 13618) (line 285) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 13618) (line 285) (column 29) (len 33)))))
    (reference r85 (scope relative) (span (offset 13612) (line 285) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13612) (line 285) (column 23) (len 4)))))
    (reference r86 (scope relative) (span (offset 13703) (line 288) (column 41) (len 24)) (segments (segment 0 (token "CartesianBurgers3dVector") (name "CartesianBurgers3dVector") (separator none) (span (offset 13703) (line 288) (column 41) (len 24)))))
    (reference r87 (scope relative) (span (offset 13865) (line 291) (column 56) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 13865) (line 291) (column 56) (len 23)))))
    (reference r88 (scope relative) (span (offset 14424) (line 304) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 14424) (line 304) (column 23) (len 7)))))
    (reference r89 (scope relative) (span (offset 14468) (line 305) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 14468) (line 305) (column 29) (len 33)))))
    (reference r90 (scope relative) (span (offset 14462) (line 305) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 14462) (line 305) (column 23) (len 4)))))
    (reference r91 (scope relative) (span (offset 14562) (line 308) (column 50) (len 33)) (segments (segment 0 (token "CartesianParticlePosition3dVector") (name "CartesianParticlePosition3dVector") (separator none) (span (offset 14562) (line 308) (column 50) (len 33)))))
    (reference r92 (scope relative) (span (offset 14739) (line 311) (column 59) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 14739) (line 311) (column 59) (len 23)))))
    (reference r93 (scope relative) (span (offset 15250) (line 324) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 15250) (line 324) (column 23) (len 7)))))
    (reference r94 (scope relative) (span (offset 15294) (line 325) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 15294) (line 325) (column 29) (len 33)))))
    (reference r95 (scope relative) (span (offset 15288) (line 325) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15288) (line 325) (column 23) (len 4)))))
    (reference r96 (scope relative) (span (offset 15391) (line 328) (column 53) (len 36)) (segments (segment 0 (token "CartesianEquilibriumPosition3dVector") (name "CartesianEquilibriumPosition3dVector") (separator none) (span (offset 15391) (line 328) (column 53) (len 36)))))
    (reference r97 (scope relative) (span (offset 15556) (line 331) (column 52) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 15556) (line 331) (column 52) (len 23)))))
    (reference r98 (scope relative) (span (offset 16305) (line 344) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 16305) (line 344) (column 23) (len 7)))))
    (reference r99 (scope relative) (span (offset 16350) (line 345) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 16350) (line 345) (column 29) (len 33)))))
    (reference r100 (scope relative) (span (offset 16344) (line 345) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16344) (line 345) (column 23) (len 4)))))
    (reference r101 (scope relative) (span (offset 16440) (line 348) (column 46) (len 29)) (segments (segment 0 (token "CartesianDisplacement3dVector") (name "CartesianDisplacement3dVector") (separator none) (span (offset 16440) (line 348) (column 46) (len 29)))))
    (reference r102 (scope relative) (span (offset 16589) (line 351) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 16589) (line 351) (column 45) (len 17)))))
    (reference r103 (scope relative) (span (offset 17235) (line 365) (column 34) (len 22)) (segments (segment 0 (token "DebyeWallerFactorValue") (name "DebyeWallerFactorValue") (separator none) (span (offset 17235) (line 365) (column 34) (len 22)))))
    (reference r104 (scope relative) (span (offset 17386) (line 368) (column 34) (len 14)) (segments (segment 0 (token "RepetencyValue") (name "RepetencyValue") (separator none) (span (offset 17386) (line 368) (column 34) (len 14)))))
    (reference r105 (scope relative) (span (offset 18679) (line 383) (column 32) (len 17)) (segments (segment 0 (token "angularWavenumber") (name "angularWavenumber") (separator none) (span (offset 18679) (line 383) (column 32) (len 17)))))
    (reference r106 (scope relative) (span (offset 18822) (line 386) (column 39) (len 14)) (segments (segment 0 (token "RepetencyValue") (name "RepetencyValue") (separator none) (span (offset 18822) (line 386) (column 39) (len 14)))))
    (reference r107 (scope relative) (span (offset 19444) (line 401) (column 37) (len 22)) (segments (segment 0 (token "fermiAngularWavenumber") (name "fermiAngularWavenumber") (separator none) (span (offset 19444) (line 401) (column 37) (len 22)))))
    (reference r108 (scope relative) (span (offset 19592) (line 404) (column 39) (len 14)) (segments (segment 0 (token "RepetencyValue") (name "RepetencyValue") (separator none) (span (offset 19592) (line 404) (column 39) (len 14)))))
    (reference r109 (scope relative) (span (offset 20278) (line 419) (column 37) (len 22)) (segments (segment 0 (token "debyeAngularWavenumber") (name "debyeAngularWavenumber") (separator none) (span (offset 20278) (line 419) (column 37) (len 22)))))
    (reference r110 (scope relative) (span (offset 20398) (line 422) (column 38) (len 21)) (segments (segment 0 (token "AngularFrequencyValue") (name "AngularFrequencyValue") (separator none) (span (offset 20398) (line 422) (column 38) (len 21)))))
    (reference r111 (scope relative) (span (offset 21042) (line 438) (column 33) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 21042) (line 438) (column 33) (len 29)))))
    (reference r112 (scope relative) (span (offset 21948) (line 454) (column 54) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21948) (line 454) (column 54) (len 19)))))
    (reference r113 (scope relative) (span (offset 22793) (line 467) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 22793) (line 467) (column 28) (len 4)))))
    (reference r114 (scope relative) (span (offset 22788) (line 467) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 22788) (line 467) (column 23) (len 3)))))
    (reference r115 (scope relative) (span (offset 22827) (line 468) (column 29) (len 30)) (segments (segment 0 (token "DensityOfVibrationalStatesUnit") (name "DensityOfVibrationalStatesUnit") (separator none) (span (offset 22827) (line 468) (column 29) (len 30)))))
    (reference r116 (scope relative) (span (offset 22821) (line 468) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22821) (line 468) (column 23) (len 4)))))
    (reference r117 (scope relative) (span (offset 22911) (line 471) (column 43) (len 31)) (segments (segment 0 (token "DensityOfVibrationalStatesValue") (name "DensityOfVibrationalStatesValue") (separator none) (span (offset 22911) (line 471) (column 43) (len 31)))))
    (reference r118 (scope relative) (span (offset 23030) (line 473) (column 53) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 23030) (line 473) (column 53) (len 11)))))
    (reference r119 (scope relative) (span (offset 23080) (line 474) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 23080) (line 474) (column 37) (len 19)))))
    (reference r120 (scope relative) (span (offset 23109) (line 474) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23109) (line 474) (column 66) (len 8)))))
    (reference r121 (scope relative) (span (offset 23120) (line 474) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 23120) (line 474) (column 77) (len 3)))))
    (reference r122 (scope relative) (span (offset 23124) (line 474) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 23124) (line 474) (column 81) (len 1)))))
    (reference r123 (scope relative) (span (offset 23131) (line 474) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 23131) (line 474) (column 88) (len 8)))))
    (reference r124 (scope relative) (span (offset 23186) (line 475) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 23186) (line 475) (column 39) (len 19)))))
    (reference r125 (scope relative) (span (offset 23215) (line 475) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23215) (line 475) (column 68) (len 8)))))
    (reference r126 (scope relative) (span (offset 23226) (line 475) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 23226) (line 475) (column 79) (len 3)))))
    (reference r127 (scope relative) (span (offset 23230) (line 475) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 23230) (line 475) (column 83) (len 1)))))
    (reference r128 (scope relative) (span (offset 23237) (line 475) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 23237) (line 475) (column 90) (len 8)))))
    (reference r129 (scope relative) (span (offset 23275) (line 476) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 23275) (line 476) (column 23) (len 17)))))
    (reference r130 (scope relative) (span (offset 23299) (line 476) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 23299) (line 476) (column 47) (len 20)))))
    (reference r131 (scope relative) (span (offset 23323) (line 476) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 23323) (line 476) (column 71) (len 8)))))
    (reference r132 (scope relative) (span (offset 23333) (line 476) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 23333) (line 476) (column 81) (len 10)))))
    (reference r133 (scope relative) (span (offset 23485) (line 480) (column 62) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 23485) (line 480) (column 62) (len 17)))))
    (reference r134 (scope relative) (span (offset 24231) (line 494) (column 51) (len 39)) (segments (segment 0 (token "'ThermodynamicGrüneisenParameterValue'") (name "ThermodynamicGrüneisenParameterValue") (separator none) (span (offset 24231) (line 494) (column 51) (len 39)))))
    (reference r135 (scope relative) (span (offset 24396) (line 497) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 24396) (line 497) (column 49) (len 17)))))
    (reference r136 (scope relative) (span (offset 25136) (line 511) (column 38) (len 26)) (segments (segment 0 (token "'GrüneisenParameterValue'") (name "GrüneisenParameterValue") (separator none) (span (offset 25136) (line 511) (column 38) (len 26)))))
    (reference r137 (scope relative) (span (offset 25284) (line 514) (column 38) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 25284) (line 514) (column 38) (len 11)))))
    (reference r138 (scope relative) (span (offset 25880) (line 530) (column 40) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 25880) (line 530) (column 40) (len 11)))))
    (reference r139 (scope relative) (span (offset 26486) (line 546) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 26486) (line 546) (column 49) (len 19)))))
    (reference r140 (scope relative) (span (offset 27347) (line 559) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 27347) (line 559) (column 28) (len 4)))))
    (reference r141 (scope relative) (span (offset 27342) (line 559) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 27342) (line 559) (column 23) (len 3)))))
    (reference r142 (scope relative) (span (offset 27381) (line 560) (column 29) (len 25)) (segments (segment 0 (token "EnergyDensityOfStatesUnit") (name "EnergyDensityOfStatesUnit") (separator none) (span (offset 27381) (line 560) (column 29) (len 25)))))
    (reference r143 (scope relative) (span (offset 27375) (line 560) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 27375) (line 560) (column 23) (len 4)))))
    (reference r144 (scope relative) (span (offset 27455) (line 563) (column 38) (len 26)) (segments (segment 0 (token "EnergyDensityOfStatesValue") (name "EnergyDensityOfStatesValue") (separator none) (span (offset 27455) (line 563) (column 38) (len 26)))))
    (reference r145 (scope relative) (span (offset 27564) (line 565) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 27564) (line 565) (column 48) (len 11)))))
    (reference r146 (scope relative) (span (offset 27614) (line 566) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27614) (line 566) (column 37) (len 19)))))
    (reference r147 (scope relative) (span (offset 27643) (line 566) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27643) (line 566) (column 66) (len 8)))))
    (reference r148 (scope relative) (span (offset 27654) (line 566) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27654) (line 566) (column 77) (len 3)))))
    (reference r149 (scope relative) (span (offset 27658) (line 566) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 27658) (line 566) (column 81) (len 1)))))
    (reference r150 (scope relative) (span (offset 27665) (line 566) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27665) (line 566) (column 88) (len 8)))))
    (reference r151 (scope relative) (span (offset 27716) (line 567) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27716) (line 567) (column 35) (len 19)))))
    (reference r152 (scope relative) (span (offset 27745) (line 567) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27745) (line 567) (column 64) (len 8)))))
    (reference r153 (scope relative) (span (offset 27756) (line 567) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27756) (line 567) (column 75) (len 3)))))
    (reference r154 (scope relative) (span (offset 27760) (line 567) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 27760) (line 567) (column 79) (len 1)))))
    (reference r155 (scope relative) (span (offset 27767) (line 567) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27767) (line 567) (column 86) (len 8)))))
    (reference r156 (scope relative) (span (offset 27822) (line 568) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27822) (line 568) (column 39) (len 19)))))
    (reference r157 (scope relative) (span (offset 27851) (line 568) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27851) (line 568) (column 68) (len 8)))))
    (reference r158 (scope relative) (span (offset 27862) (line 568) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27862) (line 568) (column 79) (len 3)))))
    (reference r159 (scope relative) (span (offset 27866) (line 568) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 27866) (line 568) (column 83) (len 1)))))
    (reference r160 (scope relative) (span (offset 27873) (line 568) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27873) (line 568) (column 90) (len 8)))))
    (reference r161 (scope relative) (span (offset 27911) (line 569) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 27911) (line 569) (column 23) (len 17)))))
    (reference r162 (scope relative) (span (offset 27935) (line 569) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 27935) (line 569) (column 47) (len 20)))))
    (reference r163 (scope relative) (span (offset 27959) (line 569) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 27959) (line 569) (column 71) (len 8)))))
    (reference r164 (scope relative) (span (offset 27969) (line 569) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 27969) (line 569) (column 81) (len 6)))))
    (reference r165 (scope relative) (span (offset 27977) (line 569) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 27977) (line 569) (column 89) (len 10)))))
    (reference r166 (scope relative) (span (offset 28089) (line 573) (column 36) (len 16)) (segments (segment 0 (token "ResistivityValue") (name "ResistivityValue") (separator none) (span (offset 28089) (line 573) (column 36) (len 16)))))
    (reference r167 (scope relative) (span (offset 28732) (line 589) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 28732) (line 589) (column 45) (len 19)))))
    (reference r168 (scope relative) (span (offset 29508) (line 602) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 29508) (line 602) (column 28) (len 4)))))
    (reference r169 (scope relative) (span (offset 29503) (line 602) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 29503) (line 602) (column 23) (len 3)))))
    (reference r170 (scope relative) (span (offset 29542) (line 603) (column 29) (len 21)) (segments (segment 0 (token "LorenzCoefficientUnit") (name "LorenzCoefficientUnit") (separator none) (span (offset 29542) (line 603) (column 29) (len 21)))))
    (reference r171 (scope relative) (span (offset 29536) (line 603) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 29536) (line 603) (column 23) (len 4)))))
    (reference r172 (scope relative) (span (offset 29608) (line 606) (column 34) (len 22)) (segments (segment 0 (token "LorenzCoefficientValue") (name "LorenzCoefficientValue") (separator none) (span (offset 29608) (line 606) (column 34) (len 22)))))
    (reference r173 (scope relative) (span (offset 29709) (line 608) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 29709) (line 608) (column 44) (len 11)))))
    (reference r174 (scope relative) (span (offset 29759) (line 609) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29759) (line 609) (column 37) (len 19)))))
    (reference r175 (scope relative) (span (offset 29788) (line 609) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29788) (line 609) (column 66) (len 8)))))
    (reference r176 (scope relative) (span (offset 29799) (line 609) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29799) (line 609) (column 77) (len 3)))))
    (reference r177 (scope relative) (span (offset 29803) (line 609) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 29803) (line 609) (column 81) (len 1)))))
    (reference r178 (scope relative) (span (offset 29810) (line 609) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29810) (line 609) (column 88) (len 8)))))
    (reference r179 (scope relative) (span (offset 29860) (line 610) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29860) (line 610) (column 35) (len 19)))))
    (reference r180 (scope relative) (span (offset 29889) (line 610) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29889) (line 610) (column 64) (len 8)))))
    (reference r181 (scope relative) (span (offset 29900) (line 610) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29900) (line 610) (column 75) (len 3)))))
    (reference r182 (scope relative) (span (offset 29904) (line 610) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 29904) (line 610) (column 79) (len 1)))))
    (reference r183 (scope relative) (span (offset 29911) (line 610) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29911) (line 610) (column 86) (len 8)))))
    (reference r184 (scope relative) (span (offset 29965) (line 611) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29965) (line 611) (column 39) (len 19)))))
    (reference r185 (scope relative) (span (offset 29994) (line 611) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29994) (line 611) (column 68) (len 8)))))
    (reference r186 (scope relative) (span (offset 30005) (line 611) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30005) (line 611) (column 79) (len 3)))))
    (reference r187 (scope relative) (span (offset 30009) (line 611) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 30009) (line 611) (column 83) (len 1)))))
    (reference r188 (scope relative) (span (offset 30016) (line 611) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30016) (line 611) (column 90) (len 8)))))
    (reference r189 (scope relative) (span (offset 30078) (line 612) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30078) (line 612) (column 46) (len 19)))))
    (reference r190 (scope relative) (span (offset 30107) (line 612) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30107) (line 612) (column 75) (len 8)))))
    (reference r191 (scope relative) (span (offset 30118) (line 612) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30118) (line 612) (column 86) (len 3)))))
    (reference r192 (scope relative) (span (offset 30122) (line 612) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 30122) (line 612) (column 90) (len 1)))))
    (reference r193 (scope relative) (span (offset 30129) (line 612) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30129) (line 612) (column 97) (len 8)))))
    (reference r194 (scope relative) (span (offset 30200) (line 613) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30200) (line 613) (column 55) (len 19)))))
    (reference r195 (scope relative) (span (offset 30229) (line 613) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30229) (line 613) (column 84) (len 8)))))
    (reference r196 (scope relative) (span (offset 30240) (line 613) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30240) (line 613) (column 95) (len 3)))))
    (reference r197 (scope relative) (span (offset 30244) (line 613) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 30244) (line 613) (column 99) (len 4)))))
    (reference r198 (scope relative) (span (offset 30254) (line 613) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30254) (line 613) (column 109) (len 8)))))
    (reference r199 (scope relative) (span (offset 30293) (line 614) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 30293) (line 614) (column 23) (len 17)))))
    (reference r200 (scope relative) (span (offset 30317) (line 614) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 30317) (line 614) (column 47) (len 20)))))
    (reference r201 (scope relative) (span (offset 30341) (line 614) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 30341) (line 614) (column 71) (len 8)))))
    (reference r202 (scope relative) (span (offset 30351) (line 614) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 30351) (line 614) (column 81) (len 6)))))
    (reference r203 (scope relative) (span (offset 30359) (line 614) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 30359) (line 614) (column 89) (len 10)))))
    (reference r204 (scope relative) (span (offset 30371) (line 614) (column 101) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 30371) (line 614) (column 101) (len 17)))))
    (reference r205 (scope relative) (span (offset 30390) (line 614) (column 120) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 30390) (line 614) (column 120) (len 26)))))
    (reference r206 (scope relative) (span (offset 30521) (line 618) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 30521) (line 618) (column 43) (len 19)))))
    (reference r207 (scope relative) (span (offset 31234) (line 631) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 31234) (line 631) (column 28) (len 4)))))
    (reference r208 (scope relative) (span (offset 31229) (line 631) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 31229) (line 631) (column 23) (len 3)))))
    (reference r209 (scope relative) (span (offset 31268) (line 632) (column 29) (len 19)) (segments (segment 0 (token "HallCoefficientUnit") (name "HallCoefficientUnit") (separator none) (span (offset 31268) (line 632) (column 29) (len 19)))))
    (reference r210 (scope relative) (span (offset 31262) (line 632) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 31262) (line 632) (column 23) (len 4)))))
    (reference r211 (scope relative) (span (offset 31330) (line 635) (column 32) (len 20)) (segments (segment 0 (token "HallCoefficientValue") (name "HallCoefficientValue") (separator none) (span (offset 31330) (line 635) (column 32) (len 20)))))
    (reference r212 (scope relative) (span (offset 31427) (line 637) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 31427) (line 637) (column 42) (len 11)))))
    (reference r213 (scope relative) (span (offset 31477) (line 638) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31477) (line 638) (column 37) (len 19)))))
    (reference r214 (scope relative) (span (offset 31506) (line 638) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31506) (line 638) (column 66) (len 8)))))
    (reference r215 (scope relative) (span (offset 31517) (line 638) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31517) (line 638) (column 77) (len 3)))))
    (reference r216 (scope relative) (span (offset 31521) (line 638) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 31521) (line 638) (column 81) (len 1)))))
    (reference r217 (scope relative) (span (offset 31528) (line 638) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31528) (line 638) (column 88) (len 8)))))
    (reference r218 (scope relative) (span (offset 31582) (line 639) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31582) (line 639) (column 39) (len 19)))))
    (reference r219 (scope relative) (span (offset 31611) (line 639) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31611) (line 639) (column 68) (len 8)))))
    (reference r220 (scope relative) (span (offset 31622) (line 639) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31622) (line 639) (column 79) (len 3)))))
    (reference r221 (scope relative) (span (offset 31626) (line 639) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 31626) (line 639) (column 83) (len 1)))))
    (reference r222 (scope relative) (span (offset 31633) (line 639) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31633) (line 639) (column 90) (len 8)))))
    (reference r223 (scope relative) (span (offset 31695) (line 640) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31695) (line 640) (column 46) (len 19)))))
    (reference r224 (scope relative) (span (offset 31724) (line 640) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31724) (line 640) (column 75) (len 8)))))
    (reference r225 (scope relative) (span (offset 31735) (line 640) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31735) (line 640) (column 86) (len 3)))))
    (reference r226 (scope relative) (span (offset 31739) (line 640) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 31739) (line 640) (column 90) (len 1)))))
    (reference r227 (scope relative) (span (offset 31746) (line 640) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31746) (line 640) (column 97) (len 8)))))
    (reference r228 (scope relative) (span (offset 31785) (line 641) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 31785) (line 641) (column 23) (len 17)))))
    (reference r229 (scope relative) (span (offset 31809) (line 641) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 31809) (line 641) (column 47) (len 20)))))
    (reference r230 (scope relative) (span (offset 31833) (line 641) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 31833) (line 641) (column 71) (len 8)))))
    (reference r231 (scope relative) (span (offset 31843) (line 641) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 31843) (line 641) (column 81) (len 10)))))
    (reference r232 (scope relative) (span (offset 31855) (line 641) (column 93) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 31855) (line 641) (column 93) (len 17)))))
    (reference r233 (scope relative) (span (offset 32029) (line 645) (column 60) (len 32)) (segments (segment 0 (token "ElectricPotentialDifferenceValue") (name "ElectricPotentialDifferenceValue") (separator none) (span (offset 32029) (line 645) (column 60) (len 32)))))
    (reference r234 (scope relative) (span (offset 32789) (line 661) (column 64) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 32789) (line 661) (column 64) (len 19)))))
    (reference r235 (scope relative) (span (offset 33568) (line 674) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 33568) (line 674) (column 28) (len 4)))))
    (reference r236 (scope relative) (span (offset 33563) (line 674) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 33563) (line 674) (column 23) (len 3)))))
    (reference r237 (scope relative) (span (offset 33602) (line 675) (column 29) (len 40)) (segments (segment 0 (token "SeebeckCoefficientForSubstancesAAndBUnit") (name "SeebeckCoefficientForSubstancesAAndBUnit") (separator none) (span (offset 33602) (line 675) (column 29) (len 40)))))
    (reference r238 (scope relative) (span (offset 33596) (line 675) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 33596) (line 675) (column 23) (len 4)))))
    (reference r239 (scope relative) (span (offset 33706) (line 678) (column 53) (len 41)) (segments (segment 0 (token "SeebeckCoefficientForSubstancesAAndBValue") (name "SeebeckCoefficientForSubstancesAAndBValue") (separator none) (span (offset 33706) (line 678) (column 53) (len 41)))))
    (reference r240 (scope relative) (span (offset 33845) (line 680) (column 63) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 33845) (line 680) (column 63) (len 11)))))
    (reference r241 (scope relative) (span (offset 33895) (line 681) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33895) (line 681) (column 37) (len 19)))))
    (reference r242 (scope relative) (span (offset 33924) (line 681) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33924) (line 681) (column 66) (len 8)))))
    (reference r243 (scope relative) (span (offset 33935) (line 681) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33935) (line 681) (column 77) (len 3)))))
    (reference r244 (scope relative) (span (offset 33939) (line 681) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 33939) (line 681) (column 81) (len 1)))))
    (reference r245 (scope relative) (span (offset 33946) (line 681) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33946) (line 681) (column 88) (len 8)))))
    (reference r246 (scope relative) (span (offset 33996) (line 682) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33996) (line 682) (column 35) (len 19)))))
    (reference r247 (scope relative) (span (offset 34025) (line 682) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 34025) (line 682) (column 64) (len 8)))))
    (reference r248 (scope relative) (span (offset 34036) (line 682) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 34036) (line 682) (column 75) (len 3)))))
    (reference r249 (scope relative) (span (offset 34040) (line 682) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 34040) (line 682) (column 79) (len 1)))))
    (reference r250 (scope relative) (span (offset 34047) (line 682) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 34047) (line 682) (column 86) (len 8)))))
    (reference r251 (scope relative) (span (offset 34101) (line 683) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 34101) (line 683) (column 39) (len 19)))))
    (reference r252 (scope relative) (span (offset 34130) (line 683) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 34130) (line 683) (column 68) (len 8)))))
    (reference r253 (scope relative) (span (offset 34141) (line 683) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 34141) (line 683) (column 79) (len 3)))))
    (reference r254 (scope relative) (span (offset 34145) (line 683) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 34145) (line 683) (column 83) (len 1)))))
    (reference r255 (scope relative) (span (offset 34152) (line 683) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 34152) (line 683) (column 90) (len 8)))))
    (reference r256 (scope relative) (span (offset 34214) (line 684) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 34214) (line 684) (column 46) (len 19)))))
    (reference r257 (scope relative) (span (offset 34243) (line 684) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 34243) (line 684) (column 75) (len 8)))))
    (reference r258 (scope relative) (span (offset 34254) (line 684) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 34254) (line 684) (column 86) (len 3)))))
    (reference r259 (scope relative) (span (offset 34258) (line 684) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 34258) (line 684) (column 90) (len 1)))))
    (reference r260 (scope relative) (span (offset 34265) (line 684) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 34265) (line 684) (column 97) (len 8)))))
    (reference r261 (scope relative) (span (offset 34336) (line 685) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 34336) (line 685) (column 55) (len 19)))))
    (reference r262 (scope relative) (span (offset 34365) (line 685) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 34365) (line 685) (column 84) (len 8)))))
    (reference r263 (scope relative) (span (offset 34376) (line 685) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 34376) (line 685) (column 95) (len 3)))))
    (reference r264 (scope relative) (span (offset 34380) (line 685) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 34380) (line 685) (column 99) (len 4)))))
    (reference r265 (scope relative) (span (offset 34390) (line 685) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 34390) (line 685) (column 109) (len 8)))))
    (reference r266 (scope relative) (span (offset 34429) (line 686) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 34429) (line 686) (column 23) (len 17)))))
    (reference r267 (scope relative) (span (offset 34453) (line 686) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 34453) (line 686) (column 47) (len 20)))))
    (reference r268 (scope relative) (span (offset 34477) (line 686) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 34477) (line 686) (column 71) (len 8)))))
    (reference r269 (scope relative) (span (offset 34487) (line 686) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 34487) (line 686) (column 81) (len 6)))))
    (reference r270 (scope relative) (span (offset 34495) (line 686) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 34495) (line 686) (column 89) (len 10)))))
    (reference r271 (scope relative) (span (offset 34507) (line 686) (column 101) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 34507) (line 686) (column 101) (len 17)))))
    (reference r272 (scope relative) (span (offset 34526) (line 686) (column 120) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 34526) (line 686) (column 120) (len 26)))))
    (reference r273 (scope relative) (span (offset 34695) (line 690) (column 53) (len 32)) (segments (segment 0 (token "ElectricPotentialDifferenceValue") (name "ElectricPotentialDifferenceValue") (separator none) (span (offset 34695) (line 690) (column 53) (len 32)))))
    (reference r274 (scope relative) (span (offset 35576) (line 706) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 35576) (line 706) (column 46) (len 19)))))
    (reference r275 (scope relative) (span (offset 36243) (line 719) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 36243) (line 719) (column 28) (len 4)))))
    (reference r276 (scope relative) (span (offset 36238) (line 719) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 36238) (line 719) (column 23) (len 3)))))
    (reference r277 (scope relative) (span (offset 36277) (line 720) (column 29) (len 22)) (segments (segment 0 (token "ThomsonCoefficientUnit") (name "ThomsonCoefficientUnit") (separator none) (span (offset 36277) (line 720) (column 29) (len 22)))))
    (reference r278 (scope relative) (span (offset 36271) (line 720) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 36271) (line 720) (column 23) (len 4)))))
    (reference r279 (scope relative) (span (offset 36345) (line 723) (column 35) (len 23)) (segments (segment 0 (token "ThomsonCoefficientValue") (name "ThomsonCoefficientValue") (separator none) (span (offset 36345) (line 723) (column 35) (len 23)))))
    (reference r280 (scope relative) (span (offset 36448) (line 725) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 36448) (line 725) (column 45) (len 11)))))
    (reference r281 (scope relative) (span (offset 36498) (line 726) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36498) (line 726) (column 37) (len 19)))))
    (reference r282 (scope relative) (span (offset 36527) (line 726) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36527) (line 726) (column 66) (len 8)))))
    (reference r283 (scope relative) (span (offset 36538) (line 726) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36538) (line 726) (column 77) (len 3)))))
    (reference r284 (scope relative) (span (offset 36542) (line 726) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 36542) (line 726) (column 81) (len 1)))))
    (reference r285 (scope relative) (span (offset 36549) (line 726) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36549) (line 726) (column 88) (len 8)))))
    (reference r286 (scope relative) (span (offset 36599) (line 727) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36599) (line 727) (column 35) (len 19)))))
    (reference r287 (scope relative) (span (offset 36628) (line 727) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36628) (line 727) (column 64) (len 8)))))
    (reference r288 (scope relative) (span (offset 36639) (line 727) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36639) (line 727) (column 75) (len 3)))))
    (reference r289 (scope relative) (span (offset 36643) (line 727) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 36643) (line 727) (column 79) (len 1)))))
    (reference r290 (scope relative) (span (offset 36650) (line 727) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36650) (line 727) (column 86) (len 8)))))
    (reference r291 (scope relative) (span (offset 36704) (line 728) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36704) (line 728) (column 39) (len 19)))))
    (reference r292 (scope relative) (span (offset 36733) (line 728) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36733) (line 728) (column 68) (len 8)))))
    (reference r293 (scope relative) (span (offset 36744) (line 728) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36744) (line 728) (column 79) (len 3)))))
    (reference r294 (scope relative) (span (offset 36748) (line 728) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 36748) (line 728) (column 83) (len 1)))))
    (reference r295 (scope relative) (span (offset 36755) (line 728) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36755) (line 728) (column 90) (len 8)))))
    (reference r296 (scope relative) (span (offset 36817) (line 729) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36817) (line 729) (column 46) (len 19)))))
    (reference r297 (scope relative) (span (offset 36846) (line 729) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36846) (line 729) (column 75) (len 8)))))
    (reference r298 (scope relative) (span (offset 36857) (line 729) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36857) (line 729) (column 86) (len 3)))))
    (reference r299 (scope relative) (span (offset 36861) (line 729) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 36861) (line 729) (column 90) (len 1)))))
    (reference r300 (scope relative) (span (offset 36868) (line 729) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36868) (line 729) (column 97) (len 8)))))
    (reference r301 (scope relative) (span (offset 36939) (line 730) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36939) (line 730) (column 55) (len 19)))))
    (reference r302 (scope relative) (span (offset 36968) (line 730) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36968) (line 730) (column 84) (len 8)))))
    (reference r303 (scope relative) (span (offset 36979) (line 730) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36979) (line 730) (column 95) (len 3)))))
    (reference r304 (scope relative) (span (offset 36983) (line 730) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 36983) (line 730) (column 99) (len 4)))))
    (reference r305 (scope relative) (span (offset 36993) (line 730) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36993) (line 730) (column 109) (len 8)))))
    (reference r306 (scope relative) (span (offset 37032) (line 731) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 37032) (line 731) (column 23) (len 17)))))
    (reference r307 (scope relative) (span (offset 37056) (line 731) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 37056) (line 731) (column 47) (len 20)))))
    (reference r308 (scope relative) (span (offset 37080) (line 731) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 37080) (line 731) (column 71) (len 8)))))
    (reference r309 (scope relative) (span (offset 37090) (line 731) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 37090) (line 731) (column 81) (len 6)))))
    (reference r310 (scope relative) (span (offset 37098) (line 731) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 37098) (line 731) (column 89) (len 10)))))
    (reference r311 (scope relative) (span (offset 37110) (line 731) (column 101) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 37110) (line 731) (column 101) (len 17)))))
    (reference r312 (scope relative) (span (offset 37129) (line 731) (column 120) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 37129) (line 731) (column 120) (len 26)))))
    (reference r313 (scope relative) (span (offset 37245) (line 735) (column 29) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 37245) (line 735) (column 29) (len 11)))))
    (reference r314 (scope relative) (span (offset 38298) (line 751) (column 33) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 38298) (line 751) (column 33) (len 11)))))
    (reference r315 (scope relative) (span (offset 38969) (line 767) (column 33) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 38969) (line 767) (column 33) (len 11)))))
    (reference r316 (scope relative) (span (offset 39665) (line 783) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 39665) (line 783) (column 46) (len 19)))))
    (reference r317 (scope relative) (span (offset 40343) (line 796) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 40343) (line 796) (column 28) (len 4)))))
    (reference r318 (scope relative) (span (offset 40338) (line 796) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 40338) (line 796) (column 23) (len 3)))))
    (reference r319 (scope relative) (span (offset 40377) (line 797) (column 29) (len 22)) (segments (segment 0 (token "RichardsonConstantUnit") (name "RichardsonConstantUnit") (separator none) (span (offset 40377) (line 797) (column 29) (len 22)))))
    (reference r320 (scope relative) (span (offset 40371) (line 797) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 40371) (line 797) (column 23) (len 4)))))
    (reference r321 (scope relative) (span (offset 40445) (line 800) (column 35) (len 23)) (segments (segment 0 (token "RichardsonConstantValue") (name "RichardsonConstantValue") (separator none) (span (offset 40445) (line 800) (column 35) (len 23)))))
    (reference r322 (scope relative) (span (offset 40548) (line 802) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 40548) (line 802) (column 45) (len 11)))))
    (reference r323 (scope relative) (span (offset 40598) (line 803) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40598) (line 803) (column 37) (len 19)))))
    (reference r324 (scope relative) (span (offset 40627) (line 803) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40627) (line 803) (column 66) (len 8)))))
    (reference r325 (scope relative) (span (offset 40638) (line 803) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40638) (line 803) (column 77) (len 3)))))
    (reference r326 (scope relative) (span (offset 40642) (line 803) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 40642) (line 803) (column 81) (len 1)))))
    (reference r327 (scope relative) (span (offset 40649) (line 803) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40649) (line 803) (column 88) (len 8)))))
    (reference r328 (scope relative) (span (offset 40711) (line 804) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40711) (line 804) (column 46) (len 19)))))
    (reference r329 (scope relative) (span (offset 40740) (line 804) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40740) (line 804) (column 75) (len 8)))))
    (reference r330 (scope relative) (span (offset 40751) (line 804) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40751) (line 804) (column 86) (len 3)))))
    (reference r331 (scope relative) (span (offset 40755) (line 804) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 40755) (line 804) (column 90) (len 1)))))
    (reference r332 (scope relative) (span (offset 40762) (line 804) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40762) (line 804) (column 97) (len 8)))))
    (reference r333 (scope relative) (span (offset 40832) (line 805) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40832) (line 805) (column 55) (len 19)))))
    (reference r334 (scope relative) (span (offset 40861) (line 805) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40861) (line 805) (column 84) (len 8)))))
    (reference r335 (scope relative) (span (offset 40872) (line 805) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40872) (line 805) (column 95) (len 3)))))
    (reference r336 (scope relative) (span (offset 40876) (line 805) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 40876) (line 805) (column 99) (len 4)))))
    (reference r337 (scope relative) (span (offset 40886) (line 805) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40886) (line 805) (column 109) (len 8)))))
    (reference r338 (scope relative) (span (offset 40925) (line 806) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 40925) (line 806) (column 23) (len 17)))))
    (reference r339 (scope relative) (span (offset 40949) (line 806) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 40949) (line 806) (column 47) (len 20)))))
    (reference r340 (scope relative) (span (offset 40973) (line 806) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 40973) (line 806) (column 71) (len 8)))))
    (reference r341 (scope relative) (span (offset 40983) (line 806) (column 81) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 40983) (line 806) (column 81) (len 17)))))
    (reference r342 (scope relative) (span (offset 41002) (line 806) (column 100) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 41002) (line 806) (column 100) (len 26)))))
    (reference r343 (scope relative) (span (offset 41116) (line 810) (column 28) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 41116) (line 810) (column 28) (len 11)))))
    (reference r344 (scope relative) (span (offset 42061) (line 826) (column 26) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 42061) (line 826) (column 26) (len 11)))))
    (reference r345 (scope relative) (span (offset 42715) (line 842) (column 33) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 42715) (line 842) (column 33) (len 29)))))
    (reference r346 (scope relative) (span (offset 43534) (line 858) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 43534) (line 858) (column 43) (len 19)))))
    (reference r347 (scope relative) (span (offset 44247) (line 871) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 44247) (line 871) (column 28) (len 4)))))
    (reference r348 (scope relative) (span (offset 44242) (line 871) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 44242) (line 871) (column 23) (len 3)))))
    (reference r349 (scope relative) (span (offset 44281) (line 872) (column 29) (len 19)) (segments (segment 0 (token "ElectronDensityUnit") (name "ElectronDensityUnit") (separator none) (span (offset 44281) (line 872) (column 29) (len 19)))))
    (reference r350 (scope relative) (span (offset 44275) (line 872) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 44275) (line 872) (column 23) (len 4)))))
    (reference r351 (scope relative) (span (offset 44343) (line 875) (column 32) (len 20)) (segments (segment 0 (token "ElectronDensityValue") (name "ElectronDensityValue") (separator none) (span (offset 44343) (line 875) (column 32) (len 20)))))
    (reference r352 (scope relative) (span (offset 44440) (line 877) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 44440) (line 877) (column 42) (len 11)))))
    (reference r353 (scope relative) (span (offset 44490) (line 878) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 44490) (line 878) (column 37) (len 19)))))
    (reference r354 (scope relative) (span (offset 44519) (line 878) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 44519) (line 878) (column 66) (len 8)))))
    (reference r355 (scope relative) (span (offset 44530) (line 878) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 44530) (line 878) (column 77) (len 3)))))
    (reference r356 (scope relative) (span (offset 44534) (line 878) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 44534) (line 878) (column 81) (len 1)))))
    (reference r357 (scope relative) (span (offset 44541) (line 878) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 44541) (line 878) (column 88) (len 8)))))
    (reference r358 (scope relative) (span (offset 44580) (line 879) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 44580) (line 879) (column 23) (len 17)))))
    (reference r359 (scope relative) (span (offset 44604) (line 879) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 44604) (line 879) (column 47) (len 20)))))
    (reference r360 (scope relative) (span (offset 44627) (line 879) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 44627) (line 879) (column 70) (len 8)))))
    (reference r361 (scope relative) (span (offset 44733) (line 883) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 44733) (line 883) (column 39) (len 19)))))
    (reference r362 (scope relative) (span (offset 45431) (line 896) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 45431) (line 896) (column 28) (len 4)))))
    (reference r363 (scope relative) (span (offset 45426) (line 896) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 45426) (line 896) (column 23) (len 3)))))
    (reference r364 (scope relative) (span (offset 45465) (line 897) (column 29) (len 15)) (segments (segment 0 (token "HoleDensityUnit") (name "HoleDensityUnit") (separator none) (span (offset 45465) (line 897) (column 29) (len 15)))))
    (reference r365 (scope relative) (span (offset 45459) (line 897) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 45459) (line 897) (column 23) (len 4)))))
    (reference r366 (scope relative) (span (offset 45519) (line 900) (column 28) (len 16)) (segments (segment 0 (token "HoleDensityValue") (name "HoleDensityValue") (separator none) (span (offset 45519) (line 900) (column 28) (len 16)))))
    (reference r367 (scope relative) (span (offset 45608) (line 902) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 45608) (line 902) (column 38) (len 11)))))
    (reference r368 (scope relative) (span (offset 45658) (line 903) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45658) (line 903) (column 37) (len 19)))))
    (reference r369 (scope relative) (span (offset 45687) (line 903) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45687) (line 903) (column 66) (len 8)))))
    (reference r370 (scope relative) (span (offset 45698) (line 903) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45698) (line 903) (column 77) (len 3)))))
    (reference r371 (scope relative) (span (offset 45702) (line 903) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 45702) (line 903) (column 81) (len 1)))))
    (reference r372 (scope relative) (span (offset 45709) (line 903) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45709) (line 903) (column 88) (len 8)))))
    (reference r373 (scope relative) (span (offset 45748) (line 904) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 45748) (line 904) (column 23) (len 17)))))
    (reference r374 (scope relative) (span (offset 45772) (line 904) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 45772) (line 904) (column 47) (len 20)))))
    (reference r375 (scope relative) (span (offset 45795) (line 904) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 45795) (line 904) (column 70) (len 8)))))
    (reference r376 (scope relative) (span (offset 45926) (line 908) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 45926) (line 908) (column 51) (len 19)))))
    (reference r377 (scope relative) (span (offset 46682) (line 921) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 46682) (line 921) (column 28) (len 4)))))
    (reference r378 (scope relative) (span (offset 46677) (line 921) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 46677) (line 921) (column 23) (len 3)))))
    (reference r379 (scope relative) (span (offset 46716) (line 922) (column 29) (len 27)) (segments (segment 0 (token "IntrinsicCarrierDensityUnit") (name "IntrinsicCarrierDensityUnit") (separator none) (span (offset 46716) (line 922) (column 29) (len 27)))))
    (reference r380 (scope relative) (span (offset 46710) (line 922) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 46710) (line 922) (column 23) (len 4)))))
    (reference r381 (scope relative) (span (offset 46794) (line 925) (column 40) (len 28)) (segments (segment 0 (token "IntrinsicCarrierDensityValue") (name "IntrinsicCarrierDensityValue") (separator none) (span (offset 46794) (line 925) (column 40) (len 28)))))
    (reference r382 (scope relative) (span (offset 46907) (line 927) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 46907) (line 927) (column 50) (len 11)))))
    (reference r383 (scope relative) (span (offset 46957) (line 928) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 46957) (line 928) (column 37) (len 19)))))
    (reference r384 (scope relative) (span (offset 46986) (line 928) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 46986) (line 928) (column 66) (len 8)))))
    (reference r385 (scope relative) (span (offset 46997) (line 928) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 46997) (line 928) (column 77) (len 3)))))
    (reference r386 (scope relative) (span (offset 47001) (line 928) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 47001) (line 928) (column 81) (len 1)))))
    (reference r387 (scope relative) (span (offset 47008) (line 928) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 47008) (line 928) (column 88) (len 8)))))
    (reference r388 (scope relative) (span (offset 47047) (line 929) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 47047) (line 929) (column 23) (len 17)))))
    (reference r389 (scope relative) (span (offset 47071) (line 929) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 47071) (line 929) (column 47) (len 20)))))
    (reference r390 (scope relative) (span (offset 47094) (line 929) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 47094) (line 929) (column 70) (len 8)))))
    (reference r391 (scope relative) (span (offset 47202) (line 933) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 47202) (line 933) (column 40) (len 19)))))
    (reference r392 (scope relative) (span (offset 47638) (line 946) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 47638) (line 946) (column 28) (len 4)))))
    (reference r393 (scope relative) (span (offset 47633) (line 946) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 47633) (line 946) (column 23) (len 3)))))
    (reference r394 (scope relative) (span (offset 47672) (line 947) (column 29) (len 16)) (segments (segment 0 (token "DonorDensityUnit") (name "DonorDensityUnit") (separator none) (span (offset 47672) (line 947) (column 29) (len 16)))))
    (reference r395 (scope relative) (span (offset 47666) (line 947) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 47666) (line 947) (column 23) (len 4)))))
    (reference r396 (scope relative) (span (offset 47728) (line 950) (column 29) (len 17)) (segments (segment 0 (token "DonorDensityValue") (name "DonorDensityValue") (separator none) (span (offset 47728) (line 950) (column 29) (len 17)))))
    (reference r397 (scope relative) (span (offset 47819) (line 952) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 47819) (line 952) (column 39) (len 11)))))
    (reference r398 (scope relative) (span (offset 47869) (line 953) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 47869) (line 953) (column 37) (len 19)))))
    (reference r399 (scope relative) (span (offset 47898) (line 953) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 47898) (line 953) (column 66) (len 8)))))
    (reference r400 (scope relative) (span (offset 47909) (line 953) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 47909) (line 953) (column 77) (len 3)))))
    (reference r401 (scope relative) (span (offset 47913) (line 953) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 47913) (line 953) (column 81) (len 1)))))
    (reference r402 (scope relative) (span (offset 47920) (line 953) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 47920) (line 953) (column 88) (len 8)))))
    (reference r403 (scope relative) (span (offset 47959) (line 954) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 47959) (line 954) (column 23) (len 17)))))
    (reference r404 (scope relative) (span (offset 47983) (line 954) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 47983) (line 954) (column 47) (len 20)))))
    (reference r405 (scope relative) (span (offset 48006) (line 954) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48006) (line 954) (column 70) (len 8)))))
    (reference r406 (scope relative) (span (offset 48120) (line 958) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 48120) (line 958) (column 43) (len 19)))))
    (reference r407 (scope relative) (span (offset 48565) (line 971) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 48565) (line 971) (column 28) (len 4)))))
    (reference r408 (scope relative) (span (offset 48560) (line 971) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 48560) (line 971) (column 23) (len 3)))))
    (reference r409 (scope relative) (span (offset 48599) (line 972) (column 29) (len 19)) (segments (segment 0 (token "AcceptorDensityUnit") (name "AcceptorDensityUnit") (separator none) (span (offset 48599) (line 972) (column 29) (len 19)))))
    (reference r410 (scope relative) (span (offset 48593) (line 972) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 48593) (line 972) (column 23) (len 4)))))
    (reference r411 (scope relative) (span (offset 48661) (line 975) (column 32) (len 20)) (segments (segment 0 (token "AcceptorDensityValue") (name "AcceptorDensityValue") (separator none) (span (offset 48661) (line 975) (column 32) (len 20)))))
    (reference r412 (scope relative) (span (offset 48758) (line 977) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 48758) (line 977) (column 42) (len 11)))))
    (reference r413 (scope relative) (span (offset 48808) (line 978) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48808) (line 978) (column 37) (len 19)))))
    (reference r414 (scope relative) (span (offset 48837) (line 978) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48837) (line 978) (column 66) (len 8)))))
    (reference r415 (scope relative) (span (offset 48848) (line 978) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48848) (line 978) (column 77) (len 3)))))
    (reference r416 (scope relative) (span (offset 48852) (line 978) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 48852) (line 978) (column 81) (len 1)))))
    (reference r417 (scope relative) (span (offset 48859) (line 978) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48859) (line 978) (column 88) (len 8)))))
    (reference r418 (scope relative) (span (offset 48898) (line 979) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 48898) (line 979) (column 23) (len 17)))))
    (reference r419 (scope relative) (span (offset 48922) (line 979) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 48922) (line 979) (column 47) (len 20)))))
    (reference r420 (scope relative) (span (offset 48945) (line 979) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48945) (line 979) (column 70) (len 8)))))
    (reference r421 (scope relative) (span (offset 49042) (line 983) (column 30) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 49042) (line 983) (column 30) (len 9)))))
    (reference r422 (scope relative) (span (offset 49919) (line 999) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 49919) (line 999) (column 41) (len 17)))))
    (reference r423 (scope relative) (span (offset 50547) (line 1013) (column 30) (len 18)) (segments (segment 0 (token "MobilityRatioValue") (name "MobilityRatioValue") (separator none) (span (offset 50547) (line 1013) (column 30) (len 18)))))
    (reference r424 (scope relative) (span (offset 50670) (line 1016) (column 31) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 50670) (line 1016) (column 31) (len 13)))))
    (reference r425 (scope relative) (span (offset 51428) (line 1032) (column 32) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 51428) (line 1032) (column 32) (len 13)))))
    (reference r426 (scope relative) (span (offset 52174) (line 1048) (column 57) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 52174) (line 1048) (column 57) (len 11)))))
    (reference r427 (scope relative) (span (offset 52913) (line 1064) (column 33) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 52913) (line 1064) (column 33) (len 11)))))
    (reference r428 (scope relative) (span (offset 53574) (line 1080) (column 33) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 53574) (line 1080) (column 33) (len 29)))))
    (reference r429 (scope relative) (span (offset 54220) (line 1096) (column 35) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 54220) (line 1096) (column 35) (len 29)))))
    (reference r430 (scope relative) (span (offset 54848) (line 1112) (column 53) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 54848) (line 1112) (column 53) (len 29)))))
    (reference r431 (scope relative) (span (offset 55525) (line 1128) (column 57) (len 24)) (segments (segment 0 (token "MagneticFluxDensityValue") (name "MagneticFluxDensityValue") (separator none) (span (offset 55525) (line 1128) (column 57) (len 24)))))
    (reference r432 (scope relative) (span (offset 56654) (line 1144) (column 49) (len 24)) (segments (segment 0 (token "MagneticFluxDensityValue") (name "MagneticFluxDensityValue") (separator none) (span (offset 56654) (line 1144) (column 49) (len 24)))))
    (reference r433 (scope relative) (span (offset 57395) (line 1160) (column 49) (len 24)) (segments (segment 0 (token "MagneticFluxDensityValue") (name "MagneticFluxDensityValue") (separator none) (span (offset 57395) (line 1160) (column 49) (len 24)))))
    (reference r434 (scope relative) (span (offset 58098) (line 1176) (column 40) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 58098) (line 1176) (column 40) (len 11)))))
    (reference r435 (scope relative) (span (offset 58698) (line 1192) (column 39) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 58698) (line 1192) (column 39) (len 11)))))
    (reference r436 (scope relative) (span (offset 59447) (line 1208) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 59447) (line 1208) (column 32) (len 11)))))
  )
  (root (library-package (name "ISQCondensedMatter") (standard true) (body brace (doc) (import (target (span (span (offset 801) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 840) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 850) (line 16) (column 30) (len 3))) (separator (span (offset 850) (line 16) (column 30) (len 2))) (marker (span (offset 852) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 874) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 895) (line 17) (column 41) (len 3))) (separator (span (offset 895) (line 17) (column 41) (len 2))) (marker (span (offset 897) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 919) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 926) (line 18) (column 27) (len 3))) (separator (span (offset 926) (line 18) (column 27) (len 2))) (marker (span (offset 928) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1017) (line 21) (column 20) (len 53))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1091) (line 22) (column 20) (len 45))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1157) (line 23) (column 20) (len 37))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1215) (line 24) (column 20) (len 47))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1283) (line 25) (column 20) (len 35))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1339) (line 26) (column 20) (len 33))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1393) (line 27) (column 20) (len 28))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1442) (line 28) (column 20) (len 30))) (all none) (ref r11) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "CartesianLattice3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2135) (line 44) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianLattice3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianFundamentalLattice3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3096) (line 64) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianFundamentalLattice3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularReciprocalLatticeVectorMagnitudeValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularReciprocalLatticeVectorMagnitude") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularReciprocalLatticeVectorMagnitudeUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4395) (line 91) (column 77) (len 5)) (member-access (base (expression (span (offset 4395) (line 91) (column 77) (len 3)) (ref r31))) (separator dot) (member (ref r32))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4417) (line 91) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 4418) (line 91) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4492) (line 92) (column 70) (len 8)) (ref r36))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAngularReciprocalLattice3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5163) (line 108) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianAngularReciprocalLattice3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r41)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianAngularReciprocalLattice3dCoordinateFrame") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r42)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5507) (line 115) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r44)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5551) (line 116) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r46)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "FundamentalReciprocalLatticeVectorMagnitudeValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r47)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r48)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r50)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "fundamentalReciprocalLatticeVectorMagnitude") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r52)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "FundamentalReciprocalLatticeVectorMagnitudeUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r53)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r54)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r55)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6801) (line 141) (column 77) (len 5)) (member-access (base (expression (span (offset 6801) (line 141) (column 77) (len 3)) (ref r56))) (separator dot) (member (ref r57))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6823) (line 141) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 6824) (line 141) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6898) (line 142) (column 70) (len 8)) (ref r61))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianFundamentalReciprocalLattice3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r62)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r63)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7609) (line 158) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r64)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianFundamentalReciprocalLattice3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r66)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianFundamentalReciprocalLattice3dCoordinateFrame") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r67)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r68)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7969) (line 165) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8013) (line 166) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r70)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "latticePlaneSpacing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r72)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "braggAngle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "ShortRangeOrderParameterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r74)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "shortRangeOrderParameter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r75)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "LongRangeOrderParameterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r76)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "longRangeOrderParameter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r77)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "AtomicScatteringFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r78)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "atomicScatteringFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "StructureFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r80)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "structureFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r81)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianBurgers3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r82)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r83)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13583) (line 284) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r84)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianBurgers3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r86)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianParticlePosition3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r87)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r88)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14434) (line 304) (column 33) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r89)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r90)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianParticlePosition3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r91)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianEquilibriumPosition3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r92)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r93)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15260) (line 324) (column 33) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r94)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r95)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianEquilibriumPosition3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r96)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianDisplacement3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r97)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r98)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16315) (line 344) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r99)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianDisplacement3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r101)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "DebyeWallerFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r102)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "debyeWallerFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r103)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "angularWavenumber") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r104)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "angularRepetency") (target (ref r105)) (body semicolon)) (attribute-def (declaration-name "fermiAngularWavenumber") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r106)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "fermiAngularRepetency") (target (ref r107)) (body semicolon)) (attribute-def (declaration-name "debyeAngularWavenumber") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r108)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "debyeAngularRepetency") (target (ref r109)) (body semicolon)) (attribute-def (declaration-name "debyeAngularFrequency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r110)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "debyeTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r111)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "DensityOfVibrationalStatesValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r112)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r113)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r114)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r115)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r116)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "densityOfVibrationalStates") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r117)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "DensityOfVibrationalStatesUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r118)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r119)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r120)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23120) (line 474) (column 77) (len 5)) (member-access (base (expression (span (offset 23120) (line 474) (column 77) (len 3)) (ref r121))) (separator dot) (member (ref r122))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r123)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23142) (line 474) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 23143) (line 474) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r124)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r125)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23226) (line 475) (column 79) (len 5)) (member-access (base (expression (span (offset 23226) (line 475) (column 79) (len 3)) (ref r126))) (separator dot) (member (ref r127))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r128)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23248) (line 475) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r129)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r130)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23322) (line 476) (column 70) (len 22)) (tuple (expression (span (offset 23323) (line 476) (column 71) (len 8)) (ref r131)) (expression (span (offset 23333) (line 476) (column 81) (len 10)) (ref r132))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermodynamicGrüneisenParameterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r133)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "thermodynamicGrüneisenParameter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r134)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "GrüneisenParameterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r135)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "grüneisenParameter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r136)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "meanFreePathOfPhonons") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r137)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "meanFreePathOfElectrons") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r138)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "EnergyDensityOfStatesValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r139)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r140)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r141)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r142)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r143)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "energyDensityOfStates") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r144)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "EnergyDensityOfStatesUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r145)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r146)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27654) (line 566) (column 77) (len 5)) (member-access (base (expression (span (offset 27654) (line 566) (column 77) (len 3)) (ref r148))) (separator dot) (member (ref r149))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r150)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27676) (line 566) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 27677) (line 566) (column 100) (len 1)) (integer 5)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r151)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r152)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27756) (line 567) (column 75) (len 5)) (member-access (base (expression (span (offset 27756) (line 567) (column 75) (len 3)) (ref r153))) (separator dot) (member (ref r154))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r155)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27778) (line 567) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 27779) (line 567) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r156)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r157)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27862) (line 568) (column 79) (len 5)) (member-access (base (expression (span (offset 27862) (line 568) (column 79) (len 3)) (ref r158))) (separator dot) (member (ref r159))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r160)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27884) (line 568) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r161)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r162)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27958) (line 569) (column 70) (len 30)) (tuple (expression (span (offset 27959) (line 569) (column 71) (len 8)) (ref r163)) (expression (span (offset 27969) (line 569) (column 81) (len 6)) (ref r164)) (expression (span (offset 27977) (line 569) (column 89) (len 10)) (ref r165))))))) (body semicolon)))))) (attribute-def (declaration-name "residualResistivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r166)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "LorenzCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r167)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r168)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r169)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r170)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r171)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "lorenzCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r172)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "LorenzCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r173)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r174)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r175)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29799) (line 609) (column 77) (len 5)) (member-access (base (expression (span (offset 29799) (line 609) (column 77) (len 3)) (ref r176))) (separator dot) (member (ref r177))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r178)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29821) (line 609) (column 99) (len 1)) (integer 4))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r179)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r180)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29900) (line 610) (column 75) (len 5)) (member-access (base (expression (span (offset 29900) (line 610) (column 75) (len 3)) (ref r181))) (separator dot) (member (ref r182))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r183)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29922) (line 610) (column 97) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r184)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r185)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30005) (line 611) (column 79) (len 5)) (member-access (base (expression (span (offset 30005) (line 611) (column 79) (len 3)) (ref r186))) (separator dot) (member (ref r187))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r188)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30027) (line 611) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 30028) (line 611) (column 102) (len 1)) (integer 6)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r189)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r190)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30118) (line 612) (column 86) (len 5)) (member-access (base (expression (span (offset 30118) (line 612) (column 86) (len 3)) (ref r191))) (separator dot) (member (ref r192))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r193)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30140) (line 612) (column 108) (len 2)) (unary (operator "-") (operand (expression (span (offset 30141) (line 612) (column 109) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r194)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r195)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30240) (line 613) (column 95) (len 8)) (member-access (base (expression (span (offset 30240) (line 613) (column 95) (len 3)) (ref r196))) (separator dot) (member (ref r197))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r198)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30265) (line 613) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 30266) (line 613) (column 121) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r199)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r200)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30340) (line 614) (column 70) (len 77)) (tuple (expression (span (offset 30341) (line 614) (column 71) (len 8)) (ref r201)) (expression (span (offset 30351) (line 614) (column 81) (len 6)) (ref r202)) (expression (span (offset 30359) (line 614) (column 89) (len 10)) (ref r203)) (expression (span (offset 30371) (line 614) (column 101) (len 17)) (ref r204)) (expression (span (offset 30390) (line 614) (column 120) (len 26)) (ref r205))))))) (body semicolon)))))) (attribute-def (declaration-name "HallCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r206)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r207)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r208)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r209)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r210)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "hallCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r211)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "HallCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r212)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r213)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r214)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31517) (line 638) (column 77) (len 5)) (member-access (base (expression (span (offset 31517) (line 638) (column 77) (len 3)) (ref r215))) (separator dot) (member (ref r216))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r217)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31539) (line 638) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r218)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r219)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31622) (line 639) (column 79) (len 5)) (member-access (base (expression (span (offset 31622) (line 639) (column 79) (len 3)) (ref r220))) (separator dot) (member (ref r221))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r222)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31644) (line 639) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 31645) (line 639) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r223)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r224)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31735) (line 640) (column 86) (len 5)) (member-access (base (expression (span (offset 31735) (line 640) (column 86) (len 3)) (ref r225))) (separator dot) (member (ref r226))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r227)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31757) (line 640) (column 108) (len 2)) (unary (operator "-") (operand (expression (span (offset 31758) (line 640) (column 109) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r228)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r229)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31832) (line 641) (column 70) (len 41)) (tuple (expression (span (offset 31833) (line 641) (column 71) (len 8)) (ref r230)) (expression (span (offset 31843) (line 641) (column 81) (len 10)) (ref r231)) (expression (span (offset 31855) (line 641) (column 93) (len 17)) (ref r232))))))) (body semicolon)))))) (attribute-def (declaration-name "thermoelectricVoltageBetweenSubstancesAAndB") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r233)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "SeebeckCoefficientForSubstancesAAndBValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r234)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r235)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r236)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r237)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r238)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "seebeckCoefficientForSubstancesAAndB") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r239)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SeebeckCoefficientForSubstancesAAndBUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r240)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r241)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r242)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33935) (line 681) (column 77) (len 5)) (member-access (base (expression (span (offset 33935) (line 681) (column 77) (len 3)) (ref r243))) (separator dot) (member (ref r244))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r245)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33957) (line 681) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r246)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r247)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34036) (line 682) (column 75) (len 5)) (member-access (base (expression (span (offset 34036) (line 682) (column 75) (len 3)) (ref r248))) (separator dot) (member (ref r249))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r250)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34058) (line 682) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r251)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r252)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34141) (line 683) (column 79) (len 5)) (member-access (base (expression (span (offset 34141) (line 683) (column 79) (len 3)) (ref r253))) (separator dot) (member (ref r254))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r255)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34163) (line 683) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 34164) (line 683) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r256)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r257)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34254) (line 684) (column 86) (len 5)) (member-access (base (expression (span (offset 34254) (line 684) (column 86) (len 3)) (ref r258))) (separator dot) (member (ref r259))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r260)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34276) (line 684) (column 108) (len 2)) (unary (operator "-") (operand (expression (span (offset 34277) (line 684) (column 109) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r261)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r262)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34376) (line 685) (column 95) (len 8)) (member-access (base (expression (span (offset 34376) (line 685) (column 95) (len 3)) (ref r263))) (separator dot) (member (ref r264))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r265)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34401) (line 685) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 34402) (line 685) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r266)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r267)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34476) (line 686) (column 70) (len 77)) (tuple (expression (span (offset 34477) (line 686) (column 71) (len 8)) (ref r268)) (expression (span (offset 34487) (line 686) (column 81) (len 6)) (ref r269)) (expression (span (offset 34495) (line 686) (column 89) (len 10)) (ref r270)) (expression (span (offset 34507) (line 686) (column 101) (len 17)) (ref r271)) (expression (span (offset 34526) (line 686) (column 120) (len 26)) (ref r272))))))) (body semicolon)))))) (attribute-def (declaration-name "peltierCoefficientForSubstancesAAndB") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r273)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "ThomsonCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r274)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r275)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r277)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r278)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thomsonCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r279)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThomsonCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r280)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r281)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r282)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36538) (line 726) (column 77) (len 5)) (member-access (base (expression (span (offset 36538) (line 726) (column 77) (len 3)) (ref r283))) (separator dot) (member (ref r284))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r285)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36560) (line 726) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r286)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r287)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36639) (line 727) (column 75) (len 5)) (member-access (base (expression (span (offset 36639) (line 727) (column 75) (len 3)) (ref r288))) (separator dot) (member (ref r289))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r290)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36661) (line 727) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r291)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r292)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36744) (line 728) (column 79) (len 5)) (member-access (base (expression (span (offset 36744) (line 728) (column 79) (len 3)) (ref r293))) (separator dot) (member (ref r294))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r295)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36766) (line 728) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 36767) (line 728) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r296)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r297)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36857) (line 729) (column 86) (len 5)) (member-access (base (expression (span (offset 36857) (line 729) (column 86) (len 3)) (ref r298))) (separator dot) (member (ref r299))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r300)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36879) (line 729) (column 108) (len 2)) (unary (operator "-") (operand (expression (span (offset 36880) (line 729) (column 109) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r301)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r302)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36979) (line 730) (column 95) (len 8)) (member-access (base (expression (span (offset 36979) (line 730) (column 95) (len 3)) (ref r303))) (separator dot) (member (ref r304))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r305)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37004) (line 730) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 37005) (line 730) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r306)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r307)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37079) (line 731) (column 70) (len 77)) (tuple (expression (span (offset 37080) (line 731) (column 71) (len 8)) (ref r308)) (expression (span (offset 37090) (line 731) (column 81) (len 6)) (ref r309)) (expression (span (offset 37098) (line 731) (column 89) (len 10)) (ref r310)) (expression (span (offset 37110) (line 731) (column 101) (len 17)) (ref r311)) (expression (span (offset 37129) (line 731) (column 120) (len 26)) (ref r312))))))) (body semicolon)))))) (attribute-def (declaration-name "workFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r313)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "ionizationEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r314)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "electronAffinity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r315)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "RichardsonConstantValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r316)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r317)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r318)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r319)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r320)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "richardsonConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r321)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "RichardsonConstantUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r322)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r323)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r324)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40638) (line 803) (column 77) (len 5)) (member-access (base (expression (span (offset 40638) (line 803) (column 77) (len 3)) (ref r325))) (separator dot) (member (ref r326))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r327)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40660) (line 803) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 40661) (line 803) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r328)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r329)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40751) (line 804) (column 86) (len 5)) (member-access (base (expression (span (offset 40751) (line 804) (column 86) (len 3)) (ref r330))) (separator dot) (member (ref r331))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r332)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40773) (line 804) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r333)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r334)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40872) (line 805) (column 95) (len 8)) (member-access (base (expression (span (offset 40872) (line 805) (column 95) (len 3)) (ref r335))) (separator dot) (member (ref r336))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r337)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40897) (line 805) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 40898) (line 805) (column 121) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r338)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r339)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40972) (line 806) (column 70) (len 57)) (tuple (expression (span (offset 40973) (line 806) (column 71) (len 8)) (ref r340)) (expression (span (offset 40983) (line 806) (column 81) (len 17)) (ref r341)) (expression (span (offset 41002) (line 806) (column 100) (len 26)) (ref r342))))))) (body semicolon)))))) (attribute-def (declaration-name "fermiEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r343)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "gapEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r344)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "fermiTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r345)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "ElectronDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r346)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r347)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r348)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r349)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r350)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "electronDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r351)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ElectronDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r352)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r353)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r354)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44530) (line 878) (column 77) (len 5)) (member-access (base (expression (span (offset 44530) (line 878) (column 77) (len 3)) (ref r355))) (separator dot) (member (ref r356))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r357)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44552) (line 878) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 44553) (line 878) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r358)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r359)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44627) (line 879) (column 70) (len 8)) (ref r360))))) (body semicolon)))))) (attribute-def (declaration-name "HoleDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r361)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r362)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r363)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r364)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r365)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "holeDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r366)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "HoleDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r367)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r368)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r369)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45698) (line 903) (column 77) (len 5)) (member-access (base (expression (span (offset 45698) (line 903) (column 77) (len 3)) (ref r370))) (separator dot) (member (ref r371))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r372)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45720) (line 903) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 45721) (line 903) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r373)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r374)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45795) (line 904) (column 70) (len 8)) (ref r375))))) (body semicolon)))))) (attribute-def (declaration-name "IntrinsicCarrierDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r376)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r377)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r378)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r379)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r380)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "intrinsicCarrierDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r381)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "IntrinsicCarrierDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r382)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r383)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r384)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46997) (line 928) (column 77) (len 5)) (member-access (base (expression (span (offset 46997) (line 928) (column 77) (len 3)) (ref r385))) (separator dot) (member (ref r386))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r387)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47019) (line 928) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 47020) (line 928) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r388)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r389)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47094) (line 929) (column 70) (len 8)) (ref r390))))) (body semicolon)))))) (attribute-def (declaration-name "DonorDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r391)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r392)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r393)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r394)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r395)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "donorDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r396)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "DonorDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r397)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r398)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r399)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47909) (line 953) (column 77) (len 5)) (member-access (base (expression (span (offset 47909) (line 953) (column 77) (len 3)) (ref r400))) (separator dot) (member (ref r401))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r402)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47931) (line 953) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 47932) (line 953) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r403)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r404)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48006) (line 954) (column 70) (len 8)) (ref r405))))) (body semicolon)))))) (attribute-def (declaration-name "AcceptorDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r406)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r407)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r409)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r410)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "acceptorDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r411)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "AcceptorDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r412)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r413)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r414)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48848) (line 978) (column 77) (len 5)) (member-access (base (expression (span (offset 48848) (line 978) (column 77) (len 3)) (ref r415))) (separator dot) (member (ref r416))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r417)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48870) (line 978) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 48871) (line 978) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r418)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r419)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48945) (line 979) (column 70) (len 8)) (ref r420))))) (body semicolon)))))) (attribute-def (declaration-name "effectiveMass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r421)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "MobilityRatioValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r422)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "mobilityRatio") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r423)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "relaxationTime") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r424)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "carrierLifetime") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r425)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "diffusionLengthForCondensedMatterPhysics") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r426)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "exchangeIntegral") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r427)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "curieTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r428)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "néelTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r429)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "superconductionTransitionTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r430)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "thermodynamicCriticalMagneticFluxDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r431)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "lowerCriticalMagneticFluxDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r432)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "upperCriticalMagneticFluxDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r433)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "superconductorEnergyGap") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r434)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "londonPenetrationDepth") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r435)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "coherenceLength") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r436)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))))))
)
~~~
