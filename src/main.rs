use std::io::{Read,Write};
use std::fs::File;
use bio::io::fastq;
use serde::{Serialize, Deserialize};
use std::env;
use std::thread;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

#[derive(Serialize)]
struct SeqProperty {
    filename: String,
    avg_qual: f32,
    min_qual: f32,
    max_qual: f32,
    total_reads: i32,
    total_bases: i32,
    min_len: i32,
    max_len: i32,
    qual_per_reads: Vec<f32>,
    len_per_reads: Vec<i32>,
}

impl SeqProperty {
    fn new(filename: String) -> SeqProperty{
        SeqProperty{
            filename,
            avg_qual: 0.0,
            min_qual: 0.0,
            max_qual: 0.0,
            total_reads: 0,
            total_bases: 0,
            min_len: 0,
            max_len: 0,
            qual_per_reads: vec![],
            len_per_reads: vec![],
        }
    }
    fn summarise (&mut self){
        self.max_qual = self.qual_per_reads.iter().fold(std::f32::MIN, |max, &val| val.max(max));
        self.min_qual = self.qual_per_reads.iter().fold(std::f32::MAX, |min, &val| val.min(min));

        let sum: f32 = self.qual_per_reads.iter().sum();
        let len: f32 = self.qual_per_reads.len() as f32;
        self.avg_qual = sum/len;

        self.min_len = *self.len_per_reads.iter().min().unwrap();
        self.max_len = *self.len_per_reads.iter().max().unwrap();
        self.total_reads = self.len_per_reads.len() as i32;
        self.total_bases = self.len_per_reads.iter().sum();
    }
    fn get_data (&mut self){
        println!("filename: {}, total_reads: {}, total_bases: {}, avg_qual: {}", 
            self.filename,
            self.total_reads,
            self.total_bases,
            self.avg_qual);
    }
    fn print_hist (&mut self, box_size: i32, bin_size: i32, max_len: i32){
        let hist = Histogram::new(&self.len_per_reads, box_size, bin_size, max_len);
        hist.print_hist();
    }
    fn to_json (&self, outpath: String) {
        let serialized = serde_json::to_string(self).unwrap(); 
        let mut json_output = match File::create(outpath) {
            Ok(json_output) => json_output,
            Err(err) => {
                panic!("Error creating output file {err}");
            },
        };
        json_output.write_all(serialized.as_bytes()).expect("Someting wrong when writing");
        println!("{serialized}");
    }
}

struct Histogram {
    hist_data: Vec<i32>,
    box_size: i32,
    bin_size: i32,
}

impl Histogram {
    fn new(data_per_reads: &Vec<i32>, box_size: i32, bin_size: i32, max_len: i32) -> Histogram {
        let num_bin: i32 = (max_len + bin_size -1)/bin_size;
        println!("{:#?}", &num_bin);
        let mut hist_data = vec![0; num_bin as usize];
        for i in data_per_reads.iter() {
            let bin_idx = i / bin_size;
            //Debug
            if bin_idx > 10 {
                println!("i of data is: {}, and bin_idx is {}", i,bin_idx);
            }

            if bin_idx < data_per_reads.len() as i32 {
			hist_data[bin_idx as usize] += 1;
            }
        }
        Histogram {
            hist_data: hist_data,
            box_size,
            bin_size,
        }
    }
    fn print_hist (self){
    for (i, count) in self.hist_data.iter().enumerate() {
            let bin_start = i as i32 * self.bin_size ;
            let bin_end = (i as i32 + 1) * self.bin_size;
            if *count > 0 {
                println!("{:4}-{:4}\t|{} {}",
                         bin_start,
                         bin_end - 1,
                         "#".repeat(*count as usize/ self.box_size as usize),
                         *count);
            }
        }
}



}

fn main()  {
	// CLI
    // Postional arguments: filename, bin_size, and box_size
    let args: Vec<String> = env::args().collect();
    let filenames = args.clone();
    // let bin_size = args[2].parse::<i32>().unwrap();
	// let box_size = args[3].parse::<i32>().unwrap();

    let mut handles: Vec<thread::JoinHandle<Result<(), std::io::Error>>> = Vec::new();
    for filename in filenames {
        let handle = thread::spawn(move || {

        let mut fastq_stats = SeqProperty::new(filename.clone());

        // Read fastq
        let reader =  match fastq::Reader::from_file(filename) {
            Ok(reader) => reader,
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                return Ok(());
            }
        };


        // Get iterator object out of fastq::Reader
        let records = reader.records().map(|record| record.unwrap());

        // Iterate over each record
        for record in records {
            let seq_len: usize = record.seq().len();
            fastq_stats.len_per_reads.push(seq_len as i32);
            fastq_stats.qual_per_reads.push(average(&record.qual()));
        }

        //print
        fastq_stats.summarise();
        fastq_stats.get_data();
        fastq_stats.to_json(String::from("placeholder"));
        Ok(())

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join();
    };

}

fn average(numbers: &[u8]) -> f32 {
    let sum: f32 = numbers.iter().map(|&i| i as f32).sum();
    sum / numbers.len() as f32 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
