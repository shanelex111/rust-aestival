use config::Config;

pub fn init(c: &mut Config, inits: &[fn(&mut Config)]) {
    for init in inits {
        init(c);
    }
}

pub fn load(c: &mut Config, loads: &[fn(&mut Config)]) {
    init(c, loads)
}

pub fn run(runs: &[fn()]) {
    for run in runs {
        run();
    }
}
