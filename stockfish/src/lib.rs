use std::{
    io::{Error, Read, Write},
    process::{Child, Command, Stdio},
    string::FromUtf8Error,
    time::Duration,
};

const PATH_TO_STOCKFISH: &str = "../stockfish/sf";
const WELCOME_MESSAGE: &str =
    "Stockfish dev-20250126-f50d52aa by the Stockfish developers (see AUTHORS file)\nreadyok\n";

pub fn get_move(fen: &str, time: u32) -> Result<SFResults, SFErrors> {
    let mut sf = match new_sf() {
        Ok(sf) => sf,
        Err(rr) => return Err(rr),
    };
    let mut sfin = match sf.stdin.take() {
        Some(sfin) => sfin,
        None => return Err(SFErrors::SFInCreation),
    };
    let mut sfout = match sf.stdout.take() {
        Some(sfout) => sfout,
        None => return Err(SFErrors::SFOutCreation),
    };
    let mut output_buffer: [u8; WELCOME_MESSAGE.len()] = [0; 87];
    match sfin.write_all("isready\n".as_bytes()) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFInWriting(rr)),
    };
    match sfout.read_exact(&mut output_buffer) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFOutReading(rr)),
    };
    match String::from_utf8(output_buffer.to_vec()) {
        Ok(s) => match s.as_str() {
            WELCOME_MESSAGE => {}
            _ => return Err(SFErrors::SFTesting),
        },
        Err(rr) => return Err(SFErrors::SFOutReadingParsing(rr)),
    };
    match sfin.write_all(&format!("position fen {}\nd\ngo movetime {}\n", fen, time).as_bytes()) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFInWriting(rr)),
    };
    std::thread::sleep(Duration::from_millis((time + 100) as u64));
    match sfin.write_all(&format!("quit\n").as_bytes()) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFInWriting(rr)),
    };
    let mut res_buffer: Vec<u8> = Vec::new();
    match sfout.read_to_end(&mut res_buffer) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFOutReading(rr)),
    };
    let res = match String::from_utf8(res_buffer) {
        Ok(s) => {
            //println!("{}", s);
            s
        }
        Err(rr) => return Err(SFErrors::SFOutReadingParsing(rr)),
    };
    let n = match res.trim_ascii_end().split("\n").last() {
        Some(s) => {
            //println!("{}", s);
            s
        }
        None => return Err(SFErrors::SFProcessing),
    };
    //println!("{:?}", n);
    match n {
        "bestmove (none)" => return Ok(SFResults::Stalemate),
        _ => {
            if n.chars().count() == 25 {
                let wlist: Vec<&str> = n.split_whitespace().collect();
                if wlist.len() == 4 {
                    Ok(SFResults::Normal(String::from(wlist[1])))
                } else {
                    Err(SFErrors::SFProcessing)
                }
            } else if n.chars().count() == 13 {
                let wlist: Vec<&str> = n.split_whitespace().collect();
                if wlist.len() == 2 {
                    Ok(SFResults::Normal(String::from(wlist[1])))
                } else {
                    Err(SFErrors::SFProcessing)
                }
            } else {
                Err(SFErrors::SFProcessing)
            }
        }
    }
}

#[derive(Debug)]
pub enum SFErrors {
    CreationError(Error),
    SFInCreation,
    SFOutCreation,
    SFTesting,
    SFInWriting(Error),
    SFOutReading(Error),
    SFOutReadingParsing(FromUtf8Error),
    SFProcessing,
    Other,
}

#[derive(Debug)]
pub enum SFResults {
    Normal(String),
    Stalemate,
}

fn new_sf() -> Result<Child, SFErrors> {
    match Command::new(PATH_TO_STOCKFISH)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(ch) => Ok(ch),
        Err(rr) => Err(SFErrors::CreationError(rr)),
    }
}
