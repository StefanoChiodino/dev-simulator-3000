use chrono::prelude::*;
use std::collections::HashMap;
use chrono::Duration;


pub struct Job {
    pub id: i32,
    pub duration: Duration,
    pub duration_remaining: Duration,
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
    tick_rate: Duration,
    warp_factor: i32,
    
    pub enhanced_graphics: bool,

    pub jobs: HashMap<i32, Job>,
    pub devs: HashMap<i32, Dev>,
    pub assignments: Vec<Assignment>,

    pub world_datetime: DateTime<Utc>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, tick_rate: Duration) -> App<'a> {
        let mut jobs = HashMap::new();
        jobs.insert(1, Job{id: 1, title:"Job 1".to_string(), duration: Duration::hours(200), duration_remaining: Duration::hours(200)});
        jobs.insert(2, Job{id: 1, title:"Job 2".to_string(), duration: Duration::hours(500), duration_remaining: Duration::hours(500)});

        let mut devs = HashMap::new();
        devs.insert(1, Dev{name:"Dev 1".to_string(), man_hours_per_hour: 1});
        devs.insert(2, Dev{name:"Dev 2".to_string(), man_hours_per_hour: 1});
        
        App {
            title,
            tick_rate,
            warp_factor: 60*60*24,
            should_quit: false,
            enhanced_graphics,
            jobs: jobs,
            devs: devs,
            assignments: vec![Assignment{job_id: 1, dev_id: 1}, Assignment{job_id: 2, dev_id: 2}],
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
        // Update progress
        let game_duration = self.tick_rate * self.warp_factor;
        self.world_datetime = self.world_datetime + game_duration;

        for assignment in &self.assignments{
            let mut job = self.jobs.get_mut(&assignment.job_id).unwrap();
            job.duration_remaining = std::cmp::max(job.duration_remaining - game_duration, Duration::zero());
        }
    }
}
