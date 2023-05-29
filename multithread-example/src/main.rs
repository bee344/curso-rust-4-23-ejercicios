mod closures_types;
mod closures_move;
mod integrated_crossbeam;
mod multithread_arc;
mod multithread_rc;
mod multithread_mutex;
mod multithread_rwlock;
mod multithread_simple;
mod multithread_with_channels;
mod multithread_with_channels_crossbeam;
mod multithread_with_crossbeam;
mod multithread_with_files;


fn main() {
    let name = "Rust".to_string();

    let my_closure = move || {
        println!("Hello, {}!", name);
    };

    my_closure();
}

#[test]
fn test_closures_types() {
    closures_types::execute();
}

#[test]
fn test_multithread_simple() {
    multithread_simple::execute();
}

#[test]
fn test_multithread_with_files() {
    multithread_with_files::execute();
}


#[test]
fn test_multithread_with_crossbeam() {
    multithread_with_crossbeam::execute();
}

#[test]
fn test_multithread_with_channels() {
    multithread_with_channels::execute();
}

#[test]
fn test_multithread_with_channels_crossbeam() {
    multithread_with_channels_crossbeam::execute();
}

#[test]
fn test_multithread_rc() {
    multithread_rc::execute();
}

#[test]
fn test_multithread_arc() {
    multithread_arc::execute();
}


#[test]
fn test_multithread_mutex() {
    multithread_mutex::execute();
}

#[test]
fn test_multithread_rwlock() {
    multithread_rwlock::execute();
}

#[test]
fn test_integrated_crossbeam() {
    integrated_crossbeam::execute();
}

