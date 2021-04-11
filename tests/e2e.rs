use mini_markdown::render;

#[test]
fn test_markdown_render() {
    let mut tests = Vec::new();
    tests.extend(vec![
        ("# Heading level 1", "<h1>Heading level 1</h1>"),
        ("## Heading level 2", "<h2>Heading level 2</h2>"),
        ("### Heading level 3", "<h3>Heading level 3</h3>"),
        ("#### Heading level 4", "<h4>Heading level 4</h4>"),
        ("##### Heading level 5", "<h5>Heading level 5</h5>"),
        ("###### Heading level 6", "<h6>Heading level 6</h6>"),
        ("####### Invalid Heading level 7", "<h6>Invalid Heading level 7</h6>"), 
    ]);

    for test in tests.iter(){
        let html = render(test.0);
        assert_eq!(html, test.1);
    }
}