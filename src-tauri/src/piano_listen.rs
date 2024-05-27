use std::error::Error;
use std::io::{ stdin, stdout, Write };
use std::sync::{ Arc, Mutex };
use std::thread::sleep;
use std::time::{ Duration, Instant };

use crossbeam_channel::Receiver;
use midir::{ Ignore, MidiInput, MidiOutput, MidiOutputPort };
use serde::Serialize;

use crate::Recording;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ClientEventType {
    KeyPress,
    KeyRelease,
    Pedal,
    Ambience,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientPianoEvent {
    event_type: ClientEventType,
    key_string: String,
    key_id: u8,
    intensity: f32,
}

impl ClientPianoEvent {
    pub fn new(
        event_type: ClientEventType,
        key_string: String,
        intensity: f32,
        key_id: u8
    ) -> Self {
        Self { event_type, intensity, key_string, key_id }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PianoEvent {
    KeyPress(PianoKeyCode, Percent),
    KeyRelease(PianoKeyCode),
    RightPedal(Percent),
    MiddlePedal(bool),
    LeftPedal(Percent),
    SetAmbience(Percent),
}

impl PianoEvent {
    pub fn to_client_event(&self) -> ClientPianoEvent {
        let event_type = match self {
            Self::KeyPress(_, _) => ClientEventType::KeyPress,
            Self::KeyRelease(_) => ClientEventType::KeyRelease,
            Self::RightPedal(_) | Self::MiddlePedal(_) | Self::LeftPedal(_) =>
                ClientEventType::Pedal,
            Self::SetAmbience(_) => ClientEventType::Ambience,
        };

        let key_string = match self {
            Self::KeyPress(key, _) | Self::KeyRelease(key) => key.to_key_name(),
            _ => "".to_string(),
        };

        let key_id = match self {
            Self::KeyPress(key, _) | Self::KeyRelease(key) => *key as u8,
            _ => 0,
        };

        let intensity = match self {
            Self::KeyRelease(_) => 0.0,
            | Self::RightPedal(percent)
            | Self::LeftPedal(percent)
            | Self::KeyPress(_, percent)
            | Self::SetAmbience(percent) => percent.0,
            Self::MiddlePedal(bool) =>
                match bool {
                    true => 1.0,
                    false => 0.0,
                }
        };

        ClientPianoEvent::new(event_type, key_string, intensity, key_id)
    }

    pub fn new(
        key_code: PianoKeyCode,
        state_code: StateCode,
        alpha: Alpha
    ) -> Result<Self, String> {
        if (state_code as u8) == (StateCode::KeyPress as u8) {
            Ok(Self::KeyPress(key_code, Percent::new(alpha)))
        } else if (state_code as u8) == (StateCode::KeyRelease as u8) {
            Ok(Self::KeyRelease(key_code))
        } else if (state_code as u8) == (StateCode::FunctionBegin as u8) {
            match key_code {
                PianoKeyCode::E4 => Ok(Self::RightPedal(Percent::new(alpha))),
                PianoKeyCode::Gb4 => {
                    if alpha.0 == 127 {
                        Ok(Self::MiddlePedal(true))
                    } else {
                        Ok(Self::MiddlePedal(false))
                    }
                }
                PianoKeyCode::G4 => Ok(Self::LeftPedal(Percent::new(alpha))),
                PianoKeyCode::G6 => Ok(Self::SetAmbience(Percent::new(alpha))),
                k => unreachable!("Unkown event 1: {:?}", k),
            }
        } else if (state_code as u8) == (StateCode::FunctionRelease as u8) {
            match key_code {
                PianoKeyCode::G6 => { Ok(Self::SetAmbience(Percent::new(alpha))) }
                k => { Err(format!("Unkown event 2: {:?}", k)) }
            }
        } else if (state_code as u8) == (StateCode::Other as u8) {
            match key_code {
                PianoKeyCode::G6 => { Ok(Self::SetAmbience(Percent::new(alpha))) }
                k => { Err(format!("Unkown event 2: {:?}", k)) }
            }
        } else {
            Err(format!("Unhandled event"))
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum StateCode {
    KeyPress = 144,
    KeyRelease = 128,
    FunctionBegin = 176,
    FunctionRelease = 178,
    Other = 181,
}

impl From<u8> for StateCode {
    fn from(key: u8) -> Self {
        match key {
            144 => Self::KeyPress,
            128 => Self::KeyRelease,
            176 => Self::FunctionBegin,
            178 => Self::FunctionRelease,
            181 => Self::Other,
            s => {
                panic!("Invalid State Code: {}", s);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Percent(f32);

impl Percent {
    pub fn new(alpha: Alpha) -> Self {
        Self((alpha.0 as f32) / 127.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Alpha(u8);

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PianoKeyCode {
    Eb0 = 15,
    E0 = 16,
    F0 = 17,
    Gb0 = 18,
    G0 = 19,
    Ab0 = 20,
    A1 = 21,
    Bb1 = 22,
    B1 = 23,
    C1 = 24,
    Db1 = 25,
    D1 = 26,
    Eb1 = 27,
    E1 = 28,
    F1 = 29,
    Gb1 = 30,
    G1 = 31,
    Ab1 = 32,
    A2 = 33,
    Bb2 = 34,
    B2 = 35,
    C2 = 36,
    Db2 = 37,
    D2 = 38,
    Eb2 = 39,
    E2 = 40,
    F2 = 41,
    Gb2 = 42,
    G2 = 43,
    Ab2 = 44,
    A3 = 45,
    Bb3 = 46,
    B3 = 47,
    C3 = 48,
    Db3 = 49,
    D3 = 50,
    Eb3 = 51,
    E3 = 52,
    F3 = 53,
    Gb3 = 54,
    G3 = 55,
    Ab3 = 56,
    A4 = 57,
    Bb4 = 58,
    B4 = 59,
    C4 = 60,
    Db4 = 61,
    D4 = 62,
    Eb4 = 63,
    E4 = 64,
    F4 = 65,
    Gb4 = 66,
    G4 = 67,
    Ab4 = 68,
    A5 = 69,
    Bb5 = 70,
    B5 = 71,
    C5 = 72,
    Db5 = 73,
    D5 = 74,
    Eb5 = 75,
    E5 = 76,
    F5 = 77,
    Gb5 = 78,
    G5 = 79,
    Ab5 = 80,
    A6 = 81,
    Bb6 = 82,
    B6 = 83,
    C6 = 84,
    Db6 = 85,
    D6 = 86,
    Eb6 = 87,
    E6 = 88,
    F6 = 89,
    Gb6 = 90,
    G6 = 91,
    Ab6 = 92,
    A7 = 93,
    Bb7 = 94,
    B7 = 95,
    C7 = 96,
    Db7 = 97,
    D7 = 98,
    Eb7 = 99,
    E7 = 100,
    F7 = 101,
    Gb7 = 102,
    G7 = 103,
    Ab7 = 104,
    A8 = 105,
    Bb8 = 106,
    B8 = 107,
    C8 = 108,
    Db8 = 109,
    D8 = 110,
    Eb8 = 111,
    E8 = 112,
    F8 = 113,
}

impl PianoKeyCode {
    pub fn to_key_name(self) -> String {
        (
            match self {
                Self::Eb0 => "D#",
                Self::E0 => "E",
                Self::F0 => "F",
                Self::Gb0 => "F#",
                Self::G0 => "G",
                Self::Ab0 => "G#",
                Self::A1 => "A",
                Self::Bb1 => "A#",
                Self::B1 => "B",
                Self::C1 => "C",
                Self::Db1 => "C#",
                Self::D1 => "D",
                Self::Eb1 => "D#",
                Self::E1 => "E",
                Self::F1 => "F",
                Self::Gb1 => "F#",
                Self::G1 => "G",
                Self::Ab1 => "G#",
                Self::A2 => "A",
                Self::Bb2 => "A#",
                Self::B2 => "B",
                Self::C2 => "C",
                Self::Db2 => "C#",
                Self::D2 => "D",
                Self::Eb2 => "D#",
                Self::E2 => "E",
                Self::F2 => "F",
                Self::Gb2 => "F#",
                Self::G2 => "G",
                Self::Ab2 => "G#",
                Self::A3 => "A",
                Self::Bb3 => "A#",
                Self::B3 => "B",
                Self::C3 => "C",
                Self::Db3 => "C#",
                Self::D3 => "D",
                Self::Eb3 => "D#",
                Self::E3 => "E",
                Self::F3 => "F",
                Self::Gb3 => "F#",
                Self::G3 => "G",
                Self::Ab3 => "G#",
                Self::A4 => "A",
                Self::Bb4 => "A#",
                Self::B4 => "B",
                Self::C4 => "C",
                Self::Db4 => "C#",
                Self::D4 => "D",
                Self::Eb4 => "D#",
                Self::E4 => "E",
                Self::F4 => "F",
                Self::Gb4 => "F#",
                Self::G4 => "G",
                Self::Ab4 => "G#",
                Self::A5 => "A",
                Self::Bb5 => "A#",
                Self::B5 => "B",
                Self::C5 => "C",
                Self::Db5 => "C#",
                Self::D5 => "D",
                Self::Eb5 => "D#",
                Self::E5 => "E",
                Self::F5 => "F",
                Self::Gb5 => "F#",
                Self::G5 => "G",
                Self::Ab5 => "G#",
                Self::A6 => "A",
                Self::Bb6 => "A#",
                Self::B6 => "B",
                Self::C6 => "C",
                Self::Db6 => "C#",
                Self::D6 => "D",
                Self::Eb6 => "D#",
                Self::E6 => "E",
                Self::F6 => "F",
                Self::Gb6 => "F#",
                Self::G6 => "G",
                Self::Ab6 => "G#",
                Self::A7 => "A",
                Self::Bb7 => "A#",
                Self::B7 => "B",
                Self::C7 => "C",
                Self::Db7 => "C#",
                Self::D7 => "D",
                Self::Eb7 => "D#",
                Self::E7 => "E",
                Self::F7 => "F",
                Self::Gb7 => "F#",
                Self::G7 => "G",
                Self::Ab7 => "G#",
                Self::A8 => "A",
                Self::Bb8 => "A#",
                Self::B8 => "B",
                Self::C8 => "C",
                Self::Db8 => "C#",
                Self::D8 => "D",
                Self::Eb8 => "D#",
                Self::E8 => "E",
                Self::F8 => "F",
            }
        ).to_string()
    }
}

impl From<u8> for PianoKeyCode {
    fn from(key: u8) -> Self {
        match key {
            15 => Self::Eb0,
            16 => Self::E0,
            17 => Self::F0,
            18 => Self::Gb0,
            19 => Self::G0,
            20 => Self::Ab0,
            21 => Self::A1,
            22 => Self::Bb1,
            23 => Self::B1,
            24 => Self::C1,
            25 => Self::Db1,
            26 => Self::D1,
            27 => Self::Eb1,
            28 => Self::E1,
            29 => Self::F1,
            30 => Self::Gb1,
            31 => Self::G1,
            32 => Self::Ab1,
            33 => Self::A2,
            34 => Self::Bb2,
            35 => Self::B2,
            36 => Self::C2,
            37 => Self::Db2,
            38 => Self::D2,
            39 => Self::Eb2,
            40 => Self::E2,
            41 => Self::F2,
            42 => Self::Gb2,
            43 => Self::G2,
            44 => Self::Ab2,
            45 => Self::A3,
            46 => Self::Bb3,
            47 => Self::B3,
            48 => Self::C3,
            49 => Self::Db3,
            50 => Self::D3,
            51 => Self::Eb3,
            52 => Self::E3,
            53 => Self::F3,
            54 => Self::Gb3,
            55 => Self::G3,
            56 => Self::Ab3,
            57 => Self::A4,
            58 => Self::Bb4,
            59 => Self::B4,
            60 => Self::C4,
            61 => Self::Db4,
            62 => Self::D4,
            63 => Self::Eb4,
            64 => Self::E4,
            65 => Self::F4,
            66 => Self::Gb4,
            67 => Self::G4,
            68 => Self::Ab4,
            69 => Self::A5,
            70 => Self::Bb5,
            71 => Self::B5,
            72 => Self::C5,
            73 => Self::Db5,
            74 => Self::D5,
            75 => Self::Eb5,
            76 => Self::E5,
            77 => Self::F5,
            78 => Self::Gb5,
            79 => Self::G5,
            80 => Self::Ab5,
            81 => Self::A6,
            82 => Self::Bb6,
            83 => Self::B6,
            84 => Self::C6,
            85 => Self::Db6,
            86 => Self::D6,
            87 => Self::Eb6,
            88 => Self::E6,
            89 => Self::F6,
            90 => Self::Gb6,
            91 => Self::G6,
            92 => Self::Ab6,
            93 => Self::A7,
            94 => Self::Bb7,
            95 => Self::B7,
            96 => Self::C7,
            97 => Self::Db7,
            98 => Self::D7,
            99 => Self::Eb7,
            100 => Self::E7,
            101 => Self::F7,
            102 => Self::Gb7,
            103 => Self::G7,
            104 => Self::Ab7,
            105 => Self::A8,
            106 => Self::Bb8,
            107 => Self::B8,
            108 => Self::C8,
            109 => Self::Db8,
            110 => Self::D8,
            111 => Self::Eb8,
            112 => Self::E8,
            113 => Self::F8,
            k => panic!("Invalid Piano Key Code: {}", k),
        }
    }
}

impl PianoKeyCode {
    pub fn new(key: u8) -> Self {
        key.into()
    }
}

pub fn listen<F>(
    handler: F,
    record: bool,
    receiver: Receiver<()>
) -> Result<Option<Arc<Mutex<Recording>>>, Box<dyn Error + Send + Sync>>
    where F: Fn(Result<PianoEvent, String>) + Send + 'static
{
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => {
            return Err("no input port found".into());
        }
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?).ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let recording = Arc::new(Mutex::new(Recording::new())); // Wrap Recording in Arc<Mutex<_>>
    let recording_clone = recording.clone(); // Create a clone for use in the closure
    let mut now = Instant::now();

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message: &[u8], _| {
            if record {
                recording_clone.lock().unwrap().push((now.elapsed(), message.to_vec()));
                now = Instant::now();
            }
            let state_code = StateCode::from(message[0]);
            let key_code = PianoKeyCode::from(message[1]);
            let alpha = Alpha(message[2]);
            let piano_event = PianoEvent::new(key_code, state_code, alpha);
            handler(piano_event);
        },
        ()
    )?;

    println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

    loop {
        match receiver.try_recv() {
            Ok(()) => {
                break;
            }
            _ => {}
        }
    }

    println!("Closing connection");

    if record {
        Ok(Some(recording))
    } else {
        Ok(None)
    }
}

pub fn play(recording: Recording) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => {
            return Err("no output port found".into());
        }
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports.get(input.trim().parse::<usize>()?).ok_or("invalid output port selected")?
        }
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            // We're ignoring errors in here
            let _ = conn_out.send(&[StateCode::KeyPress as u8, note, 60]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[StateCode::KeyRelease as u8, note, 127]);
        };

        for (time, record_chunk) in recording.recording {
            sleep(time);
            conn_out.send(&record_chunk);
        }
    }
    sleep(Duration::from_millis(150));
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
