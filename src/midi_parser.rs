use midly::{TrackEventKind, MidiMessage, Smf, Timing};

struct MotorCommand {
    time: f64,
    motor_command: (u64, u64),
}

fn main() {
    let motor_commands = parse_midi_into_motor_commands("Fur Elise.mid");
    for command in motor_commands {
        println!("time: {}, motor command: {:?}", command.time, command.motor_command);
    }
}

fn parse_midi_into_motor_commands(path: &str) -> Vec<MotorCommand> {
    let mut motor_commands = Vec::new();
    let mut tempo_map = Vec::new(); // New: a map of tempos over time

    let data = std::fs::read(path).unwrap();
    let smf = Smf::parse(&data).unwrap();
    let ticks_per_beat: u16;

    match smf.header.timing {
        Timing::Metrical(perbeat) => {ticks_per_beat = perbeat.into()},
        _ => {panic!("wrong time")} 
    }

    let mut tempo: f64 = 500_000.0;

    // First pass: Process all tracks for tempo changes
    for track in &smf.tracks {
        let mut absolute_time: f64 = 0.0;
        for event in track {
            let delta_time_in_seconds = (event.delta.as_int() as f64 / ticks_per_beat as f64) * (tempo / 1000000.0);
            absolute_time += delta_time_in_seconds;
            match event.kind {
                TrackEventKind::Meta(midly::MetaMessage::Tempo(new_tempo)) => {
                    tempo = new_tempo.as_int() as f64;
                    tempo_map.push((absolute_time, tempo)); // New: add each tempo change to the map
                },
                _ => {},
            }
        }
    }

    // Second pass: Process all tracks for MIDI messages
    for track in smf.tracks {
        let mut absolute_time: f64 = 0.0;
        for event in track {
            let delta_time_in_seconds = (event.delta.as_int() as f64 / ticks_per_beat as f64) * (tempo / 1000000.0);
            absolute_time += delta_time_in_seconds;
            match event.kind {
                TrackEventKind::Midi {message, ..} => {
                    // New: look up the correct tempo for this time from the map
                    if let Some((_, new_tempo)) = tempo_map.iter().take_while(|(time, _)| time <= &&absolute_time).last() {
                        tempo = *new_tempo;
                    }
                    let motor_commands_to_add = convert_to_motor_command(message);
                    for command in motor_commands_to_add {
                        motor_commands.push(MotorCommand { time: absolute_time+command.time, motor_command: command.motor_command });
                    }
                },
                _ => {},
            }
        }
    }

    motor_commands.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    motor_commands
}

fn convert_to_motor_command(message: MidiMessage) -> Vec<MotorCommand> {
    match message {
        MidiMessage::NoteOn { key, vel } => {
            let mut commands = Vec::new();
            if vel == 127 {
                commands.push(MotorCommand { time: 0.0, motor_command: (90, key.as_int() as u64) });
                return commands;
            }
            // Convert the velocity to a series of motor commands
            let velocity: i64 = vel.as_int().into();
            for i in 0..=90 {
                // Add a delay between commands based on the velocity
                let delay =(i as f64) * (0.001/90.0) * (((-1*velocity)+128) as f64);
                commands.push(MotorCommand { time: delay, motor_command: (i, key.as_int() as u64) });
            }
            commands
        },
        MidiMessage::NoteOff { key, .. } => {
            // For NoteOff messages, set the motor angle to 0
            vec![MotorCommand { time: 0.0, motor_command: (0, key.as_int() as u64) }]
        },
        _ => vec![],
    }
}