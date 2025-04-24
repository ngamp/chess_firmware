pub mod position {

    use std::{collections::HashMap, num::ParseIntError, cmp::{min, max}};
    use stockfish::{get_move, SFResults, SFErrors};
    use mctrl::motor::{Field, Speeds, FieldUsize, MotorInstructions, PosNow};


    #[derive(Debug)]
    #[derive(Clone, Copy)]
    #[derive(Eq, Hash, PartialEq)]
    pub enum Piece {
        King(bool),
        Queen(bool),
        Rook(bool),
        Knight(bool),
        Bishop(bool),
        Pawn(bool),
        None
    }

    #[derive(Debug)]
    pub enum DrawR {
        Repetition,
        FiftyMove,
        Stalemate
    }

    #[derive(Debug)]
    pub enum State {
        Normal,
        Mate(bool),
        Draw(DrawR),
        Resign(bool)
    }

    #[derive(Debug)]
    #[derive(Eq, Hash, PartialEq)]
    pub enum MoveType {
        Normal(Piece),
        Capturing(Piece, Piece),
        Rochade(Piece),
        EnPassant((usize, usize))
    }

    fn to_int(s: &str) -> i32 {
        match s.parse::<i32>() {
            Ok(i) => i,
            Err(_) => 0
        }
    }

    /*fn letter_p(s: &str) -> String {
        ["a","b","c","d","e","f","g","h"][["a","b","c","d","e","f","g","h"].iter().position(|x| *x == s).unwrap() + 1].to_owned()
    }*/

    /*fn letter_m(s: &str) -> String {
        ["a","b","c","d","e","f","g","h"][["a","b","c","d","e","f","g","h"].iter().position(|x| *x == s).unwrap() - 1].to_owned()
    }*/

    fn letter_to_int(s: &str) -> u32 {
        s.chars().map(|c| c as usize - 97).collect::<Vec<usize>>()[0] as u32
    }


    impl Piece {

        fn from_char(chr: char) -> Option<Self> {
            match chr {
                'K' => Some(Piece::King(true)),
                'Q' => Some(Piece::Queen(true)),
                'R' => Some(Piece::Rook(true)),
                'N' => Some(Piece::Knight(true)),
                'B' => Some(Piece::Bishop(true)),
                'P' => Some(Piece::Pawn(true)),
                'k' => Some(Piece::King(false)),
                'q' => Some(Piece::Queen(false)),
                'r' => Some(Piece::Rook(false)),
                'n' => Some(Piece::Knight(false)),
                'b' => Some(Piece::Bishop(false)),
                'p' => Some(Piece::Pawn(false)),
                _ => None
            }
        }

        fn piece_to_letter(&self) -> &str {
            match &self {
                Piece::King(true) => "K",
                Piece::King(false) => "k",
                Piece::Queen(true) => "Q",
                Piece::Queen(false) => "q",
                Piece::Rook(true) => "R",
                Piece::Rook(false) => "r",
                Piece::Bishop(true) => "B",
                Piece::Bishop(false) => "b",
                Piece::Knight(true) => "N",
                Piece::Knight(false) => "n",
                Piece::Pawn(true) => "P",
                Piece::Pawn(false) => "p",
                Piece::None => ""
            }
        }

        fn piece_to_color(&self) -> bool {
            match &self {
                Piece::King(b) => *b,
                Piece::Queen(b) => *b,
                Piece::Rook(b) => *b,
                Piece::Bishop(b) => *b,
                Piece::Knight(b) => *b,
                Piece::Pawn(b) => *b,
                Piece::None => true
            }
        }

        pub fn check_field(&self, startfield: &str, endfield: &str) -> bool {
            let (sl, sn) = startfield.split_at(1);
            let (el, en) = endfield.split_at(1);
            match self {
                Piece::None => false,
                Piece::Rook(_) => {
                    if sl == el || sn == en {
                        true
                    } else {
                        false
                    }
                },
                Piece::Bishop(_) => {
                    if (self::letter_to_int(sl) + self::to_int(sn) as u32)%2 == (self::letter_to_int(el) + self::to_int(en) as u32)%2 {
                        true
                    } else {
                        false
                    }
                },
                Piece::Queen(_) => {
                    if Piece::Bishop(true).check_field(startfield, endfield) || Piece::Rook(true).check_field(startfield, endfield) {
                        true
                    } else {
                        false
                    }
                },
                Piece::Knight(_) => {
                    if (self::letter_to_int(sl) + self::to_int(sn) as u32)%2 != (self::letter_to_int(el) + self::to_int(en) as u32)%2 && !(sl == el || sn == en) {
                        true
                    } else {
                        false
                    }
                },
                Piece::King(_) => {
                    match ((self::letter_to_int(sl) as i64 - self::letter_to_int(el) as i64), (self::to_int(sn) as i64 - self::to_int(en) as i64)) {
                        (-1..2, -1..2) => true,
                        _ => false
                    }
                },
                Piece::Pawn(true) => {
                    match ((self::letter_to_int(sl) as i64 - self::letter_to_int(el) as i64), (self::to_int(en) as i64 - self::to_int(sn) as i64)) {
                        (0, 1..3) => true,
                        (-1..2, 1) => true, 
                        _ => false
                    }
                },
                Piece::Pawn(false) => {
                    match ((self::letter_to_int(sl) as i64 - self::letter_to_int(el) as i64), (self::to_int(sn) as i64 - self::to_int(en) as i64)) {
                        (0, 1..3) => true,
                        (-1..2, 1) => true, 
                        _ => false
                    }
                }

            }

        }
        
    }

    #[derive(Debug)]
    pub enum MoveError {
        FenParse(ParseIntError),
        NoFigurStart,
        OwnFigurEnd,
        MoveNotFitPiece(Piece),
        MoveParse(ParseIntError),
        WrongFigurStart,
        UnrightCoordinates,
        UnallowedRochade,
        ImpossiblePosition,
        CleaningError(CleaningError)
    }

    #[derive(Debug)]
    #[derive(Clone, Copy)]
    pub enum PFIType {
        NMove((usize, usize), (usize, usize)),
        Rochade(Piece, [(usize, usize); 4]),
        Custom((usize, usize), (usize, usize))
    }

    #[derive(Debug)]
    pub enum UpdateError {
        ImpossibleMove(MoveError),
        EnpassantMissing,
        CleaningError(CleaningError),
        SFError(SFErrors),
        Other
    }

    #[derive(Debug)]
    pub enum CleaningError {
        KingBeforeQueen,
        ImpossiblePosition
    }

    #[derive(Debug)]
    pub enum PFError {
        MoveDoesNotFitType(PFIType),
        Rochade(u32),
        Stuck
    }
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Position {
        pub colorw: bool,
        pub fields: [[Piece;14];8],
        pub moves: u32,
        pub en_passant: String,
        pub rochade: [Piece;4],
        pub since_pawn_major: u32
    }    

    impl Position {
        pub fn new_reset() -> Self {
            Position {
                colorw: true,
                fields: [[Piece::None,Piece::None,Piece::None,Piece::Rook(false),Piece::Knight(false),Piece::Bishop(false),Piece::Queen(false),Piece::King(false),Piece::Bishop(false),Piece::Knight(false),Piece::Rook(false),Piece::None,Piece::None,Piece::Queen(false)],
                    [Piece::None,Piece::None,Piece::None,Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::None,Piece::None,Piece::None],
                    [Piece::Queen(true),Piece::None,Piece::None,Piece::Rook(true),Piece::Knight(true),Piece::Bishop(true),Piece::Queen(true),Piece::King(true),Piece::Bishop(true),Piece::Knight(true),Piece::Rook(true),Piece::None,Piece::None,Piece::None]],
                moves: 0,
                en_passant: "-".to_string(),
                rochade: [Piece::King(true),Piece::Queen(true),Piece::King(false),Piece::Queen(false)],
                since_pawn_major: 0
            }
        }

        pub fn to_fen(&self) -> String {
            let mut res = String::new();
    
            //get piece position
            for i in 0..8 {
                let mut row = String::new();
                let lis = self.fields[i];
                let mut empty_counter = 0;
                for field in &lis[3..11] {
                    match field {
                        Piece::None => {
                            empty_counter += 1;
                        },
                        _ => {
                            if empty_counter > 0 {
                            row.push_str(&(empty_counter.to_string()));
                            }
                            empty_counter = 0;
                            row.push_str(&field.piece_to_letter());
                        }
                    };
                }
                if empty_counter > 0 {
                    row.push_str(&(empty_counter.to_string()));
                }
                if i < 7 {
                    row.push_str("/");
                } else {
                    row.push_str(" ");
                }
                res.push_str(&row)
            };
    
            //get move-right
            if self.colorw {
                res.push_str("w ");
            } else {
                res.push_str("b ");
            }
    
            //rochade-rights
            let mut rochade_res = String::new();
            for piece in self.rochade {
                rochade_res.push_str(&piece.piece_to_letter());
            }
            if rochade_res == "".to_string() {
                rochade_res.push_str("-");
            }
            res.push_str(&rochade_res);
    
            //en_passant?
            res.push_str(&format!(" {}", self.en_passant));
    
            //moves (50-moves-rule)
            res.push_str(&format!(" {}", self.since_pawn_major / 2));
    
            //movenumber
            res.push_str(&format!(" {}", (self.moves / 2) + 1));
    
    
            res
        }

        pub fn from_fen(fen: &str) -> Result<Self, MoveError> {
            let fen: Vec<&str> = fen.split_ascii_whitespace().collect();
            let colorw = match fen[1] {
                "w" => true,
                _ => false
            };
            let moves = match fen[5].to_string().parse::<u32>() {
                Ok(num) => {
                    if colorw {
                        (num-1)*2
                    } else {
                        (num-1)*2 + 1
                    }
                },
                Err(rr) => return Err(MoveError::FenParse(rr))
            };
            let en_passant = fen[3].to_string();
            let mut rochade = [Piece::None; 4];
            fen[2].chars().for_each(|l| {
                match l {
                    'K' => rochade[0] = Piece::King(true),
                    'Q' => rochade[1] = Piece::Queen(true),
                    'k' => rochade[2] = Piece::King(false),
                    'q' => rochade[3] = Piece::Queen(false),
                    _ => {}
                }
            });
            let since_pawn_major = match fen[4].parse::<u32>() {
                Ok(num) => num,
                Err(rr) => return Err(MoveError::FenParse(rr))
            };
            let mut all_pieces: HashMap<Piece, u8> = HashMap::new();
            for v in [true, false] {
                all_pieces.insert(Piece::King(v), 1);
                all_pieces.insert(Piece::Queen(v), 1);
                all_pieces.insert(Piece::Rook(v), 2);
                all_pieces.insert(Piece::Knight(v), 2);
                all_pieces.insert(Piece::Bishop(v), 2);
                all_pieces.insert(Piece::Pawn(v), 8);
            };
            let mut fields = [[Piece::None; 14]; 8];
            let rows: Vec<&str> = fen[0].split("/").collect();
            let mut i = 0;
            for row in rows {
                let row: Vec<char> = row.chars().collect();
                let mut j = 3;
                for chr in row {
                    if let Some(pce) = Piece::from_char(chr) {
                        *(all_pieces.entry(pce).or_insert(0)) -= 1;
                        fields[i][j] = pce;
                        j += 1;
                    } else {
                        match chr.to_string().parse::<usize>() {
                            Ok(num) => j += num,
                            Err(rr) => return Err(MoveError::FenParse(rr))
                        };
                    }
                };
                i += 1;
            }
            let mut pos = Position { colorw, fields, moves, en_passant, rochade, since_pawn_major};
            let mut scnd_run = all_pieces.clone();
            let mut rerun = false;
            for (k, v) in all_pieces.iter() {
                for _ in 0..*v {
                    match pos.add_rest(k.clone()) {
                        Ok(_) => *(scnd_run.entry(*k).or_insert(*v)) -= 1,
                        Err(_) => rerun = true,
                    }
                };
            };
            if rerun {
                for (k, v) in scnd_run.iter() {
                    for _ in 0..*v {
                        match pos.add_rest(k.clone()) {
                            Ok(_) => {},
                            Err(_) => return Err(MoveError::ImpossiblePosition)
                        }
                    }
                }
            }
            pos.add_rest(Piece::Queen(true)).unwrap();
            pos.add_rest(Piece::Queen(false)).unwrap();
            Ok(pos)

        }

        pub fn add_rest(&mut self, pce: Piece) -> Result<(usize, usize), CleaningError> {
            match pce {
                Piece::Queen(true) => {
                    if self.field_is_empty((7, 0)) {
                        self.fields[7][0] = Piece::Queen(true);
                        Ok((7, 0))
                    } else {
                        self.fields[7][1] = Piece::Queen(true);
                        Ok((7, 1))
                    }
                },
                Piece::Queen(false) => {
                    if self.field_is_empty((0, 13)) {
                        self.fields[0][13] = Piece::Queen(true);
                        Ok((0, 13))
                    } else {
                        self.fields[0][12] = Piece::Queen(true);
                        Ok((0, 12))
                    }
                },
                Piece::King(true) => {
                    self.fields[7][2] = Piece::King(true);
                    Ok((7, 2))
                },
                Piece::King(false) => {
                    self.fields[0][11] = Piece::King(true);
                    Ok((0, 11))
                },
                Piece::Rook(true) => {
                    let (r, s) = (6, 0);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Rook(true);
                        Ok((r, s))
                    } else {
                        self.fields[r][s+1] = Piece::Rook(true);
                        Ok((r, s+1))
                    }
                },
                Piece::Rook(false) => {
                    let (r, s) = (1, 13);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Rook(false);
                        Ok((r, s))
                    } else {
                        self.fields[r][s-1] = Piece::Rook(false);
                        Ok((r, s-1))
                    }
                },
                Piece::Knight(true) => {
                    let (r, s) = (5, 0);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Knight(true);
                        Ok((r, s))
                    } else {
                        self.fields[r][s+1] = Piece::Knight(true);
                        Ok((r, s+1))
                    }
                },
                Piece::Knight(false) => {
                    let (r, s) = (2, 13);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Knight(false);
                        Ok((r, s))
                    } else {
                        self.fields[r][s-1] = Piece::Knight(false);
                        Ok((r, s-1))
                    }
                },
                Piece::Bishop(true) => {
                    let (r, s) = (4, 0);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Bishop(true);
                        Ok((r, s))
                    } else {
                        self.fields[r][s+1] = Piece::Bishop(true);
                        Ok((r, s+1))
                    }
                },
                Piece::Bishop(false) => {
                    let (r, s) = (3, 13);
                    if self.field_is_empty((r, s)) {
                        self.fields[r][s] = Piece::Bishop(false);
                        Ok((r, s))
                    } else {
                        self.fields[r][s-1] = Piece::Bishop(false);
                        Ok((r, s-1))
                    }
                },
                Piece::Pawn(true) => {
                    for r in 0..4 {
                        for s in 0..2 {
                            if self.field_is_empty((r, s)) {
                                self.fields[r][s] = Piece::Pawn(true);
                                return Ok((r, s))
                            }
                        }
                    };
                    Err(CleaningError::ImpossiblePosition)
                },
                Piece::Pawn(false) => {
                    for r in 0..4 {
                        for s in 0..2 {
                            if self.field_is_empty((7-r, 13-s)) {
                                self.fields[7-r][13-s] = Piece::Pawn(false);
                                return Ok((7-r, 13-s))
                            }
                        }
                    };
                    Err(CleaningError::ImpossiblePosition)
                }
                _ => return Err(CleaningError::KingBeforeQueen)
            }
        }

        pub fn field_is_empty(&self, field: (usize, usize)) -> bool {
            if self.fields[field.0][field.1] == Piece::None {
                true
            } else {
                false
            }
        }

        pub fn update(&mut self, ind_move: ((usize, usize), (usize, usize)), coord_move: &str, elo: u32, time: u32) -> Result<(State, Vec<PFIType>), UpdateError> {
            let mt = match self.validate_move_possibility(coord_move) {
                Err(rr) => return Err(UpdateError::ImpossibleMove(rr)),
                Ok(mt) => mt
            };
            let piece = self.index_to_piece(ind_move.0).unwrap(); //existence already checked at validate_move_possibility
            // inverting color
            println!("{:?}", piece);
            self.colorw = !self.colorw;
            //adding 1 move 
            self.moves += 1;
            // writing enpassant string, adjusting rochade and since_pawn_major
            self.since_pawn_major += 1;
            match piece {
                Piece::King(c) => {
                    if c {
                        self.rochade[0] = Piece::None;
                        self.rochade[1] = Piece::None;
                    } else {
                        self.rochade[2] = Piece::None;
                        self.rochade[3] = Piece::None;
                    }
                },
                Piece::Rook(_) => {
                    match ind_move.0 {
                        (0, 3) => self.rochade[3] = Piece::None,
                        (0,10) => self.rochade[2] = Piece::None,
                        (7, 3) => self.rochade[1] = Piece::None,
                        (7, 10) => self.rochade[0] = Piece::None,
                        _ => {}
                    }
                },
                Piece::Pawn(c) => {
                    self.since_pawn_major = 0;
                    self.en_passant = "-".to_string();
                    if c {
                        if ind_move.0.0 == 6 && ind_move.1.0 == 4 {
                            self.en_passant = format!("{}3", coord_move.chars().nth(0).unwrap());
                        };
                    } else {
                        if ind_move.0.0 == 1 && ind_move.1.0 == 3 {
                            self.en_passant = format!("{}6", coord_move.chars().nth(0).unwrap());
                        };
                    }
                },
                _ => {}
            };
            //update fields (and since_pawn_major)
            let mut moves: Vec<PFIType> = Vec::new();
            let ((sfr, sfs), (efr, efs)) = ind_move;
            match mt {
                MoveType::Normal(p) => {
                    self.fields[efr][efs] = p;
                    self.fields[sfr][sfs] = Piece::None;
                    moves.push(PFIType::NMove(ind_move.0, ind_move.1));
                },
                MoveType::Capturing(p, cp) => {
                    self.since_pawn_major = 0;
                    let rest_ind = match self.add_rest(cp) {
                        Ok(t) => t,
                        Err(rr) => return Err(UpdateError::CleaningError(rr))
                    };
                    moves.push(PFIType::Custom(ind_move.1, rest_ind));
                    self.fields[sfr][sfs] = Piece::None;
                    self.fields[efr][efs] = p;
                    moves.push(PFIType::NMove(ind_move.0, ind_move.1));
                },
                MoveType::EnPassant(bind) => {
                    let beaten_piece = match self.index_to_piece(bind) {
                        Some(i) => i,
                        None => return Err(UpdateError::EnpassantMissing)
                    };
                    let rest_ind = match self.add_rest(beaten_piece) {
                        Ok(t) => t,
                        Err(rr) => return Err(UpdateError::CleaningError(rr))
                    };
                    moves.push(PFIType::Custom(bind, rest_ind));
                    self.fields[bind.0][bind.1] = Piece::None;
                    self.fields[sfr][sfs] = Piece::None;
                    self.fields[efr][efs] = Piece::Pawn(!beaten_piece.piece_to_color());
                    moves.push(PFIType::NMove(ind_move.0, ind_move.1));
                },
                MoveType::Rochade(p) => {
                    let (ks, ke, rs, re) = match p {
                        Piece::King(true) => (coordinates_to_index("e1").unwrap(), coordinates_to_index("g1").unwrap(), coordinates_to_index("h1").unwrap(), coordinates_to_index("f1").unwrap()),
                        Piece::Queen(true) => (coordinates_to_index("e1").unwrap(), coordinates_to_index("c1").unwrap(), coordinates_to_index("a1").unwrap(), coordinates_to_index("d1").unwrap()),
                        Piece::King(false) => (coordinates_to_index("e8").unwrap(), coordinates_to_index("g8").unwrap(), coordinates_to_index("h8").unwrap(), coordinates_to_index("f8").unwrap()),
                        Piece::Queen(false) => (coordinates_to_index("e8").unwrap(), coordinates_to_index("c8").unwrap(), coordinates_to_index("a8").unwrap(), coordinates_to_index("d8").unwrap()),
                        _ => return Err(UpdateError::Other)
                    };
                    self.fields[ks.0][ks.1] = Piece::None;
                    self.fields[ke.0][ke.1] = Piece::King(p.piece_to_color());
                    self.fields[rs.0][rs.1] = Piece::None;
                    self.fields[re.0][re.1] = Piece::Rook(p.piece_to_color());
                    moves.push(PFIType::Rochade(p, [ks, ke, rs, re]));
                }
            };
            let state = match get_move(&self.to_fen(), elo, time) {
                Err(rr) => return Err(UpdateError::SFError(rr)),
                Ok(s) => {
                    match s {
                        SFResults::Normal(_) => State::Normal,
                        SFResults::Stalemate => State::Draw(DrawR::Stalemate),
                        SFResults::Mate => State::Mate(!self.colorw)
                    }
                }
            };
            Ok((state, moves))
        }

        pub fn validate_move_possibility(&self, cmove: &str) -> Result<MoveType, MoveError> {
            match cmove {
                "e1g1" => {
                    match self.rochade[0] {
                        Piece::King(true) => return Ok(MoveType::Rochade(Piece::King(true))),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e1c1" => {
                    match self.rochade[1] {
                        Piece::Queen(true) => return Ok(MoveType::Rochade(Piece::Queen(true))),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e8g8" => {
                    match self.rochade[2] {
                        Piece::King(false) => return Ok(MoveType::Rochade(Piece::King(false))),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e8c8" => {
                    match self.rochade[3] {
                        Piece::Queen(false) => return Ok(MoveType::Rochade(Piece::Queen(false))),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                _ => {}
            };
            let (start_field, end_field) = cmove.split_at(2);
            let piece = self.coordinates_to_piece(start_field)?;
            if let Some(piece) = piece {
                match (self.en_passant.as_str(), piece) {
                    ("-",  _) => {},
                    (mm, Piece::Pawn(_)) => {
                        if mm == cmove.split_at(2).1 {
                            let mut ind_mm = coordinates_to_index(mm)?;
                            if ind_mm.0 == 2 {
                                ind_mm.0 += 1
                            } else {
                                ind_mm.0 -= 1
                            };
                            return Ok(MoveType::EnPassant(ind_mm))
                        }
                    },
                    (_, _) => {}
                }
                if piece == Piece::None {
                    return Err(MoveError::NoFigurStart);
                } else {
                    if self.colorw ^ piece.piece_to_color() {
                        return Err(MoveError::WrongFigurStart)
                    }
                };
                match &piece.check_field(start_field, end_field) {
                    true => {},
                    false => return Err(MoveError::MoveNotFitPiece(piece))
                };
                let capt_piece = self.coordinates_to_piece(end_field)?;
                if let Some(capt_piece) = capt_piece {
                    if piece.piece_to_color() == capt_piece.piece_to_color() {
                        return Err(MoveError::OwnFigurEnd)
                    };
                    Ok(MoveType::Capturing(piece, capt_piece))
                } else {
                    Ok(MoveType::Normal(piece))
                }
            } else {
                return Err(MoveError::NoFigurStart)
            }            
        }

        pub fn coordinates_to_piece(&self, coord: &str) -> Result<Option<Piece>, MoveError> {
            let res = coordinates_to_index(coord)?;
            Ok(self.index_to_piece(res))
        }

        pub fn index_to_piece(&self, ind: (usize, usize)) -> Option<Piece> {
            if self.field_is_empty(ind) {
                None
            } else {
                Some(self.fields[ind.0][ind.1])
            }
        }

        pub fn pathfinding(&self, vmove: &Vec<PFIType>, pos: &mut PosNow) -> Result<MotorInstructions, PFError> {
            let mut res = MotorInstructions::new();
            let mut bitlist = BitList::from_pos(self);
            for mov in vmove {
                match *mov {
                    PFIType::NMove(start, end) => {
                        if start.0 == end.0 || start.1 == end.1 {
                            res.append_wo_pos(MotorInstructions::field_to_field(Field::ind_to_relative_ind(start), Field::ind_to_relative_ind(end), Speeds::NMovespeed, true, pos));
                        } else if start.0.abs_diff(end.0) == start.1.abs_diff(end.1) {
                            res.append_wo_pos(MotorInstructions::diagonal(Field::ind_to_relative_ind(start), Field::ind_to_relative_ind(end), Speeds::NMovespeed, true, pos));
                        } else {
                            return Err(PFError::MoveDoesNotFitType(*mov))
                        }
                    },
                    PFIType::Rochade(p, coords) => {
                        res.append_wo_pos(pathfinding_rochade(p, coords, &mut BitList::from_pos(self), pos)?)
                    },
                    PFIType::Custom(sf, ef) => {
                        res.append_wo_pos(pathfinding_custom(FieldUsize::from_tuple(sf), FieldUsize::from_tuple(ef), &mut bitlist, pos)?);
                    }
                };
            };
            Ok(res)
        }
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct OneFML(pub Vec<FieldUsize>);

    impl OneFML {
        pub fn new() -> Self {
            OneFML(Vec::new())
        }

        pub fn add(&mut self, f: FieldUsize) {
            self.0.push(f);
        }

        pub fn remove_last(&mut self) -> FieldUsize {
            self.0.pop().unwrap()
        }

        pub fn to_mi(self, pos: &mut PosNow) -> MotorInstructions {
            let movl = self.0;
            let mut res = MotorInstructions::new();
            if pos.sfh_to_field() != movl[0].to_field() {
                res.append_wo_pos(MotorInstructions::field_to_field(pos.sfh_to_field(), movl[0].to_field(), Speeds::NoFigurespeed, false, pos));
            };
            let mut i = 0;
            while i+1 < movl.len() {
                let vfield = movl[i+1].to_field()-movl[i].to_field();
                match vfield.to_tuple() {
                    (1.0 | -1.0, 1.0 | -1.0) => {
                        res.append_wo_pos(MotorInstructions::diagonal(movl[i].to_field(), movl[i+1].to_field(), Speeds::Transportspeed, true, pos));
                    },
                    _ => {
                        res.append(MotorInstructions::from_vfield(vfield, Speeds::Transportspeed, true), pos);
                    }
                };
                i += 1
            };
            res
        }

        pub fn ease(self) -> Self {
            let mut mvl = self.0;
            let mut j = mvl.len() - 1;
            let mut i = 0;
            while j > 0 {
                while i < j {
                    let ef = mvl[j];
                    let sf = mvl[i];
                    match (ef.0.abs_diff(sf.0), ef.1.abs_diff(sf.1)) {
                        (1, 0) | (1, 1) | (0, 1) => {
                            j -= mvl.drain(i+1..j).collect::<Vec<FieldUsize>>().len();
                        },
                        (0, 0) => {
                            j -= mvl.drain(i..j).collect::<Vec<FieldUsize>>().len();
                        },
                        _ => {}
                    };
                    i += 1;
                };
                j -= 1;
                i = 0;
            };
            Self(mvl)
        }

        pub fn append(mut self, mut other: Self) -> Self {
            self.0.append(&mut other.0);
            self
        }

        pub fn pf_hf(mut sf: FieldUsize, ef: FieldUsize) -> Self {
            let mut res = Self::new();
            res.add(sf);
            let xdir = ef.1 > sf.1;
            let ydir = ef.0 > sf.0;
            while sf.1 != ef.1 {
                sf = sf.edit_x(xdir);
                res.add(sf);
            };
            while sf.0 != ef.0 {
                sf = sf.edit_y(ydir);
                res.add(sf);
            };
            res
        }

        pub fn pf_vf(mut sf: FieldUsize, ef: FieldUsize) -> Self {
            let mut res = Self::new();
            res.add(sf);
            let xdir = ef.1 > sf.1;
            let ydir = ef.0 > sf.0;
            while sf.0 != ef.0 {
                sf = sf.edit_y(ydir);
                res.add(sf);
            };
            while sf.1 != ef.1 {
                sf = sf.edit_x(xdir);
                res.add(sf);
            };
            res
        }
    }

    pub fn pathfinding_custom(sf: FieldUsize, ef: FieldUsize, bl: &mut BitList, pos: &mut PosNow) -> Result<MotorInstructions, PFError> {
        if bl.count_area(sf, ef) == 0 {
            return Ok(MotorInstructions::field_to_field(Field::from_field_usize(sf), Field::from_field_usize(ef), Speeds::NMovespeed, true, pos))
        };
        bl.update(vec![sf.to_tuple()], vec![], vec![]);
        let movlist = OneFML::new();
        match pf_custom_helper(sf, sf, ef, bl, movlist) {
            Ok(ml) => {
                let mut res = ml.ease().to_mi(pos);
                res.ease();
                Ok(res)
            },
            Err(_) => {
                let mut res = pf_stuck(sf, ef, bl, pos)?;
                res.ease();
                Ok(res)
            }
        }
    }

    pub fn pf_stuck(sf: FieldUsize, ef: FieldUsize, bl: &mut BitList, pos: &mut PosNow) -> Result<MotorInstructions, PFError> {
        let mut res  = MotorInstructions::new();
        let mut moved_pieces = Vec::new();
        bl.update(vec![sf.to_tuple(), ef.to_tuple()], vec![], vec![]);
        let mut fastest = 0;
        for field in pf_custom_helper(sf, sf, ef, &mut BitList::new(), OneFML::new())?.0 {
            if bl.check_field(field) {
                fastest += 1
            }
        };
        let mut vf = 0;
        vf += bl.count_area(sf, FieldUsize::from_tuple((sf.0, ef.1)));
        vf += bl.count_area(FieldUsize::from_tuple((sf.0, ef.1)), ef);
        let mut hf = 0;
        hf += bl.count_area(sf, FieldUsize::from_tuple((ef.0, sf.1)));
        hf += bl.count_area(FieldUsize::from_tuple((ef.0, sf.1)), ef);
        let path = if fastest <= vf && fastest <= hf {
            pf_custom_helper(sf, sf, ef, &mut BitList::new(), OneFML::new())?
        } else if vf < hf {
            OneFML::pf_vf(sf, ef)
        } else {
            OneFML::pf_hf(sf, ef)
        };
        let mut fidw = Vec::new();
        for field in path.0.clone() {
            if bl.check_field(field) {
                fidw.push(field);
            }
        };
        let mut fidwr = fidw.clone();
        fidwr.reverse();
        for field in fidwr {
            let mut poss_fields = field.get_neighbors();
            for fff in path.0.clone() {
                poss_fields.retain(|x| *x != fff);
            };
            let mut free = false;
            let mut esc_f = FieldUsize(0, 0);
            for ff in &poss_fields {
                if !bl.check_field(*ff) {
                    esc_f = *ff;
                    free = true;
                    break
                }
            }
            if !free {
                let mut free2 = false;
                let mut esc_field2 = FieldUsize(0, 0);
                'outer: for poss_esc in poss_fields {
                    let mut poss_fields2 = poss_esc.get_neighbors();
                    for fff in path.0.clone() {
                        poss_fields2.retain(|x| *x != fff && *x != poss_esc);
                    };
                    for ff2 in poss_fields2 {
                        if !bl.check_field(ff2) {
                            esc_field2 = ff2;
                            esc_f = poss_esc;
                            println!("{:?}", esc_field2);
                            free2 = true;
                            break 'outer;
                        }
                    }
                };
                if free2 {
                    res.append_wo_pos(MotorInstructions::diagonal(esc_f.to_field(), esc_field2.to_field(), Speeds::Transportspeed, true, pos));
                    moved_pieces.push((esc_field2.to_field(), esc_f.to_field()));
                } else {
                    return Err(PFError::Stuck)
                }
            };
            res.append_wo_pos(MotorInstructions::diagonal(field.to_field(), esc_f.to_field(), Speeds::Transportspeed, true, pos));
            moved_pieces.push((esc_f.to_field(), field.to_field()));
        }
        let mut respath = path.to_mi(pos);
        respath.ease();
        respath.print_out();
        res.append_wo_pos(respath);
        moved_pieces.reverse();
        for i in moved_pieces {
            res.append_wo_pos(MotorInstructions::diagonal(i.0, i.1, Speeds::Transportspeed, true, pos));
        }
        Ok(res)
    }

    pub fn pf_custom_helper(ogsf: FieldUsize, sf: FieldUsize, ef: FieldUsize, bl: &mut BitList, mut movlist: OneFML) -> Result<OneFML, PFError> {
        //bl.print_out();
        println!("{:?}", movlist);
        if sf == ef {
            let mut res = OneFML::new();
            res.add(ogsf);
            return Ok(res.append(movlist))
        };
        //let ml = movlist.0.clone();
        //ml.reverse();
        let mut pml = sf.get_nearby(&ef);
        for banfield in &movlist.0 {
            if pml.contains(&banfield) {
                pml.retain(|y| y != banfield);
                pml.push(*banfield);
            }
        };
        for field in pml {
            if !bl.check_field(field) {
                while movlist.0.contains(&field) {
                    bl.update(vec![], vec![], vec![movlist.remove_last().to_tuple()]); 
                }
                movlist.add(field);
                bl.update(vec![], vec![], vec![field.to_tuple()]);
                return pf_custom_helper(ogsf, field, ef, bl, movlist)                
            }
        }
        Err(PFError::Stuck)
    }

    pub fn pathfinding_rochade(p: Piece, coords: [(usize, usize); 4], bl: &mut BitList, pos: &mut PosNow) -> Result<MotorInstructions, PFError> {
        let col = p.piece_to_color();
        let working_area = if col {
            BitList(bl.0[6..8].to_vec())
        } else {
            BitList(bl.0[0..2].to_vec())
        };
        let (kng, koq) = match p {
            Piece::King(_) => (FieldUsize::from_tuple((0u32, 7u32)), true),
            Piece::Queen(_) => (FieldUsize::from_tuple((0u32, 6u32)), false),
            _ => return Err(PFError::Rochade(line!()))
        };
        let mut res = MotorInstructions::new();
        if (!working_area.check_coords(kng.add_y(1).add_x(1).to_tuple()) && koq) || (!working_area.check_coords(kng.add_y(1).sub_x(1).to_tuple()) && !koq) {
            res.append_wo_pos(pf_rochade_helper(coords, koq, col, pos))
        } else if !working_area.check_coords(kng.add_y(1).to_tuple()) {
            let fak = if !col {
                kng.add_y(1)
            } else {
                kng.add_y(7)
            };
            let fntfak = if koq {
                fak.add_x(1)
            } else {
                fak.sub_x(1)
            };
            res.append_wo_pos(MotorInstructions::field_to_field(Field::from_field_usize(fntfak), Field::from_field_usize(fak), Speeds::Transportspeed, true, pos));
            res.append_wo_pos(pf_rochade_helper(coords, koq, col, pos));
            res.append_wo_pos(MotorInstructions::field_to_field(Field::from_field_usize(fak), Field::from_field_usize(fntfak), Speeds::Transportspeed, true, pos));
        } else {
            let mut fntr = if koq {
                FieldUsize(0, 11)
            } else {
                FieldUsize(0, 2)
            };
            if col {
                fntr = fntr.add_y(8)
            };
            let esc_f = if !col {
                fntr.add_y(1)
            } else {
                fntr.sub_y(1)
            };
            res.append_wo_pos(MotorInstructions::diagonal(Field::ind_to_relative_ind(coords[2]), Field::from_field_usize(esc_f), Speeds::Transportspeed, true, pos));
            res.append_wo_pos(MotorInstructions::field_to_field(Field::ind_to_relative_ind(coords[0]), Field::from_field_usize(fntr), Speeds::Transportspeed, true, pos));
            res.append_wo_pos(MotorInstructions::diagonal(Field::from_field_usize(esc_f), Field::ind_to_relative_ind(coords[2]), Speeds::Transportspeed, true, pos));
            res.append_wo_pos(MotorInstructions::field_to_field(Field::ind_to_relative_ind(coords[2]), Field::ind_to_relative_ind(coords[3]), Speeds::Transportspeed, true, pos));
            res.append_wo_pos(MotorInstructions::field_to_field(Field::from_field_usize(fntr), Field::ind_to_relative_ind(coords[1]), Speeds::Transportspeed, true, pos));
        };
        Ok(res)
    }

    fn pf_rochade_helper(coords: [(usize, usize); 4], koq: bool, col: bool, pos: &mut PosNow) -> MotorInstructions {
        let mut res = MotorInstructions::new();
        let mut esc_f = if koq {
            FieldUsize(1, 8)
        } else {
            FieldUsize(1, 6)
        };
        if col {
            esc_f = esc_f.add_y(5)
        };
        res.append_wo_pos(MotorInstructions::diagonal(Field::ind_to_relative_ind(coords[0]), Field::from_field_usize(esc_f), Speeds::Transportspeed, true, pos));
        res.append_wo_pos(MotorInstructions::field_to_field(Field::ind_to_relative_ind(coords[2]), Field::ind_to_relative_ind(coords[3]), Speeds::Transportspeed, true, pos));
        res.append_wo_pos(MotorInstructions::diagonal(Field::from_field_usize(esc_f), Field::ind_to_relative_ind(coords[1]), Speeds::Transportspeed, true, pos));
        res
    }

    pub fn coordinates_to_index(coordinate: &str) -> Result<(usize, usize), MoveError> {
        let (letter, number) = coordinate.split_at(1);
        let mut num: usize = match number.parse() {
            Ok(n) => n,
            Err(rr) => return Err(MoveError::MoveParse(rr))
        };
        match num {
            1..9 => {},
            _ => return Err(MoveError::UnrightCoordinates)
        }
        num = 8 - num;
        let lett = letter.chars().map(|c| c as usize - 94).collect::<Vec<usize>>()[0];
        match (num, lett) {
            (0..8, 3..11) => {},
            _ => return Err(MoveError::UnrightCoordinates)
        };
        Ok((num, lett))
    }

    #[derive(Debug)]
    pub struct BitList(pub Vec<[(bool, u8); 14]>);

    impl BitList {

        pub fn new() -> Self {
            Self(vec![[(false, 8); 14]; 8])
        }

        pub fn from_pos(position: &Position) -> Self {
            let mut res = [[(false, 8); 14]; 8].to_vec();
            for (rownum, row) in position.fields.iter().enumerate() {
                for (num, field) in row.iter().enumerate() {
                    let nei = FieldUsize(rownum, num).get_neighbors();
                    if *field != Piece::None {
                        res[rownum][num] = (true, 8);
                        for neighbor in &nei {
                            res[neighbor.0][neighbor.1].1 -= 1;
                        }
                    }
                    res[rownum][num].1 -= (8-nei.len()) as u8
                }
                
            };
            Self(res)
        }

        pub fn print_out(&self) {
            println!("BitList");
            /*for i in &self.0 {
                println!("  {:?}", i.iter().map(|x| x.0).collect::<Vec<bool>>());
            }*/
            for i in &self.0 {
                println!("{:?}", i)
            }
        }

        pub fn update(&mut self, to_remove: Vec<(usize, usize)>, to_add: Vec<(usize, usize)>, to_count: Vec<(usize, usize)>) {
            for (y, x) in to_remove {
                if y < 8 && x < 14 {
                    self.0[y][x].0 = false
                }
            };
            for (y, x) in to_add {
                if y < 8 && x < 14 {
                    self.0[y][x].0 = true
                };
            };
            for (y, x) in to_count {
                if y < 8 && x < 14 {
                    if self.0[y][x].1 > 1 {
                        self.0[y][x].1 -= 1
                    } else {
                        self.0[y][x].0 = true
                    }
                };
            };
        }

        pub fn check_coords(&self, (y, x): (usize, usize)) -> bool {
            println!("{}, {}", y, x);
            if y < 8 && x < 14 {
                self.0[y][x].0
            } else {
                false
            }
        }

        pub fn check_field(&self, f: FieldUsize) -> bool {
            self.check_coords(f.to_tuple())
        }

        pub fn count_area(&self, f1: FieldUsize, f2: FieldUsize) -> usize {
            let mut res = 0;
            let (r1, r2) = ord(f1.0, f2.0);
            let rows = self.0[r1..r2+1].to_vec();
            let (s1, s2) = ord(f1.1, f2.1);
            for row in rows {
                for field in s1..s2+1 {
                    if row[field].0 {
                        res += 1
                    }
                }
            };
            res
        }

        pub fn check_coords_num(&self, (y, x): (usize, usize)) -> u8 {
            if y < 8 && x < 14 {
                self.0[y][x].1
            } else {
                0
            }
        }

        pub fn check_field_num(&self, f: FieldUsize) -> u8 {
            self.check_coords_num(f.to_tuple())
        }
    }

    fn ord(a: usize, b: usize) -> (usize, usize) {
        (min(a, b), max(a, b))
    }

        
}



#[cfg(test)]
mod tests {
    use mctrl::motor::FieldUsize;

    use crate::position::{MoveError, MoveType, Position, Piece, BitList};

    use super::*;

    fn get_position() -> Position {
        self::position::Position::from_fen("rnbqkbnr/1p2pppp/8/p1ppP3/P7/8/1PP1PPPP/RNBQKBNR w KQkq d6 0 6").unwrap()
    }

    #[test]
    fn it_works() {
        let result = get_position().to_fen();
        assert_eq!(result, String::from("rnbqkbnr/1p2pppp/8/p1ppP3/P7/8/1PP1PPPP/RNBQKBNR w KQkq d6 0 6"));
    }

    #[test]
    fn it_works1() {
        let result: &Result<Option<Piece>, MoveError> = &get_position().coordinates_to_piece("h1");
        assert_eq!(format!("{:?}", result) , format!("{:?}", Ok::<Option<position::Piece>, MoveError>(Some(Piece::Rook(true)))));
    }

    #[test]
    fn it_works2() {
        let result: &Result<MoveType, MoveError> = &get_position().validate_move_possibility("f1b5");
        assert_eq!(format!("{:?}", result) , format!("{:?}", Ok::<position::MoveType, MoveError>(MoveType::Normal(Piece::Bishop(true)))));
    }

    #[test]
    fn it_works3() {
        let (a, b) = match  position::coordinates_to_index("f7") {
            Ok(b) => b,
            Err(_) => panic!()
        };
        assert_eq!(a, 1 as usize);
        assert_eq!(b, 8 as usize);
    }

    #[test]
    fn it_works4() {
        let rr = match position::coordinates_to_index("n9") {
            Ok(_) => panic!(),
            Err(rr) => rr
        };
        assert_eq!(format!("{:?}", rr), format!("{:?}", MoveError::UnrightCoordinates));
    }

    #[test]
    fn it_works5() {
        let result = &get_position().validate_move_possibility("g4g5");
        assert_eq!(format!("{:?}", result), format!("{:?}", Err::<MoveType, MoveError>(MoveError::NoFigurStart)));
    }

    #[test]
    fn it_works6() {
        let result = &Piece::Bishop(false).check_field("a2", "d5");
        assert_eq!(*result, true);
    }

    #[test]
    fn it_works7() {
        //println!("{:?}", BitList::from_pos(&get_position()));
        BitList::from_pos(&get_position()).print_out();
        let result = BitList::from_pos(&get_position()).check_coords((3, 5));
        assert_eq!(result, true);
    }

    #[test]
    fn it_works8() {
        let result = BitList::from_pos(&get_position()).count_area(FieldUsize(3, 7), FieldUsize(6, 4));
        assert_eq!(result, 6);
    }
}
