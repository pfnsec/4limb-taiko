use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;
use std::fs::File;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Note {
    None = 0,
    Don = 1,
    Ka = 2,
    DonBig = 3,
    KaBig = 4,
    Drumroll = 5,
    DrumrollBig = 6,
    Balloon = 7,
    RollEnd = 8,
}

impl Note {
    fn from_char(c: char) -> Note {
        match c {
            '0' => Note::None,
            '1' => Note::Don,
            '2' => Note::Ka,
            '3' => Note::DonBig,
            '4' => Note::KaBig,
            '5' => Note::Drumroll,
            '6' => Note::DrumrollBig,
            '7' => Note::Balloon,
            '8' => Note::RollEnd,
            _   => Note::None,
        }
    }
}


#[derive(Default)]
#[derive(Debug)]
pub struct Measure {
    pub start_time: f32,
    pub note_len: f32,
    pub notes: Vec<Note>
}

#[derive(Debug)]
pub struct Chart {
    pub bpm: u32,
    pub measures: Vec<Measure>,
}
#[derive(Default)]
#[derive(Debug)]
pub struct ChartPlayback {
    pub measure_i: usize,
    pub note_i: usize,
}

impl Chart {
    pub fn from_file(filename: &str) -> Chart {

        let mut bpm: u32 = 0;
        let mut level: u32 = 0;
        let mut measures = Vec::new();

        // Create a regex that matches on the union of all commands
        // Each command and argument is captured
        // Using the "extended mode" flag to write a nicer Regex
        let input_re = Regex::new(
         r#"(?x)
            (\#START) |
            (\#END) |
            (\d+), | 
            (LEVEL):(\d+) |
            (BPM):(\d+) |
            (goto)\s+(\d+)
            "#
        ).unwrap();

        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);
    
        for line in f.lines() {
            let line = line.unwrap();
            //println!("{}", line);

            let captures = input_re.captures(&line).map(|captures| {
                captures
                    .iter() // All the captured groups
                    .skip(1) // Skipping the complete match
                    .flat_map(|c| c) // Ignoring all empty optional matches
                    .map(|c| c.as_str()) // Grab the original strings
                    .collect::<Vec<_>>() // Create a vector
            });


            // Match against the captured values as a slice
            match captures.as_ref().map(|c| c.as_slice()) {
                Some(["#START"]) => println!("START!"),
                Some(["#END"]) => println!("END!"),
                Some(["BPM", bpm_s]) => {
                    bpm = bpm_s.parse().expect("can't parse BPM!");
                }
                Some(["LEVEL", level_s]) => {
                    level = level_s.parse().expect("can't parse LEVEL!");
                }
                Some([measure_s]) => {
                    let mut measure = Measure::default();
                    for c in measure_s.chars() {
                        measure.notes.push(Note::from_char(c));
                    }
                    measures.push(measure);
                }
                _ => {},
                //_ => panic!("Unknown Command: {}", line),
            }
        }

        println!("LEVEL {}, BPM {}", level, bpm);


        Chart {
            bpm: bpm,
            measures: measures
        }
    }

}


#[test]
fn test_chart_load() {
    let chart = Chart::from_file("charts/Momoiro Taiko Paradise/Momoiro Taiko Paradise.tja");
    println!("{:?}", chart);
}