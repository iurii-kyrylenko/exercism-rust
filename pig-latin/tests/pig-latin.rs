use pig_latin as pl;

#[test]
fn test_word_beginning_with_a() {
    // #1
    assert_eq!(pl::translate("apple"), "appleay");
}

#[test]
#[ignore]
fn test_word_beginning_with_e() {
    // #1
    assert_eq!(pl::translate("ear"), "earay");
}

#[test]
#[ignore]
fn test_word_beginning_with_i() {
    // #1
    assert_eq!(pl::translate("igloo"), "iglooay");
}

#[test]
#[ignore]
fn test_word_beginning_with_o() {
    // #1
    assert_eq!(pl::translate("object"), "objectay");
}

#[test]
#[ignore]
fn test_word_beginning_with_u() {
    // #1
    assert_eq!(pl::translate("under"), "underay");
}

#[test]
#[ignore]
fn test_word_beginning_with_a_vowel_and_followed_by_a_qu() {
    // #1
    assert_eq!(pl::translate("equal"), "equalay");
}

#[test]
#[ignore]
fn test_word_beginning_with_p() {
    // #3
    assert_eq!(pl::translate("pig"), "igpay");
}

#[test]
#[ignore]
fn test_word_beginning_with_k() {
    // #3
    assert_eq!(pl::translate("koala"), "oalakay");
}

#[test]
#[ignore]
fn test_word_beginning_with_y() {
    // #2
    assert_eq!(pl::translate("yellow"), "ellowyay");
}

#[test]
#[ignore]
fn test_word_beginning_with_x() {
    // #2
    assert_eq!(pl::translate("xenon"), "enonxay");
}

#[test]
#[ignore]
fn test_word_beginning_with_q_without_a_following_u() {
    // #2
    assert_eq!(pl::translate("qat"), "atqay");
}

#[test]
#[ignore]
fn test_word_beginning_with_ch() {
    // #2
    assert_eq!(pl::translate("chair"), "airchay");
}

#[test]
#[ignore]
fn test_word_beginning_with_qu() {
    // #2
    assert_eq!(pl::translate("queen"), "eenquay");
}

#[test]
#[ignore]
fn test_word_beginning_with_qu_and_a_preceding_consonant() {
    // #3
    assert_eq!(pl::translate("square"), "aresquay");
}

#[test]
#[ignore]
fn test_word_beginning_with_th() {
    // #2
    assert_eq!(pl::translate("therapy"), "erapythay");
}

#[test]
#[ignore]
fn test_word_beginning_with_thr() {
    // #2
    assert_eq!(pl::translate("thrush"), "ushthray");
}

#[test]
#[ignore]
fn test_word_beginning_with_sch() {
    // #2
    assert_eq!(pl::translate("school"), "oolschay");
}

#[test]
#[ignore]
fn test_word_beginning_with_yt() {
    // #1
    assert_eq!(pl::translate("yttria"), "yttriaay");
}

#[test]
#[ignore]
fn test_word_beginning_with_xr() {
    // #1
    assert_eq!(pl::translate("xray"), "xrayay");
}

#[test]
#[ignore]
fn test_y_is_treated_like_a_vowel_at_the_end_of_a_consonant_cluster() {
    // #4, #2
    assert_eq!(pl::translate("rhythm"), "ythmrhay");
}

#[test]
#[ignore]
fn test_a_whole_phrase() {
    assert_eq!(pl::translate("quick fast run"), "ickquay astfay unray");
}
