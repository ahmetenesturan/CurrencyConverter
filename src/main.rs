use clap::Parser;

//parser struct
#[derive(Parser)]
struct CliArguments{
    ///Name of the first currency
    #[arg(short, long)]
    first_currency:String,
    ///Amount of the first currency
    #[arg(short, long)]
    amount:f32,
    ///Name of the second currency
    #[arg(short, long)]
    second_currency:String
}

fn match_currency_name(in_cur:&String) -> String{
    let out_cur:String;
    match in_cur.as_str(){
        "us_dollar" => out_cur = String::from("US Dollar"),
        "euro" => out_cur = String::from("Euro"),
        _ => out_cur = String::from("N/A"),
    }
    return out_cur;
}

fn main() {
    let cli_args = CliArguments::parse();
    println!("{} {} is x {}", &cli_args.amount, match_currency_name(&cli_args.first_currency), match_currency_name(&cli_args.second_currency));
}
