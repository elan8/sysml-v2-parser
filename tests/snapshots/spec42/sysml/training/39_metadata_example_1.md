# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 39 (Metadata): Metadata Example-1"))
~~~
# SOURCE
~~~sysml
package 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def SecurityFeature {
		:> annotatedElement : SysML::PartDefinition;
		:> annotatedElement : SysML::PartUsage;
	}
	
	metadata SafetyFeature about 
		vehicle::interior::seatBelt,
		vehicle::interior::driverAirBag,
		vehicle::bodyAssy::bumper;
	
	metadata SecurityFeature about
		vehicle::interior::alarm,
		vehicle::bodyAssy::keylessEntry;
		
	part vehicle {
		part interior {
			part alarm;
			part seatBelt[2];
			part frontSeat[2];
			part driverAirBag;
		}
		part bodyAssy {
			part body;
			part bumper;
			part keylessEntry;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "39_metadata_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Metadata Example-1' {
    metadata def SafetyFeature;
    metadata def SecurityFeature {
        attribute annotatedElement : SysML::PartDefinition :> annotatedElement;
        attribute annotatedElement : SysML::PartUsage :> annotatedElement;
    }
    metadata SafetyFeature about vehicle::interior::seatBelt, vehicle::interior::driverAirBag, vehicle::bodyAssy::bumper;
    metadata SecurityFeature about vehicle::interior::alarm, vehicle::bodyAssy::keylessEntry;
    part vehicle {
        part interior {
            part alarm;
            part seatBelt[2];
            part frontSeat[2];
            part driverAirBag;
        }
        part bodyAssy {
            part body;
            part bumper;
            part keylessEntry;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Metadata Example-1") (body (metadata-def) (metadata-def) (metadata-usage) (metadata-usage) (part-usage))))
)
~~~
