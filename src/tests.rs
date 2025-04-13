#[cfg(test)]
mod tests {
    use crate::Agent;
    use crate::Brain;
    use crate::train_population;


    #[test]

    fn test_iterative_traing(){
        let mut population: Vec<Agent> = (0..100)
            .map(|_| Agent {
                brain: Brain::random(),
                fitness: 0.0,
                score : 0,
            })
            .collect();
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let mut elites : Vec<f32> = Vec::new(); 
        for i in 0..5{
            elites.push(population[0].fitness.clone());
            population = train_population(&mut population);
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        }

        let mut sorted_vec = elites.clone();
        sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(elites, sorted_vec);
    }
}