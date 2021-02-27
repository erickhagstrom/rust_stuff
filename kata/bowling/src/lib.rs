//////////////////
/// Public API ///
//////////////////

pub fn get_score(rolls: &[u32]) -> u32 {
  calc_score(rolls).get_score()
}

pub fn calc_score(rolls: &[u32]) -> GameScore {
  rolls.iter().fold(
    GameScore::new_game(), score_folder)
}

pub struct GameScore {
  current_frame: Frame,
  resolved_frames: Vec<Frame>,
}

impl GameScore {
  pub fn get_score(&self) -> u32 {
    let mut x = 0;
    for i in 0..self.resolved_frames.len() {
      x += match self.resolved_frames[i] {
        Frame::Closed(r1, r2) => r1 + r2,
        Frame::Spare(r1, r2) => {
          if self.resolved_frames.len() > i+1 {
            match self.resolved_frames[i+1] {
              Frame::Closed(r3, _) => r1 + r2 + r3,
              Frame::Spare(r3, _) => r1 + r2 + r3,
              Frame::Strike => 20,
              Frame::LastFrame1(r3) => r1 + r2 + r3,
              Frame::LastFrame2(r3, _) => r1 + r2 + r3,
              Frame::LastFrame3(r3, _, _) => r1 + r2 + r3,
              _ => 0,
            }
          } else {
            0
          }
        },
        Frame::Strike => {
          let r1 = 10;
          if self.resolved_frames.len() > i+1 {
            match self.resolved_frames[i+1] {
              Frame::Closed(r2, r3) => r1 + r2 + r3,
              Frame::Strike => {
                let r2 = 10;
                if self.resolved_frames.len() > i+2 {
                  match self.resolved_frames[i+2] {
                    Frame::Closed(r3, _) => r1 + r2 + r3,
                    Frame::Strike => r1 + r2 + 10,
                    Frame::LastFrame2(r3, _) => r1 + r2 + r3,
                    Frame::LastFrame3(r3, _, _) => r1 + r2 + r3,
                    _ => 0,
                  }
                } else { 0 }
              },
              Frame::LastFrame2(r2, _) => {
                if r2 < 10 {
                  r1 + r2
                } else {
                  0
                }
              },
              Frame::LastFrame3(r2, r3, _) => r1 + r2 + r3,
              _ => 0,
            }
          } else { 0 }
        },
        Frame::LastFrame2(r1, r2) => {
          if r1 + r2 < 10 {
            r1 + r2
          } else { 0 }
        },
        Frame::LastFrame3(r1, r2, r3) => r1 + r2 + r3,
        _ => 0,
      }
    }
    x
  }
}

#[derive(Debug, PartialEq)]
pub enum Frame {
  FirstRoll,
  Open(u32),
  Closed(u32, u32),
  Spare(u32, u32),
  Strike,
  LastFrame1(u32),
  LastFrame2(u32, u32),
  LastFrame3(u32, u32, u32),
  GameOver
}

///////////////
/// Private ///
///////////////

fn score_folder(mut gs: GameScore, b: &u32) -> GameScore {
  let framenum = gs.framenum();
  let new_frame: Frame;
  
  match gs.current_frame {
    Frame::FirstRoll => {
      if framenum == 10 {
        new_frame = Frame::LastFrame1(*b);
      } else {
        if *b == 10 {
          new_frame = gs.start_new_frame(Frame::Strike);
        } else {
          new_frame = Frame::Open(*b);
        };
      }
    },
    Frame::Open(r1) => {
      if r1 + *b >= 10 {
        new_frame = gs.start_new_frame(Frame::Spare(r1, *b));
      } else {
        new_frame = gs.start_new_frame(Frame::Closed(r1, *b));
      };
    },
    Frame::LastFrame1(r1) => {
      if r1 != 10 && r1 + *b != 10 {
        new_frame = gs.start_new_frame(Frame::LastFrame2(r1, *b));
      } else {
        new_frame = Frame::LastFrame2(r1, *b);
      }
    },
    Frame::LastFrame2(r1, r2) => {
      new_frame = gs.start_new_frame(Frame::LastFrame3(r1, r2, *b));
    },
    _ => {
      println!("frame_num: {:?}", gs.framenum());
      println!("resolved_frames: {:?}", gs.resolved_frames);
      panic!("Invalid current_frame: {:?}", gs.current_frame);
    },
  };

  GameScore {
    current_frame: new_frame,
    resolved_frames: gs.resolved_frames,
  }
}

impl GameScore {
  fn new_game() -> GameScore {
    GameScore {
      current_frame: Frame::FirstRoll,
      resolved_frames: Vec::new(),
    }
  }

  fn framenum(&self) -> usize {
    self.resolved_frames.len() + 1
  }

  fn start_new_frame(&mut self, frame: Frame) -> Frame{
    self.resolved_frames.push(frame);
    if self.framenum() > 10 {
      Frame::GameOver
    } else {
      Frame::FirstRoll
    }
  }
}

/////////////
/// Tests ///
/////////////

#[cfg(test)]
mod tests {
  use super::*;

  /////////////////////////////////////////
  // Tests for the calc_score() function //
  /////////////////////////////////////////

  #[test]
  fn all_zero() {
      let rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      let result = calc_score(&rolls);
      assert_eq!(0, result.get_score());
  }

  #[test]
  fn one() {
      let rolls = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      let result = calc_score(&rolls);
      assert_eq!(1, result.get_score());
  }

  #[test]
  fn ninety() {
    let rolls = [0, 9, 1, 8, 2, 7, 3, 6, 4, 5, 5, 4, 6, 3, 7, 2, 8, 1, 9, 0];
    let result = calc_score(&rolls);
    assert_eq!(90, result.get_score());
  }

  #[test]
  fn first_frame_spare() {
      // The score by frame is: 11, 12, 12, 12, 12, 12, 12, 12, 12, 12
      let rolls = [9, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
      let result = calc_score(&rolls);
      assert_eq!(12, result.get_score());
  }

  #[test]
  fn all_spares() {
    let rolls = [9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9];
    let result = calc_score(&rolls);
    assert_eq!(190, result.get_score());
  }

  #[test]
  fn perfect() {
    let rolls = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
    let result = calc_score(&rolls);
    println!("resolved_frames: {:?}", result.resolved_frames);
    assert_eq!(300, result.get_score());
  }

  #[test]
  fn lost_it() {
    let rolls = [10, 10, 10, 10, 10, 10, 10, 10, 10, 9, 0];
    let result = calc_score(&rolls);
    println!("resolved_frames: {:?}", result.resolved_frames);
    assert_eq!(10, result.resolved_frames.len(),"resolved_frames is short");
    assert_eq!(7*30 + 29 + 19 + 9, result.get_score());
  }

  ///////////////////////////
  /// Tests for internals ///
  ///////////////////////////
  
  #[test]
  fn first_ball_gutter() {
    let a = GameScore::new_game();
    let result = score_folder(a, &0);
    assert_eq!(0, result.get_score());
    assert_eq!(0, result.resolved_frames.len(),"resolved_frames is wrong length");
  }

  #[test]
  fn second_of_two_gutters() {
    let a = GameScore::new(Frame::Open(0), Vec::new());
    let result = score_folder(a, &0);
    assert_eq!(0, result.get_score(), "game_score is wrong");
    assert_eq!(1, result.resolved_frames.len(), "resolved_frames is wrong");
  }

  #[test]
  fn first_ball_one() {
    let a = GameScore::new_game();
    let result = score_folder(a, &1);
    assert_eq!(1,
      if let Frame::Open(r1) = result.current_frame
      { r1 } else { 0 }
    );
  }

  #[test]
  fn missed_split() {
    let a = GameScore::new_game();
    let a = score_folder(a, &8);
    let result = score_folder(a, &1);
    assert_eq!(9, result.get_score());
  }

  #[test]
  fn opening_spare() {
    let a = GameScore::new_game();
    let a = score_folder(a, &8);
    let a = score_folder(a, &2);
    assert!(match a.resolved_frames[0] { Frame::Spare(_r1, _r2) => true, _ => false });
  }

  #[test]
  fn adding_up_strikes() {
    let a = GameScore {
      current_frame: Frame::GameOver,
      resolved_frames : vec![
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::Strike,
        Frame::LastFrame3(10, 10, 10),
      ],
    };
    assert_eq!(300, a.get_score());
  }

  ////////////////////////
  /// Helper functions ///
  ////////////////////////

  impl GameScore {
    fn new(current_frame: Frame, resolved_frames: Vec<Frame>) -> GameScore {
      GameScore {
        current_frame,
        resolved_frames,
      }
    }
  }
}
