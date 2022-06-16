use mini_markdown::render;


#[test]
fn commonmark_test_350_emphasis_and_strong_emphasis() {
    let test_html = render("*foo bar*\n");
    let reference_html = "<p><em>foo bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_351_emphasis_and_strong_emphasis() {
    let test_html = render("a * foo bar*\n");
    let reference_html = "<p>a * foo bar*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_352_emphasis_and_strong_emphasis() {
    let test_html = render("a*\"foo\"*\n");
    let reference_html = "<p>a*&quot;foo&quot;*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_353_emphasis_and_strong_emphasis() {
    let test_html = render("* a *\n");
    let reference_html = "<p>* a *</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_354_emphasis_and_strong_emphasis() {
    let test_html = render("foo*bar*\n");
    let reference_html = "<p>foo<em>bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_355_emphasis_and_strong_emphasis() {
    let test_html = render("5*6*78\n");
    let reference_html = "<p>5<em>6</em>78</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_356_emphasis_and_strong_emphasis() {
    let test_html = render("_foo bar_\n");
    let reference_html = "<p><em>foo bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_357_emphasis_and_strong_emphasis() {
    let test_html = render("_ foo bar_\n");
    let reference_html = "<p>_ foo bar_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_358_emphasis_and_strong_emphasis() {
    let test_html = render("a_\"foo\"_\n");
    let reference_html = "<p>a_&quot;foo&quot;_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_359_emphasis_and_strong_emphasis() {
    let test_html = render("foo_bar_\n");
    let reference_html = "<p>foo_bar_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_360_emphasis_and_strong_emphasis() {
    let test_html = render("5_6_78\n");
    let reference_html = "<p>5_6_78</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_361_emphasis_and_strong_emphasis() {
    let test_html = render("пристаням_стремятся_\n");
    let reference_html = "<p>пристаням_стремятся_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_362_emphasis_and_strong_emphasis() {
    let test_html = render("aa_\"bb\"_cc\n");
    let reference_html = "<p>aa_&quot;bb&quot;_cc</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_363_emphasis_and_strong_emphasis() {
    let test_html = render("foo-_(bar)_\n");
    let reference_html = "<p>foo-<em>(bar)</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_364_emphasis_and_strong_emphasis() {
    let test_html = render("_foo*\n");
    let reference_html = "<p>_foo*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_365_emphasis_and_strong_emphasis() {
    let test_html = render("*foo bar *\n");
    let reference_html = "<p>*foo bar *</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_366_emphasis_and_strong_emphasis() {
    let test_html = render("*foo bar\n*\n");
    let reference_html = "<p>*foo bar\n*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_367_emphasis_and_strong_emphasis() {
    let test_html = render("*(*foo)\n");
    let reference_html = "<p>*(*foo)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_368_emphasis_and_strong_emphasis() {
    let test_html = render("*(*foo*)*\n");
    let reference_html = "<p><em>(<em>foo</em>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_369_emphasis_and_strong_emphasis() {
    let test_html = render("*foo*bar\n");
    let reference_html = "<p><em>foo</em>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_370_emphasis_and_strong_emphasis() {
    let test_html = render("_foo bar _\n");
    let reference_html = "<p>_foo bar _</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_371_emphasis_and_strong_emphasis() {
    let test_html = render("_(_foo)\n");
    let reference_html = "<p>_(_foo)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_372_emphasis_and_strong_emphasis() {
    let test_html = render("_(_foo_)_\n");
    let reference_html = "<p><em>(<em>foo</em>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_373_emphasis_and_strong_emphasis() {
    let test_html = render("_foo_bar\n");
    let reference_html = "<p>_foo_bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_374_emphasis_and_strong_emphasis() {
    let test_html = render("_пристаням_стремятся\n");
    let reference_html = "<p>_пристаням_стремятся</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_375_emphasis_and_strong_emphasis() {
    let test_html = render("_foo_bar_baz_\n");
    let reference_html = "<p><em>foo_bar_baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_376_emphasis_and_strong_emphasis() {
    let test_html = render("_(bar)_.\n");
    let reference_html = "<p><em>(bar)</em>.</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_377_emphasis_and_strong_emphasis() {
    let test_html = render("**foo bar**\n");
    let reference_html = "<p><strong>foo bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_378_emphasis_and_strong_emphasis() {
    let test_html = render("** foo bar**\n");
    let reference_html = "<p>** foo bar**</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_379_emphasis_and_strong_emphasis() {
    let test_html = render("a**\"foo\"**\n");
    let reference_html = "<p>a**&quot;foo&quot;**</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_380_emphasis_and_strong_emphasis() {
    let test_html = render("foo**bar**\n");
    let reference_html = "<p>foo<strong>bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_381_emphasis_and_strong_emphasis() {
    let test_html = render("__foo bar__\n");
    let reference_html = "<p><strong>foo bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_382_emphasis_and_strong_emphasis() {
    let test_html = render("__ foo bar__\n");
    let reference_html = "<p>__ foo bar__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_383_emphasis_and_strong_emphasis() {
    let test_html = render("__\nfoo bar__\n");
    let reference_html = "<p>__\nfoo bar__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_384_emphasis_and_strong_emphasis() {
    let test_html = render("a__\"foo\"__\n");
    let reference_html = "<p>a__&quot;foo&quot;__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_385_emphasis_and_strong_emphasis() {
    let test_html = render("foo__bar__\n");
    let reference_html = "<p>foo__bar__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_386_emphasis_and_strong_emphasis() {
    let test_html = render("5__6__78\n");
    let reference_html = "<p>5__6__78</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_387_emphasis_and_strong_emphasis() {
    let test_html = render("пристаням__стремятся__\n");
    let reference_html = "<p>пристаням__стремятся__</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_388_emphasis_and_strong_emphasis() {
    let test_html = render("__foo, __bar__, baz__\n");
    let reference_html = "<p><strong>foo, <strong>bar</strong>, baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_389_emphasis_and_strong_emphasis() {
    let test_html = render("foo-__(bar)__\n");
    let reference_html = "<p>foo-<strong>(bar)</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_390_emphasis_and_strong_emphasis() {
    let test_html = render("**foo bar **\n");
    let reference_html = "<p>**foo bar **</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_391_emphasis_and_strong_emphasis() {
    let test_html = render("**(**foo)\n");
    let reference_html = "<p>**(**foo)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_392_emphasis_and_strong_emphasis() {
    let test_html = render("*(**foo**)*\n");
    let reference_html = "<p><em>(<strong>foo</strong>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_393_emphasis_and_strong_emphasis() {
    let test_html = render("**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**\n");
    let reference_html = "<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.\n<em>Asclepias physocarpa</em>)</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_394_emphasis_and_strong_emphasis() {
    let test_html = render("**foo \"*bar*\" foo**\n");
    let reference_html = "<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_395_emphasis_and_strong_emphasis() {
    let test_html = render("**foo**bar\n");
    let reference_html = "<p><strong>foo</strong>bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_396_emphasis_and_strong_emphasis() {
    let test_html = render("__foo bar __\n");
    let reference_html = "<p>__foo bar __</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_397_emphasis_and_strong_emphasis() {
    let test_html = render("__(__foo)\n");
    let reference_html = "<p>__(__foo)</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_398_emphasis_and_strong_emphasis() {
    let test_html = render("_(__foo__)_\n");
    let reference_html = "<p><em>(<strong>foo</strong>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_399_emphasis_and_strong_emphasis() {
    let test_html = render("__foo__bar\n");
    let reference_html = "<p>__foo__bar</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_400_emphasis_and_strong_emphasis() {
    let test_html = render("__пристаням__стремятся\n");
    let reference_html = "<p>__пристаням__стремятся</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_401_emphasis_and_strong_emphasis() {
    let test_html = render("__foo__bar__baz__\n");
    let reference_html = "<p><strong>foo__bar__baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_402_emphasis_and_strong_emphasis() {
    let test_html = render("__(bar)__.\n");
    let reference_html = "<p><strong>(bar)</strong>.</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_403_emphasis_and_strong_emphasis() {
    let test_html = render("*foo [bar](/url)*\n");
    let reference_html = "<p><em>foo <a href=\"/url\">bar</a></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_404_emphasis_and_strong_emphasis() {
    let test_html = render("*foo\nbar*\n");
    let reference_html = "<p><em>foo\nbar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_405_emphasis_and_strong_emphasis() {
    let test_html = render("_foo __bar__ baz_\n");
    let reference_html = "<p><em>foo <strong>bar</strong> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_406_emphasis_and_strong_emphasis() {
    let test_html = render("_foo _bar_ baz_\n");
    let reference_html = "<p><em>foo <em>bar</em> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_407_emphasis_and_strong_emphasis() {
    let test_html = render("__foo_ bar_\n");
    let reference_html = "<p><em><em>foo</em> bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_408_emphasis_and_strong_emphasis() {
    let test_html = render("*foo *bar**\n");
    let reference_html = "<p><em>foo <em>bar</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_409_emphasis_and_strong_emphasis() {
    let test_html = render("*foo **bar** baz*\n");
    let reference_html = "<p><em>foo <strong>bar</strong> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_410_emphasis_and_strong_emphasis() {
    let test_html = render("*foo**bar**baz*\n");
    let reference_html = "<p><em>foo<strong>bar</strong>baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_411_emphasis_and_strong_emphasis() {
    let test_html = render("*foo**bar*\n");
    let reference_html = "<p><em>foo**bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_412_emphasis_and_strong_emphasis() {
    let test_html = render("***foo** bar*\n");
    let reference_html = "<p><em><strong>foo</strong> bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_413_emphasis_and_strong_emphasis() {
    let test_html = render("*foo **bar***\n");
    let reference_html = "<p><em>foo <strong>bar</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_414_emphasis_and_strong_emphasis() {
    let test_html = render("*foo**bar***\n");
    let reference_html = "<p><em>foo<strong>bar</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_415_emphasis_and_strong_emphasis() {
    let test_html = render("foo***bar***baz\n");
    let reference_html = "<p>foo<em><strong>bar</strong></em>baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_416_emphasis_and_strong_emphasis() {
    let test_html = render("foo******bar*********baz\n");
    let reference_html = "<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_417_emphasis_and_strong_emphasis() {
    let test_html = render("*foo **bar *baz* bim** bop*\n");
    let reference_html = "<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_418_emphasis_and_strong_emphasis() {
    let test_html = render("*foo [*bar*](/url)*\n");
    let reference_html = "<p><em>foo <a href=\"/url\"><em>bar</em></a></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_419_emphasis_and_strong_emphasis() {
    let test_html = render("** is not an empty emphasis\n");
    let reference_html = "<p>** is not an empty emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_420_emphasis_and_strong_emphasis() {
    let test_html = render("**** is not an empty strong emphasis\n");
    let reference_html = "<p>**** is not an empty strong emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_421_emphasis_and_strong_emphasis() {
    let test_html = render("**foo [bar](/url)**\n");
    let reference_html = "<p><strong>foo <a href=\"/url\">bar</a></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_422_emphasis_and_strong_emphasis() {
    let test_html = render("**foo\nbar**\n");
    let reference_html = "<p><strong>foo\nbar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_423_emphasis_and_strong_emphasis() {
    let test_html = render("__foo _bar_ baz__\n");
    let reference_html = "<p><strong>foo <em>bar</em> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_424_emphasis_and_strong_emphasis() {
    let test_html = render("__foo __bar__ baz__\n");
    let reference_html = "<p><strong>foo <strong>bar</strong> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_425_emphasis_and_strong_emphasis() {
    let test_html = render("____foo__ bar__\n");
    let reference_html = "<p><strong><strong>foo</strong> bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_426_emphasis_and_strong_emphasis() {
    let test_html = render("**foo **bar****\n");
    let reference_html = "<p><strong>foo <strong>bar</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_427_emphasis_and_strong_emphasis() {
    let test_html = render("**foo *bar* baz**\n");
    let reference_html = "<p><strong>foo <em>bar</em> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_428_emphasis_and_strong_emphasis() {
    let test_html = render("**foo*bar*baz**\n");
    let reference_html = "<p><strong>foo<em>bar</em>baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_429_emphasis_and_strong_emphasis() {
    let test_html = render("***foo* bar**\n");
    let reference_html = "<p><strong><em>foo</em> bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_430_emphasis_and_strong_emphasis() {
    let test_html = render("**foo *bar***\n");
    let reference_html = "<p><strong>foo <em>bar</em></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_431_emphasis_and_strong_emphasis() {
    let test_html = render("**foo *bar **baz**\nbim* bop**\n");
    let reference_html = "<p><strong>foo <em>bar <strong>baz</strong>\nbim</em> bop</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_432_emphasis_and_strong_emphasis() {
    let test_html = render("**foo [*bar*](/url)**\n");
    let reference_html = "<p><strong>foo <a href=\"/url\"><em>bar</em></a></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_433_emphasis_and_strong_emphasis() {
    let test_html = render("__ is not an empty emphasis\n");
    let reference_html = "<p>__ is not an empty emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_434_emphasis_and_strong_emphasis() {
    let test_html = render("____ is not an empty strong emphasis\n");
    let reference_html = "<p>____ is not an empty strong emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_435_emphasis_and_strong_emphasis() {
    let test_html = render("foo ***\n");
    let reference_html = "<p>foo ***</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_436_emphasis_and_strong_emphasis() {
    let test_html = render("foo *\\**\n");
    let reference_html = "<p>foo <em>*</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_437_emphasis_and_strong_emphasis() {
    let test_html = render("foo *_*\n");
    let reference_html = "<p>foo <em>_</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_438_emphasis_and_strong_emphasis() {
    let test_html = render("foo *****\n");
    let reference_html = "<p>foo *****</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_439_emphasis_and_strong_emphasis() {
    let test_html = render("foo **\\***\n");
    let reference_html = "<p>foo <strong>*</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_440_emphasis_and_strong_emphasis() {
    let test_html = render("foo **_**\n");
    let reference_html = "<p>foo <strong>_</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_441_emphasis_and_strong_emphasis() {
    let test_html = render("**foo*\n");
    let reference_html = "<p>*<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_442_emphasis_and_strong_emphasis() {
    let test_html = render("*foo**\n");
    let reference_html = "<p><em>foo</em>*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_443_emphasis_and_strong_emphasis() {
    let test_html = render("***foo**\n");
    let reference_html = "<p>*<strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_444_emphasis_and_strong_emphasis() {
    let test_html = render("****foo*\n");
    let reference_html = "<p>***<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_445_emphasis_and_strong_emphasis() {
    let test_html = render("**foo***\n");
    let reference_html = "<p><strong>foo</strong>*</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_446_emphasis_and_strong_emphasis() {
    let test_html = render("*foo****\n");
    let reference_html = "<p><em>foo</em>***</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_447_emphasis_and_strong_emphasis() {
    let test_html = render("foo ___\n");
    let reference_html = "<p>foo ___</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_448_emphasis_and_strong_emphasis() {
    let test_html = render("foo _\\__\n");
    let reference_html = "<p>foo <em>_</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_449_emphasis_and_strong_emphasis() {
    let test_html = render("foo _*_\n");
    let reference_html = "<p>foo <em>*</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_450_emphasis_and_strong_emphasis() {
    let test_html = render("foo _____\n");
    let reference_html = "<p>foo _____</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_451_emphasis_and_strong_emphasis() {
    let test_html = render("foo __\\___\n");
    let reference_html = "<p>foo <strong>_</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_452_emphasis_and_strong_emphasis() {
    let test_html = render("foo __*__\n");
    let reference_html = "<p>foo <strong>*</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_453_emphasis_and_strong_emphasis() {
    let test_html = render("__foo_\n");
    let reference_html = "<p>_<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_454_emphasis_and_strong_emphasis() {
    let test_html = render("_foo__\n");
    let reference_html = "<p><em>foo</em>_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_455_emphasis_and_strong_emphasis() {
    let test_html = render("___foo__\n");
    let reference_html = "<p>_<strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_456_emphasis_and_strong_emphasis() {
    let test_html = render("____foo_\n");
    let reference_html = "<p>___<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_457_emphasis_and_strong_emphasis() {
    let test_html = render("__foo___\n");
    let reference_html = "<p><strong>foo</strong>_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_458_emphasis_and_strong_emphasis() {
    let test_html = render("_foo____\n");
    let reference_html = "<p><em>foo</em>___</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_459_emphasis_and_strong_emphasis() {
    let test_html = render("**foo**\n");
    let reference_html = "<p><strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_460_emphasis_and_strong_emphasis() {
    let test_html = render("*_foo_*\n");
    let reference_html = "<p><em><em>foo</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_461_emphasis_and_strong_emphasis() {
    let test_html = render("__foo__\n");
    let reference_html = "<p><strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_462_emphasis_and_strong_emphasis() {
    let test_html = render("_*foo*_\n");
    let reference_html = "<p><em><em>foo</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_463_emphasis_and_strong_emphasis() {
    let test_html = render("****foo****\n");
    let reference_html = "<p><strong><strong>foo</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_464_emphasis_and_strong_emphasis() {
    let test_html = render("____foo____\n");
    let reference_html = "<p><strong><strong>foo</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_465_emphasis_and_strong_emphasis() {
    let test_html = render("******foo******\n");
    let reference_html = "<p><strong><strong><strong>foo</strong></strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_466_emphasis_and_strong_emphasis() {
    let test_html = render("***foo***\n");
    let reference_html = "<p><em><strong>foo</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_467_emphasis_and_strong_emphasis() {
    let test_html = render("_____foo_____\n");
    let reference_html = "<p><em><strong><strong>foo</strong></strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_468_emphasis_and_strong_emphasis() {
    let test_html = render("*foo _bar* baz_\n");
    let reference_html = "<p><em>foo _bar</em> baz_</p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_469_emphasis_and_strong_emphasis() {
    let test_html = render("*foo __bar *baz bim__ bam*\n");
    let reference_html = "<p><em>foo <strong>bar *baz bim</strong> bam</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_470_emphasis_and_strong_emphasis() {
    let test_html = render("**foo **bar baz**\n");
    let reference_html = "<p>**foo <strong>bar baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_471_emphasis_and_strong_emphasis() {
    let test_html = render("*foo *bar baz*\n");
    let reference_html = "<p>*foo <em>bar baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_472_emphasis_and_strong_emphasis() {
    let test_html = render("*[bar*](/url)\n");
    let reference_html = "<p>*<a href=\"/url\">bar*</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_473_emphasis_and_strong_emphasis() {
    let test_html = render("_foo [bar_](/url)\n");
    let reference_html = "<p>_foo <a href=\"/url\">bar_</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_474_emphasis_and_strong_emphasis() {
    let test_html = render("*<img src=\"foo\" title=\"*\"/>\n");
    let reference_html = "<p>*<img src=\"foo\" title=\"*\"/></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_475_emphasis_and_strong_emphasis() {
    let test_html = render("**<a href=\"**\">\n");
    let reference_html = "<p>**<a href=\"**\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_476_emphasis_and_strong_emphasis() {
    let test_html = render("__<a href=\"__\">\n");
    let reference_html = "<p>__<a href=\"__\"></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_477_emphasis_and_strong_emphasis() {
    let test_html = render("*a `*`*\n");
    let reference_html = "<p><em>a <code>*</code></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_478_emphasis_and_strong_emphasis() {
    let test_html = render("_a `_`_\n");
    let reference_html = "<p><em>a <code>_</code></em></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_479_emphasis_and_strong_emphasis() {
    let test_html = render("**a<http://foo.bar/?q=**>\n");
    let reference_html = "<p>**a<a href=\"http://foo.bar/?q=**\">http://foo.bar/?q=**</a></p>\n";
    assert_eq!(test_html, reference_html);
}


#[test]
fn commonmark_test_480_emphasis_and_strong_emphasis() {
    let test_html = render("__a<http://foo.bar/?q=__>\n");
    let reference_html = "<p>__a<a href=\"http://foo.bar/?q=__\">http://foo.bar/?q=__</a></p>\n";
    assert_eq!(test_html, reference_html);
}


