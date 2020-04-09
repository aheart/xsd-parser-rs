use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// itemType	        [0..1]	xsd:QName
pub struct List<'a> {
    annotation: Option<Annotation<'a>>,
    simple_type: Option<Box<LocalSimpleType<'a>>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    item_type: Option<QName<'a>>
}