use std::collections::HashMap;

pub fn group_by_hash(hashed: Vec<(String, String)>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (hash, path) in hashed {
        map.entry(hash).or_default().push(path);
    }

    for (hash, group) in &map {
        println!("Group for hash {}: {:?}", hash, group);  // Debugging line
    }

    map.into_iter()
        .filter_map(|(_, group)| if group.len() > 1 { Some(group) } else { None })
        .collect()
}
    
