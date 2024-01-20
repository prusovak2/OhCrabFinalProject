use rand::Rng;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::prelude::ThreadRng;
use rand::prelude::StdRng;
use log::{info, warn};
use log::LevelFilter;
use std::io::Read;
use itertools::Itertools;

pub struct PartitioningProblem{
    weights: Vec<u32>,
    piles: u32,
    pop_size: usize,
    max_gen: u32,
    cx_prob: f32,
    mut_prob: f32,
    mut_flip_prob: f32,
    repeats: u32
}

impl PartitioningProblem {
    pub fn new(
        weights: Vec<u32>,
        piles: u32,
        pop_size: usize,
        max_gen: u32,
        cx_prob: f32,
        mut_prob: f32,
        mut_flip_prob: f32,
        repeats: u32
    ) -> Self {
        PartitioningProblem {
            weights,
            piles,
            pop_size,
            max_gen,
            cx_prob,
            mut_prob,
            mut_flip_prob,
            repeats
        }
    }

    pub fn set_weights(&mut self, weights: Vec<u32>) {
        self.weights = weights;
    }

    pub fn set_weights_from_file(&mut self, file_name: &str) {
        let mut weights = Vec::new();
        let mut file = std::fs::File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            weights.push(line.parse::<u32>().unwrap());
        }
        self.weights = weights;
    }

    fn create_individual(&self, ind_len: usize)-> Vec<usize> {
        // individual is represented as a vector of indexes
        let mut rng = rand::thread_rng();
        let mut individual =  Vec::with_capacity(ind_len);
        for _ in 0..ind_len {
            individual.push(rng.gen_range(0..self.piles) as usize);
        }
        individual
    }

    fn create_population(&self, ind_len: usize) -> Vec<Vec<usize>> {
        let mut population = Vec::with_capacity(self.pop_size);
        for _ in 0..self.pop_size {
            population.push(self.create_individual(ind_len));
        }
        population
    }

    fn fitness_objective(&self, individual: &Vec<usize>) -> (f32, u32) { // bins: &Vec<usize>
        let bin_weights : Vec<u32> = self.bin_weights(individual);
        let max_bin_weight : u32 = *bin_weights.iter().max().unwrap();
        let min_bin_weight : u32= *bin_weights.iter().min().unwrap();
        let fitness : f32= 1.0/((max_bin_weight as f32 - min_bin_weight as f32).powf(2.0));
        let objective : u32 = max_bin_weight - min_bin_weight;
        (fitness, objective)
    }

    fn objective(&self, individual: &Vec<usize>) -> u32{
        let bin_weights : Vec<u32> = self.bin_weights(individual);
        let max_bin_weight : u32 = *bin_weights.iter().max().unwrap();
        let min_bin_weight : u32= *bin_weights.iter().min().unwrap();
        max_bin_weight - min_bin_weight
    }

    fn bin_weights(&self, bins_individual: &Vec<usize>)-> Vec<u32> {
        // weights of given bins, aka markets in our case
        //TODO: This could be potentially beter inicialized
        let mut bin_weights = Vec::with_capacity(self.piles as usize);
        for _ in 0..self.piles {
            bin_weights.push(0);
        }
        for (i, bin) in bins_individual.iter().enumerate() {
            bin_weights[*bin] += self.weights[i];
        }
        bin_weights
    }

    fn tournament_selection(&self, population: &Vec<Vec<usize>>, fitness: &Vec<f32>) -> Vec<Vec<usize>> {
        let mut rng = &mut rand::thread_rng();
        let mut selected: Vec<Vec<usize>> = Vec::with_capacity(self.pop_size);
        for _ in 0..self.pop_size{
            //let v: Vec<_> = std::ops::Range{start:0, end: population.len()}.collect::<Vec<_>>().choose_multiple(&mut rng, 2).cloned().collect();
            let v: Vec<_> = (0..population.len()).collect::<Vec<_>>().choose_multiple(&mut rng, 2).cloned().collect();
            if fitness[v[0]] > fitness[v[1]] {
                selected.push(population[v[0]].clone());
            }
            else {
                selected.push(population[v[1]].clone());
            }
        }
        selected
    }

    fn one_point_crossover(&self, parent1: &Vec<usize>, parent2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        let mut rng = &mut rand::thread_rng();
        let crossover_point = rng.gen_range(0..parent1.len());
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();
        for i in crossover_point..parent1.len() {
            child1[i] = parent2[i];
            child2[i] = parent1[i];
        }
        (child1, child2)
    }

    fn flip_mutate(&self, individual: &mut Vec<usize>){ //-> Vec<usize>
        let mut rng = &mut rand::thread_rng();
        for value in individual.iter_mut() {
            if rng.gen::<f32>() < self.mut_flip_prob {
                *value = rng.gen_range(0..self.piles) as usize;
            }
        }
       // individual.to_vec()
    }

    pub fn crossover(&self, population: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut rng = &mut rand::thread_rng();
        let pop1: Vec<_> = population.iter().cloned().step_by(2).collect();
        let pop2: Vec<_> = population.iter().cloned().skip(1).step_by(2).collect();
        let mut new_population = Vec::new();
        for i in 0..pop1.len() {
            //let v: Vec<_> = std::ops::Range{start:0, end: population.len()}.collect::<Vec<_>>().choose_multiple(&mut rng, 2).cloned().collect();
            let parent1 = &pop1[i];
            let parent2 = &pop2[i];
            if rng.gen::<f32>() < self.cx_prob {
                let (child1, child2) = self.one_point_crossover(parent1, parent2);
                new_population.push(child1);
                new_population.push(child2);
            }
            else {
                new_population.push(parent1.clone());
                new_population.push(parent2.clone());
            }
        }
        new_population
    }

    pub fn mutate(&self, population: &mut Vec<Vec<usize>>) {//-> Vec<Vec<usize>> {
        let mut rng = &mut rand::thread_rng();
        //let mut new_population = Vec::new();
        for individual in population.iter_mut() {
            if rng.gen::<f32>() < self.mut_prob {
                self.flip_mutate(individual);
            }
            // else{
            //     new_population.push(individual.clone());
            // }
        }
        //new_population
    }

    pub fn mate(&self, population: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        // crossover and mutation are operators we want to apply on the population
        let mut new_population = self.crossover(population);
        self.mutate(&mut new_population);
        //new_population = self.mutate(&mut new_population);
        new_population
    }

    pub fn run(&self, population: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>>{
        //let mut evaluation = 0;
        for generation in 0..self.max_gen{
            //evaluation += population.len();
            let mut fitness: Vec<f32> = Vec::with_capacity(population.len());
            let mut objective: Vec<u32> = Vec::with_capacity(population.len());
            for individual in population.iter() {
                let (fit, obj) = self.fitness_objective(individual); //&self.bin_weights(individual)
                fitness.push(fit);
                objective.push(obj);
            }
            if generation % 100 == 0 {
                info!("Generation: {}, fitness: {:?}, objective: {:?}", generation, fitness, objective);
            }
            let mating_pool = self.tournament_selection(population, &fitness);
            let mut new_population : Vec<Vec<usize>> = self.mate(population);
            *population = new_population;
        }
        population.to_vec()
    }

    pub fn main_exec(&self){
        let mut best_individuals = Vec::new();
        for run in 0..self.repeats {
            let mut population : Vec<Vec<usize>>= self.create_population(self.weights.len());
            population = self.run(&mut population);
            let mut fitness: Vec<f32> = Vec::with_capacity(population.len());
            let mut objective: Vec<u32> = Vec::with_capacity(population.len());
            for individual in population.iter() {
                let (fit, obj) = self.fitness_objective(individual); //&self.bin_weights(individual)
                fitness.push(fit);
                objective.push(obj);
            }
            let best_individual = population[fitness.iter().position_max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()].clone();
            //let _ = values.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap());
            best_individuals.push(best_individual.clone());
            println!("Run: {}, best individual: {:?}", run, best_individual);
            println!("Objective: {:?}", self.objective(&best_individual));
            println!("Bin Weights: {:?}", self.bin_weights(&best_individual));
            let _ = simple_logging::log_to_file("evolutionary_algo.log", LevelFilter::Info);
            info!("Run: {}, best individual: {:?}", run, best_individual);
            info!("Objective: {:?}", self.objective(&best_individual));
            info!("Bin Weights: {:?}", self.bin_weights(&best_individual));
        }
        info!("Best individuals: {:?}", best_individuals);
    }
}

pub fn create_eva_problem(){
    let mut problem = PartitioningProblem::new(
        Vec::new(),
        10,
        100,
        1000,
        0.8,
        0.2,
        0.085,
        5
    );
    problem.set_weights_from_file("test_data/partition.txt");
    println!("Weights successfully loaded");
    problem.main_exec();
}