fn main() {
    let msg_list = [
        "To die, to sleep, no more, and by a sleep to say we end the heartache, and the thousand natural shocks that flesh is heir to.",
        "To be, or not to be, that is the question: Whether 'tis nobler in the mind to suffer the slings and arrows of outrageous fortune, or to take arms against a sea of troubles, and by opposing end them.",
        "A man of genius makes no mistakes. His errors are volitional and are the portals of discovery.",
        "Le seul véritable voyage, le seul bain de jouvence, ce ne serait pas d'aller vers de nouveaux paysages, mais d'avoir d'autres yeux."
    ];
    for msg in msg_list.iter() {
        println!("{}", msg);
    }
}
