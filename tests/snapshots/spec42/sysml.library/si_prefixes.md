# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/SIPrefixes"))
~~~
# SOURCE
~~~sysml
standard library package SIPrefixes {
	doc
	/*
	 * Definition of SI unit prefixes as specified in ISO/IEC 80000-1
	 */

	private import MeasurementReferences::*;

	/*
	 * ISO/IEC 80000-1 prefixes for decimal multiples and sub-multiples
	 * 
	 * See also https://en.wikipedia.org/wiki/Unit_prefix
	 */
	attribute yocto: UnitPrefix { :>> longName = "yocto"; :>> symbol = "y"; :>> conversionFactor = 1E-24; }
	attribute zepto: UnitPrefix { :>> longName = "zepto"; :>> symbol = "z"; :>> conversionFactor = 1E-21; }
	attribute atto: UnitPrefix { :>> longName = "atto"; :>> symbol = "a"; :>> conversionFactor = 1E-18; }
	attribute femto: UnitPrefix { :>> longName = "femto"; :>> symbol = "f"; :>> conversionFactor = 1E-15; }
	attribute pico: UnitPrefix { :>> longName = "pico"; :>> symbol = "p"; :>> conversionFactor = 1E-12; }
	attribute nano: UnitPrefix { :>> longName = "nano"; :>> symbol = "n"; :>> conversionFactor = 1E-9; }
	attribute micro: UnitPrefix { :>> longName = "micro"; :>> symbol = "μ"; :>> conversionFactor = 1E-6; }
	attribute milli: UnitPrefix { :>> longName = "milli"; :>> symbol = "m"; :>> conversionFactor = 1E-3; }
	attribute centi: UnitPrefix { :>> longName = "centi"; :>> symbol = "c"; :>> conversionFactor = 1E-2; }
	attribute deci: UnitPrefix { :>> longName = "deci"; :>> symbol = "d"; :>> conversionFactor = 1E-1; }
	attribute deca: UnitPrefix { :>> longName = "deca"; :>> symbol = "da"; :>> conversionFactor = 1E1; }
	attribute hecto: UnitPrefix { :>> longName = "hecto"; :>> symbol = "h"; :>> conversionFactor = 1E2; }
	attribute kilo: UnitPrefix { :>> longName = "kilo"; :>> symbol = "k"; :>> conversionFactor = 1E3; }
	attribute mega: UnitPrefix { :>> longName = "mega"; :>> symbol = "M"; :>> conversionFactor = 1E6; }
	attribute giga: UnitPrefix { :>> longName = "giga"; :>> symbol = "G"; :>> conversionFactor = 1E9; }
	attribute tera: UnitPrefix { :>> longName = "tera"; :>> symbol = "T"; :>> conversionFactor = 1E12; }
	attribute peta: UnitPrefix { :>> longName = "peta"; :>> symbol = "P"; :>> conversionFactor = 1E15; }
	attribute exa: UnitPrefix { :>> longName = "exa"; :>> symbol = "E"; :>> conversionFactor = 1E18; }
	attribute zetta: UnitPrefix { :>> longName = "zetta"; :>> symbol = "Z"; :>> conversionFactor = 1E21; }
	attribute yotta: UnitPrefix { :>> longName = "yotta"; :>> symbol = "Y"; :>> conversionFactor = 1E24; }
	
	/*
	 * ISO/IEC 80000-1 prefixes for binary multiples, i.e. multiples of 1024 (= 2^10)
	 * 
	 * See also https://en.wikipedia.org/wiki/Binary_prefix
	 */
	attribute kibi: UnitPrefix { :>> longName = "kibi"; :>> symbol = "Ki"; :>> conversionFactor = 1024; }
	attribute mebi: UnitPrefix { :>> longName = "mebi"; :>> symbol = "Mi"; :>> conversionFactor = 1024^2; }
	attribute gibi: UnitPrefix { :>> longName = "gibi"; :>> symbol = "Gi"; :>> conversionFactor = 1024^3; }
	attribute tebi: UnitPrefix { :>> longName = "tebi"; :>> symbol = "Ti"; :>> conversionFactor = 1024^4; }
	attribute pebi: UnitPrefix { :>> longName = "pebi"; :>> symbol = "Pi"; :>> conversionFactor = 1024^5; }
	attribute exbi: UnitPrefix { :>> longName = "exbi"; :>> symbol = "Ei"; :>> conversionFactor = 1024^6; }
	attribute zebi: UnitPrefix { :>> longName = "zebi"; :>> symbol = "Zi"; :>> conversionFactor = 1024^7; }
	attribute yobi: UnitPrefix { :>> longName = "yobi"; :>> symbol = "Yi"; :>> conversionFactor = 1024^8; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "si_prefixes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package SIPrefixes {
    doc
    /*
	 * Definition of SI unit prefixes as specified in ISO/IEC 80000-1
	 */
    private import MeasurementReferences::*;
    attribute def yocto : UnitPrefix {
        attribute :>> longName = "yocto";
        attribute :>> symbol = "y";
        attribute :>> conversionFactor = 1E-24;
    }
    attribute def zepto : UnitPrefix {
        attribute :>> longName = "zepto";
        attribute :>> symbol = "z";
        attribute :>> conversionFactor = 1E-21;
    }
    attribute def atto : UnitPrefix {
        attribute :>> longName = "atto";
        attribute :>> symbol = "a";
        attribute :>> conversionFactor = 1E-18;
    }
    attribute def femto : UnitPrefix {
        attribute :>> longName = "femto";
        attribute :>> symbol = "f";
        attribute :>> conversionFactor = 1E-15;
    }
    attribute def pico : UnitPrefix {
        attribute :>> longName = "pico";
        attribute :>> symbol = "p";
        attribute :>> conversionFactor = 1E-12;
    }
    attribute def nano : UnitPrefix {
        attribute :>> longName = "nano";
        attribute :>> symbol = "n";
        attribute :>> conversionFactor = 1E-9;
    }
    attribute def micro : UnitPrefix {
        attribute :>> longName = "micro";
        attribute :>> symbol = "μ";
        attribute :>> conversionFactor = 1E-6;
    }
    attribute def milli : UnitPrefix {
        attribute :>> longName = "milli";
        attribute :>> symbol = "m";
        attribute :>> conversionFactor = 1E-3;
    }
    attribute def centi : UnitPrefix {
        attribute :>> longName = "centi";
        attribute :>> symbol = "c";
        attribute :>> conversionFactor = 1E-2;
    }
    attribute def deci : UnitPrefix {
        attribute :>> longName = "deci";
        attribute :>> symbol = "d";
        attribute :>> conversionFactor = 1E-1;
    }
    attribute def deca : UnitPrefix {
        attribute :>> longName = "deca";
        attribute :>> symbol = "da";
        attribute :>> conversionFactor = 1E1;
    }
    attribute def hecto : UnitPrefix {
        attribute :>> longName = "hecto";
        attribute :>> symbol = "h";
        attribute :>> conversionFactor = 1E2;
    }
    attribute def kilo : UnitPrefix {
        attribute :>> longName = "kilo";
        attribute :>> symbol = "k";
        attribute :>> conversionFactor = 1E3;
    }
    attribute def mega : UnitPrefix {
        attribute :>> longName = "mega";
        attribute :>> symbol = "M";
        attribute :>> conversionFactor = 1E6;
    }
    attribute def giga : UnitPrefix {
        attribute :>> longName = "giga";
        attribute :>> symbol = "G";
        attribute :>> conversionFactor = 1E9;
    }
    attribute def tera : UnitPrefix {
        attribute :>> longName = "tera";
        attribute :>> symbol = "T";
        attribute :>> conversionFactor = 1E12;
    }
    attribute def peta : UnitPrefix {
        attribute :>> longName = "peta";
        attribute :>> symbol = "P";
        attribute :>> conversionFactor = 1E15;
    }
    attribute def exa : UnitPrefix {
        attribute :>> longName = "exa";
        attribute :>> symbol = "E";
        attribute :>> conversionFactor = 1E18;
    }
    attribute def zetta : UnitPrefix {
        attribute :>> longName = "zetta";
        attribute :>> symbol = "Z";
        attribute :>> conversionFactor = 1E21;
    }
    attribute def yotta : UnitPrefix {
        attribute :>> longName = "yotta";
        attribute :>> symbol = "Y";
        attribute :>> conversionFactor = 1E24;
    }
    attribute def kibi : UnitPrefix {
        attribute :>> longName = "kibi";
        attribute :>> symbol = "Ki";
        attribute :>> conversionFactor = 1024;
    }
    attribute def mebi : UnitPrefix {
        attribute :>> longName = "mebi";
        attribute :>> symbol = "Mi";
        attribute :>> conversionFactor = 1024 ^ 2;
    }
    attribute def gibi : UnitPrefix {
        attribute :>> longName = "gibi";
        attribute :>> symbol = "Gi";
        attribute :>> conversionFactor = 1024 ^ 3;
    }
    attribute def tebi : UnitPrefix {
        attribute :>> longName = "tebi";
        attribute :>> symbol = "Ti";
        attribute :>> conversionFactor = 1024 ^ 4;
    }
    attribute def pebi : UnitPrefix {
        attribute :>> longName = "pebi";
        attribute :>> symbol = "Pi";
        attribute :>> conversionFactor = 1024 ^ 5;
    }
    attribute def exbi : UnitPrefix {
        attribute :>> longName = "exbi";
        attribute :>> symbol = "Ei";
        attribute :>> conversionFactor = 1024 ^ 6;
    }
    attribute def zebi : UnitPrefix {
        attribute :>> longName = "zebi";
        attribute :>> symbol = "Zi";
        attribute :>> conversionFactor = 1024 ^ 7;
    }
    attribute def yobi : UnitPrefix {
        attribute :>> longName = "yobi";
        attribute :>> symbol = "Yi";
        attribute :>> conversionFactor = 1024 ^ 8;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 136) (line 7) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 136) (line 7) (column 17) (len 21)))))
  )
  (root (library-package (name "SIPrefixes") (standard true) (body brace (doc) (import (target (span (span (offset 136) (line 7) (column 17) (len 24))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 157) (line 7) (column 38) (len 3))) (separator (span (offset 157) (line 7) (column 38) (len 2))) (marker (span (offset 159) (line 7) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def) (attribute-def))))
)
~~~
