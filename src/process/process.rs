use libc::sched_getparam;
use libc::sched_getscheduler;
use libc::sched_param;
use std::fmt::Display;
use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
#[derive(Debug, Default)]
pub struct Process {
    pub name: String,
    pub id: i32,
    pub tasks: Vec<i32>,
    pub sched_policy: i32,
    pub rt_prio: i32,
}
#[derive(Debug, Default)]
pub struct ProcessBuilder {
    pub name: Option<String>,
    pub id: i32,
    pub tasks: Option<Vec<i32>>,
    pub sched_policy: Option<i32>,
    pub rt_prio: Option<i32>,
}
impl Process {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn sched_policy(&self) -> i32 {
        self.sched_policy
    }
    pub fn rt_ptio(&self) -> i32 {
        self.rt_prio
    }
    pub fn builder() -> ProcessBuilder {
        ProcessBuilder::default()
    }
}
fn parse_task_from_filename(filename: &str) -> Option<i32> {
    filename.parse().ok()
}
impl ProcessBuilder {
    pub fn new(id: i32) -> Self {
        ProcessBuilder {
            id: id,
            ..Default::default()
        }
    }
    ////////////////////////////////////////////////////////////
    pub fn name(mut self) -> Self {
        let id = self.id;
        let mut name = String::new();
        File::open(format!("/proc/{}/comm", id))
            .and_then(|mut file| file.read_to_string(&mut name))
            .unwrap();
        self.name = Some(name.trim().to_string());
        self
    }
    ////////////////////////////////////////////////////////////
    pub fn tasks(mut self) -> Self {
        let id = self.id;

        let tasks = read_dir(format!("/proc/{id}/task"))
            .unwrap()
            .filter_map(|entry| {
                let task = parse_task_from_filename(entry.unwrap().file_name().to_str()?)?;
                if task != id {
                    Some(task)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        self.tasks = Some(tasks);
        self
    }
    ////////////////////////////////////////////////////////////
    pub unsafe fn sched_policy(mut self) -> Self {
        let id = self.id;
        let sched_policy = sched_getscheduler(id);
        self.sched_policy = Some(sched_policy);
        self
    }
    ////////////////////////////////////////////////////////////
    pub unsafe fn rt_prio(mut self) -> Self {
        let id = self.id;
        let mut param = sched_param { sched_priority: 0 };
        sched_getparam(id, &mut param);
        self.rt_prio = Some(param.sched_priority);
        self
    }
    ////////////////////////////////////////////////////////////
    pub fn build(self) -> Process {
        Process {
            id: self.id,
            name: self.name.unwrap_or_default(),
            sched_policy: self.sched_policy.unwrap_or(-1),
            rt_prio: self.rt_prio.unwrap_or_default(),
            tasks: self.tasks.unwrap_or_default(),
        }
    }
    pub fn full() -> Vec<Process> {
        read_dir("/proc")
            .unwrap()
            .filter_map(|entry| {
                let task = parse_task_from_filename(entry.unwrap().file_name().to_str()?)?;
                unsafe {
                    Some(
                        ProcessBuilder::new(task)
                            .name()
                            .sched_policy()
                            .rt_prio()
                            .tasks()
                            .build(),
                    )
                }
            })
            .collect::<Vec<_>>()
    }
}
impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sched = match self.sched_policy {
            0 => "SCHED_OTHER",
            1 => "SCHED_FIFO",
            2 => "SCHED_RR",
            3 => "SCHED_BATCH",
            5 => "SCHED_IDLE",
            _ => "UNKNOWN",
        };
        let rt = match self.rt_prio {
            0..=33 => "*",
            34..=66 => "**",
            67..=99 => "***",
            _ => "UNKNOWN",
        };
        write!(
            f,
            "{}\t{:-40}\t{:-15}\t{}\t\t{}",
            self.id, self.name, sched, self.rt_prio, rt
        )
    }
}
