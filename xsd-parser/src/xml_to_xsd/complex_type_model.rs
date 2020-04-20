use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use crate::xsd_model::SimpleContent;
use roxmltree::Node;

impl<'a> ComplexTypeModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let first_child = node
            .first_element_child()
            .ok_or_else(|| format!("Content xsd:complexTypeModel required: {:?}", node))?;

        let type_def_particle;

        match first_child.xsd_type()? {
            ElementType::SimpleContent => {
                return Ok(Self::SimpleContent(SimpleContent::parse(first_child)?))
            }
            // ElementType::ComplexContent => Self::ComplexContent(ComplexContent::parse(first_child)?)
            x => type_def_particle = TypeDefParticle::parse(first_child, x)?,
        };

        let skip = if type_def_particle.is_some() { 1 } else { 0 };

        let res = ComplexTypeModel::Content(
            type_def_particle,
            AttrDecls::parse(node.element_children().skip(skip))?,
        );
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
    use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
    use crate::xsd_model::{MaxOccurs, SimpleContentChoice};

    #[test]
    fn test_parse_simple_content() {
        let doc = roxmltree::Document::parse(
            r#"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:simpleContent>
                            <xs:annotation>
								<xs:documentation>Text</xs:documentation>
							</xs:annotation>
	                        <xs:extension base="xs:string" a='a' b='b'>
        	                    <xs:attribute ref="xml:lang" use="required"/>
        	                </xs:extension>
        	            </xs:simpleContent>
                    </xs:complexType>
                 "#,
        )
        .unwrap();

        let root = doc.root_element();
        if let ComplexTypeModel::SimpleContent(sc) = ComplexTypeModel::parse(root).unwrap() {
            assert!(sc.annotation.is_some());
            assert_eq!(
                sc.annotation
                    .unwrap()
                    .documentations
                    .first()
                    .unwrap()
                    .text
                    .unwrap(),
                "Text"
            );

            if let SimpleContentChoice::Extension(ext) = sc.content {
                assert_eq!(ext.base.name, "string");
                assert_eq!(ext.base.prefix.unwrap(), "xs");
                assert_eq!(ext.attributes.len(), 2);
                assert_eq!(ext.attr_decls.attributes.len(), 1);
            } else {
                panic!("Test failed!");
            }
        } else {
            panic!("Test failed!");
        }
    }

    #[test]
    fn test_parse_complex_content() {
        let doc = roxmltree::Document::parse(
            r###"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                            <xs:complexContent>
                                <xs:annotation>
                                    <xs:documentation>Text</xs:documentation>
                                </xs:annotation>
                                <xs:restriction base="xs:anyType">
                                    <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                                    <xs:attribute name="Attr2" type="xs:anyURI"/>
                                    <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                                </xs:restriction>
                            </xs:complexContent>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
    } //TODO: finish him!

    #[test]
    fn test_parse_content() {
        let doc = roxmltree::Document::parse(
            r###"
                     <xs:complexType name="reasontext" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                         <xs:choice minOccurs="2" maxOccurs="5"/>
                         <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                         <xs:attribute name="Attr2" type="xs:anyURI"/>
                         <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                         <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:complexType>
                 "###,
        )
        .unwrap();

        let root = doc.root_element();
        if let ComplexTypeModel::Content(type_def, attr_decls) =
            ComplexTypeModel::parse(root).unwrap()
        {
            if let TypeDefParticle::Choice(ch) = type_def.unwrap() {
                assert_eq!(ch.min_occurs.0, 2);
                if let MaxOccurs::Bounded(x) = ch.max_occurs {
                    assert_eq!(x.0, 5);
                } else {
                    panic!()
                }
            } else {
                panic!()
            }

            assert_eq!(attr_decls.attributes.len(), 3);
            assert_eq!(attr_decls.any_attribute.unwrap().process_contents, "lax");
        } else {
            panic!()
        }
    }
}
