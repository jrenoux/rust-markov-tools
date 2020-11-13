use float_cmp::*;
use crate::mdp::MDP;

////////////////////////////////////////////////////////////////////////////
// Structure Definition                                                   //
////////////////////////////////////////////////////////////////////////////
pub struct SimpleMDP {
    pub states:usize,
    pub actions:usize,
    pub transitions:Vec<Vec<Vec<f32>>>,
    pub reward:Vec<Vec<Vec<f32>>>,
    pub discount:f32,
    _private: ()
}


////////////////////////////////////////////////////////////////////////////
// SimpleMDP Structure Implementation                                     //
////////////////////////////////////////////////////////////////////////////
impl SimpleMDP {
    pub fn new(nb_states:usize, nb_actions:usize, transitions:Vec<Vec<Vec<f32>>>, reward:Vec<Vec<Vec<f32>>>, discount:f32) -> SimpleMDP{
	// some sanity checks on the creation of the MDP

	// check transition function size
	if transitions.len() != nb_states {
	    panic!("Transition function has size {} for {} states", transitions.len(), nb_states);
	}

	for a_element in &transitions {
	    if a_element.len() != nb_actions {
		panic!("Transition function has size {} for {} actions", a_element.len(), nb_actions);
	    }
	    for s_element in a_element {
		if s_element.len() != nb_states {
		    panic!("Transition function has size {} for {} states", s_element.len(), nb_states);
		}
		let mut sum = 0.0;
		for trans in s_element {
		    sum = sum + trans
		}
		if  !approx_eq!(f32, sum, 1.0, ulps=4) {
		    panic!("Transitions function not properly formed, does not sum to 1");
		}
	    }
	}

	// reward shape is correct
	if reward.len() != nb_states {
	    panic!("Reward function has size {} for {} states", reward.len(), nb_states);
	}
	for a_element in &reward  {
	    if a_element.len() != nb_actions {
		panic!("Reward function has size {} for {} actions", a_element.len(), nb_actions);
	    }
	    for s_element in a_element {
		if s_element.len() != nb_states {
		    panic!("Reward function has size {} for {} states", s_element.len(), nb_states);
		}
	    }
	}

	SimpleMDP {
	    states: nb_states,
	    actions: nb_actions,
	    transitions: transitions.clone(),
	    reward: reward.clone(),
	    discount: discount,
	    _private : (),		
	}
	
    }
}


////////////////////////////////////////////////////////////////////////////
// Traits Implementation                                                  //
////////////////////////////////////////////////////////////////////////////
impl MDP for SimpleMDP {
    fn get_nb_states(&self) -> usize {
	self.states
    }

    fn get_nb_actions(&self) -> usize {
	self.actions
    }
    
    fn get_discount_factor(&self) -> f32 {
	self.discount
    }
    fn get_reward(&self, s1: usize, a: usize, s2: usize) -> f32 {
	self.reward[s1][a][s2]
    }
    fn get_transition_probabilitiy(&self, s1: usize, a: usize, s2: usize) -> f32 {
	self.transitions[s1][a][s2]
    }
}



////////////////////////////////////////////////////////////////////////////
// Unit Tests                                                             //
////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test{
    use super::*;

    fn create_mdp() -> SimpleMDP{
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95)
    }
    
    #[test]
    fn creation_ok() {
	create_mdp();
    }

    #[test]
    #[should_panic]
    fn creation_wrong_transition_function_size_1() {
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.], vec![0.2, 0.8]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }

    #[test]
    #[should_panic]
    fn creation_wrong_transition_function_size_2() {
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.], vec![0.2, 0.8]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }

    #[test]
    #[should_panic]
    fn creation_wrong_transition_function_size_3() {
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]],  //s1 = 2
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }

    #[test]
    #[should_panic]
    fn creation_wrong_transition_function_size_4() {
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }

    #[test]
    #[should_panic]
    fn creation_wrong_transition_function_not_sum_1(){
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.8], vec![1., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }

    #[test]
    #[should_panic]
    fn creation_wrong_reward_size() {
	let states = 2;
	let actions = 3;
	let transitions = vec![
	    vec![vec![0.2, 0.8], vec![0.4, 0.6], vec![0., 1.]],  //s1 = 1
	    vec![vec![0.8, 0.2], vec![0.3, 0.7], vec![1., 0.]]   //s1 = 2
	];
	let rewards = vec![
	    vec![vec![1., 1.], vec![1., 1.], vec![1., 1.], vec![0., 0.]],
	    vec![vec![0., 0.], vec![0., 0.], vec![0., 0.]]
	];

	SimpleMDP::new(states, actions, transitions, rewards, 0.95);
    }
}
