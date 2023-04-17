use std::{error::Error, ops::Index, vec};
use csv::{self, StringRecord};
use mysql::*;
use mysql::prelude::*;

mod mutants;

/// Reads csv of numbers with no header
///
/// ### Error
/// If an error occurs, the error is returned to `main`.
/* fn read_from_file(path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>>
{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(path)?;

    let mut matrix: Vec<Vec<i32>> = vec![vec![]];
    matrix.pop();
    for result in reader.records()
    {
        let record: StringRecord = result?;

        let mut row = vec![];
        for i in record.into_iter()
        {
            let num = i.parse::<i32>()?;
            row.push(num);
        }
        matrix.push(row);
    }
    println!("row len = {}", matrix.len());
    println!("column len = {}", matrix.first().unwrap().len());

    Ok(matrix)
} */


/// Gets mutations from struct iterating over to find mismatches, if none returns as match for Mutant type
fn match_mutant_indexes<T: mutants::Mutations>(mutant: T, row: &Vec<i32>) -> bool {
    let mut is_mutant = true;
    let mutations = mutant.get_mutated_genes();
    for i in mutations {
        let i_usized = usize::try_from(i).unwrap();
        if *(row.index(i_usized)) == 0 {
            is_mutant = false;
        }
    }
    is_mutant
}

#[derive(Debug, PartialEq, Eq)]
struct Genetics {
    id: i32,
    gene_1: i32,
    gene_2: i32,
    gene_3: i32,
    gene_4: i32,
    gene_5: i32,
    gene_6: i32,
    gene_7: i32,
    gene_8: i32,
    gene_9: i32,
    gene_10: i32,
}

fn main()
{
    let url = "mysql://root:password@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let selected_payments = conn
        .query_map(
            "SELECT id, gene_1, gene_2, gene_3, gene_4, gene_5, gene_6, gene_7, gene_8, gene_9, gene_10 from genetics",
            |(id, gene_1, gene_2, gene_3, gene_4, gene_5, gene_6, gene_7, gene_8, gene_9, gene_10)| {
                Genetics { id, gene_1, gene_2, gene_3, gene_4, gene_5, gene_6, gene_7, gene_8, gene_9, gene_10 }
            },
        ).unwrap();
    println!("{:?}", selected_payments);


    /* match read_from_file("./test_input.csv") {
        Ok(matrix) => { 
            let mut mutant_as = 0;
            let mut mutant_bs = 0;
            for row in matrix {
                if match_mutant_indexes(mutants::MutantA, &row) {
                    mutant_as += 1;
                }

                if match_mutant_indexes(mutants::MutantB, &row) {
                    mutant_bs+= 1;
                }
            }
            println!("Mutant a count = {}", mutant_as);
            println!("Mutant b count = {}", mutant_bs);
        },
        Err(e) => {
            eprintln!("{}", e);
        },
} */



}
