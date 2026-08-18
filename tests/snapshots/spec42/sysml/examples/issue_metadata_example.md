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
        attribute text = "This issue is about the interface compatability between the engine and transmission." + "The interface def includes an end defined by a ClutchPort." + "However, the interface usage connects the transmission port that is defined by ~DrivePwrPort." + "This should have surfaced a compatibility issue, since the interface is not really compatible with its definition";
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
    (reference r1 (scope relative) (span (offset 733) (line 14) (column 16) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 733) (line 14) (column 16) (len 12)))))
    (reference r2 (scope relative) (span (offset 762) (line 15) (column 16) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 762) (line 15) (column 16) (len 10)))))
  )
  (root (package (name "IssueMetadataExample") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 23))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (metadata-usage) (interface-def (name "EngineToTransmissionInterface") (modifiers) (specializes none) (body brace (end (short-name none) (identity (declaration (name "p1") (span (offset 730) (line 14) (column 13) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "p2") (span (offset 759) (line 15) (column 13) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (redefines none) (crosses none)))) (port-def (name "DrivePwrPort") (specializes none) (body semicolon)) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))) (interface-usage))))
)
~~~
