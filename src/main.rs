use rand::prelude::*;
use rand::rng;
use chrono::{NaiveTime, Duration};

#[derive(Debug, Clone)]
struct ClassSchedule {
    course: String,
    instructor: String,
    room: String,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

impl ClassSchedule {
    fn new(course: &str, instructor: &str, room: &str, start_time: NaiveTime, duration: i64) -> Self {
        Self {
            course: course.to_string(),
            instructor: instructor.to_string(),
            room: room.to_string(),
            start_time,
            end_time: start_time + Duration::minutes(duration),
        }
    }
}

fn generate_random_schedule(courses: &[&str], instructors: &[&str], rooms: &[&str], times: &[NaiveTime]) -> Vec<ClassSchedule> {
    let mut rng = rng();
    let mut schedule = Vec::new();

    for &course in courses {
        let instructor = instructors.choose(&mut rng).unwrap();
        let room = rooms.choose(&mut rng).unwrap();
        let start_time = times.choose(&mut rng).unwrap();
        let duration = [60, 90].choose(&mut rng).unwrap();

        schedule.push(ClassSchedule::new(course, instructor, room, *start_time, *duration));
    }

    schedule
}

fn calculate_conflicts(schedule: &[ClassSchedule]) -> i32 {
    let mut conflicts = 0;

    for (i, class1) in schedule.iter().enumerate() {
        for class2 in &schedule[i + 1..] {
            if class1.instructor == class2.instructor && classes_overlap(class1, class2) {
                println!("Conflict: Instructor {} has overlapping classes {} and {}", class1.instructor, class1.course, class2.course);
                conflicts += 1;
            }
            if class1.room == class2.room && classes_overlap(class1, class2) {
                println!("Conflict: Room {} is double-booked for {} and {}", class1.room, class1.course, class2.course);
                conflicts += 1;
            }
        }
    }

    conflicts
}

fn classes_overlap(class1: &ClassSchedule, class2: &ClassSchedule) -> bool {
    class1.start_time < class2.end_time && class2.start_time < class1.end_time
}

fn mutate(schedule: &mut Vec<ClassSchedule>, rooms: &[&str], times: &[NaiveTime]) {
    let mut rng = rng();
    let index = rng.random_range(0..schedule.len());
    
    schedule[index].room = rooms.choose(&mut rng).unwrap().to_string();
    schedule[index].start_time = *times.choose(&mut rng).unwrap();
    schedule[index].end_time = schedule[index].start_time + Duration::minutes(60);
}

fn crossover(parent1: &[ClassSchedule], parent2: &[ClassSchedule]) -> Vec<ClassSchedule> {
    let mut rng = rng();
    let crossover_point = rng.random_range(0..parent1.len());
    
    let mut child = Vec::new();
    child.extend_from_slice(&parent1[..crossover_point]);
    child.extend_from_slice(&parent2[crossover_point..]);

    child
}

fn genetic_algorithm(
    courses: &[&str],
    instructors: &[&str],
    rooms: &[&str],
    times: &[NaiveTime],
    generations: usize,
) -> Vec<ClassSchedule> {
    let mut population: Vec<Vec<ClassSchedule>> = (0..10)
        .map(|_| generate_random_schedule(courses, instructors, rooms, times))
        .collect();

    for _ in 0..generations {
        population.sort_by_key(|schedule| calculate_conflicts(schedule));
        
        let parents = &population[..2]; // Best 2 schedules
        let mut new_population = Vec::new();

        for _ in 0..5 {
            let child = crossover(&parents[0], &parents[1]);
            new_population.push(child);
        }

        for mut schedule in &mut new_population {
            mutate(&mut schedule, rooms, times);
        }

        population = new_population;
    }

    population[0].clone()
}


fn main() {
    let courses = ["Math", "Science", "History", "English"];
    let instructors = ["Alice", "Bob", "Charlie"];
    let rooms = ["Room 101", "Room 102", "Room 103"];
    let times: Vec<NaiveTime> = [
        NaiveTime::from_hms_opt(8, 0, 0),
        NaiveTime::from_hms_opt(9, 30, 0),
        NaiveTime::from_hms_opt(11, 0, 0),
    ]
    .into_iter()
    .flatten()
    .collect();

    let best_schedule = genetic_algorithm(&courses, &instructors, &rooms, &times, 50);

    println!("Optimized Schedule:");
    for class in &best_schedule {
        println!("{:?}", class);
    }

    let conflicts = calculate_conflicts(&best_schedule);
    println!("Total Conflicts: {}", conflicts);
}
