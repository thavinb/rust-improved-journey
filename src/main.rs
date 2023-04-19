pub use serde::{Serialize, Deserialize};
pub mod seqprop;
use crate::seqprop::seq_property::SeqProperty;
use std::sync::mpsc;
use std::io::{Write, BufReader, Read};
use std::fs::File;
use std::env;
use std::thread;
use bio::io::fastq;
use flate2::bufread::MultiGzDecoder;

use std::io;
fn decode_reader(file: &String) -> io::Result<BufReader<MultiGzDecoder<BufReader<File>>>>{
    let f = File::open(file)?;
    let buf_reader = BufReader::new(f);
    let gz = MultiGzDecoder::new(buf_reader);
    Ok(BufReader::new(gz))
}

fn main()  {
	// CLI
    // Deprecate: Postional arguments: filename, bin_size, and box_size
    // Postional arguments: [file1, file2, ..]
    println!("Filename\tTotal_reads\tTotal_bases\tlen(mix-max)\tavg_len\tqual(min-max)\tavg_qual");
    let mut args: Vec<String> = env::args().collect();
    let files = args.splice(1.., Vec::new());
    let mut stat_vec: Vec<SeqProperty> = vec![];
    let (tx,rx) = mpsc::channel();
    println!("{:?}",&files);
    for filename in files {
        let tx_clone = tx.clone();
        thread::spawn(move || {
                let mut fastq_stats = SeqProperty::new(filename.clone());

                // println!("Starting {}",&filename);
                let f = decode_reader(&filename);
                let reader = fastq::Reader::from_bufread(f.unwrap()) ;
                let records = reader.records().map(|record| record.unwrap());

                // Iterate over each record
                for record in records {
                    let seq_len: usize = record.seq().len();
                    fastq_stats.len_per_reads.push(seq_len as i32);
                    fastq_stats.qual_per_reads.push(average(&record.qual()));
                }

                // println!("finishig {}",&filename);
                //print
                fastq_stats.summarise();
                fastq_stats.get_data();
                tx_clone.send(fastq_stats).unwrap();

        });
    }
    drop(tx);
    loop {
    match rx.recv() {
        Ok(receive) => {
            // println!("processing: {}",receive.filename);
            stat_vec.push(receive);
        }
        Err(_) => {
            // Channel is closed, break out of loop
            break;
        }
    }}
    // loop {
    // match rx.try_recv() {
    //     Ok(receive) => {
    //         println!("processing: {}",receive.filename);
    //         stat_vec.push(receive);
    //     }
    //     Err(mpsc::TryRecvError::Empty) => {
    //         // Channel is empty, continue main thread
    //         // println!("Channel is Empty");
    //         continue;
    //     }
    //     Err(mpsc::TryRecvError::Disconnected) => {
    //         // Channel is closed, break out of loop
    //         println!("Disconnected!!!");
    //         break;
    //     }
    // }}
    // println!("Wrapping up...");
    to_json(&stat_vec, String::from("output.json"));
    // println!("Done!");
}

fn average(numbers: &[u8]) -> f32 {
    let sum: f32 = numbers.iter().map(|&i| i as f32).sum();
    sum / numbers.len() as f32 
}
fn to_json (input: &Vec<SeqProperty>, outpath: String) {
    let serialized = serde_json::to_string(input).unwrap(); 
    let mut json_output = match File::create(outpath) {
        Ok(json_output) => json_output,
        Err(err) => {
            panic!("Error creating output file {err}");
        },
    };
    json_output.write_all(serialized.as_bytes()).expect("Someting wrong when writing");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
//
// if seq_len == 1 {
//     println!("id:{:?}, desc:{:?}", record.id(), record.desc());
// }
