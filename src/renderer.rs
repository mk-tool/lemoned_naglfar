use std::iter;
use std::io::BufReader;
use std::default::Default;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::rcdom::{Handle, NodeData, RcDom};

// Reference
// [Rustでスクレイピング(html5ever)](https://qiita.com/nacika_ins/items/c618c503cdc0080c7db8)

// scanning the node
fn walk(indent: usize, node: Handle) {

    fn escape_default(s: &str) -> String {
        // chars() returns an iterator
        s.chars().flat_map(|c| c.escape_default()).collect()
    }

    // express indent
    print!("{}", iter::repeat(" ").take(indent).collect::<String>());

    match node.data {
        NodeData::Document => println!("#Document"),
        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
        NodeData::Text { ref contents } => {
            println!("#text: {}", escape_default(&contents.borrow()))
        }
        NodeData::Comment { ref contents } => println!("<!-- {} -->", escape_default(contents)),
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    // children returns the `Handle` type value.
    // `Handle` implements Clone to get another reference to the same node
    // [todo]Does the borrow method return `Array` ?
    for child in node.children.borrow().iter() {
        walk(indent + 2, child.clone()); // 2 is indent space
    }
}

pub fn f() {
    let text = "
<html>
<body>
hello
<font color=\"red\">web</font>
world
!!
</body>
</html>
"
        .to_owned(); // convert &str to String
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut BufReader::new(text.as_bytes()))
        .unwrap();
    walk(0, dom.document); // dom.document is `Handle` type and we manipulate DOM from this.
}