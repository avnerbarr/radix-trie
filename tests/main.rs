use std::collections::HashSet;
use ab_radix_trie::Trie;

#[test]
fn basic_tests() {
    let mut trie: Trie<i32> = Trie::new();
    trie.insert("romanus", None);
    trie.insert("romulus", Some(10));
    trie.insert("rubens", None);
    trie.insert("ruber", None);
    trie.insert("rubicon", None);
    trie.insert("rubicundus", None);

    let results = trie.get_suffixes_values("rom").unwrap();
    let entries = results.into_iter().map(|x| x.key).collect::<HashSet<_>>();
    println!("entries => {:?}", entries);
    assert_eq!(entries.len(), 2);
    assert!(entries.contains("anus"));
    assert!(entries.contains("ulus"));


    // should do nothing
    trie.remove("rom");

    let results = trie.get_suffixes_values("rom").unwrap();
    let entries = results.into_iter().map(|x| x.key).collect::<HashSet<_>>();
    assert_eq!(entries.len(), 2);
    assert!(entries.contains("anus"));
    assert!(entries.contains("ulus"));

    trie.remove("romanus");


    let results = trie.get_suffixes_values("rom").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.first().unwrap().key.as_str(), "ulus");
    assert_eq!(results.first().unwrap().val.unwrap(), 10);
}

#[test]
fn test_cloning() {
    let mut trie1: Trie<i32> = Trie::new();
    trie1.insert("romanus", None);
    trie1.insert("romulus", Some(10));
    trie1.insert("rubens", None);
    trie1.insert("ruber", None);
    trie1.insert("rubicon", None);
    trie1.insert("rubicundus", None);

    let mut trie = trie1.clone();
    let results = trie.get_suffixes_values("rom").unwrap();
    let entries = results.into_iter().map(|x| x.key).collect::<HashSet<_>>();
    println!("entries => {:?}", entries);
    assert_eq!(entries.len(), 2);
    assert!(entries.contains("anus"));
    assert!(entries.contains("ulus"));


    // should do nothing
    trie.remove("rom");

    let results = trie.get_suffixes_values("rom").unwrap();
    let entries = results.into_iter().map(|x| x.key).collect::<HashSet<_>>();
    assert_eq!(entries.len(), 2);
    assert!(entries.contains("anus"));
    assert!(entries.contains("ulus"));

    trie.remove("romanus");


    let results = trie.get_suffixes_values("rom").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.first().unwrap().key.as_str(), "ulus");
    assert_eq!(results.first().unwrap().val.unwrap(), 10);

}

#[test]
fn test_fuzzy() {
    use ab_radix_trie::MatchingOptions;
    let mut trie: Trie<String> = Trie::new();
    trie.insert("romanus", None);
    trie.insert("rom anus", None);

    let results = trie.get_suffixes_with_matching_options("roma", &MatchingOptions::ignoring_white_space());
    // should return "romanus" and "rom anus"
    println!("{:?}", results);
}