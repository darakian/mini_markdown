use mini_markdown::iter::MiniIter;

#[test]
fn iter_test(){
    let some_chars = "jkfsgbkfgbdklfdsbh gkhsdfbg";
    let mut foo = MiniIter::new(&some_chars);
    println!("{:?}", foo.peek());
    foo.next_if_eq(&"j");
    let s = foo.consume_while_case_holds(&|c| c != " ");
    println!("{:?}", s);
    for i in foo {
        println!("{:?}", i)
    };

    let some_other_chars = "jkfsgbkfgbdklfdsbh gkhsdfbg <details> and more chars";
    let mut foo2 = MiniIter::new(&some_other_chars);
    let s2 = foo2.consume_until_tail_is("<details>");
    let s3 = foo2.consume_until_end();
    println!("> {:?}", s2);
    println!("> {:?}", s3);
}