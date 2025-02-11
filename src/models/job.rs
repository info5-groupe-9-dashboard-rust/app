use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;

#[cfg(target_arch = "wasm32")]
use chrono::{DateTime, Utc};

#[derive(Clone, Deserialize, Serialize, PartialEq, EnumIter, Debug, Eq)]

pub enum State {
    Unknown,
    Waiting,
    Hold,
    ToLaunch,
    ToError,
    ToAckReservation,
    Launching,
    Running,
    Suspended,
    Resuming,
    Finishing,
    Terminated,
    Error,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Unknown => write!(f, "Unknown"),
            State::Waiting => write!(f, "Waiting"),
            State::Hold => write!(f, "Hold"),
            State::ToLaunch => write!(f, "ToLaunch"),
            State::ToError => write!(f, "ToError"),
            State::ToAckReservation => write!(f, "ToAckReservation"),
            State::Launching => write!(f, "Launching"),
            State::Running => write!(f, "Running"),
            State::Suspended => write!(f, "Suspended"),
            State::Resuming => write!(f, "Resuming"),
            State::Finishing => write!(f, "Finishing"),
            State::Terminated => write!(f, "Terminated"),
            State::Error => write!(f, "Error"),
        }
    }
}

#[derive(Clone)]

pub struct Job {
    pub id: u32,
    pub owner: String,
    pub state: State,
    pub command: String,
    pub walltime: i64,
    pub message: Option<String>,
    pub queue: String,
    pub assigned_resources: Vec<u32>,
    pub scheduled_start: i64,
    pub submission_time: i64,
    pub start_time: i64,
    pub stop_time: i64,
    pub exit_code: Option<i32>,
}

impl Job {
    pub fn _display(&self) {
        println!("Job ID: {}", self.id);
        println!("Owner: {}", self.owner);
        println!("State: {}", self.state);
        println!("Command: {}", self.command);
        println!("Walltime: {}", self.walltime);
        println!("Message: {:?}", self.message);
        println!("Queue: {}", self.queue);
        println!("Assigned Resources: {:?}", self.assigned_resources);
        println!("Scheduled Start: {:?}", self.scheduled_start);
        println!("Submission Time: {}", self.submission_time);
        println!("Start Time: {}", self.start_time);
        println!("Stop Time: {}", self.stop_time);
        println!("Exit Code: {:?}", self.exit_code);
    }
}

// Mocking Job to Test ApplicationContext
#[cfg(target_arch = "wasm32")]
pub fn mock_job(id: u32) -> Job {
    // Liste des propriétaires possibles
    let owners = vec!["alice", "bob", "charlie", "david", "eva"];

    // Liste des commandes possibles avec leurs queues associées
    let commands = vec![
        ("python3 train_model.py", "gpu"),
        ("make test", "test"),
        ("mpirun -np 4 simulation", "cpu"),
        ("jupyter notebook", "interactive"),
        ("gcc -O3 project.c", "compile"),
    ];

    // Fonction helper pour générer un nombre aléatoire
    let random_index = |max: usize| -> usize {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value % max as u64) as usize
    };

    let random_float = || -> f32 {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value as f32) / (u64::MAX as f32)
    };

    // Génération de timestamps cohérents
    let now = Utc::now().timestamp();
    let submission_time = now - (random_index(86400) as i64);
    let scheduled_start = submission_time + (random_index(3300) as i64 + 300);
    let start_time = if random_float() < 0.7 {
        scheduled_start
    } else {
        0
    };
    let walltime = random_index(5400) as i64 + 1800;
    let stop_time = if start_time > 0 && random_float() < 0.3 {
        start_time + walltime
    } else {
        0
    };

    // Sélection aléatoire de l'état en fonction du contexte temporel
    let state = if stop_time > 0 {
        State::Terminated
    } else if start_time > 0 {
        let states = vec![State::Running, State::Suspended, State::Finishing];
        states[random_index(states.len())].clone()
    } else if scheduled_start > now {
        let states = vec![State::Waiting, State::Hold];
        states[random_index(states.len())].clone()
    } else {
        let states = vec![State::ToLaunch, State::Launching, State::Waiting];
        states[random_index(states.len())].clone()
    };

    // Sélection de la commande et de la queue
    let (command, queue) = commands[random_index(commands.len())];

    // Génération des ressources assignées
    let num_resources = random_index(7) + 1;
    let assigned_resources = if start_time > 0 {
        let mut resources = Vec::new();
        while resources.len() < num_resources {
            let resource = (random_index(20) + 1) as u32;
            if !resources.contains(&resource) {
                resources.push(resource);
            }
        }
        resources
    } else {
        vec![]
    };

    // Génération du message en fonction de l'état
    let message = match state {
        State::Error => Some("Erreur d'exécution".to_string()),
        State::Hold => Some("En attente de ressources".to_string()),
        State::Suspended => Some("Suspendu par l'administrateur".to_string()),
        _ => None,
    };

    Job {
        id,
        owner: owners[random_index(owners.len())].to_string(),
        state,
        command: command.to_string(),
        walltime,
        message,
        queue: queue.to_string(),
        assigned_resources,
        scheduled_start,
        submission_time,
        start_time,
        stop_time,
        exit_code: if stop_time > 0 {
            Some((random_index(3) as i32) - 1)
        } else {
            None
        },
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mock_jobs() -> Vec<Job> {
    (1..=50).map(|id| mock_job(id)).collect()
}
