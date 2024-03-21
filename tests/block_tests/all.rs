use mini_markdown::blocks::{CommonmarkBlock, LeafBlock, ContainerBlock};

#[test]
fn test_simple_recursive_render() {
	let test_block = ContainerBlock::BlockQuote(vec![Box::new(LeafBlock::Paragraph("foobar".to_string()))]);
	assert_eq!(test_block.render(),"<blockquote>\n<p>foobar</p>\n</blockquote>");
}