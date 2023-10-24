use libc::{sched_getscheduler, SCHED_FIFO, SCHED_OTHER};
use std::fs::{read_dir, File};
use std::io::Read;
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct Process {
    name: String,
    id: i32,
    tasks: Vec<i32>,
    sched: i32,
}

impl Process {
    fn new(id: i32, name: String, sched: i32) -> Self {
        Self {
            id,
            name,
            tasks: Vec::new(),
            sched,
        }
    }

    fn set_tasks(&mut self, tasks: Vec<i32>) {
        self.tasks = tasks;
    }

    fn get_sched_policy(&self) -> &str {
        match self.sched {
            SCHED_OTHER => "SCHED_OTHER",
            SCHED_FIFO => "SCHED_FIFO",
            _ => "Unknown",
        }
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{:-40}\t{:-15}",
            self.id,
            self.name,
            self.get_sched_policy()
        )
    }
}

fn parse_pid_from_filename(filename: &str) -> Option<i32> {
    filename.parse().ok()
}

fn read_process_info(id: i32) -> Option<Process> {
    let tasks = read_dir(format!("/proc/{}/task", id))
        .ok()?
        .filter_map(|entry| {
            let task_id = parse_pid_from_filename(&entry.unwrap().file_name().to_string_lossy())?;
            if task_id != id {
                Some(task_id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut name = String::new();
    File::open(format!("/proc/{}/comm", id))
        .ok()?
        .read_to_string(&mut name)
        .ok()?;

    Some(Process::new(id, name.trim().to_string(), unsafe {
        sched_getscheduler(id)
    }))
}

fn main() {
    println!("{}\t{:-40}\t{:-15}", "PID", "Name", "Sched Policy");
    let processes = read_dir("/proc")
        .unwrap()
        .filter_map(|entry| {
            let id = parse_pid_from_filename(&entry.unwrap().file_name().to_string_lossy())?;
            read_process_info(id)
        })
        .collect::<Vec<Process>>();

    for process in processes {
        println!("{}", process);
    }
}

