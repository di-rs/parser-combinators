use parser_combinators::parse_xml;

fn main() {
    let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;
    let parsed_doc = parse_xml(doc);
    println!("{:?}", parsed_doc);
}
