struct Task {
    wcet: f64,
}

struct Async {
    task: Task,
    release: f64,
    priority: usize,
}

struct Cyclic {
    task: Task,
    period: f64,
}

impl Cyclic {
    fn new(wcet: f64, period: f64) -> Cyclic {
        Cyclic {
            task: Task { wcet: wcet },
            period: period,
        }
    }
}

struct Event {
    start: f64,
    end: f64,
}

impl Event {
    fn new(start: f64, end: f64) -> Event {
        Event {
            start: start,
            end: end,
        }
    }
}

struct ScheduleEvent {
    task: usize,
    event: Event,
}

impl ScheduleEvent {
    fn new(task: usize, start: f64, end: f64) -> ScheduleEvent {
        ScheduleEvent {
            task: task,
            event: Event::new(start, end),
        }
    }
}

struct ReleaseSchedule {
    tasks: Vec<Async>,
}

fn response_time_rate_monotonic(tasks: &mut [Cyclic]) -> Vec<f64> {
    use std::cmp::Ordering;
    tasks.sort_by(|t0: &Cyclic, t1: &Cyclic| if t0.period < t1.period {Ordering::Less}
        else if t0.period == t1.period {Ordering::Equal} else {Ordering::Greater});
    tasks.iter().enumerate().map(|(i, t)| {
        let mut cw = t.task.wcet;
        loop {
            let nw = t.task.wcet + tasks[0..i].iter().map(|tj| (cw/tj.period).ceil() * tj.task.wcet).sum::<f64>();
            if nw == cw {
                return nw;
            }
            cw = nw;
        }
    }).collect::<Vec<_>>()
}

fn response_time_rate_monotonic_load_save(tasks: &mut [Cyclic], load: f64, save: f64) -> Vec<f64> {
    use std::cmp::Ordering;
    tasks.sort_by(|t0: &Cyclic, t1: &Cyclic| if t0.period < t1.period {Ordering::Less}
        else if t0.period == t1.period {Ordering::Equal} else {Ordering::Greater});
    tasks.iter().enumerate().map(|(i, t)| {
        let basis = t.task.wcet + load + save;
        let mut cw = basis;
        loop {
            let nw = basis + tasks[0..i].iter().map(|tj| (cw/tj.period).ceil() *
                (tj.task.wcet + load + save)).sum::<f64>();
            if nw == cw {
                return nw;
            }
            cw = nw;
        }
    }).collect::<Vec<_>>()
}

fn main() {
    println!("Response times: {:?}", response_time_rate_monotonic_load_save(
        &mut vec![
            Cyclic::new(3.0, 7.0),
            Cyclic::new(3.0, 12.0),
            Cyclic::new(5.0, 20.0),
        ],
        0.5,
        0.5,
    ));
}
