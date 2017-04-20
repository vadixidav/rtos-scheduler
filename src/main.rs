struct Task {
    wcet: f64,
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

fn response_time_rate_monotonic(mut tasks: Vec<Cyclic>) -> Vec<f64> {
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

fn main() {
    println!("Response times: {:?}", response_time_rate_monotonic(
        vec![
            Cyclic::new(3.0, 7.0),
            Cyclic::new(3.0, 12.0),
            Cyclic::new(5.0, 20.0),
        ]
    ));
}
