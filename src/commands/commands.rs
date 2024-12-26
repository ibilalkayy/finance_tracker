use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Finance {
    #[clap(subcommand)]
    pub transaction_types: Types,
}

#[derive(Debug, Subcommand)]
pub enum Types {
    /// Add a new transaction
    Add(Transactions),

    View(ViewOptions),
}

#[derive(Parser, Debug)]
pub struct Transactions {
    #[clap(short, long)]
    /// Description of the transaction
    pub description: String,

    #[clap(short, long)]
    /// Amount of the transaction
    pub amount: f64,

    #[clap(short, long)]
    /// Type of the transaction
    pub types: String,

    #[clap(short, long)]
    /// Category of the transaction
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct ViewOptions {
    /// View all transactions
    #[clap(long)]
    pub all: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub description: String,
    pub amount: f64,
    pub types: String,
    pub category: String,
}
