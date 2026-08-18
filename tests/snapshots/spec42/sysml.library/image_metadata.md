# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Metadata/ImageMetadata"))
~~~
# SOURCE
~~~sysml
standard library package ImageMetadata {
	doc
	/*
	 * This package provides attributive data and metadata to allow a model element to be
	 * annotated with an image to be used in its graphical rendering or as a marker to
	 * adorn graphical or textual renderings.
	 */
	 
	private import ScalarValues::String;
	
	attribute def Image {
		doc
		/*
		 * Image provides the data necessary for the physical definition of 
		 * a graphical image.
		 */
		 
		attribute content : String[0..1] {
			doc
			/*
			 * Binary data for the image according to the given MIME type, 
			 * encoded as given by the encoding.
			 */
		}
		
		attribute encoding : String[0..1] {
			doc
			/*
			 * Describes how characters in the content are to be decoded into 
			 * binary data. At least "base64", "hex", "identify", and "JSONescape"
			 * shall be supported.
			 */
		}
		
		attribute type : String[0..1] {
			doc
			/*
			 * The MIME type according to which the content should be interpreted.
			 */
		}
		
		attribute location : String[0..1] {
			doc
			/*
			 * A URI for the location of a resource containing the image content,
			 * as an alternative for embedding it in the content attribute.
			 */
		}
	}
	
	metadata def Icon {
		doc
		/*
		 * Icon metadata can be used to annotate a model element with an image to be used
		 * to show render the element on a diagram and/or a small image to be used as an
		 * adornment on a graphical or textual rendering. Alternatively, another metadata
		 * definition can be annotated with an Icon to indicate that any model element 
		 * annotated by the containing metadata can be rendered according to the Icon.
		 */
		 
		attribute fullImage : Image[0..1] {
			doc
			/*
			 * A full-sized image that can be used to render the annotated element on a
			 * graphical view, potentially as an alternative to its standard rendering.
			 */
		}
		
		attribute smallImage : Image[0..1] {
			doc
			/*
			 * A smaller image that can be used as an adornment on the graphical rendering
			 * of the annotated element or as a marker in a textual rendering.
			 */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "image_metadata.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ImageMetadata {
    doc
    /*
	 * This package provides attributive data and metadata to allow a model element to be
	 * annotated with an image to be used in its graphical rendering or as a marker to
	 * adorn graphical or textual renderings.
	 */
    private import ScalarValues::String;
    attribute def Image {
        doc
        /*
		 * Image provides the data necessary for the physical definition of 
		 * a graphical image.
		 */
        attribute content : String[0..1] {
            doc
            /*
			 * Binary data for the image according to the given MIME type, 
			 * encoded as given by the encoding.
			 */
        }
        attribute encoding : String[0..1] {
            doc
            /*
			 * Describes how characters in the content are to be decoded into 
			 * binary data. At least "base64", "hex", "identify", and "JSONescape"
			 * shall be supported.
			 */
        }
        attribute type : String[0..1] {
            doc
            /*
			 * The MIME type according to which the content should be interpreted.
			 */
        }
        attribute location : String[0..1] {
            doc
            /*
			 * A URI for the location of a resource containing the image content,
			 * as an alternative for embedding it in the content attribute.
			 */
        }
    }
    metadata def Icon {
        doc
        /*
		 * Icon metadata can be used to annotate a model element with an image to be used
		 * to show render the element on a diagram and/or a small image to be used as an
		 * adornment on a graphical or textual rendering. Alternatively, another metadata
		 * definition can be annotated with an Icon to indicate that any model element 
		 * annotated by the containing metadata can be rendered according to the Icon.
		 */
        attribute fullImage : Image[0..1] {
            doc
            /*
			 * A full-sized image that can be used to render the annotated element on a
			 * graphical view, potentially as an alternative to its standard rendering.
			 */
        }
        attribute smallImage : Image[0..1] {
            doc
            /*
			 * A smaller image that can be used as an adornment on the graphical rendering
			 * of the annotated element or as a marker in a textual rendering.
			 */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 288) (line 9) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 288) (line 9) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 302) (line 9) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 473) (line 18) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 473) (line 18) (column 23) (len 6)))))
    (reference r2 (scope relative) (span (offset 645) (line 26) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 645) (line 26) (column 24) (len 6)))))
    (reference r3 (scope relative) (span (offset 876) (line 35) (column 20) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 876) (line 35) (column 20) (len 6)))))
    (reference r4 (scope relative) (span (offset 1015) (line 42) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 1015) (line 42) (column 24) (len 6)))))
    (reference r5 (scope relative) (span (offset 1679) (line 61) (column 25) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 1679) (line 61) (column 25) (len 5)))))
    (reference r6 (scope relative) (span (offset 1903) (line 69) (column 26) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 1903) (line 69) (column 26) (len 5)))))
  )
  (root (library-package (name "ImageMetadata") (standard true) (body brace (doc) (import (target (span (span (offset 288) (line 9) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "Image") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name "content") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "encoding") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "type") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "location") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))))) (metadata-def (name "Icon") (abstract false) (specializes none) (body brace (doc) (attribute-usage (declaration-name "fullImage") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))) (attribute-usage (declaration-name "smallImage") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc))))))))
)
~~~
