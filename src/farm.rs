use crate::model::Dimensions;
use chrono::{DateTime, Duration, Utc};
use std::collections::{HashMap, VecDeque};

// A Farm represents one print farm which houses a number of machines
#[derive(Debug)]
pub struct Farm<'a> {
    pub machines: Vec<Machine<'a>>,
}

impl<'a> Farm<'a> {
    fn new() -> Self {
        Self {
            machines: Vec::new(),
        }
    }

    fn add_machine(&mut self, machine: Machine<'a>) {
        self.machines.push(machine);
    }

    fn remove_machine(&mut self, index: usize) {
        self.machines.remove(index);
    }

    fn add_job(&mut self, job: PrintItem<'a>) {
        for mach in &mut self.machines {
            for batch in &mut mach.schedule {
                if job.family.config == batch.family.config && batch.can_fit(&job.dims) {
                    batch.add(job);
                    return;
                }
            }
        }
    }

    fn remove_job(&mut self, index: usize) {
        self.machines.remove(index);
    }
}

// A Machine represents one printer that can process batches of jobs
#[derive(Debug)]
pub struct Machine<'a> {
    name: String,
    is_running: bool,
    schedule: VecDeque<Batch<'a>>,
    config: HashMap<&'a str, String>,
}

impl Machine<'_> {
    fn new(capacity: Dimensions) -> Self {
        Self {
            name: String::from("Untitled"),
            schedule: VecDeque::from([Batch::new(capacity, Utc::now())]),
            is_running: false,
            config: Default::default(),
        }
    }

    fn with_name(capacity: Dimensions, name: String) -> Self {
        Self {
            name,
            schedule: VecDeque::from([Batch::new(capacity, Utc::now())]),
            is_running: false,
            config: Default::default(),
        }
    }
}

// A Batch represents a family of jobs within the same JobFamily that can be processed together. As a result,
// each item in the batch has the same completion time.
#[derive(Debug)]
struct Batch<'a> {
    items: Vec<PrintItem<'a>>,
    start_time: chrono::DateTime<Utc>,
    print_time: chrono::Duration,
    // Preceding batch's completion time + max(batch.print_time for batch in batches)
    capacity: Dimensions,
    family: JobFamily<'a>,
}

impl<'a> Batch<'a> {
    fn new(capacity: Dimensions, start_time: DateTime<Utc>) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            start_time,
            print_time: Duration::nanoseconds(0),
            family: JobFamily {
                name: String::from("Job Family 1"),
                config: HashMap::new(),
            },
        }
    }

    fn can_fit(&self, dims: &Dimensions) -> bool {
        let mut leftover = Dimensions(0, 0, 0);
        for i in &self.items {
            leftover = leftover + i.dims;
        }

        leftover + *dims < self.capacity
    }

    fn est_completion_time(&self) -> DateTime<Utc> {
        self.start_time + self.print_time
    }

    fn add<'b: 'a>(&mut self, item: PrintItem<'b>) {
        self.items.push(item);
    }

    fn remove(&mut self, index: usize) {
        self.items.remove(index);
    }
}

// A PrintItem represents one print job that can be put into a batch
#[derive(Debug)]
struct PrintItem<'a> {
    dims: Dimensions,
    due_date: DateTime<Utc>,
    print_time: Duration,
    family: JobFamily<'a>,
}

impl<'a> PrintItem<'a> {
    fn new<'b: 'a>(
        dims: Dimensions,
        time_till_due: Duration,
        print_time: Duration,
        family: JobFamily<'b>,
    ) -> Self {
        Self {
            dims,
            due_date: Utc::now() + time_till_due,
            print_time,
            family,
        }
    }
}

#[derive(Debug)]
struct JobFamily<'a> {
    name: String,
    config: HashMap<&'a str, String>, // Job family characteristics. Compared for
}

impl JobFamily<'_> {
    fn new(name: String) -> Self {
        Self {
            name,
            config: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn farm_add_test() {
        let mut farm = Farm::new();
        let machine1 = Machine::new(Dimensions(300, 200, 400));
        let machine2 = Machine::new(Dimensions(100, 150, 350));

        farm.add_machine(machine2);
        farm.add_machine(machine1);

        let job1 = PrintItem::new(
            Dimensions(200, 100, 50),
            chrono::Duration::days(3),
            chrono::Duration::minutes(70),
            JobFamily::new(String::from("Job fam 1")),
        );
        farm.add_job(job1);
        println!("{farm:#?}");
    }
}
