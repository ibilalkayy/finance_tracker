mod commands;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::Value;
use crate::commands::commands::{Finance, Types, TransactionData};
use clap::Parser;

fn main() {
    let finance: Finance = Finance::parse();

    match finance.transaction_types {
        Types::Add(transaction) => {
            let new_data = TransactionData {
                description: transaction.description,
                amount: transaction.amount,
                types: transaction.types,
                category: transaction.category,
            };

            let serialized_new_data = serde_json::to_string(&new_data).unwrap();
            
            let file_path = "data.json";
            let mut transactions = Vec::new();

            if let Ok(mut file) = File::open(file_path) {
                let mut content = String::new();
                file.read_to_string(&mut content).expect("Failed to read the file");

                // Parse existing data as JSON array
                if !content.is_empty() {
                    transactions = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
                }
            }

            // Add the new transaction to the existing ones
            transactions.push(serde_json::from_str::<Value>(&serialized_new_data).unwrap());

            // Write back the updated JSON array
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .expect("Failed to open the file");
            file.write_all(serde_json::to_string_pretty(&transactions).unwrap().as_bytes())
                .expect("Failed to write the data");

            println!("The data is now successfully added to the JSON file");
        }

        Types::View(options) => {
            if options.all {
                let mut file = File::open("data.json").expect("failed to open the file");
                let mut contents = String::new();
                
                file.read_to_string(&mut contents).expect("failed to read the file");
                
                // Parse the JSON into a structured format
                let transactions: Vec<TransactionData> =
                    serde_json::from_str(&contents).expect("failed to parse JSON");
                
                // Pretty-print the JSON
                let pretty_output =
                    serde_json::to_string_pretty(&transactions).expect("failed to format JSON");
                
                println!("{}", pretty_output);
            } else {
                println!("No data found");
            }
        },
        
    }
}
