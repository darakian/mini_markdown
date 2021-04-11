use mini_markdown::render;

#[test]
fn test_simple_render() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", "<h1>Heading level 1</h1>"),
        ("## Heading level 2", "<h2>Heading level 2</h2>"),
        ("### Heading level 3", "<h3>Heading level 3</h3>"),
        ("#### Heading level 4", "<h4>Heading level 4</h4>"),
        ("##### Heading level 5", "<h5>Heading level 5</h5>"),
        ("###### Heading level 6", "<h6>Heading level 6</h6>"),
        ("####### Invalid Heading level 7", "<h6>Invalid Heading level 7</h6>"),
        ("Some text _with italics_ in the same paragraph\n", "<p>Some text <em>with italics</em> in the same paragraph</p>"),
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}

// #[test]
// fn test_moderate_render(){
//     let mut tests = Vec::new();
//     tests.extend(vec![
//         ("Text attributes _italic_, 
//         **bold**, `monospace`. Some implementations may use *single-asterisks* for italic text.",
//         "<p>Text attributes <em>italic</em>, 
//         <strong>bold</strong>, <code>monospace</code>. Some implementations may use <i>single-asterisks</i> for italic text.</p>"),
//     ]);

//     for test in tests.iter(){
//         let html = render(test.0);
//         assert_eq!(html, test.1);
//     }
// }