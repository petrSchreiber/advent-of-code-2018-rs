pub fn get_common_chars(ids: &std::vec::Vec<String>) -> String {
    let ids_count = ids.len();

    for reference_index in 0..ids_count {
        for compared_index in reference_index..ids_count {
            let info = get_match_info(&ids[reference_index], &ids[compared_index]);

            if info.is_match {
                let mut common = ids[reference_index].clone();
                common.remove(info.differ_index);

                return common.to_string()
            }
        }
    }

    "<no common letters>".to_string()
}

#[test]
fn test_get_common_chars() {
    let ids = vec!["fghij".to_string(), "fghuj".to_string()];
    assert_eq!(get_common_chars(&ids), "fghj");
}

struct MatchInfo {
    pub is_match: bool,
    pub differ_index: usize
}

fn get_match_info(reference_id: &String, compared_id: &String) -> MatchInfo {
    let mut info = MatchInfo { is_match: false, differ_index: 0 };
    let mut index = 0;
    
    let mut faulty = 0;
    let mut differ_index = 0;

    for c in reference_id.chars() {
        if compared_id.chars().nth(index) != Some(c) {
                faulty += 1;
                if faulty > 1 {
                    info.is_match = false;
                    return info;
                }
                else
                {
                    differ_index = index;
                }
        }

        index += 1;
    }

    if faulty == 1 {
        info.is_match = true;
        info.differ_index = differ_index;
    }
    
    info
}

#[test]
fn test_get_match_info() {
    let match_info = get_match_info(&"abc".to_string(), &"adc".to_string());
    assert!(match_info.is_match == true && match_info.differ_index == 1);

    let match_info = get_match_info(&"abcd".to_string(), &"abde".to_string());
    assert!(match_info.is_match == false);

    let match_info = get_match_info(&"abc".to_string(), &"def".to_string());
    assert!(match_info.is_match == false);    
}
