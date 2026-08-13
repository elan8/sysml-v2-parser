# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 41 (Language Extension): Model Library Example"))
~~~
# SOURCE
~~~sysml
library package 'Model Library Example' {
	private import ScalarValues::Real;
	private import RiskMetadata::Level;
	
	abstract occurrence def Situation;
	
	abstract occurrence situations : Situation[*] nonunique;
	
	abstract occurrence def Cause {
		attribute probability : Real;
	}
	
	abstract occurrence causes : Cause[*] nonunique :> situations;
	
	abstract occurrence def Failure {
		attribute severity : Level;
	}
	
	abstract occurrence failures : Failure[*] nonunique :> situations;
	
	abstract connection def Causation :> Occurrences::HappensBefore {
		end [*] ref cause : Situation;
		end [*] ref effect : Situation;
	}
	
	abstract connection causations : Causation[*] nonunique;
	
	item def Scenario {
		occurrence :>> situations;
		occurrence :>> causes :> situations;
		occurrence :>> failures :> situations;
	}
	
	item scenarios : Scenario[*] nonunique;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "41_model_library_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package 'Model Library Example' {
    private import ScalarValues::Real;
    private import RiskMetadata::Level;
    abstract occurrence def Situation;
    abstract occurrence situations : Situation[*];
    abstract occurrence def Cause {
        attribute probability : Real;
    }
    abstract occurrence causes : Cause[*] :> situations;
    abstract occurrence def Failure {
        attribute severity : Level;
    }
    abstract occurrence failures : Failure[*] :> situations;
    connection def Causation :> Occurrences::HappensBefore {
        end cause : Situation[*];
        end effect : Situation[*];
    }
    connection def causations : Causation;
    item def Scenario {
        occurrence  :>> situations;
        occurrence  :> situations :>> causes;
        occurrence  :> situations :>> failures;
    }
    item scenarios : Scenario;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 58) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 72) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 94) (line 3) (column 17) (len 19)) (segments (segment 0 (token "RiskMetadata") (name "RiskMetadata") (separator none) (span (offset 94) (line 3) (column 17) (len 12))) (segment 1 (token "Level") (name "Level") (separator colon-colon) (span (offset 108) (line 3) (column 31) (len 5)))))
    (reference r2 (scope relative) (span (offset 529) (line 21) (column 39) (len 26)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 529) (line 21) (column 39) (len 11))) (segment 1 (token "HappensBefore") (name "HappensBefore") (separator colon-colon) (span (offset 542) (line 21) (column 52) (len 13)))))
    (reference r3 (scope relative) (span (offset 580) (line 22) (column 23) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 580) (line 22) (column 23) (len 9)))))
    (reference r4 (scope relative) (span (offset 614) (line 23) (column 24) (len 9)) (segments (segment 0 (token "Situation") (name "Situation") (separator none) (span (offset 614) (line 23) (column 24) (len 9)))))
    (reference r5 (scope relative) (span (offset 664) (line 26) (column 35) (len 9)) (segments (segment 0 (token "Causation") (name "Causation") (separator none) (span (offset 664) (line 26) (column 35) (len 9)))))
  )
  (root (library-package (name "Model Library Example") (standard false) (body (import (target (span (span (offset 58) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 94) (line 3) (column 17) (len 19))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (occurrence-def) (occurrence (portion none) (declaration "situations") (target none)) (occurrence-def) (occurrence (portion none) (declaration "causes") (target none)) (occurrence-def) (occurrence (portion none) (declaration "failures") (target none)) (connection-def (name "Causation") (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body (end (identity (declaration (name "cause") (span (offset 572) (line 22) (column 15) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "effect") (span (offset 605) (line 23) (column 15) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (redefines none) (crosses none)))) (connection-def (name "causations") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (body semicolon)) (item-def) (item-usage))))
)
~~~
