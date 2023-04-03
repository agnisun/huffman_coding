use std::{collections::{HashMap}};

#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    char: Option<char>,
    weight: usize,
}

fn concat_nodes(left: TreeNode, right: TreeNode) -> TreeNode {
    let weight = left.weight + right.weight;
    
    TreeNode {
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        char: None,
        weight,
    }
}

fn create_node(char: Option<char>, size: usize) -> TreeNode {
    TreeNode {
        left: None,
        right: None,
        char,
        weight: size
    }
}

fn generate_freq(string: &str) -> HashMap<char, usize> {
    let mut map:HashMap<char, usize> = HashMap::new();
    
    for char in string.chars() {
        map.entry(char).and_modify(|counter| *counter += 1).or_insert(1);
    }
    
    map
}

fn make_code_tree(freq: &HashMap<char, usize>) -> TreeNode {
    let freq_collection = freq.iter().collect::<Vec<(&char, &usize)>>();

    let mut min_heap:Vec<TreeNode> = Vec::new();

    for (char, size) in freq_collection {
        let node = create_node(Some(char.clone()), size.clone());

        min_heap.push(node);
    }

    while min_heap.len() > 2 {
        min_heap.sort_by(|a, b| (&(b.weight)).cmp(&(a.weight)));
        let right = min_heap.pop().unwrap();
        let left = min_heap.pop().unwrap();

        let node = concat_nodes(left, right);

        min_heap.push(node);
    }

    let right =  min_heap.pop().unwrap();
    let left =  min_heap.pop().unwrap();

    concat_nodes(left, right)
}

fn assign_codes(tree: &TreeNode, codes: &mut HashMap<char, String>, s: String) {
    if let Some(char) = tree.char {
        codes.insert(char, s);
    } else {
        if let Some(ref left) = &tree.left {
            self::assign_codes(left, codes, s.clone() + "0");
        } 
        
        if let Some(ref right) = &tree.right {
            self::assign_codes(right, codes, s.clone() + "1");
        }
    }
}

fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut result = String::from("");

    for char in s.chars() {
        let t = h.get(&char).unwrap();
        result.push_str(t);
    }

    result
}

fn decode_string(s: &str, tree: &TreeNode) ->String {
    let mut result = String::from("");
    let mut current_root:&TreeNode = tree;

    for b in s.chars() {
        if b == '0' {
            if let Some(ref left) = current_root.left {
                current_root = left;
            }
        } else {
            if let Some(ref right) = current_root.right {
                current_root = right;
            }
        }

        if let Some(char) = current_root.char {
            result.push(char);
            current_root = tree;
        }
    }

    result
}

fn main() {
    let s = "abracadabra";
    let freq = generate_freq(&s);
    let code_tree = make_code_tree(&freq);
    let mut codes:HashMap<char, String> = HashMap::new();

    assign_codes(&code_tree, &mut codes, String::from(""));
    let encoded = encode_string(&s, &codes);
    let decoded = decode_string(&encoded, &code_tree);

    println!("{:#?}", codes);
    println!("{:#?}", code_tree);
    println!("{}", encoded);
    println!("{}", decoded);
}
