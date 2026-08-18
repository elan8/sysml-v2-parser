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
        attribute impact : Level[0..1] {
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
    (reference r1 (scope relative) (span (offset 207) (line 9) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 207) (line 9) (column 25) (len 4)))))
    (reference r2 (scope relative) (span (offset 725) (line 36) (column 27) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 725) (line 36) (column 27) (len 5)))))
    (reference r3 (scope relative) (span (offset 827) (line 43) (column 22) (len 5)) (segments (segment 0 (token "Level") (name "Level") (separator none) (span (offset 827) (line 43) (column 22) (len 5)))))
    (reference r4 (scope relative) (span (offset 1168) (line 59) (column 13) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1168) (line 59) (column 13) (len 9)))))
    (reference r5 (scope relative) (span (offset 1178) (line 59) (column 23) (len 11)) (segments (segment 0 (token "probability") (name "probability") (separator none) (span (offset 1178) (line 59) (column 23) (len 11)))))
    (reference r6 (scope relative) (span (offset 1192) (line 59) (column 37) (len 14)) (segments (segment 0 (token "LevelEnum") (name "LevelEnum") (separator none) (span (offset 1192) (line 59) (column 37) (len 9))) (segment 1 (token "low") (name "low") (separator colon-colon) (span (offset 1203) (line 59) (column 48) (len 3)))))
    (reference r7 (scope relative) (span (offset 1224) (line 60) (column 16) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1224) (line 60) (column 16) (len 9)))))
    (reference r8 (scope relative) (span (offset 1234) (line 60) (column 26) (len 11)) (segments (segment 0 (token "probability") (name "probability") (separator none) (span (offset 1234) (line 60) (column 26) (len 11)))))
    (reference r9 (scope relative) (span (offset 1248) (line 60) (column 40) (len 17)) (segments (segment 0 (token "LevelEnum") (name "LevelEnum") (separator none) (span (offset 1248) (line 60) (column 40) (len 9))) (segment 1 (token "medium") (name "medium") (separator colon-colon) (span (offset 1259) (line 60) (column 51) (len 6)))))
    (reference r10 (scope relative) (span (offset 1281) (line 61) (column 14) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1281) (line 61) (column 14) (len 9)))))
    (reference r11 (scope relative) (span (offset 1291) (line 61) (column 24) (len 11)) (segments (segment 0 (token "probability") (name "probability") (separator none) (span (offset 1291) (line 61) (column 24) (len 11)))))
    (reference r12 (scope relative) (span (offset 1305) (line 61) (column 38) (len 15)) (segments (segment 0 (token "LevelEnum") (name "LevelEnum") (separator none) (span (offset 1305) (line 61) (column 38) (len 9))) (segment 1 (token "high") (name "high") (separator colon-colon) (span (offset 1316) (line 61) (column 49) (len 4)))))
    (reference r13 (scope relative) (span (offset 1516) (line 71) (column 25) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1516) (line 71) (column 25) (len 9)))))
    (reference r14 (scope relative) (span (offset 1650) (line 78) (column 29) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1650) (line 78) (column 29) (len 9)))))
    (reference r15 (scope relative) (span (offset 1802) (line 85) (column 28) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1802) (line 85) (column 28) (len 9)))))
    (reference r16 (scope relative) (span (offset 1956) (line 92) (column 24) (len 9)) (segments (segment 0 (token "RiskLevel") (name "RiskLevel") (separator none) (span (offset 1956) (line 92) (column 24) (len 9)))))
  )
  (root (library-package (name "RiskMetadata") (standard true) (body brace (doc) (import (target (span (span (offset 161) (line 7) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "Level") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (assert-constraint))) (enum-def (name "LevelEnum") (body brace (doc) (enum-value (name "low") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 501) (line 24) (column 9) (len 4)) (real "0.25"))))) (body semicolon) (span (offset 495) (line 24) (column 3) (len 11))) (enum-value (name "medium") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 518) (line 25) (column 12) (len 4)) (real "0.50"))))) (body semicolon) (span (offset 509) (line 25) (column 3) (len 14))) (enum-value (name "high") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 533) (line 26) (column 10) (len 4)) (real "0.75"))))) (body semicolon) (span (offset 526) (line 26) (column 3) (len 12))))) (attribute-def (declaration-name "RiskLevel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name "probability") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "impact") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))))) (enum-def (name "RiskLevelEnum") (body brace (doc) (enum-value (name "low") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1164) (line 59) (column 9) (len 43)) (constructor (type (ref r4)) (arguments (argument (parameter (ref r5)) (value (expression (span (offset 1192) (line 59) (column 37) (len 14)) (ref r6)))))))))) (body semicolon) (span (offset 1158) (line 59) (column 3) (len 50))) (enum-value (name "medium") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1220) (line 60) (column 12) (len 46)) (constructor (type (ref r7)) (arguments (argument (parameter (ref r8)) (value (expression (span (offset 1248) (line 60) (column 40) (len 17)) (ref r9)))))))))) (body semicolon) (span (offset 1211) (line 60) (column 3) (len 56))) (enum-value (name "high") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1277) (line 61) (column 10) (len 44)) (constructor (type (ref r10)) (arguments (argument (parameter (ref r11)) (value (expression (span (offset 1305) (line 61) (column 38) (len 15)) (ref r12)))))))))) (body semicolon) (span (offset 1270) (line 61) (column 3) (len 52))))) (metadata-def (name "Risk") (abstract false) (specializes none) (body brace (doc) (attribute-usage (declaration-name "totalRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "technicalRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "scheduleRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "costRisk") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))))))))
)
~~~
