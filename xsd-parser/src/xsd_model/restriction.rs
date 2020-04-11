use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};
use crate::xsd_model::common_groups::{AttrDecls, SimpleRestrictionModel, TypeDefParticle};

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      xsd:simpleType [0..1]        from group xsd:simpleRestrictionModel
//      Choice [0..*]                from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// base	            [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
pub struct SimpleTypeRestriction<'a> {
    annotation: Option<Annotation<'a>>,
    model: SimpleRestrictionModel<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: Option<QName<'a>>
}


// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleRestrictionType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Sequence [0..1]                 from group xsd:simpleRestrictionModel
//          xsd:simpleType [0..1]
//          Choice [0..*]               from group xsd:facets
//              xsd:minExclusive
//              xsd:minInclusive
//              xsd:maxExclusive
//              xsd:maxInclusive
//              xsd:totalDigits
//              xsd:fractionDigits
//              xsd:length
//              xsd:minLength
//              xsd:maxLength
//              xsd:enumeration
//              xsd:whiteSpace
//              xsd:pattern
//      Choice [0..*]                   from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:simpleContent
pub struct SimpleRestrictionType<'a> {
    annotation: Option<Annotation<'a>>,
    model: SimpleRestrictionModel<'a>,
    attr_decls: AttrDecls<'a>,
}


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:complexRestrictionType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..1]               from group xsd:typeDefParticle
//          xsd:group
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//          xsd:choice
//          xsd:sequence
//      Choice [0..*]           from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:complexContent
pub struct ComplexRestrictionType<'a> {
    annotation: Option<Annotation<'a>>,
    type_def_particle: TypeDefParticle<'a>,
    attr_decls: AttrDecls<'a>,
}