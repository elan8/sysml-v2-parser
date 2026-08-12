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
  )
  (root (package (name "IssueMetadataExample") (body (import (target (span (span (offset 47) (line 2) (column 17) (len 23))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (metadata-usage) (interface-def) (port-def (name "DrivePwrPort") (specializes none) (body semicolon)) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (part-usage) (part-usage) (interface-usage))))
)
~~~
