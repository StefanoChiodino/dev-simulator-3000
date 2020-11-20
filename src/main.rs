use cursive::views::TextView;

struct Job {
    work_units_cost: i32,
    title: String,
}

struct Dev {
    name: String,
}


fn main() {
    let job = Job { work_units_cost: 100, title: "Fix typo".to_string() };

	let mut siv = cursive::default();

	siv.add_global_callback('q', |s| s.quit());
    let mut text = TextView::new(job.title);
	siv.add_layer(text);

    siv.run();
    text.
}
