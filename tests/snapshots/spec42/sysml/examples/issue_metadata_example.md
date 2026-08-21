# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Metadata): IssueMetadataExample"))
~~~
# SOURCE
~~~sysml
package IssueMetadataExample {
	private import ModelingMetadata::Issue;
	
    //Example: the following identifies an issue with the interface
    
    metadata InterfaceCompatibilityIssue : Issue about engineToTransmissionInterface {
    	text = "This issue is about the interface compatability between the engine and transmission." +
               "The interface def includes an end defined by a ClutchPort." +
               "However, the interface usage connects the transmission port that is defined by ~DrivePwrPort." +
               "This should have surfaced a compatibility issue, since the interface is not really compatible with its definition";
    }
    
    interface def EngineToTransmissionInterface{
        end p1:DrivePwrPort;
        end p2:ClutchPort;
    }
    port def DrivePwrPort;
    port def ClutchPort;
    
    part engine{
        port drivePwrPort:DrivePwrPort;
    }
    part transmission{
        port clutchPort:~DrivePwrPort;
    }

    interface engineToTransmissionInterface:EngineToTransmissionInterface
        connect engine.drivePwrPort to transmission.clutchPort;       

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "issue_metadata_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package IssueMetadataExample {
    private import ModelingMetadata::Issue;
    metadata InterfaceCompatibilityIssue : Issue about engineToTransmissionInterface {
        text = "This issue is about the interface compatability between the engine and transmission." + "The interface def includes an end defined by a ClutchPort." + "However, the interface usage connects the transmission port that is defined by ~DrivePwrPort." + "This should have surfaced a compatibility issue, since the interface is not really compatible with its definition";
    }
    interface def EngineToTransmissionInterface {
        end p1 : DrivePwrPort;
        end p2 : ClutchPort;
    }
    port def DrivePwrPort;
    port def ClutchPort;
    part engine {
        port drivePwrPort : DrivePwrPort;
    }
    part transmission {
        port clutchPort : ~DrivePwrPort;
    }
    interface engineToTransmissionInterface : EngineToTransmissionInterface connect engine.drivePwrPort to transmission.clutchPort;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 23)) (segments (segment 0 (token "ModelingMetadata") (name "ModelingMetadata") (separator none) (span (offset 47) (line 2) (column 17) (len 16))) (segment 1 (token "Issue") (name "Issue") (separator colon-colon) (span (offset 65) (line 2) (column 35) (len 5)))))
    (reference r1 (scope relative) (span (offset 190) (line 6) (column 44) (len 5)) (segments (segment 0 (token "Issue") (name "Issue") (separator none) (span (offset 190) (line 6) (column 44) (len 5)))))
    (reference r2 (scope relative) (span (offset 202) (line 6) (column 56) (len 29)) (segments (segment 0 (token "engineToTransmissionInterface") (name "engineToTransmissionInterface") (separator none) (span (offset 202) (line 6) (column 56) (len 29)))))
    (reference r3 (scope relative) (span (offset 239) (line 7) (column 6) (len 4)) (segments (segment 0 (token "text") (name "text") (separator none) (span (offset 239) (line 7) (column 6) (len 4)))))
    (reference r4 (scope relative) (span (offset 733) (line 14) (column 16) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 733) (line 14) (column 16) (len 12)))))
    (reference r5 (scope relative) (span (offset 762) (line 15) (column 16) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 762) (line 15) (column 16) (len 10)))))
    (reference r6 (scope relative) (span (offset 880) (line 21) (column 27) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 880) (line 21) (column 27) (len 12)))))
    (reference r7 (scope relative) (span (offset 948) (line 24) (column 26) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 948) (line 24) (column 26) (len 12)))))
    (reference r8 (scope relative) (span (offset 1059) (line 28) (column 17) (len 19)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 1059) (line 28) (column 17) (len 6))) (segment 1 (token "drivePwrPort") (name "drivePwrPort") (separator dot) (span (offset 1066) (line 28) (column 24) (len 12)))))
    (reference r9 (scope relative) (span (offset 1082) (line 28) (column 40) (len 23)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 1082) (line 28) (column 40) (len 12))) (segment 1 (token "clutchPort") (name "clutchPort") (separator dot) (span (offset 1095) (line 28) (column 53) (len 10)))))
  )
  (root (package (name "IssueMetadataExample") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 23))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (metadata-usage (declaration-name "InterfaceCompatibilityIssue") (type (ref r1)) (about (ref r2)) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r3)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 246) (line 7) (column 13) (len 410)) (binary (operator "+") (left (expression (span (offset 246) (line 7) (column 13) (len 277)) (binary (operator "+") (left (expression (span (offset 246) (line 7) (column 13) (len 164)) (binary (operator "+") (left (expression (span (offset 246) (line 7) (column 13) (len 86)) (string "This issue is about the interface compatability between the engine and transmission."))) (right (expression (span (offset 350) (line 8) (column 16) (len 60)) (string "The interface def includes an end defined by a ClutchPort.")))))) (right (expression (span (offset 428) (line 9) (column 16) (len 95)) (string "However, the interface usage connects the transmission port that is defined by ~DrivePwrPort.")))))) (right (expression (span (offset 541) (line 10) (column 16) (len 115)) (string "This should have surfaced a compatibility issue, since the interface is not really compatible with its definition")))))))) (body semicolon)))) (interface-def (name "EngineToTransmissionInterface") (modifiers) (specializes none) (body brace (end (introducer bare) (short-name none) (identity (declaration (name "p1") (span (offset 730) (line 14) (column 13) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (introducer bare) (short-name none) (identity (declaration (name "p2") (span (offset 759) (line 15) (column 13) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (port-def (name "DrivePwrPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "ClutchPort") (modifiers) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drivePwrPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "clutchPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r8)))) (to (interface-end (multiplicity none) (target (ref r9)))))) (body semicolon)))))
)
~~~
