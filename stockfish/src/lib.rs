use std::{
    io::{Error, Read, Write},
    process::{Child, Command, Stdio},
    string::FromUtf8Error,
    time::Duration,
};

// specifies which sf binary to use depending on architecture

#[cfg(target_arch = "aarch64")]
const PATH_TO_STOCKFISH: &str = "../stockfish/sfs/sf_raspi";
#[cfg(target_arch = "aarch64")]
const  WELCOME_MESSAGE: &str = "Stockfish dev-20260426-1a882efc by the Stockfish developers (see AUTHORS file)\nreadyok\n";


#[cfg(target_arch = "x86_64")] 
const PATH_TO_STOCKFISH: &str = "../stockfish/sfs/sf_ubuntu";
#[cfg(target_arch = "x86_64")] 
const  WELCOME_MESSAGE: &str = "Stockfish dev-20260426-1a882efc by the Stockfish developers (see AUTHORS file)\nreadyok\n";


pub fn get_move(fen: &str, elo: u32, time: u32) -> Result<SFResults, SFErrors> {    // gives back error or move, given the position and sf parameters
    let mut sf = match new_sf() { // creates sf child
        Ok(sf) => sf,
        Err(rr) => return Err(rr),
    };

    // creates io handles of the sf child
    let mut sfin = match sf.stdin.take() {
        Some(sfin) => sfin,
        None => return Err(SFErrors::SFInCreation),
    };
    let mut sfout = match sf.stdout.take() {
        Some(sfout) => sfout,
        None => return Err(SFErrors::SFOutCreation),
    };

    // creating buffer for op, sending test-sequence, matching test-op to exspected value
    let mut output_buffer: [u8; WELCOME_MESSAGE.len()] = [0; WELCOME_MESSAGE.len()];
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

    // giving sf command to determine next move, waiting for answer and ending sf
    match sfin.write_all(&format!("position fen {}\nsetoption name UCI_Elo value {}\nsetoption name UCI_LimitStrength value true \ngo movetime {}\n", fen, elo, time).as_bytes()) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFInWriting(rr)),
    };
    std::thread::sleep(Duration::from_millis((time + 100) as u64));
    match sfin.write_all(&format!("quit\n").as_bytes()) {
        Ok(_) => {}
        Err(rr) => return Err(SFErrors::SFInWriting(rr)),
    };

    // reading result, parse to string, extracting move
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
    let res_list: Vec<&str> = res.trim_ascii_end().split("\n").collect();
    let n = match res_list.last() {
        Some(s) => {
            //println!("{}", s);
            *s
        }
        None => return Err(SFErrors::SFProcessing),
    };

    // determines type of result and returning it
    match n {
        "bestmove (none)" => {
            match res_list[res_list.len() - 2].split(" ").collect::<Vec<&str>>().iter().nth_back(1) {
                Some(v) => {
                    match *v {
                        "mate" => Ok(SFResults::Mate),
                        "cp" => Ok(SFResults::Stalemate),
                        _ => Err(SFErrors::SFProcessing)
                    }
                },
                None => Err(SFErrors::SFProcessing)
            }
        },
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
pub enum SFErrors { // Errortypes related to stockfish sf
    CreationError(Error),   // creation of sf child failed
    SFInCreation,   // creation of ip handle failed
    SFOutCreation,  // creation of op handle failed
    SFTesting,  // test sequence resulted in unforseen op
    SFInWriting(Error), // writing in sf ip failed
    SFOutReading(Error),    // writing in sf op failed
    SFOutReadingParsing(FromUtf8Error), // failed to parse op to string
    SFProcessing,   // result of sf can't be processed as exspected
}

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SFResults {    // all possible types of sf outputs
    Normal(String),
    Stalemate,
    Mate
}

fn new_sf() -> Result<Child, SFErrors> {    // creates new sf child process which is used by get_move()
    match Command::new(PATH_TO_STOCKFISH)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(ch) => Ok(ch),
        Err(rr) => Err(SFErrors::CreationError(rr)),
    }
}

#[cfg(test)]    // tests all three types of possible moves, see SFResults
mod tests {

    use super::{get_move, SFResults};

    #[test]
    fn it_works() {
        let result = get_move("8/1q6/5k2/K7/Pp6/8/8/8 w - - 1 54", 3000, 1000);
        assert_eq!(result.unwrap(), SFResults::Stalemate);
    }

    #[test]
    fn it_works2() {
        let result = get_move("8/3R4/3kp3/1Q6/1P6/8/1PK1P3/8 b - - 8 44", 3000, 1000);
        assert_eq!(result.unwrap(), SFResults::Mate);
    }

    #[test]
    fn it_works3() {
        let result = get_move("1R5k/5ppp/5n2/8/8/5K2/8/8 b - - 0 1", 3000, 1000);
        assert_eq!(result.unwrap(), SFResults::Normal("f6g8".to_string()));
    }
}