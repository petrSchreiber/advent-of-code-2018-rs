pub fn get_vector_checksum(ids: &std::vec::Vec<String>) -> usize {
    let mut doubles: usize = 0;
    let mut triples: usize = 0;

    for id in ids {
        let single_checksum = get_id_info(id.to_string());
        doubles += single_checksum.has_double as usize;
        triples += single_checksum.has_triple as usize;
    }

    doubles * triples
}

#[test]
fn test_get_vector_checksum() {
    let ids = vec!["abcdef".to_string(),
                   "bababc".to_string(),
                   "abbcde".to_string(),
                   "abcccd".to_string(),
                   "aabcdd".to_string(),
                   "abcdee".to_string(),
                   "ababab".to_string()];

    let checksum = get_vector_checksum(&ids);
    assert_eq!(checksum, 12)
}

struct IdInfo {
    pub has_double: bool,
    pub has_triple: bool
}

fn get_id_info(id: String) -> IdInfo {
    let mut result = IdInfo {has_double: false, has_triple: false};

    for c in id.chars() {     
        match id.matches(c).count() {
            2 => result.has_double = true,
            3 => result.has_triple = true,
            _ => continue
        };
    }
    result
}


#[test]
fn test_get_id_info() {
    let checksum_1 = get_id_info("abcdef".to_string());
    assert!(checksum_1.has_double == false && checksum_1.has_triple == false);

    let checksum_2 = get_id_info("bababc".to_string());
    assert!(checksum_2.has_double == true && checksum_2.has_triple == true);

    let checksum_3 = get_id_info("abbcde".to_string());
    assert!(checksum_3.has_double == true && checksum_3.has_triple == false);

    let checksum_4 = get_id_info("abcccd".to_string());
    assert!(checksum_4.has_double == false && checksum_4.has_triple == true);

    let checksum_5 = get_id_info("aabcdd".to_string());
    assert!(checksum_5.has_double == true && checksum_5.has_triple == false);

    let checksum_6 = get_id_info("abcdee".to_string());
    assert!(checksum_6.has_double == true && checksum_6.has_triple == false);

    let checksum_7 = get_id_info("ababab".to_string());
    assert!(checksum_7.has_double == false && checksum_7.has_triple == true);
}
