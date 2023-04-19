pub mod seq_property {
    pub use serde::{Serialize, Deserialize};
    pub use std::fs::File;
    pub use std::io::{Read,Write};

    #[derive(Serialize,Debug)]
    pub struct SeqProperty {
        pub filename: String,
        avg_qual: f32,
        min_qual: f32,
        max_qual: f32,
        total_reads: i32,
        total_bases: i32,
        avg_len: f32,
        min_len: i32,
        max_len: i32,
        pub qual_per_reads: Vec<f32>,
        pub len_per_reads: Vec<i32>,
    }

    impl SeqProperty {
        pub fn new(filename: String) -> SeqProperty{
            SeqProperty{
                filename,
                avg_qual: 0.0,
                min_qual: 0.0,
                max_qual: 0.0,
                total_reads: 0,
                total_bases: 0,
                avg_len: 0.0,
                min_len: 0,
                max_len: 0,
                qual_per_reads: vec![],
                len_per_reads: vec![],
            }
        }
        // Add switch for histogram
        pub fn summarise (&mut self){
            self.max_qual = self.qual_per_reads.iter().fold(std::f32::NEG_INFINITY, |max, &val| val.max(max));
            self.min_qual = self.qual_per_reads.iter().fold(std::f32::MAX, |min, &val| val.min(min));

            let sum: f32 = self.qual_per_reads.iter().sum();
            let len: f32 = self.qual_per_reads.len() as f32;
            self.avg_qual = sum/len;

            self.min_len = *self.len_per_reads.iter().min().unwrap();
            self.max_len = *self.len_per_reads.iter().max().unwrap();
            self.total_reads = self.len_per_reads.len() as i32;
            self.total_bases = self.len_per_reads.iter().sum();
            self.avg_len = self.total_bases as f32/len;
        }

        pub fn get_data (&mut self){
            println!("{}\t{}\t{}\t{}-{}\t{}\t{}-{}\t{}", 
                self.filename,
                self.total_reads,
                self.total_bases,
                self.min_len,
                self.max_len,
                self.avg_len,
                self.min_qual,
                self.max_qual,
                self.avg_qual);
        }
        pub fn print_hist (&mut self, box_size: i32, bin_size: i32, max_len: i32){
            let hist = Histogram::new(&self.len_per_reads, box_size, bin_size, max_len);
            hist.print_hist();
        }
        pub fn to_json (&self, outpath: String) {
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

    pub struct Histogram {
        hist_data: Vec<i32>,
        box_size: i32,
        bin_size: i32,
    }

    impl Histogram {
        pub fn new(data_per_reads: &Vec<i32>, box_size: i32, bin_size: i32, max_len: i32) -> Histogram {
            let num_bin: i32 = (max_len + bin_size -1)/bin_size;
            println!("{:#?}", &num_bin);
            let mut hist_data = vec![0; num_bin as usize];
            for i in data_per_reads.iter() {
                let bin_idx = i / bin_size;
                if bin_idx < data_per_reads.len() as i32 {
                hist_data[bin_idx as usize] += 1;
                }
            }
            Histogram {
                hist_data,
                box_size,
                bin_size,
            }
        }
        pub fn print_hist (self){
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
}}



