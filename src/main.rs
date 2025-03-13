use rand::{Rng, prelude::{IndexedRandom, IndexedMutRandom}};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Class {
    subject: String,
    instructor: String,
    room: String,
    time_slot: usize,
}

#[derive(Clone, Debug)]
struct Schedule {
    classes: Vec<Class>,
    fitness: f64,
}

impl Schedule {
    fn new(subjects: &[String], instructors: &[String], rooms: &[String], time_slots: usize) -> Self {
        let mut rng = rand::rng();
        let mut classes = Vec::new();

        for subject in subjects {
            let instructor = instructors.choose(&mut rng).unwrap().clone();
            let room = rooms.choose(&mut rng).unwrap().clone();
            let time_slot = rng.gen_range(0..time_slots);
            classes.push(Class {
                subject: subject.clone(),
                instructor,
                room,
                time_slot,
            });
        }

        let fitness = Schedule::calculate_fitness(&classes);
        Schedule { classes, fitness }
    }

    fn calculate_fitness(classes: &[Class]) -> f64 {
        let mut conflicts = 0;
        let mut room_time_map = HashSet::new();
        let mut instructor_time_map = HashSet::new();
        let mut course_time_map = HashSet::new();
        let mut conflict_details = Vec::new();

        for class in classes {
            let room_key = (class.room.clone(), class.time_slot);
            let instructor_key = (class.instructor.clone(), class.time_slot);
            let course_key = (class.subject.clone(), class.time_slot);

            // Room Conflict: Same room booked at the same time
            if !room_time_map.insert(room_key.clone()) {
                conflicts += 1;
                conflict_details.push(format!(
                    "Room Conflict: {} at Time Slot {}",
                    room_key.0, room_key.1
                ));
            }

            // Instructor Conflict: Same instructor teaching two classes at the same time
            if !instructor_time_map.insert(instructor_key.clone()) {
                conflicts += 1;
                conflict_details.push(format!(
                    "Instructor Conflict: {} at Time Slot {}",
                    instructor_key.0, instructor_key.1
                ));
            }

            // Course Conflict: Same course scheduled in different rooms or with different instructors at the same time
            if !course_time_map.insert(course_key.clone()) {
                conflicts += 1;
                conflict_details.push(format!(
                    "Course Conflict: {} at Time Slot {} (Multiple Rooms/Instructors)",
                    course_key.0, course_key.1
                ));
            }
        }

        // Display detected conflicts
        if !conflict_details.is_empty() {
            println!("Conflicts Detected:");
            for conflict in &conflict_details {
                println!("{}", conflict);
            }
        }

        1.0 / (1.0 + conflicts as f64) // Higher fitness is better (fewer conflicts)
    }
}

impl Schedule {
    fn crossover(&self, other: &Schedule) -> Schedule {
        let mut rng = rand::rng();
        let mut new_classes = Vec::new();

        for i in 0..self.classes.len() {
            if rng.gen_bool(0.5) {
                new_classes.push(self.classes[i].clone());
            } else {
                new_classes.push(other.classes[i].clone());
            }
        }

        let fitness = Schedule::calculate_fitness(&new_classes);
        Schedule { classes: new_classes, fitness }
    }

    fn mutate(&mut self, instructors: &[String], rooms: &[String], time_slots: usize) {
        let mut rng = rand::rng();
        if let Some(class) = self.classes.choose_mut(&mut rng) {
            class.instructor = instructors.choose(&mut rng).unwrap().clone();
            class.room = rooms.choose(&mut rng).unwrap().clone();
            class.time_slot = rng.gen_range(0..time_slots);
        }

        self.fitness = Schedule::calculate_fitness(&self.classes);
    }
}

fn genetic_algorithm(
    subjects: Vec<String>,
    instructors: Vec<String>,
    rooms: Vec<String>,
    time_slots: usize,
    population_size: usize,
    generations: usize,
) -> Schedule {
    let mut rng = rand::rng();
    let mut population: Vec<Schedule> = (0..population_size)
        .map(|_| Schedule::new(&subjects, &instructors, &rooms, time_slots))
        .collect();

    for gen in 0..generations {
        // Sort by fitness (best first)
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        // Show conflicts for best schedule in this generation
        println!("\n=== Generation {} ===", gen + 1);
        println!("Best Fitness: {:.5}", population[0].fitness);
        Schedule::calculate_fitness(&population[0].classes);

        // Select top half as parents
        let parents = &population[..population_size / 2];

        // Create next generation
        let mut next_gen = parents.to_vec();

        while next_gen.len() < population_size {
            let parent1 = parents.choose(&mut rng).unwrap();
            let parent2 = parents.choose(&mut rng).unwrap();
            let mut child = parent1.crossover(parent2);
            if rng.gen_bool(0.1) { // 10% mutation rate
                child.mutate(&instructors, &rooms, time_slots);
            }
            next_gen.push(child);
        }

        population = next_gen;
    }

    // Return best schedule
    population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    population[0].clone()
}

fn main() {
    let subjects = vec![
        "Math".to_string(), 
        "Physics".to_string(), 
        //"Chemistry".to_string(),
        //"Biology".to_string()
    ];
    let instructors = vec![
        "Dr. Smith".to_string(), 
        "Dr. Jones".to_string(), 
        "Dr. Brown".to_string()
    ];
    let rooms = vec![
        "Room 101".to_string(), 
        //"Room 102".to_string()
    ];
    let time_slots = 2;
    
    let best_schedule = genetic_algorithm(subjects, instructors, rooms, time_slots, 10, 50);

    println!("\n=== Best Final Schedule ===");
    println!("{:?}", best_schedule);
}
