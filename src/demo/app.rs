use chrono::prelude::*;
use std::collections::HashMap;

pub struct Job {
    pub id: i32,
    pub man_hours: i32,
    pub man_hours_remaining: i32,
    pub title: String,
}

pub struct Dev {
    pub name: String,
    pub man_hours_per_hour: i32
}

pub struct Assignment {
    pub dev_id: i32,
    pub job_id: i32,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    
    pub enhanced_graphics: bool,

    pub jobs: HashMap<i32, Job>,
    pub devs: HashMap<i32, Dev>,
    pub assignments: Vec<Assignment>,

    pub world_datetime: DateTime<Utc>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let mut jobs = HashMap::new();
        jobs.insert(1, Job{id: 1, title:"Job 1".to_string(), man_hours: 200, man_hours_remaining: 200});

        let mut devs = HashMap::new();
        devs.insert(1, Dev{name:"Dev 1".to_string(), man_hours_per_hour: 1});
        
        App {
            title,
            should_quit: false,
            enhanced_graphics,
            jobs: jobs,
            devs: devs,
            assignments: vec![Assignment{job_id: 1, dev_id: 1}],
            world_datetime: Utc.ymd(2005, 1, 1).and_hms(0, 0, 0)
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // TODO: Delta time
        // Update progress
        self.world_datetime = self.world_datetime + chrono::Duration::hours(1);

        /* let dev = &self.dev_list[0];
        let job = &self.job_list[0]; */

        /* job.man_hours -= dev.man_hours_per_hour;
        job.man_hours = std::cmp::max(job.man_hours, 0); */
/* 
        let mut dev = self.devs.get(&1);
        let mut job = &self.jobs[&1];

        job.man_hours_remaining -= dev.man_hours_per_hour; */

        self.jobs.get_mut(&1).unwrap().man_hours_remaining -= self.devs.get_mut(&1).unwrap().man_hours_per_hour;
        self.jobs.get_mut(&1).unwrap().man_hours = std::cmp::max(self.jobs.get_mut(&1).unwrap().man_hours, 0);
    }
}
