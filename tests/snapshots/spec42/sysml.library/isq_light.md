# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQLight"))
~~~
# SOURCE
~~~sysml
standard library package ISQLight {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-7:2019 "Light and radiation"
     * see also https://www.iso.org/standard/64977.html
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

    /* ISO-80000-7 item 7-1.1 speed of light in a medium */
    attribute def SpeedOfLightInAMediumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-1.1 speed of light in a medium
         * symbol(s): `c`
         * application domain: generic
         * name: SpeedOfLightInAMedium
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: phase speed of an electromagnetic wave at a given point in a medium
         * remarks: See also ISO 80000-3. The value of the speed of light in a medium can depend on the frequency, polarization, and direction. For the definition of the speed of electromagnetic waves in vacuum, `c_0`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedOfLightInAMediumUnit[1];
    }

    attribute speedOfLightInAMedium: SpeedOfLightInAMediumValue[*] nonunique :> scalarQuantities;

    attribute def SpeedOfLightInAMediumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-1.2 refractive index */
    attribute def RefractiveIndexValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-1.2 refractive index
         * symbol(s): `n`
         * application domain: generic
         * name: RefractiveIndex (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of light in vacuum (ISO 80000-1) and speed of light in a medium (item 7-1.1)
         * remarks: The value of the refractive index can depend on the frequency, polarization, and direction. The refractive index is expressed by n = c_0/c, where c_()_0 is the speed of light in vacuum and c is the speed of light in the medium. For a medium with absorption, the complex refractive index n is defined by n = n + ik where k is spectral absorption index (IEC 60050-845) and i is imaginary unit. The refractivity is expressed by n -1, where n is refractive index.
         */
    }
    attribute refractiveIndex: RefractiveIndexValue :> scalarQuantities;

    /* ISO-80000-7 item 7-2.1 radiant energy */
    attribute radiantEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-2.1 radiant energy
         * symbol(s): `Q_e`, `W`, `U`, `(Q)`
         * application domain: electromagnetism
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) emitted, transferred or received in form of electromagnetic waves
         * remarks: Radiant energy can be expressed by the time integral of radiant flux (item 7-4.1), `Φ_e`, over a given duration (ISO 80000-3), `Δt`: `Q_e = int_(Δ t) Φ_e dt`. Radiant energy is expressed either as a function of wavelength (ISO 80000-3), `λ`, as a function of frequency (ISO 80000-3), `ν`, or as a function of wavenumber, `σ`. (See also 0.1.) The corresponding photometric quantity is "luminous energy" (item 7-12). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
    }

    /* ISO-80000-7 item 7-2.2 spectral radiant energy */
    attribute def SpectralRadiantEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-2.2 spectral radiant energy
         * symbol(s): `Q_(e,λ)`, `W_λ`, `U_λ`, `(Q_λ)`
         * application domain: generic
         * name: SpectralRadiantEnergy
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): J/nm, kg*m*s^-2
         * tensor order: 0
         * definition: spectral density of radiant energy, expressed by `Q_(e,λ) = (dQ_e) / (dλ)`, where `Q_e` is radiant energy (item 7-2.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant energy is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Q_e = int_(λ_1)^(λ_2) Q_(e,λ) dλ`
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyUnit[1];
    }

    attribute spectralRadiantEnergy: SpectralRadiantEnergyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.1 radiant energy density */
    attribute def RadiantEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.1 radiant energy density
         * symbol(s): `w`, `(ρ_e)`
         * application domain: generic
         * name: RadiantEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: volumetric density of radiant energy, expressed by `w = (dQ_e)/(dV)`, where `Q_e` is radiant energy (item 7-2.1) in an elementary three-dimensional domain and `V` is the volume (ISO 80000-3) of that domain
         * remarks: Radiant energy density within a Planckian radiator is given by `w = (4 σ)/(c_0) T^4` where `σ` is the Stefan-Boltzmann constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1) and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantEnergyDensityUnit[1];
    }

    attribute radiantEnergyDensity: RadiantEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.2 spectral radiant energy density in terms of wavelength */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.2 spectral radiant energy density in terms of wavelength
         * symbol(s): `w_λ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavelength
         * quantity dimension: L^-2*M^1*T^-2
         * measurement unit(s): J/(m^3*nm), kg*m^-2*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavelength, expressed by `w_λ = (dw)/(dλ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavelength `λ` (ISO 80000-3)
         * remarks: Spectral radiant energy density within a Planckian radiator is given by `w_λ = 8πhc_0*f(λ, T)`, where `h` is the Planck constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1), `T` is thermodynamic temperature (ISO 80000-5) and `f(λ,T) = (λ^-5)/(exp(c_2 λ^-1 T^-1) - 1)`. For the radiation constant `c_2` in `f(λ,T)`, see ISO 80000-1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavelengthUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavelength: SpectralRadiantEnergyDensityInTermsOfWavelengthValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-3.3 spectral radiant energy density in terms of wavenumber */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.3 spectral radiant energy density in terms of wavenumber
         * symbol(s): `w_ṽ`, `ρ_ṽ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavenumber
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavenumber, expressed by `w_ṽ = (dw)/(dṽ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavenumber `ṽ` (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantEnergyDensityInTermsOfWavenumberUnit[1];
    }

    attribute spectralRadiantEnergyDensityInTermsOfWavenumber: SpectralRadiantEnergyDensityInTermsOfWavenumberValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-4.1 radiant flux, radiant power */
    attribute def RadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.1 radiant flux, radiant power
         * symbol(s): `Φ_e`, `P_e`, `Φ`, `P`
         * application domain: generic
         * name: RadiantFlux
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: change in radiant energy with time, expressed by `Φ_e = (dQ_e)/(dt)`, where `Q_e` is the radiant energy (item 7-2.1) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous flux" (item 7-13). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantFluxUnit[1];
    }

    attribute radiantFlux: RadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def RadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias RadiantPowerUnit for RadiantFluxUnit;
    alias RadiantPowerValue for RadiantFluxValue;
    alias radiantPower for radiantFlux;

    /* ISO-80000-7 item 7-4.2 spectral radiant flux, spectral radiant power */
    attribute def SpectralRadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.2 spectral radiant flux, spectral radiant power
         * symbol(s): `Φ_(e,λ)`, `P_(e,λ)`, `(Φ_λ)`, `(P_λ)`
         * application domain: generic
         * name: SpectralRadiantFlux
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/nm, kg*m*s^-3
         * tensor order: 0
         * definition: spectral density of radiant flux, expressed by `Φ_(e,λ) = (dQ_e)/(dλ)`, where `Φ_e` is radiant flux (item 7-4.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant flux is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Φ_e = int_(λ_1)^(λ_2) Φ_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantFluxUnit[1];
    }

    attribute spectralRadiantFlux: SpectralRadiantFluxValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias SpectralRadiantPowerUnit for SpectralRadiantFluxUnit;
    alias SpectralRadiantPowerValue for SpectralRadiantFluxValue;
    alias spectralRadiantPower for spectralRadiantFlux;

    /* ISO-80000-7 item 7-5.1 radiant intensity */
    attribute def RadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.1 radiant intensity
         * symbol(s): `I_e`, `(I)`
         * application domain: generic
         * name: RadiantIntensity
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W/sr, kg*m^2*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant flux with respect to solid angle in a specified direction, expressed by `I_e = (dΦ_e)/(dΩ)`, where `Φ_e` is the radiant flux (item 7-4.1) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the radiant intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,φ)`, is used to determine the radiant flux (item 7-4.1) within a certain solid angle (ISO 80000-3), `Ω`, of a source: `Φ_e = int int_Ω I_e(θ, φ) sin(θ) dφ dθ`. The corresponding photometric quantity is "luminous intensity" (item 7-14). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantIntensityUnit[1];
    }

    attribute radiantIntensity: RadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def RadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-5.2 spectral radiant intensity */
    attribute def SpectralRadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.2 spectral radiant intensity
         * symbol(s): `I_(e,λ)`, `(I_λ)`
         * application domain: generic
         * name: SpectralRadiantIntensity
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/(sr*nm), kg*m*s^-3*sr^-1
         * tensor order: 0
         * definition: spectral density of radiant intensity, expressed by `I_(e, λ) = (d I_e)/(dλ)`, where `I_e` is radiant intensity (item 7-5.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant intensity is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `I_e = int_(λ_1)^(λ_2) I_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantIntensityUnit[1];
    }

    attribute spectralRadiantIntensity: SpectralRadiantIntensityValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantIntensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.1 radiance */
    attribute def RadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.1 radiance
         * symbol(s): `L_e`, `(L)`
         * application domain: generic
         * name: Radiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/(sr*m^2), kg*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_e = (d I_e)/(dA) * 1/cos(α)`, where `I_e` is radiant intensity (item 7-5.1), `A` is area (ISO 80000-3), and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: See also 0.1. For Planckian radiation, `L_e = σ/π T^4` where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminance" (item 7-15). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadianceUnit[1];
    }

    attribute radiance: RadianceValue[*] nonunique :> scalarQuantities;

    attribute def RadianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-6.2 spectral radiance */
    attribute def SpectralRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.2 spectral radiance
         * symbol(s): `L_(e,λ)`, `(L_λ)`
         * application domain: generic
         * name: SpectralRadiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(sr*m^2*nm), kg*m^-1*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiance with respect to wavelength, expressed by `L_(e, λ) = (d L_e)/(d λ)` where `L_e` is radiance (item 7-6.1) in terms of wavelength λ(ISO 80000-3)
         * remarks: For Planckian radiation, `L_(e, λ)(λ) = (c(λ))/(4 π) ω_λ(λ) = h c_0^2 * f(λ,T)`, where `c(λ)` is phase speed (ISO 80000-3) of electromagnetic radiation of a wavelength (ISO 80000-3) `λ` in a given medium, `ω_λ(λ)` is spectral radiant energy density in terms of wavelength, `c_0` is speed of light in vacuum (ISO 80000-1), `h` is the Planck constant (ISO 80000-1), and `f(λ,T) = λ^-5/(exp(c_2 λ^-1 T^-1) - 1)`, where the radiation constant `c_2 = (hc)/k`. The integral of (total) radiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `L_e = int_(λ_1)^(λ_2) L_(e,λ) dλ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadianceUnit[1];
    }

    attribute spectralRadiance: SpectralRadianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.1 irradiance */
    attribute def IrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.1 irradiance
         * symbol(s): `E_e`, `(E)`
         * application domain: generic
         * name: Irradiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of incident radiant flux with respect to area at a point on a real or imaginary surface, expressed by `E_e = (d Φ_e)/(d A)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) on which the radiant flux is incident
         * remarks: The corresponding photometric quantity is "illuminance" (item 7-16). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical irradiance" is defined by the mean value of irradiance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(e,0) = int_(4 π) L_e d Ω` where `Ω` is solid angle (ISO 80000-3) and `L_e` is radiance (item 7-6.1). (See CIE DIS 017/E:2016, term 17-21-054.) It can be expressed by the quotient of the radiant flux (item 7-4.1) of all the radiation incident on the outer surface of an infinitely small sphere centred at the specified point and the area (ISO 80000-3) of the diametrical cross-section of that sphere. Spherical irradiance is also called "fluence rate" or "radiant fluence rate". The corresponding photometric quantity to spherical irradiance is called "spherical illuminance".
         */
        attribute :>> num: Real;
        attribute :>> mRef: IrradianceUnit[1];
    }

    attribute irradiance: IrradianceValue[*] nonunique :> scalarQuantities;

    attribute def IrradianceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-7.2 spectral irradiance */
    attribute def SpectralIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.2 spectral irradiance
         * symbol(s): `E_(e,λ)`, `(E_λ)`
         * application domain: generic
         * name: SpectralIrradiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of irradiance with respect to wavelength, expressed by `E_(e,λ) = (d E_e)/(dλ)`, where `E_e` is irradiance (item 7-7.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) irradiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `E_e = int_(λ_1)^(λ_2) E_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralIrradianceUnit[1];
    }

    attribute spectralIrradiance: SpectralIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-8.1 radiant exitance , radiant emittance */
    attribute def RadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.1 radiant exitance , radiant emittance
         * symbol(s): `M_e`, `(M)`
         * application domain: generic
         * name: RadiantExitance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of exiting radiant flux with respect to area at a point on a real or imaginary surface, expressed by `M_e = (d Φ_e)/(dA)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) from which the radiant flux leaves
         * remarks: For Planckian radiation, `M_e = σT^4`, where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminous exitance" (item 7-17). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExitanceUnit[1];
    }

    attribute radiantExitance: RadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExitanceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    alias RadiantEmittanceUnit for RadiantExitanceUnit;
    alias RadiantEmittanceValue for RadiantExitanceValue;
    alias radiantEmittance for radiantExitance;

    /* ISO-80000-7 item 7-8.2 spectral radiant exitance */
    attribute def SpectralRadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.2 spectral radiant exitance
         * symbol(s): `M_(e,λ)`, `(M_λ)`
         * application domain: generic
         * name: SpectralRadiantExitance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of radiant exitance with respect to wavelength, expressed by `M_(e,λ) = (d M_e)/(dλ)`, where `M_e` is radiant exitance (item 7-8.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exitance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `M_e = int_(λ_1)^(λ_2) M_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExitanceUnit[1];
    }

    attribute spectralRadiantExitance: SpectralRadiantExitanceValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.1 radiant exposure */
    attribute def RadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.1 radiant exposure
         * symbol(s): `H_e`, `(H)`
         * application domain: generic
         * name: RadiantExposure
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: density of incident radiant energy with respect to area at a point on a real or imaginary surface, expressed by `H_e = (d Q_e)/(dA)`, where `Q_e` is radiant energy (item 7-2.1) and `A` is the area on which the radiant energy is incident (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous exposure" (item 7-18). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RadiantExposureUnit[1];
    }

    attribute radiantExposure: RadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def RadiantExposureUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-9.2 spectral radiant exposure */
    attribute def SpectralRadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.2 spectral radiant exposure
         * symbol(s): `H_(e,λ)`, `(H_λ)`
         * application domain: generic
         * name: SpectralRadiantExposure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/(m^2*nm), kg*m^-1*s^-2
         * tensor order: 0
         * definition: density of radiant exposure with respect to wavelength, expressed by `H_(e,λ) = (d H_e)/(dλ)`, where `H_e` is radiant exposure (item 7-9.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exposure is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `H_e = int_(λ_1)^(λ_2) H_(e,λ) d λ` .
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralRadiantExposureUnit[1];
    }

    attribute spectralRadiantExposure: SpectralRadiantExposureValue[*] nonunique :> scalarQuantities;

    attribute def SpectralRadiantExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-7 item 7-10.1 luminous efficiency */
    attribute def LuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.1 luminous efficiency
         * symbol(s): `V`
         * application domain: specified photometric condition
         * name: LuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant flux (item 7-4.1) weighted by the spectral luminous efficiency (item 7-10.2) and the corresponding radiant flux for a specified photometric condition
         * remarks: Luminous efficiency for photopic vision is expressed by `V = (int_0^∞ Φ_(e,λ)(λ) V(λ) d λ)/(int_0^∞ Φ_(e,λ)(λ) d λ) = K/K_m`, where `Φ_(e,λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency, `λ` is wavelength, `K` is luminous efficacy of radiation (item 7-11.1), and `K_m` is maximum luminous efficacy (item 7-11.3). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V` for photopic vision; `V'` for scotopic vision; `V_(mes;m)` for mesopic vision; `V_10` for the CIE 10° photopic photometric observer; `V_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute luminousEfficiency: LuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-10.2 spectral luminous efficiency */
    attribute def SpectralLuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.2 spectral luminous efficiency
         * symbol(s): `V(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant flux (item 7-4.1) at wavelength `λ_m` and that at wavelength `λ`, such that both produce equally intense luminous sensations for a specified photometric condition and `λ_m` is chosen so that the maximum value of this quotient is equal to 1
         * remarks: The spectral luminous efficiency of the human eye depends on a number of factors, particularly the state of visual adaptation and the size and position of the source in the visual field. The photometric condition should be specified (e.g. photopic, scotopic, mesopic). If it is not specified, photopic vision is assumed and the symbol `V(λ)` is used. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V(λ)` for photopic vision; `V'(λ)` for scotopic vision; `V_(mes;m)(λ)` for mesopic vision; `V_10(λ)` for the CIE 10° photopic photometric observer; `V_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute spectralLuminousEfficiency: SpectralLuminousEfficiencyValue :> scalarQuantities;

    /* ISO-80000-7 item 7-11.1 luminous efficacy of radiation */
    attribute def LuminousEfficacyOfRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.1 luminous efficacy of radiation
         * symbol(s): `K`
         * application domain: specified photometric condition
         * name: LuminousEfficacyOfRadiation
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of luminous flux (item 7-13) and the corresponding radiant flux (item 7-4.1) for a specified photometric condition
         * remarks: Luminous efficacy of radiation for photopic vision is expressed by `K = Φ_V/Φ_e`, where `Φ_v` is luminous flux (item 7-13) and `Φ_e` is radiant flux (item 7-4.1). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K` for photopic vision; `K'` for scotopic vision; `K_(mes;m)` for mesopic vision; `K_10` for the CIE 10° photopic photometric observer; `K_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfRadiationUnit[1];
    }

    attribute luminousEfficacyOfRadiation: LuminousEfficacyOfRadiationValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfRadiationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.2 spectral luminous efficacy */
    attribute def SpectralLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.2 spectral luminous efficacy
         * symbol(s): `K(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: product of spectral luminous efficiency (item 7-10.2) and maximum luminous efficacy (item 7-11.3) for a specified photometric condition
         * remarks: Spectral luminous efficacy for photopic vision is expressed by `K(λ) = K_m V(λ)`, where `K_m` is maximum luminous efficacy (item 7-11.3), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K(λ)` for photopic vision>; `K'(λ)` for scotopic vision; `K_(mes;m)(λ)` for mesopic vision; `K_10(λ)` for the CIE 10° photopic photometric observer; `K_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpectralLuminousEfficacyUnit[1];
    }

    attribute spectralLuminousEfficacy: SpectralLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def SpectralLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.3 maximum luminous efficacy */
    attribute def MaximumLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.3 maximum luminous efficacy
         * symbol(s): `K_m`
         * application domain: specified photometric condition
         * name: MaximumLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: maximum value of spectral luminous efficacy for a specified photometric condition
         * remarks: See also 0.4 and 0.5. The value of maximum luminous efficacy for photopic vision is calculated by `K_m = 683 / (V(λ_(cd))) ["cd"*"sr"*"W"^-1] = 683 ["lm"*"W"^-1]` where `V(λ)` is the spectral luminous efficiency for photopic vision and `λ_(cd)` is the wavelength in air corresponding to the frequency `540*10^12 ["Hz"]` specified in the definition of the SI unit candela. Symbols for different photometric conditions: `K_m` for photopic vision; `K'_m` for scotopic vision; `K_(m,mes;m)` for mesopic vision; `K_(m,10)` for the CIE 10° photopic photometric observer; `K_(m,M)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MaximumLuminousEfficacyUnit[1];
    }

    attribute maximumLuminousEfficacy: MaximumLuminousEfficacyValue[*] nonunique :> scalarQuantities;

    attribute def MaximumLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-11.4 luminous efficacy of a source */
    attribute def LuminousEfficacyOfASourceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.4 luminous efficacy of a source
         * symbol(s): `η_v`, `(η)`
         * application domain: generic
         * name: LuminousEfficacyOfASource
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of the luminous flux emitted and the power consumed by the source, expressed by `η_v = Φ_v/P`, where `Φ_v` is luminous flux (item 7-13) and `P` is the power (ISO 80000-4) consumed by the source
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEfficacyOfASourceUnit[1];
    }

    attribute luminousEfficacyOfASource: LuminousEfficacyOfASourceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEfficacyOfASourceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-12 luminous energy, quantity of light */
    attribute def LuminousEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-12 luminous energy, quantity of light
         * symbol(s): `Q_v`, `(Q)`
         * application domain: generic
         * name: LuminousEnergy
         * quantity dimension: T^1*J^1
         * measurement unit(s): lm*s, cd*sr*s
         * tensor order: 0
         * definition: energy of electromagnetic waves weighted by the spectral luminous efficiency (item 7-10.2) multiplied by maximum luminous efficacy (item 7-11.3) of a specified photometric condition
         * remarks: Luminous energy for photopic vision is expressed by `Q_v = K_m int_0^∞ Q_(e,λ)(λ) V(λ) dλ`, where `Q_(e,λ)(λ)` is the spectral radiant energy (item 7-2.2) at wavelength `λ` (ISO 80000-3), `V(λ)` is spectral luminous efficiency (item 7-10.2), and `K_m` is maximum luminous efficacy (7-11.3). Luminous energy can be emitted, transferred or received. Luminous energy can be expressed by the time integral of the luminous flux (item 7-13), `Φ_v`, over a given duration (ISO 80000-3), `Δt`: `Q_v = int_(Δt) Φ_v dt` . The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousEnergyUnit[1];
    }

    attribute luminousEnergy: LuminousEnergyValue[*] nonunique :> scalarQuantities;

    attribute def LuminousEnergyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (durationPF, luminousIntensityPF); }
    }

    alias QuantityOfLightUnit for LuminousEnergyUnit;
    alias QuantityOfLightValue for LuminousEnergyValue;
    alias quantityOfLight for luminousEnergy;

    /* ISO-80000-7 item 7-13 luminous flux */
    attribute def LuminousFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-13 luminous flux
         * symbol(s): `Φ_v`, `(Φ)`
         * application domain: generic
         * name: LuminousFlux
         * quantity dimension: J^1
         * measurement unit(s): lm, cd*sr
         * tensor order: 0
         * definition: change in luminous energy with time, expressed by `Φ_v = (d Q_v)/(dt)`, where `Q_v` is the luminous energy (item 7-12) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: Luminous flux is a quantity derived from the radiant flux (item 7-4.1), `Φ_e`, by evaluating the radiation according to its action upon the CIE standard photometric observer. (See CIE S 017/E:2011, term 17-738.) Luminous flux can be derived from the spectral radiant flux distribution by `Φ_v = K_m int_0^oo Φ_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `Φ_(e,λ)(λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength (ISO 80000-3). The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousFluxUnit[1];
    }

    attribute luminousFlux: LuminousFluxValue[*] nonunique :> scalarQuantities;

    attribute def LuminousFluxUnit :> DerivedUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    /* See package ISQBase for the declarations of LuminousIntensityValue and LuminousIntensityUnit */

    /* ISO-80000-7 item 7-15 luminance */
    attribute def LuminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-15 luminance
         * symbol(s): `L_v`, `(L)`
         * application domain: generic
         * name: Luminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: density of luminous intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_v = (dI_v)/(dA) 1/cos(α)`, where `I_v` is luminous intensity (item 7-14), `A` is area (ISO 80000-3) and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: Luminance can be derived from the spectral radiance distribution by `L_v = K_m int_0^∞ L_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `L_(e,λ)(λ)` is the spectral radiance (item 7-6.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also 0.1. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminanceUnit[1];
    }

    attribute luminance: LuminanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-16 illuminance */
    attribute def IlluminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-16 illuminance
         * symbol(s): `E_v`, `(E)`
         * application domain: generic
         * name: Illuminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lx, cd*sr*m^-2
         * tensor order: 0
         * definition: density of incident luminous flux with respect to area at a point on a real or imaginary surface, expressed by `E_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) on which the luminous flux is incident
         * remarks: Illuminance can be derived from the spectral irradiance distribution by `E_v = K_m int_0^∞ E_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `E_(e,λ)(λ)` is the spectral irradiance (item 7-7.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical illuminance" is defined by the mean value of illuminance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(v,0) = int_(4π) L_v dΩ`, where `Ω` is solid angle (ISO 80000-3) and `L_v` is luminance (item 7-15). It can be expressed by the quotient of the luminous flux (item 7-13) of all the light incident on the outer surface of an infinitely small sphere centred at the given point, and the area (ISO 80000-3) of the diametrical cross-section of that sphere.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IlluminanceUnit[1];
    }

    attribute illuminance: IlluminanceValue[*] nonunique :> scalarQuantities;

    attribute def IlluminanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-17 luminous exitance */
    attribute def LuminousExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-17 luminous exitance
         * symbol(s): `M_v`, `(M)`
         * application domain: generic
         * name: LuminousExitance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lm/m^2, cd*sr*m^-2
         * tensor order: 0
         * definition: density of exiting luminous flux with respect to area at a point on a real or imaginary surface, expressed by `M_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) from which the luminous flux leaves
         * remarks: Luminous exitance can be derived from the spectral radiant exitance distribution by `M_v = K_m int_0^∞ M_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `M_(e_λ)(λ)` is the spectral radiant exitance (item 7-8.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExitanceUnit[1];
    }

    attribute luminousExitance: LuminousExitanceValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-18 luminous exposure, quantity of illumination, light exposure */
    attribute def LuminousExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-18 luminous exposure, quantity of illumination, light exposure
         * symbol(s): `H_v`, `(H)`
         * application domain: generic
         * name: LuminousExposure
         * quantity dimension: L^-2*T^1*J^1
         * measurement unit(s): lx*s, cd*sr*m^-2*s
         * tensor order: 0
         * definition: density of incident luminous energy with respect to area at a point on a real or imaginary surface, expressed by `H_v = (dQ_v)/(dA)`, where `Q_v` is luminous energy (item 7-12) and `A` is the area on which the luminous energy is incident (ISO 80000-3)
         * remarks: Luminous exposure can be derived from the spectral radiant exposure distribution by `H_v = K_m int_0^∞ H_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `H_(e_λ)(λ)` is the spectral radiant exposure (item 7-9.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousExposureUnit[1];
    }

    attribute luminousExposure: LuminousExposureValue[*] nonunique :> scalarQuantities;

    attribute def LuminousExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, luminousIntensityPF); }
    }

    alias QuantityOfIlluminationUnit for LuminousExposureUnit;
    alias QuantityOfIlluminationValue for LuminousExposureValue;
    alias quantityOfIllumination for luminousExposure;

    alias LightExposureUnit for LuminousExposureUnit;
    alias LightExposureValue for LuminousExposureValue;
    alias lightExposure for luminousExposure;

    /* ISO-80000-7 item 7-19.1 photon number, number of photons */
    attribute def PhotonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-19.1 photon number, number of photons
         * symbol(s): `N_p`
         * application domain: generic
         * name: PhotonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant energy and photon energy, expressed by `N_p = Q_e/(h ν)`, where `Q_e` is radiant energy (item 7-2.1), `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon number can also be expressed by the time integral of the photon flux (item 7-20), `Φ_p`, over a given duration, `Δt`, `N_p = int_(Δt) Φ_p dt`
         */
    }
    attribute photonNumber: PhotonNumberValue :> scalarQuantities;

    alias numberOfPhotons for photonNumber;

    /* ISO-80000-7 item 7-19.2 photon energy */
    attribute photonEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 7-19.2 photon energy
         * symbol(s): `Q_p`, `(Q)`
         * application domain: generic
         * name: PhotonEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: product of the Planck constant and frequency, expressed by `Q_p = h ν` where `h` is the Planck constant (ISO 80000-1) and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon energy can be emitted, transferred or received. For monochromatic radiation, photon energy may be expressed by photon number (item 7-19.1). The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding photometric quantity is "luminous energy" (item 7-12).
         */
    }

    /* ISO-80000-7 item 7-20 photon flux */
    attribute def PhotonFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-20 photon flux
         * symbol(s): `Φ_p`, `(Φ)`
         * application domain: generic
         * name: PhotonFlux
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: rate of photon number per time interval, expressed by `Φ_p = (d N_p)/(dt)`, where `N_p` is photon number (e.g. given by item 7-19.1), transmitted or received, and `t` is time (ISO 80000-3)
         * remarks: Photon flux `Φ_p` is related to radiant flux (item 7-4.1), `Φ_e`, of monochromatic radiation, by `Φ_p = Φ_e/(h ν)` where `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave. The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding photometric quantity is "luminous flux" (item 7-13).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonFluxUnit[1];
    }

    attribute photonFlux: PhotonFluxValue[*] nonunique :> scalarQuantities;

    attribute def PhotonFluxUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-21 photon intensity */
    attribute def PhotonIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-21 photon intensity
         * symbol(s): `I_p`, `(I)`
         * application domain: generic
         * name: PhotonIntensity
         * quantity dimension: T^-1
         * measurement unit(s): s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon flux with respect to solid angle in a specified direction, expressed by `I_p = (dΦ_p)/(dΩ)`, where `Φ_p` is the photon flux (item 7-20) emitted in the given direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The distribution of the photon intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)` , is used to determine the photon flux (item 7-20) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_p = int int_Ω I_v(θ,ϕ) sin(θ) dϕ dθ`. The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding photometric quantity is "luminous intensity" (item 7-14).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIntensityUnit[1];
    }

    attribute photonIntensity: PhotonIntensityValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIntensityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-7 item 7-22 photon radiance */
    attribute def PhotonRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-22 photon radiance
         * symbol(s): `L_p`, `(L)`
         * application domain: generic
         * name: PhotonRadiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_p = (dI_p)/(dA) 1/cos(α)`, where `I_p` is photon intensity (item 7-21), `A` is area (ISO 80000-3) and `α` the angle between the normal to the surface at the specified point and the specified direction
         * remarks: The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding photometric quantity is "luminance" (item 7-15).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonRadianceUnit[1];
    }

    attribute photonRadiance: PhotonRadianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonRadianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-23 photon irradiance */
    attribute def PhotonIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-23 photon irradiance
         * symbol(s): `E_p`, `(E)`
         * application domain: generic
         * name: PhotonIrradiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of incident photon flux with respect to area at a point on a real or imaginary surface, expressed by `E_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) on which the photon flux is incident
         * remarks: The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding photometric quantity is "illuminance" (item 7-16).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonIrradianceUnit[1];
    }

    attribute photonIrradiance: PhotonIrradianceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonIrradianceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-24 photon exitance */
    attribute def PhotonExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-24 photon exitance
         * symbol(s): `M_p`, `(M)`
         * application domain: generic
         * name: PhotonExitance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of exiting photon flux with respect to area at a point on a real or imaginary surface, expressed by `M_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) from which the photon flux leaves
         * remarks: The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding photometric quantity is "luminous exitance" (item 7-17).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExitanceUnit[1];
    }

    attribute photonExitance: PhotonExitanceValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-7 item 7-25 photon exposure */
    attribute def PhotonExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-25 photon exposure
         * symbol(s): `H_p`, `(H)`
         * application domain: generic
         * name: PhotonExposure
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: density of incident photon number with respect to area at a point on a real or imaginary surface, expressed by `H_p = (dN_p)/(dA)`, where `N_p` is photon number (item 7-19.1) and `A` is the area (ISO 80000-3) on which the photons are incident
         * remarks: The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding photometric quantity is "luminous exposure" (item 7-18).
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhotonExposureUnit[1];
    }

    attribute photonExposure: PhotonExposureValue[*] nonunique :> scalarQuantities;

    attribute def PhotonExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer
         * symbol(s): `X,Y,Z`
         * application domain: generic
         * name: TristimulusValuesForTheCie1931StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1931 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[cd*m^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 // int_0^∞ S_λ(λ) overline y(λ) dλ`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1931StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1931StandardColorimetricObserver: TristimulusValuesForTheCie1931StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer
         * symbol(s): `X_10,Y_10,Z_10`
         * application domain: generic
         * name: TristimulusValuesForTheCie1964StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1964 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `["cd"*"m"^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 /( int_0^∞ S_λ(λ) overline y(λ) dλ)`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TristimulusValuesForTheCie1964StandardColorimetricObserverUnit[1];
    }

    attribute tristimulusValuesForTheCie1964StandardColorimetricObserver: TristimulusValuesForTheCie1964StandardColorimetricObserverValue[*] nonunique :> scalarQuantities;

    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, luminousIntensityPF); }
    }

    /* ISO-80000-7 item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer
         * symbol(s): `overline x(λ)`, `overline y(λ)`, `overline z(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x(λ)` , `overline y(λ)` , `overline z(λ)` in the CIE 1931 standard colorimetric system
         * remarks: Values of `overline x(λ)` , `overline y(λ)` and `overline z(λ)` are defined in the CIE 1931 standard colorimetric system (2° observer) — applicable to fields of observation of angular opening from 1° to 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer
         * symbol(s): `overline x_10(λ)`, `overline y_10(λ)`, `overline z_10(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x_10(λ)` , `overline y_10(λ)` , `overline z_10(λ)` in the CIE 1964 standard colorimetric system
         * remarks: Values of `overline x_10(λ)` , `overline y_10(λ)` and `overline z_10(λ)` are defined in the CIE 1964 standard colorimetric system (10° observer) — applicable to fields of observation with angles greater than 4°.
         */
    }
    attribute cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system
         * symbol(s): `x,y,z`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1931 standard colorimetric observer (item 7-26.1) and their sum, expressed by `x = X / (X+Y+Z)` , `y = Y / (X+Y+Z)` , `z = Z / (X+Y+Z)`
         * remarks: Since `x + y + z = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1931StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system
         * symbol(s): `x_10,y_10,z_10`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1964 standard colorimetric observer (item 7-26.2) and their sum, expressed by `x_10 = X_10 / (X_10+Y_10+Z_10)`, `y_10 = Y_10 / (X_10+Y_10+Z_10)`, `z_10 = Z_10 / (X_10+Y_10+Z_10)`
         * remarks: Since `x_10 + y_10 + z_10 = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute chromaticityCoordinatesInTheCie1964StandardColorimetricSystem: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> scalarQuantities;

    /* ISO-80000-7 item 7-29.1 colour temperature */
    attribute colourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.1 colour temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: ColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator whose radiation has the same chromaticity as that of a given stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-29.2 correlated colour temperature */
    attribute correlatedColourTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 7-29.2 correlated colour temperature
         * symbol(s): `T_"cp"`
         * application domain: generic
         * name: CorrelatedColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator having the chromaticity nearest the chromaticity associated with the given spectral distribution on a modified 1976 CIE Uniform Chromaticity Scale (UCS) diagram where `u',2/3 v'` are the coordinates of the Planckian locus and the test stimulus
         * remarks: None.
         */
    }

    /* ISO-80000-7 item 7-30.1 emissivity */
    attribute def EmissivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.1 emissivity
         * symbol(s): `ε`, `ε_T`
         * application domain: generic
         * name: Emissivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator and the radiant exitance of a Planckian radiator at the same temperature, expressed by `ε = M/M_b`, where `M` is the radiant exitance (item 7-8.1) of a thermal radiator and `M_b` is the radiant exitance of a Planckian radiator at the same temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute emissivity: EmissivityValue :> scalarQuantities;

    /* ISO-80000-7 item 7-30.2 emissivity at a specified wavelength */
    attribute def EmissivityAtASpecifiedWavelengthValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.2 emissivity at a specified wavelength
         * symbol(s): `ε(λ)`
         * application domain: generic
         * name: EmissivityAtASpecifiedWavelength (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator at a specified wavelength and the radiant exitance of a Planckian radiator at the same temperature and at the same wavelength, expressed by `ε(λ) = M(λ) / M_b(λ)`, where `M(λ)` is the radiant exitance (item 7-8.1) of a thermal radiator at a specified wavelength and `M_b(λ)` is the radiant exitance of a Planckian radiator at the same temperature at a specified wavelength (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute emissivityAtASpecifiedWavelength: EmissivityAtASpecifiedWavelengthValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.1 absorptance */
    attribute def AbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.1 absorptance
         * symbol(s): `α`, `a`
         * application domain: generic
         * name: Absorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed radiant flux and incident radiant flux, expressed by `α = Φ_a/Φ_m`, where `Φ_a` is absorbed radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `ρ` is reflectance (item 7-31.3) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute absorptance: AbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.2 luminous absorptance */
    attribute def LuminousAbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.2 luminous absorptance
         * symbol(s): `α_v`
         * application domain: generic
         * name: LuminousAbsorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed luminous flux and incident luminous flux, expressed by `α_v = Φ_(v,a)/Φ_(v,m)`, where `Φ_(v,a)` is absorbed luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral absorptance, `α(λ)`, luminous absorptance can be calculated by `α_v = (int_0^∞ α(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.1.
         */
    }
    attribute luminousAbsorptance: LuminousAbsorptanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.3 reflectance */
    attribute def ReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.3 reflectance
         * symbol(s): `ρ`
         * application domain: generic
         * name: Reflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected radiant flux and incident radiant flux, expressed by `ρ = Φ_r/Φ_m`, where `Φ_r` is reflected radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute reflectance: ReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.4 luminous reflectance */
    attribute def LuminousReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.4 luminous reflectance
         * symbol(s): `ρ_v`
         * application domain: generic
         * name: LuminousReflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected luminous flux and incident luminous flux, is expressed by `ρ_v = Φ_(v,r)/Φ_(v,m)`, where `Φ_(v,r)` is reflected luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral reflectance, `ρ(λ)`, luminous reflectance can be calculated by `ρ_v = (int_0^∞ ρ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.3.
         */
    }
    attribute luminousReflectance: LuminousReflectanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.5 transmittance */
    attribute def TransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.5 transmittance
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: Transmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted radiant flux and incident radiant flux, expressed by `τ = Φ_t/Φ_m`, where `Φ_t` is transmitted radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `ρ` is reflectance (item 7-31.3).
         */
    }
    attribute transmittance: TransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-31.6 luminous transmittance */
    attribute def LuminousTransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.6 luminous transmittance
         * symbol(s): `τ_v`
         * application domain: generic
         * name: LuminousTransmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted luminous flux and incident luminous flux, expressed by `τ_v = Φ_(v,t)/Φ_(v,m)`, where `Φ_(v,t)` is transmitted luminous flux (item 7-13) and `Φ_(v,m)` is luminous flux of the incident radiation
         * remarks: From the spectral transmittance `τ(λ)`, luminous transmittance can be calculated by `τ_v = (int_0^∞ τ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is the spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is the spectral luminous efficiency (item 7-10.2). See also item 7-31.5.
         */
    }
    attribute luminousTransmittance: LuminousTransmittanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance */
    attribute def TransmittanceOpticalDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance
         * symbol(s): `D`, `A_10`, `D_τ`
         * application domain: generic
         * name: TransmittanceOpticalDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: logarithm to base 10 of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the optical density can be expressed by `A_10(λ) = -log(τ(λ))`, where `τ(λ)` is the transmittance (item 7-31.5) in terms of wavelength. In spectroscopy, the name "absorbance" `A_10` is generally used.
         */
    }
    attribute transmittanceOpticalDensity: TransmittanceOpticalDensityValue :> scalarQuantities;

    alias opticalDensity for transmittanceOpticalDensity;

    alias transmittanceDensity for transmittanceOpticalDensity;

    alias decadicAbsorbance for transmittanceOpticalDensity;

    /* ISO-80000-7 item 7-32.2 Napierian absorbance */
    attribute def NapierianAbsorbanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.2 Napierian absorbance
         * symbol(s): `A_n`, `B`
         * application domain: generic
         * name: NapierianAbsorbance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: natural (Napierian) logarithm of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the Napierian absorbance can be expressed by `A_n(λ) = B(λ) = -log(τ(λ))`. It can also be expressed as `A_n(λ) = l*α(λ)`, where `α` is linear absorption coefficient (item 7-35.2) and `l` is length (ISO 80000-3) traversed.
         */
    }
    attribute napierianAbsorbance: NapierianAbsorbanceValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.1 radiance factor */
    attribute def RadianceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.1 radiance factor
         * symbol(s): `β_e`, `(β)`
         * application domain: generic
         * name: RadianceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiance of a surface element in a specified direction and the radiance of the perfect reflecting diffuser or perfect transmitting diffuser identically irradiated and viewed, expressed by `β_e = L_(e,n)/L_(e,d)`, where `L_(e,n)` is the radiance (item 7-6.1) of a surface element in a given direction and `L_(e,d)` is the radiance of the perfect reflecting or transmitting diffuser identically irradiated and viewed
         * remarks: The definition holds for a surface element of a non-self-radiating medium, in a given direction and under specified conditions of irradiation. Radiance factor is equivalent to reflectance factor (item 7-34) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is `2π ["sr"]`. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called "perfect diffuser".
         */
    }
    attribute radianceFactor: RadianceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-33.2 luminance factor */
    attribute def LuminanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.2 luminance factor
         * symbol(s): `β_v`, `(β)`
         * application domain: generic
         * name: LuminanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the luminance of a surface element in a specified direction and the luminance of the perfect reflecting diffuser or perfect transmitting diffuser identically illuminated and viewed, expressed by `β_v = L_(v,n)/L_(v,d)`, where `L_(v,n)` is the luminance (item 7-15) of a surface element in a given direction and `L_(v,d)` is the luminance of the perfect reflecting or transmitting diffuser identically illuminated and viewed
         * remarks: The definition holds for a surface element of a non-luminous medium, in a given direction and under specified conditions of irradiation. This quantity is also defined spectrally and is called "spectral luminance factor". For the analogous radiant quantity "radiance factor", see item 7-33.1.
         */
    }
    attribute luminanceFactor: LuminanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-34 reflectance factor */
    attribute def ReflectanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-34 reflectance factor
         * symbol(s): `R`
         * application domain: generic
         * name: ReflectanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the flux reflected in the directions delimited by a given cone with apex at a surface element and the flux reflected in the same directions by a perfect reflecting diffuser identically irradiated or illuminated, expressed by `R = Φ_n/Φ_d`, where `Φ_n` is the flux reflected in the directions delimited by a given cone and `Φ_d` is the flux reflected in the same directions by an identically irradiated diffuser of reflectance (item 7-31.3) equal to 1
         * remarks: The flux can be a radiant flux (item 7‐4.1) or a luminous flux (item 7‐13). The definition holds for a surface element, for the part of the reflected radiation contained in a given cone with apex at the surface element, and for incident radiation of given spectral composition, polarization and geometric distribution. Reflectance factor is equivalent to radiance factor (item 7-33.1) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is 2π sr. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called a perfect diffuser.
         */
    }
    attribute reflectanceFactor: ReflectanceFactorValue :> scalarQuantities;

    /* ISO-80000-7 item 7-35.1 linear attenuation coefficient, linear extinction coefficient */
    attribute def LinearAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.1 linear attenuation coefficient, linear extinction coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: radiometry
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux caused by absorption and scattering
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear attenuation coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing and scattering medium `μ(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAttenuationCoefficientUnit[1];
    }

    attribute linearAttenuationCoefficient: LinearAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias LinearExtinctionCoefficientUnit for LinearAttenuationCoefficientUnit;
    alias LinearExtinctionCoefficientValue for LinearAttenuationCoefficientValue;
    alias linearExtinctionCoefficient for linearAttenuationCoefficient;

    /* ISO-80000-7 item 7-35.2 linear absorption coefficient */
    attribute def LinearAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.2 linear absorption coefficient
         * symbol(s): `α_l`, `a_l`, `α`
         * application domain: radiometry
         * name: LinearAbsorptionCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux (item 7-4.1) caused by absorption
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear absorption coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing medium `α_l(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. It can also be expressed as a function of transmittance (item 7-31.5). `α_l = -ln(τ)/l = A_n/l`. The linear absorption coefficient is that part of the linear attenuation coefficient (item 7-35.1) that is due to absorption. Scattering might also contribute. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearAbsorptionCoefficientUnit[1];
    }

    attribute linearAbsorptionCoefficient: LinearAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-7 item 7-36.1 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.1 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: radiometry
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient (item 7-35.1), `μ`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `μ_m(λ) = (μ(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAttenuationCoefficientUnit[1];
    }

    attribute massAttenuationCoefficient: MassAttenuationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-36.2 mass absorption coefficient */
    attribute def MassAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.2 mass absorption coefficient
         * symbol(s): `α_m`
         * application domain: radiometry
         * name: MassAbsorptionCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear absorption coefficient (item 7-35.2), `α`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `α_m(λ) = (α(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassAbsorptionCoefficientUnit[1];
    }

    attribute massAbsorptionCoefficient: MassAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MassAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-7 item 7-37 molar absorption coefficient */
    attribute def MolarAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-37 molar absorption coefficient
         * symbol(s): `χ`
         * application domain: radiometry
         * name: MolarAbsorptionCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: product of linear absorption coefficient and molar volume, expressed by `χ = α V_m`, where `α` is linear absorption coefficient (item 7-35.2) and `V_m` is molar volume (ISO 80000-9)
         * remarks: The molar absorption coefficient can also be expressed by `χ = α c` where `c` is amount-of-substance concentration (ISO 80000-9). Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarAbsorptionCoefficientUnit[1];
    }

    attribute molarAbsorptionCoefficient: MolarAbsorptionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def MolarAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_light.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQLight {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-7:2019 "Light and radiation"
     * see also https://www.iso.org/standard/64977.html
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
    /* ISO-80000-7 item 7-1.1 speed of light in a medium */
    attribute def SpeedOfLightInAMediumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-1.1 speed of light in a medium
         * symbol(s): `c`
         * application domain: generic
         * name: SpeedOfLightInAMedium
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: phase speed of an electromagnetic wave at a given point in a medium
         * remarks: See also ISO 80000-3. The value of the speed of light in a medium can depend on the frequency, polarization, and direction. For the definition of the speed of electromagnetic waves in vacuum, `c_0`, see ISO 80000-1.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpeedOfLightInAMediumUnit[1];
    }
    attribute def speedOfLightInAMedium : SpeedOfLightInAMediumValue[*] nonunique;
    attribute def SpeedOfLightInAMediumUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-1.2 refractive index */
    attribute def RefractiveIndexValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-1.2 refractive index
         * symbol(s): `n`
         * application domain: generic
         * name: RefractiveIndex (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of light in vacuum (ISO 80000-1) and speed of light in a medium (item 7-1.1)
         * remarks: The value of the refractive index can depend on the frequency, polarization, and direction. The refractive index is expressed by n = c_0/c, where c_()_0 is the speed of light in vacuum and c is the speed of light in the medium. For a medium with absorption, the complex refractive index n is defined by n = n + ik where k is spectral absorption index (IEC 60050-845) and i is imaginary unit. The refractivity is expressed by n -1, where n is refractive index.
         */
    }
    attribute def refractiveIndex : RefractiveIndexValue;
    /* ISO-80000-7 item 7-2.1 radiant energy */
    attribute def radiantEnergy : EnergyValue {
        doc
        /*
         * source: item 7-2.1 radiant energy
         * symbol(s): `Q_e`, `W`, `U`, `(Q)`
         * application domain: electromagnetism
         * name: RadiantEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) emitted, transferred or received in form of electromagnetic waves
         * remarks: Radiant energy can be expressed by the time integral of radiant flux (item 7-4.1), `Φ_e`, over a given duration (ISO 80000-3), `Δt`: `Q_e = int_(Δ t) Φ_e dt`. Radiant energy is expressed either as a function of wavelength (ISO 80000-3), `λ`, as a function of frequency (ISO 80000-3), `ν`, or as a function of wavenumber, `σ`. (See also 0.1.) The corresponding photometric quantity is "luminous energy" (item 7-12). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
    }
    /* ISO-80000-7 item 7-2.2 spectral radiant energy */
    attribute def SpectralRadiantEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-2.2 spectral radiant energy
         * symbol(s): `Q_(e,λ)`, `W_λ`, `U_λ`, `(Q_λ)`
         * application domain: generic
         * name: SpectralRadiantEnergy
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): J/nm, kg*m*s^-2
         * tensor order: 0
         * definition: spectral density of radiant energy, expressed by `Q_(e,λ) = (dQ_e) / (dλ)`, where `Q_e` is radiant energy (item 7-2.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant energy is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Q_e = int_(λ_1)^(λ_2) Q_(e,λ) dλ`
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantEnergyUnit[1];
    }
    attribute def spectralRadiantEnergy : SpectralRadiantEnergyValue[*] nonunique;
    attribute def SpectralRadiantEnergyUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-3.1 radiant energy density */
    attribute def RadiantEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.1 radiant energy density
         * symbol(s): `w`, `(ρ_e)`
         * application domain: generic
         * name: RadiantEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: volumetric density of radiant energy, expressed by `w = (dQ_e)/(dV)`, where `Q_e` is radiant energy (item 7-2.1) in an elementary three-dimensional domain and `V` is the volume (ISO 80000-3) of that domain
         * remarks: Radiant energy density within a Planckian radiator is given by `w = (4 σ)/(c_0) T^4` where `σ` is the Stefan-Boltzmann constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1) and `T` is thermodynamic temperature (ISO 80000-5).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadiantEnergyDensityUnit[1];
    }
    attribute def radiantEnergyDensity : RadiantEnergyDensityValue[*] nonunique;
    attribute def RadiantEnergyDensityUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-3.2 spectral radiant energy density in terms of wavelength */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.2 spectral radiant energy density in terms of wavelength
         * symbol(s): `w_λ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavelength
         * quantity dimension: L^-2*M^1*T^-2
         * measurement unit(s): J/(m^3*nm), kg*m^-2*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavelength, expressed by `w_λ = (dw)/(dλ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavelength `λ` (ISO 80000-3)
         * remarks: Spectral radiant energy density within a Planckian radiator is given by `w_λ = 8πhc_0*f(λ, T)`, where `h` is the Planck constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1), `T` is thermodynamic temperature (ISO 80000-5) and `f(λ,T) = (λ^-5)/(exp(c_2 λ^-1 T^-1) - 1)`. For the radiation constant `c_2` in `f(λ,T)`, see ISO 80000-1.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantEnergyDensityInTermsOfWavelengthUnit[1];
    }
    attribute def spectralRadiantEnergyDensityInTermsOfWavelength : SpectralRadiantEnergyDensityInTermsOfWavelengthValue[*] nonunique;
    attribute def SpectralRadiantEnergyDensityInTermsOfWavelengthUnit :> DerivedUnit {
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
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-3.3 spectral radiant energy density in terms of wavenumber */
    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-3.3 spectral radiant energy density in terms of wavenumber
         * symbol(s): `w_ṽ`, `ρ_ṽ`
         * application domain: generic
         * name: SpectralRadiantEnergyDensityInTermsOfWavenumber
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: change of radiant energy density with wavenumber, expressed by `w_ṽ = (dw)/(dṽ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavenumber `ṽ` (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantEnergyDensityInTermsOfWavenumberUnit[1];
    }
    attribute def spectralRadiantEnergyDensityInTermsOfWavenumber : SpectralRadiantEnergyDensityInTermsOfWavenumberValue[*] nonunique;
    attribute def SpectralRadiantEnergyDensityInTermsOfWavenumberUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-4.1 radiant flux, radiant power */
    attribute def RadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.1 radiant flux, radiant power
         * symbol(s): `Φ_e`, `P_e`, `Φ`, `P`
         * application domain: generic
         * name: RadiantFlux
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: change in radiant energy with time, expressed by `Φ_e = (dQ_e)/(dt)`, where `Q_e` is the radiant energy (item 7-2.1) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous flux" (item 7-13). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadiantFluxUnit[1];
    }
    attribute def radiantFlux : RadiantFluxValue[*] nonunique;
    attribute def RadiantFluxUnit :> DerivedUnit {
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
    alias RadiantPowerUnit for RadiantFluxUnit;
    alias RadiantPowerValue for RadiantFluxValue;
    alias radiantPower for radiantFlux;
    /* ISO-80000-7 item 7-4.2 spectral radiant flux, spectral radiant power */
    attribute def SpectralRadiantFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-4.2 spectral radiant flux, spectral radiant power
         * symbol(s): `Φ_(e,λ)`, `P_(e,λ)`, `(Φ_λ)`, `(P_λ)`
         * application domain: generic
         * name: SpectralRadiantFlux
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/nm, kg*m*s^-3
         * tensor order: 0
         * definition: spectral density of radiant flux, expressed by `Φ_(e,λ) = (dQ_e)/(dλ)`, where `Φ_e` is radiant flux (item 7-4.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant flux is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Φ_e = int_(λ_1)^(λ_2) Φ_(e,λ) dλ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantFluxUnit[1];
    }
    attribute def spectralRadiantFlux : SpectralRadiantFluxValue[*] nonunique;
    attribute def SpectralRadiantFluxUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    alias SpectralRadiantPowerUnit for SpectralRadiantFluxUnit;
    alias SpectralRadiantPowerValue for SpectralRadiantFluxValue;
    alias spectralRadiantPower for spectralRadiantFlux;
    /* ISO-80000-7 item 7-5.1 radiant intensity */
    attribute def RadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.1 radiant intensity
         * symbol(s): `I_e`, `(I)`
         * application domain: generic
         * name: RadiantIntensity
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W/sr, kg*m^2*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant flux with respect to solid angle in a specified direction, expressed by `I_e = (dΦ_e)/(dΩ)`, where `Φ_e` is the radiant flux (item 7-4.1) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the radiant intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,φ)`, is used to determine the radiant flux (item 7-4.1) within a certain solid angle (ISO 80000-3), `Ω`, of a source: `Φ_e = int int_Ω I_e(θ, φ) sin(θ) dφ dθ`. The corresponding photometric quantity is "luminous intensity" (item 7-14). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadiantIntensityUnit[1];
    }
    attribute def radiantIntensity : RadiantIntensityValue[*] nonunique;
    attribute def RadiantIntensityUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-5.2 spectral radiant intensity */
    attribute def SpectralRadiantIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-5.2 spectral radiant intensity
         * symbol(s): `I_(e,λ)`, `(I_λ)`
         * application domain: generic
         * name: SpectralRadiantIntensity
         * quantity dimension: L^1*M^1*T^-3
         * measurement unit(s): W/(sr*nm), kg*m*s^-3*sr^-1
         * tensor order: 0
         * definition: spectral density of radiant intensity, expressed by `I_(e, λ) = (d I_e)/(dλ)`, where `I_e` is radiant intensity (item 7-5.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant intensity is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `I_e = int_(λ_1)^(λ_2) I_(e,λ) dλ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantIntensityUnit[1];
    }
    attribute def spectralRadiantIntensity : SpectralRadiantIntensityValue[*] nonunique;
    attribute def SpectralRadiantIntensityUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-6.1 radiance */
    attribute def RadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.1 radiance
         * symbol(s): `L_e`, `(L)`
         * application domain: generic
         * name: Radiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/(sr*m^2), kg*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiant intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_e = (d I_e)/(dA) * 1/cos(α)`, where `I_e` is radiant intensity (item 7-5.1), `A` is area (ISO 80000-3), and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: See also 0.1. For Planckian radiation, `L_e = σ/π T^4` where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminance" (item 7-15). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadianceUnit[1];
    }
    attribute def radiance : RadianceValue[*] nonunique;
    attribute def RadianceUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-6.2 spectral radiance */
    attribute def SpectralRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-6.2 spectral radiance
         * symbol(s): `L_(e,λ)`, `(L_λ)`
         * application domain: generic
         * name: SpectralRadiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(sr*m^2*nm), kg*m^-1*s^-3*sr^-1
         * tensor order: 0
         * definition: density of radiance with respect to wavelength, expressed by `L_(e, λ) = (d L_e)/(d λ)` where `L_e` is radiance (item 7-6.1) in terms of wavelength λ(ISO 80000-3)
         * remarks: For Planckian radiation, `L_(e, λ)(λ) = (c(λ))/(4 π) ω_λ(λ) = h c_0^2 * f(λ,T)`, where `c(λ)` is phase speed (ISO 80000-3) of electromagnetic radiation of a wavelength (ISO 80000-3) `λ` in a given medium, `ω_λ(λ)` is spectral radiant energy density in terms of wavelength, `c_0` is speed of light in vacuum (ISO 80000-1), `h` is the Planck constant (ISO 80000-1), and `f(λ,T) = λ^-5/(exp(c_2 λ^-1 T^-1) - 1)`, where the radiation constant `c_2 = (hc)/k`. The integral of (total) radiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `L_e = int_(λ_1)^(λ_2) L_(e,λ) dλ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadianceUnit[1];
    }
    attribute def spectralRadiance : SpectralRadianceValue[*] nonunique;
    attribute def SpectralRadianceUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-7.1 irradiance */
    attribute def IrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.1 irradiance
         * symbol(s): `E_e`, `(E)`
         * application domain: generic
         * name: Irradiance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of incident radiant flux with respect to area at a point on a real or imaginary surface, expressed by `E_e = (d Φ_e)/(d A)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) on which the radiant flux is incident
         * remarks: The corresponding photometric quantity is "illuminance" (item 7-16). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical irradiance" is defined by the mean value of irradiance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(e,0) = int_(4 π) L_e d Ω` where `Ω` is solid angle (ISO 80000-3) and `L_e` is radiance (item 7-6.1). (See CIE DIS 017/E:2016, term 17-21-054.) It can be expressed by the quotient of the radiant flux (item 7-4.1) of all the radiation incident on the outer surface of an infinitely small sphere centred at the specified point and the area (ISO 80000-3) of the diametrical cross-section of that sphere. Spherical irradiance is also called "fluence rate" or "radiant fluence rate". The corresponding photometric quantity to spherical irradiance is called "spherical illuminance".
         */
        attribute :>> num : Real;
        attribute :>> mRef : IrradianceUnit[1];
    }
    attribute def irradiance : IrradianceValue[*] nonunique;
    attribute def IrradianceUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-7.2 spectral irradiance */
    attribute def SpectralIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-7.2 spectral irradiance
         * symbol(s): `E_(e,λ)`, `(E_λ)`
         * application domain: generic
         * name: SpectralIrradiance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of irradiance with respect to wavelength, expressed by `E_(e,λ) = (d E_e)/(dλ)`, where `E_e` is irradiance (item 7-7.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) irradiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `E_e = int_(λ_1)^(λ_2) E_(e,λ) d λ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralIrradianceUnit[1];
    }
    attribute def spectralIrradiance : SpectralIrradianceValue[*] nonunique;
    attribute def SpectralIrradianceUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-8.1 radiant exitance , radiant emittance */
    attribute def RadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.1 radiant exitance , radiant emittance
         * symbol(s): `M_e`, `(M)`
         * application domain: generic
         * name: RadiantExitance
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: density of exiting radiant flux with respect to area at a point on a real or imaginary surface, expressed by `M_e = (d Φ_e)/(dA)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) from which the radiant flux leaves
         * remarks: For Planckian radiation, `M_e = σT^4`, where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is "luminous exitance" (item 7-17). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadiantExitanceUnit[1];
    }
    attribute def radiantExitance : RadiantExitanceValue[*] nonunique;
    attribute def RadiantExitanceUnit :> DerivedUnit {
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
    alias RadiantEmittanceUnit for RadiantExitanceUnit;
    alias RadiantEmittanceValue for RadiantExitanceValue;
    alias radiantEmittance for radiantExitance;
    /* ISO-80000-7 item 7-8.2 spectral radiant exitance */
    attribute def SpectralRadiantExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-8.2 spectral radiant exitance
         * symbol(s): `M_(e,λ)`, `(M_λ)`
         * application domain: generic
         * name: SpectralRadiantExitance
         * quantity dimension: L^-1*M^1*T^-3
         * measurement unit(s): W/(m^2*nm), kg*m^-1*s^-3
         * tensor order: 0
         * definition: density of radiant exitance with respect to wavelength, expressed by `M_(e,λ) = (d M_e)/(dλ)`, where `M_e` is radiant exitance (item 7-8.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exitance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `M_e = int_(λ_1)^(λ_2) M_(e,λ) d λ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantExitanceUnit[1];
    }
    attribute def spectralRadiantExitance : SpectralRadiantExitanceValue[*] nonunique;
    attribute def SpectralRadiantExitanceUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    /* ISO-80000-7 item 7-9.1 radiant exposure */
    attribute def RadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.1 radiant exposure
         * symbol(s): `H_e`, `(H)`
         * application domain: generic
         * name: RadiantExposure
         * quantity dimension: M^1*T^-2
         * measurement unit(s): J/m^2, kg*s^-2
         * tensor order: 0
         * definition: density of incident radiant energy with respect to area at a point on a real or imaginary surface, expressed by `H_e = (d Q_e)/(dA)`, where `Q_e` is radiant energy (item 7-2.1) and `A` is the area on which the radiant energy is incident (ISO 80000-3)
         * remarks: The corresponding photometric quantity is "luminous exposure" (item 7-18). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RadiantExposureUnit[1];
    }
    attribute def radiantExposure : RadiantExposureValue[*] nonunique;
    attribute def RadiantExposureUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-9.2 spectral radiant exposure */
    attribute def SpectralRadiantExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-9.2 spectral radiant exposure
         * symbol(s): `H_(e,λ)`, `(H_λ)`
         * application domain: generic
         * name: SpectralRadiantExposure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/(m^2*nm), kg*m^-1*s^-2
         * tensor order: 0
         * definition: density of radiant exposure with respect to wavelength, expressed by `H_(e,λ) = (d H_e)/(dλ)`, where `H_e` is radiant exposure (item 7-9.1) in terms of wavelength `λ` (ISO 80000-3)
         * remarks: The integral of (total) radiant exposure is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `H_e = int_(λ_1)^(λ_2) H_(e,λ) d λ` .
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralRadiantExposureUnit[1];
    }
    attribute def spectralRadiantExposure : SpectralRadiantExposureValue[*] nonunique;
    attribute def SpectralRadiantExposureUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-10.1 luminous efficiency */
    attribute def LuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.1 luminous efficiency
         * symbol(s): `V`
         * application domain: specified photometric condition
         * name: LuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant flux (item 7-4.1) weighted by the spectral luminous efficiency (item 7-10.2) and the corresponding radiant flux for a specified photometric condition
         * remarks: Luminous efficiency for photopic vision is expressed by `V = (int_0^∞ Φ_(e,λ)(λ) V(λ) d λ)/(int_0^∞ Φ_(e,λ)(λ) d λ) = K/K_m`, where `Φ_(e,λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency, `λ` is wavelength, `K` is luminous efficacy of radiation (item 7-11.1), and `K_m` is maximum luminous efficacy (item 7-11.3). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V` for photopic vision; `V'` for scotopic vision; `V_(mes;m)` for mesopic vision; `V_10` for the CIE 10° photopic photometric observer; `V_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute def luminousEfficiency : LuminousEfficiencyValue;
    /* ISO-80000-7 item 7-10.2 spectral luminous efficiency */
    attribute def SpectralLuminousEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-10.2 spectral luminous efficiency
         * symbol(s): `V(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant flux (item 7-4.1) at wavelength `λ_m` and that at wavelength `λ`, such that both produce equally intense luminous sensations for a specified photometric condition and `λ_m` is chosen so that the maximum value of this quotient is equal to 1
         * remarks: The spectral luminous efficiency of the human eye depends on a number of factors, particularly the state of visual adaptation and the size and position of the source in the visual field. The photometric condition should be specified (e.g. photopic, scotopic, mesopic). If it is not specified, photopic vision is assumed and the symbol `V(λ)` is used. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V(λ)` for photopic vision; `V'(λ)` for scotopic vision; `V_(mes;m)(λ)` for mesopic vision; `V_10(λ)` for the CIE 10° photopic photometric observer; `V_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
    }
    attribute def spectralLuminousEfficiency : SpectralLuminousEfficiencyValue;
    /* ISO-80000-7 item 7-11.1 luminous efficacy of radiation */
    attribute def LuminousEfficacyOfRadiationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.1 luminous efficacy of radiation
         * symbol(s): `K`
         * application domain: specified photometric condition
         * name: LuminousEfficacyOfRadiation
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of luminous flux (item 7-13) and the corresponding radiant flux (item 7-4.1) for a specified photometric condition
         * remarks: Luminous efficacy of radiation for photopic vision is expressed by `K = Φ_V/Φ_e`, where `Φ_v` is luminous flux (item 7-13) and `Φ_e` is radiant flux (item 7-4.1). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K` for photopic vision; `K'` for scotopic vision; `K_(mes;m)` for mesopic vision; `K_10` for the CIE 10° photopic photometric observer; `K_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousEfficacyOfRadiationUnit[1];
    }
    attribute def luminousEfficacyOfRadiation : LuminousEfficacyOfRadiationValue[*] nonunique;
    attribute def LuminousEfficacyOfRadiationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-11.2 spectral luminous efficacy */
    attribute def SpectralLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.2 spectral luminous efficacy
         * symbol(s): `K(λ)`
         * application domain: specified photometric condition
         * name: SpectralLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: product of spectral luminous efficiency (item 7-10.2) and maximum luminous efficacy (item 7-11.3) for a specified photometric condition
         * remarks: Spectral luminous efficacy for photopic vision is expressed by `K(λ) = K_m V(λ)`, where `K_m` is maximum luminous efficacy (item 7-11.3), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K(λ)` for photopic vision>; `K'(λ)` for scotopic vision; `K_(mes;m)(λ)` for mesopic vision; `K_10(λ)` for the CIE 10° photopic photometric observer; `K_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpectralLuminousEfficacyUnit[1];
    }
    attribute def spectralLuminousEfficacy : SpectralLuminousEfficacyValue[*] nonunique;
    attribute def SpectralLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-11.3 maximum luminous efficacy */
    attribute def MaximumLuminousEfficacyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.3 maximum luminous efficacy
         * symbol(s): `K_m`
         * application domain: specified photometric condition
         * name: MaximumLuminousEfficacy
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: maximum value of spectral luminous efficacy for a specified photometric condition
         * remarks: See also 0.4 and 0.5. The value of maximum luminous efficacy for photopic vision is calculated by `K_m = 683 / (V(λ_(cd))) ["cd"*"sr"*"W"^-1] = 683 ["lm"*"W"^-1]` where `V(λ)` is the spectral luminous efficiency for photopic vision and `λ_(cd)` is the wavelength in air corresponding to the frequency `540*10^12 ["Hz"]` specified in the definition of the SI unit candela. Symbols for different photometric conditions: `K_m` for photopic vision; `K'_m` for scotopic vision; `K_(m,mes;m)` for mesopic vision; `K_(m,10)` for the CIE 10° photopic photometric observer; `K_(m,M)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MaximumLuminousEfficacyUnit[1];
    }
    attribute def maximumLuminousEfficacy : MaximumLuminousEfficacyValue[*] nonunique;
    attribute def MaximumLuminousEfficacyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-11.4 luminous efficacy of a source */
    attribute def LuminousEfficacyOfASourceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-11.4 luminous efficacy of a source
         * symbol(s): `η_v`, `(η)`
         * application domain: generic
         * name: LuminousEfficacyOfASource
         * quantity dimension: L^-2*M^-1*T^3*J^1
         * measurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3
         * tensor order: 0
         * definition: quotient of the luminous flux emitted and the power consumed by the source, expressed by `η_v = Φ_v/P`, where `Φ_v` is luminous flux (item 7-13) and `P` is the power (ISO 80000-4) consumed by the source
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousEfficacyOfASourceUnit[1];
    }
    attribute def luminousEfficacyOfASource : LuminousEfficacyOfASourceValue[*] nonunique;
    attribute def LuminousEfficacyOfASourceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-12 luminous energy, quantity of light */
    attribute def LuminousEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-12 luminous energy, quantity of light
         * symbol(s): `Q_v`, `(Q)`
         * application domain: generic
         * name: LuminousEnergy
         * quantity dimension: T^1*J^1
         * measurement unit(s): lm*s, cd*sr*s
         * tensor order: 0
         * definition: energy of electromagnetic waves weighted by the spectral luminous efficiency (item 7-10.2) multiplied by maximum luminous efficacy (item 7-11.3) of a specified photometric condition
         * remarks: Luminous energy for photopic vision is expressed by `Q_v = K_m int_0^∞ Q_(e,λ)(λ) V(λ) dλ`, where `Q_(e,λ)(λ)` is the spectral radiant energy (item 7-2.2) at wavelength `λ` (ISO 80000-3), `V(λ)` is spectral luminous efficiency (item 7-10.2), and `K_m` is maximum luminous efficacy (7-11.3). Luminous energy can be emitted, transferred or received. Luminous energy can be expressed by the time integral of the luminous flux (item 7-13), `Φ_v`, over a given duration (ISO 80000-3), `Δt`: `Q_v = int_(Δt) Φ_v dt` . The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding quantity for photons is "photon energy" (item 7-19.2).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousEnergyUnit[1];
    }
    attribute def luminousEnergy : LuminousEnergyValue[*] nonunique;
    attribute def LuminousEnergyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (durationPF, luminousIntensityPF);
        }
    }
    alias QuantityOfLightUnit for LuminousEnergyUnit;
    alias QuantityOfLightValue for LuminousEnergyValue;
    alias quantityOfLight for luminousEnergy;
    /* ISO-80000-7 item 7-13 luminous flux */
    attribute def LuminousFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-13 luminous flux
         * symbol(s): `Φ_v`, `(Φ)`
         * application domain: generic
         * name: LuminousFlux
         * quantity dimension: J^1
         * measurement unit(s): lm, cd*sr
         * tensor order: 0
         * definition: change in luminous energy with time, expressed by `Φ_v = (d Q_v)/(dt)`, where `Q_v` is the luminous energy (item 7-12) emitted, transferred or received and `t` is time (ISO 80000-3)
         * remarks: Luminous flux is a quantity derived from the radiant flux (item 7-4.1), `Φ_e`, by evaluating the radiation according to its action upon the CIE standard photometric observer. (See CIE S 017/E:2011, term 17-738.) Luminous flux can be derived from the spectral radiant flux distribution by `Φ_v = K_m int_0^oo Φ_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `Φ_(e,λ)(λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength (ISO 80000-3). The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding quantity for photons is "photon flux" (item 7-20).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousFluxUnit[1];
    }
    attribute def luminousFlux : LuminousFluxValue[*] nonunique;
    attribute def LuminousFluxUnit :> DerivedUnit {
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = luminousIntensityPF;
        }
    }
    /* ISO-80000-7 item 7-14 luminous intensity */
    /* See package ISQBase for the declarations of LuminousIntensityValue and LuminousIntensityUnit */
    /* ISO-80000-7 item 7-15 luminance */
    attribute def LuminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-15 luminance
         * symbol(s): `L_v`, `(L)`
         * application domain: generic
         * name: Luminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: density of luminous intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_v = (dI_v)/(dA) 1/cos(α)`, where `I_v` is luminous intensity (item 7-14), `A` is area (ISO 80000-3) and `α` is the angle between the normal to the surface at the specified point and the specified direction
         * remarks: Luminance can be derived from the spectral radiance distribution by `L_v = K_m int_0^∞ L_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `L_(e,λ)(λ)` is the spectral radiance (item 7-6.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also 0.1. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding quantity for photons is "photon radiance" (item 7-22).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminanceUnit[1];
    }
    attribute def luminance : LuminanceValue[*] nonunique;
    attribute def LuminanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-16 illuminance */
    attribute def IlluminanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-16 illuminance
         * symbol(s): `E_v`, `(E)`
         * application domain: generic
         * name: Illuminance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lx, cd*sr*m^-2
         * tensor order: 0
         * definition: density of incident luminous flux with respect to area at a point on a real or imaginary surface, expressed by `E_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) on which the luminous flux is incident
         * remarks: Illuminance can be derived from the spectral irradiance distribution by `E_v = K_m int_0^∞ E_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `E_(e,λ)(λ)` is the spectral irradiance (item 7-7.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding quantity for photons is "photon irradiance" (item 7-23). The quantity "spherical illuminance" is defined by the mean value of illuminance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(v,0) = int_(4π) L_v dΩ`, where `Ω` is solid angle (ISO 80000-3) and `L_v` is luminance (item 7-15). It can be expressed by the quotient of the luminous flux (item 7-13) of all the light incident on the outer surface of an infinitely small sphere centred at the given point, and the area (ISO 80000-3) of the diametrical cross-section of that sphere.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IlluminanceUnit[1];
    }
    attribute def illuminance : IlluminanceValue[*] nonunique;
    attribute def IlluminanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-17 luminous exitance */
    attribute def LuminousExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-17 luminous exitance
         * symbol(s): `M_v`, `(M)`
         * application domain: generic
         * name: LuminousExitance
         * quantity dimension: L^-2*J^1
         * measurement unit(s): lm/m^2, cd*sr*m^-2
         * tensor order: 0
         * definition: density of exiting luminous flux with respect to area at a point on a real or imaginary surface, expressed by `M_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) from which the luminous flux leaves
         * remarks: Luminous exitance can be derived from the spectral radiant exitance distribution by `M_v = K_m int_0^∞ M_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `M_(e_λ)(λ)` is the spectral radiant exitance (item 7-8.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding quantity for photons is "photon exitance" (item 7-24).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousExitanceUnit[1];
    }
    attribute def luminousExitance : LuminousExitanceValue[*] nonunique;
    attribute def LuminousExitanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-18 luminous exposure, quantity of illumination, light exposure */
    attribute def LuminousExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-18 luminous exposure, quantity of illumination, light exposure
         * symbol(s): `H_v`, `(H)`
         * application domain: generic
         * name: LuminousExposure
         * quantity dimension: L^-2*T^1*J^1
         * measurement unit(s): lx*s, cd*sr*m^-2*s
         * tensor order: 0
         * definition: density of incident luminous energy with respect to area at a point on a real or imaginary surface, expressed by `H_v = (dQ_v)/(dA)`, where `Q_v` is luminous energy (item 7-12) and `A` is the area on which the luminous energy is incident (ISO 80000-3)
         * remarks: Luminous exposure can be derived from the spectral radiant exposure distribution by `H_v = K_m int_0^∞ H_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `H_(e_λ)(λ)` is the spectral radiant exposure (item 7-9.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding quantity for photons is "photon exposure" (item 7-25).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousExposureUnit[1];
    }
    attribute def luminousExposure : LuminousExposureValue[*] nonunique;
    attribute def LuminousExposureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 1;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, luminousIntensityPF);
        }
    }
    alias QuantityOfIlluminationUnit for LuminousExposureUnit;
    alias QuantityOfIlluminationValue for LuminousExposureValue;
    alias quantityOfIllumination for luminousExposure;
    alias LightExposureUnit for LuminousExposureUnit;
    alias LightExposureValue for LuminousExposureValue;
    alias lightExposure for luminousExposure;
    /* ISO-80000-7 item 7-19.1 photon number, number of photons */
    attribute def PhotonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-19.1 photon number, number of photons
         * symbol(s): `N_p`
         * application domain: generic
         * name: PhotonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of radiant energy and photon energy, expressed by `N_p = Q_e/(h ν)`, where `Q_e` is radiant energy (item 7-2.1), `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon number can also be expressed by the time integral of the photon flux (item 7-20), `Φ_p`, over a given duration, `Δt`, `N_p = int_(Δt) Φ_p dt`
         */
    }
    attribute def photonNumber : PhotonNumberValue;
    alias numberOfPhotons for photonNumber;
    /* ISO-80000-7 item 7-19.2 photon energy */
    attribute def photonEnergy : EnergyValue {
        doc
        /*
         * source: item 7-19.2 photon energy
         * symbol(s): `Q_p`, `(Q)`
         * application domain: generic
         * name: PhotonEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: product of the Planck constant and frequency, expressed by `Q_p = h ν` where `h` is the Planck constant (ISO 80000-1) and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave
         * remarks: Photon energy can be emitted, transferred or received. For monochromatic radiation, photon energy may be expressed by photon number (item 7-19.1). The corresponding radiometric quantity is "radiant energy" (item 7-2.1). The corresponding photometric quantity is "luminous energy" (item 7-12).
         */
    }
    /* ISO-80000-7 item 7-20 photon flux */
    attribute def PhotonFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-20 photon flux
         * symbol(s): `Φ_p`, `(Φ)`
         * application domain: generic
         * name: PhotonFlux
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: rate of photon number per time interval, expressed by `Φ_p = (d N_p)/(dt)`, where `N_p` is photon number (e.g. given by item 7-19.1), transmitted or received, and `t` is time (ISO 80000-3)
         * remarks: Photon flux `Φ_p` is related to radiant flux (item 7-4.1), `Φ_e`, of monochromatic radiation, by `Φ_p = Φ_e/(h ν)` where `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave. The corresponding radiometric quantity is "radiant flux" (item 7-4.1). The corresponding photometric quantity is "luminous flux" (item 7-13).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonFluxUnit[1];
    }
    attribute def photonFlux : PhotonFluxValue[*] nonunique;
    attribute def PhotonFluxUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-7 item 7-21 photon intensity */
    attribute def PhotonIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-21 photon intensity
         * symbol(s): `I_p`, `(I)`
         * application domain: generic
         * name: PhotonIntensity
         * quantity dimension: T^-1
         * measurement unit(s): s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon flux with respect to solid angle in a specified direction, expressed by `I_p = (dΦ_p)/(dΩ)`, where `Φ_p` is the photon flux (item 7-20) emitted in the given direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The distribution of the photon intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)` , is used to determine the photon flux (item 7-20) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_p = int int_Ω I_v(θ,ϕ) sin(θ) dϕ dθ`. The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding photometric quantity is "luminous intensity" (item 7-14).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonIntensityUnit[1];
    }
    attribute def photonIntensity : PhotonIntensityValue[*] nonunique;
    attribute def PhotonIntensityUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-7 item 7-22 photon radiance */
    attribute def PhotonRadianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-22 photon radiance
         * symbol(s): `L_p`, `(L)`
         * application domain: generic
         * name: PhotonRadiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1*sr^-1
         * tensor order: 0
         * definition: density of photon intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_p = (dI_p)/(dA) 1/cos(α)`, where `I_p` is photon intensity (item 7-21), `A` is area (ISO 80000-3) and `α` the angle between the normal to the surface at the specified point and the specified direction
         * remarks: The corresponding radiometric quantity is "radiance" (item 7-6.1). The corresponding photometric quantity is "luminance" (item 7-15).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonRadianceUnit[1];
    }
    attribute def photonRadiance : PhotonRadianceValue[*] nonunique;
    attribute def PhotonRadianceUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-23 photon irradiance */
    attribute def PhotonIrradianceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-23 photon irradiance
         * symbol(s): `E_p`, `(E)`
         * application domain: generic
         * name: PhotonIrradiance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of incident photon flux with respect to area at a point on a real or imaginary surface, expressed by `E_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) on which the photon flux is incident
         * remarks: The corresponding radiometric quantity is "irradiance" (item 7-7.1). The corresponding photometric quantity is "illuminance" (item 7-16).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonIrradianceUnit[1];
    }
    attribute def photonIrradiance : PhotonIrradianceValue[*] nonunique;
    attribute def PhotonIrradianceUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-24 photon exitance */
    attribute def PhotonExitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-24 photon exitance
         * symbol(s): `M_p`, `(M)`
         * application domain: generic
         * name: PhotonExitance
         * quantity dimension: L^-2*T^-1
         * measurement unit(s): m^-2*s^-1
         * tensor order: 0
         * definition: density of exiting photon flux with respect to area at a point on a real or imaginary surface, expressed by `M_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) from which the photon flux leaves
         * remarks: The corresponding radiometric quantity is "radiant exitance" (item 7-8.1). The corresponding photometric quantity is "luminous exitance" (item 7-17).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonExitanceUnit[1];
    }
    attribute def photonExitance : PhotonExitanceValue[*] nonunique;
    attribute def PhotonExitanceUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-25 photon exposure */
    attribute def PhotonExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-25 photon exposure
         * symbol(s): `H_p`, `(H)`
         * application domain: generic
         * name: PhotonExposure
         * quantity dimension: L^-2
         * measurement unit(s): m^-2
         * tensor order: 0
         * definition: density of incident photon number with respect to area at a point on a real or imaginary surface, expressed by `H_p = (dN_p)/(dA)`, where `N_p` is photon number (item 7-19.1) and `A` is the area (ISO 80000-3) on which the photons are incident
         * remarks: The corresponding radiometric quantity is "radiant exposure" (item 7-9.1). The corresponding photometric quantity is "luminous exposure" (item 7-18).
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhotonExposureUnit[1];
    }
    attribute def photonExposure : PhotonExposureValue[*] nonunique;
    attribute def PhotonExposureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-7 item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer
         * symbol(s): `X,Y,Z`
         * application domain: generic
         * name: TristimulusValuesForTheCie1931StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1931 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[cd*m^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 // int_0^∞ S_λ(λ) overline y(λ) dλ`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TristimulusValuesForTheCie1931StandardColorimetricObserverUnit[1];
    }
    attribute def tristimulusValuesForTheCie1931StandardColorimetricObserver : TristimulusValuesForTheCie1931StandardColorimetricObserverValue[*] nonunique;
    attribute def TristimulusValuesForTheCie1931StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer */
    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer
         * symbol(s): `X_10,Y_10,Z_10`
         * application domain: generic
         * name: TristimulusValuesForTheCie1964StandardColorimetricObserver
         * quantity dimension: L^-2*J^1
         * measurement unit(s): cd*m^-2
         * tensor order: 0
         * definition: amounts of the three reference colour stimuli in the CIE 1964 standard colorimetric system, required to match the colour of the stimulus considered
         * remarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `["cd"*"m"^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 /( int_0^∞ S_λ(λ) overline y(λ) dλ)`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TristimulusValuesForTheCie1964StandardColorimetricObserverUnit[1];
    }
    attribute def tristimulusValuesForTheCie1964StandardColorimetricObserver : TristimulusValuesForTheCie1964StandardColorimetricObserverValue[*] nonunique;
    attribute def TristimulusValuesForTheCie1964StandardColorimetricObserverUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.J;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, luminousIntensityPF);
        }
    }
    /* ISO-80000-7 item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer
         * symbol(s): `overline x(λ)`, `overline y(λ)`, `overline z(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x(λ)` , `overline y(λ)` , `overline z(λ)` in the CIE 1931 standard colorimetric system
         * remarks: Values of `overline x(λ)` , `overline y(λ)` and `overline z(λ)` are defined in the CIE 1931 standard colorimetric system (2° observer) — applicable to fields of observation of angular opening from 1° to 4°.
         */
    }
    attribute def cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver : CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue;
    /* ISO-80000-7 item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer */
    attribute def CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer
         * symbol(s): `overline x_10(λ)`, `overline y_10(λ)`, `overline z_10(λ)`
         * application domain: generic
         * name: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: functions `overline x_10(λ)` , `overline y_10(λ)` , `overline z_10(λ)` in the CIE 1964 standard colorimetric system
         * remarks: Values of `overline x_10(λ)` , `overline y_10(λ)` and `overline z_10(λ)` are defined in the CIE 1964 standard colorimetric system (10° observer) — applicable to fields of observation with angles greater than 4°.
         */
    }
    attribute def cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver : CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue;
    /* ISO-80000-7 item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system
         * symbol(s): `x,y,z`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1931 standard colorimetric observer (item 7-26.1) and their sum, expressed by `x = X / (X+Y+Z)` , `y = Y / (X+Y+Z)` , `z = Z / (X+Y+Z)`
         * remarks: Since `x + y + z = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute def chromaticityCoordinatesInTheCie1931StandardColorimetricSystem : ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue;
    /* ISO-80000-7 item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system */
    attribute def ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system
         * symbol(s): `x_10,y_10,z_10`
         * application domain: generic
         * name: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystem (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1964 standard colorimetric observer (item 7-26.2) and their sum, expressed by `x_10 = X_10 / (X_10+Y_10+Z_10)`, `y_10 = Y_10 / (X_10+Y_10+Z_10)`, `z_10 = Z_10 / (X_10+Y_10+Z_10)`
         * remarks: Since `x_10 + y_10 + z_10 = 1`, two variables are sufficient to express chromaticity.
         */
    }
    attribute def chromaticityCoordinatesInTheCie1964StandardColorimetricSystem : ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue;
    /* ISO-80000-7 item 7-29.1 colour temperature */
    attribute def colourTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 7-29.1 colour temperature
         * symbol(s): `T_c`
         * application domain: generic
         * name: ColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator whose radiation has the same chromaticity as that of a given stimulus
         * remarks: None.
         */
    }
    /* ISO-80000-7 item 7-29.2 correlated colour temperature */
    attribute def correlatedColourTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 7-29.2 correlated colour temperature
         * symbol(s): `T_"cp"`
         * application domain: generic
         * name: CorrelatedColourTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature of a Planckian radiator having the chromaticity nearest the chromaticity associated with the given spectral distribution on a modified 1976 CIE Uniform Chromaticity Scale (UCS) diagram where `u',2/3 v'` are the coordinates of the Planckian locus and the test stimulus
         * remarks: None.
         */
    }
    /* ISO-80000-7 item 7-30.1 emissivity */
    attribute def EmissivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.1 emissivity
         * symbol(s): `ε`, `ε_T`
         * application domain: generic
         * name: Emissivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator and the radiant exitance of a Planckian radiator at the same temperature, expressed by `ε = M/M_b`, where `M` is the radiant exitance (item 7-8.1) of a thermal radiator and `M_b` is the radiant exitance of a Planckian radiator at the same temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute def emissivity : EmissivityValue;
    /* ISO-80000-7 item 7-30.2 emissivity at a specified wavelength */
    attribute def EmissivityAtASpecifiedWavelengthValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-30.2 emissivity at a specified wavelength
         * symbol(s): `ε(λ)`
         * application domain: generic
         * name: EmissivityAtASpecifiedWavelength (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiant exitance of a radiator at a specified wavelength and the radiant exitance of a Planckian radiator at the same temperature and at the same wavelength, expressed by `ε(λ) = M(λ) / M_b(λ)`, where `M(λ)` is the radiant exitance (item 7-8.1) of a thermal radiator at a specified wavelength and `M_b(λ)` is the radiant exitance of a Planckian radiator at the same temperature at a specified wavelength (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute def emissivityAtASpecifiedWavelength : EmissivityAtASpecifiedWavelengthValue;
    /* ISO-80000-7 item 7-31.1 absorptance */
    attribute def AbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.1 absorptance
         * symbol(s): `α`, `a`
         * application domain: generic
         * name: Absorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed radiant flux and incident radiant flux, expressed by `α = Φ_a/Φ_m`, where `Φ_a` is absorbed radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `ρ` is reflectance (item 7-31.3) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute def absorptance : AbsorptanceValue;
    /* ISO-80000-7 item 7-31.2 luminous absorptance */
    attribute def LuminousAbsorptanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.2 luminous absorptance
         * symbol(s): `α_v`
         * application domain: generic
         * name: LuminousAbsorptance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of absorbed luminous flux and incident luminous flux, expressed by `α_v = Φ_(v,a)/Φ_(v,m)`, where `Φ_(v,a)` is absorbed luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral absorptance, `α(λ)`, luminous absorptance can be calculated by `α_v = (int_0^∞ α(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.1.
         */
    }
    attribute def luminousAbsorptance : LuminousAbsorptanceValue;
    /* ISO-80000-7 item 7-31.3 reflectance */
    attribute def ReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.3 reflectance
         * symbol(s): `ρ`
         * application domain: generic
         * name: Reflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected radiant flux and incident radiant flux, expressed by `ρ = Φ_r/Φ_m`, where `Φ_r` is reflected radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `τ` is transmittance (item 7-31.5).
         */
    }
    attribute def reflectance : ReflectanceValue;
    /* ISO-80000-7 item 7-31.4 luminous reflectance */
    attribute def LuminousReflectanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.4 luminous reflectance
         * symbol(s): `ρ_v`
         * application domain: generic
         * name: LuminousReflectance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of reflected luminous flux and incident luminous flux, is expressed by `ρ_v = Φ_(v,r)/Φ_(v,m)`, where `Φ_(v,r)` is reflected luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux
         * remarks: From spectral reflectance, `ρ(λ)`, luminous reflectance can be calculated by `ρ_v = (int_0^∞ ρ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.3.
         */
    }
    attribute def luminousReflectance : LuminousReflectanceValue;
    /* ISO-80000-7 item 7-31.5 transmittance */
    attribute def TransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.5 transmittance
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: Transmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted radiant flux and incident radiant flux, expressed by `τ = Φ_t/Φ_m`, where `Φ_t` is transmitted radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `ρ` is reflectance (item 7-31.3).
         */
    }
    attribute def transmittance : TransmittanceValue;
    /* ISO-80000-7 item 7-31.6 luminous transmittance */
    attribute def LuminousTransmittanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-31.6 luminous transmittance
         * symbol(s): `τ_v`
         * application domain: generic
         * name: LuminousTransmittance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of transmitted luminous flux and incident luminous flux, expressed by `τ_v = Φ_(v,t)/Φ_(v,m)`, where `Φ_(v,t)` is transmitted luminous flux (item 7-13) and `Φ_(v,m)` is luminous flux of the incident radiation
         * remarks: From the spectral transmittance `τ(λ)`, luminous transmittance can be calculated by `τ_v = (int_0^∞ τ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is the spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is the spectral luminous efficiency (item 7-10.2). See also item 7-31.5.
         */
    }
    attribute def luminousTransmittance : LuminousTransmittanceValue;
    /* ISO-80000-7 item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance */
    attribute def TransmittanceOpticalDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance
         * symbol(s): `D`, `A_10`, `D_τ`
         * application domain: generic
         * name: TransmittanceOpticalDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: logarithm to base 10 of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the optical density can be expressed by `A_10(λ) = -log(τ(λ))`, where `τ(λ)` is the transmittance (item 7-31.5) in terms of wavelength. In spectroscopy, the name "absorbance" `A_10` is generally used.
         */
    }
    attribute def transmittanceOpticalDensity : TransmittanceOpticalDensityValue;
    alias opticalDensity for transmittanceOpticalDensity;
    alias transmittanceDensity for transmittanceOpticalDensity;
    alias decadicAbsorbance for transmittanceOpticalDensity;
    /* ISO-80000-7 item 7-32.2 Napierian absorbance */
    attribute def NapierianAbsorbanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-32.2 Napierian absorbance
         * symbol(s): `A_n`, `B`
         * application domain: generic
         * name: NapierianAbsorbance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: natural (Napierian) logarithm of the reciprocal of the transmittance, `τ` (item 7-31.5)
         * remarks: If defined in terms of wavelength, the Napierian absorbance can be expressed by `A_n(λ) = B(λ) = -log(τ(λ))`. It can also be expressed as `A_n(λ) = l*α(λ)`, where `α` is linear absorption coefficient (item 7-35.2) and `l` is length (ISO 80000-3) traversed.
         */
    }
    attribute def napierianAbsorbance : NapierianAbsorbanceValue;
    /* ISO-80000-7 item 7-33.1 radiance factor */
    attribute def RadianceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.1 radiance factor
         * symbol(s): `β_e`, `(β)`
         * application domain: generic
         * name: RadianceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the radiance of a surface element in a specified direction and the radiance of the perfect reflecting diffuser or perfect transmitting diffuser identically irradiated and viewed, expressed by `β_e = L_(e,n)/L_(e,d)`, where `L_(e,n)` is the radiance (item 7-6.1) of a surface element in a given direction and `L_(e,d)` is the radiance of the perfect reflecting or transmitting diffuser identically irradiated and viewed
         * remarks: The definition holds for a surface element of a non-self-radiating medium, in a given direction and under specified conditions of irradiation. Radiance factor is equivalent to reflectance factor (item 7-34) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is `2π ["sr"]`. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called "perfect diffuser".
         */
    }
    attribute def radianceFactor : RadianceFactorValue;
    /* ISO-80000-7 item 7-33.2 luminance factor */
    attribute def LuminanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-33.2 luminance factor
         * symbol(s): `β_v`, `(β)`
         * application domain: generic
         * name: LuminanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the luminance of a surface element in a specified direction and the luminance of the perfect reflecting diffuser or perfect transmitting diffuser identically illuminated and viewed, expressed by `β_v = L_(v,n)/L_(v,d)`, where `L_(v,n)` is the luminance (item 7-15) of a surface element in a given direction and `L_(v,d)` is the luminance of the perfect reflecting or transmitting diffuser identically illuminated and viewed
         * remarks: The definition holds for a surface element of a non-luminous medium, in a given direction and under specified conditions of irradiation. This quantity is also defined spectrally and is called "spectral luminance factor". For the analogous radiant quantity "radiance factor", see item 7-33.1.
         */
    }
    attribute def luminanceFactor : LuminanceFactorValue;
    /* ISO-80000-7 item 7-34 reflectance factor */
    attribute def ReflectanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 7-34 reflectance factor
         * symbol(s): `R`
         * application domain: generic
         * name: ReflectanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the flux reflected in the directions delimited by a given cone with apex at a surface element and the flux reflected in the same directions by a perfect reflecting diffuser identically irradiated or illuminated, expressed by `R = Φ_n/Φ_d`, where `Φ_n` is the flux reflected in the directions delimited by a given cone and `Φ_d` is the flux reflected in the same directions by an identically irradiated diffuser of reflectance (item 7-31.3) equal to 1
         * remarks: The flux can be a radiant flux (item 7‐4.1) or a luminous flux (item 7‐13). The definition holds for a surface element, for the part of the reflected radiation contained in a given cone with apex at the surface element, and for incident radiation of given spectral composition, polarization and geometric distribution. Reflectance factor is equivalent to radiance factor (item 7-33.1) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is 2π sr. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called a perfect diffuser.
         */
    }
    attribute def reflectanceFactor : ReflectanceFactorValue;
    /* ISO-80000-7 item 7-35.1 linear attenuation coefficient, linear extinction coefficient */
    attribute def LinearAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.1 linear attenuation coefficient, linear extinction coefficient
         * symbol(s): `μ`, `μ_l`
         * application domain: radiometry
         * name: LinearAttenuationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux caused by absorption and scattering
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear attenuation coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing and scattering medium `μ(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearAttenuationCoefficientUnit[1];
    }
    attribute def linearAttenuationCoefficient : LinearAttenuationCoefficientValue[*] nonunique;
    attribute def LinearAttenuationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias LinearExtinctionCoefficientUnit for LinearAttenuationCoefficientUnit;
    alias LinearExtinctionCoefficientValue for LinearAttenuationCoefficientValue;
    alias linearExtinctionCoefficient for linearAttenuationCoefficient;
    /* ISO-80000-7 item 7-35.2 linear absorption coefficient */
    attribute def LinearAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-35.2 linear absorption coefficient
         * symbol(s): `α_l`, `a_l`, `α`
         * application domain: radiometry
         * name: LinearAbsorptionCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: relative decrease in radiant flux (item 7-4.1) caused by absorption
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name. The spectral linear absorption coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing medium `α_l(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. It can also be expressed as a function of transmittance (item 7-31.5). `α_l = -ln(τ)/l = A_n/l`. The linear absorption coefficient is that part of the linear attenuation coefficient (item 7-35.1) that is due to absorption. Scattering might also contribute. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearAbsorptionCoefficientUnit[1];
    }
    attribute def linearAbsorptionCoefficient : LinearAbsorptionCoefficientValue[*] nonunique;
    attribute def LinearAbsorptionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-7 item 7-36.1 mass attenuation coefficient */
    attribute def MassAttenuationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.1 mass attenuation coefficient
         * symbol(s): `μ_m`
         * application domain: radiometry
         * name: MassAttenuationCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear attenuation coefficient (item 7-35.1), `μ`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `μ_m(λ) = (μ(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassAttenuationCoefficientUnit[1];
    }
    attribute def massAttenuationCoefficient : MassAttenuationCoefficientValue[*] nonunique;
    attribute def MassAttenuationCoefficientUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-36.2 mass absorption coefficient */
    attribute def MassAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-36.2 mass absorption coefficient
         * symbol(s): `α_m`
         * application domain: radiometry
         * name: MassAbsorptionCoefficient
         * quantity dimension: L^2*M^-1
         * measurement unit(s): kg^-1*m^2
         * tensor order: 0
         * definition: quotient of the linear absorption coefficient (item 7-35.2), `α`, and the mass density (ISO 80000-4), `ρ`, of the medium
         * remarks: This quantity is also defined spectrally in terms of wavelength, in which case, "spectral" is added before this quantity name, which can be expressed by `α_m(λ) = (α(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassAbsorptionCoefficientUnit[1];
    }
    attribute def massAbsorptionCoefficient : MassAbsorptionCoefficientValue[*] nonunique;
    attribute def MassAbsorptionCoefficientUnit :> DerivedUnit {
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
    /* ISO-80000-7 item 7-37 molar absorption coefficient */
    attribute def MolarAbsorptionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-37 molar absorption coefficient
         * symbol(s): `χ`
         * application domain: radiometry
         * name: MolarAbsorptionCoefficient
         * quantity dimension: L^2*N^-1
         * measurement unit(s): m^2*mol^-1
         * tensor order: 0
         * definition: product of linear absorption coefficient and molar volume, expressed by `χ = α V_m`, where `α` is linear absorption coefficient (item 7-35.2) and `V_m` is molar volume (ISO 80000-9)
         * remarks: The molar absorption coefficient can also be expressed by `χ = α c` where `c` is amount-of-substance concentration (ISO 80000-9). Similarly, luminous and photon quantities can be defined.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarAbsorptionCoefficientUnit[1];
    }
    attribute def molarAbsorptionCoefficient : MolarAbsorptionCoefficientValue[*] nonunique;
    attribute def MolarAbsorptionCoefficientUnit :> DerivedUnit {
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
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 785) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 785) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 799) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 824) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 824) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 858) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 858) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 903) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 903) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1001) (line 21) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 1001) (line 21) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1020) (line 21) (column 39) (len 11)))))
    (reference r5 (scope relative) (span (offset 1142) (line 24) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1142) (line 24) (column 49) (len 19)))))
    (reference r6 (scope relative) (span (offset 1820) (line 37) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1820) (line 37) (column 28) (len 4)))))
    (reference r7 (scope relative) (span (offset 1815) (line 37) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 1815) (line 37) (column 23) (len 3)))))
    (reference r8 (scope relative) (span (offset 1854) (line 38) (column 29) (len 25)) (segments (segment 0 (token "SpeedOfLightInAMediumUnit") (name "SpeedOfLightInAMediumUnit") (separator none) (span (offset 1854) (line 38) (column 29) (len 25)))))
    (reference r9 (scope relative) (span (offset 1848) (line 38) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1848) (line 38) (column 23) (len 4)))))
    (reference r10 (scope relative) (span (offset 1928) (line 41) (column 38) (len 26)) (segments (segment 0 (token "SpeedOfLightInAMediumValue") (name "SpeedOfLightInAMediumValue") (separator none) (span (offset 1928) (line 41) (column 38) (len 26)))))
    (reference r11 (scope relative) (span (offset 2037) (line 43) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 2037) (line 43) (column 48) (len 11)))))
    (reference r12 (scope relative) (span (offset 2087) (line 44) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2087) (line 44) (column 37) (len 19)))))
    (reference r13 (scope relative) (span (offset 2116) (line 44) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2116) (line 44) (column 66) (len 8)))))
    (reference r14 (scope relative) (span (offset 2127) (line 44) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2127) (line 44) (column 77) (len 3)))))
    (reference r15 (scope relative) (span (offset 2131) (line 44) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 2131) (line 44) (column 81) (len 1)))))
    (reference r16 (scope relative) (span (offset 2138) (line 44) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2138) (line 44) (column 88) (len 8)))))
    (reference r17 (scope relative) (span (offset 2192) (line 45) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2192) (line 45) (column 39) (len 19)))))
    (reference r18 (scope relative) (span (offset 2221) (line 45) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2221) (line 45) (column 68) (len 8)))))
    (reference r19 (scope relative) (span (offset 2232) (line 45) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2232) (line 45) (column 79) (len 3)))))
    (reference r20 (scope relative) (span (offset 2236) (line 45) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 2236) (line 45) (column 83) (len 1)))))
    (reference r21 (scope relative) (span (offset 2243) (line 45) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2243) (line 45) (column 90) (len 8)))))
    (reference r22 (scope relative) (span (offset 2282) (line 46) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 2282) (line 46) (column 23) (len 17)))))
    (reference r23 (scope relative) (span (offset 2306) (line 46) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 2306) (line 46) (column 47) (len 20)))))
    (reference r24 (scope relative) (span (offset 2330) (line 46) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 2330) (line 46) (column 71) (len 8)))))
    (reference r25 (scope relative) (span (offset 2340) (line 46) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 2340) (line 46) (column 81) (len 10)))))
    (reference r26 (scope relative) (span (offset 2454) (line 50) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 2454) (line 50) (column 43) (len 17)))))
    (reference r27 (scope relative) (span (offset 3418) (line 64) (column 32) (len 20)) (segments (segment 0 (token "RefractiveIndexValue") (name "RefractiveIndexValue") (separator none) (span (offset 3418) (line 64) (column 32) (len 20)))))
    (reference r28 (scope relative) (span (offset 3538) (line 67) (column 30) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 3538) (line 67) (column 30) (len 11)))))
    (reference r29 (scope relative) (span (offset 4652) (line 83) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 4652) (line 83) (column 49) (len 19)))))
    (reference r30 (scope relative) (span (offset 5407) (line 96) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 5407) (line 96) (column 28) (len 4)))))
    (reference r31 (scope relative) (span (offset 5402) (line 96) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 5402) (line 96) (column 23) (len 3)))))
    (reference r32 (scope relative) (span (offset 5441) (line 97) (column 29) (len 25)) (segments (segment 0 (token "SpectralRadiantEnergyUnit") (name "SpectralRadiantEnergyUnit") (separator none) (span (offset 5441) (line 97) (column 29) (len 25)))))
    (reference r33 (scope relative) (span (offset 5435) (line 97) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5435) (line 97) (column 23) (len 4)))))
    (reference r34 (scope relative) (span (offset 5515) (line 100) (column 38) (len 26)) (segments (segment 0 (token "SpectralRadiantEnergyValue") (name "SpectralRadiantEnergyValue") (separator none) (span (offset 5515) (line 100) (column 38) (len 26)))))
    (reference r35 (scope relative) (span (offset 5624) (line 102) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 5624) (line 102) (column 48) (len 11)))))
    (reference r36 (scope relative) (span (offset 5674) (line 103) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5674) (line 103) (column 37) (len 19)))))
    (reference r37 (scope relative) (span (offset 5703) (line 103) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5703) (line 103) (column 66) (len 8)))))
    (reference r38 (scope relative) (span (offset 5714) (line 103) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5714) (line 103) (column 77) (len 3)))))
    (reference r39 (scope relative) (span (offset 5718) (line 103) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 5718) (line 103) (column 81) (len 1)))))
    (reference r40 (scope relative) (span (offset 5725) (line 103) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5725) (line 103) (column 88) (len 8)))))
    (reference r41 (scope relative) (span (offset 5775) (line 104) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5775) (line 104) (column 35) (len 19)))))
    (reference r42 (scope relative) (span (offset 5804) (line 104) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5804) (line 104) (column 64) (len 8)))))
    (reference r43 (scope relative) (span (offset 5815) (line 104) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5815) (line 104) (column 75) (len 3)))))
    (reference r44 (scope relative) (span (offset 5819) (line 104) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 5819) (line 104) (column 79) (len 1)))))
    (reference r45 (scope relative) (span (offset 5826) (line 104) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5826) (line 104) (column 86) (len 8)))))
    (reference r46 (scope relative) (span (offset 5880) (line 105) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5880) (line 105) (column 39) (len 19)))))
    (reference r47 (scope relative) (span (offset 5909) (line 105) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5909) (line 105) (column 68) (len 8)))))
    (reference r48 (scope relative) (span (offset 5920) (line 105) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5920) (line 105) (column 79) (len 3)))))
    (reference r49 (scope relative) (span (offset 5924) (line 105) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 5924) (line 105) (column 83) (len 1)))))
    (reference r50 (scope relative) (span (offset 5931) (line 105) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5931) (line 105) (column 90) (len 8)))))
    (reference r51 (scope relative) (span (offset 5970) (line 106) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 5970) (line 106) (column 23) (len 17)))))
    (reference r52 (scope relative) (span (offset 5994) (line 106) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 5994) (line 106) (column 47) (len 20)))))
    (reference r53 (scope relative) (span (offset 6018) (line 106) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 6018) (line 106) (column 71) (len 8)))))
    (reference r54 (scope relative) (span (offset 6028) (line 106) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 6028) (line 106) (column 81) (len 6)))))
    (reference r55 (scope relative) (span (offset 6036) (line 106) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 6036) (line 106) (column 89) (len 10)))))
    (reference r56 (scope relative) (span (offset 6161) (line 110) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 6161) (line 110) (column 48) (len 19)))))
    (reference r57 (scope relative) (span (offset 7029) (line 123) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 7029) (line 123) (column 28) (len 4)))))
    (reference r58 (scope relative) (span (offset 7024) (line 123) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 7024) (line 123) (column 23) (len 3)))))
    (reference r59 (scope relative) (span (offset 7063) (line 124) (column 29) (len 24)) (segments (segment 0 (token "RadiantEnergyDensityUnit") (name "RadiantEnergyDensityUnit") (separator none) (span (offset 7063) (line 124) (column 29) (len 24)))))
    (reference r60 (scope relative) (span (offset 7057) (line 124) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7057) (line 124) (column 23) (len 4)))))
    (reference r61 (scope relative) (span (offset 7135) (line 127) (column 37) (len 25)) (segments (segment 0 (token "RadiantEnergyDensityValue") (name "RadiantEnergyDensityValue") (separator none) (span (offset 7135) (line 127) (column 37) (len 25)))))
    (reference r62 (scope relative) (span (offset 7242) (line 129) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 7242) (line 129) (column 47) (len 11)))))
    (reference r63 (scope relative) (span (offset 7292) (line 130) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7292) (line 130) (column 37) (len 19)))))
    (reference r64 (scope relative) (span (offset 7321) (line 130) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7321) (line 130) (column 66) (len 8)))))
    (reference r65 (scope relative) (span (offset 7332) (line 130) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7332) (line 130) (column 77) (len 3)))))
    (reference r66 (scope relative) (span (offset 7336) (line 130) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 7336) (line 130) (column 81) (len 1)))))
    (reference r67 (scope relative) (span (offset 7343) (line 130) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7343) (line 130) (column 88) (len 8)))))
    (reference r68 (scope relative) (span (offset 7394) (line 131) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7394) (line 131) (column 35) (len 19)))))
    (reference r69 (scope relative) (span (offset 7423) (line 131) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7423) (line 131) (column 64) (len 8)))))
    (reference r70 (scope relative) (span (offset 7434) (line 131) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7434) (line 131) (column 75) (len 3)))))
    (reference r71 (scope relative) (span (offset 7438) (line 131) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 7438) (line 131) (column 79) (len 1)))))
    (reference r72 (scope relative) (span (offset 7445) (line 131) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7445) (line 131) (column 86) (len 8)))))
    (reference r73 (scope relative) (span (offset 7499) (line 132) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7499) (line 132) (column 39) (len 19)))))
    (reference r74 (scope relative) (span (offset 7528) (line 132) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7528) (line 132) (column 68) (len 8)))))
    (reference r75 (scope relative) (span (offset 7539) (line 132) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7539) (line 132) (column 79) (len 3)))))
    (reference r76 (scope relative) (span (offset 7543) (line 132) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 7543) (line 132) (column 83) (len 1)))))
    (reference r77 (scope relative) (span (offset 7550) (line 132) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7550) (line 132) (column 90) (len 8)))))
    (reference r78 (scope relative) (span (offset 7589) (line 133) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 7589) (line 133) (column 23) (len 17)))))
    (reference r79 (scope relative) (span (offset 7613) (line 133) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 7613) (line 133) (column 47) (len 20)))))
    (reference r80 (scope relative) (span (offset 7637) (line 133) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 7637) (line 133) (column 71) (len 8)))))
    (reference r81 (scope relative) (span (offset 7647) (line 133) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 7647) (line 133) (column 81) (len 6)))))
    (reference r82 (scope relative) (span (offset 7655) (line 133) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 7655) (line 133) (column 89) (len 10)))))
    (reference r83 (scope relative) (span (offset 7839) (line 137) (column 75) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 7839) (line 137) (column 75) (len 19)))))
    (reference r84 (scope relative) (span (offset 8851) (line 150) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 8851) (line 150) (column 28) (len 4)))))
    (reference r85 (scope relative) (span (offset 8846) (line 150) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 8846) (line 150) (column 23) (len 3)))))
    (reference r86 (scope relative) (span (offset 8885) (line 151) (column 29) (len 51)) (segments (segment 0 (token "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (separator none) (span (offset 8885) (line 151) (column 29) (len 51)))))
    (reference r87 (scope relative) (span (offset 8879) (line 151) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 8879) (line 151) (column 23) (len 4)))))
    (reference r88 (scope relative) (span (offset 9011) (line 154) (column 64) (len 52)) (segments (segment 0 (token "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (separator none) (span (offset 9011) (line 154) (column 64) (len 52)))))
    (reference r89 (scope relative) (span (offset 9172) (line 156) (column 74) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 9172) (line 156) (column 74) (len 11)))))
    (reference r90 (scope relative) (span (offset 9222) (line 157) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9222) (line 157) (column 37) (len 19)))))
    (reference r91 (scope relative) (span (offset 9251) (line 157) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9251) (line 157) (column 66) (len 8)))))
    (reference r92 (scope relative) (span (offset 9262) (line 157) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9262) (line 157) (column 77) (len 3)))))
    (reference r93 (scope relative) (span (offset 9266) (line 157) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 9266) (line 157) (column 81) (len 1)))))
    (reference r94 (scope relative) (span (offset 9273) (line 157) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9273) (line 157) (column 88) (len 8)))))
    (reference r95 (scope relative) (span (offset 9324) (line 158) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9324) (line 158) (column 35) (len 19)))))
    (reference r96 (scope relative) (span (offset 9353) (line 158) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9353) (line 158) (column 64) (len 8)))))
    (reference r97 (scope relative) (span (offset 9364) (line 158) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9364) (line 158) (column 75) (len 3)))))
    (reference r98 (scope relative) (span (offset 9368) (line 158) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 9368) (line 158) (column 79) (len 1)))))
    (reference r99 (scope relative) (span (offset 9375) (line 158) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9375) (line 158) (column 86) (len 8)))))
    (reference r100 (scope relative) (span (offset 9429) (line 159) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9429) (line 159) (column 39) (len 19)))))
    (reference r101 (scope relative) (span (offset 9458) (line 159) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9458) (line 159) (column 68) (len 8)))))
    (reference r102 (scope relative) (span (offset 9469) (line 159) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9469) (line 159) (column 79) (len 3)))))
    (reference r103 (scope relative) (span (offset 9473) (line 159) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 9473) (line 159) (column 83) (len 1)))))
    (reference r104 (scope relative) (span (offset 9480) (line 159) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9480) (line 159) (column 90) (len 8)))))
    (reference r105 (scope relative) (span (offset 9519) (line 160) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 9519) (line 160) (column 23) (len 17)))))
    (reference r106 (scope relative) (span (offset 9543) (line 160) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 9543) (line 160) (column 47) (len 20)))))
    (reference r107 (scope relative) (span (offset 9567) (line 160) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 9567) (line 160) (column 71) (len 8)))))
    (reference r108 (scope relative) (span (offset 9577) (line 160) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 9577) (line 160) (column 81) (len 6)))))
    (reference r109 (scope relative) (span (offset 9585) (line 160) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 9585) (line 160) (column 89) (len 10)))))
    (reference r110 (scope relative) (span (offset 9769) (line 164) (column 75) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 9769) (line 164) (column 75) (len 19)))))
    (reference r111 (scope relative) (span (offset 10428) (line 177) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10428) (line 177) (column 28) (len 4)))))
    (reference r112 (scope relative) (span (offset 10423) (line 177) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 10423) (line 177) (column 23) (len 3)))))
    (reference r113 (scope relative) (span (offset 10462) (line 178) (column 29) (len 51)) (segments (segment 0 (token "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (separator none) (span (offset 10462) (line 178) (column 29) (len 51)))))
    (reference r114 (scope relative) (span (offset 10456) (line 178) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 10456) (line 178) (column 23) (len 4)))))
    (reference r115 (scope relative) (span (offset 10588) (line 181) (column 64) (len 52)) (segments (segment 0 (token "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (separator none) (span (offset 10588) (line 181) (column 64) (len 52)))))
    (reference r116 (scope relative) (span (offset 10749) (line 183) (column 74) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 10749) (line 183) (column 74) (len 11)))))
    (reference r117 (scope relative) (span (offset 10797) (line 184) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10797) (line 184) (column 35) (len 19)))))
    (reference r118 (scope relative) (span (offset 10826) (line 184) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10826) (line 184) (column 64) (len 8)))))
    (reference r119 (scope relative) (span (offset 10837) (line 184) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10837) (line 184) (column 75) (len 3)))))
    (reference r120 (scope relative) (span (offset 10841) (line 184) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 10841) (line 184) (column 79) (len 1)))))
    (reference r121 (scope relative) (span (offset 10848) (line 184) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10848) (line 184) (column 86) (len 8)))))
    (reference r122 (scope relative) (span (offset 10902) (line 185) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10902) (line 185) (column 39) (len 19)))))
    (reference r123 (scope relative) (span (offset 10931) (line 185) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10931) (line 185) (column 68) (len 8)))))
    (reference r124 (scope relative) (span (offset 10942) (line 185) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10942) (line 185) (column 79) (len 3)))))
    (reference r125 (scope relative) (span (offset 10946) (line 185) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 10946) (line 185) (column 83) (len 1)))))
    (reference r126 (scope relative) (span (offset 10953) (line 185) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10953) (line 185) (column 90) (len 8)))))
    (reference r127 (scope relative) (span (offset 10992) (line 186) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 10992) (line 186) (column 23) (len 17)))))
    (reference r128 (scope relative) (span (offset 11016) (line 186) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 11016) (line 186) (column 47) (len 20)))))
    (reference r129 (scope relative) (span (offset 11040) (line 186) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 11040) (line 186) (column 71) (len 6)))))
    (reference r130 (scope relative) (span (offset 11048) (line 186) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 11048) (line 186) (column 79) (len 10)))))
    (reference r131 (scope relative) (span (offset 11169) (line 190) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 11169) (line 190) (column 39) (len 19)))))
    (reference r132 (scope relative) (span (offset 11908) (line 203) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 11908) (line 203) (column 28) (len 4)))))
    (reference r133 (scope relative) (span (offset 11903) (line 203) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 11903) (line 203) (column 23) (len 3)))))
    (reference r134 (scope relative) (span (offset 11942) (line 204) (column 29) (len 15)) (segments (segment 0 (token "RadiantFluxUnit") (name "RadiantFluxUnit") (separator none) (span (offset 11942) (line 204) (column 29) (len 15)))))
    (reference r135 (scope relative) (span (offset 11936) (line 204) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 11936) (line 204) (column 23) (len 4)))))
    (reference r136 (scope relative) (span (offset 11996) (line 207) (column 28) (len 16)) (segments (segment 0 (token "RadiantFluxValue") (name "RadiantFluxValue") (separator none) (span (offset 11996) (line 207) (column 28) (len 16)))))
    (reference r137 (scope relative) (span (offset 12085) (line 209) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 12085) (line 209) (column 38) (len 11)))))
    (reference r138 (scope relative) (span (offset 12135) (line 210) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12135) (line 210) (column 37) (len 19)))))
    (reference r139 (scope relative) (span (offset 12164) (line 210) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12164) (line 210) (column 66) (len 8)))))
    (reference r140 (scope relative) (span (offset 12175) (line 210) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12175) (line 210) (column 77) (len 3)))))
    (reference r141 (scope relative) (span (offset 12179) (line 210) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 12179) (line 210) (column 81) (len 1)))))
    (reference r142 (scope relative) (span (offset 12186) (line 210) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12186) (line 210) (column 88) (len 8)))))
    (reference r143 (scope relative) (span (offset 12236) (line 211) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12236) (line 211) (column 35) (len 19)))))
    (reference r144 (scope relative) (span (offset 12265) (line 211) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12265) (line 211) (column 64) (len 8)))))
    (reference r145 (scope relative) (span (offset 12276) (line 211) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12276) (line 211) (column 75) (len 3)))))
    (reference r146 (scope relative) (span (offset 12280) (line 211) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 12280) (line 211) (column 79) (len 1)))))
    (reference r147 (scope relative) (span (offset 12287) (line 211) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12287) (line 211) (column 86) (len 8)))))
    (reference r148 (scope relative) (span (offset 12341) (line 212) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12341) (line 212) (column 39) (len 19)))))
    (reference r149 (scope relative) (span (offset 12370) (line 212) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12370) (line 212) (column 68) (len 8)))))
    (reference r150 (scope relative) (span (offset 12381) (line 212) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12381) (line 212) (column 79) (len 3)))))
    (reference r151 (scope relative) (span (offset 12385) (line 212) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 12385) (line 212) (column 83) (len 1)))))
    (reference r152 (scope relative) (span (offset 12392) (line 212) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12392) (line 212) (column 90) (len 8)))))
    (reference r153 (scope relative) (span (offset 12431) (line 213) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 12431) (line 213) (column 23) (len 17)))))
    (reference r154 (scope relative) (span (offset 12455) (line 213) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 12455) (line 213) (column 47) (len 20)))))
    (reference r155 (scope relative) (span (offset 12479) (line 213) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 12479) (line 213) (column 71) (len 8)))))
    (reference r156 (scope relative) (span (offset 12489) (line 213) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 12489) (line 213) (column 81) (len 6)))))
    (reference r157 (scope relative) (span (offset 12497) (line 213) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 12497) (line 213) (column 89) (len 10)))))
    (reference r158 (scope relative) (span (offset 12550) (line 216) (column 32) (len 15)) (segments (segment 0 (token "RadiantFluxUnit") (name "RadiantFluxUnit") (separator none) (span (offset 12550) (line 216) (column 32) (len 15)))))
    (reference r159 (scope relative) (span (offset 12599) (line 217) (column 33) (len 16)) (segments (segment 0 (token "RadiantFluxValue") (name "RadiantFluxValue") (separator none) (span (offset 12599) (line 217) (column 33) (len 16)))))
    (reference r160 (scope relative) (span (offset 12644) (line 218) (column 28) (len 11)) (segments (segment 0 (token "radiantFlux") (name "radiantFlux") (separator none) (span (offset 12644) (line 218) (column 28) (len 11)))))
    (reference r161 (scope relative) (span (offset 12783) (line 221) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 12783) (line 221) (column 47) (len 19)))))
    (reference r162 (scope relative) (span (offset 13564) (line 234) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 13564) (line 234) (column 28) (len 4)))))
    (reference r163 (scope relative) (span (offset 13559) (line 234) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 13559) (line 234) (column 23) (len 3)))))
    (reference r164 (scope relative) (span (offset 13598) (line 235) (column 29) (len 23)) (segments (segment 0 (token "SpectralRadiantFluxUnit") (name "SpectralRadiantFluxUnit") (separator none) (span (offset 13598) (line 235) (column 29) (len 23)))))
    (reference r165 (scope relative) (span (offset 13592) (line 235) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13592) (line 235) (column 23) (len 4)))))
    (reference r166 (scope relative) (span (offset 13668) (line 238) (column 36) (len 24)) (segments (segment 0 (token "SpectralRadiantFluxValue") (name "SpectralRadiantFluxValue") (separator none) (span (offset 13668) (line 238) (column 36) (len 24)))))
    (reference r167 (scope relative) (span (offset 13773) (line 240) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 13773) (line 240) (column 46) (len 11)))))
    (reference r168 (scope relative) (span (offset 13823) (line 241) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13823) (line 241) (column 37) (len 19)))))
    (reference r169 (scope relative) (span (offset 13852) (line 241) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13852) (line 241) (column 66) (len 8)))))
    (reference r170 (scope relative) (span (offset 13863) (line 241) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13863) (line 241) (column 77) (len 3)))))
    (reference r171 (scope relative) (span (offset 13867) (line 241) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 13867) (line 241) (column 81) (len 1)))))
    (reference r172 (scope relative) (span (offset 13874) (line 241) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13874) (line 241) (column 88) (len 8)))))
    (reference r173 (scope relative) (span (offset 13924) (line 242) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13924) (line 242) (column 35) (len 19)))))
    (reference r174 (scope relative) (span (offset 13953) (line 242) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13953) (line 242) (column 64) (len 8)))))
    (reference r175 (scope relative) (span (offset 13964) (line 242) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13964) (line 242) (column 75) (len 3)))))
    (reference r176 (scope relative) (span (offset 13968) (line 242) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 13968) (line 242) (column 79) (len 1)))))
    (reference r177 (scope relative) (span (offset 13975) (line 242) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13975) (line 242) (column 86) (len 8)))))
    (reference r178 (scope relative) (span (offset 14029) (line 243) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14029) (line 243) (column 39) (len 19)))))
    (reference r179 (scope relative) (span (offset 14058) (line 243) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14058) (line 243) (column 68) (len 8)))))
    (reference r180 (scope relative) (span (offset 14069) (line 243) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14069) (line 243) (column 79) (len 3)))))
    (reference r181 (scope relative) (span (offset 14073) (line 243) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 14073) (line 243) (column 83) (len 1)))))
    (reference r182 (scope relative) (span (offset 14080) (line 243) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14080) (line 243) (column 90) (len 8)))))
    (reference r183 (scope relative) (span (offset 14119) (line 244) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 14119) (line 244) (column 23) (len 17)))))
    (reference r184 (scope relative) (span (offset 14143) (line 244) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 14143) (line 244) (column 47) (len 20)))))
    (reference r185 (scope relative) (span (offset 14167) (line 244) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 14167) (line 244) (column 71) (len 8)))))
    (reference r186 (scope relative) (span (offset 14177) (line 244) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 14177) (line 244) (column 81) (len 6)))))
    (reference r187 (scope relative) (span (offset 14185) (line 244) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 14185) (line 244) (column 89) (len 10)))))
    (reference r188 (scope relative) (span (offset 14246) (line 247) (column 40) (len 23)) (segments (segment 0 (token "SpectralRadiantFluxUnit") (name "SpectralRadiantFluxUnit") (separator none) (span (offset 14246) (line 247) (column 40) (len 23)))))
    (reference r189 (scope relative) (span (offset 14311) (line 248) (column 41) (len 24)) (segments (segment 0 (token "SpectralRadiantFluxValue") (name "SpectralRadiantFluxValue") (separator none) (span (offset 14311) (line 248) (column 41) (len 24)))))
    (reference r190 (scope relative) (span (offset 14372) (line 249) (column 36) (len 19)) (segments (segment 0 (token "spectralRadiantFlux") (name "spectralRadiantFlux") (separator none) (span (offset 14372) (line 249) (column 36) (len 19)))))
    (reference r191 (scope relative) (span (offset 14488) (line 252) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 14488) (line 252) (column 44) (len 19)))))
    (reference r192 (scope relative) (span (offset 15658) (line 265) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 15658) (line 265) (column 28) (len 4)))))
    (reference r193 (scope relative) (span (offset 15653) (line 265) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 15653) (line 265) (column 23) (len 3)))))
    (reference r194 (scope relative) (span (offset 15692) (line 266) (column 29) (len 20)) (segments (segment 0 (token "RadiantIntensityUnit") (name "RadiantIntensityUnit") (separator none) (span (offset 15692) (line 266) (column 29) (len 20)))))
    (reference r195 (scope relative) (span (offset 15686) (line 266) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15686) (line 266) (column 23) (len 4)))))
    (reference r196 (scope relative) (span (offset 15756) (line 269) (column 33) (len 21)) (segments (segment 0 (token "RadiantIntensityValue") (name "RadiantIntensityValue") (separator none) (span (offset 15756) (line 269) (column 33) (len 21)))))
    (reference r197 (scope relative) (span (offset 15855) (line 271) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 15855) (line 271) (column 43) (len 11)))))
    (reference r198 (scope relative) (span (offset 15905) (line 272) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15905) (line 272) (column 37) (len 19)))))
    (reference r199 (scope relative) (span (offset 15934) (line 272) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15934) (line 272) (column 66) (len 8)))))
    (reference r200 (scope relative) (span (offset 15945) (line 272) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15945) (line 272) (column 77) (len 3)))))
    (reference r201 (scope relative) (span (offset 15949) (line 272) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 15949) (line 272) (column 81) (len 1)))))
    (reference r202 (scope relative) (span (offset 15956) (line 272) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15956) (line 272) (column 88) (len 8)))))
    (reference r203 (scope relative) (span (offset 16006) (line 273) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16006) (line 273) (column 35) (len 19)))))
    (reference r204 (scope relative) (span (offset 16035) (line 273) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16035) (line 273) (column 64) (len 8)))))
    (reference r205 (scope relative) (span (offset 16046) (line 273) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16046) (line 273) (column 75) (len 3)))))
    (reference r206 (scope relative) (span (offset 16050) (line 273) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 16050) (line 273) (column 79) (len 1)))))
    (reference r207 (scope relative) (span (offset 16057) (line 273) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16057) (line 273) (column 86) (len 8)))))
    (reference r208 (scope relative) (span (offset 16111) (line 274) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16111) (line 274) (column 39) (len 19)))))
    (reference r209 (scope relative) (span (offset 16140) (line 274) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16140) (line 274) (column 68) (len 8)))))
    (reference r210 (scope relative) (span (offset 16151) (line 274) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16151) (line 274) (column 79) (len 3)))))
    (reference r211 (scope relative) (span (offset 16155) (line 274) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 16155) (line 274) (column 83) (len 1)))))
    (reference r212 (scope relative) (span (offset 16162) (line 274) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16162) (line 274) (column 90) (len 8)))))
    (reference r213 (scope relative) (span (offset 16201) (line 275) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 16201) (line 275) (column 23) (len 17)))))
    (reference r214 (scope relative) (span (offset 16225) (line 275) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 16225) (line 275) (column 47) (len 20)))))
    (reference r215 (scope relative) (span (offset 16249) (line 275) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 16249) (line 275) (column 71) (len 8)))))
    (reference r216 (scope relative) (span (offset 16259) (line 275) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 16259) (line 275) (column 81) (len 6)))))
    (reference r217 (scope relative) (span (offset 16267) (line 275) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 16267) (line 275) (column 89) (len 10)))))
    (reference r218 (scope relative) (span (offset 16400) (line 279) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 16400) (line 279) (column 52) (len 19)))))
    (reference r219 (scope relative) (span (offset 17167) (line 292) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 17167) (line 292) (column 28) (len 4)))))
    (reference r220 (scope relative) (span (offset 17162) (line 292) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17162) (line 292) (column 23) (len 3)))))
    (reference r221 (scope relative) (span (offset 17201) (line 293) (column 29) (len 28)) (segments (segment 0 (token "SpectralRadiantIntensityUnit") (name "SpectralRadiantIntensityUnit") (separator none) (span (offset 17201) (line 293) (column 29) (len 28)))))
    (reference r222 (scope relative) (span (offset 17195) (line 293) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17195) (line 293) (column 23) (len 4)))))
    (reference r223 (scope relative) (span (offset 17281) (line 296) (column 41) (len 29)) (segments (segment 0 (token "SpectralRadiantIntensityValue") (name "SpectralRadiantIntensityValue") (separator none) (span (offset 17281) (line 296) (column 41) (len 29)))))
    (reference r224 (scope relative) (span (offset 17396) (line 298) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 17396) (line 298) (column 51) (len 11)))))
    (reference r225 (scope relative) (span (offset 17446) (line 299) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17446) (line 299) (column 37) (len 19)))))
    (reference r226 (scope relative) (span (offset 17475) (line 299) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17475) (line 299) (column 66) (len 8)))))
    (reference r227 (scope relative) (span (offset 17486) (line 299) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17486) (line 299) (column 77) (len 3)))))
    (reference r228 (scope relative) (span (offset 17490) (line 299) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 17490) (line 299) (column 81) (len 1)))))
    (reference r229 (scope relative) (span (offset 17497) (line 299) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17497) (line 299) (column 88) (len 8)))))
    (reference r230 (scope relative) (span (offset 17547) (line 300) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17547) (line 300) (column 35) (len 19)))))
    (reference r231 (scope relative) (span (offset 17576) (line 300) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17576) (line 300) (column 64) (len 8)))))
    (reference r232 (scope relative) (span (offset 17587) (line 300) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17587) (line 300) (column 75) (len 3)))))
    (reference r233 (scope relative) (span (offset 17591) (line 300) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 17591) (line 300) (column 79) (len 1)))))
    (reference r234 (scope relative) (span (offset 17598) (line 300) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17598) (line 300) (column 86) (len 8)))))
    (reference r235 (scope relative) (span (offset 17652) (line 301) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17652) (line 301) (column 39) (len 19)))))
    (reference r236 (scope relative) (span (offset 17681) (line 301) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17681) (line 301) (column 68) (len 8)))))
    (reference r237 (scope relative) (span (offset 17692) (line 301) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17692) (line 301) (column 79) (len 3)))))
    (reference r238 (scope relative) (span (offset 17696) (line 301) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 17696) (line 301) (column 83) (len 1)))))
    (reference r239 (scope relative) (span (offset 17703) (line 301) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17703) (line 301) (column 90) (len 8)))))
    (reference r240 (scope relative) (span (offset 17742) (line 302) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 17742) (line 302) (column 23) (len 17)))))
    (reference r241 (scope relative) (span (offset 17766) (line 302) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 17766) (line 302) (column 47) (len 20)))))
    (reference r242 (scope relative) (span (offset 17790) (line 302) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 17790) (line 302) (column 71) (len 8)))))
    (reference r243 (scope relative) (span (offset 17800) (line 302) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 17800) (line 302) (column 81) (len 6)))))
    (reference r244 (scope relative) (span (offset 17808) (line 302) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 17808) (line 302) (column 89) (len 10)))))
    (reference r245 (scope relative) (span (offset 17907) (line 306) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 17907) (line 306) (column 36) (len 19)))))
    (reference r246 (scope relative) (span (offset 18971) (line 319) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 18971) (line 319) (column 28) (len 4)))))
    (reference r247 (scope relative) (span (offset 18966) (line 319) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 18966) (line 319) (column 23) (len 3)))))
    (reference r248 (scope relative) (span (offset 19005) (line 320) (column 29) (len 12)) (segments (segment 0 (token "RadianceUnit") (name "RadianceUnit") (separator none) (span (offset 19005) (line 320) (column 29) (len 12)))))
    (reference r249 (scope relative) (span (offset 18999) (line 320) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 18999) (line 320) (column 23) (len 4)))))
    (reference r250 (scope relative) (span (offset 19053) (line 323) (column 25) (len 13)) (segments (segment 0 (token "RadianceValue") (name "RadianceValue") (separator none) (span (offset 19053) (line 323) (column 25) (len 13)))))
    (reference r251 (scope relative) (span (offset 19136) (line 325) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19136) (line 325) (column 35) (len 11)))))
    (reference r252 (scope relative) (span (offset 19184) (line 326) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19184) (line 326) (column 35) (len 19)))))
    (reference r253 (scope relative) (span (offset 19213) (line 326) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19213) (line 326) (column 64) (len 8)))))
    (reference r254 (scope relative) (span (offset 19224) (line 326) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19224) (line 326) (column 75) (len 3)))))
    (reference r255 (scope relative) (span (offset 19228) (line 326) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 19228) (line 326) (column 79) (len 1)))))
    (reference r256 (scope relative) (span (offset 19235) (line 326) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19235) (line 326) (column 86) (len 8)))))
    (reference r257 (scope relative) (span (offset 19289) (line 327) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19289) (line 327) (column 39) (len 19)))))
    (reference r258 (scope relative) (span (offset 19318) (line 327) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19318) (line 327) (column 68) (len 8)))))
    (reference r259 (scope relative) (span (offset 19329) (line 327) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19329) (line 327) (column 79) (len 3)))))
    (reference r260 (scope relative) (span (offset 19333) (line 327) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 19333) (line 327) (column 83) (len 1)))))
    (reference r261 (scope relative) (span (offset 19340) (line 327) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19340) (line 327) (column 90) (len 8)))))
    (reference r262 (scope relative) (span (offset 19379) (line 328) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 19379) (line 328) (column 23) (len 17)))))
    (reference r263 (scope relative) (span (offset 19403) (line 328) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 19403) (line 328) (column 47) (len 20)))))
    (reference r264 (scope relative) (span (offset 19427) (line 328) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 19427) (line 328) (column 71) (len 6)))))
    (reference r265 (scope relative) (span (offset 19435) (line 328) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 19435) (line 328) (column 79) (len 10)))))
    (reference r266 (scope relative) (span (offset 19551) (line 332) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 19551) (line 332) (column 44) (len 19)))))
    (reference r267 (scope relative) (span (offset 20768) (line 345) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 20768) (line 345) (column 28) (len 4)))))
    (reference r268 (scope relative) (span (offset 20763) (line 345) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 20763) (line 345) (column 23) (len 3)))))
    (reference r269 (scope relative) (span (offset 20802) (line 346) (column 29) (len 20)) (segments (segment 0 (token "SpectralRadianceUnit") (name "SpectralRadianceUnit") (separator none) (span (offset 20802) (line 346) (column 29) (len 20)))))
    (reference r270 (scope relative) (span (offset 20796) (line 346) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 20796) (line 346) (column 23) (len 4)))))
    (reference r271 (scope relative) (span (offset 20866) (line 349) (column 33) (len 21)) (segments (segment 0 (token "SpectralRadianceValue") (name "SpectralRadianceValue") (separator none) (span (offset 20866) (line 349) (column 33) (len 21)))))
    (reference r272 (scope relative) (span (offset 20965) (line 351) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 20965) (line 351) (column 43) (len 11)))))
    (reference r273 (scope relative) (span (offset 21015) (line 352) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21015) (line 352) (column 37) (len 19)))))
    (reference r274 (scope relative) (span (offset 21044) (line 352) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21044) (line 352) (column 66) (len 8)))))
    (reference r275 (scope relative) (span (offset 21055) (line 352) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21055) (line 352) (column 77) (len 3)))))
    (reference r276 (scope relative) (span (offset 21059) (line 352) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 21059) (line 352) (column 81) (len 1)))))
    (reference r277 (scope relative) (span (offset 21066) (line 352) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21066) (line 352) (column 88) (len 8)))))
    (reference r278 (scope relative) (span (offset 21117) (line 353) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21117) (line 353) (column 35) (len 19)))))
    (reference r279 (scope relative) (span (offset 21146) (line 353) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21146) (line 353) (column 64) (len 8)))))
    (reference r280 (scope relative) (span (offset 21157) (line 353) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21157) (line 353) (column 75) (len 3)))))
    (reference r281 (scope relative) (span (offset 21161) (line 353) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 21161) (line 353) (column 79) (len 1)))))
    (reference r282 (scope relative) (span (offset 21168) (line 353) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21168) (line 353) (column 86) (len 8)))))
    (reference r283 (scope relative) (span (offset 21222) (line 354) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21222) (line 354) (column 39) (len 19)))))
    (reference r284 (scope relative) (span (offset 21251) (line 354) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21251) (line 354) (column 68) (len 8)))))
    (reference r285 (scope relative) (span (offset 21262) (line 354) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21262) (line 354) (column 79) (len 3)))))
    (reference r286 (scope relative) (span (offset 21266) (line 354) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 21266) (line 354) (column 83) (len 1)))))
    (reference r287 (scope relative) (span (offset 21273) (line 354) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21273) (line 354) (column 90) (len 8)))))
    (reference r288 (scope relative) (span (offset 21312) (line 355) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 21312) (line 355) (column 23) (len 17)))))
    (reference r289 (scope relative) (span (offset 21336) (line 355) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 21336) (line 355) (column 47) (len 20)))))
    (reference r290 (scope relative) (span (offset 21360) (line 355) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 21360) (line 355) (column 71) (len 8)))))
    (reference r291 (scope relative) (span (offset 21370) (line 355) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 21370) (line 355) (column 81) (len 6)))))
    (reference r292 (scope relative) (span (offset 21378) (line 355) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 21378) (line 355) (column 89) (len 10)))))
    (reference r293 (scope relative) (span (offset 21481) (line 359) (column 38) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21481) (line 359) (column 38) (len 19)))))
    (reference r294 (scope relative) (span (offset 23034) (line 372) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23034) (line 372) (column 28) (len 4)))))
    (reference r295 (scope relative) (span (offset 23029) (line 372) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23029) (line 372) (column 23) (len 3)))))
    (reference r296 (scope relative) (span (offset 23068) (line 373) (column 29) (len 14)) (segments (segment 0 (token "IrradianceUnit") (name "IrradianceUnit") (separator none) (span (offset 23068) (line 373) (column 29) (len 14)))))
    (reference r297 (scope relative) (span (offset 23062) (line 373) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23062) (line 373) (column 23) (len 4)))))
    (reference r298 (scope relative) (span (offset 23120) (line 376) (column 27) (len 15)) (segments (segment 0 (token "IrradianceValue") (name "IrradianceValue") (separator none) (span (offset 23120) (line 376) (column 27) (len 15)))))
    (reference r299 (scope relative) (span (offset 23207) (line 378) (column 37) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 23207) (line 378) (column 37) (len 11)))))
    (reference r300 (scope relative) (span (offset 23255) (line 379) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 23255) (line 379) (column 35) (len 19)))))
    (reference r301 (scope relative) (span (offset 23284) (line 379) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23284) (line 379) (column 64) (len 8)))))
    (reference r302 (scope relative) (span (offset 23295) (line 379) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 23295) (line 379) (column 75) (len 3)))))
    (reference r303 (scope relative) (span (offset 23299) (line 379) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 23299) (line 379) (column 79) (len 1)))))
    (reference r304 (scope relative) (span (offset 23306) (line 379) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 23306) (line 379) (column 86) (len 8)))))
    (reference r305 (scope relative) (span (offset 23360) (line 380) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 23360) (line 380) (column 39) (len 19)))))
    (reference r306 (scope relative) (span (offset 23389) (line 380) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23389) (line 380) (column 68) (len 8)))))
    (reference r307 (scope relative) (span (offset 23400) (line 380) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 23400) (line 380) (column 79) (len 3)))))
    (reference r308 (scope relative) (span (offset 23404) (line 380) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 23404) (line 380) (column 83) (len 1)))))
    (reference r309 (scope relative) (span (offset 23411) (line 380) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 23411) (line 380) (column 90) (len 8)))))
    (reference r310 (scope relative) (span (offset 23450) (line 381) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 23450) (line 381) (column 23) (len 17)))))
    (reference r311 (scope relative) (span (offset 23474) (line 381) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 23474) (line 381) (column 47) (len 20)))))
    (reference r312 (scope relative) (span (offset 23498) (line 381) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 23498) (line 381) (column 71) (len 6)))))
    (reference r313 (scope relative) (span (offset 23506) (line 381) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 23506) (line 381) (column 79) (len 10)))))
    (reference r314 (scope relative) (span (offset 23626) (line 385) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 23626) (line 385) (column 46) (len 19)))))
    (reference r315 (scope relative) (span (offset 24376) (line 398) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 24376) (line 398) (column 28) (len 4)))))
    (reference r316 (scope relative) (span (offset 24371) (line 398) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 24371) (line 398) (column 23) (len 3)))))
    (reference r317 (scope relative) (span (offset 24410) (line 399) (column 29) (len 22)) (segments (segment 0 (token "SpectralIrradianceUnit") (name "SpectralIrradianceUnit") (separator none) (span (offset 24410) (line 399) (column 29) (len 22)))))
    (reference r318 (scope relative) (span (offset 24404) (line 399) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 24404) (line 399) (column 23) (len 4)))))
    (reference r319 (scope relative) (span (offset 24478) (line 402) (column 35) (len 23)) (segments (segment 0 (token "SpectralIrradianceValue") (name "SpectralIrradianceValue") (separator none) (span (offset 24478) (line 402) (column 35) (len 23)))))
    (reference r320 (scope relative) (span (offset 24581) (line 404) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 24581) (line 404) (column 45) (len 11)))))
    (reference r321 (scope relative) (span (offset 24631) (line 405) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24631) (line 405) (column 37) (len 19)))))
    (reference r322 (scope relative) (span (offset 24660) (line 405) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24660) (line 405) (column 66) (len 8)))))
    (reference r323 (scope relative) (span (offset 24671) (line 405) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24671) (line 405) (column 77) (len 3)))))
    (reference r324 (scope relative) (span (offset 24675) (line 405) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 24675) (line 405) (column 81) (len 1)))))
    (reference r325 (scope relative) (span (offset 24682) (line 405) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24682) (line 405) (column 88) (len 8)))))
    (reference r326 (scope relative) (span (offset 24733) (line 406) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24733) (line 406) (column 35) (len 19)))))
    (reference r327 (scope relative) (span (offset 24762) (line 406) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24762) (line 406) (column 64) (len 8)))))
    (reference r328 (scope relative) (span (offset 24773) (line 406) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24773) (line 406) (column 75) (len 3)))))
    (reference r329 (scope relative) (span (offset 24777) (line 406) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 24777) (line 406) (column 79) (len 1)))))
    (reference r330 (scope relative) (span (offset 24784) (line 406) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24784) (line 406) (column 86) (len 8)))))
    (reference r331 (scope relative) (span (offset 24838) (line 407) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24838) (line 407) (column 39) (len 19)))))
    (reference r332 (scope relative) (span (offset 24867) (line 407) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24867) (line 407) (column 68) (len 8)))))
    (reference r333 (scope relative) (span (offset 24878) (line 407) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24878) (line 407) (column 79) (len 3)))))
    (reference r334 (scope relative) (span (offset 24882) (line 407) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 24882) (line 407) (column 83) (len 1)))))
    (reference r335 (scope relative) (span (offset 24889) (line 407) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24889) (line 407) (column 90) (len 8)))))
    (reference r336 (scope relative) (span (offset 24928) (line 408) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 24928) (line 408) (column 23) (len 17)))))
    (reference r337 (scope relative) (span (offset 24952) (line 408) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 24952) (line 408) (column 47) (len 20)))))
    (reference r338 (scope relative) (span (offset 24976) (line 408) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 24976) (line 408) (column 71) (len 8)))))
    (reference r339 (scope relative) (span (offset 24986) (line 408) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 24986) (line 408) (column 81) (len 6)))))
    (reference r340 (scope relative) (span (offset 24994) (line 408) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 24994) (line 408) (column 89) (len 10)))))
    (reference r341 (scope relative) (span (offset 25128) (line 412) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 25128) (line 412) (column 43) (len 19)))))
    (reference r342 (scope relative) (span (offset 26084) (line 425) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 26084) (line 425) (column 28) (len 4)))))
    (reference r343 (scope relative) (span (offset 26079) (line 425) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 26079) (line 425) (column 23) (len 3)))))
    (reference r344 (scope relative) (span (offset 26118) (line 426) (column 29) (len 19)) (segments (segment 0 (token "RadiantExitanceUnit") (name "RadiantExitanceUnit") (separator none) (span (offset 26118) (line 426) (column 29) (len 19)))))
    (reference r345 (scope relative) (span (offset 26112) (line 426) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 26112) (line 426) (column 23) (len 4)))))
    (reference r346 (scope relative) (span (offset 26180) (line 429) (column 32) (len 20)) (segments (segment 0 (token "RadiantExitanceValue") (name "RadiantExitanceValue") (separator none) (span (offset 26180) (line 429) (column 32) (len 20)))))
    (reference r347 (scope relative) (span (offset 26277) (line 431) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 26277) (line 431) (column 42) (len 11)))))
    (reference r348 (scope relative) (span (offset 26325) (line 432) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26325) (line 432) (column 35) (len 19)))))
    (reference r349 (scope relative) (span (offset 26354) (line 432) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26354) (line 432) (column 64) (len 8)))))
    (reference r350 (scope relative) (span (offset 26365) (line 432) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26365) (line 432) (column 75) (len 3)))))
    (reference r351 (scope relative) (span (offset 26369) (line 432) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 26369) (line 432) (column 79) (len 1)))))
    (reference r352 (scope relative) (span (offset 26376) (line 432) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26376) (line 432) (column 86) (len 8)))))
    (reference r353 (scope relative) (span (offset 26430) (line 433) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26430) (line 433) (column 39) (len 19)))))
    (reference r354 (scope relative) (span (offset 26459) (line 433) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26459) (line 433) (column 68) (len 8)))))
    (reference r355 (scope relative) (span (offset 26470) (line 433) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26470) (line 433) (column 79) (len 3)))))
    (reference r356 (scope relative) (span (offset 26474) (line 433) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 26474) (line 433) (column 83) (len 1)))))
    (reference r357 (scope relative) (span (offset 26481) (line 433) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26481) (line 433) (column 90) (len 8)))))
    (reference r358 (scope relative) (span (offset 26520) (line 434) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 26520) (line 434) (column 23) (len 17)))))
    (reference r359 (scope relative) (span (offset 26544) (line 434) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 26544) (line 434) (column 47) (len 20)))))
    (reference r360 (scope relative) (span (offset 26568) (line 434) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 26568) (line 434) (column 71) (len 6)))))
    (reference r361 (scope relative) (span (offset 26576) (line 434) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 26576) (line 434) (column 79) (len 10)))))
    (reference r362 (scope relative) (span (offset 26633) (line 437) (column 36) (len 19)) (segments (segment 0 (token "RadiantExitanceUnit") (name "RadiantExitanceUnit") (separator none) (span (offset 26633) (line 437) (column 36) (len 19)))))
    (reference r363 (scope relative) (span (offset 26690) (line 438) (column 37) (len 20)) (segments (segment 0 (token "RadiantExitanceValue") (name "RadiantExitanceValue") (separator none) (span (offset 26690) (line 438) (column 37) (len 20)))))
    (reference r364 (scope relative) (span (offset 26743) (line 439) (column 32) (len 15)) (segments (segment 0 (token "radiantExitance") (name "radiantExitance") (separator none) (span (offset 26743) (line 439) (column 32) (len 15)))))
    (reference r365 (scope relative) (span (offset 26870) (line 442) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 26870) (line 442) (column 51) (len 19)))))
    (reference r366 (scope relative) (span (offset 27649) (line 455) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 27649) (line 455) (column 28) (len 4)))))
    (reference r367 (scope relative) (span (offset 27644) (line 455) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 27644) (line 455) (column 23) (len 3)))))
    (reference r368 (scope relative) (span (offset 27683) (line 456) (column 29) (len 27)) (segments (segment 0 (token "SpectralRadiantExitanceUnit") (name "SpectralRadiantExitanceUnit") (separator none) (span (offset 27683) (line 456) (column 29) (len 27)))))
    (reference r369 (scope relative) (span (offset 27677) (line 456) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 27677) (line 456) (column 23) (len 4)))))
    (reference r370 (scope relative) (span (offset 27761) (line 459) (column 40) (len 28)) (segments (segment 0 (token "SpectralRadiantExitanceValue") (name "SpectralRadiantExitanceValue") (separator none) (span (offset 27761) (line 459) (column 40) (len 28)))))
    (reference r371 (scope relative) (span (offset 27874) (line 461) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 27874) (line 461) (column 50) (len 11)))))
    (reference r372 (scope relative) (span (offset 27924) (line 462) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27924) (line 462) (column 37) (len 19)))))
    (reference r373 (scope relative) (span (offset 27953) (line 462) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27953) (line 462) (column 66) (len 8)))))
    (reference r374 (scope relative) (span (offset 27964) (line 462) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27964) (line 462) (column 77) (len 3)))))
    (reference r375 (scope relative) (span (offset 27968) (line 462) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 27968) (line 462) (column 81) (len 1)))))
    (reference r376 (scope relative) (span (offset 27975) (line 462) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27975) (line 462) (column 88) (len 8)))))
    (reference r377 (scope relative) (span (offset 28026) (line 463) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28026) (line 463) (column 35) (len 19)))))
    (reference r378 (scope relative) (span (offset 28055) (line 463) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28055) (line 463) (column 64) (len 8)))))
    (reference r379 (scope relative) (span (offset 28066) (line 463) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28066) (line 463) (column 75) (len 3)))))
    (reference r380 (scope relative) (span (offset 28070) (line 463) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 28070) (line 463) (column 79) (len 1)))))
    (reference r381 (scope relative) (span (offset 28077) (line 463) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28077) (line 463) (column 86) (len 8)))))
    (reference r382 (scope relative) (span (offset 28131) (line 464) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28131) (line 464) (column 39) (len 19)))))
    (reference r383 (scope relative) (span (offset 28160) (line 464) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28160) (line 464) (column 68) (len 8)))))
    (reference r384 (scope relative) (span (offset 28171) (line 464) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28171) (line 464) (column 79) (len 3)))))
    (reference r385 (scope relative) (span (offset 28175) (line 464) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 28175) (line 464) (column 83) (len 1)))))
    (reference r386 (scope relative) (span (offset 28182) (line 464) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28182) (line 464) (column 90) (len 8)))))
    (reference r387 (scope relative) (span (offset 28221) (line 465) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 28221) (line 465) (column 23) (len 17)))))
    (reference r388 (scope relative) (span (offset 28245) (line 465) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 28245) (line 465) (column 47) (len 20)))))
    (reference r389 (scope relative) (span (offset 28269) (line 465) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 28269) (line 465) (column 71) (len 8)))))
    (reference r390 (scope relative) (span (offset 28279) (line 465) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 28279) (line 465) (column 81) (len 6)))))
    (reference r391 (scope relative) (span (offset 28287) (line 465) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 28287) (line 465) (column 89) (len 10)))))
    (reference r392 (scope relative) (span (offset 28401) (line 469) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 28401) (line 469) (column 43) (len 19)))))
    (reference r393 (scope relative) (span (offset 29195) (line 482) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 29195) (line 482) (column 28) (len 4)))))
    (reference r394 (scope relative) (span (offset 29190) (line 482) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 29190) (line 482) (column 23) (len 3)))))
    (reference r395 (scope relative) (span (offset 29229) (line 483) (column 29) (len 19)) (segments (segment 0 (token "RadiantExposureUnit") (name "RadiantExposureUnit") (separator none) (span (offset 29229) (line 483) (column 29) (len 19)))))
    (reference r396 (scope relative) (span (offset 29223) (line 483) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 29223) (line 483) (column 23) (len 4)))))
    (reference r397 (scope relative) (span (offset 29291) (line 486) (column 32) (len 20)) (segments (segment 0 (token "RadiantExposureValue") (name "RadiantExposureValue") (separator none) (span (offset 29291) (line 486) (column 32) (len 20)))))
    (reference r398 (scope relative) (span (offset 29388) (line 488) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 29388) (line 488) (column 42) (len 11)))))
    (reference r399 (scope relative) (span (offset 29436) (line 489) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29436) (line 489) (column 35) (len 19)))))
    (reference r400 (scope relative) (span (offset 29465) (line 489) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29465) (line 489) (column 64) (len 8)))))
    (reference r401 (scope relative) (span (offset 29476) (line 489) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29476) (line 489) (column 75) (len 3)))))
    (reference r402 (scope relative) (span (offset 29480) (line 489) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 29480) (line 489) (column 79) (len 1)))))
    (reference r403 (scope relative) (span (offset 29487) (line 489) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29487) (line 489) (column 86) (len 8)))))
    (reference r404 (scope relative) (span (offset 29541) (line 490) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29541) (line 490) (column 39) (len 19)))))
    (reference r405 (scope relative) (span (offset 29570) (line 490) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29570) (line 490) (column 68) (len 8)))))
    (reference r406 (scope relative) (span (offset 29581) (line 490) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29581) (line 490) (column 79) (len 3)))))
    (reference r407 (scope relative) (span (offset 29585) (line 490) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 29585) (line 490) (column 83) (len 1)))))
    (reference r408 (scope relative) (span (offset 29592) (line 490) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29592) (line 490) (column 90) (len 8)))))
    (reference r409 (scope relative) (span (offset 29631) (line 491) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 29631) (line 491) (column 23) (len 17)))))
    (reference r410 (scope relative) (span (offset 29655) (line 491) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 29655) (line 491) (column 47) (len 20)))))
    (reference r411 (scope relative) (span (offset 29679) (line 491) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 29679) (line 491) (column 71) (len 6)))))
    (reference r412 (scope relative) (span (offset 29687) (line 491) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 29687) (line 491) (column 79) (len 10)))))
    (reference r413 (scope relative) (span (offset 29818) (line 495) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 29818) (line 495) (column 51) (len 19)))))
    (reference r414 (scope relative) (span (offset 30597) (line 508) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 30597) (line 508) (column 28) (len 4)))))
    (reference r415 (scope relative) (span (offset 30592) (line 508) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 30592) (line 508) (column 23) (len 3)))))
    (reference r416 (scope relative) (span (offset 30631) (line 509) (column 29) (len 27)) (segments (segment 0 (token "SpectralRadiantExposureUnit") (name "SpectralRadiantExposureUnit") (separator none) (span (offset 30631) (line 509) (column 29) (len 27)))))
    (reference r417 (scope relative) (span (offset 30625) (line 509) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 30625) (line 509) (column 23) (len 4)))))
    (reference r418 (scope relative) (span (offset 30709) (line 512) (column 40) (len 28)) (segments (segment 0 (token "SpectralRadiantExposureValue") (name "SpectralRadiantExposureValue") (separator none) (span (offset 30709) (line 512) (column 40) (len 28)))))
    (reference r419 (scope relative) (span (offset 30822) (line 514) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 30822) (line 514) (column 50) (len 11)))))
    (reference r420 (scope relative) (span (offset 30872) (line 515) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30872) (line 515) (column 37) (len 19)))))
    (reference r421 (scope relative) (span (offset 30901) (line 515) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30901) (line 515) (column 66) (len 8)))))
    (reference r422 (scope relative) (span (offset 30912) (line 515) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30912) (line 515) (column 77) (len 3)))))
    (reference r423 (scope relative) (span (offset 30916) (line 515) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 30916) (line 515) (column 81) (len 1)))))
    (reference r424 (scope relative) (span (offset 30923) (line 515) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30923) (line 515) (column 88) (len 8)))))
    (reference r425 (scope relative) (span (offset 30974) (line 516) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30974) (line 516) (column 35) (len 19)))))
    (reference r426 (scope relative) (span (offset 31003) (line 516) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31003) (line 516) (column 64) (len 8)))))
    (reference r427 (scope relative) (span (offset 31014) (line 516) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31014) (line 516) (column 75) (len 3)))))
    (reference r428 (scope relative) (span (offset 31018) (line 516) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 31018) (line 516) (column 79) (len 1)))))
    (reference r429 (scope relative) (span (offset 31025) (line 516) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31025) (line 516) (column 86) (len 8)))))
    (reference r430 (scope relative) (span (offset 31079) (line 517) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31079) (line 517) (column 39) (len 19)))))
    (reference r431 (scope relative) (span (offset 31108) (line 517) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31108) (line 517) (column 68) (len 8)))))
    (reference r432 (scope relative) (span (offset 31119) (line 517) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31119) (line 517) (column 79) (len 3)))))
    (reference r433 (scope relative) (span (offset 31123) (line 517) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 31123) (line 517) (column 83) (len 1)))))
    (reference r434 (scope relative) (span (offset 31130) (line 517) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31130) (line 517) (column 90) (len 8)))))
    (reference r435 (scope relative) (span (offset 31169) (line 518) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 31169) (line 518) (column 23) (len 17)))))
    (reference r436 (scope relative) (span (offset 31193) (line 518) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 31193) (line 518) (column 47) (len 20)))))
    (reference r437 (scope relative) (span (offset 31217) (line 518) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 31217) (line 518) (column 71) (len 8)))))
    (reference r438 (scope relative) (span (offset 31227) (line 518) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 31227) (line 518) (column 81) (len 6)))))
    (reference r439 (scope relative) (span (offset 31235) (line 518) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 31235) (line 518) (column 89) (len 10)))))
    (reference r440 (scope relative) (span (offset 31356) (line 522) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 31356) (line 522) (column 46) (len 17)))))
    (reference r441 (scope relative) (span (offset 32661) (line 536) (column 35) (len 23)) (segments (segment 0 (token "LuminousEfficiencyValue") (name "LuminousEfficiencyValue") (separator none) (span (offset 32661) (line 536) (column 35) (len 23)))))
    (reference r442 (scope relative) (span (offset 32823) (line 539) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 32823) (line 539) (column 54) (len 17)))))
    (reference r443 (scope relative) (span (offset 34262) (line 553) (column 43) (len 31)) (segments (segment 0 (token "SpectralLuminousEfficiencyValue") (name "SpectralLuminousEfficiencyValue") (separator none) (span (offset 34262) (line 553) (column 43) (len 31)))))
    (reference r444 (scope relative) (span (offset 34435) (line 556) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 34435) (line 556) (column 55) (len 19)))))
    (reference r445 (scope relative) (span (offset 35512) (line 569) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 35512) (line 569) (column 28) (len 4)))))
    (reference r446 (scope relative) (span (offset 35507) (line 569) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 35507) (line 569) (column 23) (len 3)))))
    (reference r447 (scope relative) (span (offset 35546) (line 570) (column 29) (len 31)) (segments (segment 0 (token "LuminousEfficacyOfRadiationUnit") (name "LuminousEfficacyOfRadiationUnit") (separator none) (span (offset 35546) (line 570) (column 29) (len 31)))))
    (reference r448 (scope relative) (span (offset 35540) (line 570) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 35540) (line 570) (column 23) (len 4)))))
    (reference r449 (scope relative) (span (offset 35632) (line 573) (column 44) (len 32)) (segments (segment 0 (token "LuminousEfficacyOfRadiationValue") (name "LuminousEfficacyOfRadiationValue") (separator none) (span (offset 35632) (line 573) (column 44) (len 32)))))
    (reference r450 (scope relative) (span (offset 35753) (line 575) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 35753) (line 575) (column 54) (len 11)))))
    (reference r451 (scope relative) (span (offset 35803) (line 576) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 35803) (line 576) (column 37) (len 19)))))
    (reference r452 (scope relative) (span (offset 35832) (line 576) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 35832) (line 576) (column 66) (len 8)))))
    (reference r453 (scope relative) (span (offset 35843) (line 576) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 35843) (line 576) (column 77) (len 3)))))
    (reference r454 (scope relative) (span (offset 35847) (line 576) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 35847) (line 576) (column 81) (len 1)))))
    (reference r455 (scope relative) (span (offset 35854) (line 576) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 35854) (line 576) (column 88) (len 8)))))
    (reference r456 (scope relative) (span (offset 35905) (line 577) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 35905) (line 577) (column 35) (len 19)))))
    (reference r457 (scope relative) (span (offset 35934) (line 577) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 35934) (line 577) (column 64) (len 8)))))
    (reference r458 (scope relative) (span (offset 35945) (line 577) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 35945) (line 577) (column 75) (len 3)))))
    (reference r459 (scope relative) (span (offset 35949) (line 577) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 35949) (line 577) (column 79) (len 1)))))
    (reference r460 (scope relative) (span (offset 35956) (line 577) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 35956) (line 577) (column 86) (len 8)))))
    (reference r461 (scope relative) (span (offset 36011) (line 578) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36011) (line 578) (column 39) (len 19)))))
    (reference r462 (scope relative) (span (offset 36040) (line 578) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36040) (line 578) (column 68) (len 8)))))
    (reference r463 (scope relative) (span (offset 36051) (line 578) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36051) (line 578) (column 79) (len 3)))))
    (reference r464 (scope relative) (span (offset 36055) (line 578) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 36055) (line 578) (column 83) (len 1)))))
    (reference r465 (scope relative) (span (offset 36062) (line 578) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36062) (line 578) (column 90) (len 8)))))
    (reference r466 (scope relative) (span (offset 36125) (line 579) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36125) (line 579) (column 48) (len 19)))))
    (reference r467 (scope relative) (span (offset 36154) (line 579) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36154) (line 579) (column 77) (len 8)))))
    (reference r468 (scope relative) (span (offset 36165) (line 579) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36165) (line 579) (column 88) (len 3)))))
    (reference r469 (scope relative) (span (offset 36169) (line 579) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 36169) (line 579) (column 92) (len 1)))))
    (reference r470 (scope relative) (span (offset 36176) (line 579) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36176) (line 579) (column 99) (len 8)))))
    (reference r471 (scope relative) (span (offset 36214) (line 580) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 36214) (line 580) (column 23) (len 17)))))
    (reference r472 (scope relative) (span (offset 36238) (line 580) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 36238) (line 580) (column 47) (len 20)))))
    (reference r473 (scope relative) (span (offset 36262) (line 580) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 36262) (line 580) (column 71) (len 8)))))
    (reference r474 (scope relative) (span (offset 36272) (line 580) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 36272) (line 580) (column 81) (len 6)))))
    (reference r475 (scope relative) (span (offset 36280) (line 580) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 36280) (line 580) (column 89) (len 10)))))
    (reference r476 (scope relative) (span (offset 36292) (line 580) (column 101) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 36292) (line 580) (column 101) (len 19)))))
    (reference r477 (scope relative) (span (offset 36435) (line 584) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 36435) (line 584) (column 52) (len 19)))))
    (reference r478 (scope relative) (span (offset 37593) (line 597) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 37593) (line 597) (column 28) (len 4)))))
    (reference r479 (scope relative) (span (offset 37588) (line 597) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 37588) (line 597) (column 23) (len 3)))))
    (reference r480 (scope relative) (span (offset 37627) (line 598) (column 29) (len 28)) (segments (segment 0 (token "SpectralLuminousEfficacyUnit") (name "SpectralLuminousEfficacyUnit") (separator none) (span (offset 37627) (line 598) (column 29) (len 28)))))
    (reference r481 (scope relative) (span (offset 37621) (line 598) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 37621) (line 598) (column 23) (len 4)))))
    (reference r482 (scope relative) (span (offset 37707) (line 601) (column 41) (len 29)) (segments (segment 0 (token "SpectralLuminousEfficacyValue") (name "SpectralLuminousEfficacyValue") (separator none) (span (offset 37707) (line 601) (column 41) (len 29)))))
    (reference r483 (scope relative) (span (offset 37822) (line 603) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 37822) (line 603) (column 51) (len 11)))))
    (reference r484 (scope relative) (span (offset 37872) (line 604) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37872) (line 604) (column 37) (len 19)))))
    (reference r485 (scope relative) (span (offset 37901) (line 604) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37901) (line 604) (column 66) (len 8)))))
    (reference r486 (scope relative) (span (offset 37912) (line 604) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37912) (line 604) (column 77) (len 3)))))
    (reference r487 (scope relative) (span (offset 37916) (line 604) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 37916) (line 604) (column 81) (len 1)))))
    (reference r488 (scope relative) (span (offset 37923) (line 604) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37923) (line 604) (column 88) (len 8)))))
    (reference r489 (scope relative) (span (offset 37974) (line 605) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37974) (line 605) (column 35) (len 19)))))
    (reference r490 (scope relative) (span (offset 38003) (line 605) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 38003) (line 605) (column 64) (len 8)))))
    (reference r491 (scope relative) (span (offset 38014) (line 605) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 38014) (line 605) (column 75) (len 3)))))
    (reference r492 (scope relative) (span (offset 38018) (line 605) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 38018) (line 605) (column 79) (len 1)))))
    (reference r493 (scope relative) (span (offset 38025) (line 605) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 38025) (line 605) (column 86) (len 8)))))
    (reference r494 (scope relative) (span (offset 38080) (line 606) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 38080) (line 606) (column 39) (len 19)))))
    (reference r495 (scope relative) (span (offset 38109) (line 606) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 38109) (line 606) (column 68) (len 8)))))
    (reference r496 (scope relative) (span (offset 38120) (line 606) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 38120) (line 606) (column 79) (len 3)))))
    (reference r497 (scope relative) (span (offset 38124) (line 606) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 38124) (line 606) (column 83) (len 1)))))
    (reference r498 (scope relative) (span (offset 38131) (line 606) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 38131) (line 606) (column 90) (len 8)))))
    (reference r499 (scope relative) (span (offset 38194) (line 607) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 38194) (line 607) (column 48) (len 19)))))
    (reference r500 (scope relative) (span (offset 38223) (line 607) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 38223) (line 607) (column 77) (len 8)))))
    (reference r501 (scope relative) (span (offset 38234) (line 607) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 38234) (line 607) (column 88) (len 3)))))
    (reference r502 (scope relative) (span (offset 38238) (line 607) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 38238) (line 607) (column 92) (len 1)))))
    (reference r503 (scope relative) (span (offset 38245) (line 607) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 38245) (line 607) (column 99) (len 8)))))
    (reference r504 (scope relative) (span (offset 38283) (line 608) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 38283) (line 608) (column 23) (len 17)))))
    (reference r505 (scope relative) (span (offset 38307) (line 608) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 38307) (line 608) (column 47) (len 20)))))
    (reference r506 (scope relative) (span (offset 38331) (line 608) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 38331) (line 608) (column 71) (len 8)))))
    (reference r507 (scope relative) (span (offset 38341) (line 608) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 38341) (line 608) (column 81) (len 6)))))
    (reference r508 (scope relative) (span (offset 38349) (line 608) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 38349) (line 608) (column 89) (len 10)))))
    (reference r509 (scope relative) (span (offset 38361) (line 608) (column 101) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 38361) (line 608) (column 101) (len 19)))))
    (reference r510 (scope relative) (span (offset 38502) (line 612) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 38502) (line 612) (column 51) (len 19)))))
    (reference r511 (scope relative) (span (offset 39703) (line 625) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 39703) (line 625) (column 28) (len 4)))))
    (reference r512 (scope relative) (span (offset 39698) (line 625) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 39698) (line 625) (column 23) (len 3)))))
    (reference r513 (scope relative) (span (offset 39737) (line 626) (column 29) (len 27)) (segments (segment 0 (token "MaximumLuminousEfficacyUnit") (name "MaximumLuminousEfficacyUnit") (separator none) (span (offset 39737) (line 626) (column 29) (len 27)))))
    (reference r514 (scope relative) (span (offset 39731) (line 626) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 39731) (line 626) (column 23) (len 4)))))
    (reference r515 (scope relative) (span (offset 39815) (line 629) (column 40) (len 28)) (segments (segment 0 (token "MaximumLuminousEfficacyValue") (name "MaximumLuminousEfficacyValue") (separator none) (span (offset 39815) (line 629) (column 40) (len 28)))))
    (reference r516 (scope relative) (span (offset 39928) (line 631) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 39928) (line 631) (column 50) (len 11)))))
    (reference r517 (scope relative) (span (offset 39978) (line 632) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 39978) (line 632) (column 37) (len 19)))))
    (reference r518 (scope relative) (span (offset 40007) (line 632) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40007) (line 632) (column 66) (len 8)))))
    (reference r519 (scope relative) (span (offset 40018) (line 632) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40018) (line 632) (column 77) (len 3)))))
    (reference r520 (scope relative) (span (offset 40022) (line 632) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 40022) (line 632) (column 81) (len 1)))))
    (reference r521 (scope relative) (span (offset 40029) (line 632) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40029) (line 632) (column 88) (len 8)))))
    (reference r522 (scope relative) (span (offset 40080) (line 633) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40080) (line 633) (column 35) (len 19)))))
    (reference r523 (scope relative) (span (offset 40109) (line 633) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40109) (line 633) (column 64) (len 8)))))
    (reference r524 (scope relative) (span (offset 40120) (line 633) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40120) (line 633) (column 75) (len 3)))))
    (reference r525 (scope relative) (span (offset 40124) (line 633) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 40124) (line 633) (column 79) (len 1)))))
    (reference r526 (scope relative) (span (offset 40131) (line 633) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40131) (line 633) (column 86) (len 8)))))
    (reference r527 (scope relative) (span (offset 40186) (line 634) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40186) (line 634) (column 39) (len 19)))))
    (reference r528 (scope relative) (span (offset 40215) (line 634) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40215) (line 634) (column 68) (len 8)))))
    (reference r529 (scope relative) (span (offset 40226) (line 634) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40226) (line 634) (column 79) (len 3)))))
    (reference r530 (scope relative) (span (offset 40230) (line 634) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 40230) (line 634) (column 83) (len 1)))))
    (reference r531 (scope relative) (span (offset 40237) (line 634) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40237) (line 634) (column 90) (len 8)))))
    (reference r532 (scope relative) (span (offset 40300) (line 635) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 40300) (line 635) (column 48) (len 19)))))
    (reference r533 (scope relative) (span (offset 40329) (line 635) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 40329) (line 635) (column 77) (len 8)))))
    (reference r534 (scope relative) (span (offset 40340) (line 635) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 40340) (line 635) (column 88) (len 3)))))
    (reference r535 (scope relative) (span (offset 40344) (line 635) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 40344) (line 635) (column 92) (len 1)))))
    (reference r536 (scope relative) (span (offset 40351) (line 635) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 40351) (line 635) (column 99) (len 8)))))
    (reference r537 (scope relative) (span (offset 40389) (line 636) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 40389) (line 636) (column 23) (len 17)))))
    (reference r538 (scope relative) (span (offset 40413) (line 636) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 40413) (line 636) (column 47) (len 20)))))
    (reference r539 (scope relative) (span (offset 40437) (line 636) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 40437) (line 636) (column 71) (len 8)))))
    (reference r540 (scope relative) (span (offset 40447) (line 636) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 40447) (line 636) (column 81) (len 6)))))
    (reference r541 (scope relative) (span (offset 40455) (line 636) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 40455) (line 636) (column 89) (len 10)))))
    (reference r542 (scope relative) (span (offset 40467) (line 636) (column 101) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 40467) (line 636) (column 101) (len 19)))))
    (reference r543 (scope relative) (span (offset 40614) (line 640) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 40614) (line 640) (column 53) (len 19)))))
    (reference r544 (scope relative) (span (offset 41268) (line 653) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 41268) (line 653) (column 28) (len 4)))))
    (reference r545 (scope relative) (span (offset 41263) (line 653) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 41263) (line 653) (column 23) (len 3)))))
    (reference r546 (scope relative) (span (offset 41302) (line 654) (column 29) (len 29)) (segments (segment 0 (token "LuminousEfficacyOfASourceUnit") (name "LuminousEfficacyOfASourceUnit") (separator none) (span (offset 41302) (line 654) (column 29) (len 29)))))
    (reference r547 (scope relative) (span (offset 41296) (line 654) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 41296) (line 654) (column 23) (len 4)))))
    (reference r548 (scope relative) (span (offset 41384) (line 657) (column 42) (len 30)) (segments (segment 0 (token "LuminousEfficacyOfASourceValue") (name "LuminousEfficacyOfASourceValue") (separator none) (span (offset 41384) (line 657) (column 42) (len 30)))))
    (reference r549 (scope relative) (span (offset 41501) (line 659) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 41501) (line 659) (column 52) (len 11)))))
    (reference r550 (scope relative) (span (offset 41551) (line 660) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 41551) (line 660) (column 37) (len 19)))))
    (reference r551 (scope relative) (span (offset 41580) (line 660) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 41580) (line 660) (column 66) (len 8)))))
    (reference r552 (scope relative) (span (offset 41591) (line 660) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 41591) (line 660) (column 77) (len 3)))))
    (reference r553 (scope relative) (span (offset 41595) (line 660) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 41595) (line 660) (column 81) (len 1)))))
    (reference r554 (scope relative) (span (offset 41602) (line 660) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 41602) (line 660) (column 88) (len 8)))))
    (reference r555 (scope relative) (span (offset 41653) (line 661) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 41653) (line 661) (column 35) (len 19)))))
    (reference r556 (scope relative) (span (offset 41682) (line 661) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 41682) (line 661) (column 64) (len 8)))))
    (reference r557 (scope relative) (span (offset 41693) (line 661) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 41693) (line 661) (column 75) (len 3)))))
    (reference r558 (scope relative) (span (offset 41697) (line 661) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 41697) (line 661) (column 79) (len 1)))))
    (reference r559 (scope relative) (span (offset 41704) (line 661) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 41704) (line 661) (column 86) (len 8)))))
    (reference r560 (scope relative) (span (offset 41759) (line 662) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 41759) (line 662) (column 39) (len 19)))))
    (reference r561 (scope relative) (span (offset 41788) (line 662) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 41788) (line 662) (column 68) (len 8)))))
    (reference r562 (scope relative) (span (offset 41799) (line 662) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 41799) (line 662) (column 79) (len 3)))))
    (reference r563 (scope relative) (span (offset 41803) (line 662) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 41803) (line 662) (column 83) (len 1)))))
    (reference r564 (scope relative) (span (offset 41810) (line 662) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 41810) (line 662) (column 90) (len 8)))))
    (reference r565 (scope relative) (span (offset 41873) (line 663) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 41873) (line 663) (column 48) (len 19)))))
    (reference r566 (scope relative) (span (offset 41902) (line 663) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 41902) (line 663) (column 77) (len 8)))))
    (reference r567 (scope relative) (span (offset 41913) (line 663) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 41913) (line 663) (column 88) (len 3)))))
    (reference r568 (scope relative) (span (offset 41917) (line 663) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 41917) (line 663) (column 92) (len 1)))))
    (reference r569 (scope relative) (span (offset 41924) (line 663) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 41924) (line 663) (column 99) (len 8)))))
    (reference r570 (scope relative) (span (offset 41962) (line 664) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 41962) (line 664) (column 23) (len 17)))))
    (reference r571 (scope relative) (span (offset 41986) (line 664) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 41986) (line 664) (column 47) (len 20)))))
    (reference r572 (scope relative) (span (offset 42010) (line 664) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 42010) (line 664) (column 71) (len 8)))))
    (reference r573 (scope relative) (span (offset 42020) (line 664) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 42020) (line 664) (column 81) (len 6)))))
    (reference r574 (scope relative) (span (offset 42028) (line 664) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 42028) (line 664) (column 89) (len 10)))))
    (reference r575 (scope relative) (span (offset 42040) (line 664) (column 101) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 42040) (line 664) (column 101) (len 19)))))
    (reference r576 (scope relative) (span (offset 42179) (line 668) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 42179) (line 668) (column 42) (len 19)))))
    (reference r577 (scope relative) (span (offset 43442) (line 681) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 43442) (line 681) (column 28) (len 4)))))
    (reference r578 (scope relative) (span (offset 43437) (line 681) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 43437) (line 681) (column 23) (len 3)))))
    (reference r579 (scope relative) (span (offset 43476) (line 682) (column 29) (len 18)) (segments (segment 0 (token "LuminousEnergyUnit") (name "LuminousEnergyUnit") (separator none) (span (offset 43476) (line 682) (column 29) (len 18)))))
    (reference r580 (scope relative) (span (offset 43470) (line 682) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 43470) (line 682) (column 23) (len 4)))))
    (reference r581 (scope relative) (span (offset 43536) (line 685) (column 31) (len 19)) (segments (segment 0 (token "LuminousEnergyValue") (name "LuminousEnergyValue") (separator none) (span (offset 43536) (line 685) (column 31) (len 19)))))
    (reference r582 (scope relative) (span (offset 43631) (line 687) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 43631) (line 687) (column 41) (len 11)))))
    (reference r583 (scope relative) (span (offset 43683) (line 688) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43683) (line 688) (column 39) (len 19)))))
    (reference r584 (scope relative) (span (offset 43712) (line 688) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43712) (line 688) (column 68) (len 8)))))
    (reference r585 (scope relative) (span (offset 43723) (line 688) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43723) (line 688) (column 79) (len 3)))))
    (reference r586 (scope relative) (span (offset 43727) (line 688) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 43727) (line 688) (column 83) (len 1)))))
    (reference r587 (scope relative) (span (offset 43734) (line 688) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43734) (line 688) (column 90) (len 8)))))
    (reference r588 (scope relative) (span (offset 43797) (line 689) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43797) (line 689) (column 48) (len 19)))))
    (reference r589 (scope relative) (span (offset 43826) (line 689) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43826) (line 689) (column 77) (len 8)))))
    (reference r590 (scope relative) (span (offset 43837) (line 689) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43837) (line 689) (column 88) (len 3)))))
    (reference r591 (scope relative) (span (offset 43841) (line 689) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 43841) (line 689) (column 92) (len 1)))))
    (reference r592 (scope relative) (span (offset 43848) (line 689) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43848) (line 689) (column 99) (len 8)))))
    (reference r593 (scope relative) (span (offset 43886) (line 690) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 43886) (line 690) (column 23) (len 17)))))
    (reference r594 (scope relative) (span (offset 43910) (line 690) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 43910) (line 690) (column 47) (len 20)))))
    (reference r595 (scope relative) (span (offset 43934) (line 690) (column 71) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 43934) (line 690) (column 71) (len 10)))))
    (reference r596 (scope relative) (span (offset 43946) (line 690) (column 83) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 43946) (line 690) (column 83) (len 19)))))
    (reference r597 (scope relative) (span (offset 44011) (line 693) (column 35) (len 18)) (segments (segment 0 (token "LuminousEnergyUnit") (name "LuminousEnergyUnit") (separator none) (span (offset 44011) (line 693) (column 35) (len 18)))))
    (reference r598 (scope relative) (span (offset 44066) (line 694) (column 36) (len 19)) (segments (segment 0 (token "LuminousEnergyValue") (name "LuminousEnergyValue") (separator none) (span (offset 44066) (line 694) (column 36) (len 19)))))
    (reference r599 (scope relative) (span (offset 44117) (line 695) (column 31) (len 14)) (segments (segment 0 (token "luminousEnergy") (name "luminousEnergy") (separator none) (span (offset 44117) (line 695) (column 31) (len 14)))))
    (reference r600 (scope relative) (span (offset 44219) (line 698) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 44219) (line 698) (column 40) (len 19)))))
    (reference r601 (scope relative) (span (offset 45461) (line 711) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 45461) (line 711) (column 28) (len 4)))))
    (reference r602 (scope relative) (span (offset 45456) (line 711) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 45456) (line 711) (column 23) (len 3)))))
    (reference r603 (scope relative) (span (offset 45495) (line 712) (column 29) (len 16)) (segments (segment 0 (token "LuminousFluxUnit") (name "LuminousFluxUnit") (separator none) (span (offset 45495) (line 712) (column 29) (len 16)))))
    (reference r604 (scope relative) (span (offset 45489) (line 712) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 45489) (line 712) (column 23) (len 4)))))
    (reference r605 (scope relative) (span (offset 45551) (line 715) (column 29) (len 17)) (segments (segment 0 (token "LuminousFluxValue") (name "LuminousFluxValue") (separator none) (span (offset 45551) (line 715) (column 29) (len 17)))))
    (reference r606 (scope relative) (span (offset 45642) (line 717) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 45642) (line 717) (column 39) (len 11)))))
    (reference r607 (scope relative) (span (offset 45703) (line 718) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45703) (line 718) (column 48) (len 19)))))
    (reference r608 (scope relative) (span (offset 45732) (line 718) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45732) (line 718) (column 77) (len 8)))))
    (reference r609 (scope relative) (span (offset 45743) (line 718) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45743) (line 718) (column 88) (len 3)))))
    (reference r610 (scope relative) (span (offset 45747) (line 718) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 45747) (line 718) (column 92) (len 1)))))
    (reference r611 (scope relative) (span (offset 45754) (line 718) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45754) (line 718) (column 99) (len 8)))))
    (reference r612 (scope relative) (span (offset 45792) (line 719) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 45792) (line 719) (column 23) (len 17)))))
    (reference r613 (scope relative) (span (offset 45816) (line 719) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 45816) (line 719) (column 47) (len 20)))))
    (reference r614 (scope relative) (span (offset 45839) (line 719) (column 70) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 45839) (line 719) (column 70) (len 19)))))
    (reference r615 (scope relative) (span (offset 46102) (line 726) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 46102) (line 726) (column 37) (len 19)))))
    (reference r616 (scope relative) (span (offset 47414) (line 739) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 47414) (line 739) (column 28) (len 4)))))
    (reference r617 (scope relative) (span (offset 47409) (line 739) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 47409) (line 739) (column 23) (len 3)))))
    (reference r618 (scope relative) (span (offset 47448) (line 740) (column 29) (len 13)) (segments (segment 0 (token "LuminanceUnit") (name "LuminanceUnit") (separator none) (span (offset 47448) (line 740) (column 29) (len 13)))))
    (reference r619 (scope relative) (span (offset 47442) (line 740) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 47442) (line 740) (column 23) (len 4)))))
    (reference r620 (scope relative) (span (offset 47498) (line 743) (column 26) (len 14)) (segments (segment 0 (token "LuminanceValue") (name "LuminanceValue") (separator none) (span (offset 47498) (line 743) (column 26) (len 14)))))
    (reference r621 (scope relative) (span (offset 47583) (line 745) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 47583) (line 745) (column 36) (len 11)))))
    (reference r622 (scope relative) (span (offset 47633) (line 746) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 47633) (line 746) (column 37) (len 19)))))
    (reference r623 (scope relative) (span (offset 47662) (line 746) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 47662) (line 746) (column 66) (len 8)))))
    (reference r624 (scope relative) (span (offset 47673) (line 746) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 47673) (line 746) (column 77) (len 3)))))
    (reference r625 (scope relative) (span (offset 47677) (line 746) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 47677) (line 746) (column 81) (len 1)))))
    (reference r626 (scope relative) (span (offset 47684) (line 746) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 47684) (line 746) (column 88) (len 8)))))
    (reference r627 (scope relative) (span (offset 47748) (line 747) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 47748) (line 747) (column 48) (len 19)))))
    (reference r628 (scope relative) (span (offset 47777) (line 747) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 47777) (line 747) (column 77) (len 8)))))
    (reference r629 (scope relative) (span (offset 47788) (line 747) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 47788) (line 747) (column 88) (len 3)))))
    (reference r630 (scope relative) (span (offset 47792) (line 747) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 47792) (line 747) (column 92) (len 1)))))
    (reference r631 (scope relative) (span (offset 47799) (line 747) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 47799) (line 747) (column 99) (len 8)))))
    (reference r632 (scope relative) (span (offset 47837) (line 748) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 47837) (line 748) (column 23) (len 17)))))
    (reference r633 (scope relative) (span (offset 47861) (line 748) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 47861) (line 748) (column 47) (len 20)))))
    (reference r634 (scope relative) (span (offset 47885) (line 748) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 47885) (line 748) (column 71) (len 8)))))
    (reference r635 (scope relative) (span (offset 47895) (line 748) (column 81) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 47895) (line 748) (column 81) (len 19)))))
    (reference r636 (scope relative) (span (offset 48008) (line 752) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 48008) (line 752) (column 39) (len 19)))))
    (reference r637 (scope relative) (span (offset 49765) (line 765) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 49765) (line 765) (column 28) (len 4)))))
    (reference r638 (scope relative) (span (offset 49760) (line 765) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 49760) (line 765) (column 23) (len 3)))))
    (reference r639 (scope relative) (span (offset 49799) (line 766) (column 29) (len 15)) (segments (segment 0 (token "IlluminanceUnit") (name "IlluminanceUnit") (separator none) (span (offset 49799) (line 766) (column 29) (len 15)))))
    (reference r640 (scope relative) (span (offset 49793) (line 766) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 49793) (line 766) (column 23) (len 4)))))
    (reference r641 (scope relative) (span (offset 49853) (line 769) (column 28) (len 16)) (segments (segment 0 (token "IlluminanceValue") (name "IlluminanceValue") (separator none) (span (offset 49853) (line 769) (column 28) (len 16)))))
    (reference r642 (scope relative) (span (offset 49942) (line 771) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 49942) (line 771) (column 38) (len 11)))))
    (reference r643 (scope relative) (span (offset 49992) (line 772) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 49992) (line 772) (column 37) (len 19)))))
    (reference r644 (scope relative) (span (offset 50021) (line 772) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50021) (line 772) (column 66) (len 8)))))
    (reference r645 (scope relative) (span (offset 50032) (line 772) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50032) (line 772) (column 77) (len 3)))))
    (reference r646 (scope relative) (span (offset 50036) (line 772) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 50036) (line 772) (column 81) (len 1)))))
    (reference r647 (scope relative) (span (offset 50043) (line 772) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50043) (line 772) (column 88) (len 8)))))
    (reference r648 (scope relative) (span (offset 50107) (line 773) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50107) (line 773) (column 48) (len 19)))))
    (reference r649 (scope relative) (span (offset 50136) (line 773) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50136) (line 773) (column 77) (len 8)))))
    (reference r650 (scope relative) (span (offset 50147) (line 773) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50147) (line 773) (column 88) (len 3)))))
    (reference r651 (scope relative) (span (offset 50151) (line 773) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 50151) (line 773) (column 92) (len 1)))))
    (reference r652 (scope relative) (span (offset 50158) (line 773) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50158) (line 773) (column 99) (len 8)))))
    (reference r653 (scope relative) (span (offset 50196) (line 774) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 50196) (line 774) (column 23) (len 17)))))
    (reference r654 (scope relative) (span (offset 50220) (line 774) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 50220) (line 774) (column 47) (len 20)))))
    (reference r655 (scope relative) (span (offset 50244) (line 774) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 50244) (line 774) (column 71) (len 8)))))
    (reference r656 (scope relative) (span (offset 50254) (line 774) (column 81) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 50254) (line 774) (column 81) (len 19)))))
    (reference r657 (scope relative) (span (offset 50378) (line 778) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 50378) (line 778) (column 44) (len 19)))))
    (reference r658 (scope relative) (span (offset 51612) (line 791) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 51612) (line 791) (column 28) (len 4)))))
    (reference r659 (scope relative) (span (offset 51607) (line 791) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 51607) (line 791) (column 23) (len 3)))))
    (reference r660 (scope relative) (span (offset 51646) (line 792) (column 29) (len 20)) (segments (segment 0 (token "LuminousExitanceUnit") (name "LuminousExitanceUnit") (separator none) (span (offset 51646) (line 792) (column 29) (len 20)))))
    (reference r661 (scope relative) (span (offset 51640) (line 792) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 51640) (line 792) (column 23) (len 4)))))
    (reference r662 (scope relative) (span (offset 51710) (line 795) (column 33) (len 21)) (segments (segment 0 (token "LuminousExitanceValue") (name "LuminousExitanceValue") (separator none) (span (offset 51710) (line 795) (column 33) (len 21)))))
    (reference r663 (scope relative) (span (offset 51809) (line 797) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 51809) (line 797) (column 43) (len 11)))))
    (reference r664 (scope relative) (span (offset 51859) (line 798) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51859) (line 798) (column 37) (len 19)))))
    (reference r665 (scope relative) (span (offset 51888) (line 798) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51888) (line 798) (column 66) (len 8)))))
    (reference r666 (scope relative) (span (offset 51899) (line 798) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51899) (line 798) (column 77) (len 3)))))
    (reference r667 (scope relative) (span (offset 51903) (line 798) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 51903) (line 798) (column 81) (len 1)))))
    (reference r668 (scope relative) (span (offset 51910) (line 798) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51910) (line 798) (column 88) (len 8)))))
    (reference r669 (scope relative) (span (offset 51974) (line 799) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51974) (line 799) (column 48) (len 19)))))
    (reference r670 (scope relative) (span (offset 52003) (line 799) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 52003) (line 799) (column 77) (len 8)))))
    (reference r671 (scope relative) (span (offset 52014) (line 799) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 52014) (line 799) (column 88) (len 3)))))
    (reference r672 (scope relative) (span (offset 52018) (line 799) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 52018) (line 799) (column 92) (len 1)))))
    (reference r673 (scope relative) (span (offset 52025) (line 799) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 52025) (line 799) (column 99) (len 8)))))
    (reference r674 (scope relative) (span (offset 52063) (line 800) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 52063) (line 800) (column 23) (len 17)))))
    (reference r675 (scope relative) (span (offset 52087) (line 800) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 52087) (line 800) (column 47) (len 20)))))
    (reference r676 (scope relative) (span (offset 52111) (line 800) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 52111) (line 800) (column 71) (len 8)))))
    (reference r677 (scope relative) (span (offset 52121) (line 800) (column 81) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 52121) (line 800) (column 81) (len 19)))))
    (reference r678 (scope relative) (span (offset 52287) (line 804) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 52287) (line 804) (column 44) (len 19)))))
    (reference r679 (scope relative) (span (offset 53575) (line 817) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 53575) (line 817) (column 28) (len 4)))))
    (reference r680 (scope relative) (span (offset 53570) (line 817) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 53570) (line 817) (column 23) (len 3)))))
    (reference r681 (scope relative) (span (offset 53609) (line 818) (column 29) (len 20)) (segments (segment 0 (token "LuminousExposureUnit") (name "LuminousExposureUnit") (separator none) (span (offset 53609) (line 818) (column 29) (len 20)))))
    (reference r682 (scope relative) (span (offset 53603) (line 818) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 53603) (line 818) (column 23) (len 4)))))
    (reference r683 (scope relative) (span (offset 53673) (line 821) (column 33) (len 21)) (segments (segment 0 (token "LuminousExposureValue") (name "LuminousExposureValue") (separator none) (span (offset 53673) (line 821) (column 33) (len 21)))))
    (reference r684 (scope relative) (span (offset 53772) (line 823) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 53772) (line 823) (column 43) (len 11)))))
    (reference r685 (scope relative) (span (offset 53822) (line 824) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 53822) (line 824) (column 37) (len 19)))))
    (reference r686 (scope relative) (span (offset 53851) (line 824) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 53851) (line 824) (column 66) (len 8)))))
    (reference r687 (scope relative) (span (offset 53862) (line 824) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 53862) (line 824) (column 77) (len 3)))))
    (reference r688 (scope relative) (span (offset 53866) (line 824) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 53866) (line 824) (column 81) (len 1)))))
    (reference r689 (scope relative) (span (offset 53873) (line 824) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 53873) (line 824) (column 88) (len 8)))))
    (reference r690 (scope relative) (span (offset 53928) (line 825) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 53928) (line 825) (column 39) (len 19)))))
    (reference r691 (scope relative) (span (offset 53957) (line 825) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 53957) (line 825) (column 68) (len 8)))))
    (reference r692 (scope relative) (span (offset 53968) (line 825) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 53968) (line 825) (column 79) (len 3)))))
    (reference r693 (scope relative) (span (offset 53972) (line 825) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 53972) (line 825) (column 83) (len 1)))))
    (reference r694 (scope relative) (span (offset 53979) (line 825) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 53979) (line 825) (column 90) (len 8)))))
    (reference r695 (scope relative) (span (offset 54042) (line 826) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 54042) (line 826) (column 48) (len 19)))))
    (reference r696 (scope relative) (span (offset 54071) (line 826) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 54071) (line 826) (column 77) (len 8)))))
    (reference r697 (scope relative) (span (offset 54082) (line 826) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 54082) (line 826) (column 88) (len 3)))))
    (reference r698 (scope relative) (span (offset 54086) (line 826) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 54086) (line 826) (column 92) (len 1)))))
    (reference r699 (scope relative) (span (offset 54093) (line 826) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 54093) (line 826) (column 99) (len 8)))))
    (reference r700 (scope relative) (span (offset 54131) (line 827) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 54131) (line 827) (column 23) (len 17)))))
    (reference r701 (scope relative) (span (offset 54155) (line 827) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 54155) (line 827) (column 47) (len 20)))))
    (reference r702 (scope relative) (span (offset 54179) (line 827) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 54179) (line 827) (column 71) (len 8)))))
    (reference r703 (scope relative) (span (offset 54189) (line 827) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 54189) (line 827) (column 81) (len 10)))))
    (reference r704 (scope relative) (span (offset 54201) (line 827) (column 93) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 54201) (line 827) (column 93) (len 19)))))
    (reference r705 (scope relative) (span (offset 54273) (line 830) (column 42) (len 20)) (segments (segment 0 (token "LuminousExposureUnit") (name "LuminousExposureUnit") (separator none) (span (offset 54273) (line 830) (column 42) (len 20)))))
    (reference r706 (scope relative) (span (offset 54337) (line 831) (column 43) (len 21)) (segments (segment 0 (token "LuminousExposureValue") (name "LuminousExposureValue") (separator none) (span (offset 54337) (line 831) (column 43) (len 21)))))
    (reference r707 (scope relative) (span (offset 54397) (line 832) (column 38) (len 16)) (segments (segment 0 (token "luminousExposure") (name "luminousExposure") (separator none) (span (offset 54397) (line 832) (column 38) (len 16)))))
    (reference r708 (scope relative) (span (offset 54448) (line 834) (column 33) (len 20)) (segments (segment 0 (token "LuminousExposureUnit") (name "LuminousExposureUnit") (separator none) (span (offset 54448) (line 834) (column 33) (len 20)))))
    (reference r709 (scope relative) (span (offset 54503) (line 835) (column 34) (len 21)) (segments (segment 0 (token "LuminousExposureValue") (name "LuminousExposureValue") (separator none) (span (offset 54503) (line 835) (column 34) (len 21)))))
    (reference r710 (scope relative) (span (offset 54554) (line 836) (column 29) (len 16)) (segments (segment 0 (token "luminousExposure") (name "luminousExposure") (separator none) (span (offset 54554) (line 836) (column 29) (len 16)))))
    (reference r711 (scope relative) (span (offset 54679) (line 839) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 54679) (line 839) (column 40) (len 17)))))
    (reference r712 (scope relative) (span (offset 55501) (line 853) (column 29) (len 17)) (segments (segment 0 (token "PhotonNumberValue") (name "PhotonNumberValue") (separator none) (span (offset 55501) (line 853) (column 29) (len 17)))))
    (reference r713 (scope relative) (span (offset 55571) (line 855) (column 31) (len 12)) (segments (segment 0 (token "photonNumber") (name "photonNumber") (separator none) (span (offset 55571) (line 855) (column 31) (len 12)))))
    (reference r714 (scope relative) (span (offset 55662) (line 858) (column 29) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 55662) (line 858) (column 29) (len 11)))))
    (reference r715 (scope relative) (span (offset 56644) (line 874) (column 38) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 56644) (line 874) (column 38) (len 19)))))
    (reference r716 (scope relative) (span (offset 57599) (line 887) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 57599) (line 887) (column 28) (len 4)))))
    (reference r717 (scope relative) (span (offset 57594) (line 887) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 57594) (line 887) (column 23) (len 3)))))
    (reference r718 (scope relative) (span (offset 57633) (line 888) (column 29) (len 14)) (segments (segment 0 (token "PhotonFluxUnit") (name "PhotonFluxUnit") (separator none) (span (offset 57633) (line 888) (column 29) (len 14)))))
    (reference r719 (scope relative) (span (offset 57627) (line 888) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 57627) (line 888) (column 23) (len 4)))))
    (reference r720 (scope relative) (span (offset 57685) (line 891) (column 27) (len 15)) (segments (segment 0 (token "PhotonFluxValue") (name "PhotonFluxValue") (separator none) (span (offset 57685) (line 891) (column 27) (len 15)))))
    (reference r721 (scope relative) (span (offset 57772) (line 893) (column 37) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 57772) (line 893) (column 37) (len 11)))))
    (reference r722 (scope relative) (span (offset 57824) (line 894) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57824) (line 894) (column 39) (len 19)))))
    (reference r723 (scope relative) (span (offset 57853) (line 894) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57853) (line 894) (column 68) (len 8)))))
    (reference r724 (scope relative) (span (offset 57864) (line 894) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57864) (line 894) (column 79) (len 3)))))
    (reference r725 (scope relative) (span (offset 57868) (line 894) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 57868) (line 894) (column 83) (len 1)))))
    (reference r726 (scope relative) (span (offset 57875) (line 894) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57875) (line 894) (column 90) (len 8)))))
    (reference r727 (scope relative) (span (offset 57914) (line 895) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 57914) (line 895) (column 23) (len 17)))))
    (reference r728 (scope relative) (span (offset 57938) (line 895) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 57938) (line 895) (column 47) (len 20)))))
    (reference r729 (scope relative) (span (offset 57961) (line 895) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 57961) (line 895) (column 70) (len 10)))))
    (reference r730 (scope relative) (span (offset 58073) (line 899) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 58073) (line 899) (column 43) (len 19)))))
    (reference r731 (scope relative) (span (offset 59156) (line 912) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 59156) (line 912) (column 28) (len 4)))))
    (reference r732 (scope relative) (span (offset 59151) (line 912) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 59151) (line 912) (column 23) (len 3)))))
    (reference r733 (scope relative) (span (offset 59190) (line 913) (column 29) (len 19)) (segments (segment 0 (token "PhotonIntensityUnit") (name "PhotonIntensityUnit") (separator none) (span (offset 59190) (line 913) (column 29) (len 19)))))
    (reference r734 (scope relative) (span (offset 59184) (line 913) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 59184) (line 913) (column 23) (len 4)))))
    (reference r735 (scope relative) (span (offset 59252) (line 916) (column 32) (len 20)) (segments (segment 0 (token "PhotonIntensityValue") (name "PhotonIntensityValue") (separator none) (span (offset 59252) (line 916) (column 32) (len 20)))))
    (reference r736 (scope relative) (span (offset 59349) (line 918) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 59349) (line 918) (column 42) (len 11)))))
    (reference r737 (scope relative) (span (offset 59401) (line 919) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 59401) (line 919) (column 39) (len 19)))))
    (reference r738 (scope relative) (span (offset 59430) (line 919) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59430) (line 919) (column 68) (len 8)))))
    (reference r739 (scope relative) (span (offset 59441) (line 919) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59441) (line 919) (column 79) (len 3)))))
    (reference r740 (scope relative) (span (offset 59445) (line 919) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 59445) (line 919) (column 83) (len 1)))))
    (reference r741 (scope relative) (span (offset 59452) (line 919) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59452) (line 919) (column 90) (len 8)))))
    (reference r742 (scope relative) (span (offset 59491) (line 920) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 59491) (line 920) (column 23) (len 17)))))
    (reference r743 (scope relative) (span (offset 59515) (line 920) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 59515) (line 920) (column 47) (len 20)))))
    (reference r744 (scope relative) (span (offset 59538) (line 920) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 59538) (line 920) (column 70) (len 10)))))
    (reference r745 (scope relative) (span (offset 59648) (line 924) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 59648) (line 924) (column 42) (len 19)))))
    (reference r746 (scope relative) (span (offset 60532) (line 937) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 60532) (line 937) (column 28) (len 4)))))
    (reference r747 (scope relative) (span (offset 60527) (line 937) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 60527) (line 937) (column 23) (len 3)))))
    (reference r748 (scope relative) (span (offset 60566) (line 938) (column 29) (len 18)) (segments (segment 0 (token "PhotonRadianceUnit") (name "PhotonRadianceUnit") (separator none) (span (offset 60566) (line 938) (column 29) (len 18)))))
    (reference r749 (scope relative) (span (offset 60560) (line 938) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 60560) (line 938) (column 23) (len 4)))))
    (reference r750 (scope relative) (span (offset 60626) (line 941) (column 31) (len 19)) (segments (segment 0 (token "PhotonRadianceValue") (name "PhotonRadianceValue") (separator none) (span (offset 60626) (line 941) (column 31) (len 19)))))
    (reference r751 (scope relative) (span (offset 60721) (line 943) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 60721) (line 943) (column 41) (len 11)))))
    (reference r752 (scope relative) (span (offset 60771) (line 944) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 60771) (line 944) (column 37) (len 19)))))
    (reference r753 (scope relative) (span (offset 60800) (line 944) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 60800) (line 944) (column 66) (len 8)))))
    (reference r754 (scope relative) (span (offset 60811) (line 944) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 60811) (line 944) (column 77) (len 3)))))
    (reference r755 (scope relative) (span (offset 60815) (line 944) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 60815) (line 944) (column 81) (len 1)))))
    (reference r756 (scope relative) (span (offset 60822) (line 944) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 60822) (line 944) (column 88) (len 8)))))
    (reference r757 (scope relative) (span (offset 60877) (line 945) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 60877) (line 945) (column 39) (len 19)))))
    (reference r758 (scope relative) (span (offset 60906) (line 945) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 60906) (line 945) (column 68) (len 8)))))
    (reference r759 (scope relative) (span (offset 60917) (line 945) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 60917) (line 945) (column 79) (len 3)))))
    (reference r760 (scope relative) (span (offset 60921) (line 945) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 60921) (line 945) (column 83) (len 1)))))
    (reference r761 (scope relative) (span (offset 60928) (line 945) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 60928) (line 945) (column 90) (len 8)))))
    (reference r762 (scope relative) (span (offset 60967) (line 946) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 60967) (line 946) (column 23) (len 17)))))
    (reference r763 (scope relative) (span (offset 60991) (line 946) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 60991) (line 946) (column 47) (len 20)))))
    (reference r764 (scope relative) (span (offset 61015) (line 946) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 61015) (line 946) (column 71) (len 8)))))
    (reference r765 (scope relative) (span (offset 61025) (line 946) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 61025) (line 946) (column 81) (len 10)))))
    (reference r766 (scope relative) (span (offset 61140) (line 950) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 61140) (line 950) (column 44) (len 19)))))
    (reference r767 (scope relative) (span (offset 61912) (line 963) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 61912) (line 963) (column 28) (len 4)))))
    (reference r768 (scope relative) (span (offset 61907) (line 963) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 61907) (line 963) (column 23) (len 3)))))
    (reference r769 (scope relative) (span (offset 61946) (line 964) (column 29) (len 20)) (segments (segment 0 (token "PhotonIrradianceUnit") (name "PhotonIrradianceUnit") (separator none) (span (offset 61946) (line 964) (column 29) (len 20)))))
    (reference r770 (scope relative) (span (offset 61940) (line 964) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 61940) (line 964) (column 23) (len 4)))))
    (reference r771 (scope relative) (span (offset 62010) (line 967) (column 33) (len 21)) (segments (segment 0 (token "PhotonIrradianceValue") (name "PhotonIrradianceValue") (separator none) (span (offset 62010) (line 967) (column 33) (len 21)))))
    (reference r772 (scope relative) (span (offset 62109) (line 969) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 62109) (line 969) (column 43) (len 11)))))
    (reference r773 (scope relative) (span (offset 62159) (line 970) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 62159) (line 970) (column 37) (len 19)))))
    (reference r774 (scope relative) (span (offset 62188) (line 970) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 62188) (line 970) (column 66) (len 8)))))
    (reference r775 (scope relative) (span (offset 62199) (line 970) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 62199) (line 970) (column 77) (len 3)))))
    (reference r776 (scope relative) (span (offset 62203) (line 970) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 62203) (line 970) (column 81) (len 1)))))
    (reference r777 (scope relative) (span (offset 62210) (line 970) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 62210) (line 970) (column 88) (len 8)))))
    (reference r778 (scope relative) (span (offset 62265) (line 971) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 62265) (line 971) (column 39) (len 19)))))
    (reference r779 (scope relative) (span (offset 62294) (line 971) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 62294) (line 971) (column 68) (len 8)))))
    (reference r780 (scope relative) (span (offset 62305) (line 971) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 62305) (line 971) (column 79) (len 3)))))
    (reference r781 (scope relative) (span (offset 62309) (line 971) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 62309) (line 971) (column 83) (len 1)))))
    (reference r782 (scope relative) (span (offset 62316) (line 971) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 62316) (line 971) (column 90) (len 8)))))
    (reference r783 (scope relative) (span (offset 62355) (line 972) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 62355) (line 972) (column 23) (len 17)))))
    (reference r784 (scope relative) (span (offset 62379) (line 972) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 62379) (line 972) (column 47) (len 20)))))
    (reference r785 (scope relative) (span (offset 62403) (line 972) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 62403) (line 972) (column 71) (len 8)))))
    (reference r786 (scope relative) (span (offset 62413) (line 972) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 62413) (line 972) (column 81) (len 10)))))
    (reference r787 (scope relative) (span (offset 62524) (line 976) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 62524) (line 976) (column 42) (len 19)))))
    (reference r788 (scope relative) (span (offset 63300) (line 989) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 63300) (line 989) (column 28) (len 4)))))
    (reference r789 (scope relative) (span (offset 63295) (line 989) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 63295) (line 989) (column 23) (len 3)))))
    (reference r790 (scope relative) (span (offset 63334) (line 990) (column 29) (len 18)) (segments (segment 0 (token "PhotonExitanceUnit") (name "PhotonExitanceUnit") (separator none) (span (offset 63334) (line 990) (column 29) (len 18)))))
    (reference r791 (scope relative) (span (offset 63328) (line 990) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 63328) (line 990) (column 23) (len 4)))))
    (reference r792 (scope relative) (span (offset 63394) (line 993) (column 31) (len 19)) (segments (segment 0 (token "PhotonExitanceValue") (name "PhotonExitanceValue") (separator none) (span (offset 63394) (line 993) (column 31) (len 19)))))
    (reference r793 (scope relative) (span (offset 63489) (line 995) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 63489) (line 995) (column 41) (len 11)))))
    (reference r794 (scope relative) (span (offset 63539) (line 996) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 63539) (line 996) (column 37) (len 19)))))
    (reference r795 (scope relative) (span (offset 63568) (line 996) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 63568) (line 996) (column 66) (len 8)))))
    (reference r796 (scope relative) (span (offset 63579) (line 996) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 63579) (line 996) (column 77) (len 3)))))
    (reference r797 (scope relative) (span (offset 63583) (line 996) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 63583) (line 996) (column 81) (len 1)))))
    (reference r798 (scope relative) (span (offset 63590) (line 996) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 63590) (line 996) (column 88) (len 8)))))
    (reference r799 (scope relative) (span (offset 63645) (line 997) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 63645) (line 997) (column 39) (len 19)))))
    (reference r800 (scope relative) (span (offset 63674) (line 997) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 63674) (line 997) (column 68) (len 8)))))
    (reference r801 (scope relative) (span (offset 63685) (line 997) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 63685) (line 997) (column 79) (len 3)))))
    (reference r802 (scope relative) (span (offset 63689) (line 997) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 63689) (line 997) (column 83) (len 1)))))
    (reference r803 (scope relative) (span (offset 63696) (line 997) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 63696) (line 997) (column 90) (len 8)))))
    (reference r804 (scope relative) (span (offset 63735) (line 998) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 63735) (line 998) (column 23) (len 17)))))
    (reference r805 (scope relative) (span (offset 63759) (line 998) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 63759) (line 998) (column 47) (len 20)))))
    (reference r806 (scope relative) (span (offset 63783) (line 998) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 63783) (line 998) (column 71) (len 8)))))
    (reference r807 (scope relative) (span (offset 63793) (line 998) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 63793) (line 998) (column 81) (len 10)))))
    (reference r808 (scope relative) (span (offset 63904) (line 1002) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 63904) (line 1002) (column 42) (len 19)))))
    (reference r809 (scope relative) (span (offset 64675) (line 1015) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 64675) (line 1015) (column 28) (len 4)))))
    (reference r810 (scope relative) (span (offset 64670) (line 1015) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 64670) (line 1015) (column 23) (len 3)))))
    (reference r811 (scope relative) (span (offset 64709) (line 1016) (column 29) (len 18)) (segments (segment 0 (token "PhotonExposureUnit") (name "PhotonExposureUnit") (separator none) (span (offset 64709) (line 1016) (column 29) (len 18)))))
    (reference r812 (scope relative) (span (offset 64703) (line 1016) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 64703) (line 1016) (column 23) (len 4)))))
    (reference r813 (scope relative) (span (offset 64769) (line 1019) (column 31) (len 19)) (segments (segment 0 (token "PhotonExposureValue") (name "PhotonExposureValue") (separator none) (span (offset 64769) (line 1019) (column 31) (len 19)))))
    (reference r814 (scope relative) (span (offset 64864) (line 1021) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 64864) (line 1021) (column 41) (len 11)))))
    (reference r815 (scope relative) (span (offset 64914) (line 1022) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 64914) (line 1022) (column 37) (len 19)))))
    (reference r816 (scope relative) (span (offset 64943) (line 1022) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 64943) (line 1022) (column 66) (len 8)))))
    (reference r817 (scope relative) (span (offset 64954) (line 1022) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 64954) (line 1022) (column 77) (len 3)))))
    (reference r818 (scope relative) (span (offset 64958) (line 1022) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 64958) (line 1022) (column 81) (len 1)))))
    (reference r819 (scope relative) (span (offset 64965) (line 1022) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 64965) (line 1022) (column 88) (len 8)))))
    (reference r820 (scope relative) (span (offset 65004) (line 1023) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 65004) (line 1023) (column 23) (len 17)))))
    (reference r821 (scope relative) (span (offset 65028) (line 1023) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 65028) (line 1023) (column 47) (len 20)))))
    (reference r822 (scope relative) (span (offset 65051) (line 1023) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 65051) (line 1023) (column 70) (len 8)))))
    (reference r823 (scope relative) (span (offset 65256) (line 1027) (column 86) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 65256) (line 1027) (column 86) (len 19)))))
    (reference r824 (scope relative) (span (offset 67086) (line 1040) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 67086) (line 1040) (column 28) (len 4)))))
    (reference r825 (scope relative) (span (offset 67081) (line 1040) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 67081) (line 1040) (column 23) (len 3)))))
    (reference r826 (scope relative) (span (offset 67120) (line 1041) (column 29) (len 62)) (segments (segment 0 (token "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (separator none) (span (offset 67120) (line 1041) (column 29) (len 62)))))
    (reference r827 (scope relative) (span (offset 67114) (line 1041) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 67114) (line 1041) (column 23) (len 4)))))
    (reference r828 (scope relative) (span (offset 67268) (line 1044) (column 75) (len 63)) (segments (segment 0 (token "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (separator none) (span (offset 67268) (line 1044) (column 75) (len 63)))))
    (reference r829 (scope relative) (span (offset 67451) (line 1046) (column 85) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 67451) (line 1046) (column 85) (len 11)))))
    (reference r830 (scope relative) (span (offset 67501) (line 1047) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 67501) (line 1047) (column 37) (len 19)))))
    (reference r831 (scope relative) (span (offset 67530) (line 1047) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 67530) (line 1047) (column 66) (len 8)))))
    (reference r832 (scope relative) (span (offset 67541) (line 1047) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 67541) (line 1047) (column 77) (len 3)))))
    (reference r833 (scope relative) (span (offset 67545) (line 1047) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 67545) (line 1047) (column 81) (len 1)))))
    (reference r834 (scope relative) (span (offset 67552) (line 1047) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 67552) (line 1047) (column 88) (len 8)))))
    (reference r835 (scope relative) (span (offset 67616) (line 1048) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 67616) (line 1048) (column 48) (len 19)))))
    (reference r836 (scope relative) (span (offset 67645) (line 1048) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 67645) (line 1048) (column 77) (len 8)))))
    (reference r837 (scope relative) (span (offset 67656) (line 1048) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 67656) (line 1048) (column 88) (len 3)))))
    (reference r838 (scope relative) (span (offset 67660) (line 1048) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 67660) (line 1048) (column 92) (len 1)))))
    (reference r839 (scope relative) (span (offset 67667) (line 1048) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 67667) (line 1048) (column 99) (len 8)))))
    (reference r840 (scope relative) (span (offset 67705) (line 1049) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 67705) (line 1049) (column 23) (len 17)))))
    (reference r841 (scope relative) (span (offset 67729) (line 1049) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 67729) (line 1049) (column 47) (len 20)))))
    (reference r842 (scope relative) (span (offset 67753) (line 1049) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 67753) (line 1049) (column 71) (len 8)))))
    (reference r843 (scope relative) (span (offset 67763) (line 1049) (column 81) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 67763) (line 1049) (column 81) (len 19)))))
    (reference r844 (scope relative) (span (offset 67980) (line 1053) (column 86) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 67980) (line 1053) (column 86) (len 19)))))
    (reference r845 (scope relative) (span (offset 69824) (line 1066) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 69824) (line 1066) (column 28) (len 4)))))
    (reference r846 (scope relative) (span (offset 69819) (line 1066) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 69819) (line 1066) (column 23) (len 3)))))
    (reference r847 (scope relative) (span (offset 69858) (line 1067) (column 29) (len 62)) (segments (segment 0 (token "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (separator none) (span (offset 69858) (line 1067) (column 29) (len 62)))))
    (reference r848 (scope relative) (span (offset 69852) (line 1067) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 69852) (line 1067) (column 23) (len 4)))))
    (reference r849 (scope relative) (span (offset 70006) (line 1070) (column 75) (len 63)) (segments (segment 0 (token "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (separator none) (span (offset 70006) (line 1070) (column 75) (len 63)))))
    (reference r850 (scope relative) (span (offset 70189) (line 1072) (column 85) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 70189) (line 1072) (column 85) (len 11)))))
    (reference r851 (scope relative) (span (offset 70239) (line 1073) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 70239) (line 1073) (column 37) (len 19)))))
    (reference r852 (scope relative) (span (offset 70268) (line 1073) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 70268) (line 1073) (column 66) (len 8)))))
    (reference r853 (scope relative) (span (offset 70279) (line 1073) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 70279) (line 1073) (column 77) (len 3)))))
    (reference r854 (scope relative) (span (offset 70283) (line 1073) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 70283) (line 1073) (column 81) (len 1)))))
    (reference r855 (scope relative) (span (offset 70290) (line 1073) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 70290) (line 1073) (column 88) (len 8)))))
    (reference r856 (scope relative) (span (offset 70354) (line 1074) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 70354) (line 1074) (column 48) (len 19)))))
    (reference r857 (scope relative) (span (offset 70383) (line 1074) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 70383) (line 1074) (column 77) (len 8)))))
    (reference r858 (scope relative) (span (offset 70394) (line 1074) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 70394) (line 1074) (column 88) (len 3)))))
    (reference r859 (scope relative) (span (offset 70398) (line 1074) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 70398) (line 1074) (column 92) (len 1)))))
    (reference r860 (scope relative) (span (offset 70405) (line 1074) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 70405) (line 1074) (column 99) (len 8)))))
    (reference r861 (scope relative) (span (offset 70443) (line 1075) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 70443) (line 1075) (column 23) (len 17)))))
    (reference r862 (scope relative) (span (offset 70467) (line 1075) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 70467) (line 1075) (column 47) (len 20)))))
    (reference r863 (scope relative) (span (offset 70491) (line 1075) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 70491) (line 1075) (column 71) (len 8)))))
    (reference r864 (scope relative) (span (offset 70501) (line 1075) (column 81) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 70501) (line 1075) (column 81) (len 19)))))
    (reference r865 (scope relative) (span (offset 70738) (line 1079) (column 95) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 70738) (line 1079) (column 95) (len 17)))))
    (reference r866 (scope relative) (span (offset 71687) (line 1093) (column 84) (len 72)) (segments (segment 0 (token "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (separator none) (span (offset 71687) (line 1093) (column 84) (len 72)))))
    (reference r867 (scope relative) (span (offset 71988) (line 1096) (column 95) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 71988) (line 1096) (column 95) (len 17)))))
    (reference r868 (scope relative) (span (offset 72959) (line 1110) (column 84) (len 72)) (segments (segment 0 (token "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (separator none) (span (offset 72959) (line 1110) (column 84) (len 72)))))
    (reference r869 (scope relative) (span (offset 73246) (line 1113) (column 89) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 73246) (line 1113) (column 89) (len 17)))))
    (reference r870 (scope relative) (span (offset 74112) (line 1127) (column 78) (len 66)) (segments (segment 0 (token "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (separator none) (span (offset 74112) (line 1127) (column 78) (len 66)))))
    (reference r871 (scope relative) (span (offset 74393) (line 1130) (column 89) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 74393) (line 1130) (column 89) (len 17)))))
    (reference r872 (scope relative) (span (offset 75320) (line 1144) (column 78) (len 66)) (segments (segment 0 (token "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (separator none) (span (offset 75320) (line 1144) (column 78) (len 66)))))
    (reference r873 (scope relative) (span (offset 75495) (line 1147) (column 34) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 75495) (line 1147) (column 34) (len 29)))))
    (reference r874 (scope relative) (span (offset 76139) (line 1163) (column 44) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 76139) (line 1163) (column 44) (len 29)))))
    (reference r875 (scope relative) (span (offset 76956) (line 1179) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 76956) (line 1179) (column 38) (len 17)))))
    (reference r876 (scope relative) (span (offset 77678) (line 1193) (column 27) (len 15)) (segments (segment 0 (token "EmissivityValue") (name "EmissivityValue") (separator none) (span (offset 77678) (line 1193) (column 27) (len 15)))))
    (reference r877 (scope relative) (span (offset 77846) (line 1196) (column 60) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 77846) (line 1196) (column 60) (len 17)))))
    (reference r878 (scope relative) (span (offset 78761) (line 1210) (column 49) (len 37)) (segments (segment 0 (token "EmissivityAtASpecifiedWavelengthValue") (name "EmissivityAtASpecifiedWavelengthValue") (separator none) (span (offset 78761) (line 1210) (column 49) (len 37)))))
    (reference r879 (scope relative) (span (offset 78905) (line 1213) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 78905) (line 1213) (column 39) (len 17)))))
    (reference r880 (scope relative) (span (offset 79785) (line 1227) (column 28) (len 16)) (segments (segment 0 (token "AbsorptanceValue") (name "AbsorptanceValue") (separator none) (span (offset 79785) (line 1227) (column 28) (len 16)))))
    (reference r881 (scope relative) (span (offset 79925) (line 1230) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 79925) (line 1230) (column 47) (len 17)))))
    (reference r882 (scope relative) (span (offset 80897) (line 1244) (column 36) (len 24)) (segments (segment 0 (token "LuminousAbsorptanceValue") (name "LuminousAbsorptanceValue") (separator none) (span (offset 80897) (line 1244) (column 36) (len 24)))))
    (reference r883 (scope relative) (span (offset 81028) (line 1247) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 81028) (line 1247) (column 39) (len 17)))))
    (reference r884 (scope relative) (span (offset 81906) (line 1261) (column 28) (len 16)) (segments (segment 0 (token "ReflectanceValue") (name "ReflectanceValue") (separator none) (span (offset 81906) (line 1261) (column 28) (len 16)))))
    (reference r885 (scope relative) (span (offset 82046) (line 1264) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 82046) (line 1264) (column 47) (len 17)))))
    (reference r886 (scope relative) (span (offset 83023) (line 1278) (column 36) (len 24)) (segments (segment 0 (token "LuminousReflectanceValue") (name "LuminousReflectanceValue") (separator none) (span (offset 83023) (line 1278) (column 36) (len 24)))))
    (reference r887 (scope relative) (span (offset 83158) (line 1281) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 83158) (line 1281) (column 41) (len 17)))))
    (reference r888 (scope relative) (span (offset 84049) (line 1295) (column 30) (len 18)) (segments (segment 0 (token "TransmittanceValue") (name "TransmittanceValue") (separator none) (span (offset 84049) (line 1295) (column 30) (len 18)))))
    (reference r889 (scope relative) (span (offset 84195) (line 1298) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 84195) (line 1298) (column 49) (len 17)))))
    (reference r890 (scope relative) (span (offset 85211) (line 1312) (column 38) (len 26)) (segments (segment 0 (token "LuminousTransmittanceValue") (name "LuminousTransmittanceValue") (separator none) (span (offset 85211) (line 1312) (column 38) (len 26)))))
    (reference r891 (scope relative) (span (offset 85438) (line 1315) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 85438) (line 1315) (column 55) (len 17)))))
    (reference r892 (scope relative) (span (offset 86282) (line 1329) (column 44) (len 32)) (segments (segment 0 (token "TransmittanceOpticalDensityValue") (name "TransmittanceOpticalDensityValue") (separator none) (span (offset 86282) (line 1329) (column 44) (len 32)))))
    (reference r893 (scope relative) (span (offset 86366) (line 1331) (column 30) (len 27)) (segments (segment 0 (token "transmittanceOpticalDensity") (name "transmittanceOpticalDensity") (separator none) (span (offset 86366) (line 1331) (column 30) (len 27)))))
    (reference r894 (scope relative) (span (offset 86431) (line 1333) (column 36) (len 27)) (segments (segment 0 (token "transmittanceOpticalDensity") (name "transmittanceOpticalDensity") (separator none) (span (offset 86431) (line 1333) (column 36) (len 27)))))
    (reference r895 (scope relative) (span (offset 86493) (line 1335) (column 33) (len 27)) (segments (segment 0 (token "transmittanceOpticalDensity") (name "transmittanceOpticalDensity") (separator none) (span (offset 86493) (line 1335) (column 33) (len 27)))))
    (reference r896 (scope relative) (span (offset 86624) (line 1338) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 86624) (line 1338) (column 47) (len 17)))))
    (reference r897 (scope relative) (span (offset 87407) (line 1352) (column 36) (len 24)) (segments (segment 0 (token "NapierianAbsorbanceValue") (name "NapierianAbsorbanceValue") (separator none) (span (offset 87407) (line 1352) (column 36) (len 24)))))
    (reference r898 (scope relative) (span (offset 87545) (line 1355) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 87545) (line 1355) (column 42) (len 17)))))
    (reference r899 (scope relative) (span (offset 89037) (line 1369) (column 31) (len 19)) (segments (segment 0 (token "RadianceFactorValue") (name "RadianceFactorValue") (separator none) (span (offset 89037) (line 1369) (column 31) (len 19)))))
    (reference r900 (scope relative) (span (offset 89172) (line 1372) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 89172) (line 1372) (column 43) (len 17)))))
    (reference r901 (scope relative) (span (offset 90322) (line 1386) (column 32) (len 20)) (segments (segment 0 (token "LuminanceFactorValue") (name "LuminanceFactorValue") (separator none) (span (offset 90322) (line 1386) (column 32) (len 20)))))
    (reference r902 (scope relative) (span (offset 90460) (line 1389) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 90460) (line 1389) (column 45) (len 17)))))
    (reference r903 (scope relative) (span (offset 92159) (line 1403) (column 34) (len 22)) (segments (segment 0 (token "ReflectanceFactorValue") (name "ReflectanceFactorValue") (separator none) (span (offset 92159) (line 1403) (column 34) (len 22)))))
    (reference r904 (scope relative) (span (offset 92355) (line 1406) (column 56) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 92355) (line 1406) (column 56) (len 19)))))
    (reference r905 (scope relative) (span (offset 93350) (line 1419) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 93350) (line 1419) (column 28) (len 4)))))
    (reference r906 (scope relative) (span (offset 93345) (line 1419) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 93345) (line 1419) (column 23) (len 3)))))
    (reference r907 (scope relative) (span (offset 93384) (line 1420) (column 29) (len 32)) (segments (segment 0 (token "LinearAttenuationCoefficientUnit") (name "LinearAttenuationCoefficientUnit") (separator none) (span (offset 93384) (line 1420) (column 29) (len 32)))))
    (reference r908 (scope relative) (span (offset 93378) (line 1420) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 93378) (line 1420) (column 23) (len 4)))))
    (reference r909 (scope relative) (span (offset 93472) (line 1423) (column 45) (len 33)) (segments (segment 0 (token "LinearAttenuationCoefficientValue") (name "LinearAttenuationCoefficientValue") (separator none) (span (offset 93472) (line 1423) (column 45) (len 33)))))
    (reference r910 (scope relative) (span (offset 93595) (line 1425) (column 55) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 93595) (line 1425) (column 55) (len 11)))))
    (reference r911 (scope relative) (span (offset 93645) (line 1426) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 93645) (line 1426) (column 37) (len 19)))))
    (reference r912 (scope relative) (span (offset 93674) (line 1426) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 93674) (line 1426) (column 66) (len 8)))))
    (reference r913 (scope relative) (span (offset 93685) (line 1426) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 93685) (line 1426) (column 77) (len 3)))))
    (reference r914 (scope relative) (span (offset 93689) (line 1426) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 93689) (line 1426) (column 81) (len 1)))))
    (reference r915 (scope relative) (span (offset 93696) (line 1426) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 93696) (line 1426) (column 88) (len 8)))))
    (reference r916 (scope relative) (span (offset 93735) (line 1427) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 93735) (line 1427) (column 23) (len 17)))))
    (reference r917 (scope relative) (span (offset 93759) (line 1427) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 93759) (line 1427) (column 47) (len 20)))))
    (reference r918 (scope relative) (span (offset 93782) (line 1427) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 93782) (line 1427) (column 70) (len 8)))))
    (reference r919 (scope relative) (span (offset 93847) (line 1430) (column 47) (len 32)) (segments (segment 0 (token "LinearAttenuationCoefficientUnit") (name "LinearAttenuationCoefficientUnit") (separator none) (span (offset 93847) (line 1430) (column 47) (len 32)))))
    (reference r920 (scope relative) (span (offset 93928) (line 1431) (column 48) (len 33)) (segments (segment 0 (token "LinearAttenuationCoefficientValue") (name "LinearAttenuationCoefficientValue") (separator none) (span (offset 93928) (line 1431) (column 48) (len 33)))))
    (reference r921 (scope relative) (span (offset 94005) (line 1432) (column 43) (len 28)) (segments (segment 0 (token "linearAttenuationCoefficient") (name "linearAttenuationCoefficient") (separator none) (span (offset 94005) (line 1432) (column 43) (len 28)))))
    (reference r922 (scope relative) (span (offset 94154) (line 1435) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 94154) (line 1435) (column 55) (len 19)))))
    (reference r923 (scope relative) (span (offset 95366) (line 1448) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 95366) (line 1448) (column 28) (len 4)))))
    (reference r924 (scope relative) (span (offset 95361) (line 1448) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 95361) (line 1448) (column 23) (len 3)))))
    (reference r925 (scope relative) (span (offset 95400) (line 1449) (column 29) (len 31)) (segments (segment 0 (token "LinearAbsorptionCoefficientUnit") (name "LinearAbsorptionCoefficientUnit") (separator none) (span (offset 95400) (line 1449) (column 29) (len 31)))))
    (reference r926 (scope relative) (span (offset 95394) (line 1449) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 95394) (line 1449) (column 23) (len 4)))))
    (reference r927 (scope relative) (span (offset 95486) (line 1452) (column 44) (len 32)) (segments (segment 0 (token "LinearAbsorptionCoefficientValue") (name "LinearAbsorptionCoefficientValue") (separator none) (span (offset 95486) (line 1452) (column 44) (len 32)))))
    (reference r928 (scope relative) (span (offset 95607) (line 1454) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 95607) (line 1454) (column 54) (len 11)))))
    (reference r929 (scope relative) (span (offset 95657) (line 1455) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 95657) (line 1455) (column 37) (len 19)))))
    (reference r930 (scope relative) (span (offset 95686) (line 1455) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 95686) (line 1455) (column 66) (len 8)))))
    (reference r931 (scope relative) (span (offset 95697) (line 1455) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 95697) (line 1455) (column 77) (len 3)))))
    (reference r932 (scope relative) (span (offset 95701) (line 1455) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 95701) (line 1455) (column 81) (len 1)))))
    (reference r933 (scope relative) (span (offset 95708) (line 1455) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 95708) (line 1455) (column 88) (len 8)))))
    (reference r934 (scope relative) (span (offset 95747) (line 1456) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 95747) (line 1456) (column 23) (len 17)))))
    (reference r935 (scope relative) (span (offset 95771) (line 1456) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 95771) (line 1456) (column 47) (len 20)))))
    (reference r936 (scope relative) (span (offset 95794) (line 1456) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 95794) (line 1456) (column 70) (len 8)))))
    (reference r937 (scope relative) (span (offset 95929) (line 1460) (column 54) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 95929) (line 1460) (column 54) (len 19)))))
    (reference r938 (scope relative) (span (offset 96703) (line 1473) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 96703) (line 1473) (column 28) (len 4)))))
    (reference r939 (scope relative) (span (offset 96698) (line 1473) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 96698) (line 1473) (column 23) (len 3)))))
    (reference r940 (scope relative) (span (offset 96737) (line 1474) (column 29) (len 30)) (segments (segment 0 (token "MassAttenuationCoefficientUnit") (name "MassAttenuationCoefficientUnit") (separator none) (span (offset 96737) (line 1474) (column 29) (len 30)))))
    (reference r941 (scope relative) (span (offset 96731) (line 1474) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 96731) (line 1474) (column 23) (len 4)))))
    (reference r942 (scope relative) (span (offset 96821) (line 1477) (column 43) (len 31)) (segments (segment 0 (token "MassAttenuationCoefficientValue") (name "MassAttenuationCoefficientValue") (separator none) (span (offset 96821) (line 1477) (column 43) (len 31)))))
    (reference r943 (scope relative) (span (offset 96940) (line 1479) (column 53) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 96940) (line 1479) (column 53) (len 11)))))
    (reference r944 (scope relative) (span (offset 96990) (line 1480) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 96990) (line 1480) (column 37) (len 19)))))
    (reference r945 (scope relative) (span (offset 97019) (line 1480) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 97019) (line 1480) (column 66) (len 8)))))
    (reference r946 (scope relative) (span (offset 97030) (line 1480) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 97030) (line 1480) (column 77) (len 3)))))
    (reference r947 (scope relative) (span (offset 97034) (line 1480) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 97034) (line 1480) (column 81) (len 1)))))
    (reference r948 (scope relative) (span (offset 97041) (line 1480) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 97041) (line 1480) (column 88) (len 8)))))
    (reference r949 (scope relative) (span (offset 97091) (line 1481) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 97091) (line 1481) (column 35) (len 19)))))
    (reference r950 (scope relative) (span (offset 97120) (line 1481) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 97120) (line 1481) (column 64) (len 8)))))
    (reference r951 (scope relative) (span (offset 97131) (line 1481) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 97131) (line 1481) (column 75) (len 3)))))
    (reference r952 (scope relative) (span (offset 97135) (line 1481) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 97135) (line 1481) (column 79) (len 1)))))
    (reference r953 (scope relative) (span (offset 97142) (line 1481) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 97142) (line 1481) (column 86) (len 8)))))
    (reference r954 (scope relative) (span (offset 97181) (line 1482) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 97181) (line 1482) (column 23) (len 17)))))
    (reference r955 (scope relative) (span (offset 97205) (line 1482) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 97205) (line 1482) (column 47) (len 20)))))
    (reference r956 (scope relative) (span (offset 97229) (line 1482) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 97229) (line 1482) (column 71) (len 8)))))
    (reference r957 (scope relative) (span (offset 97239) (line 1482) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 97239) (line 1482) (column 81) (len 6)))))
    (reference r958 (scope relative) (span (offset 97371) (line 1486) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 97371) (line 1486) (column 53) (len 19)))))
    (reference r959 (scope relative) (span (offset 98142) (line 1499) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 98142) (line 1499) (column 28) (len 4)))))
    (reference r960 (scope relative) (span (offset 98137) (line 1499) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 98137) (line 1499) (column 23) (len 3)))))
    (reference r961 (scope relative) (span (offset 98176) (line 1500) (column 29) (len 29)) (segments (segment 0 (token "MassAbsorptionCoefficientUnit") (name "MassAbsorptionCoefficientUnit") (separator none) (span (offset 98176) (line 1500) (column 29) (len 29)))))
    (reference r962 (scope relative) (span (offset 98170) (line 1500) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 98170) (line 1500) (column 23) (len 4)))))
    (reference r963 (scope relative) (span (offset 98258) (line 1503) (column 42) (len 30)) (segments (segment 0 (token "MassAbsorptionCoefficientValue") (name "MassAbsorptionCoefficientValue") (separator none) (span (offset 98258) (line 1503) (column 42) (len 30)))))
    (reference r964 (scope relative) (span (offset 98375) (line 1505) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 98375) (line 1505) (column 52) (len 11)))))
    (reference r965 (scope relative) (span (offset 98425) (line 1506) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 98425) (line 1506) (column 37) (len 19)))))
    (reference r966 (scope relative) (span (offset 98454) (line 1506) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 98454) (line 1506) (column 66) (len 8)))))
    (reference r967 (scope relative) (span (offset 98465) (line 1506) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 98465) (line 1506) (column 77) (len 3)))))
    (reference r968 (scope relative) (span (offset 98469) (line 1506) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 98469) (line 1506) (column 81) (len 1)))))
    (reference r969 (scope relative) (span (offset 98476) (line 1506) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 98476) (line 1506) (column 88) (len 8)))))
    (reference r970 (scope relative) (span (offset 98526) (line 1507) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 98526) (line 1507) (column 35) (len 19)))))
    (reference r971 (scope relative) (span (offset 98555) (line 1507) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 98555) (line 1507) (column 64) (len 8)))))
    (reference r972 (scope relative) (span (offset 98566) (line 1507) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 98566) (line 1507) (column 75) (len 3)))))
    (reference r973 (scope relative) (span (offset 98570) (line 1507) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 98570) (line 1507) (column 79) (len 1)))))
    (reference r974 (scope relative) (span (offset 98577) (line 1507) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 98577) (line 1507) (column 86) (len 8)))))
    (reference r975 (scope relative) (span (offset 98616) (line 1508) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 98616) (line 1508) (column 23) (len 17)))))
    (reference r976 (scope relative) (span (offset 98640) (line 1508) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 98640) (line 1508) (column 47) (len 20)))))
    (reference r977 (scope relative) (span (offset 98664) (line 1508) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 98664) (line 1508) (column 71) (len 8)))))
    (reference r978 (scope relative) (span (offset 98674) (line 1508) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 98674) (line 1508) (column 81) (len 6)))))
    (reference r979 (scope relative) (span (offset 98806) (line 1512) (column 54) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 98806) (line 1512) (column 54) (len 19)))))
    (reference r980 (scope relative) (span (offset 99589) (line 1525) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 99589) (line 1525) (column 28) (len 4)))))
    (reference r981 (scope relative) (span (offset 99584) (line 1525) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 99584) (line 1525) (column 23) (len 3)))))
    (reference r982 (scope relative) (span (offset 99623) (line 1526) (column 29) (len 30)) (segments (segment 0 (token "MolarAbsorptionCoefficientUnit") (name "MolarAbsorptionCoefficientUnit") (separator none) (span (offset 99623) (line 1526) (column 29) (len 30)))))
    (reference r983 (scope relative) (span (offset 99617) (line 1526) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 99617) (line 1526) (column 23) (len 4)))))
    (reference r984 (scope relative) (span (offset 99707) (line 1529) (column 43) (len 31)) (segments (segment 0 (token "MolarAbsorptionCoefficientValue") (name "MolarAbsorptionCoefficientValue") (separator none) (span (offset 99707) (line 1529) (column 43) (len 31)))))
    (reference r985 (scope relative) (span (offset 99826) (line 1531) (column 53) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 99826) (line 1531) (column 53) (len 11)))))
    (reference r986 (scope relative) (span (offset 99876) (line 1532) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 99876) (line 1532) (column 37) (len 19)))))
    (reference r987 (scope relative) (span (offset 99905) (line 1532) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 99905) (line 1532) (column 66) (len 8)))))
    (reference r988 (scope relative) (span (offset 99916) (line 1532) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 99916) (line 1532) (column 77) (len 3)))))
    (reference r989 (scope relative) (span (offset 99920) (line 1532) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 99920) (line 1532) (column 81) (len 1)))))
    (reference r990 (scope relative) (span (offset 99927) (line 1532) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 99927) (line 1532) (column 88) (len 8)))))
    (reference r991 (scope relative) (span (offset 99990) (line 1533) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 99990) (line 1533) (column 48) (len 19)))))
    (reference r992 (scope relative) (span (offset 100019) (line 1533) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 100019) (line 1533) (column 77) (len 8)))))
    (reference r993 (scope relative) (span (offset 100030) (line 1533) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 100030) (line 1533) (column 88) (len 3)))))
    (reference r994 (scope relative) (span (offset 100034) (line 1533) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 100034) (line 1533) (column 92) (len 1)))))
    (reference r995 (scope relative) (span (offset 100041) (line 1533) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 100041) (line 1533) (column 99) (len 8)))))
    (reference r996 (scope relative) (span (offset 100080) (line 1534) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 100080) (line 1534) (column 23) (len 17)))))
    (reference r997 (scope relative) (span (offset 100104) (line 1534) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 100104) (line 1534) (column 47) (len 20)))))
    (reference r998 (scope relative) (span (offset 100128) (line 1534) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 100128) (line 1534) (column 71) (len 8)))))
    (reference r999 (scope relative) (span (offset 100138) (line 1534) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 100138) (line 1534) (column 81) (len 19)))))
  )
  (root (library-package (name "ISQLight") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 50) (line 3) (column 7) (len 712)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO-80000-7:2019 \"Light and radiation\"\nsee also https://www.iso.org/standard/64977.html\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\nNote 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \ndefined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \nor TensorMeasurementReference.\n"))) (import (target (span (span (offset 785) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 824) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 834) (line 16) (column 30) (len 3))) (separator (span (offset 834) (line 16) (column 30) (len 2))) (marker (span (offset 836) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 858) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 879) (line 17) (column 41) (len 3))) (separator (span (offset 879) (line 17) (column 41) (len 2))) (marker (span (offset 881) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 903) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 910) (line 18) (column 27) (len 3))) (separator (span (offset 910) (line 18) (column 27) (len 2))) (marker (span (offset 912) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 922) (line 20) (column 7) (len 57)) (normalized "Quantity definitions referenced from other ISQ packages "))) (import (target (span (span (offset 1001) (line 21) (column 20) (len 30))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1040) (line 23) (column 7) (len 51)) (normalized "ISO-80000-7 item 7-1.1 speed of light in a medium "))) (attribute-def (declaration-name "SpeedOfLightInAMediumValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1186) (line 26) (column 11) (len 604)) (normalized "source: item 7-1.1 speed of light in a medium\nsymbol(s): `c`\napplication domain: generic\nname: SpeedOfLightInAMedium\nquantity dimension: L^1*T^-1\nmeasurement unit(s): m*s^-1\ntensor order: 0\ndefinition: phase speed of an electromagnetic wave at a given point in a medium\nremarks: See also ISO 80000-3. The value of the speed of light in a medium can depend on the frequency, polarization, and direction. For the definition of the speed of electromagnetic waves in vacuum, `c_0`, see ISO 80000-1.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "speedOfLightInAMedium") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpeedOfLightInAMediumUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2127) (line 44) (column 77) (len 5)) (member-access (base (expression (span (offset 2127) (line 44) (column 77) (len 3)) (ref r14))) (separator dot) (member (ref r15))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2149) (line 44) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2232) (line 45) (column 79) (len 5)) (member-access (base (expression (span (offset 2232) (line 45) (column 79) (len 3)) (ref r19))) (separator dot) (member (ref r20))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2254) (line 45) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 2255) (line 45) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2329) (line 46) (column 70) (len 22)) (tuple (expression (span (offset 2330) (line 46) (column 71) (len 8)) (ref r24)) (expression (span (offset 2340) (line 46) (column 81) (len 10)) (ref r25))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2368) (line 49) (column 7) (len 41)) (normalized "ISO-80000-7 item 7-1.2 refractive index "))) (attribute-def (declaration-name "RefractiveIndexValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2496) (line 52) (column 11) (len 882)) (normalized "source: item 7-1.2 refractive index\nsymbol(s): `n`\napplication domain: generic\nname: RefractiveIndex (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of speed of light in vacuum (ISO 80000-1) and speed of light in a medium (item 7-1.1)\nremarks: The value of the refractive index can depend on the frequency, polarization, and direction. The refractive index is expressed by n = c_0/c, where c_()_0 is the speed of light in vacuum and c is the speed of light in the medium. For a medium with absorption, the complex refractive index n is defined by n = n + ik where k is spectral absorption index (IEC 60050-845) and i is imaginary unit. The refractivity is expressed by n -1, where n is refractive index.\n"))))) (attribute-def (declaration-name "refractiveIndex") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3467) (line 66) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-2.1 radiant energy "))) (attribute-def (declaration-name "radiantEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3594) (line 69) (column 11) (len 943)) (normalized "source: item 7-2.1 radiant energy\nsymbol(s): `Q_e`, `W`, `U`, `(Q)`\napplication domain: electromagnetism\nname: RadiantEnergy (specializes Energy)\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): J, kg*m^2*s^-2\ntensor order: 0\ndefinition: energy (ISO 80000-5) emitted, transferred or received in form of electromagnetic waves\nremarks: Radiant energy can be expressed by the time integral of radiant flux (item 7-4.1), `Φ_e`, over a given duration (ISO 80000-3), `Δt`: `Q_e = int_(Δ t) Φ_e dt`. Radiant energy is expressed either as a function of wavelength (ISO 80000-3), `λ`, as a function of frequency (ISO 80000-3), `ν`, or as a function of wavenumber, `σ`. (See also 0.1.) The corresponding photometric quantity is \"luminous energy\" (item 7-12). The corresponding quantity for photons is \"photon energy\" (item 7-19.2).\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4553) (line 82) (column 7) (len 48)) (normalized "ISO-80000-7 item 7-2.2 spectral radiant energy "))) (attribute-def (declaration-name "SpectralRadiantEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4696) (line 85) (column 11) (len 681)) (normalized "source: item 7-2.2 spectral radiant energy\nsymbol(s): `Q_(e,λ)`, `W_λ`, `U_λ`, `(Q_λ)`\napplication domain: generic\nname: SpectralRadiantEnergy\nquantity dimension: L^1*M^1*T^-2\nmeasurement unit(s): J/nm, kg*m*s^-2\ntensor order: 0\ndefinition: spectral density of radiant energy, expressed by `Q_(e,λ) = (dQ_e) / (dλ)`, where `Q_e` is radiant energy (item 7-2.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) radiant energy is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Q_e = int_(λ_1)^(λ_2) Q_(e,λ) dλ`\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r31)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r32)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r37)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5714) (line 103) (column 77) (len 5)) (member-access (base (expression (span (offset 5714) (line 103) (column 77) (len 3)) (ref r38))) (separator dot) (member (ref r39))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5736) (line 103) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r41)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5815) (line 104) (column 75) (len 5)) (member-access (base (expression (span (offset 5815) (line 104) (column 75) (len 3)) (ref r43))) (separator dot) (member (ref r44))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5837) (line 104) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5920) (line 105) (column 79) (len 5)) (member-access (base (expression (span (offset 5920) (line 105) (column 79) (len 3)) (ref r48))) (separator dot) (member (ref r49))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5942) (line 105) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 5943) (line 105) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6017) (line 106) (column 70) (len 30)) (tuple (expression (span (offset 6018) (line 106) (column 71) (len 8)) (ref r53)) (expression (span (offset 6028) (line 106) (column 81) (len 6)) (ref r54)) (expression (span (offset 6036) (line 106) (column 89) (len 10)) (ref r55))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 6064) (line 109) (column 7) (len 47)) (normalized "ISO-80000-7 item 7-3.1 radiant energy density "))) (attribute-def (declaration-name "RadiantEnergyDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r56)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6205) (line 112) (column 11) (len 794)) (normalized "source: item 7-3.1 radiant energy density\nsymbol(s): `w`, `(ρ_e)`\napplication domain: generic\nname: RadiantEnergyDensity\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): J/m^3, kg*m^-1*s^-2\ntensor order: 0\ndefinition: volumetric density of radiant energy, expressed by `w = (dQ_e)/(dV)`, where `Q_e` is radiant energy (item 7-2.1) in an elementary three-dimensional domain and `V` is the volume (ISO 80000-3) of that domain\nremarks: Radiant energy density within a Planckian radiator is given by `w = (4 σ)/(c_0) T^4` where `σ` is the Stefan-Boltzmann constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1) and `T` is thermodynamic temperature (ISO 80000-5).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r59)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiantEnergyDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadiantEnergyDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r62)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r63)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r64)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7332) (line 130) (column 77) (len 5)) (member-access (base (expression (span (offset 7332) (line 130) (column 77) (len 3)) (ref r65))) (separator dot) (member (ref r66))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r67)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7354) (line 130) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 7355) (line 130) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r68)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7434) (line 131) (column 75) (len 5)) (member-access (base (expression (span (offset 7434) (line 131) (column 75) (len 3)) (ref r70))) (separator dot) (member (ref r71))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r72)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7456) (line 131) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r73)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r74)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7539) (line 132) (column 79) (len 5)) (member-access (base (expression (span (offset 7539) (line 132) (column 79) (len 3)) (ref r75))) (separator dot) (member (ref r76))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r77)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7561) (line 132) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 7562) (line 132) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r78)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r79)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7636) (line 133) (column 70) (len 30)) (tuple (expression (span (offset 7637) (line 133) (column 71) (len 8)) (ref r80)) (expression (span (offset 7647) (line 133) (column 81) (len 6)) (ref r81)) (expression (span (offset 7655) (line 133) (column 89) (len 10)) (ref r82))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7683) (line 136) (column 7) (len 79)) (normalized "ISO-80000-7 item 7-3.2 spectral radiant energy density in terms of wavelength "))) (attribute-def (declaration-name "SpectralRadiantEnergyDensityInTermsOfWavelengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r83)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7883) (line 139) (column 11) (len 938)) (normalized "source: item 7-3.2 spectral radiant energy density in terms of wavelength\nsymbol(s): `w_λ`\napplication domain: generic\nname: SpectralRadiantEnergyDensityInTermsOfWavelength\nquantity dimension: L^-2*M^1*T^-2\nmeasurement unit(s): J/(m^3*nm), kg*m^-2*s^-2\ntensor order: 0\ndefinition: change of radiant energy density with wavelength, expressed by `w_λ = (dw)/(dλ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavelength `λ` (ISO 80000-3)\nremarks: Spectral radiant energy density within a Planckian radiator is given by `w_λ = 8πhc_0*f(λ, T)`, where `h` is the Planck constant (ISO 80000-1), `c_0` is speed of light in vacuum (ISO 80000-1), `T` is thermodynamic temperature (ISO 80000-5) and `f(λ,T) = (λ^-5)/(exp(c_2 λ^-1 T^-1) - 1)`. For the radiation constant `c_2` in `f(λ,T)`, see ISO 80000-1.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r84)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r86)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r87)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantEnergyDensityInTermsOfWavelength") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r88)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r89)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r90)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r91)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9262) (line 157) (column 77) (len 5)) (member-access (base (expression (span (offset 9262) (line 157) (column 77) (len 3)) (ref r92))) (separator dot) (member (ref r93))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r94)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9284) (line 157) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 9285) (line 157) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r95)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r96)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9364) (line 158) (column 75) (len 5)) (member-access (base (expression (span (offset 9364) (line 158) (column 75) (len 3)) (ref r97))) (separator dot) (member (ref r98))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r99)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9386) (line 158) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r100)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r101)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9469) (line 159) (column 79) (len 5)) (member-access (base (expression (span (offset 9469) (line 159) (column 79) (len 3)) (ref r102))) (separator dot) (member (ref r103))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r104)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9491) (line 159) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 9492) (line 159) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r105)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r106)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9566) (line 160) (column 70) (len 30)) (tuple (expression (span (offset 9567) (line 160) (column 71) (len 8)) (ref r107)) (expression (span (offset 9577) (line 160) (column 81) (len 6)) (ref r108)) (expression (span (offset 9585) (line 160) (column 89) (len 10)) (ref r109))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9613) (line 163) (column 7) (len 79)) (normalized "ISO-80000-7 item 7-3.3 spectral radiant energy density in terms of wavenumber "))) (attribute-def (declaration-name "SpectralRadiantEnergyDensityInTermsOfWavenumberValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r110)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9813) (line 166) (column 11) (len 585)) (normalized "source: item 7-3.3 spectral radiant energy density in terms of wavenumber\nsymbol(s): `w_ṽ`, `ρ_ṽ`\napplication domain: generic\nname: SpectralRadiantEnergyDensityInTermsOfWavenumber\nquantity dimension: M^1*T^-2\nmeasurement unit(s): J/m^2, kg*s^-2\ntensor order: 0\ndefinition: change of radiant energy density with wavenumber, expressed by `w_ṽ = (dw)/(dṽ)`, where `w` is radiant energy density (item 7-3.1) as a function of wavenumber `ṽ` (ISO 80000-3)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r111)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r112)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r113)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r114)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantEnergyDensityInTermsOfWavenumber") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r115)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r116)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r117)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r118)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10837) (line 184) (column 75) (len 5)) (member-access (base (expression (span (offset 10837) (line 184) (column 75) (len 3)) (ref r119))) (separator dot) (member (ref r120))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r121)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10859) (line 184) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r122)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r123)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10942) (line 185) (column 79) (len 5)) (member-access (base (expression (span (offset 10942) (line 185) (column 79) (len 3)) (ref r124))) (separator dot) (member (ref r125))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r126)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10964) (line 185) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 10965) (line 185) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r127)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r128)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11039) (line 186) (column 70) (len 20)) (tuple (expression (span (offset 11040) (line 186) (column 71) (len 6)) (ref r129)) (expression (span (offset 11048) (line 186) (column 79) (len 10)) (ref r130))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 11076) (line 189) (column 7) (len 52)) (normalized "ISO-80000-7 item 7-4.1 radiant flux, radiant power "))) (attribute-def (declaration-name "RadiantFluxValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r131)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 11213) (line 192) (column 11) (len 665)) (normalized "source: item 7-4.1 radiant flux, radiant power\nsymbol(s): `Φ_e`, `P_e`, `Φ`, `P`\napplication domain: generic\nname: RadiantFlux\nquantity dimension: L^2*M^1*T^-3\nmeasurement unit(s): W, kg*m^2*s^-3\ntensor order: 0\ndefinition: change in radiant energy with time, expressed by `Φ_e = (dQ_e)/(dt)`, where `Q_e` is the radiant energy (item 7-2.1) emitted, transferred or received and `t` is time (ISO 80000-3)\nremarks: The corresponding photometric quantity is \"luminous flux\" (item 7-13). The corresponding quantity for photons is \"photon flux\" (item 7-20).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r133)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r134)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r135)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiantFlux") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r136)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadiantFluxUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r137)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r138)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r139)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12175) (line 210) (column 77) (len 5)) (member-access (base (expression (span (offset 12175) (line 210) (column 77) (len 3)) (ref r140))) (separator dot) (member (ref r141))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r142)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12197) (line 210) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r143)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r144)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12276) (line 211) (column 75) (len 5)) (member-access (base (expression (span (offset 12276) (line 211) (column 75) (len 3)) (ref r145))) (separator dot) (member (ref r146))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12298) (line 211) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r148)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r149)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12381) (line 212) (column 79) (len 5)) (member-access (base (expression (span (offset 12381) (line 212) (column 79) (len 3)) (ref r150))) (separator dot) (member (ref r151))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r152)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12403) (line 212) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 12404) (line 212) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r153)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r154)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12478) (line 213) (column 70) (len 30)) (tuple (expression (span (offset 12479) (line 213) (column 71) (len 8)) (ref r155)) (expression (span (offset 12489) (line 213) (column 81) (len 6)) (ref r156)) (expression (span (offset 12497) (line 213) (column 89) (len 10)) (ref r157))))))) (body semicolon)))))) (alias (name "RadiantPowerUnit") (target (ref r158)) (body semicolon)) (alias (name "RadiantPowerValue") (target (ref r159)) (body semicolon)) (alias (name "radiantPower") (target (ref r160)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 12664) (line 220) (column 7) (len 70)) (normalized "ISO-80000-7 item 7-4.2 spectral radiant flux, spectral radiant power "))) (attribute-def (declaration-name "SpectralRadiantFluxValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r161)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12827) (line 223) (column 11) (len 707)) (normalized "source: item 7-4.2 spectral radiant flux, spectral radiant power\nsymbol(s): `Φ_(e,λ)`, `P_(e,λ)`, `(Φ_λ)`, `(P_λ)`\napplication domain: generic\nname: SpectralRadiantFlux\nquantity dimension: L^1*M^1*T^-3\nmeasurement unit(s): W/nm, kg*m*s^-3\ntensor order: 0\ndefinition: spectral density of radiant flux, expressed by `Φ_(e,λ) = (dQ_e)/(dλ)`, where `Φ_e` is radiant flux (item 7-4.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) radiant flux is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `Φ_e = int_(λ_1)^(λ_2) Φ_(e,λ) dλ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r162)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r163)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r164)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r165)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantFlux") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r166)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantFluxUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r167)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r168)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r169)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13863) (line 241) (column 77) (len 5)) (member-access (base (expression (span (offset 13863) (line 241) (column 77) (len 3)) (ref r170))) (separator dot) (member (ref r171))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r172)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13885) (line 241) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r173)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r174)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13964) (line 242) (column 75) (len 5)) (member-access (base (expression (span (offset 13964) (line 242) (column 75) (len 3)) (ref r175))) (separator dot) (member (ref r176))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r177)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13986) (line 242) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r178)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r179)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14069) (line 243) (column 79) (len 5)) (member-access (base (expression (span (offset 14069) (line 243) (column 79) (len 3)) (ref r180))) (separator dot) (member (ref r181))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r182)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14091) (line 243) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 14092) (line 243) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r183)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r184)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14166) (line 244) (column 70) (len 30)) (tuple (expression (span (offset 14167) (line 244) (column 71) (len 8)) (ref r185)) (expression (span (offset 14177) (line 244) (column 81) (len 6)) (ref r186)) (expression (span (offset 14185) (line 244) (column 89) (len 10)) (ref r187))))))) (body semicolon)))))) (alias (name "SpectralRadiantPowerUnit") (target (ref r188)) (body semicolon)) (alias (name "SpectralRadiantPowerValue") (target (ref r189)) (body semicolon)) (alias (name "spectralRadiantPower") (target (ref r190)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 14400) (line 251) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-5.1 radiant intensity "))) (attribute-def (declaration-name "RadiantIntensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r191)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14532) (line 254) (column 11) (len 1096)) (normalized "source: item 7-5.1 radiant intensity\nsymbol(s): `I_e`, `(I)`\napplication domain: generic\nname: RadiantIntensity\nquantity dimension: L^2*M^1*T^-3\nmeasurement unit(s): W/sr, kg*m^2*s^-3*sr^-1\ntensor order: 0\ndefinition: density of radiant flux with respect to solid angle in a specified direction, expressed by `I_e = (dΦ_e)/(dΩ)`, where `Φ_e` is the radiant flux (item 7-4.1) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction\nremarks: The definition holds strictly only for a point source. The distribution of the radiant intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,φ)`, is used to determine the radiant flux (item 7-4.1) within a certain solid angle (ISO 80000-3), `Ω`, of a source: `Φ_e = int int_Ω I_e(θ, φ) sin(θ) dφ dθ`. The corresponding photometric quantity is \"luminous intensity\" (item 7-14). The corresponding quantity for photons is \"photon intensity\" (item 7-21).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r192)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r193)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r194)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r195)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiantIntensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r196)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadiantIntensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r197)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r198)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r199)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15945) (line 272) (column 77) (len 5)) (member-access (base (expression (span (offset 15945) (line 272) (column 77) (len 3)) (ref r200))) (separator dot) (member (ref r201))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15967) (line 272) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r203)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r204)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16046) (line 273) (column 75) (len 5)) (member-access (base (expression (span (offset 16046) (line 273) (column 75) (len 3)) (ref r205))) (separator dot) (member (ref r206))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r207)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16068) (line 273) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r208)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r209)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16151) (line 274) (column 79) (len 5)) (member-access (base (expression (span (offset 16151) (line 274) (column 79) (len 3)) (ref r210))) (separator dot) (member (ref r211))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r212)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16173) (line 274) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 16174) (line 274) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r213)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r214)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16248) (line 275) (column 70) (len 30)) (tuple (expression (span (offset 16249) (line 275) (column 71) (len 8)) (ref r215)) (expression (span (offset 16259) (line 275) (column 81) (len 6)) (ref r216)) (expression (span (offset 16267) (line 275) (column 89) (len 10)) (ref r217))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 16295) (line 278) (column 7) (len 51)) (normalized "ISO-80000-7 item 7-5.2 spectral radiant intensity "))) (attribute-def (declaration-name "SpectralRadiantIntensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r218)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16444) (line 281) (column 11) (len 693)) (normalized "source: item 7-5.2 spectral radiant intensity\nsymbol(s): `I_(e,λ)`, `(I_λ)`\napplication domain: generic\nname: SpectralRadiantIntensity\nquantity dimension: L^1*M^1*T^-3\nmeasurement unit(s): W/(sr*nm), kg*m*s^-3*sr^-1\ntensor order: 0\ndefinition: spectral density of radiant intensity, expressed by `I_(e, λ) = (d I_e)/(dλ)`, where `I_e` is radiant intensity (item 7-5.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) radiant intensity is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `I_e = int_(λ_1)^(λ_2) I_(e,λ) dλ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r219)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r220)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r221)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r222)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantIntensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r223)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantIntensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r224)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r225)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r226)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17486) (line 299) (column 77) (len 5)) (member-access (base (expression (span (offset 17486) (line 299) (column 77) (len 3)) (ref r227))) (separator dot) (member (ref r228))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r229)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17508) (line 299) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r230)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r231)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17587) (line 300) (column 75) (len 5)) (member-access (base (expression (span (offset 17587) (line 300) (column 75) (len 3)) (ref r232))) (separator dot) (member (ref r233))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r234)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17609) (line 300) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r235)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r236)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17692) (line 301) (column 79) (len 5)) (member-access (base (expression (span (offset 17692) (line 301) (column 79) (len 3)) (ref r237))) (separator dot) (member (ref r238))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r239)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17714) (line 301) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 17715) (line 301) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r240)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r241)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17789) (line 302) (column 70) (len 30)) (tuple (expression (span (offset 17790) (line 302) (column 71) (len 8)) (ref r242)) (expression (span (offset 17800) (line 302) (column 81) (len 6)) (ref r243)) (expression (span (offset 17808) (line 302) (column 89) (len 10)) (ref r244))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 17836) (line 305) (column 7) (len 33)) (normalized "ISO-80000-7 item 7-6.1 radiance "))) (attribute-def (declaration-name "RadianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r245)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 17951) (line 308) (column 11) (len 990)) (normalized "source: item 7-6.1 radiance\nsymbol(s): `L_e`, `(L)`\napplication domain: generic\nname: Radiance\nquantity dimension: M^1*T^-3\nmeasurement unit(s): W/(sr*m^2), kg*s^-3*sr^-1\ntensor order: 0\ndefinition: density of radiant intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_e = (d I_e)/(dA) * 1/cos(α)`, where `I_e` is radiant intensity (item 7-5.1), `A` is area (ISO 80000-3), and `α` is the angle between the normal to the surface at the specified point and the specified direction\nremarks: See also 0.1. For Planckian radiation, `L_e = σ/π T^4` where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is \"luminance\" (item 7-15). The corresponding quantity for photons is \"photon radiance\" (item 7-22).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r246)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r247)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r248)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r249)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r250)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r251)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r252)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r253)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19224) (line 326) (column 75) (len 5)) (member-access (base (expression (span (offset 19224) (line 326) (column 75) (len 3)) (ref r254))) (separator dot) (member (ref r255))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r256)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19246) (line 326) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r257)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r258)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19329) (line 327) (column 79) (len 5)) (member-access (base (expression (span (offset 19329) (line 327) (column 79) (len 3)) (ref r259))) (separator dot) (member (ref r260))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r261)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19351) (line 327) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 19352) (line 327) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r262)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r263)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19426) (line 328) (column 70) (len 20)) (tuple (expression (span (offset 19427) (line 328) (column 71) (len 6)) (ref r264)) (expression (span (offset 19435) (line 328) (column 79) (len 10)) (ref r265))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 19463) (line 331) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-6.2 spectral radiance "))) (attribute-def (declaration-name "SpectralRadianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r266)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19595) (line 334) (column 11) (len 1143)) (normalized "source: item 7-6.2 spectral radiance\nsymbol(s): `L_(e,λ)`, `(L_λ)`\napplication domain: generic\nname: SpectralRadiance\nquantity dimension: L^-1*M^1*T^-3\nmeasurement unit(s): W/(sr*m^2*nm), kg*m^-1*s^-3*sr^-1\ntensor order: 0\ndefinition: density of radiance with respect to wavelength, expressed by `L_(e, λ) = (d L_e)/(d λ)` where `L_e` is radiance (item 7-6.1) in terms of wavelength λ(ISO 80000-3)\nremarks: For Planckian radiation, `L_(e, λ)(λ) = (c(λ))/(4 π) ω_λ(λ) = h c_0^2 * f(λ,T)`, where `c(λ)` is phase speed (ISO 80000-3) of electromagnetic radiation of a wavelength (ISO 80000-3) `λ` in a given medium, `ω_λ(λ)` is spectral radiant energy density in terms of wavelength, `c_0` is speed of light in vacuum (ISO 80000-1), `h` is the Planck constant (ISO 80000-1), and `f(λ,T) = λ^-5/(exp(c_2 λ^-1 T^-1) - 1)`, where the radiation constant `c_2 = (hc)/k`. The integral of (total) radiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `L_e = int_(λ_1)^(λ_2) L_(e,λ) dλ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r267)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r268)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r269)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r270)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r271)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r272)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r273)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r274)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21055) (line 352) (column 77) (len 5)) (member-access (base (expression (span (offset 21055) (line 352) (column 77) (len 3)) (ref r275))) (separator dot) (member (ref r276))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r277)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21077) (line 352) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 21078) (line 352) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r278)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r279)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21157) (line 353) (column 75) (len 5)) (member-access (base (expression (span (offset 21157) (line 353) (column 75) (len 3)) (ref r280))) (separator dot) (member (ref r281))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r282)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21179) (line 353) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r283)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r284)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21262) (line 354) (column 79) (len 5)) (member-access (base (expression (span (offset 21262) (line 354) (column 79) (len 3)) (ref r285))) (separator dot) (member (ref r286))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r287)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21284) (line 354) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 21285) (line 354) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r288)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r289)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21359) (line 355) (column 70) (len 30)) (tuple (expression (span (offset 21360) (line 355) (column 71) (len 8)) (ref r290)) (expression (span (offset 21370) (line 355) (column 81) (len 6)) (ref r291)) (expression (span (offset 21378) (line 355) (column 89) (len 10)) (ref r292))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 21406) (line 358) (column 7) (len 35)) (normalized "ISO-80000-7 item 7-7.1 irradiance "))) (attribute-def (declaration-name "IrradianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r293)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 21525) (line 361) (column 11) (len 1479)) (normalized "source: item 7-7.1 irradiance\nsymbol(s): `E_e`, `(E)`\napplication domain: generic\nname: Irradiance\nquantity dimension: M^1*T^-3\nmeasurement unit(s): W/m^2, kg*s^-3\ntensor order: 0\ndefinition: density of incident radiant flux with respect to area at a point on a real or imaginary surface, expressed by `E_e = (d Φ_e)/(d A)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) on which the radiant flux is incident\nremarks: The corresponding photometric quantity is \"illuminance\" (item 7-16). The corresponding quantity for photons is \"photon irradiance\" (item 7-23). The quantity \"spherical irradiance\" is defined by the mean value of irradiance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(e,0) = int_(4 π) L_e d Ω` where `Ω` is solid angle (ISO 80000-3) and `L_e` is radiance (item 7-6.1). (See CIE DIS 017/E:2016, term 17-21-054.) It can be expressed by the quotient of the radiant flux (item 7-4.1) of all the radiation incident on the outer surface of an infinitely small sphere centred at the specified point and the area (ISO 80000-3) of the diametrical cross-section of that sphere. Spherical irradiance is also called \"fluence rate\" or \"radiant fluence rate\". The corresponding photometric quantity to spherical irradiance is called \"spherical illuminance\".\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r294)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r295)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r296)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r297)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "irradiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r298)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "IrradianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r299)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r300)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r301)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23295) (line 379) (column 75) (len 5)) (member-access (base (expression (span (offset 23295) (line 379) (column 75) (len 3)) (ref r302))) (separator dot) (member (ref r303))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r304)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23317) (line 379) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r305)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r306)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23400) (line 380) (column 79) (len 5)) (member-access (base (expression (span (offset 23400) (line 380) (column 79) (len 3)) (ref r307))) (separator dot) (member (ref r308))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r309)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23422) (line 380) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 23423) (line 380) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r310)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r311)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23497) (line 381) (column 70) (len 20)) (tuple (expression (span (offset 23498) (line 381) (column 71) (len 6)) (ref r312)) (expression (span (offset 23506) (line 381) (column 79) (len 10)) (ref r313))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 23534) (line 384) (column 7) (len 44)) (normalized "ISO-80000-7 item 7-7.2 spectral irradiance "))) (attribute-def (declaration-name "SpectralIrradianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r314)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 23670) (line 387) (column 11) (len 676)) (normalized "source: item 7-7.2 spectral irradiance\nsymbol(s): `E_(e,λ)`, `(E_λ)`\napplication domain: generic\nname: SpectralIrradiance\nquantity dimension: L^-1*M^1*T^-3\nmeasurement unit(s): W/(m^2*nm), kg*m^-1*s^-3\ntensor order: 0\ndefinition: density of irradiance with respect to wavelength, expressed by `E_(e,λ) = (d E_e)/(dλ)`, where `E_e` is irradiance (item 7-7.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) irradiance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `E_e = int_(λ_1)^(λ_2) E_(e,λ) d λ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r315)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r316)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r317)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r318)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralIrradiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r319)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralIrradianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r320)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r321)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r322)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24671) (line 405) (column 77) (len 5)) (member-access (base (expression (span (offset 24671) (line 405) (column 77) (len 3)) (ref r323))) (separator dot) (member (ref r324))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r325)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24693) (line 405) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 24694) (line 405) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r326)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r327)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24773) (line 406) (column 75) (len 5)) (member-access (base (expression (span (offset 24773) (line 406) (column 75) (len 3)) (ref r328))) (separator dot) (member (ref r329))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r330)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24795) (line 406) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r331)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r332)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24878) (line 407) (column 79) (len 5)) (member-access (base (expression (span (offset 24878) (line 407) (column 79) (len 3)) (ref r333))) (separator dot) (member (ref r334))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r335)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24900) (line 407) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 24901) (line 407) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r336)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r337)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24975) (line 408) (column 70) (len 30)) (tuple (expression (span (offset 24976) (line 408) (column 71) (len 8)) (ref r338)) (expression (span (offset 24986) (line 408) (column 81) (len 6)) (ref r339)) (expression (span (offset 24994) (line 408) (column 89) (len 10)) (ref r340))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 25022) (line 411) (column 7) (len 61)) (normalized "ISO-80000-7 item 7-8.1 radiant exitance , radiant emittance "))) (attribute-def (declaration-name "RadiantExitanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r341)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 25172) (line 414) (column 11) (len 882)) (normalized "source: item 7-8.1 radiant exitance , radiant emittance\nsymbol(s): `M_e`, `(M)`\napplication domain: generic\nname: RadiantExitance\nquantity dimension: M^1*T^-3\nmeasurement unit(s): W/m^2, kg*s^-3\ntensor order: 0\ndefinition: density of exiting radiant flux with respect to area at a point on a real or imaginary surface, expressed by `M_e = (d Φ_e)/(dA)`, where `Φ_e` is radiant flux (item 7-4.1) and `A` is the area (ISO 80000-3) from which the radiant flux leaves\nremarks: For Planckian radiation, `M_e = σT^4`, where `T` is thermodynamic temperature (ISO 80000-5) and `σ` is the Stefan-Boltzmann constant (ISO 80000-1). The corresponding photometric quantity is \"luminous exitance\" (item 7-17). The corresponding quantity for photons is \"photon exitance\" (item 7-24).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r342)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r343)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r344)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r345)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiantExitance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r346)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadiantExitanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r347)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r348)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r349)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26365) (line 432) (column 75) (len 5)) (member-access (base (expression (span (offset 26365) (line 432) (column 75) (len 3)) (ref r350))) (separator dot) (member (ref r351))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r352)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26387) (line 432) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r353)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r354)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26470) (line 433) (column 79) (len 5)) (member-access (base (expression (span (offset 26470) (line 433) (column 79) (len 3)) (ref r355))) (separator dot) (member (ref r356))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r357)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26492) (line 433) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 26493) (line 433) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r358)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r359)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26567) (line 434) (column 70) (len 20)) (tuple (expression (span (offset 26568) (line 434) (column 71) (len 6)) (ref r360)) (expression (span (offset 26576) (line 434) (column 79) (len 10)) (ref r361))))))) (body semicolon)))))) (alias (name "RadiantEmittanceUnit") (target (ref r362)) (body semicolon)) (alias (name "RadiantEmittanceValue") (target (ref r363)) (body semicolon)) (alias (name "radiantEmittance") (target (ref r364)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 26767) (line 441) (column 7) (len 50)) (normalized "ISO-80000-7 item 7-8.2 spectral radiant exitance "))) (attribute-def (declaration-name "SpectralRadiantExitanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r365)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 26914) (line 444) (column 11) (len 705)) (normalized "source: item 7-8.2 spectral radiant exitance\nsymbol(s): `M_(e,λ)`, `(M_λ)`\napplication domain: generic\nname: SpectralRadiantExitance\nquantity dimension: L^-1*M^1*T^-3\nmeasurement unit(s): W/(m^2*nm), kg*m^-1*s^-3\ntensor order: 0\ndefinition: density of radiant exitance with respect to wavelength, expressed by `M_(e,λ) = (d M_e)/(dλ)`, where `M_e` is radiant exitance (item 7-8.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) radiant exitance is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `M_e = int_(λ_1)^(λ_2) M_(e,λ) d λ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r366)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r367)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r368)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r369)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantExitance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r370)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantExitanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r371)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r372)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r373)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27964) (line 462) (column 77) (len 5)) (member-access (base (expression (span (offset 27964) (line 462) (column 77) (len 3)) (ref r374))) (separator dot) (member (ref r375))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r376)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27986) (line 462) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 27987) (line 462) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r377)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r378)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28066) (line 463) (column 75) (len 5)) (member-access (base (expression (span (offset 28066) (line 463) (column 75) (len 3)) (ref r379))) (separator dot) (member (ref r380))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r381)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28088) (line 463) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r382)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28171) (line 464) (column 79) (len 5)) (member-access (base (expression (span (offset 28171) (line 464) (column 79) (len 3)) (ref r384))) (separator dot) (member (ref r385))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r386)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28193) (line 464) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 28194) (line 464) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r387)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r388)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28268) (line 465) (column 70) (len 30)) (tuple (expression (span (offset 28269) (line 465) (column 71) (len 8)) (ref r389)) (expression (span (offset 28279) (line 465) (column 81) (len 6)) (ref r390)) (expression (span (offset 28287) (line 465) (column 89) (len 10)) (ref r391))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28315) (line 468) (column 7) (len 41)) (normalized "ISO-80000-7 item 7-9.1 radiant exposure "))) (attribute-def (declaration-name "RadiantExposureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r392)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 28445) (line 471) (column 11) (len 720)) (normalized "source: item 7-9.1 radiant exposure\nsymbol(s): `H_e`, `(H)`\napplication domain: generic\nname: RadiantExposure\nquantity dimension: M^1*T^-2\nmeasurement unit(s): J/m^2, kg*s^-2\ntensor order: 0\ndefinition: density of incident radiant energy with respect to area at a point on a real or imaginary surface, expressed by `H_e = (d Q_e)/(dA)`, where `Q_e` is radiant energy (item 7-2.1) and `A` is the area on which the radiant energy is incident (ISO 80000-3)\nremarks: The corresponding photometric quantity is \"luminous exposure\" (item 7-18). The corresponding quantity for photons is \"photon exposure\" (item 7-25).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r393)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r394)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r395)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r396)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "radiantExposure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r397)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RadiantExposureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r398)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r399)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r400)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29476) (line 489) (column 75) (len 5)) (member-access (base (expression (span (offset 29476) (line 489) (column 75) (len 3)) (ref r401))) (separator dot) (member (ref r402))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r403)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29498) (line 489) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r404)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r405)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29581) (line 490) (column 79) (len 5)) (member-access (base (expression (span (offset 29581) (line 490) (column 79) (len 3)) (ref r406))) (separator dot) (member (ref r407))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29603) (line 490) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 29604) (line 490) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r409)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r410)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29678) (line 491) (column 70) (len 20)) (tuple (expression (span (offset 29679) (line 491) (column 71) (len 6)) (ref r411)) (expression (span (offset 29687) (line 491) (column 79) (len 10)) (ref r412))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 29715) (line 494) (column 7) (len 50)) (normalized "ISO-80000-7 item 7-9.2 spectral radiant exposure "))) (attribute-def (declaration-name "SpectralRadiantExposureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r413)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 29862) (line 497) (column 11) (len 705)) (normalized "source: item 7-9.2 spectral radiant exposure\nsymbol(s): `H_(e,λ)`, `(H_λ)`\napplication domain: generic\nname: SpectralRadiantExposure\nquantity dimension: L^-1*M^1*T^-2\nmeasurement unit(s): J/(m^2*nm), kg*m^-1*s^-2\ntensor order: 0\ndefinition: density of radiant exposure with respect to wavelength, expressed by `H_(e,λ) = (d H_e)/(dλ)`, where `H_e` is radiant exposure (item 7-9.1) in terms of wavelength `λ` (ISO 80000-3)\nremarks: The integral of (total) radiant exposure is determined by the wavelength interval `(λ_1, λ_2)` under consideration: `H_e = int_(λ_1)^(λ_2) H_(e,λ) d λ` .\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r414)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r415)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r416)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r417)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralRadiantExposure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r418)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralRadiantExposureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r419)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r420)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r421)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30912) (line 515) (column 77) (len 5)) (member-access (base (expression (span (offset 30912) (line 515) (column 77) (len 3)) (ref r422))) (separator dot) (member (ref r423))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r424)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30934) (line 515) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 30935) (line 515) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r425)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r426)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31014) (line 516) (column 75) (len 5)) (member-access (base (expression (span (offset 31014) (line 516) (column 75) (len 3)) (ref r427))) (separator dot) (member (ref r428))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r429)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31036) (line 516) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r430)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r431)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31119) (line 517) (column 79) (len 5)) (member-access (base (expression (span (offset 31119) (line 517) (column 79) (len 3)) (ref r432))) (separator dot) (member (ref r433))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r434)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31141) (line 517) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 31142) (line 517) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r435)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r436)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31216) (line 518) (column 70) (len 30)) (tuple (expression (span (offset 31217) (line 518) (column 71) (len 8)) (ref r437)) (expression (span (offset 31227) (line 518) (column 81) (len 6)) (ref r438)) (expression (span (offset 31235) (line 518) (column 89) (len 10)) (ref r439))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 31263) (line 521) (column 7) (len 45)) (normalized "ISO-80000-7 item 7-10.1 luminous efficiency "))) (attribute-def (declaration-name "LuminousEfficiencyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r440)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 31398) (line 524) (column 11) (len 1220)) (normalized "source: item 7-10.1 luminous efficiency\nsymbol(s): `V`\napplication domain: specified photometric condition\nname: LuminousEfficiency (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of radiant flux (item 7-4.1) weighted by the spectral luminous efficiency (item 7-10.2) and the corresponding radiant flux for a specified photometric condition\nremarks: Luminous efficiency for photopic vision is expressed by `V = (int_0^∞ Φ_(e,λ)(λ) V(λ) d λ)/(int_0^∞ Φ_(e,λ)(λ) d λ) = K/K_m`, where `Φ_(e,λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency, `λ` is wavelength, `K` is luminous efficacy of radiation (item 7-11.1), and `K_m` is maximum luminous efficacy (item 7-11.3). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V` for photopic vision; `V'` for scotopic vision; `V_(mes;m)` for mesopic vision; `V_10` for the CIE 10° photopic photometric observer; `V_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.\n"))))) (attribute-def (declaration-name "luminousEfficiency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r441)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 32713) (line 538) (column 7) (len 54)) (normalized "ISO-80000-7 item 7-10.2 spectral luminous efficiency "))) (attribute-def (declaration-name "SpectralLuminousEfficiencyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r442)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 32865) (line 541) (column 11) (len 1346)) (normalized "source: item 7-10.2 spectral luminous efficiency\nsymbol(s): `V(λ)`\napplication domain: specified photometric condition\nname: SpectralLuminousEfficiency (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the radiant flux (item 7-4.1) at wavelength `λ_m` and that at wavelength `λ`, such that both produce equally intense luminous sensations for a specified photometric condition and `λ_m` is chosen so that the maximum value of this quotient is equal to 1\nremarks: The spectral luminous efficiency of the human eye depends on a number of factors, particularly the state of visual adaptation and the size and position of the source in the visual field. The photometric condition should be specified (e.g. photopic, scotopic, mesopic). If it is not specified, photopic vision is assumed and the symbol `V(λ)` is used. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `V(λ)` for photopic vision; `V'(λ)` for scotopic vision; `V_(mes;m)(λ)` for mesopic vision; `V_10(λ)` for the CIE 10° photopic photometric observer; `V_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.\n"))))) (attribute-def (declaration-name "spectralLuminousEfficiency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r443)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 34322) (line 555) (column 7) (len 56)) (normalized "ISO-80000-7 item 7-11.1 luminous efficacy of radiation "))) (attribute-def (declaration-name "LuminousEfficacyOfRadiationValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r444)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 34479) (line 558) (column 11) (len 1003)) (normalized "source: item 7-11.1 luminous efficacy of radiation\nsymbol(s): `K`\napplication domain: specified photometric condition\nname: LuminousEfficacyOfRadiation\nquantity dimension: L^-2*M^-1*T^3*J^1\nmeasurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3\ntensor order: 0\ndefinition: quotient of luminous flux (item 7-13) and the corresponding radiant flux (item 7-4.1) for a specified photometric condition\nremarks: Luminous efficacy of radiation for photopic vision is expressed by `K = Φ_V/Φ_e`, where `Φ_v` is luminous flux (item 7-13) and `Φ_e` is radiant flux (item 7-4.1). For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K` for photopic vision; `K'` for scotopic vision; `K_(mes;m)` for mesopic vision; `K_10` for the CIE 10° photopic photometric observer; `K_M` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r445)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r446)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r447)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r448)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousEfficacyOfRadiation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r449)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousEfficacyOfRadiationUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r450)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r451)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r452)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 35843) (line 576) (column 77) (len 5)) (member-access (base (expression (span (offset 35843) (line 576) (column 77) (len 3)) (ref r453))) (separator dot) (member (ref r454))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r455)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 35865) (line 576) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 35866) (line 576) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r456)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r457)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 35945) (line 577) (column 75) (len 5)) (member-access (base (expression (span (offset 35945) (line 577) (column 75) (len 3)) (ref r458))) (separator dot) (member (ref r459))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r460)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 35967) (line 577) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 35968) (line 577) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r461)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r462)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36051) (line 578) (column 79) (len 5)) (member-access (base (expression (span (offset 36051) (line 578) (column 79) (len 3)) (ref r463))) (separator dot) (member (ref r464))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r465)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36073) (line 578) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r466)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r467)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36165) (line 579) (column 88) (len 5)) (member-access (base (expression (span (offset 36165) (line 579) (column 88) (len 3)) (ref r468))) (separator dot) (member (ref r469))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r470)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36187) (line 579) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r471)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r472)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36261) (line 580) (column 70) (len 51)) (tuple (expression (span (offset 36262) (line 580) (column 71) (len 8)) (ref r473)) (expression (span (offset 36272) (line 580) (column 81) (len 6)) (ref r474)) (expression (span (offset 36280) (line 580) (column 89) (len 10)) (ref r475)) (expression (span (offset 36292) (line 580) (column 101) (len 19)) (ref r476))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 36329) (line 583) (column 7) (len 52)) (normalized "ISO-80000-7 item 7-11.2 spectral luminous efficacy "))) (attribute-def (declaration-name "SpectralLuminousEfficacyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r477)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36479) (line 586) (column 11) (len 1084)) (normalized "source: item 7-11.2 spectral luminous efficacy\nsymbol(s): `K(λ)`\napplication domain: specified photometric condition\nname: SpectralLuminousEfficacy\nquantity dimension: L^-2*M^-1*T^3*J^1\nmeasurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3\ntensor order: 0\ndefinition: product of spectral luminous efficiency (item 7-10.2) and maximum luminous efficacy (item 7-11.3) for a specified photometric condition\nremarks: Spectral luminous efficacy for photopic vision is expressed by `K(λ) = K_m V(λ)`, where `K_m` is maximum luminous efficacy (item 7-11.3), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength. For scotopic and mesopic vision see 0.4 and 0.5. Symbols for different photometric conditions: `K(λ)` for photopic vision>; `K'(λ)` for scotopic vision; `K_(mes;m)(λ)` for mesopic vision; `K_10(λ)` for the CIE 10° photopic photometric observer; `K_M(λ)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r478)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r479)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r480)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r481)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "spectralLuminousEfficacy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r482)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpectralLuminousEfficacyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r483)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r484)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r485)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37912) (line 604) (column 77) (len 5)) (member-access (base (expression (span (offset 37912) (line 604) (column 77) (len 3)) (ref r486))) (separator dot) (member (ref r487))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r488)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37934) (line 604) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 37935) (line 604) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r489)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r490)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38014) (line 605) (column 75) (len 5)) (member-access (base (expression (span (offset 38014) (line 605) (column 75) (len 3)) (ref r491))) (separator dot) (member (ref r492))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r493)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38036) (line 605) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 38037) (line 605) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r494)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r495)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38120) (line 606) (column 79) (len 5)) (member-access (base (expression (span (offset 38120) (line 606) (column 79) (len 3)) (ref r496))) (separator dot) (member (ref r497))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r498)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38142) (line 606) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r499)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r500)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38234) (line 607) (column 88) (len 5)) (member-access (base (expression (span (offset 38234) (line 607) (column 88) (len 3)) (ref r501))) (separator dot) (member (ref r502))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r503)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38256) (line 607) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r504)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r505)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38330) (line 608) (column 70) (len 51)) (tuple (expression (span (offset 38331) (line 608) (column 71) (len 8)) (ref r506)) (expression (span (offset 38341) (line 608) (column 81) (len 6)) (ref r507)) (expression (span (offset 38349) (line 608) (column 89) (len 10)) (ref r508)) (expression (span (offset 38361) (line 608) (column 101) (len 19)) (ref r509))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38398) (line 611) (column 7) (len 51)) (normalized "ISO-80000-7 item 7-11.3 maximum luminous efficacy "))) (attribute-def (declaration-name "MaximumLuminousEfficacyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r510)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38546) (line 614) (column 11) (len 1127)) (normalized "source: item 7-11.3 maximum luminous efficacy\nsymbol(s): `K_m`\napplication domain: specified photometric condition\nname: MaximumLuminousEfficacy\nquantity dimension: L^-2*M^-1*T^3*J^1\nmeasurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3\ntensor order: 0\ndefinition: maximum value of spectral luminous efficacy for a specified photometric condition\nremarks: See also 0.4 and 0.5. The value of maximum luminous efficacy for photopic vision is calculated by `K_m = 683 / (V(λ_(cd))) [\"cd\"*\"sr\"*\"W\"^-1] = 683 [\"lm\"*\"W\"^-1]` where `V(λ)` is the spectral luminous efficiency for photopic vision and `λ_(cd)` is the wavelength in air corresponding to the frequency `540*10^12 [\"Hz\"]` specified in the definition of the SI unit candela. Symbols for different photometric conditions: `K_m` for photopic vision; `K'_m` for scotopic vision; `K_(m,mes;m)` for mesopic vision; `K_(m,10)` for the CIE 10° photopic photometric observer; `K_(m,M)` for the CIE 1988 modified 2° spectral luminous efficiency function for photopic vision.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r511)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r512)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r513)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r514)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "maximumLuminousEfficacy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r515)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MaximumLuminousEfficacyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r516)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r517)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r518)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40018) (line 632) (column 77) (len 5)) (member-access (base (expression (span (offset 40018) (line 632) (column 77) (len 3)) (ref r519))) (separator dot) (member (ref r520))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r521)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40040) (line 632) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 40041) (line 632) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r522)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r523)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40120) (line 633) (column 75) (len 5)) (member-access (base (expression (span (offset 40120) (line 633) (column 75) (len 3)) (ref r524))) (separator dot) (member (ref r525))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r526)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40142) (line 633) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 40143) (line 633) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r527)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r528)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40226) (line 634) (column 79) (len 5)) (member-access (base (expression (span (offset 40226) (line 634) (column 79) (len 3)) (ref r529))) (separator dot) (member (ref r530))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r531)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40248) (line 634) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r532)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r533)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40340) (line 635) (column 88) (len 5)) (member-access (base (expression (span (offset 40340) (line 635) (column 88) (len 3)) (ref r534))) (separator dot) (member (ref r535))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r536)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40362) (line 635) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r537)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r538)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 40436) (line 636) (column 70) (len 51)) (tuple (expression (span (offset 40437) (line 636) (column 71) (len 8)) (ref r539)) (expression (span (offset 40447) (line 636) (column 81) (len 6)) (ref r540)) (expression (span (offset 40455) (line 636) (column 89) (len 10)) (ref r541)) (expression (span (offset 40467) (line 636) (column 101) (len 19)) (ref r542))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 40504) (line 639) (column 7) (len 55)) (normalized "ISO-80000-7 item 7-11.4 luminous efficacy of a source "))) (attribute-def (declaration-name "LuminousEfficacyOfASourceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r543)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40658) (line 642) (column 11) (len 580)) (normalized "source: item 7-11.4 luminous efficacy of a source\nsymbol(s): `η_v`, `(η)`\napplication domain: generic\nname: LuminousEfficacyOfASource\nquantity dimension: L^-2*M^-1*T^3*J^1\nmeasurement unit(s): lm/W, cd*sr*kg^-1*m^-2*s^3\ntensor order: 0\ndefinition: quotient of the luminous flux emitted and the power consumed by the source, expressed by `η_v = Φ_v/P`, where `Φ_v` is luminous flux (item 7-13) and `P` is the power (ISO 80000-4) consumed by the source\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r544)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r545)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r546)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r547)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousEfficacyOfASource") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r548)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousEfficacyOfASourceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r549)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r550)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r551)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41591) (line 660) (column 77) (len 5)) (member-access (base (expression (span (offset 41591) (line 660) (column 77) (len 3)) (ref r552))) (separator dot) (member (ref r553))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r554)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41613) (line 660) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 41614) (line 660) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r555)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r556)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41693) (line 661) (column 75) (len 5)) (member-access (base (expression (span (offset 41693) (line 661) (column 75) (len 3)) (ref r557))) (separator dot) (member (ref r558))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r559)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41715) (line 661) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 41716) (line 661) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r560)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r561)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41799) (line 662) (column 79) (len 5)) (member-access (base (expression (span (offset 41799) (line 662) (column 79) (len 3)) (ref r562))) (separator dot) (member (ref r563))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r564)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41821) (line 662) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r565)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r566)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41913) (line 663) (column 88) (len 5)) (member-access (base (expression (span (offset 41913) (line 663) (column 88) (len 3)) (ref r567))) (separator dot) (member (ref r568))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r569)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41935) (line 663) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r570)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r571)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42009) (line 664) (column 70) (len 51)) (tuple (expression (span (offset 42010) (line 664) (column 71) (len 8)) (ref r572)) (expression (span (offset 42020) (line 664) (column 81) (len 6)) (ref r573)) (expression (span (offset 42028) (line 664) (column 89) (len 10)) (ref r574)) (expression (span (offset 42040) (line 664) (column 101) (len 19)) (ref r575))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 42077) (line 667) (column 7) (len 58)) (normalized "ISO-80000-7 item 7-12 luminous energy, quantity of light "))) (attribute-def (declaration-name "LuminousEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r576)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 42223) (line 670) (column 11) (len 1189)) (normalized "source: item 7-12 luminous energy, quantity of light\nsymbol(s): `Q_v`, `(Q)`\napplication domain: generic\nname: LuminousEnergy\nquantity dimension: T^1*J^1\nmeasurement unit(s): lm*s, cd*sr*s\ntensor order: 0\ndefinition: energy of electromagnetic waves weighted by the spectral luminous efficiency (item 7-10.2) multiplied by maximum luminous efficacy (item 7-11.3) of a specified photometric condition\nremarks: Luminous energy for photopic vision is expressed by `Q_v = K_m int_0^∞ Q_(e,λ)(λ) V(λ) dλ`, where `Q_(e,λ)(λ)` is the spectral radiant energy (item 7-2.2) at wavelength `λ` (ISO 80000-3), `V(λ)` is spectral luminous efficiency (item 7-10.2), and `K_m` is maximum luminous efficacy (7-11.3). Luminous energy can be emitted, transferred or received. Luminous energy can be expressed by the time integral of the luminous flux (item 7-13), `Φ_v`, over a given duration (ISO 80000-3), `Δt`: `Q_v = int_(Δt) Φ_v dt` . The corresponding radiometric quantity is \"radiant energy\" (item 7-2.1). The corresponding quantity for photons is \"photon energy\" (item 7-19.2).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r577)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r578)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r579)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r580)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r581)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r582)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r583)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r584)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43723) (line 688) (column 79) (len 5)) (member-access (base (expression (span (offset 43723) (line 688) (column 79) (len 3)) (ref r585))) (separator dot) (member (ref r586))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r587)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43745) (line 688) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r588)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r589)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43837) (line 689) (column 88) (len 5)) (member-access (base (expression (span (offset 43837) (line 689) (column 88) (len 3)) (ref r590))) (separator dot) (member (ref r591))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r592)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43859) (line 689) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r593)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r594)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43933) (line 690) (column 70) (len 33)) (tuple (expression (span (offset 43934) (line 690) (column 71) (len 10)) (ref r595)) (expression (span (offset 43946) (line 690) (column 83) (len 19)) (ref r596))))))) (body semicolon)))))) (alias (name "QuantityOfLightUnit") (target (ref r597)) (body semicolon)) (alias (name "QuantityOfLightValue") (target (ref r598)) (body semicolon)) (alias (name "quantityOfLight") (target (ref r599)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 44140) (line 697) (column 7) (len 37)) (normalized "ISO-80000-7 item 7-13 luminous flux "))) (attribute-def (declaration-name "LuminousFluxValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r600)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 44263) (line 700) (column 11) (len 1168)) (normalized "source: item 7-13 luminous flux\nsymbol(s): `Φ_v`, `(Φ)`\napplication domain: generic\nname: LuminousFlux\nquantity dimension: J^1\nmeasurement unit(s): lm, cd*sr\ntensor order: 0\ndefinition: change in luminous energy with time, expressed by `Φ_v = (d Q_v)/(dt)`, where `Q_v` is the luminous energy (item 7-12) emitted, transferred or received and `t` is time (ISO 80000-3)\nremarks: Luminous flux is a quantity derived from the radiant flux (item 7-4.1), `Φ_e`, by evaluating the radiation according to its action upon the CIE standard photometric observer. (See CIE S 017/E:2011, term 17-738.) Luminous flux can be derived from the spectral radiant flux distribution by `Φ_v = K_m int_0^oo Φ_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `Φ_(e,λ)(λ)` is spectral radiant flux (item 7-4.2), `V(λ)` is spectral luminous efficiency (item 7-10.2) and `λ` is wavelength (ISO 80000-3). The corresponding radiometric quantity is \"radiant flux\" (item 7-4.1). The corresponding quantity for photons is \"photon flux\" (item 7-20).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r601)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r602)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r603)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r604)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousFlux") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r605)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousFluxUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r606)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r607)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r608)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45743) (line 718) (column 88) (len 5)) (member-access (base (expression (span (offset 45743) (line 718) (column 88) (len 3)) (ref r609))) (separator dot) (member (ref r610))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r611)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45765) (line 718) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r612)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r613)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45839) (line 719) (column 70) (len 19)) (ref r614))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45875) (line 722) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-14 luminous intensity "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45926) (line 723) (column 7) (len 94)) (normalized "See package ISQBase for the declarations of LuminousIntensityValue and LuminousIntensityUnit "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 46030) (line 725) (column 7) (len 33)) (normalized "ISO-80000-7 item 7-15 luminance "))) (attribute-def (declaration-name "LuminanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r615)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 46146) (line 728) (column 11) (len 1238)) (normalized "source: item 7-15 luminance\nsymbol(s): `L_v`, `(L)`\napplication domain: generic\nname: Luminance\nquantity dimension: L^-2*J^1\nmeasurement unit(s): cd*m^-2\ntensor order: 0\ndefinition: density of luminous intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_v = (dI_v)/(dA) 1/cos(α)`, where `I_v` is luminous intensity (item 7-14), `A` is area (ISO 80000-3) and `α` is the angle between the normal to the surface at the specified point and the specified direction\nremarks: Luminance can be derived from the spectral radiance distribution by `L_v = K_m int_0^∞ L_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `L_(e,λ)(λ)` is the spectral radiance (item 7-6.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also 0.1. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is \"radiance\" (item 7-6.1). The corresponding quantity for photons is \"photon radiance\" (item 7-22).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r616)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r617)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r618)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r619)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r620)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r621)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r622)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r623)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47673) (line 746) (column 77) (len 5)) (member-access (base (expression (span (offset 47673) (line 746) (column 77) (len 3)) (ref r624))) (separator dot) (member (ref r625))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r626)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47695) (line 746) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 47696) (line 746) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r627)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r628)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47788) (line 747) (column 88) (len 5)) (member-access (base (expression (span (offset 47788) (line 747) (column 88) (len 3)) (ref r629))) (separator dot) (member (ref r630))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r631)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47810) (line 747) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r632)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r633)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 47884) (line 748) (column 70) (len 31)) (tuple (expression (span (offset 47885) (line 748) (column 71) (len 8)) (ref r634)) (expression (span (offset 47895) (line 748) (column 81) (len 19)) (ref r635))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 47932) (line 751) (column 7) (len 35)) (normalized "ISO-80000-7 item 7-16 illuminance "))) (attribute-def (declaration-name "IlluminanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r636)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 48052) (line 754) (column 11) (len 1683)) (normalized "source: item 7-16 illuminance\nsymbol(s): `E_v`, `(E)`\napplication domain: generic\nname: Illuminance\nquantity dimension: L^-2*J^1\nmeasurement unit(s): lx, cd*sr*m^-2\ntensor order: 0\ndefinition: density of incident luminous flux with respect to area at a point on a real or imaginary surface, expressed by `E_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) on which the luminous flux is incident\nremarks: Illuminance can be derived from the spectral irradiance distribution by `E_v = K_m int_0^∞ E_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `E_(e,λ)(λ)` is the spectral irradiance (item 7-7.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is \"irradiance\" (item 7-7.1). The corresponding quantity for photons is \"photon irradiance\" (item 7-23). The quantity \"spherical illuminance\" is defined by the mean value of illuminance on the outer curved surface of a very small (real or imaginary) sphere at a point in space. It can be expressed by `E_(v,0) = int_(4π) L_v dΩ`, where `Ω` is solid angle (ISO 80000-3) and `L_v` is luminance (item 7-15). It can be expressed by the quotient of the luminous flux (item 7-13) of all the light incident on the outer surface of an infinitely small sphere centred at the given point, and the area (ISO 80000-3) of the diametrical cross-section of that sphere.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r637)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r638)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r639)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r640)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "illuminance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r641)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "IlluminanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r642)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r643)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r644)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50032) (line 772) (column 77) (len 5)) (member-access (base (expression (span (offset 50032) (line 772) (column 77) (len 3)) (ref r645))) (separator dot) (member (ref r646))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r647)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50054) (line 772) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 50055) (line 772) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r648)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r649)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50147) (line 773) (column 88) (len 5)) (member-access (base (expression (span (offset 50147) (line 773) (column 88) (len 3)) (ref r650))) (separator dot) (member (ref r651))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r652)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50169) (line 773) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r653)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r654)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50243) (line 774) (column 70) (len 31)) (tuple (expression (span (offset 50244) (line 774) (column 71) (len 8)) (ref r655)) (expression (span (offset 50254) (line 774) (column 81) (len 19)) (ref r656))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 50291) (line 777) (column 7) (len 41)) (normalized "ISO-80000-7 item 7-17 luminous exitance "))) (attribute-def (declaration-name "LuminousExitanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r657)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50422) (line 780) (column 11) (len 1160)) (normalized "source: item 7-17 luminous exitance\nsymbol(s): `M_v`, `(M)`\napplication domain: generic\nname: LuminousExitance\nquantity dimension: L^-2*J^1\nmeasurement unit(s): lm/m^2, cd*sr*m^-2\ntensor order: 0\ndefinition: density of exiting luminous flux with respect to area at a point on a real or imaginary surface, expressed by `M_v = (dΦ_v)/(dA)`, where `Φ_v` is luminous flux (item 7-13) and `A` is the area (ISO 80000-3) from which the luminous flux leaves\nremarks: Luminous exitance can be derived from the spectral radiant exitance distribution by `M_v = K_m int_0^∞ M_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `M_(e_λ)(λ)` is the spectral radiant exitance (item 7-8.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is \"radiant exitance\" (item 7-8.1). The corresponding quantity for photons is \"photon exitance\" (item 7-24).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r658)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r659)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r660)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r661)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousExitance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r662)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousExitanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r663)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r664)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r665)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51899) (line 798) (column 77) (len 5)) (member-access (base (expression (span (offset 51899) (line 798) (column 77) (len 3)) (ref r666))) (separator dot) (member (ref r667))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r668)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51921) (line 798) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 51922) (line 798) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r669)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r670)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52014) (line 799) (column 88) (len 5)) (member-access (base (expression (span (offset 52014) (line 799) (column 88) (len 3)) (ref r671))) (separator dot) (member (ref r672))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r673)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52036) (line 799) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r674)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r675)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52110) (line 800) (column 70) (len 31)) (tuple (expression (span (offset 52111) (line 800) (column 71) (len 8)) (ref r676)) (expression (span (offset 52121) (line 800) (column 81) (len 19)) (ref r677))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 52158) (line 803) (column 7) (len 83)) (normalized "ISO-80000-7 item 7-18 luminous exposure, quantity of illumination, light exposure "))) (attribute-def (declaration-name "LuminousExposureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r678)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 52331) (line 806) (column 11) (len 1214)) (normalized "source: item 7-18 luminous exposure, quantity of illumination, light exposure\nsymbol(s): `H_v`, `(H)`\napplication domain: generic\nname: LuminousExposure\nquantity dimension: L^-2*T^1*J^1\nmeasurement unit(s): lx*s, cd*sr*m^-2*s\ntensor order: 0\ndefinition: density of incident luminous energy with respect to area at a point on a real or imaginary surface, expressed by `H_v = (dQ_v)/(dA)`, where `Q_v` is luminous energy (item 7-12) and `A` is the area on which the luminous energy is incident (ISO 80000-3)\nremarks: Luminous exposure can be derived from the spectral radiant exposure distribution by `H_v = K_m int_0^∞ H_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `H_(e_λ)(λ)` is the spectral radiant exposure (item 7-9.2) at wavelength λ(ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. The corresponding radiometric quantity is \"radiant exposure\" (item 7-9.1). The corresponding quantity for photons is \"photon exposure\" (item 7-25).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r679)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r680)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r681)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r682)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "luminousExposure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r683)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LuminousExposureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r684)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r685)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r686)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53862) (line 824) (column 77) (len 5)) (member-access (base (expression (span (offset 53862) (line 824) (column 77) (len 3)) (ref r687))) (separator dot) (member (ref r688))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r689)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53884) (line 824) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 53885) (line 824) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r690)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r691)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53968) (line 825) (column 79) (len 5)) (member-access (base (expression (span (offset 53968) (line 825) (column 79) (len 3)) (ref r692))) (separator dot) (member (ref r693))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r694)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53990) (line 825) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r695)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r696)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54082) (line 826) (column 88) (len 5)) (member-access (base (expression (span (offset 54082) (line 826) (column 88) (len 3)) (ref r697))) (separator dot) (member (ref r698))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r699)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54104) (line 826) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r700)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r701)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54178) (line 827) (column 70) (len 43)) (tuple (expression (span (offset 54179) (line 827) (column 71) (len 8)) (ref r702)) (expression (span (offset 54189) (line 827) (column 81) (len 10)) (ref r703)) (expression (span (offset 54201) (line 827) (column 93) (len 19)) (ref r704))))))) (body semicolon)))))) (alias (name "QuantityOfIlluminationUnit") (target (ref r705)) (body semicolon)) (alias (name "QuantityOfIlluminationValue") (target (ref r706)) (body semicolon)) (alias (name "quantityOfIllumination") (target (ref r707)) (body semicolon)) (alias (name "LightExposureUnit") (target (ref r708)) (body semicolon)) (alias (name "LightExposureValue") (target (ref r709)) (body semicolon)) (alias (name "lightExposure") (target (ref r710)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 54579) (line 838) (column 7) (len 58)) (normalized "ISO-80000-7 item 7-19.1 photon number, number of photons "))) (attribute-def (declaration-name "PhotonNumberValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r711)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 54721) (line 841) (column 11) (len 743)) (normalized "source: item 7-19.1 photon number, number of photons\nsymbol(s): `N_p`\napplication domain: generic\nname: PhotonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of radiant energy and photon energy, expressed by `N_p = Q_e/(h ν)`, where `Q_e` is radiant energy (item 7-2.1), `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave\nremarks: Photon number can also be expressed by the time integral of the photon flux (item 7-20), `Φ_p`, over a given duration, `Δt`, `N_p = int_(Δt) Φ_p dt`\n"))))) (attribute-def (declaration-name "photonNumber") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r712)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "numberOfPhotons") (target (ref r713)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 55592) (line 857) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-19.2 photon energy "))) (attribute-def (declaration-name "photonEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r714)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 55718) (line 860) (column 11) (len 835)) (normalized "source: item 7-19.2 photon energy\nsymbol(s): `Q_p`, `(Q)`\napplication domain: generic\nname: PhotonEnergy (specializes Energy)\nquantity dimension: L^2*M^1*T^-2\nmeasurement unit(s): J, kg*m^2*s^-2\ntensor order: 0\ndefinition: product of the Planck constant and frequency, expressed by `Q_p = h ν` where `h` is the Planck constant (ISO 80000-1) and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave\nremarks: Photon energy can be emitted, transferred or received. For monochromatic radiation, photon energy may be expressed by photon number (item 7-19.1). The corresponding radiometric quantity is \"radiant energy\" (item 7-2.1). The corresponding photometric quantity is \"luminous energy\" (item 7-12).\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 56569) (line 873) (column 7) (len 35)) (normalized "ISO-80000-7 item 7-20 photon flux "))) (attribute-def (declaration-name "PhotonFluxValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r715)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 56688) (line 876) (column 11) (len 881)) (normalized "source: item 7-20 photon flux\nsymbol(s): `Φ_p`, `(Φ)`\napplication domain: generic\nname: PhotonFlux\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: rate of photon number per time interval, expressed by `Φ_p = (d N_p)/(dt)`, where `N_p` is photon number (e.g. given by item 7-19.1), transmitted or received, and `t` is time (ISO 80000-3)\nremarks: Photon flux `Φ_p` is related to radiant flux (item 7-4.1), `Φ_e`, of monochromatic radiation, by `Φ_p = Φ_e/(h ν)` where `h` is the Planck constant (ISO 80000-1), and `ν` is the frequency (ISO 80000-3) of the corresponding electromagnetic wave. The corresponding radiometric quantity is \"radiant flux\" (item 7-4.1). The corresponding photometric quantity is \"luminous flux\" (item 7-13).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r716)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r717)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r718)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r719)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonFlux") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r720)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonFluxUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r721)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r722)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r723)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57864) (line 894) (column 79) (len 5)) (member-access (base (expression (span (offset 57864) (line 894) (column 79) (len 3)) (ref r724))) (separator dot) (member (ref r725))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r726)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57886) (line 894) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 57887) (line 894) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r727)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r728)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57961) (line 895) (column 70) (len 10)) (ref r729))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 57988) (line 898) (column 7) (len 40)) (normalized "ISO-80000-7 item 7-21 photon intensity "))) (attribute-def (declaration-name "PhotonIntensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r730)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 58117) (line 901) (column 11) (len 1009)) (normalized "source: item 7-21 photon intensity\nsymbol(s): `I_p`, `(I)`\napplication domain: generic\nname: PhotonIntensity\nquantity dimension: T^-1\nmeasurement unit(s): s^-1*sr^-1\ntensor order: 0\ndefinition: density of photon flux with respect to solid angle in a specified direction, expressed by `I_p = (dΦ_p)/(dΩ)`, where `Φ_p` is the photon flux (item 7-20) emitted in the given direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction\nremarks: The distribution of the photon intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)` , is used to determine the photon flux (item 7-20) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_p = int int_Ω I_v(θ,ϕ) sin(θ) dϕ dθ`. The corresponding radiometric quantity is \"radiant intensity\" (item 7-5.1). The corresponding photometric quantity is \"luminous intensity\" (item 7-14).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r731)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r732)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r733)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r734)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonIntensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r735)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonIntensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r736)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r737)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r738)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59441) (line 919) (column 79) (len 5)) (member-access (base (expression (span (offset 59441) (line 919) (column 79) (len 3)) (ref r739))) (separator dot) (member (ref r740))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r741)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59463) (line 919) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 59464) (line 919) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r742)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r743)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59538) (line 920) (column 70) (len 10)) (ref r744))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 59565) (line 923) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-22 photon radiance "))) (attribute-def (declaration-name "PhotonRadianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r745)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 59692) (line 926) (column 11) (len 810)) (normalized "source: item 7-22 photon radiance\nsymbol(s): `L_p`, `(L)`\napplication domain: generic\nname: PhotonRadiance\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1*sr^-1\ntensor order: 0\ndefinition: density of photon intensity with respect to projected area in a specified direction at a specified point on a real or imaginary surface, expressed by `L_p = (dI_p)/(dA) 1/cos(α)`, where `I_p` is photon intensity (item 7-21), `A` is area (ISO 80000-3) and `α` the angle between the normal to the surface at the specified point and the specified direction\nremarks: The corresponding radiometric quantity is \"radiance\" (item 7-6.1). The corresponding photometric quantity is \"luminance\" (item 7-15).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r746)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r747)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r748)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r749)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonRadiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r750)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonRadianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r751)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r752)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r753)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60811) (line 944) (column 77) (len 5)) (member-access (base (expression (span (offset 60811) (line 944) (column 77) (len 3)) (ref r754))) (separator dot) (member (ref r755))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r756)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60833) (line 944) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 60834) (line 944) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r757)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r758)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60917) (line 945) (column 79) (len 5)) (member-access (base (expression (span (offset 60917) (line 945) (column 79) (len 3)) (ref r759))) (separator dot) (member (ref r760))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r761)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 60939) (line 945) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 60940) (line 945) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r762)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r763)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61014) (line 946) (column 70) (len 22)) (tuple (expression (span (offset 61015) (line 946) (column 71) (len 8)) (ref r764)) (expression (span (offset 61025) (line 946) (column 81) (len 10)) (ref r765))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 61053) (line 949) (column 7) (len 41)) (normalized "ISO-80000-7 item 7-23 photon irradiance "))) (attribute-def (declaration-name "PhotonIrradianceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r766)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 61184) (line 952) (column 11) (len 698)) (normalized "source: item 7-23 photon irradiance\nsymbol(s): `E_p`, `(E)`\napplication domain: generic\nname: PhotonIrradiance\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1\ntensor order: 0\ndefinition: density of incident photon flux with respect to area at a point on a real or imaginary surface, expressed by `E_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) on which the photon flux is incident\nremarks: The corresponding radiometric quantity is \"irradiance\" (item 7-7.1). The corresponding photometric quantity is \"illuminance\" (item 7-16).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r767)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r768)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r769)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r770)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonIrradiance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r771)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonIrradianceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r772)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r773)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r774)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62199) (line 970) (column 77) (len 5)) (member-access (base (expression (span (offset 62199) (line 970) (column 77) (len 3)) (ref r775))) (separator dot) (member (ref r776))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r777)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62221) (line 970) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 62222) (line 970) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r778)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r779)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62305) (line 971) (column 79) (len 5)) (member-access (base (expression (span (offset 62305) (line 971) (column 79) (len 3)) (ref r780))) (separator dot) (member (ref r781))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r782)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62327) (line 971) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 62328) (line 971) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r783)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r784)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62402) (line 972) (column 70) (len 22)) (tuple (expression (span (offset 62403) (line 972) (column 71) (len 8)) (ref r785)) (expression (span (offset 62413) (line 972) (column 81) (len 10)) (ref r786))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 62441) (line 975) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-24 photon exitance "))) (attribute-def (declaration-name "PhotonExitanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r787)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 62568) (line 978) (column 11) (len 702)) (normalized "source: item 7-24 photon exitance\nsymbol(s): `M_p`, `(M)`\napplication domain: generic\nname: PhotonExitance\nquantity dimension: L^-2*T^-1\nmeasurement unit(s): m^-2*s^-1\ntensor order: 0\ndefinition: density of exiting photon flux with respect to area at a point on a real or imaginary surface, expressed by `M_p = (dΦ_p)/(dA)`, where `Φ_p` is photon flux (item 7-20) and `A` is the area (ISO 80000-3) from which the photon flux leaves\nremarks: The corresponding radiometric quantity is \"radiant exitance\" (item 7-8.1). The corresponding photometric quantity is \"luminous exitance\" (item 7-17).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r788)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r789)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r790)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r791)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonExitance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r792)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonExitanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r793)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r794)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r795)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63579) (line 996) (column 77) (len 5)) (member-access (base (expression (span (offset 63579) (line 996) (column 77) (len 3)) (ref r796))) (separator dot) (member (ref r797))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r798)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63601) (line 996) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 63602) (line 996) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r799)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r800)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63685) (line 997) (column 79) (len 5)) (member-access (base (expression (span (offset 63685) (line 997) (column 79) (len 3)) (ref r801))) (separator dot) (member (ref r802))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r803)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63707) (line 997) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 63708) (line 997) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r804)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r805)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63782) (line 998) (column 70) (len 22)) (tuple (expression (span (offset 63783) (line 998) (column 71) (len 8)) (ref r806)) (expression (span (offset 63793) (line 998) (column 81) (len 10)) (ref r807))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 63821) (line 1001) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-25 photon exposure "))) (attribute-def (declaration-name "PhotonExposureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r808)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 63948) (line 1004) (column 11) (len 697)) (normalized "source: item 7-25 photon exposure\nsymbol(s): `H_p`, `(H)`\napplication domain: generic\nname: PhotonExposure\nquantity dimension: L^-2\nmeasurement unit(s): m^-2\ntensor order: 0\ndefinition: density of incident photon number with respect to area at a point on a real or imaginary surface, expressed by `H_p = (dN_p)/(dA)`, where `N_p` is photon number (item 7-19.1) and `A` is the area (ISO 80000-3) on which the photons are incident\nremarks: The corresponding radiometric quantity is \"radiant exposure\" (item 7-9.1). The corresponding photometric quantity is \"luminous exposure\" (item 7-18).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r809)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r810)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r811)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r812)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "photonExposure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r813)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhotonExposureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r814)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r815)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r816)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 64954) (line 1022) (column 77) (len 5)) (member-access (base (expression (span (offset 64954) (line 1022) (column 77) (len 3)) (ref r817))) (separator dot) (member (ref r818))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r819)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 64976) (line 1022) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 64977) (line 1022) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r820)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r821)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65051) (line 1023) (column 70) (len 8)) (ref r822))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 65076) (line 1026) (column 7) (len 92)) (normalized "ISO-80000-7 item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer "))) (attribute-def (declaration-name "TristimulusValuesForTheCie1931StandardColorimetricObserverValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r823)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 65300) (line 1029) (column 11) (len 1756)) (normalized "source: item 7-26.1 tristimulus values for the CIE 1931 standard colorimetric observer\nsymbol(s): `X,Y,Z`\napplication domain: generic\nname: TristimulusValuesForTheCie1931StandardColorimetricObserver\nquantity dimension: L^-2*J^1\nmeasurement unit(s): cd*m^-2\ntensor order: 0\ndefinition: amounts of the three reference colour stimuli in the CIE 1931 standard colorimetric system, required to match the colour of the stimulus considered\nremarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[cd*m^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 // int_0^∞ S_λ(λ) overline y(λ) dλ`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r824)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r825)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r826)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r827)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "tristimulusValuesForTheCie1931StandardColorimetricObserver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r828)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "TristimulusValuesForTheCie1931StandardColorimetricObserverUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r829)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r830)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r831)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67541) (line 1047) (column 77) (len 5)) (member-access (base (expression (span (offset 67541) (line 1047) (column 77) (len 3)) (ref r832))) (separator dot) (member (ref r833))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r834)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67563) (line 1047) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 67564) (line 1047) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r835)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r836)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67656) (line 1048) (column 88) (len 5)) (member-access (base (expression (span (offset 67656) (line 1048) (column 88) (len 3)) (ref r837))) (separator dot) (member (ref r838))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r839)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67678) (line 1048) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r840)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r841)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67752) (line 1049) (column 70) (len 31)) (tuple (expression (span (offset 67753) (line 1049) (column 71) (len 8)) (ref r842)) (expression (span (offset 67763) (line 1049) (column 81) (len 19)) (ref r843))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 67800) (line 1052) (column 7) (len 92)) (normalized "ISO-80000-7 item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer "))) (attribute-def (declaration-name "TristimulusValuesForTheCie1964StandardColorimetricObserverValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r844)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 68024) (line 1055) (column 11) (len 1770)) (normalized "source: item 7-26.2 tristimulus values for the CIE 1964 standard colorimetric observer\nsymbol(s): `X_10,Y_10,Z_10`\napplication domain: generic\nname: TristimulusValuesForTheCie1964StandardColorimetricObserver\nquantity dimension: L^-2*J^1\nmeasurement unit(s): cd*m^-2\ntensor order: 0\ndefinition: amounts of the three reference colour stimuli in the CIE 1964 standard colorimetric system, required to match the colour of the stimulus considered\nremarks: For a given colour stimulus described by the colour stimulus function `φ_λ(λ)` of a radiometric quantity, `X = k int_0^∞ φ_λ(λ) overline x(λ) dλ`, `Y = k int_0^∞ φ_λ(λ) overline y(λ) dλ`, `Z = k int_0^∞ φ_λ(λ) overline z(λ) dλ`, where `overline x(λ)`, `overline y(λ)`, `overline z(λ)` are the CIE colour-matching functions for the CIE 1931 standard colorimetric observer (2° observer) (item 7-27.1). For sources, `k` may be chosen as `k = K_m` where `K_m` is the maximum luminous efficacy (item 7-11.3) so that `Y = L_v` (item 7-15) and the unit of `X`, `Y`, `Z` is `[\"cd\"*\"m\"^-2]`. For object colours, `φ_λ(λ)` is given by one of the three products `φ_λ(λ) = S_λ(λ) * {(ρ(λ)), (τ(λ)), (β(λ)):}` where `S_λ(λ)` is the relative spectral distribution of a quantity characterizing the source illuminating the object, `ρ(λ)` is the spectral reflectance, `τ(λ)` is the spectral transmittance, `β(λ)` is the spectral radiance factor, and `k` is chosen to be `k = 100 /( int_0^∞ S_λ(λ) overline y(λ) dλ)`. Integral limits can be confined depending on the spectral sensitivity of the detectors used as a sensor. In this case, the unit of `X`, `Y`, `Z` is `[1]`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r845)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r846)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r847)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r848)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "tristimulusValuesForTheCie1964StandardColorimetricObserver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r849)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "TristimulusValuesForTheCie1964StandardColorimetricObserverUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r850)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r851)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r852)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70279) (line 1073) (column 77) (len 5)) (member-access (base (expression (span (offset 70279) (line 1073) (column 77) (len 3)) (ref r853))) (separator dot) (member (ref r854))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r855)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70301) (line 1073) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 70302) (line 1073) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r856)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r857)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70394) (line 1074) (column 88) (len 5)) (member-access (base (expression (span (offset 70394) (line 1074) (column 88) (len 3)) (ref r858))) (separator dot) (member (ref r859))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r860)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70416) (line 1074) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r861)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r862)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70490) (line 1075) (column 70) (len 31)) (tuple (expression (span (offset 70491) (line 1075) (column 71) (len 8)) (ref r863)) (expression (span (offset 70501) (line 1075) (column 81) (len 19)) (ref r864))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 70538) (line 1078) (column 7) (len 103)) (normalized "ISO-80000-7 item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer "))) (attribute-def (declaration-name "CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserverValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r865)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 70780) (line 1081) (column 11) (len 815)) (normalized "source: item 7-27.1 CIE colour-matching functions for the CIE 1931 standard colorimetric observer\nsymbol(s): `overline x(λ)`, `overline y(λ)`, `overline z(λ)`\napplication domain: generic\nname: CieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: functions `overline x(λ)` , `overline y(λ)` , `overline z(λ)` in the CIE 1931 standard colorimetric system\nremarks: Values of `overline x(λ)` , `overline y(λ)` and `overline z(λ)` are defined in the CIE 1931 standard colorimetric system (2° observer) — applicable to fields of observation of angular opening from 1° to 4°.\n"))))) (attribute-def (declaration-name "cieColourMatchingFunctionsForTheCie1931StandardColorimetricObserver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r866)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 71788) (line 1095) (column 7) (len 103)) (normalized "ISO-80000-7 item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer "))) (attribute-def (declaration-name "CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserverValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r867)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 72030) (line 1098) (column 11) (len 837)) (normalized "source: item 7-27.2 CIE colour-matching functions for the CIE 1964 standard colorimetric observer\nsymbol(s): `overline x_10(λ)`, `overline y_10(λ)`, `overline z_10(λ)`\napplication domain: generic\nname: CieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: functions `overline x_10(λ)` , `overline y_10(λ)` , `overline z_10(λ)` in the CIE 1964 standard colorimetric system\nremarks: Values of `overline x_10(λ)` , `overline y_10(λ)` and `overline z_10(λ)` are defined in the CIE 1964 standard colorimetric system (10° observer) — applicable to fields of observation with angles greater than 4°.\n"))))) (attribute-def (declaration-name "cieColourMatchingFunctionsForTheCie1964StandardColorimetricObserver") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r868)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 73060) (line 1112) (column 7) (len 95)) (normalized "ISO-80000-7 item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system "))) (attribute-def (declaration-name "ChromaticityCoordinatesInTheCie1931StandardColorimetricSystemValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r869)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 73288) (line 1115) (column 11) (len 738)) (normalized "source: item 7-28.1 chromaticity coordinates in the CIE 1931 standard colorimetric system\nsymbol(s): `x,y,z`\napplication domain: generic\nname: ChromaticityCoordinatesInTheCie1931StandardColorimetricSystem (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1931 standard colorimetric observer (item 7-26.1) and their sum, expressed by `x = X / (X+Y+Z)` , `y = Y / (X+Y+Z)` , `z = Z / (X+Y+Z)`\nremarks: Since `x + y + z = 1`, two variables are sufficient to express chromaticity.\n"))))) (attribute-def (declaration-name "chromaticityCoordinatesInTheCie1931StandardColorimetricSystem") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r870)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 74207) (line 1129) (column 7) (len 95)) (normalized "ISO-80000-7 item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system "))) (attribute-def (declaration-name "ChromaticityCoordinatesInTheCie1964StandardColorimetricSystemValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r871)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 74435) (line 1132) (column 11) (len 799)) (normalized "source: item 7-28.2 chromaticity coordinates in the CIE 1964 standard colorimetric system\nsymbol(s): `x_10,y_10,z_10`\napplication domain: generic\nname: ChromaticityCoordinatesInTheCie1964StandardColorimetricSystem (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: coordinates expressing the quotients of each of a set of three tristimulus values for the CIE 1964 standard colorimetric observer (item 7-26.2) and their sum, expressed by `x_10 = X_10 / (X_10+Y_10+Z_10)`, `y_10 = Y_10 / (X_10+Y_10+Z_10)`, `z_10 = Z_10 / (X_10+Y_10+Z_10)`\nremarks: Since `x_10 + y_10 + z_10 = 1`, two variables are sufficient to express chromaticity.\n"))))) (attribute-def (declaration-name "chromaticityCoordinatesInTheCie1964StandardColorimetricSystem") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r872)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 75415) (line 1146) (column 7) (len 44)) (normalized "ISO-80000-7 item 7-29.1 colour temperature "))) (attribute-def (declaration-name "colourTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r873)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 75569) (line 1149) (column 11) (len 453)) (normalized "source: item 7-29.1 colour temperature\nsymbol(s): `T_c`\napplication domain: generic\nname: ColourTemperature (specializes ThermodynamicTemperature)\nquantity dimension: Θ^1\nmeasurement unit(s): K\ntensor order: 0\ndefinition: temperature of a Planckian radiator whose radiation has the same chromaticity as that of a given stimulus\nremarks: None.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 76038) (line 1162) (column 7) (len 55)) (normalized "ISO-80000-7 item 7-29.2 correlated colour temperature "))) (attribute-def (declaration-name "correlatedColourTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r874)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 76213) (line 1165) (column 11) (len 651)) (normalized "source: item 7-29.2 correlated colour temperature\nsymbol(s): `T_\"cp\"`\napplication domain: generic\nname: CorrelatedColourTemperature (specializes ThermodynamicTemperature)\nquantity dimension: Θ^1\nmeasurement unit(s): K\ntensor order: 0\ndefinition: temperature of a Planckian radiator having the chromaticity nearest the chromaticity associated with the given spectral distribution on a modified 1976 CIE Uniform Chromaticity Scale (UCS) diagram where `u',2/3 v'` are the coordinates of the Planckian locus and the test stimulus\nremarks: None.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 76880) (line 1178) (column 7) (len 36)) (normalized "ISO-80000-7 item 7-30.1 emissivity "))) (attribute-def (declaration-name "EmissivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r875)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 76998) (line 1181) (column 11) (len 645)) (normalized "source: item 7-30.1 emissivity\nsymbol(s): `ε`, `ε_T`\napplication domain: generic\nname: Emissivity (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the radiant exitance of a radiator and the radiant exitance of a Planckian radiator at the same temperature, expressed by `ε = M/M_b`, where `M` is the radiant exitance (item 7-8.1) of a thermal radiator and `M_b` is the radiant exitance of a Planckian radiator at the same temperature (ISO 80000-5)\nremarks: None.\n"))))) (attribute-def (declaration-name "emissivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r876)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 77722) (line 1195) (column 7) (len 62)) (normalized "ISO-80000-7 item 7-30.2 emissivity at a specified wavelength "))) (attribute-def (declaration-name "EmissivityAtASpecifiedWavelengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r877)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 77888) (line 1198) (column 11) (len 816)) (normalized "source: item 7-30.2 emissivity at a specified wavelength\nsymbol(s): `ε(λ)`\napplication domain: generic\nname: EmissivityAtASpecifiedWavelength (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the radiant exitance of a radiator at a specified wavelength and the radiant exitance of a Planckian radiator at the same temperature and at the same wavelength, expressed by `ε(λ) = M(λ) / M_b(λ)`, where `M(λ)` is the radiant exitance (item 7-8.1) of a thermal radiator at a specified wavelength and `M_b(λ)` is the radiant exitance of a Planckian radiator at the same temperature at a specified wavelength (ISO 80000-3)\nremarks: None.\n"))))) (attribute-def (declaration-name "emissivityAtASpecifiedWavelength") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r878)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 78827) (line 1212) (column 7) (len 37)) (normalized "ISO-80000-7 item 7-31.1 absorptance "))) (attribute-def (declaration-name "AbsorptanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r879)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 78947) (line 1215) (column 11) (len 802)) (normalized "source: item 7-31.1 absorptance\nsymbol(s): `α`, `a`\napplication domain: generic\nname: Absorptance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of absorbed radiant flux and incident radiant flux, expressed by `α = Φ_a/Φ_m`, where `Φ_a` is absorbed radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case \"spectral\" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `ρ` is reflectance (item 7-31.3) and `τ` is transmittance (item 7-31.5).\n"))))) (attribute-def (declaration-name "absorptance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r880)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 79830) (line 1229) (column 7) (len 46)) (normalized "ISO-80000-7 item 7-31.2 luminous absorptance "))) (attribute-def (declaration-name "LuminousAbsorptanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r881)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 79967) (line 1232) (column 11) (len 886)) (normalized "source: item 7-31.2 luminous absorptance\nsymbol(s): `α_v`\napplication domain: generic\nname: LuminousAbsorptance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of absorbed luminous flux and incident luminous flux, expressed by `α_v = Φ_(v,a)/Φ_(v,m)`, where `Φ_(v,a)` is absorbed luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux\nremarks: From spectral absorptance, `α(λ)`, luminous absorptance can be calculated by `α_v = (int_0^∞ α(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.1.\n"))))) (attribute-def (declaration-name "luminousAbsorptance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r882)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 80950) (line 1246) (column 7) (len 37)) (normalized "ISO-80000-7 item 7-31.3 reflectance "))) (attribute-def (declaration-name "ReflectanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r883)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 81070) (line 1249) (column 11) (len 800)) (normalized "source: item 7-31.3 reflectance\nsymbol(s): `ρ`\napplication domain: generic\nname: Reflectance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of reflected radiant flux and incident radiant flux, expressed by `ρ = Φ_r/Φ_m`, where `Φ_r` is reflected radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `τ` is transmittance (item 7-31.5).\n"))))) (attribute-def (declaration-name "reflectance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r884)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 81951) (line 1263) (column 7) (len 46)) (normalized "ISO-80000-7 item 7-31.4 luminous reflectance "))) (attribute-def (declaration-name "LuminousReflectanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r885)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 82088) (line 1266) (column 11) (len 891)) (normalized "source: item 7-31.4 luminous reflectance\nsymbol(s): `ρ_v`\napplication domain: generic\nname: LuminousReflectance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of reflected luminous flux and incident luminous flux, is expressed by `ρ_v = Φ_(v,r)/Φ_(v,m)`, where `Φ_(v,r)` is reflected luminous flux (item 7-13) and `Φ_(v,m)` is incident luminous flux\nremarks: From spectral reflectance, `ρ(λ)`, luminous reflectance can be calculated by `ρ_v = (int_0^∞ ρ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is spectral luminous efficiency (item 7-10.2). See also item 7-31.3.\n"))))) (attribute-def (declaration-name "luminousReflectance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r886)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 83076) (line 1280) (column 7) (len 39)) (normalized "ISO-80000-7 item 7-31.5 transmittance "))) (attribute-def (declaration-name "TransmittanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r887)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 83200) (line 1283) (column 11) (len 811)) (normalized "source: item 7-31.5 transmittance\nsymbol(s): `τ`, `T`\napplication domain: generic\nname: Transmittance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of transmitted radiant flux and incident radiant flux, expressed by `τ = Φ_t/Φ_m`, where `Φ_t` is transmitted radiant flux (item 7-4.1) and `Φ_m` is incident radiant flux\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before the quantity name. Due to energy conservation, `α + ρ + τ = 1` except when polarized radiation is observed, where `α` is absorptance (item 7-31.1) and `ρ` is reflectance (item 7-31.3).\n"))))) (attribute-def (declaration-name "transmittance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r888)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 84096) (line 1297) (column 7) (len 48)) (normalized "ISO-80000-7 item 7-31.6 luminous transmittance "))) (attribute-def (declaration-name "LuminousTransmittanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r889)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 84237) (line 1300) (column 11) (len 928)) (normalized "source: item 7-31.6 luminous transmittance\nsymbol(s): `τ_v`\napplication domain: generic\nname: LuminousTransmittance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of transmitted luminous flux and incident luminous flux, expressed by `τ_v = Φ_(v,t)/Φ_(v,m)`, where `Φ_(v,t)` is transmitted luminous flux (item 7-13) and `Φ_(v,m)` is luminous flux of the incident radiation\nremarks: From the spectral transmittance `τ(λ)`, luminous transmittance can be calculated by `τ_v = (int_0^∞ τ(λ) Φ_(e,λ)(λ) V(λ) dλ)/(int_0^∞ Φ_(e,λ)(λ) V(λ) dλ)`, where `Φ_(e,λ)(λ)` is the spectral radiant flux (or relative spectral distribution) of the source, and `V(λ)` is the spectral luminous efficiency (item 7-10.2). See also item 7-31.5.\n"))))) (attribute-def (declaration-name "luminousTransmittance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r890)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 85266) (line 1314) (column 7) (len 115)) (normalized "ISO-80000-7 item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance "))) (attribute-def (declaration-name "TransmittanceOpticalDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r891)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 85480) (line 1317) (column 11) (len 750)) (normalized "source: item 7-32.1 transmittance optical density, optical density, transmittance density, decadic absorbance\nsymbol(s): `D`, `A_10`, `D_τ`\napplication domain: generic\nname: TransmittanceOpticalDensity (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: logarithm to base 10 of the reciprocal of the transmittance, `τ` (item 7-31.5)\nremarks: If defined in terms of wavelength, the optical density can be expressed by `A_10(λ) = -log(τ(λ))`, where `τ(λ)` is the transmittance (item 7-31.5) in terms of wavelength. In spectroscopy, the name \"absorbance\" `A_10` is generally used.\n"))))) (attribute-def (declaration-name "transmittanceOpticalDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r892)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "opticalDensity") (target (ref r893)) (body semicolon)) (alias (name "transmittanceDensity") (target (ref r894)) (body semicolon)) (alias (name "decadicAbsorbance") (target (ref r895)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 86529) (line 1337) (column 7) (len 46)) (normalized "ISO-80000-7 item 7-32.2 Napierian absorbance "))) (attribute-def (declaration-name "NapierianAbsorbanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r896)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 86666) (line 1340) (column 11) (len 697)) (normalized "source: item 7-32.2 Napierian absorbance\nsymbol(s): `A_n`, `B`\napplication domain: generic\nname: NapierianAbsorbance (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: natural (Napierian) logarithm of the reciprocal of the transmittance, `τ` (item 7-31.5)\nremarks: If defined in terms of wavelength, the Napierian absorbance can be expressed by `A_n(λ) = B(λ) = -log(τ(λ))`. It can also be expressed as `A_n(λ) = l*α(λ)`, where `α` is linear absorption coefficient (item 7-35.2) and `l` is length (ISO 80000-3) traversed.\n"))))) (attribute-def (declaration-name "napierianAbsorbance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r897)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 87460) (line 1354) (column 7) (len 41)) (normalized "ISO-80000-7 item 7-33.1 radiance factor "))) (attribute-def (declaration-name "RadianceFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r898)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 87587) (line 1357) (column 11) (len 1411)) (normalized "source: item 7-33.1 radiance factor\nsymbol(s): `β_e`, `(β)`\napplication domain: generic\nname: RadianceFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the radiance of a surface element in a specified direction and the radiance of the perfect reflecting diffuser or perfect transmitting diffuser identically irradiated and viewed, expressed by `β_e = L_(e,n)/L_(e,d)`, where `L_(e,n)` is the radiance (item 7-6.1) of a surface element in a given direction and `L_(e,d)` is the radiance of the perfect reflecting or transmitting diffuser identically irradiated and viewed\nremarks: The definition holds for a surface element of a non-self-radiating medium, in a given direction and under specified conditions of irradiation. Radiance factor is equivalent to reflectance factor (item 7-34) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is `2π [\"sr\"]`. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called \"perfect diffuser\".\n"))))) (attribute-def (declaration-name "radianceFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r899)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 89085) (line 1371) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-33.2 luminance factor "))) (attribute-def (declaration-name "LuminanceFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r900)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 89214) (line 1374) (column 11) (len 1068)) (normalized "source: item 7-33.2 luminance factor\nsymbol(s): `β_v`, `(β)`\napplication domain: generic\nname: LuminanceFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the luminance of a surface element in a specified direction and the luminance of the perfect reflecting diffuser or perfect transmitting diffuser identically illuminated and viewed, expressed by `β_v = L_(v,n)/L_(v,d)`, where `L_(v,n)` is the luminance (item 7-15) of a surface element in a given direction and `L_(v,d)` is the luminance of the perfect reflecting or transmitting diffuser identically illuminated and viewed\nremarks: The definition holds for a surface element of a non-luminous medium, in a given direction and under specified conditions of irradiation. This quantity is also defined spectrally and is called \"spectral luminance factor\". For the analogous radiant quantity \"radiance factor\", see item 7-33.1.\n"))))) (attribute-def (declaration-name "luminanceFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r901)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 90371) (line 1388) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-34 reflectance factor "))) (attribute-def (declaration-name "ReflectanceFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r902)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 90502) (line 1391) (column 11) (len 1615)) (normalized "source: item 7-34 reflectance factor\nsymbol(s): `R`\napplication domain: generic\nname: ReflectanceFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the flux reflected in the directions delimited by a given cone with apex at a surface element and the flux reflected in the same directions by a perfect reflecting diffuser identically irradiated or illuminated, expressed by `R = Φ_n/Φ_d`, where `Φ_n` is the flux reflected in the directions delimited by a given cone and `Φ_d` is the flux reflected in the same directions by an identically irradiated diffuser of reflectance (item 7-31.3) equal to 1\nremarks: The flux can be a radiant flux (item 7‐4.1) or a luminous flux (item 7‐13). The definition holds for a surface element, for the part of the reflected radiation contained in a given cone with apex at the surface element, and for incident radiation of given spectral composition, polarization and geometric distribution. Reflectance factor is equivalent to radiance factor (item 7-33.1) or luminance factor (item 7-33.2) when the cone angle is infinitely small, and is equivalent to reflectance (item 7-31.3) when the cone angle is 2π sr. These quantities are also defined spectrally and called spectral radiance factor `β(λ)` and spectral reflectance factor `R(λ)`. The ideal isotropic (Lambertian) diffuser with reflectance (item 7-31.3) or transmittance (item 7-31.5) equal to 1 is called a perfect diffuser.\n"))))) (attribute-def (declaration-name "reflectanceFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r903)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 92210) (line 1405) (column 7) (len 87)) (normalized "ISO-80000-7 item 7-35.1 linear attenuation coefficient, linear extinction coefficient "))) (attribute-def (declaration-name "LinearAttenuationCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r904)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 92399) (line 1408) (column 11) (len 921)) (normalized "source: item 7-35.1 linear attenuation coefficient, linear extinction coefficient\nsymbol(s): `μ`, `μ_l`\napplication domain: radiometry\nname: LinearAttenuationCoefficient\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: relative decrease in radiant flux caused by absorption and scattering\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before this quantity name. The spectral linear attenuation coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing and scattering medium `μ(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. Similarly, luminous and photon quantities can be defined.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r905)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r906)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r907)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r908)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "linearAttenuationCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r909)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LinearAttenuationCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r910)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r911)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r912)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93685) (line 1426) (column 77) (len 5)) (member-access (base (expression (span (offset 93685) (line 1426) (column 77) (len 3)) (ref r913))) (separator dot) (member (ref r914))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r915)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93707) (line 1426) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 93708) (line 1426) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r916)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r917)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 93782) (line 1427) (column 70) (len 8)) (ref r918))))) (body semicolon)))))) (alias (name "LinearExtinctionCoefficientUnit") (target (ref r919)) (body semicolon)) (alias (name "LinearExtinctionCoefficientValue") (target (ref r920)) (body semicolon)) (alias (name "linearExtinctionCoefficient") (target (ref r921)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 94042) (line 1434) (column 7) (len 55)) (normalized "ISO-80000-7 item 7-35.2 linear absorption coefficient "))) (attribute-def (declaration-name "LinearAbsorptionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r922)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 94198) (line 1437) (column 11) (len 1138)) (normalized "source: item 7-35.2 linear absorption coefficient\nsymbol(s): `α_l`, `a_l`, `α`\napplication domain: radiometry\nname: LinearAbsorptionCoefficient\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: relative decrease in radiant flux (item 7-4.1) caused by absorption\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before this quantity name. The spectral linear absorption coefficient can be expressed by the relative decrease in the spectral radiant flux, `Φ_(e,λ)(λ)`, with respect to propagation length, `l`, of a collimated beam at a point in an absorbing medium `α_l(λ) = 1/(Φ_(e,λ)(λ)) (d Φ_(e,λ)(λ))/(dl)`. It can also be expressed as a function of transmittance (item 7-31.5). `α_l = -ln(τ)/l = A_n/l`. The linear absorption coefficient is that part of the linear attenuation coefficient (item 7-35.1) that is due to absorption. Scattering might also contribute. Similarly, luminous and photon quantities can be defined.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r923)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r924)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r925)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r926)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "linearAbsorptionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r927)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LinearAbsorptionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r928)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r929)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r930)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 95697) (line 1455) (column 77) (len 5)) (member-access (base (expression (span (offset 95697) (line 1455) (column 77) (len 3)) (ref r931))) (separator dot) (member (ref r932))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r933)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 95719) (line 1455) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 95720) (line 1455) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r934)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r935)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 95794) (line 1456) (column 70) (len 8)) (ref r936))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 95819) (line 1459) (column 7) (len 54)) (normalized "ISO-80000-7 item 7-36.1 mass attenuation coefficient "))) (attribute-def (declaration-name "MassAttenuationCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r937)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 95973) (line 1462) (column 11) (len 700)) (normalized "source: item 7-36.1 mass attenuation coefficient\nsymbol(s): `μ_m`\napplication domain: radiometry\nname: MassAttenuationCoefficient\nquantity dimension: L^2*M^-1\nmeasurement unit(s): kg^-1*m^2\ntensor order: 0\ndefinition: quotient of the linear attenuation coefficient (item 7-35.1), `μ`, and the mass density (ISO 80000-4), `ρ`, of the medium\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before this quantity name, which can be expressed by `μ_m(λ) = (μ(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r938)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r939)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r940)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r941)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massAttenuationCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r942)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MassAttenuationCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r943)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r944)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r945)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97030) (line 1480) (column 77) (len 5)) (member-access (base (expression (span (offset 97030) (line 1480) (column 77) (len 3)) (ref r946))) (separator dot) (member (ref r947))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r948)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97052) (line 1480) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r949)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r950)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97131) (line 1481) (column 75) (len 5)) (member-access (base (expression (span (offset 97131) (line 1481) (column 75) (len 3)) (ref r951))) (separator dot) (member (ref r952))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r953)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97153) (line 1481) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 97154) (line 1481) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r954)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r955)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 97228) (line 1482) (column 70) (len 18)) (tuple (expression (span (offset 97229) (line 1482) (column 71) (len 8)) (ref r956)) (expression (span (offset 97239) (line 1482) (column 81) (len 6)) (ref r957))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 97263) (line 1485) (column 7) (len 53)) (normalized "ISO-80000-7 item 7-36.2 mass absorption coefficient "))) (attribute-def (declaration-name "MassAbsorptionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r958)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 97415) (line 1488) (column 11) (len 697)) (normalized "source: item 7-36.2 mass absorption coefficient\nsymbol(s): `α_m`\napplication domain: radiometry\nname: MassAbsorptionCoefficient\nquantity dimension: L^2*M^-1\nmeasurement unit(s): kg^-1*m^2\ntensor order: 0\ndefinition: quotient of the linear absorption coefficient (item 7-35.2), `α`, and the mass density (ISO 80000-4), `ρ`, of the medium\nremarks: This quantity is also defined spectrally in terms of wavelength, in which case, \"spectral\" is added before this quantity name, which can be expressed by `α_m(λ) = (α(λ))/ρ_m`. Similarly, luminous and photon quantities can be defined.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r959)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r960)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r961)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r962)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massAbsorptionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r963)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MassAbsorptionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r964)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r965)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r966)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98465) (line 1506) (column 77) (len 5)) (member-access (base (expression (span (offset 98465) (line 1506) (column 77) (len 3)) (ref r967))) (separator dot) (member (ref r968))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r969)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98487) (line 1506) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r970)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r971)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98566) (line 1507) (column 75) (len 5)) (member-access (base (expression (span (offset 98566) (line 1507) (column 75) (len 3)) (ref r972))) (separator dot) (member (ref r973))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r974)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98588) (line 1507) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 98589) (line 1507) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r975)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r976)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 98663) (line 1508) (column 70) (len 18)) (tuple (expression (span (offset 98664) (line 1508) (column 71) (len 8)) (ref r977)) (expression (span (offset 98674) (line 1508) (column 81) (len 6)) (ref r978))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 98698) (line 1511) (column 7) (len 52)) (normalized "ISO-80000-7 item 7-37 molar absorption coefficient "))) (attribute-def (declaration-name "MolarAbsorptionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r979)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 98850) (line 1514) (column 11) (len 709)) (normalized "source: item 7-37 molar absorption coefficient\nsymbol(s): `χ`\napplication domain: radiometry\nname: MolarAbsorptionCoefficient\nquantity dimension: L^2*N^-1\nmeasurement unit(s): m^2*mol^-1\ntensor order: 0\ndefinition: product of linear absorption coefficient and molar volume, expressed by `χ = α V_m`, where `α` is linear absorption coefficient (item 7-35.2) and `V_m` is molar volume (ISO 80000-9)\nremarks: The molar absorption coefficient can also be expressed by `χ = α c` where `c` is amount-of-substance concentration (ISO 80000-9). Similarly, luminous and photon quantities can be defined.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r980)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r981)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r982)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r983)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarAbsorptionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r984)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarAbsorptionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r985)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r986)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r987)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 99916) (line 1532) (column 77) (len 5)) (member-access (base (expression (span (offset 99916) (line 1532) (column 77) (len 3)) (ref r988))) (separator dot) (member (ref r989))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r990)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 99938) (line 1532) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r991)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r992)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100030) (line 1533) (column 88) (len 5)) (member-access (base (expression (span (offset 100030) (line 1533) (column 88) (len 3)) (ref r993))) (separator dot) (member (ref r994))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r995)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100052) (line 1533) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 100053) (line 1533) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r996)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r997)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 100127) (line 1534) (column 70) (len 31)) (tuple (expression (span (offset 100128) (line 1534) (column 71) (len 8)) (ref r998)) (expression (span (offset 100138) (line 1534) (column 81) (len 19)) (ref r999))))))) (body semicolon)))))))))
)
~~~
