#[derive(Debug)]
struct Arena{
    next_id: usize
}

impl Arena{
    fn new() -> Self{
        Arena{ next_id: 0 }
    }
}

#[derive(Debug)]
struct Node{
    id: usize,
    end: usize,
    cost: usize,
    prev: Option<usize>,
    text: String,
}

impl Node{
    fn new(arena: &mut Arena,end: usize,cost: usize,prev: Option<usize>,text: String) -> Self{
        let ret = Node{ id: arena.next_id,end: end,cost: cost,prev: prev,text: text };
        arena.next_id += 1;
        ret
    }
}

struct Morpheme{
    yomi: Vec<char>,
    text: String,
    cost: usize
}

fn viterbi(morphemes: &Vec<Morpheme>,input: &str) -> String{
    let input = input.chars().collect::<Vec<_>>();

    let mut arena = Arena::new();
    let mut nodes = vec![];

    nodes.push(Node::new(&mut arena,0,0,None,"start".into()));

    for i in 0..input.len(){
        let mut new_nodes = vec![];
        for ref morpheme in morphemes.iter().filter(|&morpheme| morpheme.yomi[0] == input[i]){
            let mut node = Node::new(&mut arena,i + morpheme.yomi.len(),std::usize::MAX,None,morpheme.text.clone());
            for prev in nodes.iter().filter(|&node| node.end == i){
                if prev.cost + morpheme.cost < node.cost{
                    node.cost = prev.cost + morpheme.cost;
                    node.prev = Some(prev.id);
                }
            }
            if node.prev.is_some(){
                new_nodes.push(node);
            }
        }
        nodes.append(&mut new_nodes);
    }

    let mut last_id = None;
    let mut cost = std::usize::MAX;
    for last in nodes.iter().filter(|&node| node.end == input.len()){
        if last.cost < cost{
            last_id = Some(last.id);
            cost = last.cost;
        }
    }

    println!("{:?}",nodes);

    match last_id{
        None => panic!("単語分割に失敗"),
        Some(mut id) => {
            let mut ret = "".into();
            while id != 0{
                let node = nodes.iter().filter(|&node| node.id == id).next().unwrap();
                ret = format!("{} {}",node.text,ret);
                id = node.prev.unwrap();
            }
            ret
        }
    }
}

fn main() {
    let mut words = Vec::new();
    words.push(Morpheme{ yomi: "みず".chars().collect(), text: "水".into(), cost: 20 });
    words.push(Morpheme{ yomi: "み".chars().collect(), text: "身".into(), cost: 25 });
    words.push(Morpheme{ yomi: "さいかい".chars().collect(), text: "再開".into(), cost: 30 });
    words.push(Morpheme{ yomi: "さいかい".chars().collect(), text: "最下位".into(), cost: 20 });
    words.push(Morpheme{ yomi: "さいかい".chars().collect(), text: "再会".into(), cost: 10 });
    words.push(Morpheme{ yomi: "よろこぶ".chars().collect(), text: "喜ぶ".into(), cost: 15 });
    words.push(Morpheme{ yomi: "みずから".chars().collect(), text: "自ら".into(), cost: 25 });
    words.push(Morpheme{ yomi: "か".chars().collect(), text: "か".into(), cost: 20 });
    words.push(Morpheme{ yomi: "から".chars().collect(), text: "から".into(), cost: 10 });
    words.push(Morpheme{ yomi: "を".chars().collect(), text: "を".into(), cost: 10 });
    println!("{}",viterbi(&words,"みずからさいかいをよろこぶ"));
}
