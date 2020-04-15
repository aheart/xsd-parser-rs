use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::Annotation;
use crate::xsd_model::Include;
use roxmltree::Node;

impl<'a> Include<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Include<'a>, String> {
        let mut res = Include::default();
        if let Some(child) = node.first_element_child() {
            if child.xsd_type()? != ElementType::Annotation {
                return Err(format!("Invalid content type of xsd:include: {:?}", node));
            } else {
                res.annotation = Some(Annotation::parse(child)?);
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "schemaLocation" => res.schema_location = attr.into(),
                "id" => res.id = Some(attr.into()),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::elements::include::Include;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<include id="ID" schemaLocation="http://uri" b='a'>
                    <annotation>
                        <appInfo>Some appinfo</appInfo>
                        <documentation>Some doc2</documentation>
                    </annotation>
            </include>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Include::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().documentations.len(), 1);
        assert_eq!(res.annotation.unwrap().app_infos.len(), 1);
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.schema_location.0, "http://uri");
    }
}