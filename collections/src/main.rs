use std::collections::{HashMap, HashSet};

fn main() {
    let vec = vec![1, 5, 2, 3, 4, 2, 4, 3, 1, 3];

    assert_eq!(get_median(&vec), Some(3));
    assert_eq!(get_median(&[]), None);

    assert_eq!(get_mode(&vec), Some(3));
    assert_eq!(get_mode(&[]), None);

    assert_eq!(convert_to_pig_latin("first"), Some("irst-fay".to_string()));
    assert_eq!(convert_to_pig_latin("apple"), Some("apple-hay".to_string()));
    assert_eq!(convert_to_pig_latin(""), None);
    assert_eq!(convert_to_pig_latin("안녕"), Some("녕-안ay".to_string()));

    let mut company = Company::new();

    company.command("add Engineering Alice");
    company.command("add Engineering Bob");
    company.command("add Sales Charlie");

    company.command("get -d Engineering");
    company.command("get -d Sales");
    company.command("get -d None");

    company.command("get -a");
}

fn get_median(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut arr = arr.to_vec();
    arr.sort();

    let mid = arr.len() / 2;

    Some(if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) / 2
    } else {
        arr[mid]
    })
}

fn get_mode(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut hash_map: HashMap<i32, u32> = HashMap::new();

    for num in arr {
        let count = hash_map.entry(*num).or_insert(0);
        *count += 1;
    }

    hash_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}

fn convert_to_pig_latin(str: &str) -> Option<String> {
    if str.is_empty() {
        return None;
    }

    let mut chars = str.chars();
    let first_char = chars.next()?;

    Some(match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{str}-hay"),
        _ => {
            let rest: String = chars.collect();
            format!("{rest}-{first_char}ay")
        }
    })
}

#[derive(Default)]
struct Company(HashMap<String, HashSet<String>>);

impl Company {
    fn new() -> Self {
        Self::default()
    }

    fn add_person(&mut self, department: &str, name: &str) {
        let hash_map = &mut self.0;

        hash_map
            .entry(department.to_string())
            .or_default()
            .insert(name.to_string());
    }

    fn get_all_sorted_people(&self) -> Vec<&str> {
        let hash_map = &self.0;
        let mut people: Vec<&str> = hash_map.values().flatten().map(String::as_str).collect();
        people.sort();
        people
    }

    fn get_people_in_department(&self, department: &str) -> Option<Vec<&str>> {
        let hash_map = &self.0;

        hash_map.get(department).map(|s| {
            let mut people: Vec<&str> = s.iter().map(String::as_str).collect();
            people.sort();
            people
        })
    }

    // Add <department> <person>
    // Get [-a | -d <department>]
    fn command(&mut self, string: &str) {
        let parts: Vec<&str> = string.split_whitespace().collect();

        if parts.is_empty() {
            return;
        }

        let command = parts[0];

        if command.eq_ignore_ascii_case("add") {
            if let (Some(&department), Some(&person), None) =
                (parts.get(1), parts.get(2), parts.get(3))
            {
                self.add_person(department, person);
            }
        } else if command.eq_ignore_ascii_case("get") {
            match (parts.get(1), parts.get(2), parts.get(3)) {
                (Some(&"-a"), None, None) => {
                    println!("{:?}", self.get_all_sorted_people());
                }
                (Some(&"-d"), Some(&department), None) => {
                    match self.get_people_in_department(department) {
                        Some(arr) => {
                            println!("{:?}", arr);
                        }
                        None => {
                            println!("There is no person in {department}");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
