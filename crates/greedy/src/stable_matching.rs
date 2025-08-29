use std::collections::{HashMap, VecDeque};


#[allow(unused)]
/// 初始化所有男性为自由状态，并设置他们下一个求婚对象的索引为0。
///
/// # 参数
/// * `men_preferences` - 每个男性的偏好列表，键为男性名字，值为其偏好女性的有序列表。
///
/// # 返回值
/// 返回一个元组：
/// - 第一个元素是自由男性的队列；
/// - 第二个元素是记录每个男性下一个求婚对象索引的哈希表。
fn initialize_men(
    men_preferences: &HashMap<String, Vec<String>>
) -> (VecDeque<String>, HashMap<String, usize>) {
    let mut free_men = VecDeque::new();
    let mut next_proposal = HashMap::new();
    for man in men_preferences.keys() {
        free_men.push_back(man.clone());
        next_proposal.insert(man.clone(), 0);
    }
    (free_men, next_proposal)
}

#[allow(unused)]
/// 初始化所有女性当前伴侣为None。
///
/// # 参数
/// * `women_preferences` - 每个女性的偏好列表，键为女性名字，值为其偏好男性的有序列表。
///
/// # 返回值
/// 返回一个哈希表，键为女性名字，值为Option<String>表示当前伴侣（None表示未配对）。
fn initialize_women(
    women_preferences: &HashMap<String, Vec<String>>
) -> HashMap<String, Option<String>> {
    let mut curr_partner = HashMap::new();
    for woman in women_preferences.keys() {
        curr_partner.insert(woman.clone(), None);
    }
    curr_partner
}

/// 预计算每个女性对各个男性的偏好排名。
///
/// # 参数
/// * `women_preferences` - 每个女性的偏好列表。
///
/// # 返回值
/// 返回一个嵌套哈希表，外层键为女性名字，内层键为男性名字，值为该男性的排名（数字越小越优先）。
fn precompute_woman_ranks(
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, usize>> {
    let mut woman_ranks = HashMap::new();
    for (woman, preferences) in women_preferences {
        let mut rank_map = HashMap::new();
        for (rank, man) in preferences.iter().enumerate() {
            rank_map.insert(man.clone(), rank);
        }
        woman_ranks.insert(woman.clone(), rank_map);
    }
    woman_ranks
}

/// 处理一个男性的求婚请求。
///
/// 根据男性当前的偏好列表和已记录的求婚进度，向其下一个最偏好的女性求婚。
/// 如果该女性已有伴侣，则比较她对新旧两个男性的偏好来决定是否更换伴侣。
///
/// # 参数
/// * `man` - 发起求婚的男性名字。
/// * `free_men` - 自由男性的队列，用于在求婚失败时重新加入队列。
/// * `curr_partner` - 当前女性伴侣映射。
/// * `man_engaged` - 记录男性是否已有伴侣。
/// * `next_proposal` - 每个男性下一次求婚对象的索引。
/// * `men_preferences` - 所有男性的偏好列表。
/// * `woman_ranks` - 女性对男性的偏好排名预计算结果。
fn process_proposal(
    man: &str,
    free_men: &mut VecDeque<String>,
    curr_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    next_proposal: &mut HashMap<String, usize>,
    men_preferences: &HashMap<String, Vec<String>>,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) {
    let man_pref_list = &men_preferences[man];
    let next_woman_idx = next_proposal[man];
    let woman = &man_pref_list[next_woman_idx];
    next_proposal.insert(man.to_string(), next_woman_idx + 1);
    if let Some(curr_man) = curr_partner[woman].clone() {
        if woman_prefers_new_man(woman, man, &curr_man, woman_ranks) {
            engage_man(
                man,
                woman,
                free_men,
                curr_partner,
                man_engaged,
                Some(curr_man),
            );
        } else {
            free_men.push_back(man.to_string());
        }
    } else {
        engage_man(man, woman, free_men, curr_partner, man_engaged, None);
    }
}

/// 判断女性是否更偏好新的男性而不是当前伴侣。
///
/// # 参数
/// * `woman` - 女性名字。
/// * `man1` - 新的求婚男性。
/// * `man2` - 当前伴侣男性。
/// * `woman_ranks` - 女性对男性的偏好排名。
///
/// # 返回值
/// 如果女性更偏好man1则返回true，否则返回false。
fn woman_prefers_new_man(
    woman: &str,
    man1: &str,
    man2: &str,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) -> bool {
    let ranks = &woman_ranks[woman];
    ranks[man1] < ranks[man2]
}

/// 建立或更新男性与女性之间的婚约关系。
///
/// 如果女性已有伴侣，则将原伴侣重新加入自由男性队列。
///
/// # 参数
/// * `man` - 男性名字。
/// * `woman` - 女性名字。
/// * `free_men` - 自由男性队列。
/// * `curr_partner` - 当前女性伴侣映射。
/// * `man_engaged` - 男性婚约状态映射。
/// * `curr_man` - 原来的伴侣（如果有的话）。
fn engage_man(
    man: &str,
    woman: &str,
    free_men: &mut VecDeque<String>,
    curr_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    curr_man: Option<String>,
) {
    man_engaged.insert(man.to_string(), Some(woman.to_string()));
    curr_partner.insert(woman.to_string(), Some(man.to_string()));
    if let Some(curr_man) = curr_man {
        free_men.push_back(curr_man);
    }
}

/// 将男性婚约状态转换为最终稳定匹配结果。
///
/// # 参数
/// * `man_engaged` - 男性婚约状态映射。
///
/// # 返回值
/// 返回最终的稳定匹配结果，键为男性名字，值为对应的女性名字。
fn finalize_matches(man_engaged: HashMap<String, Option<String>>) -> HashMap<String, String> {
    let mut stable_matches = HashMap::new();
    for (man, woman_option) in man_engaged {
        if let Some(woman) = woman_option {
            stable_matches.insert(man, woman);
        }
    }
    stable_matches
}

/// 执行稳定婚姻匹配算法（Gale-Shapley算法）。
///
/// 通过迭代地让自由男性向其偏好列表中的女性求婚，
/// 并根据女性偏好选择最优伴侣，直到所有男性都被匹配为止。
///
/// # 参数
/// * `men_preferences` - 所有男性的偏好列表。
/// * `women_preferences` - 所有女性的偏好列表。
///
/// # 返回值
/// 返回一个稳定匹配结果，键为男性名字，值为对应女性名字。
pub fn stable_matching(
    men_preferences: &HashMap<String, Vec<String>>,
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let (mut free_men, mut next_proposal) = initialize_men(men_preferences);
    let mut current_partner = initialize_women(women_preferences);
    let mut man_engaged = HashMap::new();

    let woman_ranks = precompute_woman_ranks(women_preferences);

    while let Some(man) = free_men.pop_front() {
        process_proposal(
            &man,
            &mut free_men,
            &mut current_partner,
            &mut man_engaged,
            &mut next_proposal,
            men_preferences,
            &woman_ranks,
        );
    }

    finalize_matches(man_engaged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_stable_matching_scenario_1() {
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["Y".to_string(), "X".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);

        let expected_matches1 = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        let expected_matches2 = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        assert!(matches == expected_matches1 || matches == expected_matches2);
    }

    #[test]
    fn test_stable_matching_empty() {
        let men_preferences = HashMap::new();
        let women_preferences = HashMap::new();

        let matches = stable_matching(&men_preferences, &women_preferences);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_stable_matching_duplicate_preferences() {
        let men_preferences = HashMap::from([
            ("A".to_string(), vec!["X".to_string(), "X".to_string()]), // Man with duplicate preferences
            ("B".to_string(), vec!["Y".to_string()]),
        ]);

        let women_preferences = HashMap::from([
            ("X".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("Y".to_string(), vec!["B".to_string()]),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
        ]);

        assert_eq!(matches, expected_matches);
    }

    #[test]
    fn test_stable_matching_single_pair() {
        let men_preferences = HashMap::from([("A".to_string(), vec!["X".to_string()])]);
        let women_preferences = HashMap::from([("X".to_string(), vec!["A".to_string()])]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([("A".to_string(), "X".to_string())]);

        assert_eq!(matches, expected_matches);
    }
    #[test]
    fn test_woman_prefers_new_man() {
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);

        let expected_matches = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        assert_eq!(matches, expected_matches);
    }
}
