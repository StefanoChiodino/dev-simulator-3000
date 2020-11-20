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
    pub progress: f64,
    pub enhanced_graphics: bool,

    pub job_list: Vec<Job>,
    pub dev_list: Vec<Dev>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            progress: 0.0,
            enhanced_graphics,
            job_list: vec![Job{title:"Job 1".to_string(), work_units_cost: 2}],
            dev_list: vec![{Dev{name:"Dev 1".to_string()}}]
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
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}
