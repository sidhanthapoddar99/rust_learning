


enum Number_Card
{
    one, two, three, four, five, six, seven, eight, nine
}

enum Action_Card
{
    skip, reverse, draw_two
}

enum Wild_Card
{
    wild, wild_draw_four
}


enum Card
{
    Number(u8),
    Action(Action_Card),
    Wild(Wild_Card)
}

impl Card
{
    fn Number(num: u8) -> Card
    {   
        if num>0 && num<10
        {
            return Card::Number(num);
        }
        return Card::Number(1);
    }

}



fn is_number_card(Card_type: &Card) -> bool
{
    match Card_type
    {
        Card::Number(_) => true,
        _ => false
    }
}

fn is_action_card(Card_type: &Card) -> bool
{
    match Card_type
    {
        Card::Action(_) => true,
        _ => false
    }
}

fn is_wild_card(Card_type : &Card) -> bool
{
    match Card_type
    {
        Card::Wild(_) => true,
        _ => false
    }
}


fn main() {

    let card1 = Card::Number(7);
    let card2 = Card::Action(Action_Card::skip);
    let card3 = Card::Wild(Wild_Card::wild_draw_four);

    assert_eq!(is_number_card(&card1), true);
    assert_eq!(is_action_card(&card2), true);
    assert_eq!(is_wild_card(&card3), true);

    // print result

    println!("Card 1 is a number card: {}", is_number_card(&card1));
    println!("Card 2 is an action card: {}", is_action_card(&card2));
    println!("Card 3 is a wild card: {}", is_wild_card(&card3));


}