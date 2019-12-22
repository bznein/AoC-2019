use std::io::{self, Read};

fn deal(deck: &mut Vec<u16>)
{
    deck.reverse()
}

fn cut_front(deck: &mut Vec<u16>, n: usize)
{
    let num_cards = deck.len();
    let mut buf = Vec::new();
    for i in 0..n
    {
        buf.push(deck[i]);
    }
    for i in 0..num_cards-n
    {
        deck[i]=deck[i+n];
    }
    for i in 0..n
    {
        deck[num_cards-n+i]=buf[i];
    }
}

fn cut_back(deck: &mut Vec<u16>, n: usize)
{
    let num_cards = deck.len();
    let mut buf = Vec::new();
    for i in 0..n
    {
        buf.push(deck[num_cards-1-i]);
    }
    for i in 0..num_cards-n
    {
        deck[num_cards-1-i]=deck[num_cards-1-i-n];
    }
    for i in 0..n
    {
        deck[i]=buf[n-1-i];
    }
}

fn deal_with_increment(deck: &Vec<u16>, increment: usize) -> Vec<u16>
{
    let num_cards = deck.len();
    let mut new_deck = vec![0;num_cards];
    for i in 0..num_cards
    {
        new_deck[(increment*i)%num_cards]=deck[i]
    }
    new_deck
}

fn cut(deck: &mut Vec<u16>, n: i32)
{
    if n > 0
    {
        cut_front(deck, n as usize);
    }
    else
    {
        cut_back(deck, (-n) as usize);
    }
}

fn main() {
    let num_cards = 10007;
    let mut deck : Vec<_> = (0u16..num_cards).map(u16::from).collect();
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    if input.ends_with('\n') {
        input.truncate(input.len() - 1);
    }

    for command in input.split('\n')
    {
        let param : Vec<_> = command.split_whitespace().collect();
        if param[0]=="deal"
        {
            if param[1] == "into"
            {
                deal(& mut deck);
            }
            else if param[1] == "with"
            {
                deck = deal_with_increment(&deck, param[3].parse::<usize>().unwrap());
            }
            else
            {
                panic!("Unknown command! : {:?}", param);
            }
        }
        else if param[0] =="cut"
        {
            cut(&mut deck, param[1].parse::<i32>().unwrap());
        }
        else
        {
            panic!("Unknown command! : {:?}", param);
        }
    }
    println!("Part 1: {}", deck.iter().position(|&x| x==2019).unwrap());
}
