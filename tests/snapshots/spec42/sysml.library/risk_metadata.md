# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Metadata/RiskMetadata"))
~~~
# SOURCE
~~~sysml
standard library package RiskMetadata {
	doc
	/*
	 * This package defines metadata for annotating model elements with assessments of risk.
	 */

	private import ScalarValues::Real;
	
	attribute def Level :> Real {
		doc
		/*
		 * A Level is a Real number in the interval 0.0 to 1.0, inclusive.
		 */
	
		assert constraint { that >= 0.0 and that <= 1.0 }
	}
	
	enum def LevelEnum :> Level {
		doc
		/*
		 * LevelEnum provides standard probability Levels for low, medium and high risks.
		 */
	
		low = 0.25;
		medium = 0.50;
		high = 0.75;
	}

	attribute def RiskLevel {
		doc
		/*
		 * RiskLevel gives the probability of a risk occurring and, optionally, the impact
		 * if the risk occurs.
		 */
	
		attribute probability : Level {
			doc
			/*
			 * The probability that a risk will occur.
			 */
		}
		
		attribute impact : Level [0..1] {
			doc
			/*
			 * The impact of the risk if it occurs (with 0.0 being no impact and 1.0 being 
			 * the most severe impact).
			 */
		}
	}
	
	enum def RiskLevelEnum :> RiskLevel {
		doc
		/*
		 * RiskLevelEnum enumerates standard RiskLevels for low, medium and high risks
		 * (without including impact).
		 */

		low = new RiskLevel(probability = LevelEnum::low);
		medium = new RiskLevel(probability = LevelEnum::medium);
		high = new RiskLevel(probability = LevelEnum::high);
	}
	
	metadata def Risk {
		doc
		/*
		 * Risk is used to annotate a model element with an assessment of the risk related to it
		 * in some typical risk areas.
		 */
	
		attribute totalRisk : RiskLevel [0..1] {
			doc
			/*
			 * The total risk associated with the annotated element.
			 */
		}
		
		attribute technicalRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk of unresolved technical issues regarding the annotated element.
			 */
		}
		
		attribute scheduleRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk that work on the annotated element will not be completed on schedule.
			 */
		}
		
		attribute costRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk that work on the annotated element will exceed its planned cost.
			 */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "risk_metadata.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package RiskMetadata {
    doc
    /*
	 * This package defines metadata for annotating model elements with assessments of risk.
	 */
    private import ScalarValues::Real;
    attribute def Level :> Real {
        doc
        /*
		 * A Level is a Real number in the interval 0.0 to 1.0, inclusive.
		 */
        assert constraint {
            that >= 0.0 && that <= 1.0;
        }
    }
    enum def LevelEnum :> Level {
        low;
        medium;
        high;
    }
    attribute def RiskLevel {
        doc
        /*
		 * RiskLevel gives the probability of a risk occurring and, optionally, the impact
		 * if the risk occurs.
		 */
        attribute probability : Level {
            doc
            /*
			 * The probability that a risk will occur.
			 */
        }
        attribute impact : Level[0..1] {
            doc
            /*
			 * The impact of the risk if it occurs (with 0.0 being no impact and 1.0 being 
			 * the most severe impact).
			 */
        }
    }
    enum def RiskLevelEnum :> RiskLevel {
        low;
        medium;
        high;
    }
    metadata def Risk {
        doc
        /*
		 * Risk is used to annotate a model element with an assessment of the risk related to it
		 * in some typical risk areas.
		 */
        attribute totalRisk : RiskLevel[0..1] {
            doc
            /*
			 * The total risk associated with the annotated element.
			 */
        }
        attribute technicalRisk : RiskLevel[0..1] {
            doc
            /*
			 * The risk of unresolved technical issues regarding the annotated element.
			 */
        }
        attribute scheduleRisk : RiskLevel[0..1] {
            doc
            /*
			 * The risk that work on the annotated element will not be completed on schedule.
			 */
        }
        attribute costRisk : RiskLevel[0..1] {
            doc
            /*
			 * The risk that work on the annotated element will exceed its planned cost.
			 */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 161) (line 7) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 161) (line 7) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 175) (line 7) (column 31) (len 4)))))
  )
  (root (library-package (name "RiskMetadata") (standard true) (body brace (doc) (import (target (span (span (offset 161) (line 7) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def) (enum-def) (attribute-def) (enum-def) (metadata-def))))
)
~~~
