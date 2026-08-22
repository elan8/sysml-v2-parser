# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (14-Language Extensions): 14c-Language Extensions"))
~~~
# SOURCE
~~~sysml
package '14c-Language-Extensions' {
	private import ScalarValues::*;
	
	library package FMEALibrary {
		
		abstract occurrence def Situation;
		
		abstract occurrence situations : Situation[*] nonunique;
		
		occurrence def Cause :> Situation {
			attribute occurs[0..1]: Real;
		}
		
		abstract occurrence causes : Cause[*] nonunique;
		
		occurrence def FailureMode :> Situation {
			attribute detected[0..1]: Real;
		}
		
		abstract occurrence failureModes : FailureMode[*] nonunique;
		
		occurrence def Effect :> Situation {
			attribute severity[0..1]: String;
		}
		
		abstract occurrence effects : Effect[*] nonunique;
		
		item def FMEAItem :> Situation {
			attribute RPN: Real[0..1];
			
			occurrence :>> causes;
			occurrence :>> failureModes;
			occurrence :>> effects;
		}
		
		abstract item fmeaItems : FMEAItem[*] nonunique;
				
		connection def Causation :> Occurrences::HappensBefore {
			end [*] ref cause: Situation;
			end [*] ref effect: Situation;
		}
		
		abstract connection causations : Causation[*] nonunique;
		
		requirement def FMEARequirement;
		
		abstract requirement fmeaRequirements : FMEARequirement[*] nonunique;
		
		requirement def RequirementWithSIL :> FMEARequirement {
			attribute sil: SIL;
		}
		
		enum def SIL { A; B; C; }
		
		connection def Violation {
			end [*] ref sit: Situation;
			end [*] ref req: FMEARequirement;
		}
		
		abstract connection violations : Violation[*] nonunique;
		
		abstract connection def ControllingMeasure {
			end [*] ref sit: Situation;
			end [*] ref req: FMEARequirement;
		}
		
		connection def Prevention :> ControllingMeasure;
		
		abstract connection preventions : Prevention[*] nonunique;
		
		connection def Mitigation :> ControllingMeasure;
		
		abstract connection mitigations : Mitigation[*] nonunique;
		
	}
	
	library package FMEAMetadata {
		private import Metaobjects::SemanticMetadata;
		private import FMEALibrary::*;

		enum def Status {
			Approved;
			NotApproved;
		}
		
		metadata def StatusHolder {
			status: Status;
		}
		
		metadata def <situation> SituationMetadata :> SemanticMetadata {
			:>> baseType default situations meta SysML::Usage;
		}
		
		metadata def <cause> CauseMetadata :> SituationMetadata {
			:>> baseType = causes meta SysML::Usage;
		}
		
		metadata def <failure> FailureModeMetadata :> SituationMetadata {
			:>> baseType = failureModes meta SysML::Usage;
		}
		
		metadata def <effect> EffectMetadata :> SituationMetadata {
			:>> baseType = effects meta SysML::Usage;
		}
		
		metadata def <fmea> FMEAItemMetadata :> SituationMetadata {
			:> annotatedElement : SysML::ItemDefinition;
			:> annotatedElement : SysML::ItemUsage;
			:>> baseType = fmeaItems meta SysML::Usage;
		}
		
		metadata def <causation> CausationMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
			:>> baseType = causations meta SysML::Usage;
		}
		
		metadata def <fmeaspec> FMEARequirementMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::RequirementUsage;
			:>> baseType = fmeaRequirements meta SysML::Usage;
		}
		
		metadata def <violation> ViolationMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
			:>> baseType = violations meta SysML::Usage;
		}
		
		abstract metadata def ControllingMeasureMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
		}
		
		metadata def <prevention> PreventionMetadata :> ControllingMeasureMetadata {
			:>> baseType = preventions meta SysML::Usage;
		}
		
		metadata def <mitigation> MitigationMetadata :> ControllingMeasureMetadata {
			:>> baseType = mitigations meta SysML::Usage;
		}
		
	}
	
	package FMEAUserModel {
		private import FMEALibrary::*;
		private import FMEAMetadata::*;
		
		#fmeaspec requirement req1 {
			doc /* Meter designed according to ISO00124 */
		}
		
		#fmeaspec requirement req2 {
			doc /* Device working for 1 week without the need to replace batteries */
		}
		
		#fmeaspec requirement req3: RequirementWithSIL {
			@StatusHolder { status = Status::Approved; }
			
			doc /* Alarm when battery has sank */
			
			:>> sil = SIL::A;
		}
		
		#fmea item def 'Glucose FMEA Item' {

			#prevention connect 'battery depleted' to req1;
			
			#cause occurrence 'battery depleted' {
				:>> occurs = 0.005;
			}
			
			#causation connect 'battery depleted' to 'battery cannot be charged';
			
			#failure occurrence 'battery cannot be charged' {
				:>> detected = 0.013;
			}
			
			#causation connect 'battery cannot be charged' to 'glucose level undetected';
			
			#effect occurrence 'glucose level undetected';
			
			#causation connect 'glucose level undetected' to 'therapy delay';
			
			#effect occurrence 'therapy delay' {
				:>> severity = "High";
			}

		}
		
		#violation connect 'Glucose Meter in Use' to req2;
		#mitigation connect 'Glucose Meter in Use' to req3;
			
		#fmea item 'Glucose Meter in Use' : 'Glucose FMEA Item' {
			
			part 'glucose meter' {
				event 'glucose level undetected'[*];
				part battery {
					event 'battery depleted'[*];
					event 'battery cannot be charged'[*];
				}
				part pump;
				part reservoir;
			}
			
			part patient {
				event 'therapy delay'[*];
			}
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14c_language_extensions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '14c-Language-Extensions' {
    private import ScalarValues::*;
    library package FMEALibrary {
        abstract occurrence def Situation;
        abstract occurrence situations : Situation[*];
        occurrence def Cause :> Situation {
            attribute occurs : Real[0..1];
        }
        abstract occurrence causes : Cause[*];
        occurrence def FailureMode :> Situation {
            attribute detected : Real[0..1];
        }
        abstract occurrence failureModes : FailureMode[*];
        occurrence def Effect :> Situation {
            attribute severity : String[0..1];
        }
        abstract occurrence effects : Effect[*];
        item def FMEAItem :> Situation {
            attribute RPN : Real[0..1];
            occurrence :>> causes;
            occurrence :>> failureModes;
            occurrence :>> effects;
        }
        abstract item fmeaItems : FMEAItem[*] nonunique;
        connection def Causation :> Occurrences::HappensBefore {
            end ref cause : Situation[*];
            end ref effect : Situation[*];
        }
        abstract connection def causations : Causation;
        requirement def FMEARequirement;
        abstract requirement fmeaRequirements : FMEARequirement[*];
        requirement def RequirementWithSIL :> FMEARequirement {
            attribute sil : SIL;
        }
        enum def SIL {
            A;
            B;
            C;
        }
        connection def Violation {
            end ref sit : Situation[*];
            end ref req : FMEARequirement[*];
        }
        abstract connection def violations : Violation;
        abstract connection def ControllingMeasure {
            end ref sit : Situation[*];
            end ref req : FMEARequirement[*];
        }
        connection def Prevention :> ControllingMeasure;
        abstract connection def preventions : Prevention;
        connection def Mitigation :> ControllingMeasure;
        abstract connection def mitigations : Mitigation;
    }
    library package FMEAMetadata {
        private import Metaobjects::SemanticMetadata;
        private import FMEALibrary::*;
        enum def Status {
            Approved;
            NotApproved;
        }
        metadata def StatusHolder {
            attribute status : Status;
        }
        metadata def <situation> SituationMetadata :> SemanticMetadata {
            attribute :>> baseType default situations meta SysML::Usage;
        }
        metadata def <cause> CauseMetadata :> SituationMetadata {
            attribute :>> baseType = causes meta SysML::Usage;
        }
        metadata def <failure> FailureModeMetadata :> SituationMetadata {
            attribute :>> baseType = failureModes meta SysML::Usage;
        }
        metadata def <effect> EffectMetadata :> SituationMetadata {
            attribute :>> baseType = effects meta SysML::Usage;
        }
        metadata def <fmea> FMEAItemMetadata :> SituationMetadata {
            attribute :> annotatedElement : SysML::ItemDefinition;
            attribute :> annotatedElement : SysML::ItemUsage;
            attribute :>> baseType = fmeaItems meta SysML::Usage;
        }
        metadata def <causation> CausationMetadata :> SemanticMetadata {
            attribute :>> annotatedElement : SysML::ConnectionUsage;
            attribute :>> baseType = causations meta SysML::Usage;
        }
        metadata def <fmeaspec> FMEARequirementMetadata :> SemanticMetadata {
            attribute :>> annotatedElement : SysML::RequirementUsage;
            attribute :>> baseType = fmeaRequirements meta SysML::Usage;
        }
        metadata def <violation> ViolationMetadata :> SemanticMetadata {
            attribute :>> annotatedElement : SysML::ConnectionUsage;
            attribute :>> baseType = violations meta SysML::Usage;
        }
        abstract metadata def ControllingMeasureMetadata :> SemanticMetadata {
            attribute :>> annotatedElement : SysML::ConnectionUsage;
        }
        metadata def <prevention> PreventionMetadata :> ControllingMeasureMetadata {
            attribute :>> baseType = preventions meta SysML::Usage;
        }
        metadata def <mitigation> MitigationMetadata :> ControllingMeasureMetadata {
            attribute :>> baseType = mitigations meta SysML::Usage;
        }
    }
    package FMEAUserModel {
        private import FMEALibrary::*;
        private import FMEAMetadata::*;
        #fmeaspec
        requirement req1 {
            doc
            /* Meter designed according to ISO00124 */
        }
        #fmeaspec
        requirement req2 {
            doc
            /* Device working for 1 week without the need to replace batteries */
        }
        #fmeaspec
        requirement req3 : RequirementWithSIL {
            @StatusHolder {
                status = Status::Approved;
            }
            doc
            /* Alarm when battery has sank */
            :>> sil = SIL::A;
        }
        #fmea
        item def 'Glucose FMEA Item' {
            #prevention
            connect 'battery depleted' to req1;
            #cause occurrence 'battery depleted' {
                attribute :>> occurs = 0.005;
            }
            #causation
            connect 'battery depleted' to 'battery cannot be charged';
            #failure occurrence 'battery cannot be charged' {
                attribute :>> detected = 0.013;
            }
            #causation
            connect 'battery cannot be charged' to 'glucose level undetected';
            #effect occurrence 'glucose level undetected';
            #causation
            connect 'glucose level undetected' to 'therapy delay';
            #effect occurrence 'therapy delay' {
                attribute :>> severity = "High";
            }
        }
        #violation
        connect 'Glucose Meter in Use' to req2;
        #mitigation
        connect 'Glucose Meter in Use' to req3;
        #fmea item 'Glucose Meter in Use' : 'Glucose FMEA Item' {
            part 'glucose meter' {
                event 'glucose level undetected'[*];
                part battery {
                    event 'battery depleted'[*];
                    event 'battery cannot be charged'[*];
                }
                part pump;
                part reservoir;
            }
            part patient {
                event 'therapy delay'[*];
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 52) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 653) (line 28) (column 24) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 653) (line 28) (column 24) (len 9)))))
    (reference r2 (scope relative) (span (offset 683) (line 29) (column 19) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 683) (line 29) (column 19) (len 4)))))
    (reference r3 (scope relative) (span (offset 819) (line 36) (column 29) (len 8)) (segments (segment 0 (token "FMEAItem") (name "FMEAItem") (separator none) (span (offset 819) (line 36) (column 29) (len 8)))))
    (reference r4 (scope relative) (span (offset 877) (line 38) (column 31) (len 26)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 877) (line 38) (column 31) (len 11))) (segment 1 (token "HappensBefore") (name "HappensBefore") (separator colon-colon) (span (offset 890) (line 38) (column 44) (len 13)))))
    (reference r5 (scope relative) (span (offset 928) (line 39) (column 23) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 928) (line 39) (column 23) (len 9)))))
    (reference r6 (scope relative) (span (offset 962) (line 40) (column 24) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 962) (line 40) (column 24) (len 9)))))
    (reference r7 (scope relative) (span (offset 1015) (line 43) (column 36) (len 9)) (segments (segment 0 (token "Causation") (name "Causation") (separator none) (span (offset 1015) (line 43) (column 36) (len 9)))))
    (reference r8 (scope relative) (span (offset 1323) (line 56) (column 21) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 1323) (line 56) (column 21) (len 9)))))
    (reference r9 (scope relative) (span (offset 1354) (line 57) (column 21) (len 15)) (segments (segment 0 (token "FMEARequirement") (name "FMEARequirement") (separator none) (span (offset 1354) (line 57) (column 21) (len 15)))))
    (reference r10 (scope relative) (span (offset 1413) (line 60) (column 36) (len 9)) (segments (segment 0 (token "Violation") (name "Violation") (separator none) (span (offset 1413) (line 60) (column 36) (len 9)))))
    (reference r11 (scope relative) (span (offset 1507) (line 63) (column 21) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 1507) (line 63) (column 21) (len 9)))))
    (reference r12 (scope relative) (span (offset 1538) (line 64) (column 21) (len 15)) (segments (segment 0 (token "FMEARequirement") (name "FMEARequirement") (separator none) (span (offset 1538) (line 64) (column 21) (len 15)))))
    (reference r13 (scope relative) (span (offset 1593) (line 67) (column 32) (len 18)) (segments (segment 0 (token "ControllingMeasure") (name "ControllingMeasure") (separator none) (span (offset 1593) (line 67) (column 32) (len 18)))))
    (reference r14 (scope relative) (span (offset 1652) (line 69) (column 37) (len 10)) (segments (segment 0 (token "Prevention") (name "Prevention") (separator none) (span (offset 1652) (line 69) (column 37) (len 10)))))
    (reference r15 (scope relative) (span (offset 1711) (line 71) (column 32) (len 18)) (segments (segment 0 (token "ControllingMeasure") (name "ControllingMeasure") (separator none) (span (offset 1711) (line 71) (column 32) (len 18)))))
    (reference r16 (scope relative) (span (offset 1770) (line 73) (column 37) (len 10)) (segments (segment 0 (token "Mitigation") (name "Mitigation") (separator none) (span (offset 1770) (line 73) (column 37) (len 10)))))
    (reference r17 (scope relative) (span (offset 1852) (line 78) (column 18) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 1852) (line 78) (column 18) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 1865) (line 78) (column 31) (len 16)))))
    (reference r18 (scope relative) (span (offset 1900) (line 79) (column 18) (len 11)) (segments (segment 0 (token "FMEALibrary") (name "FMEALibrary") (separator none) (span (offset 1900) (line 79) (column 18) (len 11)))))
    (reference r19 (scope relative) (span (offset 2014) (line 87) (column 12) (len 6)) (segments (segment 0 (token "Status") (name "Status") (separator none) (span (offset 2014) (line 87) (column 12) (len 6)))))
    (reference r20 (scope relative) (span (offset 2077) (line 90) (column 49) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2077) (line 90) (column 49) (len 16)))))
    (reference r21 (scope relative) (span (offset 2103) (line 91) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2103) (line 91) (column 8) (len 8)))))
    (reference r22 (scope relative) (span (offset 2120) (line 91) (column 25) (len 10)) (segments (segment 0 (token "situations") (name "situations") (separator none) (span (offset 2120) (line 91) (column 25) (len 10)))))
    (reference r23 (scope relative) (span (offset 2136) (line 91) (column 41) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2136) (line 91) (column 41) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2143) (line 91) (column 48) (len 5)))))
    (reference r24 (scope relative) (span (offset 2197) (line 94) (column 41) (len 17)) (segments (segment 0 (token "SituationMetadata") (name "SituationMetadata") (separator none) (span (offset 2197) (line 94) (column 41) (len 17)))))
    (reference r25 (scope relative) (span (offset 2224) (line 95) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2224) (line 95) (column 8) (len 8)))))
    (reference r26 (scope relative) (span (offset 2235) (line 95) (column 19) (len 6)) (segments (segment 0 (token "causes") (name "causes") (separator none) (span (offset 2235) (line 95) (column 19) (len 6)))))
    (reference r27 (scope relative) (span (offset 2247) (line 95) (column 31) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2247) (line 95) (column 31) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2254) (line 95) (column 38) (len 5)))))
    (reference r28 (scope relative) (span (offset 2316) (line 98) (column 49) (len 17)) (segments (segment 0 (token "SituationMetadata") (name "SituationMetadata") (separator none) (span (offset 2316) (line 98) (column 49) (len 17)))))
    (reference r29 (scope relative) (span (offset 2343) (line 99) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2343) (line 99) (column 8) (len 8)))))
    (reference r30 (scope relative) (span (offset 2354) (line 99) (column 19) (len 12)) (segments (segment 0 (token "failureModes") (name "failureModes") (separator none) (span (offset 2354) (line 99) (column 19) (len 12)))))
    (reference r31 (scope relative) (span (offset 2372) (line 99) (column 37) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2372) (line 99) (column 37) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2379) (line 99) (column 44) (len 5)))))
    (reference r32 (scope relative) (span (offset 2435) (line 102) (column 43) (len 17)) (segments (segment 0 (token "SituationMetadata") (name "SituationMetadata") (separator none) (span (offset 2435) (line 102) (column 43) (len 17)))))
    (reference r33 (scope relative) (span (offset 2462) (line 103) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2462) (line 103) (column 8) (len 8)))))
    (reference r34 (scope relative) (span (offset 2473) (line 103) (column 19) (len 7)) (segments (segment 0 (token "effects") (name "effects") (separator none) (span (offset 2473) (line 103) (column 19) (len 7)))))
    (reference r35 (scope relative) (span (offset 2486) (line 103) (column 32) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2486) (line 103) (column 32) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2493) (line 103) (column 39) (len 5)))))
    (reference r36 (scope relative) (span (offset 2549) (line 106) (column 43) (len 17)) (segments (segment 0 (token "SituationMetadata") (name "SituationMetadata") (separator none) (span (offset 2549) (line 106) (column 43) (len 17)))))
    (reference r37 (scope relative) (span (offset 2594) (line 107) (column 26) (len 21)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2594) (line 107) (column 26) (len 5))) (segment 1 (token "ItemDefinition") (name "ItemDefinition") (separator colon-colon) (span (offset 2601) (line 107) (column 33) (len 14)))))
    (reference r38 (scope relative) (span (offset 2575) (line 107) (column 7) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 2575) (line 107) (column 7) (len 16)))))
    (reference r39 (scope relative) (span (offset 2642) (line 108) (column 26) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2642) (line 108) (column 26) (len 5))) (segment 1 (token "ItemUsage") (name "ItemUsage") (separator colon-colon) (span (offset 2649) (line 108) (column 33) (len 9)))))
    (reference r40 (scope relative) (span (offset 2623) (line 108) (column 7) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 2623) (line 108) (column 7) (len 16)))))
    (reference r41 (scope relative) (span (offset 2667) (line 109) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2667) (line 109) (column 8) (len 8)))))
    (reference r42 (scope relative) (span (offset 2678) (line 109) (column 19) (len 9)) (segments (segment 0 (token "fmeaItems") (name "fmeaItems") (separator none) (span (offset 2678) (line 109) (column 19) (len 9)))))
    (reference r43 (scope relative) (span (offset 2693) (line 109) (column 34) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2693) (line 109) (column 34) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2700) (line 109) (column 41) (len 5)))))
    (reference r44 (scope relative) (span (offset 2762) (line 112) (column 49) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2762) (line 112) (column 49) (len 16)))))
    (reference r45 (scope relative) (span (offset 2807) (line 113) (column 27) (len 22)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2807) (line 113) (column 27) (len 5))) (segment 1 (token "ConnectionUsage") (name "ConnectionUsage") (separator colon-colon) (span (offset 2814) (line 113) (column 34) (len 15)))))
    (reference r46 (scope relative) (span (offset 2788) (line 113) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 2788) (line 113) (column 8) (len 16)))))
    (reference r47 (scope relative) (span (offset 2838) (line 114) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2838) (line 114) (column 8) (len 8)))))
    (reference r48 (scope relative) (span (offset 2849) (line 114) (column 19) (len 10)) (segments (segment 0 (token "causations") (name "causations") (separator none) (span (offset 2849) (line 114) (column 19) (len 10)))))
    (reference r49 (scope relative) (span (offset 2865) (line 114) (column 35) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2865) (line 114) (column 35) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 2872) (line 114) (column 42) (len 5)))))
    (reference r50 (scope relative) (span (offset 2939) (line 117) (column 54) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2939) (line 117) (column 54) (len 16)))))
    (reference r51 (scope relative) (span (offset 2984) (line 118) (column 27) (len 23)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2984) (line 118) (column 27) (len 5))) (segment 1 (token "RequirementUsage") (name "RequirementUsage") (separator colon-colon) (span (offset 2991) (line 118) (column 34) (len 16)))))
    (reference r52 (scope relative) (span (offset 2965) (line 118) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 2965) (line 118) (column 8) (len 16)))))
    (reference r53 (scope relative) (span (offset 3016) (line 119) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 3016) (line 119) (column 8) (len 8)))))
    (reference r54 (scope relative) (span (offset 3027) (line 119) (column 19) (len 16)) (segments (segment 0 (token "fmeaRequirements") (name "fmeaRequirements") (separator none) (span (offset 3027) (line 119) (column 19) (len 16)))))
    (reference r55 (scope relative) (span (offset 3049) (line 119) (column 41) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3049) (line 119) (column 41) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 3056) (line 119) (column 48) (len 5)))))
    (reference r56 (scope relative) (span (offset 3118) (line 122) (column 49) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 3118) (line 122) (column 49) (len 16)))))
    (reference r57 (scope relative) (span (offset 3163) (line 123) (column 27) (len 22)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3163) (line 123) (column 27) (len 5))) (segment 1 (token "ConnectionUsage") (name "ConnectionUsage") (separator colon-colon) (span (offset 3170) (line 123) (column 34) (len 15)))))
    (reference r58 (scope relative) (span (offset 3144) (line 123) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 3144) (line 123) (column 8) (len 16)))))
    (reference r59 (scope relative) (span (offset 3194) (line 124) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 3194) (line 124) (column 8) (len 8)))))
    (reference r60 (scope relative) (span (offset 3205) (line 124) (column 19) (len 10)) (segments (segment 0 (token "violations") (name "violations") (separator none) (span (offset 3205) (line 124) (column 19) (len 10)))))
    (reference r61 (scope relative) (span (offset 3221) (line 124) (column 35) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3221) (line 124) (column 35) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 3228) (line 124) (column 42) (len 5)))))
    (reference r62 (scope relative) (span (offset 3296) (line 127) (column 55) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 3296) (line 127) (column 55) (len 16)))))
    (reference r63 (scope relative) (span (offset 3341) (line 128) (column 27) (len 22)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3341) (line 128) (column 27) (len 5))) (segment 1 (token "ConnectionUsage") (name "ConnectionUsage") (separator colon-colon) (span (offset 3348) (line 128) (column 34) (len 15)))))
    (reference r64 (scope relative) (span (offset 3322) (line 128) (column 8) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 3322) (line 128) (column 8) (len 16)))))
    (reference r65 (scope relative) (span (offset 3422) (line 131) (column 51) (len 26)) (segments (segment 0 (token "ControllingMeasureMetadata") (name "ControllingMeasureMetadata") (separator none) (span (offset 3422) (line 131) (column 51) (len 26)))))
    (reference r66 (scope relative) (span (offset 3458) (line 132) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 3458) (line 132) (column 8) (len 8)))))
    (reference r67 (scope relative) (span (offset 3469) (line 132) (column 19) (len 11)) (segments (segment 0 (token "preventions") (name "preventions") (separator none) (span (offset 3469) (line 132) (column 19) (len 11)))))
    (reference r68 (scope relative) (span (offset 3486) (line 132) (column 36) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3486) (line 132) (column 36) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 3493) (line 132) (column 43) (len 5)))))
    (reference r69 (scope relative) (span (offset 3557) (line 135) (column 51) (len 26)) (segments (segment 0 (token "ControllingMeasureMetadata") (name "ControllingMeasureMetadata") (separator none) (span (offset 3557) (line 135) (column 51) (len 26)))))
    (reference r70 (scope relative) (span (offset 3593) (line 136) (column 8) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 3593) (line 136) (column 8) (len 8)))))
    (reference r71 (scope relative) (span (offset 3604) (line 136) (column 19) (len 11)) (segments (segment 0 (token "mitigations") (name "mitigations") (separator none) (span (offset 3604) (line 136) (column 19) (len 11)))))
    (reference r72 (scope relative) (span (offset 3621) (line 136) (column 36) (len 12)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3621) (line 136) (column 36) (len 5))) (segment 1 (token "Usage") (name "Usage") (separator colon-colon) (span (offset 3628) (line 136) (column 43) (len 5)))))
    (reference r73 (scope relative) (span (offset 3689) (line 142) (column 18) (len 11)) (segments (segment 0 (token "FMEALibrary") (name "FMEALibrary") (separator none) (span (offset 3689) (line 142) (column 18) (len 11)))))
    (reference r74 (scope relative) (span (offset 3722) (line 143) (column 18) (len 12)) (segments (segment 0 (token "FMEAMetadata") (name "FMEAMetadata") (separator none) (span (offset 3722) (line 143) (column 18) (len 12)))))
    (reference r75 (scope relative) (span (offset 3745) (line 145) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3745) (line 145) (column 4) (len 8)))))
    (reference r76 (scope relative) (span (offset 3833) (line 149) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3833) (line 149) (column 4) (len 8)))))
    (reference r77 (scope relative) (span (offset 3948) (line 153) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3948) (line 153) (column 4) (len 8)))))
    (reference r78 (scope relative) (span (offset 4124) (line 161) (column 4) (len 4)) (segments (segment 0 (token "fmea") (name "fmea") (separator none) (span (offset 4124) (line 161) (column 4) (len 4)))))
    (reference r79 (scope relative) (span (offset 4165) (line 163) (column 5) (len 10)) (segments (segment 0 (token "prevention") (name "prevention") (separator none) (span (offset 4165) (line 163) (column 5) (len 10)))))
    (reference r80 (scope relative) (span (offset 4220) (line 165) (column 5) (len 5)) (segments (segment 0 (token "cause") (name "cause") (separator none) (span (offset 4220) (line 165) (column 5) (len 5)))))
    (reference r81 (scope relative) (span (offset 4266) (line 166) (column 9) (len 6)) (segments (segment 0 (token "occurs") (name "occurs") (separator none) (span (offset 4266) (line 166) (column 9) (len 6)))))
    (reference r82 (scope relative) (span (offset 4295) (line 169) (column 5) (len 9)) (segments (segment 0 (token "causation") (name "causation") (separator none) (span (offset 4295) (line 169) (column 5) (len 9)))))
    (reference r83 (scope relative) (span (offset 4372) (line 171) (column 5) (len 7)) (segments (segment 0 (token "failure") (name "failure") (separator none) (span (offset 4372) (line 171) (column 5) (len 7)))))
    (reference r84 (scope relative) (span (offset 4429) (line 172) (column 9) (len 8)) (segments (segment 0 (token "detected") (name "detected") (separator none) (span (offset 4429) (line 172) (column 9) (len 8)))))
    (reference r85 (scope relative) (span (offset 4460) (line 175) (column 5) (len 9)) (segments (segment 0 (token "causation") (name "causation") (separator none) (span (offset 4460) (line 175) (column 5) (len 9)))))
    (reference r86 (scope relative) (span (offset 4545) (line 177) (column 5) (len 6)) (segments (segment 0 (token "effect") (name "effect") (separator none) (span (offset 4545) (line 177) (column 5) (len 6)))))
    (reference r87 (scope relative) (span (offset 4599) (line 179) (column 5) (len 9)) (segments (segment 0 (token "causation") (name "causation") (separator none) (span (offset 4599) (line 179) (column 5) (len 9)))))
    (reference r88 (scope relative) (span (offset 4672) (line 181) (column 5) (len 6)) (segments (segment 0 (token "effect") (name "effect") (separator none) (span (offset 4672) (line 181) (column 5) (len 6)))))
    (reference r89 (scope relative) (span (offset 4716) (line 182) (column 9) (len 8)) (segments (segment 0 (token "severity") (name "severity") (separator none) (span (offset 4716) (line 182) (column 9) (len 8)))))
    (reference r90 (scope relative) (span (offset 4751) (line 187) (column 4) (len 9)) (segments (segment 0 (token "violation") (name "violation") (separator none) (span (offset 4751) (line 187) (column 4) (len 9)))))
    (reference r91 (scope relative) (span (offset 4769) (line 187) (column 22) (len 22)) (segments (segment 0 (token "'Glucose Meter in Use'") (name "Glucose Meter in Use") (separator none) (span (offset 4769) (line 187) (column 22) (len 22)))))
    (reference r92 (scope relative) (span (offset 4795) (line 187) (column 48) (len 4)) (segments (segment 0 (token "req2") (name "req2") (separator none) (span (offset 4795) (line 187) (column 48) (len 4)))))
    (reference r93 (scope relative) (span (offset 4804) (line 188) (column 4) (len 10)) (segments (segment 0 (token "mitigation") (name "mitigation") (separator none) (span (offset 4804) (line 188) (column 4) (len 10)))))
    (reference r94 (scope relative) (span (offset 4823) (line 188) (column 23) (len 22)) (segments (segment 0 (token "'Glucose Meter in Use'") (name "Glucose Meter in Use") (separator none) (span (offset 4823) (line 188) (column 23) (len 22)))))
    (reference r95 (scope relative) (span (offset 4849) (line 188) (column 49) (len 4)) (segments (segment 0 (token "req3") (name "req3") (separator none) (span (offset 4849) (line 188) (column 49) (len 4)))))
    (reference r96 (scope relative) (span (offset 4862) (line 190) (column 4) (len 4)) (segments (segment 0 (token "fmea") (name "fmea") (separator none) (span (offset 4862) (line 190) (column 4) (len 4)))))
    (reference r97 (scope relative) (span (offset 4897) (line 190) (column 39) (len 19)) (segments (segment 0 (token "'Glucose FMEA Item'") (name "Glucose FMEA Item") (separator none) (span (offset 4897) (line 190) (column 39) (len 19)))))
    (reference r98 (scope relative) (span (offset 4959) (line 193) (column 11) (len 26)) (segments (segment 0 (token "'glucose level undetected'") (name "glucose level undetected") (separator none) (span (offset 4959) (line 193) (column 11) (len 26)))))
    (reference r99 (scope relative) (span (offset 5020) (line 195) (column 12) (len 18)) (segments (segment 0 (token "'battery depleted'") (name "battery depleted") (separator none) (span (offset 5020) (line 195) (column 12) (len 18)))))
    (reference r100 (scope relative) (span (offset 5054) (line 196) (column 12) (len 27)) (segments (segment 0 (token "'battery cannot be charged'") (name "battery cannot be charged") (separator none) (span (offset 5054) (line 196) (column 12) (len 27)))))
    (reference r101 (scope relative) (span (offset 5164) (line 203) (column 11) (len 15)) (segments (segment 0 (token "'therapy delay'") (name "therapy delay") (separator none) (span (offset 5164) (line 203) (column 11) (len 15)))))
  )
  (root (package (name "14c-Language-Extensions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 64) (line 2) (column 29) (len 3))) (separator (span (offset 64) (line 2) (column 29) (len 2))) (marker (span (offset 66) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (library-package (name "FMEALibrary") (standard false) (body brace (occurrence-def (modifiers (abstract (span (offset 107) (line 6) (column 3) (len 8))))) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "situations") (short-name none) (target none) (body semicolon)) (occurrence-def (modifiers)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "causes") (short-name none) (target none) (body semicolon)) (occurrence-def (modifiers)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "failureModes") (short-name none) (target none) (body semicolon)) (occurrence-def (modifiers)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "effects") (short-name none) (target none) (body semicolon)) (item-def (name "FMEAItem") (modifiers) (individual false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (attribute-usage (declaration-name "RPN") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fmeaItems") (short-name none) (type (ref r3)) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets none) (redefines none) (value none) (body semicolon)) (connection-def (name "Causation") (modifiers) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 917) (line 39) (column 12) (len 3)))) (short-name none) (identity (declaration (name "cause") (span (offset 921) (line 39) (column 16) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 950) (line 40) (column 12) (len 3)))) (short-name none) (identity (declaration (name "effect") (span (offset 954) (line 40) (column 16) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "causations") (modifiers (abstract (span (offset 982) (line 43) (column 3) (len 8)))) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (body semicolon)) (requirement-def (name "FMEARequirement") (modifiers) (body semicolon)) (requirement-usage (name "fmeaRequirements") (multiplicity (lower unbounded) (upper unbounded))) (requirement-def (name "RequirementWithSIL") (modifiers) (body brace (attribute-usage))) (enum-def (name "SIL") (body brace (enum-value (enum-keyword none) (visibility none) (name "A") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1260) (line 53) (column 18) (len 2))) (enum-value (enum-keyword none) (visibility none) (name "B") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1263) (line 53) (column 21) (len 2))) (enum-value (enum-keyword none) (visibility none) (name "C") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1266) (line 53) (column 24) (len 2))))) (connection-def (name "Violation") (modifiers) (role ordinary) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 1314) (line 56) (column 12) (len 3)))) (short-name none) (identity (declaration (name "sit") (span (offset 1318) (line 56) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 1345) (line 57) (column 12) (len 3)))) (short-name none) (identity (declaration (name "req") (span (offset 1349) (line 57) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "violations") (modifiers (abstract (span (offset 1380) (line 60) (column 3) (len 8)))) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (body semicolon)) (connection-def (name "ControllingMeasure") (modifiers (abstract (span (offset 1442) (line 62) (column 3) (len 8)))) (role ordinary) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 1498) (line 63) (column 12) (len 3)))) (short-name none) (identity (declaration (name "sit") (span (offset 1502) (line 63) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 1529) (line 64) (column 12) (len 3)))) (short-name none) (identity (declaration (name "req") (span (offset 1533) (line 64) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "Prevention") (modifiers) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (body semicolon)) (connection-def (name "preventions") (modifiers (abstract (span (offset 1618) (line 69) (column 3) (len 8)))) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (body semicolon)) (connection-def (name "Mitigation") (modifiers) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r15)))) (body semicolon)) (connection-def (name "mitigations") (modifiers (abstract (span (offset 1736) (line 73) (column 3) (len 8)))) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (body semicolon)))) (library-package (name "FMEAMetadata") (standard false) (body brace (import (target (span (span (offset 1852) (line 78) (column 18) (len 29))) (all none) (ref r17) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1900) (line 79) (column 18) (len 14))) (all none) (ref r18) (shape (namespace (wildcard-suffix (span (span (offset 1911) (line 79) (column 29) (len 3))) (separator (span (offset 1911) (line 79) (column 29) (len 2))) (marker (span (offset 1913) (line 79) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (enum-def (name "Status") (body brace (enum-value (enum-keyword none) (visibility none) (name "Approved") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1940) (line 82) (column 4) (len 9))) (enum-value (enum-keyword none) (visibility none) (name "NotApproved") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 1953) (line 83) (column 4) (len 12))))) (metadata-def (name "StatusHolder") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name "status") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "SituationMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 2120) (line 91) (column 25) (len 28)) (meta-cast (base (expression (span (offset 2120) (line 91) (column 25) (len 10)) (ref r22))) (metaclass (ref r23))))))) (body semicolon)))) (metadata-def (name "CauseMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2235) (line 95) (column 19) (len 24)) (meta-cast (base (expression (span (offset 2235) (line 95) (column 19) (len 6)) (ref r26))) (metaclass (ref r27))))))) (body semicolon)))) (metadata-def (name "FailureModeMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2354) (line 99) (column 19) (len 30)) (meta-cast (base (expression (span (offset 2354) (line 99) (column 19) (len 12)) (ref r30))) (metaclass (ref r31))))))) (body semicolon)))) (metadata-def (name "EffectMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r32)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2473) (line 103) (column 19) (len 25)) (meta-cast (base (expression (span (offset 2473) (line 103) (column 19) (len 7)) (ref r34))) (metaclass (ref r35))))))) (body semicolon)))) (metadata-def (name "FMEAItemMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r36)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r38)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r40)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2678) (line 109) (column 19) (len 27)) (meta-cast (base (expression (span (offset 2678) (line 109) (column 19) (len 9)) (ref r42))) (metaclass (ref r43))))))) (body semicolon)))) (metadata-def (name "CausationMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r44)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r46)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2849) (line 114) (column 19) (len 28)) (meta-cast (base (expression (span (offset 2849) (line 114) (column 19) (len 10)) (ref r48))) (metaclass (ref r49))))))) (body semicolon)))) (metadata-def (name "FMEARequirementMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r50)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r53)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3027) (line 119) (column 19) (len 34)) (meta-cast (base (expression (span (offset 3027) (line 119) (column 19) (len 16)) (ref r54))) (metaclass (ref r55))))))) (body semicolon)))) (metadata-def (name "ViolationMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r56)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3205) (line 124) (column 19) (len 28)) (meta-cast (base (expression (span (offset 3205) (line 124) (column 19) (len 10)) (ref r60))) (metaclass (ref r61))))))) (body semicolon)))) (metadata-def (name "ControllingMeasureMetadata") (abstract true) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r62)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r63)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r64)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "PreventionMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r65)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3469) (line 132) (column 19) (len 29)) (meta-cast (base (expression (span (offset 3469) (line 132) (column 19) (len 11)) (ref r67))) (metaclass (ref r68))))))) (body semicolon)))) (metadata-def (name "MitigationMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r69)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r70)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3604) (line 136) (column 19) (len 29)) (meta-cast (base (expression (span (offset 3604) (line 136) (column 19) (len 11)) (ref r71))) (metaclass (ref r72))))))) (body semicolon)))))) (package (name "FMEAUserModel") (body brace (import (target (span (span (offset 3689) (line 142) (column 18) (len 14))) (all none) (ref r73) (shape (namespace (wildcard-suffix (span (span (offset 3700) (line 142) (column 29) (len 3))) (separator (span (offset 3700) (line 142) (column 29) (len 2))) (marker (span (offset 3702) (line 142) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 3722) (line 143) (column 18) (len 15))) (all none) (ref r74) (shape (namespace (wildcard-suffix (span (span (offset 3734) (line 143) (column 30) (len 3))) (separator (span (offset 3734) (line 143) (column 30) (len 2))) (marker (span (offset 3736) (line 143) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-keyword-usage (type (ref r75)) (body none)) (requirement-usage (name "req1") (multiplicity none)) (metadata-keyword-usage (type (ref r76)) (body none)) (requirement-usage (name "req2") (multiplicity none)) (metadata-keyword-usage (type (ref r77)) (body none)) (requirement-usage (name "req3") (multiplicity none)) (metadata-keyword-usage (type (ref r78)) (body none)) (item-def (name "Glucose FMEA Item") (modifiers) (individual false) (specializes none) (body brace (metadata-keyword-usage (type (ref r79)) (body none)) (connect) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r80))) (declaration "battery depleted") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r81)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4275) (line 166) (column 18) (len 5)) (real "0.005"))))) (body semicolon)))) (metadata-keyword-usage (type (ref r82)) (body none)) (connect) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r83))) (declaration "battery cannot be charged") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r84)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4440) (line 172) (column 20) (len 5)) (real "0.013"))))) (body semicolon)))) (metadata-keyword-usage (type (ref r85)) (body none)) (connect) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r86))) (declaration "glucose level undetected") (short-name none) (target none) (body semicolon)) (metadata-keyword-usage (type (ref r87)) (body none)) (connect) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r88))) (declaration "therapy delay") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4727) (line 182) (column 20) (len 6)) (string "High"))))) (body semicolon)))))) (metadata-keyword-usage (type (ref r90)) (body none)) (connect (from (expression (span (offset 4769) (line 187) (column 22) (len 22)) (ref r91))) (to (expression (span (offset 4795) (line 187) (column 48) (len 4)) (ref r92))) (body semicolon) (subsets none) (redefines none)) (metadata-keyword-usage (type (ref r93)) (body none)) (connect (from (expression (span (offset 4823) (line 188) (column 23) (len 22)) (ref r94))) (to (expression (span (offset 4849) (line 188) (column 49) (len 4)) (ref r95))) (body semicolon) (subsets none) (redefines none)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r96))) (declaration "Glucose Meter in Use") (short-name none) (type (ref r97)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "glucose meter") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r98)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "battery") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r99)) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r100)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "reservoir") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "patient") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r101)) (body semicolon)))))))))))
)
~~~
