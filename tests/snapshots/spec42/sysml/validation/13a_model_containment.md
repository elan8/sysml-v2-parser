# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (13-Model Containment): 13a-Model Containment"))
~~~
# SOURCE
~~~sysml
package '13a-Model Containment' {
	private import '2a-Parts Interconnection'::*;
	private import '8-Requirements'::*;
	
	requirement BodyAndInteriorRequirements {
		public import MassLimitationRequirement; 
	}
	
	requirement PowerTrainRequirements;
	
	package 'Vehicle Model' {
		doc
		/*
		 * This package is used to represent a top-level "model".
		 * There is no specific syntax for identifying a package
		 * used in this way.
		 */
	
		
		package 'Vehicle1-Configuration' {			
			alias 'Sport Sedan' for vehicle1_c1;
			
			public import 'vehicle1_c1 Specification Context'::'vehicle1-c1 Specification';		
		}
		
		package 'Vehicle Reference Model' {
			doc
			/*
			 * This package is used to represent a "model library".
			 * There is no specific syntax for identifying a package
			 * used in this way.
			 */
		
			public import VehicleA;			
			public import VehicleSubsystems;
			
			//*
			// The following would transitively import all the
			// members of the VehicleSubsystems package, rather
			// then importing the package itself.
			 
			   public import VehicleSubsystems::*;
			*/
		}
		
		package VehicleSubsystems {
			public import 'Body&Interior';
			public import 'PowerTrain';
		}
		
		package 'Body&Interior' {
			public import BodyAndInteriorRequirements;			
		}
		
		package PowerTrain {
			public import Engine;
			public import Transmission;
			public import PowerTrainRequirements;			
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13a_model_containment.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '13a-Model Containment' {
    private import '2a-Parts Interconnection'::*;
    private import '8-Requirements'::*;
    requirement BodyAndInteriorRequirements {
        public import MassLimitationRequirement;
    }
    requirement PowerTrainRequirements;
    package 'Vehicle Model' {
        doc
        /*
		 * This package is used to represent a top-level "model".
		 * There is no specific syntax for identifying a package
		 * used in this way.
		 */
        package 'Vehicle1-Configuration' {
            alias 'Sport Sedan' for vehicle1_c1;
            public import 'vehicle1_c1 Specification Context'::'vehicle1-c1 Specification';
        }
        package 'Vehicle Reference Model' {
            doc
            /*
			 * This package is used to represent a "model library".
			 * There is no specific syntax for identifying a package
			 * used in this way.
			 */
            public import VehicleA;
            public import VehicleSubsystems;
        }
        package VehicleSubsystems {
            public import 'Body&Interior';
            public import PowerTrain;
        }
        package 'Body&Interior' {
            public import BodyAndInteriorRequirements;
        }
        package PowerTrain {
            public import Engine;
            public import Transmission;
            public import PowerTrainRequirements;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 26)) (segments (segment 0 (token "'2a-Parts Interconnection'") (name "2a-Parts Interconnection") (separator none) (span (offset 50) (line 2) (column 17) (len 26)))))
    (reference r1 (scope relative) (span (offset 97) (line 3) (column 17) (len 16)) (segments (segment 0 (token "'8-Requirements'") (name "8-Requirements") (separator none) (span (offset 97) (line 3) (column 17) (len 16)))))
    (reference r2 (scope relative) (span (offset 509) (line 21) (column 28) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 509) (line 21) (column 28) (len 11)))))
    (reference r3 (scope relative) (span (offset 543) (line 23) (column 18) (len 64)) (segments (segment 0 (token "'vehicle1_c1 Specification Context'") (name "vehicle1_c1 Specification Context") (separator none) (span (offset 543) (line 23) (column 18) (len 35))) (segment 1 (token "'vehicle1-c1 Specification'") (name "vehicle1-c1 Specification") (separator colon-colon) (span (offset 580) (line 23) (column 55) (len 27)))))
    (reference r4 (scope relative) (span (offset 839) (line 34) (column 18) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 839) (line 34) (column 18) (len 8)))))
    (reference r5 (scope relative) (span (offset 869) (line 35) (column 18) (len 17)) (segments (segment 0 (token "VehicleSubsystems") (name "VehicleSubsystems") (separator none) (span (offset 869) (line 35) (column 18) (len 17)))))
    (reference r6 (scope relative) (span (offset 1156) (line 47) (column 18) (len 15)) (segments (segment 0 (token "'Body&Interior'") (name "Body&Interior") (separator none) (span (offset 1156) (line 47) (column 18) (len 15)))))
    (reference r7 (scope relative) (span (offset 1190) (line 48) (column 18) (len 12)) (segments (segment 0 (token "'PowerTrain'") (name "PowerTrain") (separator none) (span (offset 1190) (line 48) (column 18) (len 12)))))
    (reference r8 (scope relative) (span (offset 1256) (line 52) (column 18) (len 27)) (segments (segment 0 (token "BodyAndInteriorRequirements") (name "BodyAndInteriorRequirements") (separator none) (span (offset 1256) (line 52) (column 18) (len 27)))))
    (reference r9 (scope relative) (span (offset 1335) (line 56) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1335) (line 56) (column 18) (len 6)))))
    (reference r10 (scope relative) (span (offset 1360) (line 57) (column 18) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1360) (line 57) (column 18) (len 12)))))
    (reference r11 (scope relative) (span (offset 1391) (line 58) (column 18) (len 22)) (segments (segment 0 (token "PowerTrainRequirements") (name "PowerTrainRequirements") (separator none) (span (offset 1391) (line 58) (column 18) (len 22)))))
  )
  (root (package (name "13a-Model Containment") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 29))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 76) (line 2) (column 43) (len 3))) (separator (span (offset 76) (line 2) (column 43) (len 2))) (marker (span (offset 78) (line 2) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 97) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 113) (line 3) (column 33) (len 3))) (separator (span (offset 113) (line 3) (column 33) (len 2))) (marker (span (offset 115) (line 3) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-usage (name "BodyAndInteriorRequirements") (multiplicity none)) (requirement-usage (name "PowerTrainRequirements") (multiplicity none)) (package (name "Vehicle Model") (body brace (doc (name none) (locale none) (body (span (offset 288) (line 13) (column 5) (len 146)) (normalized "This package is used to represent a top-level \"model\".\nThere is no specific syntax for identifying a package\nused in this way.\n"))) (package (name "Vehicle1-Configuration") (body brace (alias (name "Sport Sedan") (target (ref r2)) (body semicolon)) (import (target (span (span (offset 543) (line 23) (column 18) (len 64))) (all none) (ref r3) (shape (membership (recursive-suffix none))))))) (package (name "Vehicle Reference Model") (body brace (doc (name none) (locale none) (body (span (offset 668) (line 28) (column 6) (len 148)) (normalized "This package is used to represent a \"model library\".\nThere is no specific syntax for identifying a package\nused in this way.\n"))) (import (target (span (span (offset 839) (line 34) (column 18) (len 8))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 869) (line 35) (column 18) (len 17))) (all none) (ref r5) (shape (membership (recursive-suffix none))))))) (package (name "VehicleSubsystems") (body brace (import (target (span (span (offset 1156) (line 47) (column 18) (len 15))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1190) (line 48) (column 18) (len 12))) (all none) (ref r7) (shape (membership (recursive-suffix none))))))) (package (name "Body&Interior") (body brace (import (target (span (span (offset 1256) (line 52) (column 18) (len 27))) (all none) (ref r8) (shape (membership (recursive-suffix none))))))) (package (name "PowerTrain") (body brace (import (target (span (span (offset 1335) (line 56) (column 18) (len 6))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1360) (line 57) (column 18) (len 12))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1391) (line 58) (column 18) (len 22))) (all none) (ref r11) (shape (membership (recursive-suffix none))))))))))))
)
~~~
