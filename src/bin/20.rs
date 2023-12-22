use std::fmt::{Debug, Display, Write};
use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let mut conjunction_ids: Vec<String> = Vec::new();
    let mut mapping: HashMap<String, Module> =
        input.lines().fold(HashMap::new(), |mut acc, line| {
            let mut parts = line.split(" -> ");

            let mod_id = parts.next().unwrap();
            let module_type = match &mod_id[..1] {
                "b" => ModuleType::Broadcaster,
                "%" => ModuleType::FlipFlop,
                "&" => {
                    conjunction_ids.push(mod_id[1..].to_string());
                    ModuleType::Conjuction
                }
                _ => panic!("unknown module"),
            };

            let targets = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            acc.insert(
                mod_id[1..].to_string(),
                Module::new(mod_id[1..].to_string(), module_type, targets),
            );

            acc
        });

    for module in mapping.values() {
        for target in &module.targets {
            if conjunction_ids.contains(target) {
                let con = mapping.get(target).unwrap();
                let mut sources = con.sources.borrow_mut();
                sources.push(module.id.clone());
            }
        }
    }

    for con in conjunction_ids {
        let con = mapping.get_mut(&con).unwrap();

        for source in con.sources.borrow().iter() {
            con.con_state
                .insert(source.to_string(), Cell::new(Signal::Low));
        }
    }

    let mut count: usize = 0;
    let mut high_count: usize = 0;
    let mut low_count: usize = 0;

    for _ in 0..1000 {
        count += 1;

        let start = mapping.get("roadcaster").unwrap();
        let mut any_high_state = false;

        let mut processing_queue: VecDeque<(&Module, Signal, &Module)> = VecDeque::new();
        processing_queue.push_back((start, Signal::Low, start));
        while let Some((module, signal, source)) = processing_queue.pop_front() {
            match module.module_type {
                ModuleType::Broadcaster => {
                    low_count += 1;
                    for target in &module.targets {
                        low_count += 1;

                        match mapping.get(target) {
                            Some(t) => processing_queue.push_back((t, signal, module)),
                            None => (),
                        }
                    }
                }
                ModuleType::FlipFlop => {
                    if signal == Signal::Low {
                        module.ff_state.set(module.ff_state.get().flip());

                        for target in &module.targets {
                            match module.ff_state.get() {
                                Signal::High => {
                                    high_count += 1;
                                    any_high_state = true;
                                }
                                Signal::Low => low_count += 1,
                            }

                            match mapping.get(target) {
                                Some(t) => {
                                    processing_queue.push_back((t, module.ff_state.get(), module))
                                }
                                None => (),
                            }
                        }
                    }
                }
                ModuleType::Conjuction => {
                    module.con_state.get(&source.id).unwrap().set(signal);

                    if module
                        .con_state
                        .values()
                        .all(|state| state.get() == Signal::High)
                    {
                        for target in &module.targets {
                            low_count += 1;

                            match mapping.get(target) {
                                Some(t) => processing_queue.push_back((t, Signal::Low, module)),
                                None => (),
                            }
                        }
                    } else {
                        for target in &module.targets {
                            high_count += 1;

                            match mapping.get(target) {
                                Some(t) => processing_queue.push_back((t, Signal::High, module)),
                                None => (),
                            }
                        }
                    }

                    if module
                        .con_state
                        .values()
                        .any(|state| state.get() == Signal::High)
                    {
                        any_high_state = true;
                    }
                }
                ModuleType::Output => (),
            }
        }

        if !any_high_state {
            break;
        }
    }

    if count < 1000 {
        if 1000 % count == 0 {
            Some(high_count * 1000 / count * low_count * 1000 / count)
        } else {
            None
        }
    } else {
        Some(high_count * low_count)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut conjunction_ids: Vec<String> = Vec::new();
    let mut mapping: HashMap<String, Module> =
        input.lines().fold(HashMap::new(), |mut acc, line| {
            let mut parts = line.split(" -> ");

            let mod_id = parts.next().unwrap();
            let module_type = match &mod_id[..1] {
                "b" => ModuleType::Broadcaster,
                "%" => ModuleType::FlipFlop,
                "&" => {
                    conjunction_ids.push(mod_id[1..].to_string());
                    ModuleType::Conjuction
                }
                _ => panic!("unknown module"),
            };

            let targets = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            acc.insert(
                mod_id[1..].to_string(),
                Module::new(mod_id[1..].to_string(), module_type, targets),
            );

            acc
        });

    mapping.insert("rx".to_string(), Module::new("rx".to_string(), ModuleType::Output, Vec::new()));
    mapping.insert("output".to_string(), Module::new("output".to_string(), ModuleType::Output, Vec::new()));

    for module in mapping.values() {
        for target in &module.targets {
            if conjunction_ids.contains(target) {
                let con = mapping.get(target).unwrap();
                let mut sources = con.sources.borrow_mut();
                sources.push(module.id.clone());
            }
        }
    }

    let mut gate_ids: Vec<String> = Vec::new();
    let mut con_lows = HashMap::new();

    for con_id in conjunction_ids {
        con_lows.insert(con_id.clone(), Vec::<usize>::new());

        let con = mapping.get_mut(&con_id).unwrap();

        for source in con.sources.borrow().iter() {
            con.con_state
                .insert(source.to_string(), Cell::new(Signal::Low));
        }

        if con.sources.borrow().len() > 1 {
            gate_ids.push(con_id);
        }
    }
    
    
    
    let mut count: usize = 0;

    for _ in 0..5000 {
        count += 1;

        let start = mapping.get("roadcaster").unwrap();
        let mut any_high_state = false;

        let mut operations: Vec<(&Module, Signal, &Module)> = Vec::new();

        let mut processing_queue: VecDeque<(&Module, Signal, &Module)> = VecDeque::new();
        processing_queue.push_back((start, Signal::Low, start));
        operations.push((start, Signal::Low, start));
        while let Some((module, signal, source)) = processing_queue.pop_front() {
            match module.module_type {
                ModuleType::Broadcaster => {
                    for target in &module.targets {
                        match mapping.get(target) {
                            Some(t) => {
                                operations.push((t, signal, module));
                                processing_queue.push_back((t, signal, module))
                            }
                            None => (),
                        }
                    }
                }
                ModuleType::FlipFlop => {
                    if signal == Signal::Low {
                        module.ff_state.set(module.ff_state.get().flip());
                        match module.ff_state.get() {
                            Signal::High => {
                                any_high_state = true;
                            }
                            Signal::Low => (),
                        }

                        for target in &module.targets {
                            match mapping.get(target) {
                                Some(t) => {
                                    operations.push((t, module.ff_state.get(), module));
                                    processing_queue.push_back((t, module.ff_state.get(), module))
                                }
                                None => (),
                            }
                        }
                    }
                }
                ModuleType::Conjuction => {
                    module.con_state.get(&source.id).unwrap().set(signal);

                    if module
                        .con_state
                        .values()
                        .all(|state| state.get() == Signal::High)
                    {
                        for target in &module.targets {
                            match mapping.get(target) {
                                Some(t) => {
                                    operations.push((t, Signal::Low, module));
                                    let low_out = con_lows.get_mut(&module.id).unwrap();
                                    low_out.push(count);

                                    processing_queue.push_back((t, Signal::Low, module))
                                }
                                None => (),
                            }
                        }
                    } else {
                        for target in &module.targets {
                            match mapping.get(target) {
                                Some(t) => {
                                    operations.push((t, Signal::High, module));
                                    processing_queue.push_back((t, Signal::High, module))
                                }
                                None => (),
                            }
                        }
                    }

                    if module
                        .con_state
                        .values()
                        .any(|state| state.get() == Signal::High)
                    {
                        any_high_state = true;
                    }
                }
                ModuleType::Output => (),
            }
        }
        // println!("{:?}", con_lows.iter().filter(|(k,_)| {
        //     gate_ids.contains(k) && k != &"lx"
        // }).collect::<Vec<_>>());


        if !any_high_state {
            break;
        }
    }

    con_lows.iter().filter_map(|(k,v)| {
        if gate_ids.contains(k) && k != &"lx" {
            Some(*v.first().unwrap())
        }else{
            None
        }
    }).reduce(|acc, elem| acc * elem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Signal {
    High,
    Low,
}
impl Display for Signal{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::High => f.write_char('H'),
            Signal::Low => f.write_char('L'),
        }
    }
}

impl Signal {
    fn flip(&self) -> Self {
        match self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,
        }
    }
}

enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjuction,
    Output,
}

struct Module {
    id: String,
    module_type: ModuleType,
    sources: RefCell<Vec<String>>,
    targets: Vec<String>,
    ff_state: Cell<Signal>,
    con_state: HashMap<String, Cell<Signal>>,
}

impl Module {
    fn new(id: String, module_type: ModuleType, targets: Vec<String>) -> Self {
        Self {
            id,
            module_type,
            sources: RefCell::new(Vec::new()),
            targets,
            ff_state: Cell::new(Signal::Low),
            con_state: HashMap::new(),
        }
    }
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}