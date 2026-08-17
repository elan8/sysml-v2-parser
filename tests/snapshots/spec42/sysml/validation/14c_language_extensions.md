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
            occurrence  :>> causes;
            occurrence  :>> failureModes;
            occurrence  :>> effects;
        }
        abstract item fmeaItems : FMEAItem[*] nonunique;
        connection def Causation :> Occurrences::HappensBefore {
            end cause : Situation[*];
            end effect : Situation[*];
        }
        connection def causations : Causation;
        requirement def FMEARequirement;
        abstract requirement fmeaRequirements : FMEARequirement;
        requirement def RequirementWithSIL :> FMEARequirement {
            attribute sil : SIL;
        }
        enum def SIL {
            A;
            B;
            C;
        }
        connection def Violation {
            end sit : Situation[*];
            end req : FMEARequirement[*];
        }
        connection def violations : Violation;
        connection def ControllingMeasure {
            end sit : Situation[*];
            end req : FMEARequirement[*];
        }
        connection def Prevention :> ControllingMeasure;
        connection def preventions : Prevention;
        connection def Mitigation :> ControllingMeasure;
        connection def mitigations : Mitigation;
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
            attribute baseType :>> baseType default situations meta SysML::Usage;
        }
        metadata def <cause> CauseMetadata :> SituationMetadata {
            attribute baseType :>> baseType = causes meta SysML::Usage;
        }
        metadata def <failure> FailureModeMetadata :> SituationMetadata {
            attribute baseType :>> baseType = failureModes meta SysML::Usage;
        }
        metadata def <effect> EffectMetadata :> SituationMetadata {
            attribute baseType :>> baseType = effects meta SysML::Usage;
        }
        metadata def <fmea> FMEAItemMetadata :> SituationMetadata {
            attribute annotatedElement : SysML::ItemDefinition :> annotatedElement;
            attribute annotatedElement : SysML::ItemUsage :> annotatedElement;
            attribute baseType :>> baseType = fmeaItems meta SysML::Usage;
        }
        metadata def <causation> CausationMetadata :> SemanticMetadata {
            attribute annotatedElement : SysML::ConnectionUsage :>> annotatedElement;
            attribute baseType :>> baseType = causations meta SysML::Usage;
        }
        metadata def <fmeaspec> FMEARequirementMetadata :> SemanticMetadata {
            attribute annotatedElement : SysML::RequirementUsage :>> annotatedElement;
            attribute baseType :>> baseType = fmeaRequirements meta SysML::Usage;
        }
        metadata def <violation> ViolationMetadata :> SemanticMetadata {
            attribute annotatedElement : SysML::ConnectionUsage :>> annotatedElement;
            attribute baseType :>> baseType = violations meta SysML::Usage;
        }
        abstract metadata def ControllingMeasureMetadata :> SemanticMetadata {
            attribute annotatedElement : SysML::ConnectionUsage :>> annotatedElement;
        }
        metadata def <prevention> PreventionMetadata :> ControllingMeasureMetadata {
            attribute baseType :>> baseType = preventions meta SysML::Usage;
        }
        metadata def <mitigation> MitigationMetadata :> ControllingMeasureMetadata {
            attribute baseType :>> baseType = mitigations meta SysML::Usage;
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
                attribute status = Status::Approved;
            }
            doc
            /* Alarm when battery has sank */
            :>> sil = SIL::A;
        }
        #fmea
        item def 'Glucose FMEA Item' {
            #prevention
            connect 'battery depleted' to req1;
            #cause
            occurrence 'battery depleted' {
                attribute :>> occurs = 0.005;
            }
            #causation
            connect 'battery depleted' to 'battery cannot be charged';
            #failure
            occurrence 'battery cannot be charged' {
                attribute :>> detected = 0.013;
            }
            #causation
            connect 'battery cannot be charged' to 'glucose level undetected';
            #effect
            occurrence 'glucose level undetected';
            #causation
            connect 'glucose level undetected' to 'therapy delay';
            #effect
            occurrence 'therapy delay' {
                attribute :>> severity = "High";
            }
        }
        #violation
        connect 'Glucose Meter in Use' to req2;
        #mitigation
        connect 'Glucose Meter in Use' to req3;
        #fmea
        item 'Glucose Meter in Use' : 'Glucose FMEA Item' {
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
    (reference r1 (scope relative) (span (offset 877) (line 38) (column 31) (len 26)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 877) (line 38) (column 31) (len 11))) (segment 1 (token "HappensBefore") (name "HappensBefore") (separator colon-colon) (span (offset 890) (line 38) (column 44) (len 13)))))
    (reference r2 (scope relative) (span (offset 928) (line 39) (column 23) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 928) (line 39) (column 23) (len 9)))))
    (reference r3 (scope relative) (span (offset 962) (line 40) (column 24) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 962) (line 40) (column 24) (len 9)))))
    (reference r4 (scope relative) (span (offset 1015) (line 43) (column 36) (len 9)) (segments (segment 0 (token "Causation") (name "Causation") (separator none) (span (offset 1015) (line 43) (column 36) (len 9)))))
    (reference r5 (scope relative) (span (offset 1323) (line 56) (column 21) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 1323) (line 56) (column 21) (len 9)))))
    (reference r6 (scope relative) (span (offset 1354) (line 57) (column 21) (len 15)) (segments (segment 0 (token "FMEARequirement") (name "FMEARequirement") (separator none) (span (offset 1354) (line 57) (column 21) (len 15)))))
    (reference r7 (scope relative) (span (offset 1413) (line 60) (column 36) (len 9)) (segments (segment 0 (token "Violation") (name "Violation") (separator none) (span (offset 1413) (line 60) (column 36) (len 9)))))
    (reference r8 (scope relative) (span (offset 1507) (line 63) (column 21) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 1507) (line 63) (column 21) (len 9)))))
    (reference r9 (scope relative) (span (offset 1538) (line 64) (column 21) (len 15)) (segments (segment 0 (token "FMEARequirement") (name "FMEARequirement") (separator none) (span (offset 1538) (line 64) (column 21) (len 15)))))
    (reference r10 (scope relative) (span (offset 1593) (line 67) (column 32) (len 18)) (segments (segment 0 (token "ControllingMeasure") (name "ControllingMeasure") (separator none) (span (offset 1593) (line 67) (column 32) (len 18)))))
    (reference r11 (scope relative) (span (offset 1652) (line 69) (column 37) (len 10)) (segments (segment 0 (token "Prevention") (name "Prevention") (separator none) (span (offset 1652) (line 69) (column 37) (len 10)))))
    (reference r12 (scope relative) (span (offset 1711) (line 71) (column 32) (len 18)) (segments (segment 0 (token "ControllingMeasure") (name "ControllingMeasure") (separator none) (span (offset 1711) (line 71) (column 32) (len 18)))))
    (reference r13 (scope relative) (span (offset 1770) (line 73) (column 37) (len 10)) (segments (segment 0 (token "Mitigation") (name "Mitigation") (separator none) (span (offset 1770) (line 73) (column 37) (len 10)))))
    (reference r14 (scope relative) (span (offset 1852) (line 78) (column 18) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 1852) (line 78) (column 18) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 1865) (line 78) (column 31) (len 16)))))
    (reference r15 (scope relative) (span (offset 1900) (line 79) (column 18) (len 11)) (segments (segment 0 (token "FMEALibrary") (name "FMEALibrary") (separator none) (span (offset 1900) (line 79) (column 18) (len 11)))))
    (reference r16 (scope relative) (span (offset 3689) (line 142) (column 18) (len 11)) (segments (segment 0 (token "FMEALibrary") (name "FMEALibrary") (separator none) (span (offset 3689) (line 142) (column 18) (len 11)))))
    (reference r17 (scope relative) (span (offset 3722) (line 143) (column 18) (len 12)) (segments (segment 0 (token "FMEAMetadata") (name "FMEAMetadata") (separator none) (span (offset 3722) (line 143) (column 18) (len 12)))))
    (reference r18 (scope relative) (span (offset 3745) (line 145) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3745) (line 145) (column 4) (len 8)))))
    (reference r19 (scope relative) (span (offset 3833) (line 149) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3833) (line 149) (column 4) (len 8)))))
    (reference r20 (scope relative) (span (offset 3948) (line 153) (column 4) (len 8)) (segments (segment 0 (token "fmeaspec") (name "fmeaspec") (separator none) (span (offset 3948) (line 153) (column 4) (len 8)))))
    (reference r21 (scope relative) (span (offset 4124) (line 161) (column 4) (len 4)) (segments (segment 0 (token "fmea") (name "fmea") (separator none) (span (offset 4124) (line 161) (column 4) (len 4)))))
    (reference r22 (scope relative) (span (offset 4751) (line 187) (column 4) (len 9)) (segments (segment 0 (token "violation") (name "violation") (separator none) (span (offset 4751) (line 187) (column 4) (len 9)))))
    (reference r23 (scope relative) (span (offset 4769) (line 187) (column 22) (len 22)) (segments (segment 0 (token "'Glucose Meter in Use'") (name "Glucose Meter in Use") (separator none) (span (offset 4769) (line 187) (column 22) (len 22)))))
    (reference r24 (scope relative) (span (offset 4795) (line 187) (column 48) (len 4)) (segments (segment 0 (token "req2") (name "req2") (separator none) (span (offset 4795) (line 187) (column 48) (len 4)))))
    (reference r25 (scope relative) (span (offset 4804) (line 188) (column 4) (len 10)) (segments (segment 0 (token "mitigation") (name "mitigation") (separator none) (span (offset 4804) (line 188) (column 4) (len 10)))))
    (reference r26 (scope relative) (span (offset 4823) (line 188) (column 23) (len 22)) (segments (segment 0 (token "'Glucose Meter in Use'") (name "Glucose Meter in Use") (separator none) (span (offset 4823) (line 188) (column 23) (len 22)))))
    (reference r27 (scope relative) (span (offset 4849) (line 188) (column 49) (len 4)) (segments (segment 0 (token "req3") (name "req3") (separator none) (span (offset 4849) (line 188) (column 49) (len 4)))))
    (reference r28 (scope relative) (span (offset 4862) (line 190) (column 4) (len 4)) (segments (segment 0 (token "fmea") (name "fmea") (separator none) (span (offset 4862) (line 190) (column 4) (len 4)))))
  )
  (root (package (name "14c-Language-Extensions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 64) (line 2) (column 29) (len 3))) (separator (span (offset 64) (line 2) (column 29) (len 2))) (marker (span (offset 66) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (library-package (name "FMEALibrary") (standard false) (body brace (occurrence-def) (occurrence (portion none) (declaration "situations") (target none)) (occurrence-def) (occurrence (portion none) (declaration "causes") (target none)) (occurrence-def) (occurrence (portion none) (declaration "failureModes") (target none)) (occurrence-def) (occurrence (portion none) (declaration "effects") (target none)) (item-def) (item-usage) (connection-def (name "Causation") (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (end (identity (declaration (name "cause") (span (offset 921) (line 39) (column 16) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "effect") (span (offset 954) (line 40) (column 16) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (redefines none) (crosses none)))) (connection-def (name "causations") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (body semicolon)) (requirement-def (name "FMEARequirement") (body semicolon)) (requirement-usage) (requirement-def (name "RequirementWithSIL") (body brace (attribute-usage))) (enum-def (name "SIL") (body brace (enum-value (name "A") (span (offset 1260) (line 53) (column 18) (len 1))) (enum-value (name "B") (span (offset 1263) (line 53) (column 21) (len 1))) (enum-value (name "C") (span (offset 1266) (line 53) (column 24) (len 1))))) (connection-def (name "Violation") (role ordinary) (specializes none) (body brace (end (identity (declaration (name "sit") (span (offset 1318) (line 56) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "req") (span (offset 1349) (line 57) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (references none) (redefines none) (crosses none)))) (connection-def (name "violations") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (body semicolon)) (connection-def (name "ControllingMeasure") (role ordinary) (specializes none) (body brace (end (identity (declaration (name "sit") (span (offset 1502) (line 63) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "req") (span (offset 1533) (line 64) (column 16) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (references none) (redefines none) (crosses none)))) (connection-def (name "Prevention") (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r10)))) (body semicolon)) (connection-def (name "preventions") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (body semicolon)) (connection-def (name "Mitigation") (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (body semicolon)) (connection-def (name "mitigations") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (body semicolon)))) (library-package (name "FMEAMetadata") (standard false) (body brace (import (target (span (span (offset 1852) (line 78) (column 18) (len 29))) (all none) (ref r14) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1900) (line 79) (column 18) (len 14))) (all none) (ref r15) (shape (namespace (wildcard-suffix (span (span (offset 1911) (line 79) (column 29) (len 3))) (separator (span (offset 1911) (line 79) (column 29) (len 2))) (marker (span (offset 1913) (line 79) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (enum-def (name "Status") (body brace (enum-value (name "Approved") (span (offset 1940) (line 82) (column 4) (len 8))) (enum-value (name "NotApproved") (span (offset 1953) (line 83) (column 4) (len 11))))) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def))) (package (name "FMEAUserModel") (body brace (import (target (span (span (offset 3689) (line 142) (column 18) (len 14))) (all none) (ref r16) (shape (namespace (wildcard-suffix (span (span (offset 3700) (line 142) (column 29) (len 3))) (separator (span (offset 3700) (line 142) (column 29) (len 2))) (marker (span (offset 3702) (line 142) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 3722) (line 143) (column 18) (len 15))) (all none) (ref r17) (shape (namespace (wildcard-suffix (span (span (offset 3734) (line 143) (column 30) (len 3))) (separator (span (offset 3734) (line 143) (column 30) (len 2))) (marker (span (offset 3736) (line 143) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (metadata-keyword-usage (type (ref r18)) (body none)) (requirement-usage) (metadata-keyword-usage (type (ref r19)) (body none)) (requirement-usage) (metadata-keyword-usage (type (ref r20)) (body none)) (requirement-usage) (metadata-keyword-usage (type (ref r21)) (body none)) (item-def) (metadata-keyword-usage (type (ref r22)) (body none)) (connect (from (expression (span (offset 4769) (line 187) (column 22) (len 22)) (ref r23))) (to (expression (span (offset 4795) (line 187) (column 48) (len 4)) (ref r24))) (body semicolon) (subsets none) (redefines none)) (metadata-keyword-usage (type (ref r25)) (body none)) (connect (from (expression (span (offset 4823) (line 188) (column 23) (len 22)) (ref r26))) (to (expression (span (offset 4849) (line 188) (column 49) (len 4)) (ref r27))) (body semicolon) (subsets none) (redefines none)) (metadata-keyword-usage (type (ref r28)) (body none)) (item-usage))))))
)
~~~
