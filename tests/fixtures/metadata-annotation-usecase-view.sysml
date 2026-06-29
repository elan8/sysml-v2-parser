package MetadataAnnotationBodies {
    use case def MyCaseWithAnnotation {
        @SomeMetaclass;
    }

    view def MyViewWithAnnotation {
        @AnotherMetaclass;
    }

    calc def MyCalcWithAnnotation {
        @YetAnotherMetaclass;
    }
}
