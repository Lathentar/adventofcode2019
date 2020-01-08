extern crate num;
extern crate nalgebra as na;

use permutohedron::LexicalPermutation;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use na::Vector2;
use crate::num::Integer;

pub struct Config {
    pub aoc_day: u32,
    pub input_filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let aoc_day = args[1].clone().parse().unwrap();
        let input_filename = args[2].clone();
    
        Ok(Config { aoc_day, input_filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input_filename)?;

    match config.aoc_day {
        1 => aoc_dayone(&contents, false),
        2 => aoc_dayone(&contents, true),
        3 => aoc_daytwo(&contents),
        4 => aoc_daytwo_part2(&contents),
        5 => aoc_daythree(&contents),
        6 => aoc_daythree(&contents),
        7 => aoc_dayfour(&contents, false),
        8 => aoc_dayfour(&contents, true),
        9 => aoc_dayfive(&contents, 1),
        10 => aoc_dayfive(&contents, 5),
        11 => aoc_daysix(&contents),
        12 => aoc_daysix_parttwo(&contents),
        13 => aoc_dayseven(&contents),
        14 => aoc_dayseven_parttwo(&contents),
        15 => aoc_dayeight(&contents),
        16 => aoc_dayeight_parttwo(&contents),
        17 => aoc_daynine(&contents,1),
        18 => aoc_daynine(&contents,2),
        19 => aoc_dayten(&contents),
        20 => aoc_dayten_parttwo(&contents),
        _ => ()
    };

    Ok(())
}

fn aoc_dayone_fuel_req(mass : i32, include_fuel_mass : bool) -> i32 {
    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    let mut fuel_req = std::cmp::max( ( mass / 3 ) - 2, 0 );

    if fuel_req > 0 && include_fuel_mass == true {
        fuel_req += aoc_dayone_fuel_req(fuel_req, true);
    }
    
    fuel_req
}

fn aoc_dayone(input: &str, include_fuel_mass : bool) {
    let mut fuel_req = 0;
    for line in input.lines() {
        let mass : i32 = line.parse().unwrap();
        let fuel_for_mass = aoc_dayone_fuel_req(mass, include_fuel_mass);
        fuel_req += fuel_for_mass;
        println!("Mass: {}, Fuel for Mass: {}, Fuel Req: {}", mass, fuel_for_mass, fuel_req);
    }
}

fn compute_intopcodes_from_string(input: &str) -> Vec<i64> {
    let mut intopcodes = Vec::new();
    let v: Vec<&str> = input.split_terminator(',').collect();
    for intopcode_str in v.iter() {
        let intopcode : i64 = intopcode_str.parse().unwrap();
        intopcodes.push(intopcode);
    }
    intopcodes
}

#[derive(Debug)]
enum ParameterMode {
    Position, // parameter is interpreted as a position
    ImmediateMode, // parameter is interpreted as a value
    RelativeMode(i64), // parameter is intrepreted as a position + relative_base (i64)
}

impl ParameterMode {
    fn from_index(idx: i64, relative_base: i64) -> ParameterMode {
        match idx {
            0 => ParameterMode::Position,
            1 => ParameterMode::ImmediateMode,
            2 => ParameterMode::RelativeMode(relative_base),
            _ => panic!("Unknown Parameter Mode {}", idx)
        }
    }

    fn get_index_for_write(&self, _intopcodes: &Vec<i64>, param: i64) -> usize {
        match self {
            ParameterMode::RelativeMode(relative_base) => {
                relative_base + param
            },
            _ => {  // default behavior is to use Immediate
                param
            },
        }.try_into().unwrap()
    }

    fn get_value(&self, intopcodes: &Vec<i64>, param: i64) -> i64 {
        match self {
            ParameterMode::Position => {
                let index : usize = param.try_into().unwrap();
                if index < intopcodes.len() {
                    intopcodes[index]
                }
                else {
                    0
                }
            },
            ParameterMode::ImmediateMode => {
                param
            },
            ParameterMode::RelativeMode(relative_base) => {
                let index : usize = (relative_base + param).try_into().unwrap();
                if index < intopcodes.len() {
                    intopcodes[index]
                }
                else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
struct IntOpCodeComp {    
    intopcodes: Vec<i64>,
    input_values: VecDeque<i64>,
    output_values: VecDeque<i64>,
    index: usize,
    relative_base: i64,
    complete: bool
}

impl IntOpCodeComp {
    fn new(intopcodes: &Vec<i64>, input_values: &VecDeque<i64>) -> IntOpCodeComp {
        IntOpCodeComp{
            intopcodes: intopcodes.clone(),
            input_values: input_values.clone(),
            output_values: VecDeque::new(),
            index: 0,
            relative_base: 0,
            complete: intopcodes.len() == 0,
        }
    }

    fn store_at_index(&mut self, value: i64, index: usize) {
        if index >= self.intopcodes.len() {
            self.intopcodes.resize(index + 1, 0);   // Expand memory to necessary size filled with 0s
        }
        self.intopcodes[index] = value;
    }

    fn tick_to_completion(&mut self) -> &VecDeque<i64> {
        while self.complete == false {
            self.tick();
        }

        &self.output_values
    }

    fn tick(&mut self) {
        if self.complete == true {
            return;
        }

        let opcode = self.intopcodes[self.index];
        let two_digit_opcode = opcode % 100;
        let mode_1st_param = ParameterMode::from_index((opcode / 100) % 10, self.relative_base);
        let mode_2nd_param = ParameterMode::from_index((opcode / 1000) % 10, self.relative_base);
        let mode_3rd_param = ParameterMode::from_index((opcode / 10000) % 10, self.relative_base);
        
        match two_digit_opcode {
            1 => {
                // Adds the next two numbers together, store in index of third number                
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);                
                let store_index = mode_3rd_param.get_index_for_write(&self.intopcodes, self.intopcodes[self.index + 3]);
                //println!("Idx {} Op1 ({:?}, {:?}): {} + {} => {} -> StoreAtIdx {}", index, mode_1st_param, mode_2nd_param, a, b, (a+b), store_index);
                self.store_at_index(a + b, store_index);
                self.index += 4;
            },
            2 => {
                // Multiplies the next two numbers together, store in index of third number
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);                
                let store_index = mode_3rd_param.get_index_for_write(&self.intopcodes, self.intopcodes[self.index + 3]);
                //println!("Idx {} Op2 ({:?}, {:?}): {} * {} => {} -> StoreAtIdx {}", index, mode_1st_param, mode_2nd_param, a, b, (a*b), store_index);
                self.store_at_index(a * b, store_index);
                self.index += 4;
            },
            3 => {
                // Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. 
                // For example, the instruction 3,50 would take an input value and store it at address 50.
                let store_index = mode_1st_param.get_index_for_write(&self.intopcodes, self.intopcodes[self.index + 1]);
                //println!("Op3: {} = {:?}", store_index, self.input_values.front() );
                match self.input_values.pop_front() {
                    Some(val) => {
                        self.store_at_index(val, store_index);
                        self.index += 2;
                    },
                    None => ()  // just wait until we get an input at some point
                };
            },
            4 => {
                // Opcode 4 outputs the value of its only parameter.
                // For example, the instruction 4,50 would output the value at address 50.
                let output_index = self.intopcodes[self.index + 1];
                self.output_values.push_back(mode_1st_param.get_value(&self.intopcodes, output_index).try_into().unwrap());
                //println!("Op4: {} = {}", output_index, self.output_values.back().unwrap());
                self.index += 2;
            },
            5 => {
                // jump-if-true: if the first parameter is non-zero, 
                // it sets the instruction pointer to the value from the second parameter. 
                // Otherwise, it does nothing.
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                if a != 0 {
                    let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);
                    self.index = b.try_into().unwrap();
                }
                else {
                    self.index += 3;
                }
            },
            6 => {
                // jump-if-false: if the first parameter is zero,
                // it sets the instruction pointer to the value from the second parameter. 
                // Otherwise, it does nothing.
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                if a == 0 {
                    let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);
                    self.index = b.try_into().unwrap();
                }
                else {
                    self.index += 3;
                }
            },
            7 => {
                // less than: if the first parameter is less than the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);
                let store_index = mode_3rd_param.get_index_for_write(&self.intopcodes, self.intopcodes[self.index + 3]);
                self.store_at_index(match a < b { true => 1, false => 0}, store_index);
                self.index += 4;
            },
            8 => {
                // equals: if the first parameter is equal to the second parameter,
                // it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                let b = mode_2nd_param.get_value(&self.intopcodes, self.intopcodes[self.index + 2]);
                let store_index = mode_3rd_param.get_index_for_write(&self.intopcodes, self.intopcodes[self.index + 3]);
                self.store_at_index(match a == b { true => 1, false => 0 }, store_index);
                self.index += 4;
            },
            9 => {
                // adjusts the relative base by the value of its only parameter
                let a = mode_1st_param.get_value(&self.intopcodes, self.intopcodes[self.index + 1]);
                self.relative_base += a;
                self.index += 2;
            },
            99 => {
                self.index = self.intopcodes.len();
                self.complete = true;
            }
            _ => panic!("Unexpected op code {}", two_digit_opcode)
        }
    }
}

#[derive(Debug)]
struct Amplifiers {
    computers: Vec<IntOpCodeComp>,
    feedback: bool
}

impl Amplifiers {
    fn new(intopcode_base: &Vec<i64>, phases: &Vec<i64>, feedback: bool) -> Amplifiers {
        // Create a new computer for each phase
        let mut amps = Amplifiers {
            computers: Vec::new(),
            feedback
        };
        for p in phases {
            let mut input_values = VecDeque::new();
            input_values.push_back( *p );
            amps.computers.push( IntOpCodeComp::new(&intopcode_base, &input_values) );
        }
        amps
    }

    fn tick(&mut self) {
        for i in 0..self.computers.len() {
            // feed in any output from previous computers as our input
            if i > 0 || self.feedback == true {
                let prev_index = if i == 0 { self.computers.len() - 1 } else { i - 1 };
                match self.computers[prev_index].output_values.pop_front() {
                    Some(val) => self.computers[i].input_values.push_back(val),
                    None => ()
                };
            }
            self.computers[i].tick();         
        }
    }

    fn tick_to_completion(&mut self) -> i64 {
        let mut last_valid_output = 0;
        while self.computers.last().unwrap().complete == false {
            self.tick();
            match self.computers.last().unwrap().output_values.front() {
                Some(val) => last_valid_output = *val,
                None => ()
            };  
        }

        last_valid_output
    }
}

fn process_amp_intopcode_with_phases(intopcodes: &Vec<i64>, phases: &Vec<i64>, feedback: bool) -> i64 {
    let mut amps = Amplifiers::new(intopcodes, phases, feedback);
    amps.computers[0].input_values.push_back(0);
    amps.tick_to_completion()
}

fn compute_max_amp_intopcode(intopcodes: &Vec<i64>, phases: &[i64;5], feedback: bool) -> i64 {
    let mut max_output = 0;
    let mut phase_perm = phases.clone();
    loop {
        let output = process_amp_intopcode_with_phases(&intopcodes, &phase_perm.to_vec(), feedback);
        max_output = std::cmp::max(max_output, output);
        if !phase_perm.next_permutation() {
            break;
        }
    }
    max_output
}

fn get_intopcode_output(intopcodes_base: &Vec<i64>, input_value: i64) -> VecDeque<i64> {
    let mut input_values = VecDeque::new();
    input_values.push_back(input_value);
    let mut comp = IntOpCodeComp::new(intopcodes_base, &input_values);
    comp.tick_to_completion().clone()
}

fn process_simple_intopcode(intopcodes_base: &Vec<i64>) -> Vec<i64> {    
    let input_values = VecDeque::new();
    let mut comp = IntOpCodeComp::new(intopcodes_base, &input_values);
    comp.tick_to_completion();
    comp.intopcodes.clone()
}

fn aoc_daytwo(input: &str) {
    let mut intopcodes = compute_intopcodes_from_string(input);

    // before running the program, replace position 1 with the value 12
    // and replace position 2 with the value 2.
    // What value is left at position 0 after the program halts?
    intopcodes[1] = 12;
    intopcodes[2] = 2;
    
    let processed_intopcodes = process_simple_intopcode(&intopcodes);

    println!("{:?}", processed_intopcodes);
}

fn aoc_daytwo_part2(input: &str) {
    // find values for index 1 and 2 (between 0 and 99) to produce result 19690720
    let mut intopcodes = compute_intopcodes_from_string(input);

    let mut index1 = 0;
    let mut index2 = 0;
    
    while index1 < 100 {
        while index2 < 100 {
            intopcodes[1] = index1;
            intopcodes[2] = index2;

            let processed_intopcodes = process_simple_intopcode(&intopcodes);

            if processed_intopcodes[0] == 19690720 {
                println!("{}", index1 * 100 + index2);
                println!("{:?}", processed_intopcodes);
                return
            }
            index2 += 1;
        }
        index1 += 1;
        index2 = 0;
    }

    println!("No values found.");
}

struct WireCmd {
    dir : char,
    dist : u32
}

fn compute_wire_cmds(input: &str) -> Vec<WireCmd> {
    let mut wirecmds = Vec::new();
    let v: Vec<&str> = input.trim().split_terminator(',').collect();
    for cmd in v.iter() {
        let dir = cmd.chars().next().unwrap();
        let dist : u32 = cmd[1..].parse().unwrap();
        wirecmds.push( WireCmd{ dir, dist } );
    }

    wirecmds
}

fn compute_position_hash(wirecmds : &Vec<WireCmd>) -> HashMap<(i32, i32), i32> {
    let mut position_hash = HashMap::new();
    let mut pos = (0, 0);
    let mut dist = 0;
    for cmd in wirecmds.iter() {
        //println!("cmd {} {}", cmd.dir, cmd.dist);
        for _i in 0..cmd.dist {
            match cmd.dir {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => panic!("Unknown direction")
            }
            dist += 1;
            //println!("\tpos {:?}", pos);
            position_hash.entry(pos).or_insert(dist);
        }
    }

    //println!("Hash Pos: {:?}", position_hash);
    position_hash
}

fn compute_min_dist(input: &str) -> (i32, i32) {
    let mut lines = input.lines();
    let wirecmds1 = compute_wire_cmds( lines.next().unwrap() );
    let wirecmds2 = compute_wire_cmds( lines.next().unwrap() );

    let hashpos1 = compute_position_hash( &wirecmds1 );
    let hashpos2 = compute_position_hash( &wirecmds2 );

    let mut min_dist = 999999;
    let mut min_wiredist = 99999;
    for (key, wiredist1) in &hashpos1 {
        match hashpos2.get(key) {
            Some(wiredist2) => {
                let int_dist = key.0.abs() + key.1.abs();
                //println!("Intersection: {:?}, Dist: {}", intersection, int_dist);
                min_dist = std::cmp::min( min_dist, int_dist );
                
                let wired_dist = wiredist1 + wiredist2;
                min_wiredist = std::cmp::min( min_wiredist, wired_dist );
            },
            None => ()
        }
    }
    //println!("=================================");
    (min_dist, min_wiredist)
}

fn aoc_daythree(input: &str) {
    let dist = compute_min_dist(&input);
    println!("Min Dist {}, Min Wire Dist {}", dist.0, dist.1);
}

fn vet_password(pass: u32, must_find_pair : bool) -> bool {
    
    // It is a six-digit number.
    if pass > 999999 {
        return false
    }

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    let mut found_pair = false;
    let mut found_dupe = false;
    let mut dupe_length = 1;
    let mut last_digit = pass % 10;
    let mut pass_check = pass / 10;
    //println!("Checking {}", pass);
    while pass_check > 0 {
        let current_digit = pass_check % 10;
        found_dupe |= current_digit == last_digit;
        if current_digit > last_digit {
            //println!("Current digit failed")
            return false;
        } else if current_digit == last_digit {
            dupe_length += 1;
        } else {
            if dupe_length == 2 {
                found_pair = true;
            }

            dupe_length = 1;
        }

        last_digit = current_digit;
        pass_check /= 10;
    }

    if dupe_length == 2 {
        found_pair = true;
    }

    if found_dupe == false {
        return false;
    }

    if must_find_pair == true && found_pair == false {
        return false;
    }

    true
}

fn aoc_dayfour_range(begin: u32, end: u32, must_find_pair : bool) -> u32 {
    let mut valid_passwords = 0;

    // The value is within the range given in your puzzle input.
    for pass in begin..end {
        if vet_password(pass, must_find_pair) == true {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn aoc_dayfour(input: &str, must_find_pair : bool) {
    let v: Vec<&str> = input.split_terminator('-').collect();
    let begin : u32 = v[0].parse().unwrap();
    let end : u32 = v[1].parse().unwrap();
    
    let count = aoc_dayfour_range(begin, end, must_find_pair);
    println!("Valid passwords between {}-{} = {}", begin, end, count);
}

fn aoc_dayfive(input: &str, input_value: i64) {
    let intopcodes = compute_intopcodes_from_string(input);
    let output = get_intopcode_output(&intopcodes, input_value);

    println!("Output={:?}", output);
}

fn parse_orbits_and_planets(input: &str) -> (Vec<(&str, &str)>, HashSet<&str>) {
    let mut orbits = Vec::new();
    let mut planets = HashSet::new();

    let lines = input.lines();
    for line in lines {
        let v: Vec<&str> = line.trim().split_terminator(')').collect();
        planets.insert( v[0] );
        planets.insert( v[1] );
        orbits.push( ( v[0], v[1] ) );
    }

    (orbits, planets)
}

fn compute_orbits(orbits: &Vec<(&str, &str)>, planet: &str) -> u32 {
    for o in orbits {
        if o.1 == planet {
            return 1 + compute_orbits(orbits, o.0)       
        }
    }

    0
}

fn sum_all_orbits(orbits: &Vec<(&str, &str)>, planets: &HashSet<&str>) -> u32 {
    let mut total_orbits = 0;
    for p in planets {
        let num_orbits = compute_orbits(&orbits, p);
        total_orbits += num_orbits;
    }
    total_orbits
}

fn aoc_daysix(input: &str) {
    let (orbits, planets) = parse_orbits_and_planets(input);
    let total_orbits = sum_all_orbits(&orbits, &planets);

    println!("Total Orbits {}", total_orbits);
}

fn build_planet_hierarchy(orbits: &Vec<(&str, &str)>, planet: &str, hierarchy: &mut HashSet<String>) {
    for o in orbits {
        if o.1 == planet {
            hierarchy.insert(String::from(o.0));
            build_planet_hierarchy(&orbits, o.0, hierarchy);
            break;
        }
    }
}

fn orbital_distance(input: &str, planet1: &str, planet2: &str) -> u32 {
    let (orbits, _planets) = parse_orbits_and_planets(input);
    let mut p1_hierarchy = HashSet::new();
    let mut p2_hierarchy = HashSet::new();
    build_planet_hierarchy(&orbits, planet1, &mut p1_hierarchy );
    build_planet_hierarchy(&orbits, planet2, &mut p2_hierarchy );

    //println!("P1: {}, Hier: {:?}", planet1, p1_hierarchy);
    //println!("P2: {}, Hier: {:?}", planet2, p2_hierarchy);
    let difference = p1_hierarchy.symmetric_difference( &p2_hierarchy );
    //println!("SYMDIFF {:?}", difference);
    let distance : u32 = difference.count().try_into().unwrap();
    distance
}

fn aoc_daysix_parttwo(input: &str) {
    let distance = orbital_distance(input, "YOU", "SAN");
    println!("Distance: {}", distance);
}

fn aoc_dayseven(input: &str) {
    let intopcode = compute_intopcodes_from_string(input);
    let phases = [0,1,2,3,4];
    let output = compute_max_amp_intopcode(&intopcode, &phases, false);
    println!("MaxOutput: {}", output);
}

fn aoc_dayseven_parttwo(input: &str) {
    let intopcode = compute_intopcodes_from_string(input);
    let phases = [5,6,7,8,9];
    let output = compute_max_amp_intopcode(&intopcode, &phases, true);
    println!("MaxOutput: {}", output);
}

struct ElfImage {
    height: u32,
    width: u32,
    transmitted_layers: Vec<Vec<u32>>
}

impl ElfImage {
    fn from_input(height: u32, width: u32, input: &str) -> ElfImage {
        let mut layers : Vec<Vec<u32>> = Vec::new();
        let mut parsed_input = input;
        while parsed_input.len() > 0 {
            let (transmission, remaining_input) = parsed_input.split_at( (height * width).try_into().unwrap() );
            parsed_input = remaining_input;
            
            let mut layer = Vec::new();
            for t in transmission.chars() {
                let i : u32 = t.to_digit(10).unwrap();
                layer.push(i);
            }
            layers.push(layer);
        }
        ElfImage{height, width, transmitted_layers: layers}
    }

    fn layer_with_min_character(&self, c: u32) -> &Vec<u32> {
        let mut min_layer = self.transmitted_layers.last().unwrap();
        let mut min_count : usize = ( self.height * self.width + 1 ).try_into().unwrap();
        for layer in self.transmitted_layers.iter() {
            let count = layer.iter().filter(|&n| *n == c).count();
            if count < min_count {
                min_count = count;
                min_layer = layer;
            }
        }
        min_layer
    }

    fn decode_image(&self) -> Vec<u32> {
        let mut image = Vec::new();

        for layer in self.transmitted_layers.iter() {
            if image.is_empty() {
                image = layer.clone();
            }
            else {
                for i in 0..layer.len() {
                    match image[i] {
                        0 => (),    // black
                        1 => (),    // white
                        2 => {      // transparent, adopt color
                            image[i] = layer[i];
                        },
                        _ => panic!("Unknown")
                    };
                }
            }
        }

        image
    }
}

fn aoc_dayeight(input: &str)
{
    let img = ElfImage::from_input(6, 25, input);
    let layer_str = img.layer_with_min_character(0);
    let one_count = layer_str.iter().filter(|&n| *n == 1).count();
    let two_count = layer_str.iter().filter(|&n| *n == 2).count();
    println!("Min 0 layer {:?}, 1 count * 2 count = {}", layer_str, one_count * two_count);
}

fn aoc_dayeight_parttwo(input: &str)
{
    let img = ElfImage::from_input(6, 25, input);
    let decoded = img.decode_image();
    for h in 0..img.height {
        for w in 0..img.width {
            let index : usize = (w + h * img.width).try_into().unwrap();
            print!( "{}", if decoded[index] == 1 { '#' } else { ' ' } );
        }
        println!("");
    }
}

fn aoc_daynine(input: &str, input_value: i64) {
    let intopcode = compute_intopcodes_from_string(input);
    let output = get_intopcode_output(&intopcode, input_value);
    println!("Output {:?}", output);
}

#[derive(Debug)]
struct Asteroid {
    pos: na::Vector2<i32>,
}

fn parse_asteroid_field(input: &str) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    let mut pos = Vector2::new( 0, 0 );
    for l in input.lines() {
        for c in l.trim().chars() {
            match c {
                '#' => asteroids.push( Asteroid{ pos } ),
                _ => ()
            };
            pos.x += 1;
        }
        pos.x = 0;
        pos.y += 1;
    }
    asteroids
}

fn compute_asteroid_los_hashmap<'a>(src: &'a Asteroid, field: &'a Vec<Asteroid>) -> HashMap<Vector2<i32>, Vec<&'a Asteroid>> {
    let mut hashmap : HashMap<Vector2<i32>, Vec<&'a Asteroid>> = HashMap::new();
    for dest in field {
        if src.pos == dest.pos {
            continue;
        }

        // Generate a vector from src to dest
        let v = dest.pos - src.pos;
        let gcd = v.x.gcd( &v.y );
        let nv = Vector2::new( v.x / gcd, v.y / gcd );
        let dist = v.x.abs() + v.y.abs();

        // Add to hash map, using nv as the key
        let asteroid_vec = hashmap.entry(nv).or_default();

        // insert the asteroid into the vector sorted farthest->closest
        if asteroid_vec.is_empty() {
            asteroid_vec.push(dest);
        }
        else {
            let mut insert_index = asteroid_vec.len();
            for i in 0..asteroid_vec.len() {
                let iv = asteroid_vec[i].pos - src.pos;
                let idist = iv.x.abs() + iv.y.abs();
                if idist < dist {
                    insert_index = i;
                    break;
                }
            }
            asteroid_vec.insert(insert_index, dest);
        }
    }

    hashmap
}

fn compute_all_asteroid_los_count(field: &Vec<Asteroid>) -> Vec<u32> {
    let mut counts = Vec::new();
    for a in field {
        counts.push( compute_asteroid_los_hashmap(&a, &field).keys().count().try_into().unwrap() );
    }
    counts
}

fn compute_max_asteroid_los_count(field: &Vec<Asteroid>) -> (&Asteroid, u32) {
    let counts = compute_all_asteroid_los_count(&field);
    let mut max_i = 0;
    for i in 1..counts.len() {
        if counts[max_i] < counts[i] {
            max_i = i;
        }
    }
    ( &field[max_i], counts[max_i] )
}

fn compute_asteroid_destruction_order<'a>(src: &'a Asteroid, field: &'a Vec<Asteroid>) -> Vec<&'a Asteroid> {
    let mut asteroid_do = Vec::new();
    let mut los_hashmap = compute_asteroid_los_hashmap(src, field);

    let mut sorted_keys : Vec<Vector2<i32>> = los_hashmap.keys().copied().collect();
    sorted_keys.sort_by(|&a, &b| { 
        let ay : f64 = (-a.y).try_into().unwrap();
        let ax : f64 = a.x.try_into().unwrap();
        let mut atan = ax.atan2( ay );
        if atan < 0.0 { atan += 2.0 * std::f64::consts::PI; }
        let by : f64 = (-b.y).try_into().unwrap();
        let bx : f64 = b.x.try_into().unwrap();
        let mut btan = bx.atan2( by );
        if btan < 0.0 { btan += 2.0 * std::f64::consts::PI; }

        atan.partial_cmp(&btan).unwrap() } );

    loop {
        let mut found_asteroid = false;

        for k in sorted_keys.iter() {
            match los_hashmap.get_mut(k) {
                Some(asteroids) => {
                    if !asteroids.is_empty() {
                        asteroid_do.push( asteroids.pop().unwrap() );
                        found_asteroid = true;
                    }
                },
                None => ()
            }
        }

        if found_asteroid == false {
            break;
        }
    }

    asteroid_do
}

fn aoc_dayten(input: &str) {
    let asteroids = parse_asteroid_field(&input);
    let (a, max) = compute_max_asteroid_los_count(&asteroids);
    println!("Max Count {} from {:?}", max, a.pos);
}

fn aoc_dayten_parttwo(input: &str) {
    let asteroids = parse_asteroid_field(&input);
    let (a, _max) = compute_max_asteroid_los_count(&asteroids);
    let destruction_order = compute_asteroid_destruction_order(&a, &asteroids);
    println!("200th {:?} => {}", destruction_order[199], destruction_order[199].pos.x * 100 + destruction_order[199].pos.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_req_test() {
        let tests = [(12, 2, 2), (14, 2, 2), (1969, 654, 966), (100756, 33583, 50346)];

        for test in tests.iter() {
            assert_eq!( aoc_dayone_fuel_req(test.0, false), test.1 );
            assert_eq!( aoc_dayone_fuel_req(test.0, true), test.2 );
        }
    }

    #[test]
    fn simple_intopcode_test() {
        assert_eq!( process_simple_intopcode(&vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!( process_simple_intopcode(&vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!( process_simple_intopcode(&vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!( process_simple_intopcode(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!( process_simple_intopcode(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
        assert_eq!( process_simple_intopcode(&vec![1002,4,3,4,33]), vec![1002,4,3,4,99]);
        assert_eq!( process_simple_intopcode(&vec![1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn leq_intopcode_test() {
        {
            let test = vec![3,9,8,9,10,9,4,9,99,-1,8]; // 1 if equal to 8, 0 otherwise
            assert_eq!( get_intopcode_output(&test, 8), vec![1] );
            assert_eq!( get_intopcode_output(&test, 7), vec![0] );
            assert_eq!( get_intopcode_output(&test, 9), vec![0] );
        }
        
        {
            let test = vec![3,9,7,9,10,9,4,9,99,-1,8]; // 1 if less than 8, 0 otherwise
            assert_eq!( get_intopcode_output(&test, 7), vec![1] );
            assert_eq!( get_intopcode_output(&test, 8), vec![0] );
            assert_eq!( get_intopcode_output(&test, 9), vec![0] );
        }
        
        {
            let test = vec![3,3,1108,-1,8,3,4,3,99]; // 1 if equal to 8, 0 otherwise (immediate)
            assert_eq!( get_intopcode_output(&test, 8), vec![1] );
            assert_eq!( get_intopcode_output(&test, 7), vec![0] );
            assert_eq!( get_intopcode_output(&test, 9), vec![0] );
        }
        
        {
            let test = vec![3,3,1107,-1,8,3,4,3,99]; // 1 if less than 8, 0 otherwise (immediate)
            assert_eq!( get_intopcode_output(&test, 7), vec![1] );
            assert_eq!( get_intopcode_output(&test, 8), vec![0] );
            assert_eq!( get_intopcode_output(&test, 9), vec![0] );
        }
    }

    #[test]
    fn jump_intopcode_test() {
        // Here are some jump tests that take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero:
        {
            let test = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
            assert_eq!( get_intopcode_output(&test, 0), vec![0] );
            assert_eq!( get_intopcode_output(&test, 10), vec![1] );
        }

        {
            let test = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
            assert_eq!( get_intopcode_output(&test, 0), vec![0] );
            assert_eq!( get_intopcode_output(&test, 10), vec![1] );
        }
    }

    #[test]
    fn complex_jump_intopcode_test() {
        let test = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!( get_intopcode_output(&test, 0), vec![999] );
        assert_eq!( get_intopcode_output(&test, 7), vec![999] );
        assert_eq!( get_intopcode_output(&test, 8), vec![1000] );
        assert_eq!( get_intopcode_output(&test, 9), vec![1001] );
        assert_eq!( get_intopcode_output(&test, 15), vec![1001] );
    }

    #[test]
    fn aoc_daythree_test() {
        let tests = [("R8,U5,L5,D3
        U7,R6,D4,L4", 6, 30 ),
        ("R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83", 159, 610 ),
        ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135, 410 )];

        for test in tests.iter() {
            let dist = compute_min_dist(&test.0);
            assert_eq!( dist.0, test.1 );
            assert_eq!( dist.1, test.2 );
        }
    }

    #[test]
    fn aoc_dayfour_valid_password_test() {
        assert_eq!( vet_password(122345, false), true );
        assert_eq!( vet_password(122345, true), true );
        assert_eq!( vet_password(111123, false), true );
        assert_eq!( vet_password(111123, true), false ); // fails part 2, no pair
        assert_eq!( vet_password(135679, false), false ); // no dupe
        assert_eq!( vet_password(135679, true), false ); // no dupe
        assert_eq!( vet_password(111111, false), true ); // no dupe
        assert_eq!( vet_password(111111, true), false ); // fails part 2, no pair
        assert_eq!( vet_password(223450, false), false ); // not decreasing
        assert_eq!( vet_password(223450, true), false ); // not decreasing
        assert_eq!( vet_password(123789, false), false ); // no double
        assert_eq!( vet_password(123789, true), false ); // no double
        assert_eq!( vet_password(1000000, false), false ); // too large
        assert_eq!( vet_password(1000000, true), false ); // too large
        assert_eq!( vet_password(112233, false), true );
        assert_eq!( vet_password(112233, true), true );
        assert_eq!( vet_password(123444, false), true );
        assert_eq!( vet_password(123444, true), false ); // fails part 2, no pair
        assert_eq!( vet_password(111122, false), true );
        assert_eq!( vet_password(111122, true), true );
    }

    #[test]
    fn parse_orbits_and_planets_test() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

        let (orbits, planets) = parse_orbits_and_planets(input);
        assert_eq!( orbits, [("COM", "B"), ("B", "C"), ("C", "D"), ("D","E"), ("E","F"), ("B","G"), ("G","H"), ("D","I"), ("E","J"), ("J","K"), ("K","L")] );
        assert_eq!( compute_orbits( &orbits, "D" ), 3 );
        assert_eq!( compute_orbits( &orbits, "L" ), 7 );
        assert_eq!( compute_orbits( &orbits, "COM" ), 0 );
        assert_eq!( sum_all_orbits( &orbits, &planets), 42 );
    }

    #[test]
    fn test_orbital_distance() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
 
        assert_eq!(orbital_distance(input, "YOU", "SAN"), 4);
    }

    #[test]
    fn test_amp() {
        {
            let test = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
            let phases = vec![4,3,2,1,0];
            assert_eq!(process_amp_intopcode_with_phases(&test, &phases, false), 43210);
        }    

        {
            let test = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
            let phases = vec![0,1,2,3,4];
            assert_eq!(process_amp_intopcode_with_phases(&test, &phases, false), 54321);
        }

        {
            let test = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
            let phases = vec![1,0,4,3,2];
            assert_eq!(process_amp_intopcode_with_phases(&test, &phases, false), 65210);
        }
    }

    #[test]
    fn test_amp_feedback() {
        {
            let test = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
            let phases = vec![9,8,7,6,5];
            assert_eq!(process_amp_intopcode_with_phases(&test, &phases, true), 139629729);
        }    

        {
            let test = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
            let phases = vec![9,7,8,5,6];
            assert_eq!(process_amp_intopcode_with_phases(&test, &phases, true), 18216);
        }
    }

    #[test]
    fn test_max_amp() {
        let phases = [0,1,2,3,4];
        {
            let test = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
            assert_eq!(compute_max_amp_intopcode(&test, &phases, false), 43210);
        }    

        {
            let test = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
            assert_eq!(compute_max_amp_intopcode(&test, &phases, false), 54321);
        }

        {
            let test = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
            assert_eq!(compute_max_amp_intopcode(&test, &phases, false), 65210);
        }
    }

    #[test]
    fn test_max_amp_feedback() {
        let phases = [5,6,7,8,9];
        {
            let test = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
            assert_eq!(compute_max_amp_intopcode(&test, &phases, true), 139629729);
        }    
        {
            let test = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
            assert_eq!(compute_max_amp_intopcode(&test, &phases, true), 18216);
        }    
    }

    #[test]
    fn test_parse_elf_images() {
        let input = "123456789012";
        let height = 2;
        let width = 3;
        let elf_images = ElfImage::from_input(height, width, input);
        let first_layer : Vec<u32> = vec![1,2,3,4,5,6];
        assert_eq!(elf_images.transmitted_layers.len(), 2);
        assert_eq!(elf_images.transmitted_layers[0], first_layer);
        assert_eq!(elf_images.transmitted_layers[1], vec![7,8,9,0,1,2]);
        assert_eq!(elf_images.height, height);
        assert_eq!(elf_images.width, width);
        assert_eq!(elf_images.layer_with_min_character(0), &first_layer);
    }

    #[test]
    fn test_decode_image() {
        let input = "0222112222120000";
        let height = 2;
        let width = 2;
        let elf_image = ElfImage::from_input(height, width, input);
        assert_eq!(elf_image.transmitted_layers.len(), 4 );
        assert_eq!(elf_image.decode_image(), vec![0,1,1,0] );
    }

    #[test]
    fn aoc_day_nine_tests() {
        {
            let test = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
            assert_eq!(get_intopcode_output(&test, 0), test);
        }
        {
            let test = vec![1102,34915192,34915192,7,4,7,99,0];
            assert_eq!(get_intopcode_output(&test, 0), vec![1219070632396864]);
        }
        {
            let test = vec![104,1125899906842624,99];
            assert_eq!(get_intopcode_output(&test, 0), vec![1125899906842624]);
        }
    }

    #[test]
    fn parse_asteroid_field_test() {
        let input = ".#..#
        .....
        #####
        ....#
        ...##";

        let asteroids = parse_asteroid_field(&input);
        assert_eq!(asteroids.len(), 10);
        assert_eq!(asteroids[0].pos, Vector2::new(1, 0));
        assert_eq!(asteroids[9].pos, Vector2::new(4, 4));
    }

    #[test]
    fn asteroid_los_test() {
        let input = ".#..#
        .....
        #####
        ....#
        ...##";

        let asteroids = parse_asteroid_field(&input);
        assert_eq!(compute_asteroid_los_hashmap(&asteroids[0], &asteroids).keys().count(), 7);
        assert_eq!(compute_all_asteroid_los_count(&asteroids), vec![7,7,6,7,7,7,5,7,8,7]);
        assert_eq!(compute_max_asteroid_los_count(&asteroids).1, 8);
    }

    #[test]
    fn asteroid_los_test_complex() {
        {
            let input = "......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####";

            let asteroids = parse_asteroid_field(&input);
            let a = Asteroid{ pos: Vector2::new(5, 8)};
            assert_eq!(compute_asteroid_los_hashmap(&a, &asteroids).keys().count(), 33);
            let (a, count) = compute_max_asteroid_los_count(&asteroids);
            assert_eq!(a.pos, Vector2::new(5, 8));
            assert_eq!(count, 33);
        }

        {
            let input = "#.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.";
            let asteroids = parse_asteroid_field(&input);
            let a = Asteroid{ pos: Vector2::new(1, 2)};
            assert_eq!(compute_asteroid_los_hashmap(&a, &asteroids).keys().count(), 35);
            assert_eq!(compute_max_asteroid_los_count(&asteroids).1, 35);
        }
        
        {
            let input = ".#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..";
            let asteroids = parse_asteroid_field(&input);
            let a = Asteroid{ pos: Vector2::new(6, 3)};
            assert_eq!(compute_asteroid_los_hashmap(&a, &asteroids).keys().count(), 41);
        }
    }

    #[test]
    fn asteroid_los_laser_test() {
        let input = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";

        let asteroids = parse_asteroid_field(&input);
        let a = Asteroid{ pos: Vector2::new(11, 13)};
        assert_eq!(compute_asteroid_los_hashmap(&a, &asteroids).keys().count(), 210);
        let asteroid_do = compute_asteroid_destruction_order(&a, &asteroids);
        assert_eq!(asteroid_do[0].pos, Vector2::new(11,12));
        assert_eq!(asteroid_do[1].pos, Vector2::new(12,1));
        assert_eq!(asteroid_do[2].pos, Vector2::new(12,2));
        assert_eq!(asteroid_do[9].pos, Vector2::new(12,8));
        assert_eq!(asteroid_do[19].pos, Vector2::new(16,0));
        assert_eq!(asteroid_do[49].pos, Vector2::new(16,9));
        assert_eq!(asteroid_do[99].pos, Vector2::new(10,16));
        assert_eq!(asteroid_do[198].pos, Vector2::new(9,6));
        assert_eq!(asteroid_do[199].pos, Vector2::new(8,2));
        assert_eq!(asteroid_do[200].pos, Vector2::new(10,9));
        assert_eq!(asteroid_do[298].pos, Vector2::new(11,1));
    }
}