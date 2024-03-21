use mini_markdown::blocks::{CommonmarkBlock, LeafBlock, ContainerBlock};

#[test]
fn test_simple_recursive_render() {
	let test_block = ContainerBlock::BlockQuote(vec![Box::new(LeafBlock::Paragraph("foobar".to_string()))]);
	assert_eq!(test_block.render(),"<blockquote>\n<p>foobar</p>\n</blockquote>");
}

#[test]
fn test_simple_recursive_render2() {
	let test_block = ContainerBlock::BlockQuote(vec![
		Box::new(LeafBlock::ATXHeading(1, "foobar".to_string())),
		Box::new(LeafBlock::Paragraph("bizbaz".to_string()))
		]);
	assert_eq!(test_block.render(),"<blockquote>\n<h1>foobar</h1>\n<p>bizbaz</p>\n</blockquote>");
}