use crate::Problem;

pub fn ida<P, A, H>(mut state: P, heuristic: H) -> Option<Vec<P>>
    where P: Problem<A>,
          A: Clone,
          H: Fn(&P) -> usize {
    let mut nub: usize = heuristic(&state);
    let mut path = vec![state.clone()];
    let mut nb_visited_nodes = 0;

    loop {
        let ub = nub;
        nub = usize::max_value();

        print!("upper bound: {}", ub);
        let res = search(&mut state, ub, &mut nub, &mut path, &heuristic, &mut nb_visited_nodes);
        println!(" ; nb_visited_state: {}", nb_visited_nodes);

        if res.is_some() {
            return res;
        }
        if nub == usize::max_value() {
            return None;
        }
    }
}

fn search<P, A, H>(
    state: &mut P,
    ub: usize,
    nub: &mut usize,
    path: &mut Vec<P>,
    heuristic: &H,
    nb_visited_state: &mut usize,
) -> Option<Vec<P>>
    where P: Problem<A>,
          A: Clone,
          H: Fn(&P) -> usize
{
    *nb_visited_state += 1;
    let g = path.len() - 1;

    if state.is_solution() {
        return Some(path.clone());
    }

    let mut neighbors: Vec<(A, usize)> = Vec::new();
    for action in state.actions() {
        let c = state.clone();
        state.do_movement(&action);
        if !path.contains(state) {
            neighbors.push((action.clone(), heuristic(state)))
        }
        state.undo_movement(&action);
        if &c != state { panic!(); }
    }

    neighbors.sort_by_key(|it| it.1);

    for (movement, cost) in neighbors {
        let f = g + 1 + cost;
        if f > ub {
            if f < *nub {
                *nub = f;
            }
        } else {
            state.do_movement(&movement);
            path.push(state.clone());
            let res = search(state, ub, nub, path, heuristic, nb_visited_state);
            path.pop();
            state.undo_movement(&movement);
            if res.is_some() { return res; }
        }
    }

    None
}

