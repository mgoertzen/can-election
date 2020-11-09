use std::error::Error;
use std::env::args;
use std::fs::File;
use serde::Deserialize;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Votes {
	Contested(u64),
	Uncontested(String)
}

#[derive(Debug, Deserialize)]
struct Candidate {
	#[serde(rename = "Election Date")]
	date: String,
	#[serde(rename = "Election Type")]
	election_type: String,
	#[serde(rename = "Parliament")]
	parliament: u64,
	#[serde(rename = "Province")]
	province: String,
	#[serde(rename = "Riding")]
	riding: String,
	#[serde(rename = "Last Name")]
	surname: String,
	#[serde(rename = "First Name")]
	given_name: Option<String>,
	#[serde(rename = "Gender")]
	gender: Option<String>,
	#[serde(rename = "Occupation")]
	occupation: Option<String>,
	#[serde(rename = "Party")]
	party: Option<String>,
	#[serde(rename = "Votes")]
	votes: Votes,
	#[serde(rename = "Votes (%)")]
	vote_percent: Option<f64>,
	#[serde(rename = "Elected")]
	elected: u8
}

fn read_results(filename: String) -> Result<Vec<Candidate>, Box<dyn Error>> {
	let file = File::open(filename)?;
	let transcoded = DecodeReaderBytesBuilder::new()
		.encoding(Some(WINDOWS_1252))
		.build(file);
	let mut reader = csv::Reader::from_reader(transcoded);
	let mut results: Vec<Candidate> = Vec::new();
	for result in reader.deserialize() {
		let candidate: Candidate = result?;
		results.push(candidate);
	}
	Ok(results)
}

fn get_filename() -> Option<String> {
	args().nth(1)
}

fn main() {
	let filename = get_filename().expect("No filename given");
	let candidates = read_results(filename).expect("Error parsing list");
	for candidate in candidates {
		println!("{:?}", candidate);
	}
}
