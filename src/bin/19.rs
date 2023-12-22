use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input_parts = input.split("\n\n");
    let workflows: HashMap<String, Vec<WorkflowStep>> =
        input_parts
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::new(), |mut hm, line| {
                let mut wf_parts = line.split(&['{', '}']);

                let wf_id = wf_parts.next().unwrap().to_string();
                let wf_steps = wf_parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|step| {
                        let mut step_parts = step.split(&['<', '>', ':']);
                        if step.contains('>') {
                            let wf_type = match step_parts.next() {
                                Some("x") => WorkflowStepType::X,
                                Some("m") => WorkflowStepType::M,
                                Some("a") => WorkflowStepType::A,
                                Some("s") => WorkflowStepType::S,
                                _ => WorkflowStepType::Default,
                            };
                            let val = step_parts.next().unwrap().parse().unwrap();
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: wf_type,
                                condition: Condition::GT,
                                value: val,
                                result,
                            }
                        } else if step.contains('<') {
                            let wf_type = match step_parts.next() {
                                Some("x") => WorkflowStepType::X,
                                Some("m") => WorkflowStepType::M,
                                Some("a") => WorkflowStepType::A,
                                Some("s") => WorkflowStepType::S,
                                _ => WorkflowStepType::Default,
                            };
                            let val = step_parts.next().unwrap().parse().unwrap();
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: wf_type,
                                condition: Condition::LT,
                                value: val,
                                result,
                            }
                        } else {
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: WorkflowStepType::Default,
                                condition: Condition::None,
                                value: 0,
                                result,
                            }
                        }
                    })
                    .collect::<Vec<WorkflowStep>>();

                hm.insert(wf_id, wf_steps);

                hm
            });

    let parts = input_parts.next().unwrap().lines().map(|line| {
        let binding = line.replace(&['{', '}', '=', 'x', 'm', 'a', 's'], "");
        let mut part = binding.split(',').map(|n| n.parse::<usize>().unwrap());

        Part {
            x: part.next().unwrap(),
            m: part.next().unwrap(),
            a: part.next().unwrap(),
            s: part.next().unwrap(),
        }
    });

    let first_wf = "in".to_string();

    let mut accepted_parts: Vec<Part> = Vec::new();
    let mut rejected_parts: Vec<Part> = Vec::new();

    for part in parts {
        let mut next_wf = WorkflowResult::Next(first_wf.clone());

        loop {
            match next_wf {
                WorkflowResult::Accepted => {
                    accepted_parts.push(part);
                    break;
                }
                WorkflowResult::Rejected => {
                    rejected_parts.push(part);
                    break;
                }
                WorkflowResult::Next(ref wf_id) => {
                    let wf = workflows.get(wf_id).unwrap().iter();
                    for step in wf {
                        match step.step_type {
                            WorkflowStepType::X => match step.condition {
                                Condition::GT => {
                                    if part.x > step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::LT => {
                                    if part.x < step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::None => panic!("No condition"),
                            },
                            WorkflowStepType::M => match step.condition {
                                Condition::GT => {
                                    if part.m > step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::LT => {
                                    if part.m < step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::None => panic!("No condition"),
                            },
                            WorkflowStepType::A => match step.condition {
                                Condition::GT => {
                                    if part.a > step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::LT => {
                                    if part.a < step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::None => panic!("No condition"),
                            },
                            WorkflowStepType::S => match step.condition {
                                Condition::GT => {
                                    if part.s > step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::LT => {
                                    if part.s < step.value {
                                        next_wf = step.result.clone();
                                        break;
                                    }
                                }
                                Condition::None => panic!("No condition"),
                            },
                            WorkflowStepType::Default => {
                                next_wf = step.result.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(accepted_parts.iter().map(|p| p.x + p.m + p.a + p.s).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input_parts = input.split("\n\n");
    let workflows: HashMap<String, Vec<WorkflowStep>> =
        input_parts
            .next()
            .unwrap()
            .lines()
            .fold(HashMap::new(), |mut hm, line| {
                let mut wf_parts = line.split(&['{', '}']);

                let wf_id = wf_parts.next().unwrap().to_string();
                let wf_steps = wf_parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|step| {
                        let mut step_parts = step.split(&['<', '>', ':']);
                        if step.contains('>') {
                            let wf_type = match step_parts.next() {
                                Some("x") => WorkflowStepType::X,
                                Some("m") => WorkflowStepType::M,
                                Some("a") => WorkflowStepType::A,
                                Some("s") => WorkflowStepType::S,
                                _ => WorkflowStepType::Default,
                            };
                            let val = step_parts.next().unwrap().parse().unwrap();
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: wf_type,
                                condition: Condition::GT,
                                value: val,
                                result,
                            }
                        } else if step.contains('<') {
                            let wf_type = match step_parts.next() {
                                Some("x") => WorkflowStepType::X,
                                Some("m") => WorkflowStepType::M,
                                Some("a") => WorkflowStepType::A,
                                Some("s") => WorkflowStepType::S,
                                _ => WorkflowStepType::Default,
                            };
                            let val = step_parts.next().unwrap().parse().unwrap();
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: wf_type,
                                condition: Condition::LT,
                                value: val,
                                result,
                            }
                        } else {
                            let result = match step_parts.next() {
                                Some("A") => WorkflowResult::Accepted,
                                Some("R") => WorkflowResult::Rejected,
                                Some(key) => WorkflowResult::Next(key.to_string()),
                                None => panic!("unknown result"),
                            };
                            WorkflowStep {
                                step_type: WorkflowStepType::Default,
                                condition: Condition::None,
                                value: 0,
                                result,
                            }
                        }
                    })
                    .collect::<Vec<WorkflowStep>>();

                hm.insert(wf_id, wf_steps);

                hm
            });

    let mut accepted_windows: Vec<RatingWindow> = Vec::new();

    let first_wf = "in".to_string();
    let window = RatingWindow::new();

    let mut paths_to_explore: VecDeque<(WorkflowResult, RatingWindow)> = VecDeque::new();

    paths_to_explore.push_back((WorkflowResult::Next(first_wf), window));

    while let Some((next_wf, r_window)) = paths_to_explore.pop_front() {
        match next_wf {
            WorkflowResult::Accepted => {
                accepted_windows.push(r_window);
            }
            WorkflowResult::Rejected => continue,
            WorkflowResult::Next(ref wf_id) => {
                let wf = workflows.get(wf_id).unwrap().iter();
                let mut remaining_window = r_window.clone();
                for step in wf {
                    let mut new_window = remaining_window.clone();
                    match step.step_type {
                        WorkflowStepType::X => match step.condition {
                            Condition::GT => {
                                new_window.min_x = step.value + 1;
                                remaining_window.max_x = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }
                            Condition::LT => {
                                new_window.max_x = step.value - 1;
                                remaining_window.min_x = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }

                            Condition::None => panic!("No condition"),
                        },
                        WorkflowStepType::M => match step.condition {
                            Condition::GT => {
                                new_window.min_m = step.value + 1;
                                remaining_window.max_m = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }
                            Condition::LT => {
                                new_window.max_m = step.value - 1;
                                remaining_window.min_m = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }

                            Condition::None => panic!("No condition"),
                        },
                        WorkflowStepType::A => match step.condition {
                            Condition::GT => {
                                new_window.min_a = step.value + 1;
                                remaining_window.max_a = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }
                            Condition::LT => {
                                new_window.max_a = step.value - 1;
                                remaining_window.min_a = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }

                            Condition::None => panic!("No condition"),
                        },
                        WorkflowStepType::S => match step.condition {
                            Condition::GT => {
                                new_window.min_s = step.value + 1;
                                remaining_window.max_s = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }
                            Condition::LT => {
                                new_window.max_s = step.value - 1;
                                remaining_window.min_s = step.value;
                                if new_window.is_valid() {
                                    paths_to_explore.push_back((step.result.clone(), new_window));
                                }
                            }

                            Condition::None => panic!("No condition"),
                        },
                        WorkflowStepType::Default => {
                            paths_to_explore.push_back((step.result.clone(), new_window));
                        }
                    }
                }
            }
        }
    }

    Some(
        accepted_windows
            .iter()
            .map(|w| {
                (w.max_x - w.min_x + 1)
                    * (w.max_m - w.min_m + 1)
                    * (w.max_a - w.min_a + 1)
                    * (w.max_s - w.min_s + 1)
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}

#[derive(Clone)]
enum WorkflowResult {
    Accepted,
    Rejected,
    Next(String),
}

enum WorkflowStepType {
    X,
    M,
    A,
    S,
    Default,
}
enum Condition {
    GT,
    LT,
    None,
}
struct WorkflowStep {
    step_type: WorkflowStepType,
    condition: Condition,
    value: usize,
    result: WorkflowResult,
}

#[derive(Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Copy)]
struct RatingWindow {
    min_x: usize,
    max_x: usize,
    min_m: usize,
    max_m: usize,
    min_a: usize,
    max_a: usize,
    min_s: usize,
    max_s: usize,
}

impl RatingWindow {
    fn new() -> Self {
        Self {
            min_x: 1,
            max_x: 4000,
            min_m: 1,
            max_m: 4000,
            min_a: 1,
            max_a: 4000,
            min_s: 1,
            max_s: 4000,
        }
    }

    fn is_valid(&self) -> bool {
        self.min_x <= self.max_x
            && self.min_m <= self.max_m
            && self.min_a <= self.max_a
            && self.min_s <= self.max_s
    }
}
