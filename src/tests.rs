use crate::*;

#[test]
fn test_xkcd() {
    assert_eq!(xkcd::Comic {
        month: "7".to_owned(),
        num: 614,
        link: "".to_owned(),
        year: "2009".to_owned(),
        news: "".to_owned(),
        safe_title: "Woodpecker".to_owned(),
        transcript: "[[A man with a beret and a woman are standing on a boardwalk, leaning on a handrail.]]\nMan: A woodpecker!\n<<Pop pop pop>>\nWoman: Yup.\n\n[[The woodpecker is banging its head against a tree.]]\nWoman: He hatched about this time last year.\n<<Pop pop pop pop>>\n\n[[The woman walks away.  The man is still standing at the handrail.]]\n\nMan: ... woodpecker?\nMan: It's your birthday!\n\nMan: Did you know?\n\nMan: Did... did nobody tell you?\n\n[[The man stands, looking.]]\n\n[[The man walks away.]]\n\n[[There is a tree.]]\n\n[[The man approaches the tree with a present in a box, tied up with ribbon.]]\n\n[[The man sets the present down at the base of the tree and looks up.]]\n\n[[The man walks away.]]\n\n[[The present is sitting at the bottom of the tree.]]\n\n[[The woodpecker looks down at the present.]]\n\n[[The woodpecker sits on the present.]]\n\n[[The woodpecker pulls on the ribbon tying the present closed.]]\n\n((full width panel))\n[[The woodpecker is flying, with an electric drill dangling from its feet, held by the cord.]]\n\n{{Title text: If you don't have an extension cord I can get that too.  Because we're friends!  Right?}}".to_owned(),
        alt: "If you don't have an extension cord I can get that too.  Because we're friends!  Right?".to_owned(),
        img: "https://imgs.xkcd.com/comics/woodpecker.png".to_owned(),
        title: "Woodpecker".to_owned(),
        day: "24".to_owned(),
    }, xkcd::Comic::get(Some(614)).unwrap())
}

#[test]
fn test_neonrogue() {
    neonrogue::Scores::get(neonrogue::ScoreList::AllTime).unwrap();
    neonrogue::Scores::get(neonrogue::ScoreList::PastWeek).unwrap();
    neonrogue::Scores::get(neonrogue::ScoreList::Past30Days).unwrap();
    neonrogue::Scores::get(neonrogue::ScoreList::Past24Hours).unwrap();
}

#[test]
fn test_bbq() {
    breakingbad::Quote::get(None).unwrap();
    breakingbad::Quote::get(Some(10)).unwrap();
}
