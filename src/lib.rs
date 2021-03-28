#[cfg(test)]
mod tests {
    use crate::render_to_html;
    
    #[test]
    fn test_headings() {
        let heading_tests = vec![
            ("<h1>Heading level 1</h1>", "# Heading level 1"),
            ("<h2>Heading level 2</h2>", "## Heading level 2"),
            ("<h3>Heading level 3</h3>", "### Heading level 3"),
            ("<h4>Heading level 4</h4>", "#### Heading level 4"),
            ("<h5>Heading level 5</h5>", "##### Heading level 5"),
            ("<h6>Heading level 6</h6>", "###### Heading level 6"),
        ];
        for test in heading_tests.iter(){
            println!("Testing: {} -> {}", test.1, test.0);
            assert_eq!(test.0, render_to_html(test.1));
        }
    }
}

pub fn render_to_html(source: &str) -> String {
    let mut output = String::new();
    for line in source.lines() {
        if line.starts_with("#"){
            output.push_str(&render_heading(line))
        }
    }
    output
}

fn render_heading(line: &str) -> String{
    let mut output = String::new();
    let hashes: String = line.chars().take_while(|x| x == &'#').collect();
    let lead_heading = "<h".to_owned()+&hashes.len().to_string()+&">".to_owned();
    let end_heading = "</h".to_owned()+&hashes.len().to_string()+&">".to_owned();
    let prefix = hashes + " ";
    output.push_str(&lead_heading);
    output.push_str(line.strip_prefix(&prefix).unwrap_or(""));
    output.push_str(&end_heading);
    output
}