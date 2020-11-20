use chrono::prelude::*;

pub struct Job {
    pub work_units_cost: i32,
    pub title: String,
}

pub struct Dev {
    pub name: String,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    
    pub enhanced_graphics: bool,

    pub job_list: Vec<Job>,
    pub dev_list: Vec<Dev>,

    pub world_datetime: DateTime<Utc>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            enhanced_graphics,
            job_list: vec![Job{title:"Job 1".to_string(), work_units_cost: 2}],
            dev_list: vec![{Dev{name:"Dev 1".to_string()}}],
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
        self.world_datetime = self.world_datetime + chrono::Duration::seconds(1);
    }
}
