use std::{fs::File, io::{BufReader, BufWriter}};
use std::path::{Path, PathBuf};

use read_input::prelude::*;

use wav::BitDepth;

fn main() {
	let (wav_info, mut wav_data) = {
		let mut src_file = {
			let mut returner = None;

			while returner.is_none() {
				//Get filepath from user.
				let filepath : String = input()
					.msg("Enter the path of the .wav file to bitshift: ")
					.add_err_test(|val| {
							File::open(val).is_ok()
						},
						"File not found."
					)
					.get()
				;
				let filepath = Path::new(&filepath);
				
				//Get new input if the path doesn't work.
				match filepath.extension() {
					Some(ex) => if ex != "wav" {continue},
					None => continue //can be hit if the file is extensionless
				};


				returner = Some(BufReader::new(File::open(filepath).unwrap()) ); //Filepath input will fail before an Err val can get to this point
			}

			returner.unwrap()
		};

		println!("Loading file...", );

		wav::read(&mut src_file).unwrap_or_default()
	};

	match wav_data {
		BitDepth::Empty => println!("This file contains no data."),
		BitDepth::Eight(_) => println!("This file's bit depth is 8."),
		BitDepth::Sixteen(_) => println!("This file's bit depth is 16."),
		BitDepth::TwentyFour(_) => println!("This file's bit depth is 24."),
		BitDepth::ThirtyTwoFloat(_) => println!("This file is floating point; bit shifts won't really work.")
	};
		
	let shift_amount = {
		println!("How many bits should I shift the samples?");
		
		let bits_max = (wav_info.bits_per_sample as i8) - 1;
		input()
			.msg("Negative=L, Positive=R: ")
			.inside_err(
				-bits_max..=bits_max,
				"Shifting that far will completely blank all the samples."
			)
			.add_err_test(|val| {
					*val != 0
				},
				"Shifting by 0 bits won't change the audio at all."
			)
			.get()
	};

	println!("Shifting audio...");
	let do_output = match wav_data {
		BitDepth::Empty => false,
		BitDepth::Eight(mut dta) => {
			if shift_amount < 0 { 
				for sample in dta.iter_mut() {
					*sample <<= (-shift_amount) as u8;
				};
			} else {
				for sample in dta.iter_mut() {
					*sample >>= shift_amount as u8;
				};
			}
			wav_data = BitDepth::from(dta);
			true
		},
		BitDepth::Sixteen(mut dta) => {
			if shift_amount < 0 { 
				for sample in dta.iter_mut() {
					*sample <<= (-shift_amount) as u8;
				};
			} else {
				for sample in dta.iter_mut() {
					*sample >>= shift_amount as u8;
				};
			}
			wav_data = BitDepth::from(dta);
			true
		},
		BitDepth::TwentyFour(mut dta) => {
			if shift_amount < 0 { 
				for sample in dta.iter_mut() {
					*sample <<= (-shift_amount) as u8;
				};
			} else {
				for sample in dta.iter_mut() {
					*sample >>= shift_amount as u8;
				};
			}
			wav_data = BitDepth::from(dta);
			true
		},
		BitDepth::ThirtyTwoFloat(_) => false
	};
	println!("");


	if do_output {
		let mut dst_file = {
			//Get filepath from user.
			let outpath : String = input()
				.msg("Enter the path to save the shifted audio to: ")
				.add_err_test(|val| {
						*val != ""
					},
					"Please enter a path. "
				)
				.add_err_test(|val| {
						File::open(val).is_err()
					},
					"That file already exists."
				)
				.add_err_test(|val| {
						!Path::new(val).is_dir()
					},
					"That path already points to a folder."
				)
				.get()
			;
			let mut outpath = PathBuf::from(outpath);
			
			//Make sure the out file ends up with a .wav extension, even if it doesn't already Γ)
			if let Some(ex) = outpath.extension() {
				if ex != "wav" {
					outpath.set_extension("wav");
				}
			} else {
				outpath.set_extension("wav");
			};

			println!("Saving bit-shifted audio to {}...", outpath.to_str().unwrap());

			BufWriter::new(File::create(outpath).unwrap())
		};


		if let Err(errval) = wav::write(wav_info, &wav_data, &mut dst_file) {
			eprint!("ERROR: {}", errval);
		} else {
			print!("Audio saved successfully.");
		}
	}
}