use rand::Rng;
pub struct PartitioningProblem{
    wights: Vec<usize>,
    piles: u32,
    pop_size: u32,
    max_gen: u32,
    cx_prob: f32,
    mut_prob: f32,
    mut_flip_prob: f32,
    repeats: u32
}

impl PartitioningProblem {
    pub fn new(
        wights: Vec<usize>,
        piles: u32,
        pop_size: u32,
        max_gen: u32,
        cx_prob: f32,
        mut_prob: f32,
        mut_flip_prob: f32,
        repeats: u32
    ) -> Self {
        PartitioningProblem {
            wights,
            piles,
            pop_size,
            max_gen,
            cx_prob,
            mut_prob,
            mut_flip_prob,
            repeats
        }
    }

    pub fn set_wights(&mut self, wights: Vec<usize>) {
        self.wights = wights;
    }

    pub fn create_individual(&self, ind_len: usize)-> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut individual = Vec::new();
        for _ in 0..ind_len {
            individual.push(rng.gen_range(0..self.piles));
        }
        individual
    }

    pub fn create_population(&self, ind_len: usize) -> Vec<Vec<u32>> {
        let mut population = Vec::new();
        for _ in 0..self.pop_size {
            population.push(self.create_individual(ind_len));
        }
        population
    }

    pub fn fitness_objective(&self, individual: &Vec<u32>, bins: &Vec<usize>) -> (usize, usize) {
        let bin_weights = self.bin_weights(bins);
        let max_bin_weight = bin_weights.iter().max().unwrap();
        let min_bin_weight = bin_weights.iter().min().unwrap();
        let fitness = 1/((max_bin_weight - min_bin_weight).pow(2));
        let objective = max_bin_weight - min_bin_weight;
        (fitness, objective)
    }

    pub fn bin_weights(&self, bins: &Vec<usize>)-> Vec<usize> {
        let mut bin_weights = Vec::new();
        for _ in 0..self.piles {
            bin_weights.push(0);
        }
        for (i, bin) in bins.iter().enumerate() {
            bin_weights[*bin] += self.wights[i];
        }
        bin_weights
    }


}