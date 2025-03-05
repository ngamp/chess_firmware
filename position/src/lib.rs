pub mod position {

    use std::{collections::HashMap, num::ParseIntError};


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

    pub enum DrawR {
        Repetition,
        FiftyMove,
        Stalemate
    }

    pub enum State {
        Normal,
        Mate(bool),
        Draw(DrawR),
        Resign(bool)
    }

    #[derive(Debug)]
    pub enum MoveType {
        Normal,
        Capturing(Piece),
        Rochade(String),
        EnPassant
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
                    if (self::letter_to_int(sl) + self::to_int(sn) as u32)%2 != (self::letter_to_int(el) + self::to_int(en) as u32)%2 {
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
                    match ((self::letter_to_int(sl) as i64 - self::letter_to_int(el) as i64), (self::to_int(sn) as i64 - self::to_int(el) as i64)) {
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
        ImpossiblePosition
    }

    #[derive(Debug)]
    pub enum ModuleError {
        ImpossibleMove(MoveError),
        MoveError(MoveError),
        Other
    }

    pub enum CleaningError {
        KingBeforeQueen,
        ImpossiblePosition
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
                fields: [[Piece::None,Piece::None,Piece::None,Piece::Rook(false),Piece::Knight(false),Piece::Bishop(false),Piece::Queen(false),Piece::King(false),Piece::Bishop(false),Piece::Knight(false),Piece::Rook(false),Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::Pawn(false),Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::Pawn(true),Piece::None,Piece::None,Piece::None],
                    [Piece::None,Piece::None,Piece::None,Piece::Rook(true),Piece::Knight(true),Piece::Bishop(true),Piece::Queen(true),Piece::King(true),Piece::Bishop(true),Piece::Knight(true),Piece::Rook(true),Piece::None,Piece::None,Piece::None]],
                moves: 0,
                en_passant: "-".to_string(),
                rochade: [Piece::King(true),Piece::Queen(true),Piece::King(false),Piece::Queen(false)],
                since_pawn_major: 0
            }
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
            Ok(pos)

        }

        pub fn add_rest(&mut self, pce: Piece) -> Result<(usize, usize), CleaningError> {
            match pce {
                Piece::Queen(true) => {
                    self.fields[7][0] = Piece::Queen(true);
                    Ok((7, 0))
                },
                Piece::Queen(false) => {
                    self.fields[0][13] = Piece::Queen(false);
                    Ok((0, 13))
                },
                Piece::King(true) => {
                    if !self.field_is_empty((7, 0)) {
                        self.fields[7][1] = Piece::King(true);
                        Ok((7, 1))
                    } else {
                        Err(CleaningError::KingBeforeQueen)
                    }
                },
                Piece::King(false) => {
                    if !self.field_is_empty((0, 13)) {
                        self.fields[0][12] = Piece::King(false);
                        Ok((0, 12))
                    } else {
                        Err(CleaningError::KingBeforeQueen)
                    }
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
                    println!("here");
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
                    println!("here");
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

        pub fn update(&mut self, cmove: &str) -> Result<(State, [[Piece; 14]; 8]), ModuleError> {
            let pos_before  = self.fields.clone();
            let movetype = match self.validate_move_possibility(cmove) {
                Ok(mt) => mt,
                Err(rr) => return Err(ModuleError::ImpossibleMove(rr))
            };
            let piece = match self.coordinates_to_piece(cmove.split_at(2).0) {
                Ok(p) => p,
                Err(rr) => return Err(ModuleError::MoveError(rr))
            };
            let (sf, ef) = cmove.split_at(2);
            match movetype {
                MoveType::Normal => {
                    match self.update_wo_capt(piece, sf, ef) {
                        Ok(_) => {},
                        Err(rr) => return Err(ModuleError::MoveError(rr))
                    };
                },
                MoveType::Capturing(capt_piece) => {

                },
                MoveType::EnPassant => {

                },
                MoveType::Rochade(rtype) => {

                }
            };
            Ok((State::Normal, pos_before))
        }

        fn update_wo_capt(&mut self, piece: Piece, sf: &str, ef: &str) -> Result<(), MoveError> {
            self.update_ex_moves(piece, sf, ef);
            let (mut lc, mut nc) = match coordinates_to_index(ef) {
                Ok(t) => t,
                Err(rr) => return Err(rr)
            };
            self.fields[lc][nc] = piece;
            (lc, nc) = match coordinates_to_index(sf) {
                Ok(t) => t,
                Err(rr) => return Err(rr)
            };
            self.fields[lc][nc] = Piece::None;
            Ok(())
        }

        fn update_ex_moves(&mut self, piece: Piece, sf: &str, ef: &str) {
            match piece {
                Piece::Pawn(_) => {
                    self.since_pawn_major = 0;
                    match (sf.split_at(1).1, ef.split_at(1).1) {
                        ("2", "4") => self.en_passant = format!("{}3", sf.split_at(1).0),
                        ("7", "5") => self.en_passant = format!("{}6", sf.split_at(1).0),
                        _ => {}
                    }
                },
                Piece::Rook(b) => {
                    match (sf, b) {
                        ("a1", true) => self.rochade[0] = Piece::None,
                        ("h1", true) => self.rochade[0] = Piece::None,
                        ("a8", false) => self.rochade[0] = Piece::None,
                        ("h8", false) => self.rochade[0] = Piece::None,
                        _ => {}
                    }
                }
                _ => self.since_pawn_major += 1
            };
            self.moves += 1;
            self.colorw = !self.colorw;
        }

        pub fn validate_move_possibility(&self, cmove: &str) -> Result<MoveType, MoveError> {
            match cmove {
                "e1g1" => {
                    match self.rochade[0] {
                        Piece::King(true) => return Ok(MoveType::Rochade("K".to_owned())),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e1c1" => {
                    match self.rochade[1] {
                        Piece::Queen(true) => return Ok(MoveType::Rochade("Q".to_owned())),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e8g8" => {
                    match self.rochade[2] {
                        Piece::King(false) => return Ok(MoveType::Rochade("k".to_owned())),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                "e8c8" => {
                    match self.rochade[3] {
                        Piece::Queen(false) => return Ok(MoveType::Rochade("q".to_owned())),
                        _ => return Err(MoveError::UnallowedRochade)
                    }
                },
                _ => {}
            };
            let (start_field, end_field) = cmove.split_at(2);
            let piece = match self.coordinates_to_piece(start_field) {
                Err(rr) => return Err(rr),
                Ok(piece) => piece
            };
            //let next_match_statement = String::from("-");
            match (self.en_passant.as_str(), piece) {
                ("-",  _) => {},
                (mm, Piece::Pawn(_)) => {
                    println!("{}", mm);
                    if mm == cmove.split_at(2).1 {
                        return Ok(MoveType::EnPassant)
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
            let capt_piece = match self.coordinates_to_piece(end_field) {
                Ok(p) => p,
                Err(rr) => return Err(rr)
            };
            match capt_piece {
                Piece::None => {},
                rp => {
                    if self.colorw && rp.piece_to_color() {
                        return Err(MoveError::OwnFigurEnd)
                    }
                }
            };
            match &piece.check_field(start_field, end_field) {
                true => {},
                false => return Err(MoveError::MoveNotFitPiece(piece))
            };
            match capt_piece {
                Piece::None => Ok(MoveType::Normal),
                _ => Ok(MoveType::Capturing(capt_piece))
            }

            
        }

        pub fn coordinates_to_piece(&self, coordinate: &str) -> Result<Piece, MoveError> {
            let (lett, num) = match coordinates_to_index(coordinate) {
                Ok(a) => (a.0, a.1),
                Err(rr) => return Err(rr)
            };
            Ok((&self.fields[lett][num]).clone())
        }    
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

    pub fn get_fen(position: &Position) -> String {
        let mut res = String::new();

        //get piece position
        for i in 0..8 {
            let mut row = String::new();
            let lis = &position.fields[i];
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
        if position.colorw {
            res.push_str("w ");
        } else {
            res.push_str("b ");
        }

        //rochade-rights
        let mut rochade_res = String::new();
        for piece in &position.rochade {
            rochade_res.push_str(&piece.piece_to_letter());
        }
        if rochade_res == "".to_string() {
            rochade_res.push_str("-");
        }
        res.push_str(&rochade_res);

        //en_passant?
        res.push_str(&format!(" {}", position.en_passant));

        //moves (50-moves-rule)
        res.push_str(&format!(" {}", position.since_pawn_major / 2));

        //movenumber
        res.push_str(&format!(" {}", (position.moves / 2) + 1));


        res
    }
}

#[cfg(test)]
mod tests {
    use crate::position::{MoveError, MoveType, Position, Piece};

    use super::*;

    fn get_position() -> Position {
        self::position::Position::from_fen("rnbqkbnr/1p2pppp/8/p1ppP3/P7/8/1PP1PPPP/RNBQKBNR w KQkq d6 0 6").unwrap()
    }

    #[test]
    fn it_works() {
        let result = position::get_fen(&get_position());
        assert_eq!(result, String::from("rnbqkbnr/1p2pppp/8/p1ppP3/P7/8/1PP1PPPP/RNBQKBNR w KQkq d6 0 6"));
    }

    #[test]
    fn it_works1() {
        let result: &Result<Piece, MoveError> = &get_position().coordinates_to_piece("a1");
        assert_eq!(format!("{:?}", result) , format!("{:?}", Ok::<position::Piece, MoveError>(Piece::Rook(true))));
    }

    #[test]
    fn it_works2() {
        let result: &Result<MoveType, MoveError> = &get_position().validate_move_possibility("f1b5");
        assert_eq!(format!("{:?}", result) , format!("{:?}", Ok::<position::MoveType, MoveError>(MoveType::Normal)));
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
}
