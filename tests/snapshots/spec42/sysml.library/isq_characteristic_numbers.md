# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQCharacteristicNumbers"))
~~~
# SOURCE
~~~sysml
standard library package ISQCharacteristicNumbers {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 "Characteristic numbers"
     * see also https://www.iso.org/standard/64982.html
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

    /* ISO-80000-11 item 11-4.1 Reynolds number */
    attribute def ReynoldsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.1 Reynolds number
         * symbol(s): `Re`
         * application domain: generic
         * name: ReynoldsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.
         */
    }
    attribute reynoldsNumber: ReynoldsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.2 Euler number */
    attribute def EulerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.2 Euler number
         * symbol(s): `Eu`
         * application domain: generic
         * name: EulerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^"'" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).
         */
    }
    attribute eulerNumber: EulerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.3 Froude number */
    attribute def FroudeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.3 Froude number
         * symbol(s): `Fr`
         * application domain: generic
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)
         * remarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.
         */
    }
    attribute froudeNumber: FroudeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.4 Grashof number */
    attribute def GrashofNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.4 Grashof number
         * symbol(s): `Gr`
         * application domain: generic
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).
         */
    }
    attribute grashofNumber: GrashofNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.5 Weber number */
    attribute def WeberNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.5 Weber number
         * symbol(s): `We`
         * application domain: generic
         * name: WeberNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)
         * remarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.
         */
    }
    attribute weberNumber: WeberNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.6 Mach number */
    attribute def MachNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.6 Mach number
         * symbol(s): `Ma`
         * application domain: generic
         * name: MachNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid
         * remarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).
         */
    }
    attribute machNumber: MachNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.7 Knudsen number */
    attribute def KnudsenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.7 Knudsen number
         * symbol(s): `Kn`
         * application domain: generic
         * name: KnudsenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.
         */
    }
    attribute knudsenNumber: KnudsenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.8 Strouhal number, Thomson number */
    attribute def StrouhalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.8 Strouhal number, Thomson number
         * symbol(s): `Sr`, `Sh`
         * application domain: generic
         * name: StrouhalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow
         * remarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.
         */
    }
    attribute strouhalNumber: StrouhalNumberValue :> scalarQuantities;

    alias thomsonNumber for strouhalNumber;

    /* ISO-80000-11 item 11-4.9 drag coefficient */
    /* Refer to declaration for DragCoefficient in ISQMechanics item 4-23.4 drag coefficient */

    /* ISO-80000-11 item 11-4.10 Bagnold number */
    attribute def BagnoldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.10 Bagnold number
         * symbol(s): `Bg`
         * application domain: generic
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body
         * remarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.
         */
    }
    attribute bagnoldNumber: BagnoldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.11 Bagnold number */
    attribute def BagnoldNumberForSolidParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.11 Bagnold number
         * symbol(s): `Ba_2`
         * application domain: solid particles
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles
         * remarks: None.
         */
    }
    attribute bagnoldNumberForSolidParticles: BagnoldNumberForSolidParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.12 lift coefficient */
    attribute def LiftCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.12 lift coefficient
         * symbol(s): `c_l`, `c_A`
         * application domain: generic
         * name: LiftCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure
         * remarks: The lift coefficient is dependant on the shape of the wing.
         */
    }
    attribute liftCoefficient: LiftCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.13 thrust coefficient */
    attribute def ThrustCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.13 thrust coefficient
         * symbol(s): `c_t`
         * application domain: generic
         * name: ThrustCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller
         * remarks: The thrust coefficient is dependant on the shape of the propeller.
         */
    }
    attribute thrustCoefficient: ThrustCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.14 Dean number */
    attribute def DeanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.14 Dean number
         * symbol(s): `Dn`
         * application domain: generic
         * name: DeanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe
         * remarks: None.
         */
    }
    attribute deanNumber: DeanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.15 Bejan number */
    attribute def BejanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.15 Bejan number
         * symbol(s): `Be`
         * application domain: generic
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)
         * remarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.
         */
    }
    attribute bejanNumber: BejanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.16 Lagrange number */
    attribute def LagrangeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.16 Lagrange number
         * symbol(s): `Lg`
         * application domain: generic
         * name: LagrangeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).
         */
    }
    attribute lagrangeNumber: LagrangeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.17 Bingham number, plasticity number */
    attribute def BinghamNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.17 Bingham number, plasticity number
         * symbol(s): `Bm`, `Bn`
         * application domain: generic
         * name: BinghamNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute binghamNumber: BinghamNumberValue :> scalarQuantities;

    alias plasticityNumber for binghamNumber;

    /* ISO-80000-11 item 11-4.18 Hedström number */
    attribute def 'HedströmNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.18 Hedström number
         * symbol(s): `He`, `Hd`
         * application domain: generic
         * name: HedströmNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'hedströmNumber': 'HedströmNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-4.19 Bodenstein number */
    attribute def BodensteinNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.19 Bodenstein number
         * symbol(s): `Bd`
         * application domain: generic
         * name: BodensteinNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Bodenstein number is also given by `Bd = Pe^"*" = Re*Sc`, where `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).
         */
    }
    attribute bodensteinNumber: BodensteinNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.20 Rossby number, Kiebel number */
    attribute def RossbyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.20 Rossby number, Kiebel number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RossbyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude
         * remarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.
         */
    }
    attribute rossbyNumber: RossbyNumberValue :> scalarQuantities;

    alias kiebelNumber for rossbyNumber;

    /* ISO-80000-11 item 11-4.21 Ekman number */
    attribute def EkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.21 Ekman number
         * symbol(s): `Ek`
         * application domain: generic
         * name: EkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude
         * remarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).
         */
    }
    attribute ekmanNumber: EkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.22 elasticity number */
    attribute def ElasticityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.22 elasticity number
         * symbol(s): `El`
         * application domain: generic
         * name: ElasticityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe
         * remarks: See also Deborah number (item 11-7.8).
         */
    }
    attribute elasticityNumber: ElasticityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.23 Darcy friction factor, Moody friction factor */
    attribute def DarcyFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.23 Darcy friction factor, Moody friction factor
         * symbol(s): `f_D`
         * application domain: generic
         * name: DarcyFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute darcyFrictionFactor: DarcyFrictionFactorValue :> scalarQuantities;

    alias moodyFrictionFactor for darcyFrictionFactor;

    /* ISO-80000-11 item 11-4.24 Fanning number */
    attribute def FanningNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.24 Fanning number
         * symbol(s): `f_n`, `f`
         * application domain: generic
         * name: FanningNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe
         * remarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.
         */
    }
    attribute fanningNumber: FanningNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.25 Goertler number, Goertler parameter */
    attribute def GoertlerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.25 Goertler number, Goertler parameter
         * symbol(s): `Go`
         * application domain: generic
         * name: GoertlerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)
         * remarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.
         */
    }
    attribute goertlerNumber: GoertlerNumberValue :> scalarQuantities;

    alias goertlerParameter for goertlerNumber;

    /* ISO-80000-11 item 11-4.26 Hagen number */
    attribute def HagenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.26 Hagen number
         * symbol(s): `Hg`, `Ha`
         * application domain: generic
         * name: HagenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).
         */
    }
    attribute hagenNumber: HagenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.27 Laval number */
    attribute def LavalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.27 Laval number
         * symbol(s): `La`
         * application domain: generic
         * name: LavalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3),  `R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)
         * remarks: The Laval number is a specific kind of Mach number (item 11-4.6).
         */
    }
    attribute lavalNumber: LavalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.28 Poiseuille number */
    attribute def PoiseuilleNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.28 Poiseuille number
         * symbol(s): `Poi`
         * application domain: generic
         * name: PoiseuilleNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid
         * remarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).
         */
    }
    attribute poiseuilleNumber: PoiseuilleNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.29 power number */
    attribute def PowerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.29 power number
         * symbol(s): `Pn`
         * application domain: generic
         * name: PowerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer
         * remarks: None.
         */
    }
    attribute powerNumber: PowerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.30 Richardson number */
    attribute def RichardsonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.30 Richardson number
         * symbol(s): `Ri`
         * application domain: generic
         * name: RichardsonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)
         * remarks: In geophysics differences of these quantities are of interest.
         */
    }
    attribute richardsonNumber: RichardsonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.31 Reech number */
    attribute def ReechNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.31 Reech number
         * symbol(s): `Ree`
         * application domain: generic
         * name: ReechNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water
         * remarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .
         */
    }
    attribute reechNumber: ReechNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.32 Stokes number */
    attribute def StokesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.32 Stokes number
         * symbol(s): `Stk`
         * application domain: time-related
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence
         * remarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.
         */
    }
    attribute stokesNumber: StokesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.33 Stokes number */
    attribute def StokesNumberForVibratingParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.33 Stokes number
         * symbol(s): `Stk_1`
         * application domain: vibrating particles
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute stokesNumberForVibratingParticles: StokesNumberForVibratingParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.34 Stokes number, power coefficient */
    attribute def StokesNumberForRotameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.34 Stokes number, power coefficient
         * symbol(s): `Stk_2`
         * application domain: rotameter
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).
         */
    }
    attribute stokesNumberForRotameter: StokesNumberForRotameterValue :> scalarQuantities;

    alias powerCoefficient for stokesNumber;

    /* ISO-80000-11 item 11-4.35 Stokes number */
    attribute def StokesNumberForGravityValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.35 Stokes number
         * symbol(s): `Stk_3`
         * application domain: gravity
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall
         * remarks: None.
         */
    }
    attribute stokesNumberForGravity: StokesNumberForGravityValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.36 Stokes number */
    attribute def StokesNumberForDragValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.36 Stokes number
         * symbol(s): `Stk_4`
         * application domain: drag
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute stokesNumberForDrag: StokesNumberForDragValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.37 Laplace number, Suratman number */
    attribute def LaplaceNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.37 Laplace number, Suratman number
         * symbol(s): `La`, `Su`
         * application domain: generic
         * name: LaplaceNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid
         * remarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).
         */
    }
    attribute laplaceNumber: LaplaceNumberValue :> scalarQuantities;

    alias suratmanNumber for laplaceNumber;

    /* ISO-80000-11 item 11-4.38 Blake number */
    attribute def BlakeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.38 Blake number
         * symbol(s): `Bl`
         * application domain: generic
         * name: BlakeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)
         * remarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.
         */
    }
    attribute blakeNumber: BlakeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.39 Sommerfeld number */
    attribute def SommerfeldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.39 Sommerfeld number
         * symbol(s): `So`, `Sm`
         * application domain: generic
         * name: SommerfeldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute sommerfeldNumber: SommerfeldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.40 Taylor number */
    attribute def TaylorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.40 Taylor number
         * symbol(s): `Ta`
         * application domain: momentum transfer
         * name: TaylorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.
         */
    }
    attribute taylorNumber: TaylorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.41 Galilei number */
    attribute def GalileiNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.41 Galilei number
         * symbol(s): `Ga`
         * application domain: generic
         * name: GalileiNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).
         */
    }
    attribute galileiNumber: GalileiNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.42 Womersley number */
    attribute def WomersleyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.42 Womersley number
         * symbol(s): `Wo`, `α`
         * application domain: generic
         * name: WomersleyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Womersley number is used for pulsating flows e.g. in blood flow.
         */
    }
    attribute womersleyNumber: WomersleyNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.1 Fourier number */
    attribute def FourierNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.1 Fourier number
         * symbol(s): `Fo`
         * application domain: heat transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.
         */
    }
    attribute fourierNumber: FourierNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.2 Péclet number */
    attribute def 'PécletNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.2 Péclet number
         * symbol(s): `Pe`
         * application domain: heat transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.
         */
    }
    attribute 'pécletNumber': 'PécletNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-5.3 Rayleigh number */
    attribute def RayleighNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.3 Rayleigh number
         * symbol(s): `Ra`
         * application domain: generic
         * name: RayleighNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid
         * remarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).
         */
    }
    attribute rayleighNumber: RayleighNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.4 Froude number */
    attribute def FroudeNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.4 Froude number
         * symbol(s): `Fr^"*"`
         * application domain: heat transfer
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^"*" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)"
         * remarks: None.
         */
    }
    attribute froudeNumberForHeatTransfer: FroudeNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.5 Nusselt number */
    attribute def NusseltNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.5 Nusselt number
         * symbol(s): `Nu`
         * application domain: heat transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)
         * remarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the "Biot number for heat transfer" (item 11-5.6) is used.
         */
    }
    attribute nusseltNumber: NusseltNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.6 Biot number */
    attribute def BiotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.6 Biot number
         * symbol(s): `Bi`
         * application domain: heat transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body
         * remarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.
         */
    }
    attribute biotNumber: BiotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.7 Stanton number */
    attribute def StantonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.7 Stanton number
         * symbol(s): `St`
         * application domain: heat transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid
         * remarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.
         */
    }
    attribute stantonNumber: StantonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.8 j-factor, heat transfer factor, Colburn number */
    attribute def JFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.8 j-factor, heat transfer factor, Colburn number
         * symbol(s): `j`, `Co`, `Jq`
         * application domain: heat transfer
         * name: JFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).
         */
    }
    attribute jFactor: JFactorValue :> scalarQuantities;

    alias heatTransferFactor for jFactor;

    alias colburnNumber for jFactor;

    /* ISO-80000-11 item 11-5.9 Bejan number */
    attribute def BejanNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.9 Bejan number
         * symbol(s): `Be_1`
         * application domain: heat transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute bejanNumberForHeatTransfer: BejanNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.10 Bejan number */
    attribute def BejanNumberForEntropyValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.10 Bejan number
         * symbol(s): `Be_S`
         * application domain: entropy
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction
         * remarks: None.
         */
    }
    attribute bejanNumberForEntropy: BejanNumberForEntropyValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.11 Stefan number */
    attribute def StefanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.11 Stefan number
         * symbol(s): `Ste`, `Stf`
         * application domain: phase transition
         * name: StefanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute stefanNumber: StefanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.12 Brinkman number */
    attribute def BrinkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.12 Brinkman number
         * symbol(s): `Br`, `N_(Br)`
         * application domain: generic
         * name: BrinkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature
         * remarks: None.
         */
    }
    attribute brinkmanNumber: BrinkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.13 Clausius number */
    attribute def ClausiusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.13 Clausius number
         * symbol(s): `Cl`
         * application domain: generic
         * name: ClausiusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`
         * remarks: None.
         */
    }
    attribute clausiusNumber: ClausiusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.14 Carnot number */
    attribute def CarnotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.14 Carnot number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CarnotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively
         * remarks: None.
         */
    }
    attribute carnotNumber: CarnotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.15 Eckert number, Dulong number */
    attribute def EckertNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.15 Eckert number, Dulong number
         * symbol(s): `Ec`
         * application domain: generic
         * name: EckertNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)
         * remarks: None.
         */
    }
    attribute eckertNumber: EckertNumberValue :> scalarQuantities;

    alias dulongNumber for eckertNumber;

    /* ISO-80000-11 item 11-5.16 Graetz number */
    attribute def GraetzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.16 Graetz number
         * symbol(s): `Gz`
         * application domain: heat transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute graetzNumber: GraetzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.17 heat transfer number */
    attribute def HeatTransferNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.17 heat transfer number
         * symbol(s): `K_Q`
         * application domain: generic
         * name: HeatTransferNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute heatTransferNumber: HeatTransferNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.18 Pomerantsev number */
    attribute def PomerantsevNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.18 Pomerantsev number
         * symbol(s): `Po`, `Pov`
         * application domain: heat transfer
         * name: PomerantsevNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)
         * remarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.
         */
    }
    attribute pomerantsevNumber: PomerantsevNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.19 Boltzmann number */
    attribute def BoltzmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.19 Boltzmann number
         * symbol(s): `Bz`, `Bol`, `Bo`
         * application domain: generic
         * name: BoltzmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute boltzmannNumber: BoltzmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.20 Stark number */
    attribute def StarkNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.20 Stark number
         * symbol(s): `Sk`
         * application domain: generic
         * name: StarkNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.
         */
    }
    attribute starkNumber: StarkNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.1 Fourier number */
    attribute def FourierNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.1 Fourier number
         * symbol(s): `Fo^"*"`
         * application domain: mass transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^"*" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer"
         * remarks: The Fourier number for mass transfer is also given by `Fo^*" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1)."
         */
    }
    attribute fourierNumberForMassTransfer: FourierNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.2 Péclet number */
    attribute def 'PécletNumberForMassTransferValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.2 Péclet number
         * symbol(s): `Pe^"*"`, `Bd`, `Bod`
         * application domain: mass transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The Péclet number for mass transfer is also given by `Pe^"*" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.
         */
    }
    attribute 'pécletNumberForMassTransfer': 'PécletNumberForMassTransferValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-6.3 Grashof number */
    attribute def GrashofNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.3 Grashof number
         * symbol(s): `Gr^"*"`
         * application domain: mass transfer
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^"*" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)"
         * remarks: Instead of "amount-of-substance fraction" the "amount-of-substance concentration" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.
         */
    }
    attribute grashofNumberForMassTransfer: GrashofNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.4 Nusselt number */
    attribute def NusseltNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.4 Nusselt number
         * symbol(s): `Nu^"*"`
         * application domain: mass transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^"*" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.
         */
    }
    attribute nusseltNumberForMassTransfer: NusseltNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.5 Stanton number */
    attribute def StantonNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.5 Stanton number
         * symbol(s): `St^"*"`
         * application domain: mass transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^"*" = k^"*"
         * remarks: The Stanton number for mass transfer is also given by `St^*" = (Nu^"*")/(Pe^"*"*)`, where `Nu^"*"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer."
         */
    }
    attribute stantonNumberForMassTransfer: StantonNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.6 Graetz number */
    attribute def GraetzNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.6 Graetz number
         * symbol(s): `Gz^"*"`
         * application domain: mass transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^"*" = (v*d)/D = d/l*Pe^"*"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2)"
         * remarks: None.
         */
    }
    attribute graetzNumberForMassTransfer: GraetzNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.7 mass transfer factor */
    attribute def MassTransferFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.7 mass transfer factor
         * symbol(s): `j^"*"`
         * application domain: mass transfer
         * name: MassTransferFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The mass transfer factor is also given by `j_m = j^*" = St^"*" * (Sc)^(2/3)` where `St^"*"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17)."
         */
    }
    attribute massTransferFactor: MassTransferFactorValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.8 Atwood number */
    attribute def AtwoodNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.8 Atwood number
         * symbol(s): `At`
         * application domain: generic
         * name: AtwoodNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid
         * remarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.
         */
    }
    attribute atwoodNumber: AtwoodNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.9 Biot number */
    attribute def BiotNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.9 Biot number
         * symbol(s): `Bi^"*"`
         * application domain: mass transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*" = (k*l)/D_"int"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_"int"` is diffusion coefficient (ISO 80000-9) at the interface"
         * remarks: None.
         */
    }
    attribute biotNumberForMassTransfer: BiotNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.10 Morton number */
    attribute def MortonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.10 Morton number
         * symbol(s): `Mo`
         * application domain: generic
         * name: MortonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop
         * remarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute mortonNumber: MortonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.11 Bond number, Eötvös number */
    attribute def BondNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.11 Bond number, Eötvös number
         * symbol(s): `Bo`, `Eo`
         * application domain: generic
         * name: BondNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble
         * remarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.
         */
    }
    attribute bondNumber: BondNumberValue :> scalarQuantities;

    alias 'eötvösNumber' for bondNumber;

    /* ISO-80000-11 item 11-6.12 Archimedes number */
    attribute def ArchimedesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.12 Archimedes number
         * symbol(s): `Ar`
         * application domain: generic
         * name: ArchimedesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid
         * remarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).
         */
    }
    attribute archimedesNumber: ArchimedesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.13 expansion number */
    attribute def ExpansionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.13 expansion number
         * symbol(s): `Ex`
         * application domain: generic
         * name: ExpansionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid
         * remarks: None.
         */
    }
    attribute expansionNumber: ExpansionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.14 Marangoni number */
    attribute def MarangoniNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.14 Marangoni number
         * symbol(s): `Mg`, `Mar`
         * application domain: generic
         * name: MarangoniNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film
         * remarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.
         */
    }
    attribute marangoniNumber: MarangoniNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.15 Lockhart-Martinelli parameter */
    attribute def LockhartMartinelliParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.15 Lockhart-Martinelli parameter
         * symbol(s): `Lp`
         * application domain: generic
         * name: LockhartMartinelliParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density
         * remarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.
         */
    }
    attribute lockhartMartinelliParameter: LockhartMartinelliParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.16 Bejan number */
    attribute def BejanNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.16 Bejan number
         * symbol(s): `Be^"*"`, `Be_2`
         * application domain: mass transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity"
         * remarks: A similar quantity exists for heat transfer (item 11-5.9).
         */
    }
    attribute bejanNumberForMassTransfer: BejanNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.17 cavitation number */
    attribute def CavitationNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.17 cavitation number
         * symbol(s): `Ca`, `Cn`
         * application domain: generic
         * name: CavitationNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow
         * remarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.
         */
    }
    attribute cavitationNumber: CavitationNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.18 absorption number */
    attribute def AbsorptionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.18 absorption number
         * symbol(s): `Ab`
         * application domain: generic
         * name: AbsorptionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter
         * remarks: None.
         */
    }
    attribute absorptionNumber: AbsorptionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.19 capillary number */
    attribute def CapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.19 capillary number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid
         * remarks: None.
         */
    }
    attribute capillaryNumber: CapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.20 dynamic capillary number */
    attribute def DynamicCapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.20 dynamic capillary number
         * symbol(s): `Ca^"*"`, `Cn`
         * application domain: generic
         * name: DynamicCapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)"
         * remarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.
         */
    }
    attribute dynamicCapillaryNumber: DynamicCapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.1 Prandtl number */
    attribute def PrandtlNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.1 Prandtl number
         * symbol(s): `Pr`
         * application domain: generic
         * name: PrandtlNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute prandtlNumber: PrandtlNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.2 Schmidt number */
    attribute def SchmidtNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.2 Schmidt number
         * symbol(s): `Sc`
         * application domain: generic
         * name: SchmidtNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.
         */
    }
    attribute schmidtNumber: SchmidtNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.3 Lewis number */
    attribute def LewisNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.3 Lewis number
         * symbol(s): `Le`
         * application domain: generic
         * name: LewisNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. 
         */
    }
    attribute lewisNumber: LewisNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.4 Ohnesorge number */
    attribute def OhnesorgeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.4 Ohnesorge number
         * symbol(s): `Oh`
         * application domain: generic
         * name: OhnesorgeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.
         */
    }
    attribute ohnesorgeNumber: OhnesorgeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.5 Cauchy number, aeroelasticity parameter */
    attribute def CauchyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.5 Cauchy number, aeroelasticity parameter
         * symbol(s): `Cy`
         * application domain: generic
         * name: CauchyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute cauchyNumber: CauchyNumberValue :> scalarQuantities;

    alias aeroelasticityParameter for cauchyNumber;

    /* ISO-80000-11 item 11-7.6 Hooke number */
    attribute def HookeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.6 Hooke number
         * symbol(s): `Ho_2`
         * application domain: generic
         * name: HookeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute hookeNumber: HookeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.7 Weissenberg number */
    attribute def WeissenbergNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.7 Weissenberg number
         * symbol(s): `Wi`
         * application domain: generic
         * name: WeissenbergNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)
         * remarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.
         */
    }
    attribute weissenbergNumber: WeissenbergNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.8 Deborah number */
    attribute def DeborahNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.8 Deborah number
         * symbol(s): `De`
         * application domain: generic
         * name: DeborahNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)
         * remarks: The stress relaxation time is sometimes called the Maxwell relaxation time.
         */
    }
    attribute deborahNumber: DeborahNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.9 Lorentz number */
    attribute def LorentzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.9 Lorentz number
         * symbol(s): `Lo`
         * application domain: generic
         * name: LorentzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points
         * remarks: None.
         */
    }
    attribute lorentzNumber: LorentzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.10 compressibility number */
    attribute def CompressibilityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.10 compressibility number
         * symbol(s): `Z`
         * application domain: generic
         * name: CompressibilityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute compressibilityNumber: CompressibilityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.1 Reynolds magnetic number */
    attribute def ReynoldsMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.1 Reynolds magnetic number
         * symbol(s): `Rm`
         * application domain: generic
         * name: ReynoldsMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)
         * remarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).
         */
    }
    attribute reynoldsMagneticNumber: ReynoldsMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.2 Batchelor number */
    attribute def BatchelorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.2 Batchelor number
         * symbol(s): `Bt`
         * application domain: generic
         * name: BatchelorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute batchelorNumber: BatchelorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.3 Nusselt electric number */
    attribute def NusseltElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.3 Nusselt electric number
         * symbol(s): `Ne`
         * application domain: generic
         * name: NusseltElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^"*" = D^"+" + D^"-"`, where `D^"+"`, `D^"-"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively"
         * remarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.
         */
    }
    attribute nusseltElectricNumber: NusseltElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.4 Alfvén number, Mach magnetic number, Kárman number */
    attribute def 'AlfvénNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number
         * symbol(s): `Al`
         * application domain: generic
         * name: AlfvénNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: Often, the inverse of this number is wrongly used. The name "Alfvén Mach number" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute 'alfvénNumber': 'AlfvénNumberValue' :> scalarQuantities;

    alias machMagneticNumber for 'alfvénNumber';

    alias 'kármanNumber' for 'alfvénNumber';

    /* ISO-80000-11 item 11-8.5 Hartmann number */
    attribute def HartmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.5 Hartmann number
         * symbol(s): `Ha`
         * application domain: generic
         * name: HartmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: The Hartmann number represents also the ratio of magnetic force to viscous force.
         */
    }
    attribute hartmannNumber: HartmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.6 Cowling number, Euler magnetic number */
    attribute def CowlingNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.6 Cowling number, Euler magnetic number
         * symbol(s): `Co`
         * application domain: magnetism
         * name: CowlingNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).
         */
    }
    attribute cowlingNumber: CowlingNumberValue :> scalarQuantities;

    alias eulerMagneticNumber for cowlingNumber;

    /* ISO-80000-11 item 11-8.7 Stuart electrical number */
    attribute def StuartElectricalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.7 Stuart electrical number
         * symbol(s): `Se`
         * application domain: generic
         * name: StuartElectricalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).
         */
    }
    attribute stuartElectricalNumber: StuartElectricalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.8 magnetic pressure number */
    attribute def MagneticPressureNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.8 magnetic pressure number
         * symbol(s): `N_(mp)`
         * application domain: generic
         * name: MagneticPressureNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute magneticPressureNumber: MagneticPressureNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.9 Chandrasekhar number */
    attribute def ChandrasekharNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.9 Chandrasekhar number
         * symbol(s): `Q`, `Ch`
         * application domain: generic
         * name: ChandrasekharNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).
         */
    }
    attribute chandrasekharNumber: ChandrasekharNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.10 Prandtl magnetic number */
    attribute def PrandtlMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.10 Prandtl magnetic number
         * symbol(s): `Pr_m`
         * application domain: generic
         * name: PrandtlMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.
         */
    }
    attribute prandtlMagneticNumber: PrandtlMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.11 Roberts number */
    attribute def RobertsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.11 Roberts number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RobertsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).
         */
    }
    attribute robertsNumber: RobertsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.12 Stuart number */
    attribute def StuartNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.12 Stuart number
         * symbol(s): `Stw`
         * application domain: generic
         * name: StuartNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. 
         */
    }
    attribute stuartNumber: StuartNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.13 magnetic number */
    attribute def MagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.13 magnetic number
         * symbol(s): `N_(mg)`
         * application domain: generic
         * name: MagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute magneticNumber: MagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.14 electric field parameter */
    attribute def ElectricFieldParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.14 electric field parameter
         * symbol(s): `Ef`
         * application domain: generic
         * name: ElectricFieldParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute electricFieldParameter: ElectricFieldParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.15 Hall number */
    attribute def HallNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.15 Hall number
         * symbol(s): `Hc`, `CH`
         * application domain: generic
         * name: HallNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)
         * remarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.
         */
    }
    attribute hallNumber: HallNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.16 Lundquist number */
    attribute def LundquistNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.16 Lundquist number
         * symbol(s): `Lu`
         * application domain: generic
         * name: LundquistNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).
         */
    }
    attribute lundquistNumber: LundquistNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.17 Joule magnetic number */
    attribute def JouleMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.17 Joule magnetic number
         * symbol(s): `Jo_m`
         * application domain: generic
         * name: JouleMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: This number is also called magnetic Joule number.
         */
    }
    attribute jouleMagneticNumber: JouleMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.18 Grashof magnetic number */
    attribute def GrashofMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.18 Grashof magnetic number
         * symbol(s): `Gr_m`
         * application domain: generic
         * name: GrashofMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).
         */
    }
    attribute grashofMagneticNumber: GrashofMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.19 Naze number */
    attribute def NazeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.19 Naze number
         * symbol(s): `Na`
         * application domain: generic
         * name: NazeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.
         */
    }
    attribute nazeNumber: NazeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.20 Reynolds electric number */
    attribute def ReynoldsElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.20 Reynolds electric number
         * symbol(s): `Re_e`
         * application domain: generic
         * name: ReynoldsElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers
         * remarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.
         */
    }
    attribute reynoldsElectricNumber: ReynoldsElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.21 Ampère number */
    attribute def 'AmpèreNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.21 Ampère number
         * symbol(s): `Am`
         * application domain: generic
         * name: AmpèreNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)
         * remarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).
         */
    }
    attribute 'ampèreNumber': 'AmpèreNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-9.1 Arrhenius number */
    attribute def ArrheniusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.1 Arrhenius number
         * symbol(s): `α`
         * application domain: generic
         * name: ArrheniusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute arrheniusNumber: ArrheniusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-9.2 Landau-Ginzburg number */
    attribute def LandauGinzburgNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.2 Landau-Ginzburg number
         * symbol(s): `κ`
         * application domain: generic
         * name: LandauGinzburgNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)
         * remarks: None.
         */
    }
    attribute landauGinzburgNumber: LandauGinzburgNumberValue :> scalarQuantities;

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_characteristic_numbers.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQCharacteristicNumbers {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 "Characteristic numbers"
     * see also https://www.iso.org/standard/64982.html
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
    /* ISO-80000-11 item 11-4.1 Reynolds number */
    attribute def ReynoldsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.1 Reynolds number
         * symbol(s): `Re`
         * application domain: generic
         * name: ReynoldsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.
         */
    }
    attribute reynoldsNumber : ReynoldsNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.2 Euler number */
    attribute def EulerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.2 Euler number
         * symbol(s): `Eu`
         * application domain: generic
         * name: EulerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^"'" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).
         */
    }
    attribute eulerNumber : EulerNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.3 Froude number */
    attribute def FroudeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.3 Froude number
         * symbol(s): `Fr`
         * application domain: generic
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)
         * remarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.
         */
    }
    attribute froudeNumber : FroudeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.4 Grashof number */
    attribute def GrashofNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.4 Grashof number
         * symbol(s): `Gr`
         * application domain: generic
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).
         */
    }
    attribute grashofNumber : GrashofNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.5 Weber number */
    attribute def WeberNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.5 Weber number
         * symbol(s): `We`
         * application domain: generic
         * name: WeberNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)
         * remarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.
         */
    }
    attribute weberNumber : WeberNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.6 Mach number */
    attribute def MachNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.6 Mach number
         * symbol(s): `Ma`
         * application domain: generic
         * name: MachNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid
         * remarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).
         */
    }
    attribute machNumber : MachNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.7 Knudsen number */
    attribute def KnudsenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.7 Knudsen number
         * symbol(s): `Kn`
         * application domain: generic
         * name: KnudsenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.
         */
    }
    attribute knudsenNumber : KnudsenNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.8 Strouhal number, Thomson number */
    attribute def StrouhalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.8 Strouhal number, Thomson number
         * symbol(s): `Sr`, `Sh`
         * application domain: generic
         * name: StrouhalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow
         * remarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.
         */
    }
    attribute strouhalNumber : StrouhalNumberValue :> scalarQuantities;
    alias thomsonNumber for strouhalNumber;
    /* ISO-80000-11 item 11-4.9 drag coefficient */
    /* Refer to declaration for DragCoefficient in ISQMechanics item 4-23.4 drag coefficient */
    /* ISO-80000-11 item 11-4.10 Bagnold number */
    attribute def BagnoldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.10 Bagnold number
         * symbol(s): `Bg`
         * application domain: generic
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body
         * remarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.
         */
    }
    attribute bagnoldNumber : BagnoldNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.11 Bagnold number */
    attribute def BagnoldNumberForSolidParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.11 Bagnold number
         * symbol(s): `Ba_2`
         * application domain: solid particles
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles
         * remarks: None.
         */
    }
    attribute bagnoldNumberForSolidParticles : BagnoldNumberForSolidParticlesValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.12 lift coefficient */
    attribute def LiftCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.12 lift coefficient
         * symbol(s): `c_l`, `c_A`
         * application domain: generic
         * name: LiftCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure
         * remarks: The lift coefficient is dependant on the shape of the wing.
         */
    }
    attribute liftCoefficient : LiftCoefficientValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.13 thrust coefficient */
    attribute def ThrustCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.13 thrust coefficient
         * symbol(s): `c_t`
         * application domain: generic
         * name: ThrustCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller
         * remarks: The thrust coefficient is dependant on the shape of the propeller.
         */
    }
    attribute thrustCoefficient : ThrustCoefficientValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.14 Dean number */
    attribute def DeanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.14 Dean number
         * symbol(s): `Dn`
         * application domain: generic
         * name: DeanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe
         * remarks: None.
         */
    }
    attribute deanNumber : DeanNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.15 Bejan number */
    attribute def BejanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.15 Bejan number
         * symbol(s): `Be`
         * application domain: generic
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)
         * remarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.
         */
    }
    attribute bejanNumber : BejanNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.16 Lagrange number */
    attribute def LagrangeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.16 Lagrange number
         * symbol(s): `Lg`
         * application domain: generic
         * name: LagrangeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).
         */
    }
    attribute lagrangeNumber : LagrangeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.17 Bingham number, plasticity number */
    attribute def BinghamNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.17 Bingham number, plasticity number
         * symbol(s): `Bm`, `Bn`
         * application domain: generic
         * name: BinghamNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute binghamNumber : BinghamNumberValue :> scalarQuantities;
    alias plasticityNumber for binghamNumber;
    /* ISO-80000-11 item 11-4.18 Hedström number */
    attribute def 'HedströmNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.18 Hedström number
         * symbol(s): `He`, `Hd`
         * application domain: generic
         * name: HedströmNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'hedströmNumber' : 'HedströmNumberValue' :> scalarQuantities;
    /* ISO-80000-11 item 11-4.19 Bodenstein number */
    attribute def BodensteinNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.19 Bodenstein number
         * symbol(s): `Bd`
         * application domain: generic
         * name: BodensteinNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Bodenstein number is also given by `Bd = Pe^"*" = Re*Sc`, where `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).
         */
    }
    attribute bodensteinNumber : BodensteinNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.20 Rossby number, Kiebel number */
    attribute def RossbyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.20 Rossby number, Kiebel number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RossbyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude
         * remarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.
         */
    }
    attribute rossbyNumber : RossbyNumberValue :> scalarQuantities;
    alias kiebelNumber for rossbyNumber;
    /* ISO-80000-11 item 11-4.21 Ekman number */
    attribute def EkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.21 Ekman number
         * symbol(s): `Ek`
         * application domain: generic
         * name: EkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude
         * remarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).
         */
    }
    attribute ekmanNumber : EkmanNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.22 elasticity number */
    attribute def ElasticityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.22 elasticity number
         * symbol(s): `El`
         * application domain: generic
         * name: ElasticityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe
         * remarks: See also Deborah number (item 11-7.8).
         */
    }
    attribute elasticityNumber : ElasticityNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.23 Darcy friction factor, Moody friction factor */
    attribute def DarcyFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.23 Darcy friction factor, Moody friction factor
         * symbol(s): `f_D`
         * application domain: generic
         * name: DarcyFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute darcyFrictionFactor : DarcyFrictionFactorValue :> scalarQuantities;
    alias moodyFrictionFactor for darcyFrictionFactor;
    /* ISO-80000-11 item 11-4.24 Fanning number */
    attribute def FanningNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.24 Fanning number
         * symbol(s): `f_n`, `f`
         * application domain: generic
         * name: FanningNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe
         * remarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.
         */
    }
    attribute fanningNumber : FanningNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.25 Goertler number, Goertler parameter */
    attribute def GoertlerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.25 Goertler number, Goertler parameter
         * symbol(s): `Go`
         * application domain: generic
         * name: GoertlerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)
         * remarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.
         */
    }
    attribute goertlerNumber : GoertlerNumberValue :> scalarQuantities;
    alias goertlerParameter for goertlerNumber;
    /* ISO-80000-11 item 11-4.26 Hagen number */
    attribute def HagenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.26 Hagen number
         * symbol(s): `Hg`, `Ha`
         * application domain: generic
         * name: HagenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).
         */
    }
    attribute hagenNumber : HagenNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.27 Laval number */
    attribute def LavalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.27 Laval number
         * symbol(s): `La`
         * application domain: generic
         * name: LavalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3),  `R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)
         * remarks: The Laval number is a specific kind of Mach number (item 11-4.6).
         */
    }
    attribute lavalNumber : LavalNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.28 Poiseuille number */
    attribute def PoiseuilleNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.28 Poiseuille number
         * symbol(s): `Poi`
         * application domain: generic
         * name: PoiseuilleNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid
         * remarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).
         */
    }
    attribute poiseuilleNumber : PoiseuilleNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.29 power number */
    attribute def PowerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.29 power number
         * symbol(s): `Pn`
         * application domain: generic
         * name: PowerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer
         * remarks: None.
         */
    }
    attribute powerNumber : PowerNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.30 Richardson number */
    attribute def RichardsonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.30 Richardson number
         * symbol(s): `Ri`
         * application domain: generic
         * name: RichardsonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)
         * remarks: In geophysics differences of these quantities are of interest.
         */
    }
    attribute richardsonNumber : RichardsonNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.31 Reech number */
    attribute def ReechNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.31 Reech number
         * symbol(s): `Ree`
         * application domain: generic
         * name: ReechNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water
         * remarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .
         */
    }
    attribute reechNumber : ReechNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.32 Stokes number */
    attribute def StokesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.32 Stokes number
         * symbol(s): `Stk`
         * application domain: time-related
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence
         * remarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.
         */
    }
    attribute stokesNumber : StokesNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.33 Stokes number */
    attribute def StokesNumberForVibratingParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.33 Stokes number
         * symbol(s): `Stk_1`
         * application domain: vibrating particles
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute stokesNumberForVibratingParticles : StokesNumberForVibratingParticlesValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.34 Stokes number, power coefficient */
    attribute def StokesNumberForRotameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.34 Stokes number, power coefficient
         * symbol(s): `Stk_2`
         * application domain: rotameter
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).
         */
    }
    attribute stokesNumberForRotameter : StokesNumberForRotameterValue :> scalarQuantities;
    alias powerCoefficient for stokesNumber;
    /* ISO-80000-11 item 11-4.35 Stokes number */
    attribute def StokesNumberForGravityValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.35 Stokes number
         * symbol(s): `Stk_3`
         * application domain: gravity
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall
         * remarks: None.
         */
    }
    attribute stokesNumberForGravity : StokesNumberForGravityValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.36 Stokes number */
    attribute def StokesNumberForDragValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.36 Stokes number
         * symbol(s): `Stk_4`
         * application domain: drag
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute stokesNumberForDrag : StokesNumberForDragValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.37 Laplace number, Suratman number */
    attribute def LaplaceNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.37 Laplace number, Suratman number
         * symbol(s): `La`, `Su`
         * application domain: generic
         * name: LaplaceNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid
         * remarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).
         */
    }
    attribute laplaceNumber : LaplaceNumberValue :> scalarQuantities;
    alias suratmanNumber for laplaceNumber;
    /* ISO-80000-11 item 11-4.38 Blake number */
    attribute def BlakeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.38 Blake number
         * symbol(s): `Bl`
         * application domain: generic
         * name: BlakeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)
         * remarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.
         */
    }
    attribute blakeNumber : BlakeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.39 Sommerfeld number */
    attribute def SommerfeldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.39 Sommerfeld number
         * symbol(s): `So`, `Sm`
         * application domain: generic
         * name: SommerfeldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute sommerfeldNumber : SommerfeldNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.40 Taylor number */
    attribute def TaylorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.40 Taylor number
         * symbol(s): `Ta`
         * application domain: momentum transfer
         * name: TaylorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.
         */
    }
    attribute taylorNumber : TaylorNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.41 Galilei number */
    attribute def GalileiNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.41 Galilei number
         * symbol(s): `Ga`
         * application domain: generic
         * name: GalileiNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).
         */
    }
    attribute galileiNumber : GalileiNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-4.42 Womersley number */
    attribute def WomersleyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.42 Womersley number
         * symbol(s): `Wo`, `α`
         * application domain: generic
         * name: WomersleyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Womersley number is used for pulsating flows e.g. in blood flow.
         */
    }
    attribute womersleyNumber : WomersleyNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.1 Fourier number */
    attribute def FourierNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.1 Fourier number
         * symbol(s): `Fo`
         * application domain: heat transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.
         */
    }
    attribute fourierNumber : FourierNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.2 Péclet number */
    attribute def 'PécletNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.2 Péclet number
         * symbol(s): `Pe`
         * application domain: heat transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.
         */
    }
    attribute 'pécletNumber' : 'PécletNumberValue' :> scalarQuantities;
    /* ISO-80000-11 item 11-5.3 Rayleigh number */
    attribute def RayleighNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.3 Rayleigh number
         * symbol(s): `Ra`
         * application domain: generic
         * name: RayleighNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid
         * remarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).
         */
    }
    attribute rayleighNumber : RayleighNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.4 Froude number */
    attribute def FroudeNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.4 Froude number
         * symbol(s): `Fr^"*"`
         * application domain: heat transfer
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^"*" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)"
         * remarks: None.
         */
    }
    attribute froudeNumberForHeatTransfer : FroudeNumberForHeatTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.5 Nusselt number */
    attribute def NusseltNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.5 Nusselt number
         * symbol(s): `Nu`
         * application domain: heat transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)
         * remarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the "Biot number for heat transfer" (item 11-5.6) is used.
         */
    }
    attribute nusseltNumber : NusseltNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.6 Biot number */
    attribute def BiotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.6 Biot number
         * symbol(s): `Bi`
         * application domain: heat transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body
         * remarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.
         */
    }
    attribute biotNumber : BiotNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.7 Stanton number */
    attribute def StantonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.7 Stanton number
         * symbol(s): `St`
         * application domain: heat transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid
         * remarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.
         */
    }
    attribute stantonNumber : StantonNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.8 j-factor, heat transfer factor, Colburn number */
    attribute def JFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.8 j-factor, heat transfer factor, Colburn number
         * symbol(s): `j`, `Co`, `Jq`
         * application domain: heat transfer
         * name: JFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).
         */
    }
    attribute jFactor : JFactorValue :> scalarQuantities;
    alias heatTransferFactor for jFactor;
    alias colburnNumber for jFactor;
    /* ISO-80000-11 item 11-5.9 Bejan number */
    attribute def BejanNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.9 Bejan number
         * symbol(s): `Be_1`
         * application domain: heat transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute bejanNumberForHeatTransfer : BejanNumberForHeatTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.10 Bejan number */
    attribute def BejanNumberForEntropyValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.10 Bejan number
         * symbol(s): `Be_S`
         * application domain: entropy
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction
         * remarks: None.
         */
    }
    attribute bejanNumberForEntropy : BejanNumberForEntropyValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.11 Stefan number */
    attribute def StefanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.11 Stefan number
         * symbol(s): `Ste`, `Stf`
         * application domain: phase transition
         * name: StefanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute stefanNumber : StefanNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.12 Brinkman number */
    attribute def BrinkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.12 Brinkman number
         * symbol(s): `Br`, `N_(Br)`
         * application domain: generic
         * name: BrinkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature
         * remarks: None.
         */
    }
    attribute brinkmanNumber : BrinkmanNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.13 Clausius number */
    attribute def ClausiusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.13 Clausius number
         * symbol(s): `Cl`
         * application domain: generic
         * name: ClausiusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`
         * remarks: None.
         */
    }
    attribute clausiusNumber : ClausiusNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.14 Carnot number */
    attribute def CarnotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.14 Carnot number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CarnotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively
         * remarks: None.
         */
    }
    attribute carnotNumber : CarnotNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.15 Eckert number, Dulong number */
    attribute def EckertNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.15 Eckert number, Dulong number
         * symbol(s): `Ec`
         * application domain: generic
         * name: EckertNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)
         * remarks: None.
         */
    }
    attribute eckertNumber : EckertNumberValue :> scalarQuantities;
    alias dulongNumber for eckertNumber;
    /* ISO-80000-11 item 11-5.16 Graetz number */
    attribute def GraetzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.16 Graetz number
         * symbol(s): `Gz`
         * application domain: heat transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute graetzNumber : GraetzNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.17 heat transfer number */
    attribute def HeatTransferNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.17 heat transfer number
         * symbol(s): `K_Q`
         * application domain: generic
         * name: HeatTransferNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute heatTransferNumber : HeatTransferNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.18 Pomerantsev number */
    attribute def PomerantsevNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.18 Pomerantsev number
         * symbol(s): `Po`, `Pov`
         * application domain: heat transfer
         * name: PomerantsevNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)
         * remarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.
         */
    }
    attribute pomerantsevNumber : PomerantsevNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.19 Boltzmann number */
    attribute def BoltzmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.19 Boltzmann number
         * symbol(s): `Bz`, `Bol`, `Bo`
         * application domain: generic
         * name: BoltzmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute boltzmannNumber : BoltzmannNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-5.20 Stark number */
    attribute def StarkNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.20 Stark number
         * symbol(s): `Sk`
         * application domain: generic
         * name: StarkNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.
         */
    }
    attribute starkNumber : StarkNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.1 Fourier number */
    attribute def FourierNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.1 Fourier number
         * symbol(s): `Fo^"*"`
         * application domain: mass transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^"*" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer"
         * remarks: The Fourier number for mass transfer is also given by `Fo^*" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1)."
         */
    }
    attribute fourierNumberForMassTransfer : FourierNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.2 Péclet number */
    attribute def 'PécletNumberForMassTransferValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.2 Péclet number
         * symbol(s): `Pe^"*"`, `Bd`, `Bod`
         * application domain: mass transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The Péclet number for mass transfer is also given by `Pe^"*" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.
         */
    }
    attribute 'pécletNumberForMassTransfer' : 'PécletNumberForMassTransferValue' :> scalarQuantities;
    /* ISO-80000-11 item 11-6.3 Grashof number */
    attribute def GrashofNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.3 Grashof number
         * symbol(s): `Gr^"*"`
         * application domain: mass transfer
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^"*" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)"
         * remarks: Instead of "amount-of-substance fraction" the "amount-of-substance concentration" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.
         */
    }
    attribute grashofNumberForMassTransfer : GrashofNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.4 Nusselt number */
    attribute def NusseltNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.4 Nusselt number
         * symbol(s): `Nu^"*"`
         * application domain: mass transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^"*" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.
         */
    }
    attribute nusseltNumberForMassTransfer : NusseltNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.5 Stanton number */
    attribute def StantonNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.5 Stanton number
         * symbol(s): `St^"*"`
         * application domain: mass transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^"*" = k^"*"
         * remarks: The Stanton number for mass transfer is also given by `St^*" = (Nu^"*")/(Pe^"*"*)`, where `Nu^"*"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer."
         */
    }
    attribute stantonNumberForMassTransfer : StantonNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.6 Graetz number */
    attribute def GraetzNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.6 Graetz number
         * symbol(s): `Gz^"*"`
         * application domain: mass transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^"*" = (v*d)/D = d/l*Pe^"*"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2)"
         * remarks: None.
         */
    }
    attribute graetzNumberForMassTransfer : GraetzNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.7 mass transfer factor */
    attribute def MassTransferFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.7 mass transfer factor
         * symbol(s): `j^"*"`
         * application domain: mass transfer
         * name: MassTransferFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The mass transfer factor is also given by `j_m = j^*" = St^"*" * (Sc)^(2/3)` where `St^"*"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17)."
         */
    }
    attribute massTransferFactor : MassTransferFactorValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.8 Atwood number */
    attribute def AtwoodNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.8 Atwood number
         * symbol(s): `At`
         * application domain: generic
         * name: AtwoodNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid
         * remarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.
         */
    }
    attribute atwoodNumber : AtwoodNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.9 Biot number */
    attribute def BiotNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.9 Biot number
         * symbol(s): `Bi^"*"`
         * application domain: mass transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*" = (k*l)/D_"int"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_"int"` is diffusion coefficient (ISO 80000-9) at the interface"
         * remarks: None.
         */
    }
    attribute biotNumberForMassTransfer : BiotNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.10 Morton number */
    attribute def MortonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.10 Morton number
         * symbol(s): `Mo`
         * application domain: generic
         * name: MortonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop
         * remarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute mortonNumber : MortonNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.11 Bond number, Eötvös number */
    attribute def BondNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.11 Bond number, Eötvös number
         * symbol(s): `Bo`, `Eo`
         * application domain: generic
         * name: BondNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble
         * remarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.
         */
    }
    attribute bondNumber : BondNumberValue :> scalarQuantities;
    alias 'eötvösNumber' for bondNumber;
    /* ISO-80000-11 item 11-6.12 Archimedes number */
    attribute def ArchimedesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.12 Archimedes number
         * symbol(s): `Ar`
         * application domain: generic
         * name: ArchimedesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid
         * remarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).
         */
    }
    attribute archimedesNumber : ArchimedesNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.13 expansion number */
    attribute def ExpansionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.13 expansion number
         * symbol(s): `Ex`
         * application domain: generic
         * name: ExpansionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid
         * remarks: None.
         */
    }
    attribute expansionNumber : ExpansionNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.14 Marangoni number */
    attribute def MarangoniNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.14 Marangoni number
         * symbol(s): `Mg`, `Mar`
         * application domain: generic
         * name: MarangoniNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film
         * remarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.
         */
    }
    attribute marangoniNumber : MarangoniNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.15 Lockhart-Martinelli parameter */
    attribute def LockhartMartinelliParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.15 Lockhart-Martinelli parameter
         * symbol(s): `Lp`
         * application domain: generic
         * name: LockhartMartinelliParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density
         * remarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.
         */
    }
    attribute lockhartMartinelliParameter : LockhartMartinelliParameterValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.16 Bejan number */
    attribute def BejanNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.16 Bejan number
         * symbol(s): `Be^"*"`, `Be_2`
         * application domain: mass transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity"
         * remarks: A similar quantity exists for heat transfer (item 11-5.9).
         */
    }
    attribute bejanNumberForMassTransfer : BejanNumberForMassTransferValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.17 cavitation number */
    attribute def CavitationNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.17 cavitation number
         * symbol(s): `Ca`, `Cn`
         * application domain: generic
         * name: CavitationNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow
         * remarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.
         */
    }
    attribute cavitationNumber : CavitationNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.18 absorption number */
    attribute def AbsorptionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.18 absorption number
         * symbol(s): `Ab`
         * application domain: generic
         * name: AbsorptionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter
         * remarks: None.
         */
    }
    attribute absorptionNumber : AbsorptionNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.19 capillary number */
    attribute def CapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.19 capillary number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid
         * remarks: None.
         */
    }
    attribute capillaryNumber : CapillaryNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-6.20 dynamic capillary number */
    attribute def DynamicCapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.20 dynamic capillary number
         * symbol(s): `Ca^"*"`, `Cn`
         * application domain: generic
         * name: DynamicCapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)"
         * remarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.
         */
    }
    attribute dynamicCapillaryNumber : DynamicCapillaryNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.1 Prandtl number */
    attribute def PrandtlNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.1 Prandtl number
         * symbol(s): `Pr`
         * application domain: generic
         * name: PrandtlNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute prandtlNumber : PrandtlNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.2 Schmidt number */
    attribute def SchmidtNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.2 Schmidt number
         * symbol(s): `Sc`
         * application domain: generic
         * name: SchmidtNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.
         */
    }
    attribute schmidtNumber : SchmidtNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.3 Lewis number */
    attribute def LewisNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.3 Lewis number
         * symbol(s): `Le`
         * application domain: generic
         * name: LewisNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. 
         */
    }
    attribute lewisNumber : LewisNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.4 Ohnesorge number */
    attribute def OhnesorgeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.4 Ohnesorge number
         * symbol(s): `Oh`
         * application domain: generic
         * name: OhnesorgeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.
         */
    }
    attribute ohnesorgeNumber : OhnesorgeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.5 Cauchy number, aeroelasticity parameter */
    attribute def CauchyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.5 Cauchy number, aeroelasticity parameter
         * symbol(s): `Cy`
         * application domain: generic
         * name: CauchyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute cauchyNumber : CauchyNumberValue :> scalarQuantities;
    alias aeroelasticityParameter for cauchyNumber;
    /* ISO-80000-11 item 11-7.6 Hooke number */
    attribute def HookeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.6 Hooke number
         * symbol(s): `Ho_2`
         * application domain: generic
         * name: HookeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute hookeNumber : HookeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.7 Weissenberg number */
    attribute def WeissenbergNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.7 Weissenberg number
         * symbol(s): `Wi`
         * application domain: generic
         * name: WeissenbergNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)
         * remarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.
         */
    }
    attribute weissenbergNumber : WeissenbergNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.8 Deborah number */
    attribute def DeborahNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.8 Deborah number
         * symbol(s): `De`
         * application domain: generic
         * name: DeborahNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)
         * remarks: The stress relaxation time is sometimes called the Maxwell relaxation time.
         */
    }
    attribute deborahNumber : DeborahNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.9 Lorentz number */
    attribute def LorentzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.9 Lorentz number
         * symbol(s): `Lo`
         * application domain: generic
         * name: LorentzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points
         * remarks: None.
         */
    }
    attribute lorentzNumber : LorentzNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-7.10 compressibility number */
    attribute def CompressibilityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.10 compressibility number
         * symbol(s): `Z`
         * application domain: generic
         * name: CompressibilityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute compressibilityNumber : CompressibilityNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.1 Reynolds magnetic number */
    attribute def ReynoldsMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.1 Reynolds magnetic number
         * symbol(s): `Rm`
         * application domain: generic
         * name: ReynoldsMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)
         * remarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).
         */
    }
    attribute reynoldsMagneticNumber : ReynoldsMagneticNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.2 Batchelor number */
    attribute def BatchelorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.2 Batchelor number
         * symbol(s): `Bt`
         * application domain: generic
         * name: BatchelorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute batchelorNumber : BatchelorNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.3 Nusselt electric number */
    attribute def NusseltElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.3 Nusselt electric number
         * symbol(s): `Ne`
         * application domain: generic
         * name: NusseltElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^"*" = D^"+" + D^"-"`, where `D^"+"`, `D^"-"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively"
         * remarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.
         */
    }
    attribute nusseltElectricNumber : NusseltElectricNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.4 Alfvén number, Mach magnetic number, Kárman number */
    attribute def 'AlfvénNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number
         * symbol(s): `Al`
         * application domain: generic
         * name: AlfvénNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: Often, the inverse of this number is wrongly used. The name "Alfvén Mach number" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute 'alfvénNumber' : 'AlfvénNumberValue' :> scalarQuantities;
    alias machMagneticNumber for 'alfvénNumber';
    alias 'kármanNumber' for 'alfvénNumber';
    /* ISO-80000-11 item 11-8.5 Hartmann number */
    attribute def HartmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.5 Hartmann number
         * symbol(s): `Ha`
         * application domain: generic
         * name: HartmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: The Hartmann number represents also the ratio of magnetic force to viscous force.
         */
    }
    attribute hartmannNumber : HartmannNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.6 Cowling number, Euler magnetic number */
    attribute def CowlingNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.6 Cowling number, Euler magnetic number
         * symbol(s): `Co`
         * application domain: magnetism
         * name: CowlingNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).
         */
    }
    attribute cowlingNumber : CowlingNumberValue :> scalarQuantities;
    alias eulerMagneticNumber for cowlingNumber;
    /* ISO-80000-11 item 11-8.7 Stuart electrical number */
    attribute def StuartElectricalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.7 Stuart electrical number
         * symbol(s): `Se`
         * application domain: generic
         * name: StuartElectricalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).
         */
    }
    attribute stuartElectricalNumber : StuartElectricalNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.8 magnetic pressure number */
    attribute def MagneticPressureNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.8 magnetic pressure number
         * symbol(s): `N_(mp)`
         * application domain: generic
         * name: MagneticPressureNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute magneticPressureNumber : MagneticPressureNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.9 Chandrasekhar number */
    attribute def ChandrasekharNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.9 Chandrasekhar number
         * symbol(s): `Q`, `Ch`
         * application domain: generic
         * name: ChandrasekharNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).
         */
    }
    attribute chandrasekharNumber : ChandrasekharNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.10 Prandtl magnetic number */
    attribute def PrandtlMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.10 Prandtl magnetic number
         * symbol(s): `Pr_m`
         * application domain: generic
         * name: PrandtlMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.
         */
    }
    attribute prandtlMagneticNumber : PrandtlMagneticNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.11 Roberts number */
    attribute def RobertsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.11 Roberts number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RobertsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).
         */
    }
    attribute robertsNumber : RobertsNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.12 Stuart number */
    attribute def StuartNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.12 Stuart number
         * symbol(s): `Stw`
         * application domain: generic
         * name: StuartNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. 
         */
    }
    attribute stuartNumber : StuartNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.13 magnetic number */
    attribute def MagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.13 magnetic number
         * symbol(s): `N_(mg)`
         * application domain: generic
         * name: MagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute magneticNumber : MagneticNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.14 electric field parameter */
    attribute def ElectricFieldParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.14 electric field parameter
         * symbol(s): `Ef`
         * application domain: generic
         * name: ElectricFieldParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute electricFieldParameter : ElectricFieldParameterValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.15 Hall number */
    attribute def HallNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.15 Hall number
         * symbol(s): `Hc`, `CH`
         * application domain: generic
         * name: HallNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)
         * remarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.
         */
    }
    attribute hallNumber : HallNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.16 Lundquist number */
    attribute def LundquistNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.16 Lundquist number
         * symbol(s): `Lu`
         * application domain: generic
         * name: LundquistNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).
         */
    }
    attribute lundquistNumber : LundquistNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.17 Joule magnetic number */
    attribute def JouleMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.17 Joule magnetic number
         * symbol(s): `Jo_m`
         * application domain: generic
         * name: JouleMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: This number is also called magnetic Joule number.
         */
    }
    attribute jouleMagneticNumber : JouleMagneticNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.18 Grashof magnetic number */
    attribute def GrashofMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.18 Grashof magnetic number
         * symbol(s): `Gr_m`
         * application domain: generic
         * name: GrashofMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).
         */
    }
    attribute grashofMagneticNumber : GrashofMagneticNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.19 Naze number */
    attribute def NazeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.19 Naze number
         * symbol(s): `Na`
         * application domain: generic
         * name: NazeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.
         */
    }
    attribute nazeNumber : NazeNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.20 Reynolds electric number */
    attribute def ReynoldsElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.20 Reynolds electric number
         * symbol(s): `Re_e`
         * application domain: generic
         * name: ReynoldsElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers
         * remarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.
         */
    }
    attribute reynoldsElectricNumber : ReynoldsElectricNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-8.21 Ampère number */
    attribute def 'AmpèreNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.21 Ampère number
         * symbol(s): `Am`
         * application domain: generic
         * name: AmpèreNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)
         * remarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).
         */
    }
    attribute 'ampèreNumber' : 'AmpèreNumberValue' :> scalarQuantities;
    /* ISO-80000-11 item 11-9.1 Arrhenius number */
    attribute def ArrheniusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.1 Arrhenius number
         * symbol(s): `α`
         * application domain: generic
         * name: ArrheniusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute arrheniusNumber : ArrheniusNumberValue :> scalarQuantities;
    /* ISO-80000-11 item 11-9.2 Landau-Ginzburg number */
    attribute def LandauGinzburgNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.2 Landau-Ginzburg number
         * symbol(s): `κ`
         * application domain: generic
         * name: LandauGinzburgNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)
         * remarks: None.
         */
    }
    attribute landauGinzburgNumber : LandauGinzburgNumberValue :> scalarQuantities;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 805) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 805) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 819) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 844) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 844) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 878) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 878) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 923) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 923) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1028) (line 21) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1028) (line 21) (column 42) (len 17)))))
    (reference r5 (scope relative) (span (offset 2102) (line 38) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 2102) (line 38) (column 39) (len 17)))))
    (reference r6 (scope relative) (span (offset 3147) (line 55) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 3147) (line 55) (column 40) (len 17)))))
    (reference r7 (scope relative) (span (offset 4085) (line 72) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 4085) (line 72) (column 41) (len 17)))))
    (reference r8 (scope relative) (span (offset 5405) (line 89) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 5405) (line 89) (column 39) (len 17)))))
    (reference r9 (scope relative) (span (offset 6676) (line 106) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 6676) (line 106) (column 38) (len 17)))))
    (reference r10 (scope relative) (span (offset 7613) (line 123) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 7613) (line 123) (column 41) (len 17)))))
    (reference r11 (scope relative) (span (offset 8556) (line 140) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 8556) (line 140) (column 42) (len 17)))))
    (reference r12 (scope relative) (span (offset 9470) (line 156) (column 29) (len 14)) (segments (segment 0 (token "strouhalNumber") (name "strouhalNumber") (separator none) (span (offset 9470) (line 156) (column 29) (len 14)))))
    (reference r13 (scope relative) (span (offset 9727) (line 162) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 9727) (line 162) (column 41) (len 17)))))
    (reference r14 (scope relative) (span (offset 10789) (line 179) (column 58) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 10789) (line 179) (column 58) (len 17)))))
    (reference r15 (scope relative) (span (offset 11809) (line 196) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 11809) (line 196) (column 43) (len 17)))))
    (reference r16 (scope relative) (span (offset 12909) (line 213) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 12909) (line 213) (column 45) (len 17)))))
    (reference r17 (scope relative) (span (offset 13872) (line 230) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 13872) (line 230) (column 38) (len 17)))))
    (reference r18 (scope relative) (span (offset 14742) (line 247) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 14742) (line 247) (column 39) (len 17)))))
    (reference r19 (scope relative) (span (offset 15745) (line 264) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 15745) (line 264) (column 42) (len 17)))))
    (reference r20 (scope relative) (span (offset 16745) (line 281) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 16745) (line 281) (column 41) (len 17)))))
    (reference r21 (scope relative) (span (offset 17592) (line 297) (column 32) (len 13)) (segments (segment 0 (token "binghamNumber") (name "binghamNumber") (separator none) (span (offset 17592) (line 297) (column 32) (len 13)))))
    (reference r22 (scope relative) (span (offset 17705) (line 300) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 17705) (line 300) (column 45) (len 17)))))
    (reference r23 (scope relative) (span (offset 18648) (line 317) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 18648) (line 317) (column 44) (len 17)))))
    (reference r24 (scope relative) (span (offset 19695) (line 334) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 19695) (line 334) (column 40) (len 17)))))
    (reference r25 (scope relative) (span (offset 20724) (line 350) (column 28) (len 12)) (segments (segment 0 (token "rossbyNumber") (name "rossbyNumber") (separator none) (span (offset 20724) (line 350) (column 28) (len 12)))))
    (reference r26 (scope relative) (span (offset 20826) (line 353) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 20826) (line 353) (column 39) (len 17)))))
    (reference r27 (scope relative) (span (offset 21952) (line 370) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 21952) (line 370) (column 44) (len 17)))))
    (reference r28 (scope relative) (span (offset 22816) (line 387) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 22816) (line 387) (column 47) (len 17)))))
    (reference r29 (scope relative) (span (offset 23772) (line 403) (column 35) (len 19)) (segments (segment 0 (token "darcyFrictionFactor") (name "darcyFrictionFactor") (separator none) (span (offset 23772) (line 403) (column 35) (len 19)))))
    (reference r30 (scope relative) (span (offset 23885) (line 406) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 23885) (line 406) (column 41) (len 17)))))
    (reference r31 (scope relative) (span (offset 24914) (line 423) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 24914) (line 423) (column 42) (len 17)))))
    (reference r32 (scope relative) (span (offset 25850) (line 439) (column 33) (len 14)) (segments (segment 0 (token "goertlerNumber") (name "goertlerNumber") (separator none) (span (offset 25850) (line 439) (column 33) (len 14)))))
    (reference r33 (scope relative) (span (offset 25954) (line 442) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 25954) (line 442) (column 39) (len 17)))))
    (reference r34 (scope relative) (span (offset 26988) (line 459) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 26988) (line 459) (column 39) (len 17)))))
    (reference r35 (scope relative) (span (offset 27985) (line 476) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 27985) (line 476) (column 44) (len 17)))))
    (reference r36 (scope relative) (span (offset 29034) (line 493) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 29034) (line 493) (column 39) (len 17)))))
    (reference r37 (scope relative) (span (offset 29904) (line 510) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 29904) (line 510) (column 44) (len 17)))))
    (reference r38 (scope relative) (span (offset 30756) (line 527) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 30756) (line 527) (column 39) (len 17)))))
    (reference r39 (scope relative) (span (offset 31836) (line 544) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 31836) (line 544) (column 40) (len 17)))))
    (reference r40 (scope relative) (span (offset 32872) (line 561) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 32872) (line 561) (column 61) (len 17)))))
    (reference r41 (scope relative) (span (offset 33860) (line 578) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 33860) (line 578) (column 52) (len 17)))))
    (reference r42 (scope relative) (span (offset 35026) (line 594) (column 32) (len 12)) (segments (segment 0 (token "stokesNumber") (name "stokesNumber") (separator none) (span (offset 35026) (line 594) (column 32) (len 12)))))
    (reference r43 (scope relative) (span (offset 35140) (line 597) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 35140) (line 597) (column 50) (len 17)))))
    (reference r44 (scope relative) (span (offset 36038) (line 614) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 36038) (line 614) (column 47) (len 17)))))
    (reference r45 (scope relative) (span (offset 36875) (line 631) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 36875) (line 631) (column 41) (len 17)))))
    (reference r46 (scope relative) (span (offset 38029) (line 647) (column 30) (len 13)) (segments (segment 0 (token "laplaceNumber") (name "laplaceNumber") (separator none) (span (offset 38029) (line 647) (column 30) (len 13)))))
    (reference r47 (scope relative) (span (offset 38132) (line 650) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 38132) (line 650) (column 39) (len 17)))))
    (reference r48 (scope relative) (span (offset 39182) (line 667) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 39182) (line 667) (column 44) (len 17)))))
    (reference r49 (scope relative) (span (offset 40170) (line 684) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 40170) (line 684) (column 40) (len 17)))))
    (reference r50 (scope relative) (span (offset 41529) (line 701) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 41529) (line 701) (column 41) (len 17)))))
    (reference r51 (scope relative) (span (offset 42572) (line 718) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 42572) (line 718) (column 43) (len 17)))))
    (reference r52 (scope relative) (span (offset 43482) (line 735) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 43482) (line 735) (column 41) (len 17)))))
    (reference r53 (scope relative) (span (offset 44487) (line 752) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 44487) (line 752) (column 43) (len 17)))))
    (reference r54 (scope relative) (span (offset 45501) (line 769) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 45501) (line 769) (column 42) (len 17)))))
    (reference r55 (scope relative) (span (offset 46830) (line 786) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 46830) (line 786) (column 55) (len 17)))))
    (reference r56 (scope relative) (span (offset 47699) (line 803) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 47699) (line 803) (column 41) (len 17)))))
    (reference r57 (scope relative) (span (offset 49082) (line 820) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 49082) (line 820) (column 38) (len 17)))))
    (reference r58 (scope relative) (span (offset 50043) (line 837) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 50043) (line 837) (column 41) (len 17)))))
    (reference r59 (scope relative) (span (offset 51300) (line 854) (column 35) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 51300) (line 854) (column 35) (len 17)))))
    (reference r60 (scope relative) (span (offset 52419) (line 870) (column 34) (len 7)) (segments (segment 0 (token "jFactor") (name "jFactor") (separator none) (span (offset 52419) (line 870) (column 34) (len 7)))))
    (reference r61 (scope relative) (span (offset 52457) (line 872) (column 29) (len 7)) (segments (segment 0 (token "jFactor") (name "jFactor") (separator none) (span (offset 52457) (line 872) (column 29) (len 7)))))
    (reference r62 (scope relative) (span (offset 52568) (line 875) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 52568) (line 875) (column 54) (len 17)))))
    (reference r63 (scope relative) (span (offset 53472) (line 892) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 53472) (line 892) (column 49) (len 17)))))
    (reference r64 (scope relative) (span (offset 54248) (line 909) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 54248) (line 909) (column 40) (len 17)))))
    (reference r65 (scope relative) (span (offset 55197) (line 926) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 55197) (line 926) (column 42) (len 17)))))
    (reference r66 (scope relative) (span (offset 56201) (line 943) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 56201) (line 943) (column 42) (len 17)))))
    (reference r67 (scope relative) (span (offset 57175) (line 960) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 57175) (line 960) (column 40) (len 17)))))
    (reference r68 (scope relative) (span (offset 58009) (line 977) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 58009) (line 977) (column 40) (len 17)))))
    (reference r69 (scope relative) (span (offset 58880) (line 993) (column 28) (len 12)) (segments (segment 0 (token "eckertNumber") (name "eckertNumber") (separator none) (span (offset 58880) (line 993) (column 28) (len 12)))))
    (reference r70 (scope relative) (span (offset 58984) (line 996) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 58984) (line 996) (column 40) (len 17)))))
    (reference r71 (scope relative) (span (offset 59877) (line 1013) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 59877) (line 1013) (column 46) (len 17)))))
    (reference r72 (scope relative) (span (offset 60727) (line 1030) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 60727) (line 1030) (column 45) (len 17)))))
    (reference r73 (scope relative) (span (offset 61840) (line 1047) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 61840) (line 1047) (column 43) (len 17)))))
    (reference r74 (scope relative) (span (offset 62838) (line 1064) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 62838) (line 1064) (column 39) (len 17)))))
    (reference r75 (scope relative) (span (offset 64142) (line 1081) (column 56) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 64142) (line 1081) (column 56) (len 17)))))
    (reference r76 (scope relative) (span (offset 65273) (line 1098) (column 58) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 65273) (line 1098) (column 58) (len 17)))))
    (reference r77 (scope relative) (span (offset 66487) (line 1115) (column 56) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 66487) (line 1115) (column 56) (len 17)))))
    (reference r78 (scope relative) (span (offset 67742) (line 1132) (column 56) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 67742) (line 1132) (column 56) (len 17)))))
    (reference r79 (scope relative) (span (offset 68926) (line 1149) (column 56) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 68926) (line 1149) (column 56) (len 17)))))
    (reference r80 (scope relative) (span (offset 69973) (line 1166) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 69973) (line 1166) (column 55) (len 17)))))
    (reference r81 (scope relative) (span (offset 70985) (line 1183) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 70985) (line 1183) (column 46) (len 17)))))
    (reference r82 (scope relative) (span (offset 72342) (line 1200) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 72342) (line 1200) (column 40) (len 17)))))
    (reference r83 (scope relative) (span (offset 73168) (line 1217) (column 53) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 73168) (line 1217) (column 53) (len 17)))))
    (reference r84 (scope relative) (span (offset 74242) (line 1234) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 74242) (line 1234) (column 40) (len 17)))))
    (reference r85 (scope relative) (span (offset 75513) (line 1251) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 75513) (line 1251) (column 38) (len 17)))))
    (reference r86 (scope relative) (span (offset 76833) (line 1267) (column 32) (len 10)) (segments (segment 0 (token "bondNumber") (name "bondNumber") (separator none) (span (offset 76833) (line 1267) (column 32) (len 10)))))
    (reference r87 (scope relative) (span (offset 76943) (line 1270) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 76943) (line 1270) (column 44) (len 17)))))
    (reference r88 (scope relative) (span (offset 78046) (line 1287) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 78046) (line 1287) (column 43) (len 17)))))
    (reference r89 (scope relative) (span (offset 79008) (line 1304) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 79008) (line 1304) (column 43) (len 17)))))
    (reference r90 (scope relative) (span (offset 80309) (line 1321) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 80309) (line 1321) (column 55) (len 17)))))
    (reference r91 (scope relative) (span (offset 81317) (line 1338) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 81317) (line 1338) (column 54) (len 17)))))
    (reference r92 (scope relative) (span (offset 82337) (line 1355) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 82337) (line 1355) (column 44) (len 17)))))
    (reference r93 (scope relative) (span (offset 83387) (line 1372) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 83387) (line 1372) (column 44) (len 17)))))
    (reference r94 (scope relative) (span (offset 84541) (line 1389) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 84541) (line 1389) (column 43) (len 17)))))
    (reference r95 (scope relative) (span (offset 85430) (line 1406) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 85430) (line 1406) (column 50) (len 17)))))
    (reference r96 (scope relative) (span (offset 86503) (line 1423) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 86503) (line 1423) (column 41) (len 17)))))
    (reference r97 (scope relative) (span (offset 87587) (line 1440) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 87587) (line 1440) (column 41) (len 17)))))
    (reference r98 (scope relative) (span (offset 88435) (line 1457) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 88435) (line 1457) (column 39) (len 17)))))
    (reference r99 (scope relative) (span (offset 89418) (line 1474) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 89418) (line 1474) (column 43) (len 17)))))
    (reference r100 (scope relative) (span (offset 90574) (line 1491) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 90574) (line 1491) (column 40) (len 17)))))
    (reference r101 (scope relative) (span (offset 91319) (line 1507) (column 39) (len 12)) (segments (segment 0 (token "cauchyNumber") (name "cauchyNumber") (separator none) (span (offset 91319) (line 1507) (column 39) (len 12)))))
    (reference r102 (scope relative) (span (offset 91420) (line 1510) (column 39) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 91420) (line 1510) (column 39) (len 17)))))
    (reference r103 (scope relative) (span (offset 92191) (line 1527) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 92191) (line 1527) (column 45) (len 17)))))
    (reference r104 (scope relative) (span (offset 93151) (line 1544) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 93151) (line 1544) (column 41) (len 17)))))
    (reference r105 (scope relative) (span (offset 93991) (line 1561) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 93991) (line 1561) (column 41) (len 17)))))
    (reference r106 (scope relative) (span (offset 94931) (line 1578) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 94931) (line 1578) (column 49) (len 17)))))
    (reference r107 (scope relative) (span (offset 95815) (line 1595) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 95815) (line 1595) (column 50) (len 17)))))
    (reference r108 (scope relative) (span (offset 97012) (line 1612) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 97012) (line 1612) (column 43) (len 17)))))
    (reference r109 (scope relative) (span (offset 97984) (line 1629) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 97984) (line 1629) (column 49) (len 17)))))
    (reference r110 (scope relative) (span (offset 99042) (line 1646) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 99042) (line 1646) (column 43) (len 17)))))
    (reference r111 (scope relative) (span (offset 100154) (line 1662) (column 34) (len 15)) (segments (segment 0 (token "'alfvénNumber'") (name "alfvénNumber") (separator none) (span (offset 100154) (line 1662) (column 34) (len 15)))))
    (reference r112 (scope relative) (span (offset 100202) (line 1664) (column 31) (len 15)) (segments (segment 0 (token "'alfvénNumber'") (name "alfvénNumber") (separator none) (span (offset 100202) (line 1664) (column 31) (len 15)))))
    (reference r113 (scope relative) (span (offset 100312) (line 1667) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 100312) (line 1667) (column 42) (len 17)))))
    (reference r114 (scope relative) (span (offset 101291) (line 1684) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 101291) (line 1684) (column 41) (len 17)))))
    (reference r115 (scope relative) (span (offset 102394) (line 1700) (column 35) (len 13)) (segments (segment 0 (token "cowlingNumber") (name "cowlingNumber") (separator none) (span (offset 102394) (line 1700) (column 35) (len 13)))))
    (reference r116 (scope relative) (span (offset 102519) (line 1703) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 102519) (line 1703) (column 50) (len 17)))))
    (reference r117 (scope relative) (span (offset 103484) (line 1720) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 103484) (line 1720) (column 50) (len 17)))))
    (reference r118 (scope relative) (span (offset 104467) (line 1737) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 104467) (line 1737) (column 47) (len 17)))))
    (reference r119 (scope relative) (span (offset 105525) (line 1754) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 105525) (line 1754) (column 49) (len 17)))))
    (reference r120 (scope relative) (span (offset 106693) (line 1771) (column 41) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 106693) (line 1771) (column 41) (len 17)))))
    (reference r121 (scope relative) (span (offset 107682) (line 1788) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 107682) (line 1788) (column 40) (len 17)))))
    (reference r122 (scope relative) (span (offset 108799) (line 1805) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 108799) (line 1805) (column 42) (len 17)))))
    (reference r123 (scope relative) (span (offset 109714) (line 1822) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 109714) (line 1822) (column 50) (len 17)))))
    (reference r124 (scope relative) (span (offset 110543) (line 1839) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 110543) (line 1839) (column 38) (len 17)))))
    (reference r125 (scope relative) (span (offset 111432) (line 1856) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 111432) (line 1856) (column 43) (len 17)))))
    (reference r126 (scope relative) (span (offset 112815) (line 1873) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 112815) (line 1873) (column 47) (len 17)))))
    (reference r127 (scope relative) (span (offset 113814) (line 1890) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 113814) (line 1890) (column 49) (len 17)))))
    (reference r128 (scope relative) (span (offset 115080) (line 1907) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 115080) (line 1907) (column 38) (len 17)))))
    (reference r129 (scope relative) (span (offset 115990) (line 1924) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 115990) (line 1924) (column 50) (len 17)))))
    (reference r130 (scope relative) (span (offset 117240) (line 1941) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 117240) (line 1941) (column 43) (len 17)))))
    (reference r131 (scope relative) (span (offset 118382) (line 1958) (column 43) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 118382) (line 1958) (column 43) (len 17)))))
    (reference r132 (scope relative) (span (offset 119285) (line 1975) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 119285) (line 1975) (column 48) (len 17)))))
  )
  (root (library-package (name "ISQCharacteristicNumbers") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 66) (line 3) (column 7) (len 716)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 \"Characteristic numbers\"\nsee also https://www.iso.org/standard/64982.html\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\nNote 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \ndefined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \nor TensorMeasurementReference.\n"))) (import (target (span (span (offset 805) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 844) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 854) (line 16) (column 30) (len 3))) (separator (span (offset 854) (line 16) (column 30) (len 2))) (marker (span (offset 856) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 878) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 899) (line 17) (column 41) (len 3))) (separator (span (offset 899) (line 17) (column 41) (len 2))) (marker (span (offset 901) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 923) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 930) (line 18) (column 27) (len 3))) (separator (span (offset 930) (line 18) (column 27) (len 2))) (marker (span (offset 932) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 942) (line 20) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-4.1 Reynolds number "))) (attribute-def (declaration-name "ReynoldsNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1070) (line 23) (column 11) (len 865)) (normalized "source: item 11-4.1 Reynolds number\nsymbol(s): `Re`\napplication domain: generic\nname: ReynoldsNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2022) (line 37) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-4.2 Euler number "))) (attribute-def (declaration-name "EulerNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2144) (line 40) (column 11) (len 840)) (normalized "source: item 11-4.2 Euler number\nsymbol(s): `Eu`\napplication domain: generic\nname: EulerNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^\"'\" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3065) (line 54) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.3 Froude number "))) (attribute-def (declaration-name "FroudeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3189) (line 57) (column 11) (len 729)) (normalized "source: item 11-4.3 Froude number\nsymbol(s): `Fr`\napplication domain: generic\nname: FroudeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)\nremarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4001) (line 71) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.4 Grashof number "))) (attribute-def (declaration-name "GrashofNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4127) (line 74) (column 11) (len 1113)) (normalized "source: item 11-4.4 Grashof number\nsymbol(s): `Gr`\napplication domain: generic\nname: GrashofNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5325) (line 88) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-4.5 Weber number "))) (attribute-def (declaration-name "WeberNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5447) (line 91) (column 11) (len 1070)) (normalized "source: item 11-4.5 Weber number\nsymbol(s): `We`\napplication domain: generic\nname: WeberNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)\nremarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 6598) (line 105) (column 7) (len 38)) (normalized "ISO-80000-11 item 11-4.6 Mach number "))) (attribute-def (declaration-name "MachNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6718) (line 108) (column 11) (len 732)) (normalized "source: item 11-4.6 Mach number\nsymbol(s): `Ma`\napplication domain: generic\nname: MachNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid\nremarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7529) (line 122) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.7 Knudsen number "))) (attribute-def (declaration-name "KnudsenNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7655) (line 125) (column 11) (len 714)) (normalized "source: item 11-4.7 Knudsen number\nsymbol(s): `Kn`\napplication domain: generic\nname: KnudsenNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)\nremarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 8454) (line 139) (column 7) (len 58)) (normalized "ISO-80000-11 item 11-4.8 Strouhal number, Thomson number "))) (attribute-def (declaration-name "StrouhalNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8598) (line 142) (column 11) (len 763)) (normalized "source: item 11-4.8 Strouhal number, Thomson number\nsymbol(s): `Sr`, `Sh`\napplication domain: generic\nname: StrouhalNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow\nremarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.\n"))))) (attribute-usage) (alias (name "thomsonNumber") (target (ref r12)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9493) (line 158) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-4.9 drag coefficient "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9545) (line 159) (column 7) (len 87)) (normalized "Refer to declaration for DragCoefficient in ISQMechanics item 4-23.4 drag coefficient "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9642) (line 161) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-4.10 Bagnold number "))) (attribute-def (declaration-name "BagnoldNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9769) (line 164) (column 11) (len 833)) (normalized "source: item 11-4.10 Bagnold number\nsymbol(s): `Bg`\napplication domain: generic\nname: BagnoldNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body\nremarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 10687) (line 178) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-4.11 Bagnold number "))) (attribute-def (declaration-name "BagnoldNumberForSolidParticlesValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 10831) (line 181) (column 11) (len 770)) (normalized "source: item 11-4.11 Bagnold number\nsymbol(s): `Ba_2`\napplication domain: solid particles\nname: BagnoldNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 11720) (line 195) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-4.12 lift coefficient "))) (attribute-def (declaration-name "LiftCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 11851) (line 198) (column 11) (len 876)) (normalized "source: item 11-4.12 lift coefficient\nsymbol(s): `c_l`, `c_A`\napplication domain: generic\nname: LiftCoefficient (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure\nremarks: The lift coefficient is dependant on the shape of the wing.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 12816) (line 212) (column 7) (len 46)) (normalized "ISO-80000-11 item 11-4.13 thrust coefficient "))) (attribute-def (declaration-name "ThrustCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12951) (line 215) (column 11) (len 749)) (normalized "source: item 11-4.13 thrust coefficient\nsymbol(s): `c_t`\napplication domain: generic\nname: ThrustCoefficient (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller\nremarks: The thrust coefficient is dependant on the shape of the propeller.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 13793) (line 229) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-4.14 Dean number "))) (attribute-def (declaration-name "DeanNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 13914) (line 232) (column 11) (len 668)) (normalized "source: item 11-4.14 Dean number\nsymbol(s): `Dn`\napplication domain: generic\nname: DeanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 14661) (line 246) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.15 Bejan number "))) (attribute-def (declaration-name "BejanNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 14784) (line 249) (column 11) (len 793)) (normalized "source: item 11-4.15 Bejan number\nsymbol(s): `Be`\napplication domain: generic\nname: BejanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)\nremarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 15658) (line 263) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-4.16 Lagrange number "))) (attribute-def (declaration-name "LagrangeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15787) (line 266) (column 11) (len 767)) (normalized "source: item 11-4.16 Lagrange number\nsymbol(s): `Lg`\napplication domain: generic\nname: LagrangeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 16641) (line 280) (column 7) (len 61)) (normalized "ISO-80000-11 item 11-4.17 Bingham number, plasticity number "))) (attribute-def (declaration-name "BinghamNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 16787) (line 283) (column 11) (len 695)) (normalized "source: item 11-4.17 Bingham number, plasticity number\nsymbol(s): `Bm`, `Bn`\napplication domain: generic\nname: BinghamNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: None.\n"))))) (attribute-usage) (alias (name "plasticityNumber") (target (ref r21)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 17614) (line 299) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-4.18 Hedström number "))) (attribute-def (declaration-name "HedströmNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 17747) (line 302) (column 11) (len 717)) (normalized "source: item 11-4.18 Hedström number\nsymbol(s): `He`, `Hd`\napplication domain: generic\nname: HedströmNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 18557) (line 316) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-4.19 Bodenstein number "))) (attribute-def (declaration-name "BodensteinNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 18690) (line 319) (column 11) (len 816)) (normalized "source: item 11-4.19 Bodenstein number\nsymbol(s): `Bd`\napplication domain: generic\nname: BodensteinNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)\nremarks: The Bodenstein number is also given by `Bd = Pe^\"*\" = Re*Sc`, where `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 19597) (line 333) (column 7) (len 56)) (normalized "ISO-80000-11 item 11-4.20 Rossby number, Kiebel number "))) (attribute-def (declaration-name "RossbyNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 19737) (line 336) (column 11) (len 883)) (normalized "source: item 11-4.20 Rossby number, Kiebel number\nsymbol(s): `Ro`\napplication domain: generic\nname: RossbyNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude\nremarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.\n"))))) (attribute-usage) (alias (name "kiebelNumber") (target (ref r25)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 20745) (line 352) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.21 Ekman number "))) (attribute-def (declaration-name "EkmanNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 20868) (line 355) (column 11) (len 912)) (normalized "source: item 11-4.21 Ekman number\nsymbol(s): `Ek`\napplication domain: generic\nname: EkmanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude\nremarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 21861) (line 369) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-4.22 elasticity number "))) (attribute-def (declaration-name "ElasticityNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 21994) (line 372) (column 11) (len 610)) (normalized "source: item 11-4.22 elasticity number\nsymbol(s): `El`\napplication domain: generic\nname: ElasticityNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe\nremarks: See also Deborah number (item 11-7.8).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 22695) (line 386) (column 7) (len 72)) (normalized "ISO-80000-11 item 11-4.23 Darcy friction factor, Moody friction factor "))) (attribute-def (declaration-name "DarcyFrictionFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 22858) (line 389) (column 11) (len 789)) (normalized "source: item 11-4.23 Darcy friction factor, Moody friction factor\nsymbol(s): `f_D`\napplication domain: generic\nname: DarcyFrictionFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe\nremarks: None.\n"))))) (attribute-usage) (alias (name "moodyFrictionFactor") (target (ref r29)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 23800) (line 405) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-4.24 Fanning number "))) (attribute-def (declaration-name "FanningNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 23927) (line 408) (column 11) (len 795)) (normalized "source: item 11-4.24 Fanning number\nsymbol(s): `f_n`, `f`\napplication domain: generic\nname: FanningNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe\nremarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 24807) (line 422) (column 7) (len 63)) (normalized "ISO-80000-11 item 11-4.25 Goertler number, Goertler parameter "))) (attribute-def (declaration-name "GoertlerNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 24956) (line 425) (column 11) (len 781)) (normalized "source: item 11-4.25 Goertler number, Goertler parameter\nsymbol(s): `Go`\napplication domain: generic\nname: GoertlerNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)\nremarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.\n"))))) (attribute-usage) (alias (name "goertlerParameter") (target (ref r32)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 25873) (line 441) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.26 Hagen number "))) (attribute-def (declaration-name "HagenNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 25996) (line 444) (column 11) (len 830)) (normalized "source: item 11-4.26 Hagen number\nsymbol(s): `Hg`, `Ha`\napplication domain: generic\nname: HagenNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 26907) (line 458) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.27 Laval number "))) (attribute-def (declaration-name "LavalNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 27030) (line 461) (column 11) (len 783)) (normalized "source: item 11-4.27 Laval number\nsymbol(s): `La`\napplication domain: generic\nname: LavalNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3),  `R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)\nremarks: The Laval number is a specific kind of Mach number (item 11-4.6).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 27894) (line 475) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-4.28 Poiseuille number "))) (attribute-def (declaration-name "PoiseuilleNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 28027) (line 478) (column 11) (len 835)) (normalized "source: item 11-4.28 Poiseuille number\nsymbol(s): `Poi`\napplication domain: generic\nname: PoiseuilleNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid\nremarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28953) (line 492) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.29 power number "))) (attribute-def (declaration-name "PowerNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r36)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 29076) (line 495) (column 11) (len 656)) (normalized "source: item 11-4.29 power number\nsymbol(s): `Pn`\napplication domain: generic\nname: PowerNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 29813) (line 509) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-4.30 Richardson number "))) (attribute-def (declaration-name "RichardsonNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 29946) (line 512) (column 11) (len 638)) (normalized "source: item 11-4.30 Richardson number\nsymbol(s): `Ri`\napplication domain: generic\nname: RichardsonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)\nremarks: In geophysics differences of these quantities are of interest.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 30675) (line 526) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.31 Reech number "))) (attribute-def (declaration-name "ReechNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 30798) (line 529) (column 11) (len 874)) (normalized "source: item 11-4.31 Reech number\nsymbol(s): `Ree`\napplication domain: generic\nname: ReechNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water\nremarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 31753) (line 543) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.32 Stokes number "))) (attribute-def (declaration-name "StokesNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r39)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 31878) (line 546) (column 11) (len 807)) (normalized "source: item 11-4.32 Stokes number\nsymbol(s): `Stk`\napplication domain: time-related\nname: StokesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence\nremarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 32768) (line 560) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.33 Stokes number "))) (attribute-def (declaration-name "StokesNumberForVibratingParticlesValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r40)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 32914) (line 563) (column 11) (len 707)) (normalized "source: item 11-4.33 Stokes number\nsymbol(s): `Stk_1`\napplication domain: vibrating particles\nname: StokesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations\nremarks: Sometimes the inverse of this number is wrongly used.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 33746) (line 577) (column 7) (len 60)) (normalized "ISO-80000-11 item 11-4.34 Stokes number, power coefficient "))) (attribute-def (declaration-name "StokesNumberForRotameterValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r41)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 33902) (line 580) (column 11) (len 992)) (normalized "source: item 11-4.34 Stokes number, power coefficient\nsymbol(s): `Stk_2`\napplication domain: rotameter\nname: StokesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid\nremarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).\n"))))) (attribute-usage) (alias (name "powerCoefficient") (target (ref r42)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 35047) (line 596) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.35 Stokes number "))) (attribute-def (declaration-name "StokesNumberForGravityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r43)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 35182) (line 599) (column 11) (len 663)) (normalized "source: item 11-4.35 Stokes number\nsymbol(s): `Stk_3`\napplication domain: gravity\nname: StokesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 35948) (line 613) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.36 Stokes number "))) (attribute-def (declaration-name "StokesNumberForDragValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r44)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36080) (line 616) (column 11) (len 596)) (normalized "source: item 11-4.36 Stokes number\nsymbol(s): `Stk_4`\napplication domain: drag\nname: StokesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 36773) (line 630) (column 7) (len 59)) (normalized "ISO-80000-11 item 11-4.37 Laplace number, Suratman number "))) (attribute-def (declaration-name "LaplaceNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r45)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36917) (line 633) (column 11) (len 1004)) (normalized "source: item 11-4.37 Laplace number, Suratman number\nsymbol(s): `La`, `Su`\napplication domain: generic\nname: LaplaceNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid\nremarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).\n"))))) (attribute-usage) (alias (name "suratmanNumber") (target (ref r46)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38051) (line 649) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-4.38 Blake number "))) (attribute-def (declaration-name "BlakeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r47)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38174) (line 652) (column 11) (len 836)) (normalized "source: item 11-4.38 Blake number\nsymbol(s): `Bl`\napplication domain: generic\nname: BlakeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)\nremarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 39091) (line 666) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-4.39 Sommerfeld number "))) (attribute-def (declaration-name "SommerfeldNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r48)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 39224) (line 669) (column 11) (len 772)) (normalized "source: item 11-4.39 Sommerfeld number\nsymbol(s): `So`, `Sm`\napplication domain: generic\nname: SommerfeldNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus\nremarks: Sometimes the inverse of this number is wrongly used.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 40087) (line 683) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-4.40 Taylor number "))) (attribute-def (declaration-name "TaylorNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r49)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40212) (line 686) (column 11) (len 1149)) (normalized "source: item 11-4.40 Taylor number\nsymbol(s): `Ta`\napplication domain: momentum transfer\nname: TaylorNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 41444) (line 700) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-4.41 Galilei number "))) (attribute-def (declaration-name "GalileiNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r50)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 41571) (line 703) (column 11) (len 827)) (normalized "source: item 11-4.41 Galilei number\nsymbol(s): `Ga`\napplication domain: generic\nname: GalileiNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid\nremarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 42483) (line 717) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-4.42 Womersley number "))) (attribute-def (declaration-name "WomersleyNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r51)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 42614) (line 720) (column 11) (len 695)) (normalized "source: item 11-4.42 Womersley number\nsymbol(s): `Wo`, `α`\napplication domain: generic\nname: WomersleyNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: The Womersley number is used for pulsating flows e.g. in blood flow.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 43398) (line 734) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.1 Fourier number "))) (attribute-def (declaration-name "FourierNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r52)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 43524) (line 737) (column 11) (len 792)) (normalized "source: item 11-5.1 Fourier number\nsymbol(s): `Fo`\napplication domain: heat transfer\nname: FourierNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)\nremarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 44401) (line 751) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.2 Péclet number "))) (attribute-def (declaration-name "PécletNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r53)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 44529) (line 754) (column 11) (len 797)) (normalized "source: item 11-5.2 Péclet number\nsymbol(s): `Pe`\napplication domain: heat transfer\nname: PécletNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)\nremarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45415) (line 768) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-5.3 Rayleigh number "))) (attribute-def (declaration-name "RayleighNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r54)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 45543) (line 771) (column 11) (len 1103)) (normalized "source: item 11-5.3 Rayleigh number\nsymbol(s): `Ra`\napplication domain: generic\nname: RayleighNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid\nremarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 46733) (line 785) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-5.4 Froude number "))) (attribute-def (declaration-name "FroudeNumberForHeatTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r55)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 46872) (line 788) (column 11) (len 630)) (normalized "source: item 11-5.4 Froude number\nsymbol(s): `Fr^\"*\"`\napplication domain: heat transfer\nname: FroudeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^\"*\" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)\"\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 47615) (line 802) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.5 Nusselt number "))) (attribute-def (declaration-name "NusseltNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r56)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 47741) (line 805) (column 11) (len 1178)) (normalized "source: item 11-5.5 Nusselt number\nsymbol(s): `Nu`\napplication domain: heat transfer\nname: NusseltNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)\nremarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the \"Biot number for heat transfer\" (item 11-5.6) is used.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49004) (line 819) (column 7) (len 38)) (normalized "ISO-80000-11 item 11-5.6 Biot number "))) (attribute-def (declaration-name "BiotNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r57)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 49124) (line 822) (column 11) (len 756)) (normalized "source: item 11-5.6 Biot number\nsymbol(s): `Bi`\napplication domain: heat transfer\nname: BiotNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body\nremarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49959) (line 836) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.7 Stanton number "))) (attribute-def (declaration-name "StantonNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r58)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50085) (line 839) (column 11) (len 1020)) (normalized "source: item 11-5.7 Stanton number\nsymbol(s): `St`\napplication domain: heat transfer\nname: StantonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid\nremarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 51190) (line 853) (column 7) (len 73)) (normalized "ISO-80000-11 item 11-5.8 j-factor, heat transfer factor, Colburn number "))) (attribute-def (declaration-name "JFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r59)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 51342) (line 856) (column 11) (len 977)) (normalized "source: item 11-5.8 j-factor, heat transfer factor, Colburn number\nsymbol(s): `j`, `Co`, `Jq`\napplication domain: heat transfer\nname: JFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)\nremarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).\n"))))) (attribute-usage) (alias (name "heatTransferFactor") (target (ref r60)) (body semicolon)) (alias (name "colburnNumber") (target (ref r61)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 52473) (line 874) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-5.9 Bejan number "))) (attribute-def (declaration-name "BejanNumberForHeatTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r62)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 52610) (line 877) (column 11) (len 660)) (normalized "source: item 11-5.9 Bejan number\nsymbol(s): `Be_1`\napplication domain: heat transfer\nname: BejanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 53381) (line 891) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-5.10 Bejan number "))) (attribute-def (declaration-name "BejanNumberForEntropyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r63)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 53514) (line 894) (column 11) (len 550)) (normalized "source: item 11-5.10 Bejan number\nsymbol(s): `Be_S`\napplication domain: entropy\nname: BejanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 54165) (line 908) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.11 Stefan number "))) (attribute-def (declaration-name "StefanNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r64)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 54290) (line 911) (column 11) (len 737)) (normalized "source: item 11-5.11 Stefan number\nsymbol(s): `Ste`, `Stf`\napplication domain: phase transition\nname: StefanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 55110) (line 925) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-5.12 Brinkman number "))) (attribute-def (declaration-name "BrinkmanNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r65)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 55239) (line 928) (column 11) (len 788)) (normalized "source: item 11-5.12 Brinkman number\nsymbol(s): `Br`, `N_(Br)`\napplication domain: generic\nname: BrinkmanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 56114) (line 942) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-5.13 Clausius number "))) (attribute-def (declaration-name "ClausiusNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r66)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 56243) (line 945) (column 11) (len 762)) (normalized "source: item 11-5.13 Clausius number\nsymbol(s): `Cl`\napplication domain: generic\nname: ClausiusNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 57092) (line 959) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.14 Carnot number "))) (attribute-def (declaration-name "CarnotNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r67)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 57217) (line 962) (column 11) (len 611)) (normalized "source: item 11-5.14 Carnot number\nsymbol(s): `Ca`\napplication domain: generic\nname: CarnotNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 57911) (line 976) (column 7) (len 56)) (normalized "ISO-80000-11 item 11-5.15 Eckert number, Dulong number "))) (attribute-def (declaration-name "EckertNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r68)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 58051) (line 979) (column 11) (len 725)) (normalized "source: item 11-5.15 Eckert number, Dulong number\nsymbol(s): `Ec`\napplication domain: generic\nname: EckertNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)\nremarks: None.\n"))))) (attribute-usage) (alias (name "dulongNumber") (target (ref r69)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 58901) (line 995) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-5.16 Graetz number "))) (attribute-def (declaration-name "GraetzNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r70)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 59026) (line 998) (column 11) (len 672)) (normalized "source: item 11-5.16 Graetz number\nsymbol(s): `Gz`\napplication domain: heat transfer\nname: GraetzNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 59781) (line 1012) (column 7) (len 48)) (normalized "ISO-80000-11 item 11-5.17 heat transfer number "))) (attribute-def (declaration-name "HeatTransferNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r71)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 59919) (line 1015) (column 11) (len 620)) (normalized "source: item 11-5.17 heat transfer number\nsymbol(s): `K_Q`\napplication domain: generic\nname: HeatTransferNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 60634) (line 1029) (column 7) (len 46)) (normalized "ISO-80000-11 item 11-5.18 Pomerantsev number "))) (attribute-def (declaration-name "PomerantsevNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r72)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 60769) (line 1032) (column 11) (len 889)) (normalized "source: item 11-5.18 Pomerantsev number\nsymbol(s): `Po`, `Pov`\napplication domain: heat transfer\nname: PomerantsevNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)\nremarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 61751) (line 1046) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-5.19 Boltzmann number "))) (attribute-def (declaration-name "BoltzmannNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 61882) (line 1049) (column 11) (len 786)) (normalized "source: item 11-5.19 Boltzmann number\nsymbol(s): `Bz`, `Bol`, `Bo`\napplication domain: generic\nname: BoltzmannNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 62757) (line 1063) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-5.20 Stark number "))) (attribute-def (declaration-name "StarkNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r74)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 62880) (line 1066) (column 11) (len 1082)) (normalized "source: item 11-5.20 Stark number\nsymbol(s): `Sk`\napplication domain: generic\nname: StarkNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)\nremarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 64043) (line 1080) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.1 Fourier number "))) (attribute-def (declaration-name "FourierNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r75)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 64184) (line 1083) (column 11) (len 873)) (normalized "source: item 11-6.1 Fourier number\nsymbol(s): `Fo^\"*\"`\napplication domain: mass transfer\nname: FourierNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^\"*\" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer\"\nremarks: The Fourier number for mass transfer is also given by `Fo^*\" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1).\"\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 65172) (line 1097) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.2 Péclet number "))) (attribute-def (declaration-name "PécletNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r76)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 65315) (line 1100) (column 11) (len 954)) (normalized "source: item 11-6.2 Péclet number\nsymbol(s): `Pe^\"*\"`, `Bd`, `Bod`\napplication domain: mass transfer\nname: PécletNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*\" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)\"\nremarks: The Péclet number for mass transfer is also given by `Pe^\"*\" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 66388) (line 1114) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.3 Grashof number "))) (attribute-def (declaration-name "GrashofNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r77)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 66529) (line 1117) (column 11) (len 999)) (normalized "source: item 11-6.3 Grashof number\nsymbol(s): `Gr^\"*\"`\napplication domain: mass transfer\nname: GrashofNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^\"*\" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)\"\nremarks: Instead of \"amount-of-substance fraction\" the \"amount-of-substance concentration\" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 67643) (line 1131) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.4 Nusselt number "))) (attribute-def (declaration-name "NusseltNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r78)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 67784) (line 1134) (column 11) (len 928)) (normalized "source: item 11-6.4 Nusselt number\nsymbol(s): `Nu^\"*\"`\napplication domain: mass transfer\nname: NusseltNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^\"*\" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)\"\nremarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 68827) (line 1148) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.5 Stanton number "))) (attribute-def (declaration-name "StantonNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r79)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 68968) (line 1151) (column 11) (len 793)) (normalized "source: item 11-6.5 Stanton number\nsymbol(s): `St^\"*\"`\napplication domain: mass transfer\nname: StantonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^\"*\" = k^\"*\"\nremarks: The Stanton number for mass transfer is also given by `St^*\" = (Nu^\"*\")/(Pe^\"*\"*)`, where `Nu^\"*\"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer.\"\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 69876) (line 1165) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-6.6 Graetz number "))) (attribute-def (declaration-name "GraetzNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r80)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 70015) (line 1168) (column 11) (len 762)) (normalized "source: item 11-6.6 Graetz number\nsymbol(s): `Gz^\"*\"`\napplication domain: mass transfer\nname: GraetzNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^\"*\" = (v*d)/D = d/l*Pe^\"*\"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2)\"\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 70890) (line 1182) (column 7) (len 47)) (normalized "ISO-80000-11 item 11-6.7 mass transfer factor "))) (attribute-def (declaration-name "MassTransferFactorValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r81)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 71027) (line 1185) (column 11) (len 1138)) (normalized "source: item 11-6.7 mass transfer factor\nsymbol(s): `j^\"*\"`\napplication domain: mass transfer\nname: MassTransferFactor (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*\" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)\"\nremarks: The mass transfer factor is also given by `j_m = j^*\" = St^\"*\" * (Sc)^(2/3)` where `St^\"*\"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17).\"\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 72260) (line 1199) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-6.8 Atwood number "))) (attribute-def (declaration-name "AtwoodNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r82)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 72384) (line 1202) (column 11) (len 608)) (normalized "source: item 11-6.8 Atwood number\nsymbol(s): `At`\napplication domain: generic\nname: AtwoodNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid\nremarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 73075) (line 1216) (column 7) (len 38)) (normalized "ISO-80000-11 item 11-6.9 Biot number "))) (attribute-def (declaration-name "BiotNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r83)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 73210) (line 1219) (column 11) (len 840)) (normalized "source: item 11-6.9 Biot number\nsymbol(s): `Bi^\"*\"`\napplication domain: mass transfer\nname: BiotNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*\" = (k*l)/D_\"int\"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_\"int\"` is diffusion coefficient (ISO 80000-9) at the interface\"\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 74159) (line 1233) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-6.10 Morton number "))) (attribute-def (declaration-name "MortonNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r84)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 74284) (line 1236) (column 11) (len 1050)) (normalized "source: item 11-6.10 Morton number\nsymbol(s): `Mo`\napplication domain: generic\nname: MortonNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop\nremarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). \n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 75417) (line 1250) (column 7) (len 56)) (normalized "ISO-80000-11 item 11-6.11 Bond number, Eötvös number "))) (attribute-def (declaration-name "BondNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r85)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 75555) (line 1253) (column 11) (len 1174)) (normalized "source: item 11-6.11 Bond number, Eötvös number\nsymbol(s): `Bo`, `Eo`\napplication domain: generic\nname: BondNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble\nremarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.\n"))))) (attribute-usage) (alias (name "eötvösNumber") (target (ref r86)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 76852) (line 1269) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-6.12 Archimedes number "))) (attribute-def (declaration-name "ArchimedesNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r87)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 76985) (line 1272) (column 11) (len 881)) (normalized "source: item 11-6.12 Archimedes number\nsymbol(s): `Ar`\napplication domain: generic\nname: ArchimedesNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid\nremarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 77957) (line 1286) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-6.13 expansion number "))) (attribute-def (declaration-name "ExpansionNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r88)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 78088) (line 1289) (column 11) (len 742)) (normalized "source: item 11-6.13 expansion number\nsymbol(s): `Ex`\napplication domain: generic\nname: ExpansionNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 78919) (line 1303) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-6.14 Marangoni number "))) (attribute-def (declaration-name "MarangoniNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r89)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 79050) (line 1306) (column 11) (len 1056)) (normalized "source: item 11-6.14 Marangoni number\nsymbol(s): `Mg`, `Mar`\napplication domain: generic\nname: MarangoniNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film\nremarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 80195) (line 1320) (column 7) (len 57)) (normalized "ISO-80000-11 item 11-6.15 Lockhart-Martinelli parameter "))) (attribute-def (declaration-name "LockhartMartinelliParameterValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r90)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 80351) (line 1323) (column 11) (len 757)) (normalized "source: item 11-6.15 Lockhart-Martinelli parameter\nsymbol(s): `Lp`\napplication domain: generic\nname: LockhartMartinelliParameter (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density\nremarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 81221) (line 1337) (column 7) (len 40)) (normalized "ISO-80000-11 item 11-6.16 Bejan number "))) (attribute-def (declaration-name "BejanNumberForMassTransferValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r91)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 81359) (line 1340) (column 11) (len 776)) (normalized "source: item 11-6.16 Bejan number\nsymbol(s): `Be^\"*\"`, `Be_2`\napplication domain: mass transfer\nname: BejanNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*\" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity\"\nremarks: A similar quantity exists for heat transfer (item 11-5.9).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 82246) (line 1354) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-6.17 cavitation number "))) (attribute-def (declaration-name "CavitationNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r92)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 82379) (line 1357) (column 11) (len 826)) (normalized "source: item 11-6.17 cavitation number\nsymbol(s): `Ca`, `Cn`\napplication domain: generic\nname: CavitationNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow\nremarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 83296) (line 1371) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-6.18 absorption number "))) (attribute-def (declaration-name "AbsorptionNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r93)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 83429) (line 1374) (column 11) (len 932)) (normalized "source: item 11-6.18 absorption number\nsymbol(s): `Ab`\napplication domain: generic\nname: AbsorptionNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 84452) (line 1388) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-6.19 capillary number "))) (attribute-def (declaration-name "CapillaryNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r94)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 84583) (line 1391) (column 11) (len 654)) (normalized "source: item 11-6.19 capillary number\nsymbol(s): `Ca`\napplication domain: generic\nname: CapillaryNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 85326) (line 1405) (column 7) (len 52)) (normalized "ISO-80000-11 item 11-6.20 dynamic capillary number "))) (attribute-def (declaration-name "DynamicCapillaryNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r95)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 85472) (line 1408) (column 11) (len 844)) (normalized "source: item 11-6.20 dynamic capillary number\nsymbol(s): `Ca^\"*\"`, `Cn`\napplication domain: generic\nname: DynamicCapillaryNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*\" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)\"\nremarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 86419) (line 1422) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-7.1 Prandtl number "))) (attribute-def (declaration-name "PrandtlNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r96)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 86545) (line 1425) (column 11) (len 873)) (normalized "source: item 11-7.1 Prandtl number\nsymbol(s): `Pr`\napplication domain: generic\nname: PrandtlNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)\nremarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). \n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 87503) (line 1439) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-7.2 Schmidt number "))) (attribute-def (declaration-name "SchmidtNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r97)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 87629) (line 1442) (column 11) (len 641)) (normalized "source: item 11-7.2 Schmidt number\nsymbol(s): `Sc`\napplication domain: generic\nname: SchmidtNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)\nremarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 88355) (line 1456) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-7.3 Lewis number "))) (attribute-def (declaration-name "LewisNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r98)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 88477) (line 1459) (column 11) (len 772)) (normalized "source: item 11-7.3 Lewis number\nsymbol(s): `Le`\napplication domain: generic\nname: LewisNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)\nremarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. \n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 89330) (line 1473) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-7.4 Ohnesorge number "))) (attribute-def (declaration-name "OhnesorgeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r99)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 89460) (line 1476) (column 11) (len 917)) (normalized "source: item 11-7.4 Ohnesorge number\nsymbol(s): `Oh`\napplication domain: generic\nname: OhnesorgeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)\nremarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 90466) (line 1490) (column 7) (len 66)) (normalized "ISO-80000-11 item 11-7.5 Cauchy number, aeroelasticity parameter "))) (attribute-def (declaration-name "CauchyNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r100)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 90616) (line 1493) (column 11) (len 588)) (normalized "source: item 11-7.5 Cauchy number, aeroelasticity parameter\nsymbol(s): `Cy`\napplication domain: generic\nname: CauchyNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)\nremarks: None.\n"))))) (attribute-usage) (alias (name "aeroelasticityParameter") (target (ref r101)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 91340) (line 1509) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-7.6 Hooke number "))) (attribute-def (declaration-name "HookeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r102)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 91462) (line 1512) (column 11) (len 556)) (normalized "source: item 11-7.6 Hooke number\nsymbol(s): `Ho_2`\napplication domain: generic\nname: HookeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 92099) (line 1526) (column 7) (len 45)) (normalized "ISO-80000-11 item 11-7.7 Weissenberg number "))) (attribute-def (declaration-name "WeissenbergNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r103)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 92233) (line 1529) (column 11) (len 741)) (normalized "source: item 11-7.7 Weissenberg number\nsymbol(s): `Wi`\napplication domain: generic\nname: WeissenbergNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)\nremarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 93067) (line 1543) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-7.8 Deborah number "))) (attribute-def (declaration-name "DeborahNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r104)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 93193) (line 1546) (column 11) (len 629)) (normalized "source: item 11-7.8 Deborah number\nsymbol(s): `De`\napplication domain: generic\nname: DeborahNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)\nremarks: The stress relaxation time is sometimes called the Maxwell relaxation time.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 93907) (line 1560) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-7.9 Lorentz number "))) (attribute-def (declaration-name "LorentzNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r105)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 94033) (line 1563) (column 11) (len 712)) (normalized "source: item 11-7.9 Lorentz number\nsymbol(s): `Lo`\napplication domain: generic\nname: LorentzNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 94830) (line 1577) (column 7) (len 50)) (normalized "ISO-80000-11 item 11-7.10 compressibility number "))) (attribute-def (declaration-name "CompressibilityNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r106)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 94973) (line 1580) (column 11) (len 638)) (normalized "source: item 11-7.10 compressibility number\nsymbol(s): `Z`\napplication domain: generic\nname: CompressibilityNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 95712) (line 1594) (column 7) (len 51)) (normalized "ISO-80000-11 item 11-8.1 Reynolds magnetic number "))) (attribute-def (declaration-name "ReynoldsMagneticNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r107)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 95857) (line 1597) (column 11) (len 964)) (normalized "source: item 11-8.1 Reynolds magnetic number\nsymbol(s): `Rm`\napplication domain: generic\nname: ReynoldsMagneticNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)\nremarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 96924) (line 1611) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-8.2 Batchelor number "))) (attribute-def (declaration-name "BatchelorNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r108)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 97054) (line 1614) (column 11) (len 740)) (normalized "source: item 11-8.2 Batchelor number\nsymbol(s): `Bt`\napplication domain: generic\nname: BatchelorNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 97883) (line 1628) (column 7) (len 50)) (normalized "ISO-80000-11 item 11-8.3 Nusselt electric number "))) (attribute-def (declaration-name "NusseltElectricNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r109)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 98026) (line 1631) (column 11) (len 791)) (normalized "source: item 11-8.3 Nusselt electric number\nsymbol(s): `Ne`\napplication domain: generic\nname: NusseltElectricNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*\"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^\"*\" = D^\"+\" + D^\"-\"`, where `D^\"+\"`, `D^\"-\"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively\"\nremarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 98918) (line 1645) (column 7) (len 79)) (normalized "ISO-80000-11 item 11-8.4 Alfvén number, Mach magnetic number, Kárman number "))) (attribute-def (declaration-name "AlfvénNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r110)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 99084) (line 1648) (column 11) (len 954)) (normalized "source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number\nsymbol(s): `Al`\napplication domain: generic\nname: AlfvénNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)\nremarks: Often, the inverse of this number is wrongly used. The name \"Alfvén Mach number\" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).\n"))))) (attribute-usage) (alias (name "machMagneticNumber") (target (ref r111)) (body semicolon)) (alias (name "kármanNumber") (target (ref r112)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 100226) (line 1666) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-8.5 Hartmann number "))) (attribute-def (declaration-name "HartmannNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r113)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 100354) (line 1669) (column 11) (len 743)) (normalized "source: item 11-8.5 Hartmann number\nsymbol(s): `Ha`\napplication domain: generic\nname: HartmannNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)\nremarks: The Hartmann number represents also the ratio of magnetic force to viscous force.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 101184) (line 1683) (column 7) (len 64)) (normalized "ISO-80000-11 item 11-8.6 Cowling number, Euler magnetic number "))) (attribute-def (declaration-name "CowlingNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r114)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 101333) (line 1686) (column 11) (len 948)) (normalized "source: item 11-8.6 Cowling number, Euler magnetic number\nsymbol(s): `Co`\napplication domain: magnetism\nname: CowlingNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).\n"))))) (attribute-usage) (alias (name "eulerMagneticNumber") (target (ref r115)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 102416) (line 1702) (column 7) (len 51)) (normalized "ISO-80000-11 item 11-8.7 Stuart electrical number "))) (attribute-def (declaration-name "StuartElectricalNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r116)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 102561) (line 1705) (column 11) (len 717)) (normalized "source: item 11-8.7 Stuart electrical number\nsymbol(s): `Se`\napplication domain: generic\nname: StuartElectricalNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 103381) (line 1719) (column 7) (len 51)) (normalized "ISO-80000-11 item 11-8.8 magnetic pressure number "))) (attribute-def (declaration-name "MagneticPressureNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r117)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 103526) (line 1722) (column 11) (len 742)) (normalized "source: item 11-8.8 magnetic pressure number\nsymbol(s): `N_(mp)`\napplication domain: generic\nname: MagneticPressureNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)\nremarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 104371) (line 1736) (column 7) (len 47)) (normalized "ISO-80000-11 item 11-8.9 Chandrasekhar number "))) (attribute-def (declaration-name "ChandrasekharNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r118)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 104509) (line 1739) (column 11) (len 817)) (normalized "source: item 11-8.9 Chandrasekhar number\nsymbol(s): `Q`, `Ch`\napplication domain: generic\nname: ChandrasekharNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 105423) (line 1753) (column 7) (len 51)) (normalized "ISO-80000-11 item 11-8.10 Prandtl magnetic number "))) (attribute-def (declaration-name "PrandtlMagneticNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r119)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 105567) (line 1756) (column 11) (len 940)) (normalized "source: item 11-8.10 Prandtl magnetic number\nsymbol(s): `Pr_m`\napplication domain: generic\nname: PrandtlMagneticNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)\nremarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 106608) (line 1770) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-8.11 Roberts number "))) (attribute-def (declaration-name "RobertsNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r120)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 106735) (line 1773) (column 11) (len 779)) (normalized "source: item 11-8.11 Roberts number\nsymbol(s): `Ro`\napplication domain: generic\nname: RobertsNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)\nremarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 107599) (line 1787) (column 7) (len 41)) (normalized "ISO-80000-11 item 11-8.12 Stuart number "))) (attribute-def (declaration-name "StuartNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r121)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 107724) (line 1790) (column 11) (len 905)) (normalized "source: item 11-8.12 Stuart number\nsymbol(s): `Stw`\napplication domain: generic\nname: StuartNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)\nremarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. \n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 108712) (line 1804) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-8.13 magnetic number "))) (attribute-def (declaration-name "MagneticNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r122)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 108841) (line 1807) (column 11) (len 682)) (normalized "source: item 11-8.13 magnetic number\nsymbol(s): `N_(mg)`\napplication domain: generic\nname: MagneticNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 109610) (line 1821) (column 7) (len 52)) (normalized "ISO-80000-11 item 11-8.14 electric field parameter "))) (attribute-def (declaration-name "ElectricFieldParameterValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r123)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 109756) (line 1824) (column 11) (len 605)) (normalized "source: item 11-8.14 electric field parameter\nsymbol(s): `Ef`\napplication domain: generic\nname: ElectricFieldParameter (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 110464) (line 1838) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-8.15 Hall number "))) (attribute-def (declaration-name "HallNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r124)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 110585) (line 1841) (column 11) (len 679)) (normalized "source: item 11-8.15 Hall number\nsymbol(s): `Hc`, `CH`\napplication domain: generic\nname: HallNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)\nremarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 111343) (line 1855) (column 7) (len 44)) (normalized "ISO-80000-11 item 11-8.16 Lundquist number "))) (attribute-def (declaration-name "LundquistNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r125)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 111474) (line 1858) (column 11) (len 1154)) (normalized "source: item 11-8.16 Lundquist number\nsymbol(s): `Lu`\napplication domain: generic\nname: LundquistNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)\nremarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 112717) (line 1872) (column 7) (len 49)) (normalized "ISO-80000-11 item 11-8.17 Joule magnetic number "))) (attribute-def (declaration-name "JouleMagneticNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r126)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 112857) (line 1875) (column 11) (len 758)) (normalized "source: item 11-8.17 Joule magnetic number\nsymbol(s): `Jo_m`\napplication domain: generic\nname: JouleMagneticNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)\nremarks: This number is also called magnetic Joule number.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 113712) (line 1889) (column 7) (len 51)) (normalized "ISO-80000-11 item 11-8.18 Grashof magnetic number "))) (attribute-def (declaration-name "GrashofMagneticNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r127)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 113856) (line 1892) (column 11) (len 1044)) (normalized "source: item 11-8.18 Grashof magnetic number\nsymbol(s): `Gr_m`\napplication domain: generic\nname: GrashofMagneticNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)\nremarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 115001) (line 1906) (column 7) (len 39)) (normalized "ISO-80000-11 item 11-8.19 Naze number "))) (attribute-def (declaration-name "NazeNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r128)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 115122) (line 1909) (column 11) (len 685)) (normalized "source: item 11-8.19 Naze number\nsymbol(s): `Na`\napplication domain: generic\nname: NazeNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)\nremarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 115886) (line 1923) (column 7) (len 52)) (normalized "ISO-80000-11 item 11-8.20 Reynolds electric number "))) (attribute-def (declaration-name "ReynoldsElectricNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r129)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 116032) (line 1926) (column 11) (len 1018)) (normalized "source: item 11-8.20 Reynolds electric number\nsymbol(s): `Re_e`\napplication domain: generic\nname: ReynoldsElectricNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers\nremarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 117153) (line 1940) (column 7) (len 42)) (normalized "ISO-80000-11 item 11-8.21 Ampère number "))) (attribute-def (declaration-name "AmpèreNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r130)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 117282) (line 1943) (column 11) (len 923)) (normalized "source: item 11-8.21 Ampère number\nsymbol(s): `Am`\napplication domain: generic\nname: AmpèreNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)\nremarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 118294) (line 1957) (column 7) (len 43)) (normalized "ISO-80000-11 item 11-9.1 Arrhenius number "))) (attribute-def (declaration-name "ArrheniusNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r131)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 118424) (line 1960) (column 11) (len 673)) (normalized "source: item 11-9.1 Arrhenius number\nsymbol(s): `α`\napplication domain: generic\nname: ArrheniusNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)\nremarks: None.\n"))))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 119186) (line 1974) (column 7) (len 49)) (normalized "ISO-80000-11 item 11-9.2 Landau-Ginzburg number "))) (attribute-def (declaration-name "LandauGinzburgNumberValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r132)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 119327) (line 1977) (column 11) (len 693)) (normalized "source: item 11-9.2 Landau-Ginzburg number\nsymbol(s): `κ`\napplication domain: generic\nname: LandauGinzburgNumber (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)\nremarks: None.\n"))))) (attribute-usage))))
)
~~~
