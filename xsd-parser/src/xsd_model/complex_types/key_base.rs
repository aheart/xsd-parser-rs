use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::elements::field::Field;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::elements::selector::Selector;

// xsd:keybase
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      xsd:selector [1..1]
//      xsd:field [1..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used by
// Element xsd:key
// Element xsd:unique
// Element xsd:keyref via derived anonymous type
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:keybase
// extended by Anonymous type of element xsd:keyref
pub struct KeyBase<'a> {
    annotation: Option<Annotation<'a>>,
    selector: Selector<'a>,
    fields: Vec<Field<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: NCName<'a>,
}