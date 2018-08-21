use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Minute {
    pub title: &'static str,
    pub author: &'static str,
    pub start: &'static str,
    pub time: &'static str,
    pub end: &'static str,
}

pub fn get_minute(time: String) -> Minute {
    let mut minutes: HashMap<&str, &[&[&str]]> = HashMap::new();

    minutes.insert("00:00", &[
&["As ", "midnight", " was striking bronze blows upon the dusky air, Dorian Gray, dressed commonly, and with a muffler wrapped round his throat, crept quietly out of his house.", "Oscar Wilde", "The Picture of Dorian Gray"],
&["\"But wait till I tell you,\" he said. :We had a ", "midnight", " lunch too after all the jollification and when we sallied forth it was blue o'clock the morning after the night before\"", "James Joyce", "Ulysses"],
&["\"", "Midnight", ",\" you said. What is midnight to the young? And suddenly a festive blaze was flung Across five cedar trunks, snow patches showed, And a patrol car on our bumpy road Came to a crunching stop. Retake, retake!", "Vladimir Nabokov", "Pale Fire"],
&["That a man who could hardly see anything more than two feet away from him could be employed as a security guard suggested to me that our job was not to secure anything but to report for work every night, fill the bulky ledger with cryptic remarks like 'Patrolled perimeter ", "12.00 pm", ", No Incident' and go to the office every fortnight for our wages and listen to the talkative Ms Elgassier.", "Ike Oguine", "A Squatter's Tale"],
&["'Tis the year's ", "midnight", ", and it is the day's, Lucy's, who scarce seven hours herself unmasks; The sun is spent, and now his flasks Send forth light squibs, no constant rays;", "John Donne", "A Nocturnal upon St Lucy's Day"],
&["At ", "midnight", " his wife and daughter might still be bustling about, preparing holiday delicacies in the kitchen, straightening up the house, or perhaps getting their kimonos ready or arranging flowers. Oki would sit in the living room and listen to the radio. As the bells rang he would look back at the departing year. He always found it a moving experience.", "Yasunari Kawabata", "Beauty and Sadness"],
&["Bernardo: 'Tis now struck ", "twelve", "; get thee to bed, Francisco.", "Shakespeare", "Hamlet"],
&["Big Ben concluded the run-up, struck and went on striking. (...) But, odder still - Big Ben had once again struck ", "midnight", ". The time outside still corresponded to that registered by the stopped gilt clock, inside. Inside and outside matched exactly, but both were badly wrong. H'm.", "Angela Carter", "Nights At The Circus"],
&["But in the end I understood this language. I understood it, I understood it, all wrong perhaps. That is not what matters. It told me to write the report. Does this mean I am freer now than I was? I do not know. I shall learn. Then I went back into the house and wrote, It is ", "midnight", ". The rain is beating on the windows. It was not midnight. It was not raining.", "Samuel Beckett", "Molloy"],
&["Cartridges not allowed after ", "0000h.", ", to encourage sleep.", "David Foster Wallace", "Infinite Jest"],
&["Francisco. You come most carefully upon your hour. Bernardo. 'Tis now struck ", "twelve", ". Get thee to bed, Francisco.", "William Shakespeare", "Hamlet"],
&["Gately can hear the horns and raised voices and u-turn squeals way down below on Wash. That indicate it's around ", "0000h", "., the switching hour.", "David Foster Wallace", "Infinite Jest"],
&["Hamlet: What hour now? Horatio: I think it lacks of ", "twelve", ". Marcellus: No, it is struck.", "William Shakespeare", "Hamlet"],
&["He is certain he heard footsteps: they come nearer, and then die away. The ray of light beneath his door is extinguished. It is ", "midnight", "; some one has turned out the gas; the last servant has gone to bed, and he must lie all night in agony with no one to bring him any help.", "Marcel Proust", "Swann's Way"],
&["I am conceived to the chimes of ", "midnight", " on the clock on the mantelpiece in the room across the hall. The clock once belonged to my great-grandmother (a woman called Alice) and its tired chime counts me into the world.", "Kate Atkinson", "Behind the Scenes at the Museum"],
&["I took her hand in mine, and bid her be composed; for a succession of shudders convulsed her frame, and she would keep straining her gaze towards the glass. 'There's nobody here!' I insisted. 'It was YOURSELF, Mrs. Linton: you knew it a while since.' 'Myself!' she gasped, 'and the clock is striking ", "twelve", "! It's true, then! that's dreadful!'", "Emily Brontë", "Wuthering Heights"],
&["I was born in the city of Bombay ... On the stroke of ", "midnight", ", as a matter of fact. Clock-hands joined palms in respectful greeting as I came. Oh, spell it out, spell it out: at the precise instant of India's arrival at independence, I tumbled forth into the world.", "Salman Rushdie", "Midnight's Children"],
&["It is ", "midnight", ". The rain is beating on the windows. I am calm. All is sleeping. Nevertheless I get up and go to my desk. I can't sleep. ...", "Samuel Beckett", "Molloy"],
&["It was nearing ", "midnight", " and the Prime Minister was sitting alone in his office, reading a long memo that was slipping through his brain without leaving the slightest trace of meaning behind.", "JK Rowling", "Harry Potter and the Half-Blood Prince"],
&["", "Midnight", " had come upon the crowded city. The palace, the night-cellar, the jail, the madhouse; the chambers of birth and death, of health and sickness; the rigid face of the corpse and the calm sleep of the child - midnight was upon them all.", "Charles Dickens", "Oliver Twist"],
&["", "Midnight", " is approaching, and while the peak of activity has passed, the basal metabolism that maintains life continues undiminished, producing the basso continuo of the city's moan, a monotonous sound that neither rises or falls but is pregnant with foreboding.", "Murakami", "After Dark"],
&["Once upon a ", "midnight", " dreary, while I pondered weak and weary, Over many a quaint and curious volume of forgotten lore, While I nodded, nearly napping, suddenly there came a tapping, As of some one gently rapping, rapping at my chamber door. `'Tis some visitor,' I muttered, `tapping at my chamber door - Only this, and nothing more.'", "Edgar Allan Poe", "The Raven"],
&["The clock striketh ", "twelve", " O it strikes, it strikes! Now body, turn to air, Or Lucifer will bear thee quick to hell. O soul, be changed into little water drops, And fall into the ocean, ne'er to be found. My God, my God, look not so fierce on me!", "Christopher Marlowe", "Dr Faustus"],
&["The first night, as soon as the corporal had conducted my uncle Toby up stairs, which was about 10 - Mrs. Wadman threw herself into her arm chair, and crossing her left knee with her right, which formed a resting-place for her elbow, she reclin'd her cheek upon the palm of her hand, and leaning forwards, ruminated until ", "midnight", " upon both sides of the question.'", "Laurence Sterne", "The Life and Opinions of Tristram Shandy, Gentleman"],
&["To begin my life with the beginning of my life, I record that I was born (as I have been informed an believe) on a Friday, at ", "twelve o'clock", " at night. It was remarked that the clock began to strike, and I began to cry, simultaneously.", "Charles Dickens", "David Copperfield"],
&["We have heard the chimes at ", "midnight", ".", "William Shakespeare", "Henry IV"],
]);
    minutes.insert("00:01", &[
&["With the appointed execution time of ", "one minute past midnight", " just seconds away, I knocked on the metal door twice. The lock turned and the door swiftly swung open.", "Donald A. Cabana", "Death at Midnight"],
]);
    minutes.insert("00:02", &[
&["", "Two minutes past midnight", ". With me in the lead the fourteen other men of Teams Yellow, White and Red moved out of the clearing and separated for points along the wall where they would cross over into the grounds.", "Shashi Warrier", "Night of the Krait"],
]);
    minutes.insert("00:03", &[
&["It was ", "after twelve o'clock", " when Easton came home. Ruth recognised his footsteps before he reached the house, and her heart seemed to stop beating when she heard the clang of the gate, as it closed after he had passed through.", "Robert Tressell", "The Ragged Trousered Philanthropists"],
&["It was just ", "three minutes past midnight", " when I last saw Archer Harrison alive. I remember, because I said it was two minutes past and he looked at his watch and said it was three minutes past.", "George Jean Nathan", "Since Ibsen"],
&["Suddenly I felt a great stillness in the air, then a snapping of tension. I glanced at my watch. ", "Three minutes after midnight.", " I was breathing normally and my pen moved freely across the page. Whatever stalked me wasn’t quite as clever as I’d feared, I thought, careful not to pause in my work.", "Elizabeth Kostova", "The Historian"],
]);
    minutes.insert("00:04", &[
&["At ", "four minutes past midnight", ", January 22, Admiral Lowry's armada of more than 250 ships reached the transport area off Anzio. The sea was calm, the night was black.", "Fred Sheehan", "Anzio: Epic of Bravery"],
]);
    minutes.insert("00:05", &[
&["E.M. Security, normally so scrupulous with their fucking trucks at ", "0005h", "., is nowhere around, lending weight to yet another cliché. If you asked Gately what he was feeling right this second he'd have no idea.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert(
        "00:06",
        &[&[
            "At ",
            "six minutes past midnight",
            ", death relieved the sufferer.",
            "Glenn Shirley",
            "West of Hell's Fringe",
        ]],
    );
    minutes.insert("00:07", &[
&["It was ", "seven minutes after midnight", ". The dog was lying on the grass in the middle of the lawn in front of Mrs Shears' house. Its eyes were closed. It looked as if it was running on its side, the way dogs run when they think they are chasing a cat in a dream.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert(
        "00:08",
        &[&[
            "\"Hour of the night!\" exclaimed the priest; \"it is day, not night, and the hour is ",
            "eight past midnight",
            "!\"",
            "James Pascoe",
            "The Brigantine",
        ]],
    );
    minutes.insert("00:09", &[
&["At ", "12.09am", " on 18 October, the cavalcade had reached the Karsaz Bridge, still ten kilometres from her destination.", "Amir Mir", "The Bhutto Murder Trail: From Waziristan to GHQ"],
]);
    minutes.insert("00:10", &[
&["It was at ", "ten minutes past midnight", ". Three police cars, Alsations and a black maria arrive at the farmhouse. The farmer clad only in a jock-strap, refused them entry.", "Jonathan Barrow", "The Queue"],
]);
    minutes.insert("00:11", &[
&["The first incendiaries to hit St Thomas's Hospital had splattered Riddell House at ", "eleven minutes past midnight", ", from where a few hours earlier the Archbishop of Canterbury had given 'an inspiring address'.", "Gavin Mortimer", "The Longest Night"],
]);
    minutes.insert("00:12", &[
&["Clock time is ", "0 Hours, 12 Minutes", ", 0 Seconds. Twenty three minutes later, they have their first sight of Venus. Each lies with his Eye clapp'd to the Snout of an identical two and a half foot Gregorian reflector made by Mr Short, with Darkening-Nozzles by Mr Bird.", "Thomas Pynchon", "Mason & Dixon"],
&["It was ", "twelve minutes past midnight", " when mother and daughter saw the first lightning strike. It hit the main barn with such force the ground trembled under their feet.", "Fern Michaels", "Kentucky heat"],
]);
    minutes.insert("00:13", &[
&["Clock time is ", "0 Hours, 12 Minutes", ", 0 Seconds. Twenty three minutes later, they have their first sight of Venus. Each lies with his Eye clapp'd to the Snout of an identical two and a half foot Gregorian reflector made by Mr Short, with Darkening-Nozzles by Mr Bird.", "Thomas Pynchon", "Mason & Dixon"],
&["It was ", "twelve minutes past midnight", " when mother and daughter saw the first lightning strike. It hit the main barn with such force the ground trembled under their feet.", "Fern Michaels", "Kentucky heat"],
]);
    minutes.insert("00:14", &[
&["It was exactly ", "fourteen minutes past midnight", " when he completed the final call. Among the men he had reched were honourable men. Their voices would be heard by the President.", "Robert Ludlum", "The Matarese Circle"],
]);
    minutes.insert("00:15", &[
&["At ", "twelve-fifteen", " he got out of the van. He tucked the pistol under the waistband of his trousers and crossed the silent, deserted street to the Hudston house.", "Dean Koontz", "Watchers"],
&["At ", "twelve-fifteen", " he got out of the van. He tucked the pistol under the waistband of his trousers and crossed the silent, deserted street to the Hudston house. He let himself through an unlocked wooden gate onto a side patio brightened only by moonlight filtered through the leafy branches of an enormous sheltering coral tree. He paused to pull on a pair of supple leather gloves.", "Dean Koontz", "Watchers"],
]);
    minutes.insert(
        "00:16",
        &[&[
            "At ",
            "sixteen minutes past midnight",
            ", Block 4 was hit and the roof set alight.",
            "Gavin Mortimer",
            "The Longest Night",
        ]],
    );
    minutes.insert("00:17", &[
&["Kava ordered two glasses of coffee for himself and his beloved and some cake. When the pair left, exactly ", "seventeen minutes after twelve", ", the club began to buzz with excitement.", "Isaac Bashevis Singer", "Vanvild Kava"],
]);
    minutes.insert("00:18", &[
&["21st December 1985, ", "12.18am", " [In bed] Michael doesn’t believe in Heaven or Hell. He’s got closer to death than most living people and he tells me there was no tunnel of light or dancing angels. I’m a bit disappointed, to be honest.", "Mary Horlock", "The Book of Lies"],
]);
    minutes.insert("00:19", &[
&["21st December 1985, ", "12.18am", " [In bed] Michael doesn’t believe in Heaven or Hell. He’s got closer to death than most living people and he tells me there was no tunnel of light or dancing angels. I’m a bit disappointed, to be honest.", "Mary Horlock", "The Book of Lies"],
]);
    minutes.insert("00:20", &[
&["Now she was kneading the little ball of hot paste on the convex margin of the bowl and I could smell the opium. There is no smell like it. Beside the bed the alarm-clock showed ", "twelve-twenty", ", but already my tension was over. Pyle had diminished.", "Graham Greene", "The Quiet American"],
]);
    minutes.insert("00:21", &[
&["Nobody had been one of Mycroft Ward's most important operatives and for sixty seconds every day, between ", "12.21am", " and 12.22am., his laptop was permitted to connect directly with the gigantic online database of self that was Mycroft Ward's mind.", "Steven Hall", "The Raw Shark Texts"],
]);
    minutes.insert("00:22", &[
&["Nobody had been one of Mycroft Ward's most important operatives and for sixty seconds every day, between 12.21am and ", "12.22am.", ", his laptop was permitted to connect directly with the gigantic online database of self that was Mycroft Ward's mind.", "Steven Hall", "The Raw Shark Texts"],
]);
    minutes.insert("00:23", &[
&["Oskar weighed the wristwatch in his hand, then gave the rather fine piece with its luminous dial showing ", "twenty-three minutes past midnight", " to little Pinchcoal. He looked up inquiringly at his chief. Störtebeker nodded his assent. And Oskar said, as he adjusted his drum snugly for the trip home: 'Jesus will lead the way. Follow thou me!'", "Günter Grass", "The Tin Drum"],
]);
    minutes.insert(
        "00:24",
        &[&[
            "Sanders with Sutton as his gunner began their patrol at ",
            "12.24am",
            ", turning south towards Beachy Head at 10,000 ft.",
            "Gavin Mortimer",
            "The Longest Night",
        ]],
    );
    minutes.insert("00:25", &[
&["Charlotte remembered that she had heard Gregoire go downstairs again, almost immediately after entering his bedroom, and before the servants had even bolted the house-doors for the night. He had certainly rushed off to join Therese in some coppice, whence they must have hurried away to Vieux-Bourg station which the last train to Paris quitted at ", "five-and-twenty minutes past midnight", ". And it was indeed this which had taken place.", "Emile Zola", "Fruitfulness"],
&["I mean, look at the time! ", "Twenty-five past midnight", "! It was a triumph, it really was!", "Joanna Trollope", "The Soldier's Wife"],
]);
    minutes.insert("00:26", &[
&["\"A Mr Dutta from King's Cross called and told me you were on your way. He said you wanted to see the arrival of yesterday's ", "12.26am.", " It'll take me a few minutes to cue up the footage. Our regular security bloke isn't here today; he's up before Haringey Magistrates' Court for gross indecency outside the headquarters of the Dagenham Girl Pipers.\"", "Christopher Fowler", "Bryant & May Off the Rails"],
]);
    minutes.insert("00:27", &[
&["\"A Mr Dutta from King's Cross called and told me you were on your way. He said you wanted to see the arrival of yesterday's ", "12.26am.", " It'll take me a few minutes to cue up the footage. Our regular security bloke isn't here today; he's up before Haringey Magistrates' Court for gross indecency outside the headquarters of the Dagenham Girl Pipers.\"", "Christopher Fowler", "Bryant & May Off the Rails"],
]);
    minutes.insert(
        "00:28",
        &[&[
            "The DRINK CHEER-UP COFFEE wall clock read ",
            "12.28",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert("00:29", &[
&["\"What time is it?\" asked Teeny-bits. The station agent hauled out his big silver watch, looked at it critically and announced: \"", "Twenty-nine minutes past twelve", ".” “Past twelve!\" repeated Teeny-bits. \"It can't be.\"", "Clayton H Ernst", "The Mark of the Knife"],
]);
    minutes.insert("00:30", &[
&["It was ", "half-past twelve", " when I returned to the Albany as a last desperate resort. The scene of my disaster was much as I had left it. The baccarat-counters still strewed the table, with the empty glasses and the loaded ash-trays. A window had been opened to let the smoke out, and was letting in the fog instead.", "E.W. Hornung", "The Amateur Cracksman"],
]);
    minutes.insert("00:31", &[
&["Third individual approaches unnoticed and without caution. Once within reach, individual reaches out toward subjects. Recording terminates: timecode: ", "00:31", ":02.", "Stephen Jones", "Zombie Apocalypse! Fightback"],
]);
    minutes.insert("00:32", &[
&["", "Thirty-two minutes past midnight", "; the way things were going I could be at it all night. Before beginning a completely new search of the dial I had a thought: maybe this safe didn't open on zero as older models did, but on a factory-set number.", "Everette Howard Hunt", "Ixtapa"],
]);
    minutes.insert("00:33", &[
&["\"So that at twelve-thirty-three you bolted the south door?\" \"Yes,\" replied Stephen Maxie easily. \"At ", "thirty-three minutes past midnight", ".\"", "P.D. James", "Cover her Face"],
]);
    minutes.insert("00:34", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert("00:35", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert("00:36", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert("00:37", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert("00:38", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert("00:39", &[
&["", "Thirty-four minutes past midnight", ". 'We got ten minutes to be back here.' LT didn't argue. Schoolboy knew his former trade. LT's eyes fretted over the museum. 'Not still worrying about the security, are you, because there ain't none.'", "Dreda Say Mitchell", "Killer Tune"],
]);
    minutes.insert(
        "00:40",
        &[&[
            "We sat in the car park till ",
            "twenty to one",
            "/ And now I'm engaged to Miss Joan Hunter Dunn.",
            "John Betjeman",
            "A Subaltern's Love Song",
        ]],
    );
    minutes.insert(
        "00:41",
        &[&[
            "We sat in the car park till ",
            "twenty to one",
            "/ And now I'm engaged to Miss Joan Hunter Dunn.",
            "John Betjeman",
            "A Subaltern's Love Song",
        ]],
    );
    minutes.insert("00:42", &[
&["The butt had been growing warm in her fingers; now the glowing end stung her skin. She crushed the cigarette out and stood, brushing ash from her black skirt. It was ", "eighteen minutes to one", ". She went to the house phone and called his room. The telephone rang and rang, but there was no answer.", "Herman Wouk", "Marjorie Morningstar"],
]);
    minutes.insert(
        "00:43",
        &[&[
            "Died five minutes ago, you say? he asked. His eye went to the watch on his wrist. ",
            "Twelve-forty-three",
            ", he wrote on the blotter.",
            "Agatha Christie",
            "A Pocket Full of Rye",
        ]],
    );
    minutes.insert(
        "00:44",
        &[&[
            "Died five minutes ago, you say? he asked. His eye went to the watch on his wrist. ",
            "Twelve-forty-three",
            ", he wrote on the blotter.",
            "Agatha Christie",
            "A Pocket Full of Rye",
        ]],
    );
    minutes.insert("00:45", &[
&["At ", "12.45", ", during a lull, Mr Yoshogi told me that owing to the war there were now many more women in England than men.", "David Footman", "Pig and Pepper: A Comedy of Youth"],
&["At the thought he jumped to his feet and took down from its hook the coat in which he had left Miss Viner's letter. The clock marked the ", "third quarter after midnight", ", and he knew it would make no difference if he went down to the post-box now or early the next morning; but he wanted to clear his conscience, and having found the letter he went to the door.", "Edith Wharton", "The Reef"],
]);
    minutes.insert("00:46", &[
&["At ", "12.45", ", during a lull, Mr Yoshogi told me that owing to the war there were now many more women in England than men.", "David Footman", "Pig and Pepper: A Comedy of Youth"],
&["At the thought he jumped to his feet and took down from its hook the coat in which he had left Miss Viner's letter. The clock marked the ", "third quarter after midnight", ", and he knew it would make no difference if he went down to the post-box now or early the next morning; but he wanted to clear his conscience, and having found the letter he went to the door.", "Edith Wharton", "The Reef"],
]);
    minutes.insert(
        "00:47",
        &[&[
            "At ",
            "12:47a.m",
            ", Uncle Ho left us forever.",
            "Andrew X. Pham",
            "Last Night I Dreamed Of Peace",
        ]],
    );
    minutes.insert(
        "00:48",
        &[&[
            "At ",
            "12:47a.m",
            ", Uncle Ho left us forever.",
            "Andrew X. Pham",
            "Last Night I Dreamed Of Peace",
        ]],
    );
    minutes.insert(
        "00:49",
        &[&[
            "At ",
            "12:47a.m",
            ", Uncle Ho left us forever.",
            "Andrew X. Pham",
            "Last Night I Dreamed Of Peace",
        ]],
    );
    minutes.insert("00:50", &[
&["The packing was done at ", "12.50", "; and Harris sat on the big hamper, and said he hoped nothing would be found broken. George said that if anything was broken it was broken, which reflection seemed to comfort him. He also said he was ready for bed.", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert("00:51", &[
&["The packing was done at ", "12.50", "; and Harris sat on the big hamper, and said he hoped nothing would be found broken. George said that if anything was broken it was broken, which reflection seemed to comfort him. He also said he was ready for bed.", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert("00:52", &[
&["The packing was done at ", "12.50", "; and Harris sat on the big hamper, and said he hoped nothing would be found broken. George said that if anything was broken it was broken, which reflection seemed to comfort him. He also said he was ready for bed.", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert("00:53", &[
&["The packing was done at ", "12.50", "; and Harris sat on the big hamper, and said he hoped nothing would be found broken. George said that if anything was broken it was broken, which reflection seemed to comfort him. He also said he was ready for bed.", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert("00:54", &[
&["Everybody was happy; everybody was complimentary; the ice was soon broken; songs, anecdotes, and more drinks followed, and the pregnant minutes flew. At ", "six minutes to one", ", when the jollity was at its highest— BOOM! There was silence instantly.", "Mark Twain", "A Double Barrelled Detective Story"],
]);
    minutes.insert("00:55", &[
&["He rolled one way, rolled the other, listened to the loud tick of the clock, and was asleep a minute later. ", "Five to one", " in the morning. Fifty-one hours to go.", "Lee Child", "61 Hours"],
]);
    minutes.insert("00:56", &[
&["It was ", "12:56 A.M.", " when Gerald drove up onto the grass and pulled the limousine right next to the cemetery.", "Jonathan Safran Foer", "Extremely Loud and Incredibly Close"],
&["Teacher used to lie awake at night facing that clock, batting his eyelashes against his pillowcase to mimic the sound of the rolling drop action. One night, and this first night is lost in the countless later nights of compounding wonder, he discovered a game. Say the time was ", "12:56", ".", "Dana Standridge", "Lessons in Essence"],
]);
    minutes.insert("00:57", &[
&["A minute had passed, and the roller dropped a new leaf. ", "12:57", ". 12 + 57 = 69; 6 + 9 = 15; 1 + 5 = 6. 712 + 5 = 717; 71 + 7 = 78; 7 + 8 = 15; 1 + 5 = 6 again.", "Dana Standridge", "Lessons in Essence"],
]);
    minutes.insert(
        "00:58",
        &[&[
            "It was downright shameless on his part to come visiting them, especially at night, ",
            "almost at one in the morning",
            ", after all that had happened.",
            "Fyodor Dostoyevsky",
            "The Idiot",
        ]],
    );
    minutes.insert("00:59", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:00", &[
&["", "1.00 am.", " I felt the surrounding quietness suffocating me.", "Rosamund Lupton", "Sister"],
&["He didn’t know what was at the end of the chute. The opening was narrow (though large enough to take the canary). He dreamed that the chute opened onto vast garbage bins filled with old coffee filters, ravioli in tomato sauce and mangled genitalia. Huge worms, as big as the canary, armed with terrible beaks, would attack the body. Tear off the feet, rip out its intestines, burst the eyeballs. He woke up, trembling; it was only ", "one o’clock", ". He swallowed three Xanax. So ended his first night of freedom.", "Michel Houellebecq", "Atomised"],
&["I looked attentively at her, as she put that singular question to me. It was then ", "nearly one o'clock", ". All I could discern distinctly by the moonlight was a colourless, youthful face, meagre and sharp to look at about the cheeks and chin; large, grave, wistfully attentive eyes; nervous, uncertain lips; and light hair of a pale, brownish-yellow hue.", "Wilkie Collins", "The Woman in White"],
&["I'm the only one awake in this house on this night before the day that will change all our lives. Though it's already that day: the little luminous hands on my alarm clock (which I haven't set) show just gone ", "one in the morning", ".", "Graham Swift", "Tomorrow"],
&["It was the thirtieth of May by now. ", "One am", " on the thirtieth of May 1940. Quite a famous date on which to be lying awake and staring at the ceiling. Already in the creeks and tidal estuaries of England the pleasure-boats and paddle-steamers were casting their moorings for the day trip to Dunkirk. And, over on the other side, Ted stood as a good a chance as anyone else.", "Norman Collins", "London Belongs to Me"],
&["Last night of all, When yon same star that's westward from the pole Had made his course t'illume that part of heaven Where now it burns, Marcellus and myself, The bell then beating ", "one", " -", "William Shakespeare", "Hamlet"],
&["The station was more crowded than he had expected to find it at - what was it? he looked up at the clock - ", "one o'clock in the morning", ". What in the name of God was he doing on King's Cross station at one o'clock in the morning, with no cigarette and no home that he could reasonably expect to get into without being hacked to death by a homicidal bird?", "Douglas Adams", "The Long Dark Tea-time of the Soul"],
]);
    minutes.insert("01:01", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:02", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:03", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:04", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:05", &[
&["‘What time is it now?’ she said. ‘", "About one o’clock", "’. ‘In the morning?’ Herera’s friend leered at her. ‘No, there’s a total eclipse of the sun’.", "Jonathan Frantzen", "Freedom"],
]);
    minutes.insert("01:06", &[
&["When he woke it was ", "1:06", " by the digital clock on the bedside table. He lay there looking at the ceiling, the raw glare of the vaporlamp outside bathing the bedroom in a cold and bluish light. Like a winter moon.", "Cormac McCarthy", "No Country for Old Men"],
]);
    minutes.insert("01:07", &[
&["When he woke it was ", "1:06", " by the digital clock on the bedside table. He lay there looking at the ceiling, the raw glare of the vaporlamp outside bathing the bedroom in a cold and bluish light. Like a winter moon.", "Cormac McCarthy", "No Country for Old Men"],
]);
    minutes.insert(
        "01:08",
        &[&[
            "It was ",
            "1.08a.m.",
            " but he had left the ball at the same time as I did, and had further to travel.",
            "Graeme Simsion",
            "The Rosie Project",
        ]],
    );
    minutes.insert("01:09", &[
&["They made an unostentatious exit from their coach, finding themselves, when the express had rolled on into the west, upon a station platform in a foreign city at ", "nine minutes past one", " o'clock in the morning - but at length without their shadow.", "Louis Joseph Vance", "The Black Bag"],
]);
    minutes.insert("01:10", &[
&["February 26, Saturday - Richards went out ", "1.10am", " and found it clearing a bit, so we got under way as soon as possible, which was 2:10a.m.", "Ernest Shackleton", "South: The Endurance Expedition"],
]);
    minutes.insert("01:11", &[
&["Declares one of the waiters was the worse for liquor, and that he was giving him a dressing down. Also that it was ", "nearer to one than half past", ".", "Agatha Christie", "The Affair at the Victory Ball"],
]);
    minutes.insert("01:12", &[
&["It was ", "1:12am", " when Father arrived at the police station. I did not see him until 1:28am but I knew he was there because I could hear him. He was shouting, 'I want to see my son,' and 'Why the hell is he locked up?' and, 'Of course I'm bloody angry.'", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert("01:13", &[
&["It was ", "1:12am", " when Father arrived at the police station. I did not see him until 1:28am but I knew he was there because I could hear him. He was shouting, 'I want to see my son,' and 'Why the hell is he locked up?' and, 'Of course I'm bloody angry.'", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert("01:14", &[
&["It was ", "1:12am", " when Father arrived at the police station. I did not see him until 1:28am but I knew he was there because I could hear him. He was shouting, 'I want to see my son,' and 'Why the hell is he locked up?' and, 'Of course I'm bloody angry.'", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert("01:15", &[
&["I am sorry, therefore, as I have said, that I ever paid any attention to the footsteps. They began about a ", "quarter past one", " o'clock in the morning, a rhythmic, quick-cadenced walking around the dining-room table.", "James Thurber", "My Life and Hard Times: \"The Night the Ghost Got In\""],
&["Lily Chen always prepared an 'evening' snack for her husband to consume on his return at ", "1.15am.", "", "Timothy Mo", "Sour Sweet"],
&["The ghost that got into our house on the night of November 17, 1915, raised such a hullabaloo of misunderstandings that I am sorry I didn't just let it keep on walking, and go to bed. Its advent caused my mother to throw a shoe through a window of the house next door and ended up with my grandfather shooting a patrolman. I am sorry, therefore, as I have said, that I ever paid any attention to the footsteps. They began about a ", "quarter past one", " o'clock in the morning, a rhythmic, quick-cadenced walking around the dining-room table.", "James Thurber", "My Life and Hard Times: \"The Night the Ghost Got In\""],
]);
    minutes.insert(
        "01:16",
        &[
            &[
                "At ",
                "sixteen past one",
                ", they walked into the interview room.",
                "Dana Stabenow",
                "Nothing Gold Can Stay",
            ],
            &[
                "From 1am to ",
                "1.16am",
                " vouched for by other two conductors.",
                "Agatha Christie",
                "Murder on the Orient Express",
            ],
        ],
    );
    minutes.insert("01:17", &[
&["At that moment (it was ", "seventeen minutes past one", " in the morning) Lieutenant Bronsfield was preparing to leave the watch and return to his cabin, when his attention was attracted by a distant hissing noise.", "Jules Verne", "A voyage round the moon"],
&["The clocks stopped at ", "1:17", ". A long shear of light and then a series of low concussions. He got up and went to the window. What is it? she said. He didnt answer. He went into the bathroom and threw the lightswitch but the power was already gone. A dull rose glow in the windowglass. He dropped to one knee and raised the lever to stop the tub and then turned on both taps as far as they would go. She was standing in the doorway in her nightwear, clutching the jamb, cradling her belly in one hand. What is it? she said. What is happening?", "Cormac McCarthy", "The Road"],
]);
    minutes.insert("01:18", &[
&["At that moment (it was ", "seventeen minutes past one", " in the morning) Lieutenant Bronsfield was preparing to leave the watch and return to his cabin, when his attention was attracted by a distant hissing noise.", "Jules Verne", "A voyage round the moon"],
&["The clocks stopped at ", "1:17", ". A long shear of light and then a series of low concussions. He got up and went to the window. What is it? she said. He didnt answer. He went into the bathroom and threw the lightswitch but the power was already gone. A dull rose glow in the windowglass. He dropped to one knee and raised the lever to stop the tub and then turned on both taps as far as they would go. She was standing in the doorway in her nightwear, clutching the jamb, cradling her belly in one hand. What is it? she said. What is happening?", "Cormac McCarthy", "The Road"],
]);
    minutes.insert("01:19", &[
&["At that moment (it was ", "seventeen minutes past one", " in the morning) Lieutenant Bronsfield was preparing to leave the watch and return to his cabin, when his attention was attracted by a distant hissing noise.", "Jules Verne", "A voyage round the moon"],
&["The clocks stopped at ", "1:17", ". A long shear of light and then a series of low concussions. He got up and went to the window. What is it? she said. He didnt answer. He went into the bathroom and threw the lightswitch but the power was already gone. A dull rose glow in the windowglass. He dropped to one knee and raised the lever to stop the tub and then turned on both taps as far as they would go. She was standing in the doorway in her nightwear, clutching the jamb, cradling her belly in one hand. What is it? she said. What is happening?", "Cormac McCarthy", "The Road"],
]);
    minutes.insert("01:20", &[
&["\"Well!\" she said, looking like a minor female prophet about to curse the sins of the people. \"May I trespass on your valuable time long enough to ask what in the name of everything bloodsome you think you're playing at, young piefaced Bertie? It is now some ", "twenty minutes past one", " o'clock in the morning, and not a spot of action on your part.\"", "P.G. Wodehouse", "Jeeves and the Feudal Spirit"],
&["Then it was ", "1.20am", ", but I hadn't heard Father come upstairs to bed. I wondered if he was asleep downstairs or whether he was waiting to come in and kill me. So I got out my Swiss Army Knife and opened the saw blade so that I could defend myself.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("01:21", &[
&["\"Well!\" she said, looking like a minor female prophet about to curse the sins of the people. \"May I trespass on your valuable time long enough to ask what in the name of everything bloodsome you think you're playing at, young piefaced Bertie? It is now some ", "twenty minutes past one", " o'clock in the morning, and not a spot of action on your part.\"", "P.G. Wodehouse", "Jeeves and the Feudal Spirit"],
&["Then it was ", "1.20am", ", but I hadn't heard Father come upstairs to bed. I wondered if he was asleep downstairs or whether he was waiting to come in and kill me. So I got out my Swiss Army Knife and opened the saw blade so that I could defend myself.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert(
        "01:22",
        &[&[
            "It was ",
            "1:22",
            " when we found Dad's grave.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert("01:23", &[
&["The clock marked ", "twenty-three minutes past one", ". He was suddenly full of agitation, yet hopeful. She had come! Who could tell what she would say? She might offer the most natural explanation of her late arrival.", "Anatole France", "A Mummer's Tale"],
]);
    minutes.insert("01:24", &[
&["Larkin had died at ", "1.24am", ", turning to the nurse who was with him, squeezing her hand, and saying faintly, 'I am going to the inevitable.'", "Hermione Lee", "Body Parts: Essays on Life-Writing"],
]);
    minutes.insert("01:25", &[
&["He made a last effort; he tried to rise, and sank back. His head fell on the sofa cushions. It was then ", "twenty-five minutes past one", " o'clock.", "Wilkie Collins", "The Moonstone"],
]);
    minutes.insert(
        "01:26",
        &[&[
            "When I reached the stop and got off, it was already ",
            "one twenty-six A.M.",
            " by the bus's own clock. I had been gone over ten hours.",
            "Tanith Lee",
            "The Silver Metal Lover",
        ]],
    );
    minutes.insert(
        "01:27",
        &[&[
            "At ",
            "twenty-seven minutes past one",
            " she felt as if she was levitating out of her body.",
            "Deon Meyer",
            "Trackers",
        ]],
    );
    minutes.insert("01:28", &[
&["It was 1:12 am when Father arrived at the police station. I did not see him until ", "1:28 am", " but I knew he was there because I could hear him. He was shouting, 'I want to see my son,' and 'Why the hell is he locked up?' and, 'Of course I'm bloody angry.'", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert(
        "01:29",
        &[&[
            "He exited the men's room at ",
            "one-twenty-nine A.M.",
            "",
            "William Edmund Butterworth",
            "The Narc",
        ]],
    );
    minutes.insert("01:30", &[
&["\"", "Half-past one", "”, The street lamp sputtered, The street lamp muttered, The street lamp said, \"Regard that woman ...\"", "TS Eliot", "Rhapsody on a Windy Night"],
&["Around ", "1:30 A.M.", " the door opened and I thought it was Karla, but it was Bug, saying Karla and Laura had gone out for a stag night after they ran out of paint.", "Douglas Coupland", "Microserfs"],
&["The late hour helped. It simplified things. It categorized the population. Innocent bystanders were mostly home in bed. I walked for half an hour, but nothing happened. Until ", "one thirty", " in the morning. Until I looped around to 22nd and Broadway.", "Lee Child", "Gone Tomorrow"],
&["The radio alarm clock glowed ", "1:30 a.m.", " Bad karaoke throbbed through walls. I was wide awake, straightjacketed by my sweaty sheets. A headache dug its thumbs into my temples. My gut pulsed with gamma interference: I lurched to the toilet.", "David Mitchell", "Ghostwritten"],
]);
    minutes.insert("01:31", &[
&["\"", "Half-past one", "”, The street lamp sputtered, The street lamp muttered, The street lamp said, \"Regard that woman ...\"", "TS Eliot", "Rhapsody on a Windy Night"],
&["Around ", "1:30 A.M.", " the door opened and I thought it was Karla, but it was Bug, saying Karla and Laura had gone out for a stag night after they ran out of paint.", "Douglas Coupland", "Microserfs"],
&["The late hour helped. It simplified things. It categorized the population. Innocent bystanders were mostly home in bed. I walked for half an hour, but nothing happened. Until ", "one thirty", " in the morning. Until I looped around to 22nd and Broadway.", "Lee Child", "Gone Tomorrow"],
&["The radio alarm clock glowed ", "1:30 a.m.", " Bad karaoke throbbed through walls. I was wide awake, straightjacketed by my sweaty sheets. A headache dug its thumbs into my temples. My gut pulsed with gamma interference: I lurched to the toilet.", "David Mitchell", "Ghostwritten"],
]);
    minutes.insert("01:32", &[
&["She grinned at him with malicious playfulness, showing great square teeth, and then ran for the stairs. ", "One-thirty-two", ". She thought that she heard a whistle blown and took the last three steps in one stride.", "Graham Greene", "Stamboul Train"],
]);
    minutes.insert(
        "01:33",
        &[&[
            "He looked at his watch. ",
            "One-thirty-three a.m.",
            " He had been asleep on this bench for over an hour and a half.",
            "Kat Fox",
            "Skeletons",
        ]],
    );
    minutes.insert(
        "01:34",
        &[&[
            "He looked at his watch. ",
            "One-thirty-three a.m.",
            " He had been asleep on this bench for over an hour and a half.",
            "Kat Fox",
            "Skeletons",
        ]],
    );
    minutes.insert(
        "01:35",
        &[&[
            "He looked at his watch. ",
            "One-thirty-three a.m.",
            " He had been asleep on this bench for over an hour and a half.",
            "Kat Fox",
            "Skeletons",
        ]],
    );
    minutes.insert(
        "01:36",
        &[&[
            "He looked at his watch. ",
            "One-thirty-three a.m.",
            " He had been asleep on this bench for over an hour and a half.",
            "Kat Fox",
            "Skeletons",
        ]],
    );
    minutes.insert(
        "01:37",
        &[&[
            "He looked at his watch. ",
            "One-thirty-three a.m.",
            " He had been asleep on this bench for over an hour and a half.",
            "Kat Fox",
            "Skeletons",
        ]],
    );
    minutes.insert("01:38", &[
&["At ", "one-thirty-eight", " am suspect left the Drive-In and drove to seven hundred and twenty three North Walnut, to the rear of the residence, and parked the car.", "William Edmund Butterworth", "The Narc"],
]);
    minutes.insert("01:39", &[
&["At ", "one-thirty-eight", " am suspect left the Drive-In and drove to seven hundred and twenty three North Walnut, to the rear of the residence, and parked the car.", "William Edmund Butterworth", "The Narc"],
]);
    minutes.insert(
        "01:40",
        &[&[
            "March twelfth, ",
            "one-forty am",
            ", she leaves a group of drinking buddies to catch a bus home. She never makes it.",
            "Kathy Reichs",
            "Bones to Ashes",
        ]],
    );
    minutes.insert(
        "01:41",
        &[&[
            "March twelfth, ",
            "one-forty am",
            ", she leaves a group of drinking buddies to catch a bus home. She never makes it.",
            "Kathy Reichs",
            "Bones to Ashes",
        ]],
    );
    minutes.insert(
        "01:42",
        &[&[
            "March twelfth, ",
            "one-forty am",
            ", she leaves a group of drinking buddies to catch a bus home. She never makes it.",
            "Kathy Reichs",
            "Bones to Ashes",
        ]],
    );
    minutes.insert(
        "01:43",
        &[&[
            "March twelfth, ",
            "one-forty am",
            ", she leaves a group of drinking buddies to catch a bus home. She never makes it.",
            "Kathy Reichs",
            "Bones to Ashes",
        ]],
    );
    minutes.insert("01:44", &[
&["She knew it was the stress, two long days of stress, and she looked at her watch, ", "sixteen minutes to two", ", and she almost leaped with fright, a shock wave rippling through her body, where had the time gone?", "Deon Meyer", "Trackers"],
]);
    minutes.insert("01:45", &[
&["She knew it was the stress, two long days of stress, and she looked at her watch, ", "sixteen minutes to two", ", and she almost leaped with fright, a shock wave rippling through her body, where had the time gone?", "Deon Meyer", "Trackers"],
]);
    minutes.insert("01:46", &[
&["That particular phenomenom got Presto up at ", "one forty-six a.m.", "; silently, he painted his face and naked body with camouflage paint. He opened the door to his room and stepped out into the common lobby.", "J.W. Stockton", "Fardnock's Revenge"],
]);
    minutes.insert("01:47", &[
&["That particular phenomenom got Presto up at ", "one forty-six a.m.", "; silently, he painted his face and naked body with camouflage paint. He opened the door to his room and stepped out into the common lobby.", "J.W. Stockton", "Fardnock's Revenge"],
]);
    minutes.insert("01:48", &[
&["That particular phenomenom got Presto up at ", "one forty-six a.m.", "; silently, he painted his face and naked body with camouflage paint. He opened the door to his room and stepped out into the common lobby.", "J.W. Stockton", "Fardnock's Revenge"],
]);
    minutes.insert("01:49", &[
&["That particular phenomenom got Presto up at ", "one forty-six a.m.", "; silently, he painted his face and naked body with camouflage paint. He opened the door to his room and stepped out into the common lobby.", "J.W. Stockton", "Fardnock's Revenge"],
]);
    minutes.insert("01:50", &[
&["No, she thought: every spinster legal secretary, bartender, and orthodontist had a cat or two—and she could not tolerate (not even as a lark, not even for a moment at ", "ten minutes before two AM", "), embodying cliché.", "Michelle Herman", "Dog"],
]);
    minutes.insert("01:51", &[
&["At ", "nine minutes to two", " the other vehicle arrived. At first Milla didn't believe her eyes: that shape, those markings.", "Deon Meyer", "Trackers"],
]);
    minutes.insert("01:52", &[
&["At ", "nine minutes to two", " the other vehicle arrived. At first Milla didn't believe her eyes: that shape, those markings.", "Deon Meyer", "Trackers"],
]);
    minutes.insert("01:53", &[
&["At ", "nine minutes to two", " the other vehicle arrived. At first Milla didn't believe her eyes: that shape, those markings.", "Deon Meyer", "Trackers"],
]);
    minutes.insert("01:54", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("01:55", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("01:56", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("01:57", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("01:58", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("01:59", &[
&["", "Six minutes to two", ". Janina Mentz watched the screen, where the small program window flickered with files scrolling too fast to read, searching for the keyword.", "Dean Koontz", "Trackers"],
]);
    minutes.insert("02:00", &[
&["\"The middle of the night?\" Alec asked sharply.\"Can you be more definite?\" \"", "About two", ". Just past.\" Daisy noted that he expressed no concern for her safety.", "Carola Dunn", "Dead in the water"],
&["As ", "two o'clock", " pealed from the cathedral bell, Jean Valjean awoke.", "Victor Hugo", "Les Miserables"],
&["Get on plane at ", "2 A.M.", ", amid bundles, chickens, gypsies, sit opposite pair of plump fortune tellers who groan and (very discreetly) throw up all the way to Tbilisi.", "J. Updike", "Bech: A Book"],
&["Lady Macbeth: Out, damned spot! out, I say!—One: ", "two", ": why, then, 'tis time to do't.—Hell is murky!—Fie, my lord, fie! a soldier, and afeard? What need we fear who knows it, when none can call our power to account?—Yet who would have thought the old man to have had so much blood in him.", "Shakespeare", "Macbeth"],
&["Somewhere behind a screen a clock began wheezing, as though oppressed by something, as though someone were strangling it. After an unnaturally prolonged wheezing there followed a shrill, nasty, and as it were unexpectedly rapid, chime - as though someone were suddenly jumping forward. ", "It struck two.", " I woke up, though I had indeed not been asleep but lying half-conscious.", "Fyodor Dostoyevsky", "Notes from the underground"],
&["When all had grown quiet and Fyodor Pavlovich went to bed at around ", "two o'clock", ", Ivan Fyodorovich also went to bed with the firm resolve of falling quickly asleep, as he felt horribly exhausted.'", "Fyodor Dostoyevsky", "The Brothers Karamazov"],
]);
    minutes.insert("02:01", &[
&["I checked my watch. ", "2.01am.", " The cheeseburger Happy Meal was now only a distant memory. I cursed myself for not also ordering a breakfast sandwich for the morning.", "Reif Larsen", "The Selected Works of T.S. Spivet"],
]);
    minutes.insert("02:02", &[
&["\"Wake up.\" \"Having the worst dream.\" \"I should certainly say you were.\" \"It was awful. It just went on and on.\" \"I shook you and shook you and.\" \"Time is it.\" \"It's nearly - ", "almost 2:04", ".”", "David Foster Wallace", "Oblivion"],
]);
    minutes.insert("02:03", &[
&["\"Wake up.\" \"Having the worst dream.\" \"I should certainly say you were.\" \"It was awful. It just went on and on.\" \"I shook you and shook you and.\" \"Time is it.\" \"It's nearly - ", "almost 2:04", ".”", "David Foster Wallace", "Oblivion"],
]);
    minutes.insert("02:04", &[
&["\"Wake up.\" \"Having the worst dream.\" \"I should certainly say you were.\" \"It was awful. It just went on and on.\" \"I shook you and shook you and.\" \"Time is it.\" \"It's nearly - almost ", "2:04", ".”", "David Foster Wallace", "Oblivion"],
]);
    minutes.insert("02:05", &[
&["At ", "2.05", " the fizzy tights came crackling off.", "Martin Amis", "London Fields"],
&["Then he began ringing the bell. In about ten minutes his valet appeared, half dressed, and looking very drowsy. ‘I am sorry to have had to wake you up, Francis,’ he said, stepping in; ‘but I had forgotten my latch-key. What time is it?’ ‘", "Five minutes past two", ", sir,’ answered the man, looking at the clock and yawning. ‘Five minutes past two? How horribly late! You must wake me at nine to-morrow. I have some work to do.’", "Oscar Wilde", "The Picture of Dorian Gray"],
]);
    minutes.insert("02:06", &[
&["At ", "2.05", " the fizzy tights came crackling off.", "Martin Amis", "London Fields"],
&["Then he began ringing the bell. In about ten minutes his valet appeared, half dressed, and looking very drowsy. ‘I am sorry to have had to wake you up, Francis,’ he said, stepping in; ‘but I had forgotten my latch-key. What time is it?’ ‘", "Five minutes past two", ", sir,’ answered the man, looking at the clock and yawning. ‘Five minutes past two? How horribly late! You must wake me at nine to-morrow. I have some work to do.’", "Oscar Wilde", "The Picture of Dorian Gray"],
]);
    minutes.insert("02:07", &[
&["At ", "2:07 a.m.", " I decided that I wanted a drink of orange squash before I brushed my teeth and got into bed, so I went downstairs to the kitchen. Father was sitting on the sofa watching snooker on the television and drinking whisky. There were tears coming out of his eyes.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["But I couldn't sleep. And I got out of bed at ", "2.07 am", " and I felt scared of Mr. Shears so I went downstairs and out of the front door into Chapter Road.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Saturday, 17 November — ", "2.07 a.m.", " I cannot sleep. Ben is upstairs, back in bed, and I am writing this in the kitchen. He thinks I am drinking a cup of cocoa that he has just made for me. He thinks I will come back to bed soon. I will, but first I must write again.", "S. J. Watson", "Before I Go to Sleep"],
]);
    minutes.insert("02:08", &[
&["At ", "2:07 a.m.", " I decided that I wanted a drink of orange squash before I brushed my teeth and got into bed, so I went downstairs to the kitchen. Father was sitting on the sofa watching snooker on the television and drinking whisky. There were tears coming out of his eyes.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["But I couldn't sleep. And I got out of bed at ", "2.07 am", " and I felt scared of Mr. Shears so I went downstairs and out of the front door into Chapter Road.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Saturday, 17 November — ", "2.07 a.m.", " I cannot sleep. Ben is upstairs, back in bed, and I am writing this in the kitchen. He thinks I am drinking a cup of cocoa that he has just made for me. He thinks I will come back to bed soon. I will, but first I must write again.", "S. J. Watson", "Before I Go to Sleep"],
]);
    minutes.insert("02:09", &[
&["At ", "2:07 a.m.", " I decided that I wanted a drink of orange squash before I brushed my teeth and got into bed, so I went downstairs to the kitchen. Father was sitting on the sofa watching snooker on the television and drinking whisky. There were tears coming out of his eyes.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["But I couldn't sleep. And I got out of bed at ", "2.07 am", " and I felt scared of Mr. Shears so I went downstairs and out of the front door into Chapter Road.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Saturday, 17 November — ", "2.07 a.m.", " I cannot sleep. Ben is upstairs, back in bed, and I am writing this in the kitchen. He thinks I am drinking a cup of cocoa that he has just made for me. He thinks I will come back to bed soon. I will, but first I must write again.", "S. J. Watson", "Before I Go to Sleep"],
]);
    minutes.insert("02:10", &[
&["“", "Ten minutes past two", ", sir,\" answered the man, looking at the clock and blinking. \"Ten minutes past two? How horribly late! ..”", "Oscar Wilde", "The Picture of Dorian Gray"],
&["Decided to get under way again as soon as there is any clearance. Snowing and blowing, force about fifty or sixty miles an hour. February 26, Saturday - Richards went out 1:10am and found it clearing a bit, so we got under way as soon as possible, which was ", "2:10am", "", "Ernest Shackleton", "South: The Endurance Expedition"],
]);
    minutes.insert("02:11", &[
&["“", "Ten minutes past two", ", sir,\" answered the man, looking at the clock and blinking. \"Ten minutes past two? How horribly late! ..”", "Oscar Wilde", "The Picture of Dorian Gray"],
&["Decided to get under way again as soon as there is any clearance. Snowing and blowing, force about fifty or sixty miles an hour. February 26, Saturday - Richards went out 1:10am and found it clearing a bit, so we got under way as soon as possible, which was ", "2:10am", "", "Ernest Shackleton", "South: The Endurance Expedition"],
]);
    minutes.insert("02:12", &[
&["Then the lights went out all over the city. It happened at ", "2.12am", " according to power-house records, but Blake's diary gives no indication of the time. The entry is merely, 'Lights out - God help me.'", "HP Lovecraft", "The Haunter of the Dark"],
]);
    minutes.insert("02:13", &[
&["Now, listen: your destination is Friday, 4 August 1944, and the window will punch through at 22.30 hours. You're going to a dimension that diverged from our own at ", "02.13", " on the morning of Wednesday 20 February 1918, over twenty-six years earlier. You don't know what it's going to be like...", "Chris James", "The Second Internet Cafe, Part 1: The Dimension Researcher"],
]);
    minutes.insert("02:14", &[
&["Now, listen: your destination is Friday, 4 August 1944, and the window will punch through at 22.30 hours. You're going to a dimension that diverged from our own at ", "02.13", " on the morning of Wednesday 20 February 1918, over twenty-six years earlier. You don't know what it's going to be like...", "Chris James", "The Second Internet Cafe, Part 1: The Dimension Researcher"],
]);
    minutes.insert("02:15", &[
&["At ", "2.15am", " a policeman observed the place in darkness, but with the stranger's motor still at the curb.", "H.P. Lovecraft", "The Shadow Out of Time"],
&["It did. When the alarm rang at ", "two fifteen", ", Lew shut it off, snapped on the little bedside lamp, then swung his feet to the floor to sit on the edge of the bed, holding his eyes open.", "Jack Finney", "The Night People"],
]);
    minutes.insert("02:16", &[
&["At ", "2.15am", " a policeman observed the place in darkness, but with the stranger's motor still at the curb.", "H.P. Lovecraft", "The Shadow Out of Time"],
&["It did. When the alarm rang at ", "two fifteen", ", Lew shut it off, snapped on the little bedside lamp, then swung his feet to the floor to sit on the edge of the bed, holding his eyes open.", "Jack Finney", "The Night People"],
]);
    minutes.insert("02:17", &[
&["\"What time is it now?\" He turned her very dusty alarm clock to check. \"", "Two-seventeen", ",\" he marveled. It was the strangest time he'd seen in his entire life. \"I apologize that the room is so messy,\" Lalitha said. \"I like it. I love how you are. Are you hungry? I'm a little hungry.\" \"No, Walter.\" She smiled. \"I'm not hungry. But I can get you something.\" \"I was thinking, like, a glass of soy milk. Soy beverage.\"", "Jonathan Franzen", "Freedom"],
&["One of the \"choppers\" stopped, did an about-turn and came back to me. The flare spluttered and faded, and now the glare of the spotlight blinded me. I sat very still. It was ", "2.17", ". Against the noise of the blades a deeper resonant sound bit into the chill black air.", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert(
        "02:18",
        &[&[
            "It was ",
            "2:18 in the morning",
            ", and Donna could see no one else in any other office working so late.",
            "Jane Smiley",
            "Moo",
        ]],
    );
    minutes.insert(
        "02:19",
        &[&[
            "It was ",
            "2:18 in the morning",
            ", and Donna could see no one else in any other office working so late.",
            "Jane Smiley",
            "Moo",
        ]],
    );
    minutes.insert("02:20", &[
&["She turned abruptly to the nurse and asked the time. '", "Two-twenty", "' 'Ah...Two-twenty!' Genevieve repeated, as though there was something urgent to be done.", "Antoine de Saint Exupery", "Southern Mail"],
&["The night of his third walk Lew slept in his own apartment. When his eyes opened at ", "two twenty", ", by the green hands of his alarm, he knew that this time he'd actually been waiting for it in his sleep.", "Jack Finney", "The Night People"],
]);
    minutes.insert("02:21", &[
&["", "2:21 a.m.", " Lance-Corporal Hartmann emerged from the house in the Rue de Londres.", "Hans Hellmut Kirst", "The Night of the Generals"],
&["It was the urge to look up at the sky. But of course there was no sun nor moon nor stars overhead. Darkness hung heavy over me. Each breath I took, each wet footstep, everything wanted to slide like mud to the ground. I lifted my left hand and pressed on the light of my digital wristwatch. ", "Two-twenty-one", ". It was midnight when we headed underground, so only a little over two hours had passed. We continued walking down, down the narrow trench, mouths clamped tight.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("02:22", &[
&["", "2:21 a.m.", " Lance-Corporal Hartmann emerged from the house in the Rue de Londres.", "Hans Hellmut Kirst", "The Night of the Generals"],
&["It was the urge to look up at the sky. But of course there was no sun nor moon nor stars overhead. Darkness hung heavy over me. Each breath I took, each wet footstep, everything wanted to slide like mud to the ground. I lifted my left hand and pressed on the light of my digital wristwatch. ", "Two-twenty-one", ". It was midnight when we headed underground, so only a little over two hours had passed. We continued walking down, down the narrow trench, mouths clamped tight.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("02:23", &[
&["", "2:21 a.m.", " Lance-Corporal Hartmann emerged from the house in the Rue de Londres.", "Hans Hellmut Kirst", "The Night of the Generals"],
&["It was the urge to look up at the sky. But of course there was no sun nor moon nor stars overhead. Darkness hung heavy over me. Each breath I took, each wet footstep, everything wanted to slide like mud to the ground. I lifted my left hand and pressed on the light of my digital wristwatch. ", "Two-twenty-one", ". It was midnight when we headed underground, so only a little over two hours had passed. We continued walking down, down the narrow trench, mouths clamped tight.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("02:24", &[
&["It was ", "2.24am.", " She stumbled out of bed, tripping on her shoes that she’d kicked off earlier and pulled on a jumper.", "Maggie O’Farrell", "After You’d Gone"],
]);
    minutes.insert(
        "02:25",
        &[&[
            "You see it is time: ",
            "2.25am.",
            " You get out of bed.",
            "David Peace",
            "Nineteen Eighty-Three: The Red Riding Quartet, Book Four",
        ]],
    );
    minutes.insert(
        "02:26",
        &[&[
            "Listened to a voicemail message left at ",
            "2.26am",
            " by Claude.",
            "Richard Mason",
            "The Lighted Rooms",
        ]],
    );
    minutes.insert(
        "02:27",
        &[&[
            "The moon didn’t shine again until ",
            "2.27am.",
            " It was enough to show Wallander that he was positioned some distance below the tree.",
            "Henning Mankell",
            "One Step Behind",
        ]],
    );
    minutes.insert(
        "02:28",
        &[&[
            "",
            "2.28am",
            ": Ran out of sheep and began counting other farmyard animals.",
            "Mike Gayle",
            "Mr Commitment",
        ]],
    );
    minutes.insert(
        "02:29",
        &[&[
            "",
            "2.28am",
            ": Ran out of sheep and began counting other farmyard animals.",
            "Mike Gayle",
            "Mr Commitment",
        ]],
    );
    minutes.insert("02:30", &[
&["\"Get into the mood, Shirl!\" Lew said. \"The party's already started! Yippee! You dressed for a party, Harry?\" \"Yep. Something told me to put on dinner clothes when I went to bed tonight.\" \"I'm in mufti myself: white gloves and matching tennis shoes. But I'm sorry to report that Jo is still in her Dr. Dentons. What're you wearing, Shirl?\" \"My old drum majorette's outfit. The one I wore to the State Finals. Listen, we can't tie up the phones like this.\" \"Why not?\" said Harry. \"Who's going to call at ", "2:30 a.m.", " with a better idea? Yippee, to quote Lew, we're having a party! What're we serving, Lew?\" \"Beer, I guess. Haven't got any wine, have we, Jo?\" \"Just for cooking.\"", "Jack Finney", "The Night People"],
&["At about ", "half past two", " she had been woken by the creak of footsteps out on the stairs. At first she had been frightened.", "Sarah Waters", "The Little Stranger"],
&["Inc, I tried to pull her off about 0", "230", ", and there was this fucking… sound.", "David Foster Wallace", "Infinite Jest"],
&["It is ", "2.30am", " and I am tight. As a tick, as a lord, as a newt. Must write this down before the sublime memories fade and blur.", "William Boyd", "Any Human Heart"],
]);
    minutes.insert(
        "02:31",
        &[&[
            "And then I woke up because there were people shouting in the flat and it was ",
            "2.31am.",
            " And one of the people was Father and I was frightened.",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "02:32",
        &[&[
            "The last guests departed at ",
            "2.32 a.m.",
            ", two hours and two minutes after the scheduled completion time.",
            "Graeme Simsion",
            "The Rosie Project",
        ]],
    );
    minutes.insert("02:33", &[
&["But it wasn't going on! It was two-thirty-four, well. ", "Two-thirty-three", " and nothing had happened. Suppose he got a room call, or the elevator night-bell rang, now.", "Jim Thompson", "A Swell-looking Babe"],
]);
    minutes.insert("02:34", &[
&["But it wasn't going on! It was ", "two-thirty-four", ", well. Two-thirty-three and nothing had happened. Suppose he got a room call, or the elevator night-bell rang, now.", "Jim Thompson", "A Swell-looking Babe"],
]);
    minutes.insert("02:35", &[
&["For what happened at ", "2.35", " we have the testimony of the priest, a young, intelligent, and well-educated person; of Patrolman William J. Monohan of the Central Station, an officer of the highest reliability who had paused at that part of his beat to inspect the crowd.", "HP Lovecraft", "The Haunter of the Dark"],
]);
    minutes.insert("02:36", &[
&["It was about ", "2.36am", " when a provost colonel arrived to arrest me. At 2.36 1/2 I remembered the big insulating gauntlets. But even had I remembered before, what could I have done?", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert(
        "02:37",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:38",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:39",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:40",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:41",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:42",
        &[&[
            "June 13, 1990. ",
            "Thirty-seven minutes past two",
            " in the morning. And sixteen seconds.",
            "Stephen King",
            "The Stand",
        ]],
    );
    minutes.insert(
        "02:43",
        &[&[
            "She settled back beside him. 'It's ",
            "2:43",
            ":12am, Case. Got a readout chipped into my optic nerve.'",
            "William Gibson",
            "Neuromancer",
        ]],
    );
    minutes.insert(
        "02:44",
        &[&[
            "She settled back beside him. 'It's ",
            "2:43",
            ":12am, Case. Got a readout chipped into my optic nerve.'",
            "William Gibson",
            "Neuromancer",
        ]],
    );
    minutes.insert(
        "02:45",
        &[&[
            "",
            "0245h",
            "., Ennet House, the hours that are truly wee.",
            "David Foster Wallace",
            "Infinite Jest",
        ]],
    );
    minutes.insert("02:46", &[
&["", "2.46am.", " The chain drive whirred and the paper target slid down the darkened range, ducking in and out of shafts of yellow incandescent light. At the firing station, a figure waited in the shadows. As the target passed the twenty-five-foot mark, the man opened fire: eight shots-rapid, unhesitating.", "Steve Sohmer", "Patriots"],
&["Vicki shoved her glasses at her face and peered at the clock. ", "Two forty-six", ". 'I don't have time for this' she muttered, sttling back against the pillows, heart still slamming against her ribs.", "Tanya Huff", "Blood Lines"],
]);
    minutes.insert("02:47", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:48", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:49", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:50", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:51", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:52", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:53", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:54", &[
&["The glowing numbers read ", "2.47am.", " Moisés sighs and turns back to the bathroom door. Finally, the doorknob turns and Conchita comes back to bed. She resumes her place next to Moisés. Relieved, he pulls her close.", "Daniel A. Olivas", "The Book of Want"],
]);
    minutes.insert("02:55", &[
&["\"It's the way the world will end, Harry. Recorded cocktail music nuclear-powered to play on for centuries after all life has been destroyed. Selections from 'No, No, Nanette,' throughout eternity. That do you for ", "2:55 a.m.", "?\"", "Jack Finney", "The Night People"],
&["Time to go: ", "2.55am.", " Two-handed, Cec lifted his peak cap from the chair.", "Iain Sinclair", "Downriver"],
]);
    minutes.insert(
        "02:56",
        &[&[
            "It was ",
            "2:56",
            " when the shovel touched the coffin. We all heard the sound and looked at each other.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert(
        "02:57",
        &[&[
            "It was ",
            "2:56",
            " when the shovel touched the coffin. We all heard the sound and looked at each other.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert(
        "02:58",
        &[&[
            "It was ",
            "2:56",
            " when the shovel touched the coffin. We all heard the sound and looked at each other.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert("02:59", &[
&["I remembered arriving in this room at ", "2.59", " one night. I remembered the sergeant who called me names: mostly Anglo-Saxon monosyllabic four-letter ones with an odd \"Commie\" thrown in for syntax.", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert("03:00", &[
&["\"She died this morning, very early, about ", "three o'clock", ".\"", "Virginia Woolf", "The Voyage Out"],
&["Three a.m. That’s our reward. ", "Three in the morn.", " The soul’s midnight. The tide goes out, the soul ebbs. And a train arrives at an hour of despair. Why?", "Ray Bradbury", "Something Wicked This Way Comes"],
&["According to her watch it was shortly after ", "three o'clock", ", and according to everything else it was night-time.", "Douglas Adams", "The Long Dark Tea-time of the Soul"],
&["", "At three am", " I was walking the floor and listening to Katchaturian working in a tractor factory. He called it a violin concerto. I called it a loose fan belt and the hell with it.", "Raymond Chandler", "The Long Goodbye"],
&["At ", "three o' clock in the morning", " Eurydice is bound to come into it. After all, why did I sit here like a telegrapher at a lost outpost if not to receive messages from everywhere about the lost Eurydice who was never mine to begin with but whom I lamented and sought continually both professionally and amateurishly. This is not a digression. Where I am at three o' clock in the morning - and by now every hour is three o' clock in the morning - there are no digressions, it's all one thing.", "Russell Hoban", "The Medusa Frequency"],
&["But ", "at three o’clock in the morning", ", a forgotten package has the same tragic importance as a death sentence, and the cure doesn’t work -- and in a real dark night of the soul it is always three o’clock in the morning, day after day.", "F. Scott Fitzgerald", "The Crack-Up"],
&["Early mornings, my mother is about, drifting in her pale nightie, making herself a cup of tea in the kitchen. Water begins to boil in the kettle; it starts as a private, secluded sound, pure as rain, and grows to a steady, solipsistic bubbling. Not till she has had one cup of tea, so weak that it has a colour accidentally golden, can she begin her day. She is an insomniac. Her nights are wide-eyed and excited with worry. Even at ", "three o'clock", " in the morning one might hear her eating a Bain Marie biscuit in the kitchen.", "Amit Chaudhuri", "Afternoon Raag"],
&["I slam the phone down but it misses the base. I hit the clock instead, which flashes ", "3 a.m.", "", "Dan Holloway", "Songs from the Other Side of the Wall"],
&["In a real dark night of the soul it is always ", "3 o'clock", " in the morning.", "F. Scott Fitzgerald", "The Crack-Up"],
&["It was six untroubled days later – the best days at the camp so far, lavish July light thickly spread everywhere, six masterpiece mountain midsummer days, one replicating the other – that someone stumbled jerkily, as if his ankles were in chains, to the Comanche cabin’s bathroom ", "at three A.M.", "", "Philip Roth", "Nemesis"],
&["It was ", "three in the morning", " when his taxi stopped by giant mounds of snow outside his hotel. He had not eaten in hours.", "Ian McEwan", "Solar"],
&["Once I saw a figure I shall never forget. It was ", "three o'clock at night", ", as I was going home from Blacky as usual; it was a short-cut for me, and there would be nobody in the street at this time of night, I thought, especially not in this frightful cold.", "Max Frisch", "I'm Not Stiller"],
&["Roused from her sleep, Freya Gaines groped for the switch of the vidphone; groggily she found it and snapped it on. 'Lo,' she mumbled, wondering what time it was. She made out the luminous dial of the clock beside the bed. ", "Three AM.", " Good grief.", "Philip K Dick", "The Game Players of Titan"],
&["Schact clears his mouth and swallows mightily. 'Tavis can't even regrout tile in the locker room without calling a Community meeting or appointing a committee. The Regrouting Committee's been dragging along since may. Suddenly they're pulling secret 0", "300", " milk-switches? It doesn't ring true, Jim.", "David Foster Wallace", "Infinite Jest"],
&["", "Three in the morning", ", thought Charles Halloway, seated on the edge of his bed. Why did the train come at that hour? For, he thought, it’s a special hour. Women never wake then, do they? They sleep the sleep of babes and children. But men in middle age? They know that hour well.", "Ray Bradbury", "Something Wicked This Way Comes"],
&["What's the time?\" said the man, eyeing George up and down with evident suspicion; \"why, if you listen you will hear it strike.\" George listened, and a neighbouring clock immediately obliged. \"But it's only gone ", "three", "!\" said George in an injured tone, when it had finished.", "Jerome K Jerome", "Three Men in a Boat"],
&["When Sophie awoke, it was ", "3:00 a.m.", "", "Paula Fox", "Desperate Characters"],
&["You hearken, Missy. It’s ", "three o’clock", " in the morning and I’ve got all my faculties as well as ever I had in my life. I know all my property and where the money’s put out. And I’ve made everything ready to change my mind, and do as I like at the last. Do you hear, Missy? I’ve got my faculties.”", "George Eliot", "Middlemarch"],
]);
    minutes.insert("03:01", &[
&["It was now ", "about three o'clock", " in the morning and Francis Macomber, who had been asleep a little while after he had stopped thinking about the lion, wakened and then slept again.", "Ernest Hemingway", "The Short Happy Life of Francis Macomber"],
]);
    minutes.insert("03:02", &[
&["It was now ", "about three o'clock", " in the morning and Francis Macomber, who had been asleep a little while after he had stopped thinking about the lion, wakened and then slept again.", "Ernest Hemingway", "The Short Happy Life of Francis Macomber"],
]);
    minutes.insert("03:03", &[
&["It was now ", "about three o'clock", " in the morning and Francis Macomber, who had been asleep a little while after he had stopped thinking about the lion, wakened and then slept again.", "Ernest Hemingway", "The Short Happy Life of Francis Macomber"],
]);
    minutes.insert("03:04", &[
&["…his back-up alarm clock rang. He looked at his front-line clock on the bedside table and noted that it had stopped at ", "3.04", ". So, you couldn’t even rely on alarm clocks.", "Henning Mankell", "The Return of the Dancing Master"],
]);
    minutes.insert("03:05", &[
&["On the Sunday before Christmas she awoke at ", "3:05 a.m.", " and though: Thirty-six hours. Four hours later she got up thinking: Thirty-two hours. Late in the day she took Alfred to the street-association Christmas party at Dale and Honey Driblett’s, sat him down safely with Kirby Root, and proceeded to remind all her neighbors that her favorite grandson, who’d been looking forward all year to a Christmas in St. Jude, was arriving tomorrow afternoon.", "Jonathan Franzen", "The Corrections"],
]);
    minutes.insert("03:06", &[
&["On the Sunday before Christmas she awoke at ", "3:05 a.m.", " and though: Thirty-six hours. Four hours later she got up thinking: Thirty-two hours. Late in the day she took Alfred to the street-association Christmas party at Dale and Honey Driblett’s, sat him down safely with Kirby Root, and proceeded to remind all her neighbors that her favorite grandson, who’d been looking forward all year to a Christmas in St. Jude, was arriving tomorrow afternoon.", "Jonathan Franzen", "The Corrections"],
]);
    minutes.insert("03:07", &[
&["Wayne late-logged in: ", "3.07am", " -the late-late show. He parked. He dumped his milk can. He yawned, he stretched. He scratched.", "James Ellroy", "The Cold Six Thousand"],
]);
    minutes.insert("03:08", &[
&["Wayne late-logged in: ", "3.07am", " -the late-late show. He parked. He dumped his milk can. He yawned, he stretched. He scratched.", "James Ellroy", "The Cold Six Thousand"],
]);
    minutes.insert("03:09", &[
&["Wayne late-logged in: ", "3.07am", " -the late-late show. He parked. He dumped his milk can. He yawned, he stretched. He scratched.", "James Ellroy", "The Cold Six Thousand"],
]);
    minutes.insert("03:10", &[
&["I think my credit card was in there too. I wrote down the words credit card and said that if they wouldn't let me cancel them I'd demand that they registered the loss so you couldn't be charge for anything beyond the time of my calling them up. I looked at the clock. It was ", "ten-past three", ".", "Ali Smith", "The Whole Story and Other Stories"],
&["Love again; wanking at ", "ten past three", "", "Philip Larkin", "Love Again"],
]);
    minutes.insert("03:11", &[
&["I think my credit card was in there too. I wrote down the words credit card and said that if they wouldn't let me cancel them I'd demand that they registered the loss so you couldn't be charge for anything beyond the time of my calling them up. I looked at the clock. It was ", "ten-past three", ".", "Ali Smith", "The Whole Story and Other Stories"],
&["Love again; wanking at ", "ten past three", "", "Philip Larkin", "Love Again"],
]);
    minutes.insert("03:12", &[
&["I think my credit card was in there too. I wrote down the words credit card and said that if they wouldn't let me cancel them I'd demand that they registered the loss so you couldn't be charge for anything beyond the time of my calling them up. I looked at the clock. It was ", "ten-past three", ".", "Ali Smith", "The Whole Story and Other Stories"],
&["Love again; wanking at ", "ten past three", "", "Philip Larkin", "Love Again"],
]);
    minutes.insert("03:13", &[
&["I think my credit card was in there too. I wrote down the words credit card and said that if they wouldn't let me cancel them I'd demand that they registered the loss so you couldn't be charge for anything beyond the time of my calling them up. I looked at the clock. It was ", "ten-past three", ".", "Ali Smith", "The Whole Story and Other Stories"],
&["Love again; wanking at ", "ten past three", "", "Philip Larkin", "Love Again"],
]);
    minutes.insert("03:14", &[
&["Since he had told the girl that it had to end, he'd been waking up every morning at ", "3.14", ", without fail. Every morning his eyes would flick open, alert, and the red numerals on his electric alarm clock would read 3.14.", "Christos Tsiolkas", "The Slap"],
]);
    minutes.insert("03:15", &[
&["Above the door of Room 69 the clock ticked on at ", "3:15", ". The motion was accelerating. What had once been the gymnasium was now a small room, seven feet wide, a tight, almost perfect cube.", "JG Ballard", "Manhole 69"],
]);
    minutes.insert("03:16", &[
&["Above the door of Room 69 the clock ticked on at ", "3:15", ". The motion was accelerating. What had once been the gymnasium was now a small room, seven feet wide, a tight, almost perfect cube.", "JG Ballard", "Manhole 69"],
]);
    minutes.insert("03:17", &[
&["The two of us sat there, listening—Boris more intently than me. “Who’s that with him then?” I said. “Some whore.” He listened for a moment, brow furrowed, his profile sharp in the moonlight, and then lay back down. “Two of them.” I rolled over, and checked my iPod. It was ", "3:17", " in the morning.", "Donna Tartt", "The Goldfinch"],
&["He turned to the monitors again and flicked through the screens, each one able to display eight different camera mountings, giving Kurt 192 different still lives of Green Oaks at ", "3.17 a.m.", " this March night.", "Catherine O'Flynn", "What Was Lost"],
]);
    minutes.insert("03:18", &[
&["The two of us sat there, listening—Boris more intently than me. “Who’s that with him then?” I said. “Some whore.” He listened for a moment, brow furrowed, his profile sharp in the moonlight, and then lay back down. “Two of them.” I rolled over, and checked my iPod. It was ", "3:17", " in the morning.", "Donna Tartt", "The Goldfinch"],
&["He turned to the monitors again and flicked through the screens, each one able to display eight different camera mountings, giving Kurt 192 different still lives of Green Oaks at ", "3.17 a.m.", " this March night.", "Catherine O'Flynn", "What Was Lost"],
]);
    minutes.insert(
        "03:19",
        &[&[
            "The time stamp on Navidson's camcorder indicates that it is exactly ",
            "3.19 A.M.",
            "",
            "Mark Z Danielewski",
            "House of Leaves",
        ]],
    );
    minutes.insert(
        "03:20",
        &[&[
            "Prabath Kumara, 16. 17th November 1989. At ",
            "3.20am",
            " from the home of a friend.",
            "Michael Ondaatje",
            "Anil's Ghost",
        ]],
    );
    minutes.insert("03:21", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at ", "twenty-one minutes past three", ", the half-ebb at a quarter past seven, low water at thirty-three minutes past nine, and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("03:22", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at ", "twenty-one minutes past three", ", the half-ebb at a quarter past seven, low water at thirty-three minutes past nine, and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("03:23", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at ", "twenty-one minutes past three", ", the half-ebb at a quarter past seven, low water at thirty-three minutes past nine, and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("03:24", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at ", "twenty-one minutes past three", ", the half-ebb at a quarter past seven, low water at thirty-three minutes past nine, and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert(
        "03:25",
        &[&[
            "It was ",
            "3:25 a.m.",
            " A strange thrill, to think I was the only Mulvaney awake in the house.",
            "Joyce Carol Oates",
            "We Were the Mulvaneys",
        ]],
    );
    minutes.insert(
        "03:26",
        &[&[
            "It was ",
            "3:25 a.m.",
            " A strange thrill, to think I was the only Mulvaney awake in the house.",
            "Joyce Carol Oates",
            "We Were the Mulvaneys",
        ]],
    );
    minutes.insert(
        "03:27",
        &[&[
            "It was ",
            "3:25 a.m.",
            " A strange thrill, to think I was the only Mulvaney awake in the house.",
            "Joyce Carol Oates",
            "We Were the Mulvaneys",
        ]],
    );
    minutes.insert("03:28", &[
&["Now somebody was running past his room. A door slammed. That foreign language again. What the devil was going on? he switched on his light and peered at his watch. ", "3.28", ". He got out of bed.", "Rupert Thomson", "Dreams of Leaving"],
]);
    minutes.insert("03:29", &[
&["Now somebody was running past his room. A door slammed. That foreign language again. What the devil was going on? he switched on his light and peered at his watch. ", "3.28", ". He got out of bed.", "Rupert Thomson", "Dreams of Leaving"],
]);
    minutes.insert("03:30", &[
&["At ", "Half past Three", ", a single Bird Unto a silent Sky Propounded but a single term Of cautious melody.", "Emily Dickinson", "At Half past Three, a single Bird"],
&["At ", "half-past three A.M.", " he lost one illusion: officers sent to reconnoitre informed him that the enemy was making no movement.", "Victor Hugo", "Les Miserables"],
&["It's ", "3:30 A.M.", " in Mrs. Ralph's finally quiet house when Garp decides to clean the kitchen, to kill the time until dawn. Familiar with a housewife's tasks, Garp fills the sink and starts to wash the dishes.", "John Irving", "The World According to Garp"],
&["Let's go to sleep, I say. \"Look at what time it is.\" The clock radio is right there beside the bed. Anyone can see it says ", "three-thirty", ".", "Raymond Carver", "Whoever Was Using This Bed"],
&["Now, look. I am not going to call Dr. McGrath at ", "three thirty", " in the morning to ask if it's all right for my son to eat worms. That's flat.", "Thomas Rockwell", "How to Eat Fried Worms"],
]);
    minutes.insert("03:31", &[
&["At ", "Half past Three", ", a single Bird Unto a silent Sky Propounded but a single term Of cautious melody.", "Emily Dickinson", "At Half past Three, a single Bird"],
&["At ", "half-past three A.M.", " he lost one illusion: officers sent to reconnoitre informed him that the enemy was making no movement.", "Victor Hugo", "Les Miserables"],
&["It's ", "3:30 A.M.", " in Mrs. Ralph's finally quiet house when Garp decides to clean the kitchen, to kill the time until dawn. Familiar with a housewife's tasks, Garp fills the sink and starts to wash the dishes.", "John Irving", "The World According to Garp"],
&["Let's go to sleep, I say. \"Look at what time it is.\" The clock radio is right there beside the bed. Anyone can see it says ", "three-thirty", ".", "Raymond Carver", "Whoever Was Using This Bed"],
&["Now, look. I am not going to call Dr. McGrath at ", "three thirty", " in the morning to ask if it's all right for my son to eat worms. That's flat.", "Thomas Rockwell", "How to Eat Fried Worms"],
]);
    minutes.insert("03:32", &[
&["At ", "Half past Three", ", a single Bird Unto a silent Sky Propounded but a single term Of cautious melody.", "Emily Dickinson", "At Half past Three, a single Bird"],
&["At ", "half-past three A.M.", " he lost one illusion: officers sent to reconnoitre informed him that the enemy was making no movement.", "Victor Hugo", "Les Miserables"],
&["It's ", "3:30 A.M.", " in Mrs. Ralph's finally quiet house when Garp decides to clean the kitchen, to kill the time until dawn. Familiar with a housewife's tasks, Garp fills the sink and starts to wash the dishes.", "John Irving", "The World According to Garp"],
&["Let's go to sleep, I say. \"Look at what time it is.\" The clock radio is right there beside the bed. Anyone can see it says ", "three-thirty", ".", "Raymond Carver", "Whoever Was Using This Bed"],
&["Now, look. I am not going to call Dr. McGrath at ", "three thirty", " in the morning to ask if it's all right for my son to eat worms. That's flat.", "Thomas Rockwell", "How to Eat Fried Worms"],
]);
    minutes.insert("03:33", &[
&["A draft whistled in around the kitchen window frame and I shivered. The digital clock on Perkus's stove read ", "3:33", ".", "Jonathan Lethem", "Chronic City"],
]);
    minutes.insert(
        "03:34",
        &[&[
            "It was ",
            "3:34 am.",
            " and he was wide-awake. He'd heard the phone ring and the sound of his uncle's voice.",
            "Muriel Jensen",
            "Always Florence",
        ]],
    );
    minutes.insert(
        "03:35",
        &[&[
            "He could just see the hands of the alarm clock in the darkness: ",
            "3.35 a.m.",
            " He adjusted his pillow and shut his eyes.",
            "Henning Mankell",
            "The Dogs of Riga",
        ]],
    );
    minutes.insert("03:36", &[
&["As I near Deadhorse, it's ", "3:36 a.m.", " and seventeen below. Tall, sodium vapor lights spill on the road and there are no trees, only machines, mechanical shadows. There isn't even a church. It tells you everything.", "Richard C Matheson", "Zoopraxis"],
]);
    minutes.insert("03:37", &[
&["It was three ", "thirty-seven A.M.", ", and for once Maggie was asleep. She had got to be a pretty good sleeper in the last few months. Clyde was prouder of this fact than anything.", "Stephen Bury", "The Cobweb"],
]);
    minutes.insert("03:38", &[
&["At ", "3.38am", ", it began to snow in Bowling Green, Kentucky. The geese circling the city flew back to the park, landed, and hunkered down to sit it out on their island in the lake.", "Connie Willis", "Just Like the Ones we Used to Know"],
]);
    minutes.insert("03:39", &[
&["23 October 1893 ", "3.39am.", " Upon further thought, I feel it necessary to explain that exile into the Master's workshop is not an unpleasant fate. It is not simply some bare-walled cellar devoid of stimulation - quite the opposite.", "William Jablonsky", "The Clockwork man"],
]);
    minutes.insert("03:40", &[
&["His bedside clock shows ", "three forty", ". He has no idea what he's doing out of bed: he has no need to relieve himself, nor is he disturbed by a dream or some element of the day before, or even by the state of the world.", "Ian McEwan", "Saturday"],
]);
    minutes.insert("03:41", &[
&["The alarm clock said ", "3.41am.", " He sat up. Why was the alarm clock slow? He picked up the alarm clock and adjusted the hands to show the same time as his wristwatch: 3.44am", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("03:42", &[
&["\"We are due in Yellow Sky at ", "3:42", ",\" he said, looking tenderly into her eyes. \"\"Oh, are we?\"\" she said, as if she had not been aware of it. To evince surprise at her husband's statement was part of her wifely amiability.", "Stephen Crane", "Bride Comes to Yellow Sky"],
]);
    minutes.insert("03:43", &[
&["The clock says ", "3.43am.", " The thermometer says it's a chilly fourteen degrees Fahrenheit. The weatherman says the cold spell will last until Thursday, so bundle up and bundle up some more. There are icicles barring the window of the bat cave.", "David Mitchell", "Ghostwritten"],
]);
    minutes.insert("03:44", &[
&["It was dark. After she had switched the light on and been to the toilet, she checked her watch: ", "3.44 a.m.", " She undressed, put the cat out the door and returned to the twin bed.", "Will Self", "Liver: Leberknödel"],
]);
    minutes.insert("03:45", &[
&["LORD CAVERSHAM: Well, sir! what are you doing here? Wasting your life as usual! You should be in bed, sir. You keep too late hours! I heard of you the other night at Lady Rufford's dancing till four o' clock in the morning! LORD GORING: Only a ", "quarter to four", ", father.", "Oscar Wilde", "An Ideal Husband"],
]);
    minutes.insert("03:46", &[
&["LORD CAVERSHAM: Well, sir! what are you doing here? Wasting your life as usual! You should be in bed, sir. You keep too late hours! I heard of you the other night at Lady Rufford's dancing till four o' clock in the morning! LORD GORING: Only a ", "quarter to four", ", father.", "Oscar Wilde", "An Ideal Husband"],
]);
    minutes.insert("03:47", &[
&["I stayed awake until ", "3:47", ". That was the last time I looked at my watch before I fell asleep. It has a luminous face and lights up if you press a button so I could read it in the dark. I was cold and I was frightened Father might come out and find me. But I felt safer in the garden because I was hidden.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("03:48", &[
&["I stayed awake until ", "3:47", ". That was the last time I looked at my watch before I fell asleep. It has a luminous face and lights up if you press a button so I could read it in the dark. I was cold and I was frightened Father might come out and find me. But I felt safer in the garden because I was hidden.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("03:49", &[
&["It was ", "3.49", " when he hit me because of the two hundred times I had said, \"I don't know.\" He hit me a lot after that.", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert("03:50", &[
&["She had used her cell phone to leave several messages on the answering machine in Sao Paulo of the young dentist of the previous evening, whose name was Fernando. The first was recorded at ", "ten or five to four", " in the morning. I'm never going to forget you ... I'm sure we'll meet again somewhere.", "Peter Robb", "A Death in Brazil: A Book of Omissions"],
]);
    minutes.insert("03:51", &[
&["I lacked the will and physical strength to get out of bed and move through the dark house, clutching walls and stair rails. To feel my way, reinhabit my body, re-enter the world. Sweat trickled down my ribs. The digital reading on the clock-radio was ", "3:51", ". Always odd numbered at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
&["The digital reading on the clock-radio was ", "3:51", ". Always odd numbers at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
]);
    minutes.insert("03:52", &[
&["I lacked the will and physical strength to get out of bed and move through the dark house, clutching walls and stair rails. To feel my way, reinhabit my body, re-enter the world. Sweat trickled down my ribs. The digital reading on the clock-radio was ", "3:51", ". Always odd numbered at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
&["The digital reading on the clock-radio was ", "3:51", ". Always odd numbers at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
]);
    minutes.insert("03:53", &[
&["I lacked the will and physical strength to get out of bed and move through the dark house, clutching walls and stair rails. To feel my way, reinhabit my body, re-enter the world. Sweat trickled down my ribs. The digital reading on the clock-radio was ", "3:51", ". Always odd numbered at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
&["The digital reading on the clock-radio was ", "3:51", ". Always odd numbers at times like this. What does it mean? Is death odd-numbered?", "Don DeLillo", "White Noise"],
]);
    minutes.insert(
        "03:54",
        &[&[
            "The charter flight from Florida touched down at Aldergrove minutes earlier, at ",
            "3.54 a.m.",
            "",
            "Paul Muldoon",
            "The More a Man Has, the More a Man Wants",
        ]],
    );
    minutes.insert(
        "03:55",
        &[&[
            "Here in the cavernous basement at ",
            "3.55 a.m.",
            ", in a single pool of light, is Theo Perowne.",
            "Ian McEwan",
            "Saturday",
        ]],
    );
    minutes.insert(
        "03:56",
        &[&[
            "Here in the cavernous basement at ",
            "3.55 a.m.",
            ", in a single pool of light, is Theo Perowne.",
            "Ian McEwan",
            "Saturday",
        ]],
    );
    minutes.insert("03:57", &[
&["Certain facts were apparent: dark; cold; thundering boots; quilts; pillow; light under the door – the materials of reality - but I could not pin these materials down in time. And the raw materials of reality without that glue of time are materials adrift and reality is as meaningless as the balsa parts of a model airplane scattered to the wind...I am in my old room, yes, in the dark, certainly, and it is cold, obviously, but what time is it? \"", "Nearly four", ", son.\" But I mean what time?", "Ken Kesey", "Sometimes a Great Notion"],
]);
    minutes.insert("03:58", &[
&["The ancient house was deserted, the crumbling garage padlocked, and one was just able to discern - by peering through a crack in the bubbling sun on the window - the face of a clock on the opposite wall. The clock had stopped at ", "two minutes to four", " early in the morning, or who could tell, it may have been earlier still, yesterday in the afternoon, a couple of hours after Kaiser had left Kamaria for Bartica.", "Wilson Harris", "Heartland"],
&["The clock atop the clubhouse reads ", "3:58", ".", "Don Delillo", "Underworld"],
]);
    minutes.insert("03:59", &[
&["And the raw materials of reality without that glue of time are materials adrift and reality is as meaningless as the balsa parts of a model airplane scattered to the wind...I am in my old room, yes, in the dark, certainly, and it is cold, obviously, but what time is it? \"", "Nearly four", ", son.\"", "Ken Kesey", "Sometimes a Great Notion"],
]);
    minutes.insert("04:00", &[
&["\"Nothing happened,\" he said wanly. \"I waited, and about ", "four o'clock", " she came to the window and stood there for a minute and then turned out the light.\"", "F. Scott Fitzgerald", "The Great Gatsby"],
&["I looked at the clock and it was (yes, you guessed it) ", "four am.", " I should have taken comfort from the fact that approximately quarter of the Greenwich Mean Time world had just jolted awake also and were lying, staring miserably into the darkness, worrying ...\"", "Marian Keyes", "Watermelon"],
&["Suddenly, he started to cry. Curled up on the sofa he sobbed loudly. Michel looked at his watch; it was just after ", "4am", ". On the screen a wild cat had a rabbit in its mouth.", "Michel Houellebecq", "Atomised"],
&["The Birds begun at ", "Four o'clock", "— Their period for Dawn—", "Emily Dickinson", "The Birds begun at Four o'clock"],
&["The night before Albert Kessler arrived in Santa Teresa, ", "at four", " in the morning, Sergio Gonzalez Rodriguez got a call from Azucena Esquivel Plata, reporter and PRI congresswoman.", "Roberto Bolano", "2666"],
&["Waking ", "at four", " to soundless dark, I stare. In time the curtain-edges will grow light. Till then I see what's really always there: Unresting death, a whole day nearer now, Making all thought impossible but how And where and when I shall myself die.", "Philip Larkin", "Aubade"],
&["When he noticed that the chefs from the grand hotels and restaurants - a picky, impatient bunch - tended to move around from seller to seller, buying apples here and broccoli there, he asked if he could have tea available for them. Tommy agreed, and the chefs, grateful for a hot drink ", "at four", " in the morning, lingered and bought.", "Jennifer Donnelly", "The Tea Rose"],
]);
    minutes.insert("04:01", &[
&["Suddenly, he started to cry. Curled up on the sofa he sobbed loudly. Michel looked at his watch; it was ", "just after 4am", ". On the screen a wild cat had a rabbit in its mouth.", "Michel Houellebecq", "Atomised"],
]);
    minutes.insert("04:02", &[
&["I walked up and down the row. No one gave me a second look. Finally I sat down next to a man. He paid no attention. My watch said ", "4:02", ". Maybe he was late.", "Nicole Krauss", "The History of Love"],
]);
    minutes.insert("04:03", &[
&["It's ", "4:03 a.m.", " on a supremely cold January morning and I'm just getting home. I've been out dancing and I'm only half drunk but utterly exhausted.", "Audrey Niffenegger", "The Time Traveler's Wife"],
]);
    minutes.insert("04:04", &[
&["", "Four minutes after four!", " It's still very early and to get from here to there won't take me more than 15 minutes, even walking slowly. She told me around five o'clock. Wouldn't it be better to wait on the corner?", "Cirilo Villaverde", "Angel Hill"],
]);
    minutes.insert("04:05", &[
&["Leaves were being blown against my window. It was ", "4.05am.", " The moon had shifted in the sky, glaring through a clotted mass of clouds like a candled egg.", "Joyce Carol Oates", "We Were the Mulvaneys"],
]);
    minutes.insert(
        "04:06",
        &[&[
            "Dexter looked at Kate's note, then her face, then the clock. It was ",
            "4.06am",
            ", the night before they would go to the restaurant.",
            "Chris Pavone",
            "The Expats",
        ]],
    );
    minutes.insert("04:07", &[
&["", "4.07am.", " Why am I standing? My shoulders feel cold and I'm shivering. I become aware that I'm standing in the middle of the room. I immediately look at the bedroom door. Closed, with no signs of a break-in. Why did I get up?", "Miha Mazzini", "Guarding Hanna: A Novel"],
]);
    minutes.insert("04:08", &[
&["It was at ", "4:08 a.m.", " beneath the cool metal of a jungle gym that all Andrew's dreams came true. He kissed his one true love and swore up and down that it would last forever to this exhausted companion throughout their long trek home.", "Seth O'Connell", "Dying in the Twilight of Summer"],
]);
    minutes.insert("04:09", &[
&["It was at ", "4:08 a.m.", " beneath the cool metal of a jungle gym that all Andrew's dreams came true. He kissed his one true love and swore up and down that it would last forever to this exhausted companion throughout their long trek home.", "Seth O'Connell", "Dying in the Twilight of Summer"],
]);
    minutes.insert("04:10", &[
&["It was at ", "4:08 a.m.", " beneath the cool metal of a jungle gym that all Andrew's dreams came true. He kissed his one true love and swore up and down that it would last forever to this exhausted companion throughout their long trek home.", "Seth O'Connell", "Dying in the Twilight of Summer"],
]);
    minutes.insert("04:11", &[
&["The next morning I awaken at exactly ", "eleven minutes after four", ", having slept straight through my normal middle-of-the-night insomniac waking at three.", "Karen Karbo", "The Stuff of Life"],
]);
    minutes.insert("04:12", &[
&["Finally, she signalled with her light that she'd made it to the top. I signalled back, then shined the light downward to see how far the water had risen. I couldn't make out a thing. My watch read ", "four-twelve", " in the morning. Not yet dawn. The morning papers still not delivered, trains not yet running, citizens of the surface world fast asleep, oblivious to all this. I pulled the rope taut with both hands, took a deep breath, then slowly began my climb.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Karen felt the bed move beneath Harry's weight. Lying on her side she opened her eyes to see digital numbers in the dark, ", "4:12", " in pale green. Behind her Harry continued to move, settling in. She watched the numbers change to 4:13.", "Elmore Leonard", "Get Shorty"],
]);
    minutes.insert("04:13", &[
&["Karen felt the bed move beneath Harry's weight. Lying on her side she opened her eyes to see digital numbers in the dark, 4:12 in pale green. Behind her Harry continued to move, settling in. She watched the numbers change to ", "4:13", ".", "Elmore Leonard", "Get Shorty"],
]);
    minutes.insert("04:14", &[
&["At ", "4:14 a.m.", ", the two men returned to the Jeep. After the passenger replaced the cans in the back of the Jeep, the driver backed out of the driveway and headed east. The last images found on the film appeared to be flames or smoke.", "David H Swendsen", "A Real Nightmare"],
]);
    minutes.insert("04:15", &[
&["Alice wants to warn her that a defect runs in the family, like flat feet or diabetes: they're all in danger of ending up alone by their own stubborn choice. The ugly kitchen clock says ", "four-fifteen", ".", "Barbara Kingsolver", "Pigs in Heaven"],
]);
    minutes.insert("04:16", &[
&["I stooped to pick up my watch from the floor. ", "Four-sixteen", ". Another hour until dawn. I went to the telephone and dialled my own number. It'd been a long time since I'd called home, so I had to struggle to remember the number. I let it ring fifteen times; no answer. I hung up, dialled again, and let it ring another fifteen times. Nobody.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["They pulled into the visitor's carpark at ", "four sixteen", " am. He knew it was four sixteen because the entrance to the maternity unit sported a digital clock beneath the signage.", "Maree Anderson", "Freaks in the City: Book Two of the Freaks Series"],
]);
    minutes.insert("04:17", &[
&["He awoke at ", "4.17am", " in a sweat. He had been dreaming of Africa again, and then the dream had continued in the U.S. when he was a young man. But Inbata had been there, watching him.", "Douglas Phinney", "The Vile"],
]);
    minutes.insert("04:18", &[
&["I grabbed the alarm clock, threw it on my lap, and slapped the red and black buttons with both hands. The ringing didn't stop. The telephone! The clock read ", "four-eighteen", ". It was dark outside. Four-eighteen a.m. I got out of bed and picked up the receiver. \"Hello?\"", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("04:19", &[
&["I grabbed the alarm clock, threw it on my lap, and slapped the red and black buttons with both hands. The ringing didn't stop. The telephone! The clock read ", "four-eighteen", ". It was dark outside. Four-eighteen a.m. I got out of bed and picked up the receiver. \"Hello?\"", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("04:20", &[
&["I grabbed the alarm clock, threw it on my lap, and slapped the red and black buttons with both hands. The ringing didn't stop. The telephone! The clock read ", "four-eighteen", ". It was dark outside. Four-eighteen a.m. I got out of bed and picked up the receiver. \"Hello?\"", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("04:21", &[
&["I grabbed the alarm clock, threw it on my lap, and slapped the red and black buttons with both hands. The ringing didn't stop. The telephone! The clock read ", "four-eighteen", ". It was dark outside. Four-eighteen a.m. I got out of bed and picked up the receiver. \"Hello?\"", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert(
        "04:22",
        &[&[
            "He hurt me to the point where I wanted to tell him something. My watch said ",
            "4.22",
            " now. It had stopped. It was smashed.",
            "Len Deighton",
            "The Ipcress File",
        ]],
    );
    minutes.insert("04:23", &[
&["", "4:23", ", Monday morning, Iceland Square. A number of people in the vicinity of Bjornsongatan are awakened by loud screams.", "John Ajvide Lindqvist", "Let The Right One In"],
&["Her chip pulsed the time. ", "04:23", ":04. It had been a long day.", "William Gibson", "Neuromancer"],
]);
    minutes.insert("04:24", &[
&["", "4:23", ", Monday morning, Iceland Square. A number of people in the vicinity of Bjornsongatan are awakened by loud screams.", "John Ajvide Lindqvist", "Let The Right One In"],
&["Her chip pulsed the time. ", "04:23", ":04. It had been a long day.", "William Gibson", "Neuromancer"],
]);
    minutes.insert("04:25", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ".", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("04:26", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ".", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("04:27", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ".", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("04:28", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ".", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("04:29", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ".", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("04:30", &[
&["At the end of a relationship, it is the one who is not in love who makes the tender speeches. I was overwhelmed by a sense of betrayal, betrayal because a union in which I had invested so much had been declared bankrupt without my feeling it to be so. Chloe had not given it a chance, I argued with myself, knowing the hopelessness of these inner courts announcing hollow verdicts at ", "four thirty", " in the morning.", "Alain de Botton", "Essays on Love"],
&["Hester Thrale undulates in in a false fox jacket at 2330 as usual even though she has to be up at like 0", "430", " for the breakfast shift at the Provident Nursing Home and sometimes eats breakfast with Gately, both their faces nodding perilously close to their Frosted Flakes.", "David Foster Wallace", "Infinite Jest"],
&["Tonight Clenette H. and the deeply whacked out Yolanda W. come back in from Footprints around 2315 in purple skirts and purple lipstick and ironed hair, tottering on heels and telling each other what a wicked time they just had. Hester Thrale undulates in in a false fox jacket at 2330 as usual even though she has to be up at like 0", "430", " for the breakfast shift at the Provident Nursing Home and sometimes eats breakfast with Gately, both their faces nodding perilously close to their Frosted Flakes.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert(
        "04:31",
        &[&[
            "An earthquake hit Los Angeles at ",
            "4:31",
            " this morning and the images began arriving via CNN right away.",
            "Douglas Coupland",
            "Microserfs",
        ]],
    );
    minutes.insert("04:32", &[
&["On his first day of kindergarten, Peter Houghton woke up at ", "4:32 a.m.", " He padded into his parents' room and asked if it was time yet to take the school bus.", "Jodi Picoult", "Nineteen Minutes"],
]);
    minutes.insert("04:33", &[
&["On his first day of kindergarten, Peter Houghton woke up at ", "4:32 a.m.", " He padded into his parents' room and asked if it was time yet to take the school bus.", "Jodi Picoult", "Nineteen Minutes"],
]);
    minutes.insert("04:34", &[
&["On his first day of kindergarten, Peter Houghton woke up at ", "4:32 a.m.", " He padded into his parents' room and asked if it was time yet to take the school bus.", "Jodi Picoult", "Nineteen Minutes"],
]);
    minutes.insert("04:35", &[
&["No manner of exhaustion can keep a child asleep much later than six a.m. on Christmas Day. Colby awoke at ", "4:35", ".", "C Robert Cargill", "Dreams and Shadows"],
]);
    minutes.insert("04:36", &[
&["At ", "4:36", " that morning, alone in my hotel room, it had been a much better scene. Spencer had blanched, confounded by the inescapable logic of my accusation. A few drops of perspiration had formed on his upper lip. A tiny vein had started to throb in his temple.", "Ross Thomas", "The Brass Go-Between"],
]);
    minutes.insert("04:37", &[
&["At ", "4:36", " that morning, alone in my hotel room, it had been a much better scene. Spencer had blanched, confounded by the inescapable logic of my accusation. A few drops of perspiration had formed on his upper lip. A tiny vein had started to throb in his temple.", "Ross Thomas", "The Brass Go-Between"],
]);
    minutes.insert("04:38", &[
&["At ", "4.38 a.m.", " as the sun is coming up over Gorley Woods, I hear a strange rustling in the grass beside me. I peer closely but can see nothing.", "Jonathan Barrow", "The Queue"],
]);
    minutes.insert("04:39", &[
&["At ", "4.38 a.m.", " as the sun is coming up over Gorley Woods, I hear a strange rustling in the grass beside me. I peer closely but can see nothing.", "Jonathan Barrow", "The Queue"],
]);
    minutes.insert(
        "04:40",
        &[&[
            "I settled into a daily routine. Wake up at ",
            "4.40am",
            ", shower, get on the train north by ten after five.",
            "Tina Fey",
            "Bossypants",
        ]],
    );
    minutes.insert("04:41", &[
&["At ", "4:41", " Crane's voice crackled through the walkie-talkie as if he'd read their thoughts of mutiny. “Everyone into the elevator. Now!” Only moments before the call he and C.J. had finished what they hoped would be a successful diversion.", "Roland S. Jefferson", "Damaged Goods: A Novel"],
]);
    minutes.insert("04:42", &[
&["At ", "4:41", " Crane's voice crackled through the walkie-talkie as if he'd read their thoughts of mutiny. “Everyone into the elevator. Now!” Only moments before the call he and C.J. had finished what they hoped would be a successful diversion.", "Roland S. Jefferson", "Damaged Goods: A Novel"],
]);
    minutes.insert(
        "04:43",
        &[&[
            "The time is ",
            "four forty-three",
            " in the mornin an it's almost light oot there.",
            "Suhayl Saadi",
            "Pyschoraag",
        ]],
    );
    minutes.insert(
        "04:44",
        &[&[
            "The time is ",
            "four forty-three",
            " in the mornin an it's almost light oot there.",
            "Suhayl Saadi",
            "Pyschoraag",
        ]],
    );
    minutes.insert("04:45", &[
&["He lies still in the darkness and listens. His wife's breathing at his side is so faint that he can scarcely hear it. One of these mornings she'll be lying dead beside me and I won't even notice, he thinks. Or maybe it'll be me. Daybreak will reveal that one of us has been left alone. He checks the clock on the table next to the bed. The hands glow and register ", "4:45 a.m.", "", "Henning Mankell", "Faceless Killers"],
&["His wife's breathing at his side is so faint that he can scarcely hear it. One of these mornings she'll be lying dead beside me and I won't even notice, he thinks. Or maybe it'll be me. Daybreak will reveal that one of us has been left alone. He checks the clock on the table next to the bed. The hands glow and register ", "4:45 a.m.", "", "Henning Mankell", "Faceless Killers"],
]);
    minutes.insert("04:46", &[
&["The phone rang again at ", "four-forty-six", ".\"Hello,\" I said. \"Hello,\" came a woman's voice. \"Sorry about the time before. There's a disturbance in the sound field. Sometimes the sound goes away.\" \"The sound goes away?\" \"Yes,\" she said. \"The sound field's slipping. Can you hear me?\" \"Loud and clear,\" I said. It was the granddaughter of that kooky old scientist who'd given me the unicorn skull. The girl in the pink suit.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("04:47", &[
&["The phone rang again at ", "four-forty-six", ".\"Hello,\" I said. \"Hello,\" came a woman's voice. \"Sorry about the time before. There's a disturbance in the sound field. Sometimes the sound goes away.\" \"The sound goes away?\" \"Yes,\" she said. \"The sound field's slipping. Can you hear me?\" \"Loud and clear,\" I said. It was the granddaughter of that kooky old scientist who'd given me the unicorn skull. The girl in the pink suit.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert(
        "04:48",
        &[&[
            "At ",
            "4:48",
            " the happy hour when clarity visits warm darkness which soaks my eyes I know no sin",
            "Sarah Kane",
            "4:48 Psychosis",
        ]],
    );
    minutes.insert(
        "04:49",
        &[&[
            "At ",
            "4:48",
            " the happy hour when clarity visits warm darkness which soaks my eyes I know no sin",
            "Sarah Kane",
            "4:48 Psychosis",
        ]],
    );
    minutes.insert("04:50", &[
&["Even the hands of his watch and the hands of all the thirteen clocks were frozen. They had all frozen at the same time, on a snowy night, seven years before, and after that it was always ", "ten minutes to five", " in the castle.", "James Thurber", "The 13 Clocks"],
]);
    minutes.insert("04:51", &[
&["Even the hands of his watch and the hands of all the thirteen clocks were frozen. They had all frozen at the same time, on a snowy night, seven years before, and after that it was always ", "ten minutes to five", " in the castle.", "James Thurber", "The 13 Clocks"],
]);
    minutes.insert("04:52", &[
&["Even the hands of his watch and the hands of all the thirteen clocks were frozen. They had all frozen at the same time, on a snowy night, seven years before, and after that it was always ", "ten minutes to five", " in the castle.", "James Thurber", "The 13 Clocks"],
]);
    minutes.insert("04:53", &[
&["Even the hands of his watch and the hands of all the thirteen clocks were frozen. They had all frozen at the same time, on a snowy night, seven years before, and after that it was always ", "ten minutes to five", " in the castle.", "James Thurber", "The 13 Clocks"],
]);
    minutes.insert("04:54", &[
&["", "Six minutes to five", ". Six minutes to go. Suddenly I felt quite clearheaded. There was an unexpected light in the cell; the boundaries were drawn, the roles well defined. The time of doubt and questioning and uncertainty was over.", "Elie Wiesel", "Dawn: A Novel"],
]);
    minutes.insert("04:55", &[
&["", "4:55", " - Mank holding phone. Turns to Caddell - 'Who is this?' Caddell: 'Jim.' (shrugs) 'I think he's our man in Cincinnati.'", "Hunter S. Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("04:56", &[
&["", "4:55", " - Mank holding phone. Turns to Caddell - 'Who is this?' Caddell: 'Jim.' (shrugs) 'I think he's our man in Cincinnati.'", "Hunter S. Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("04:57", &[
&["The second said the same thing a ", "few minutes before five", ", and mentioned eternity... I'm sure I'll meet you in the other world. Four minutes later she left a last, fleeting message: My love. Fernando. It's Suzana. Then, it seemed, she had shot herself.", "Peter Robb", "A Death in Brazil: A Book of Omissions"],
]);
    minutes.insert("04:58", &[
&["He wants to look death in the face. ", "Two minutes to five", ". I took a handkerchief out of my pocket, but John Dawson ordered me to put it back. An Englishman dies with his eyes open. He wants to look death in the face.", "Elie Wiesel", "Dawn: A Novel"],
]);
    minutes.insert("04:59", &[
&["The whole place smells like death no matter what the fuck you do. Gately gets to the shelter at 0", "459", ".9h and just shuts his head off as if his head had a control switch.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("05:00", &[
&["", "Five o'clock", " had hardly struck on the morning of the 19th of January, when Bessie brought a candle into my closet and found me already up and nearly dressed.", "Charlotte Brontë", "Jane Eyre"],
&["", "Five o'clock", " had hardly struck on the morning of the 19th of January, when Bessie brought a candle into my closet and found me already up and nearly dressed. I had risen half-an-hour before her entrance, and had washed my face, and put on my clothes by the light of a half-moon just setting, whose rays streamed through the narrow window near my crib.", "Charlotte Brontë", "Jane Eyre"],
&["It was in the township of Dunwich, in a large and hardly inhabited farmhouse set against a hillside 4 miles from the village and a mile and a half from any other dwelling, that Wilbur Whately was born at ", "5 a.m.", " on Sunday, 2 February, 1913. The date was recalled because it was Candlemas, which people in Dunwich curiously observe under another name...", "H.P. Lovecraft", "The Dunwich Horror"],
&["Just after ", "five o'clock", " on this chill September morning, the fishmonger's cart, containing Kirsten and Emilia and such possessions as they have been able to assemble in the time allowed to them, is driven out of the gates of Rosenborg?", "Rose Tremain", "Music and Silence"],
&["The cold eye of the Duke was dazzled by the gleaming of a thousand jewels that sparkled on the table. His ears were filled with chiming as the clocks began to strike. \"One!\" said Hark. \"Two!\" cried Zorn of Zorna. \"Three!\" the Duke's voice almost whispered. 'Four!\" sighed Saralinda. \"", "Five", "!\" the Golux crowed, and pointed at the table. \"The task is done, the terms are met,\" he said.", "James Thurber", "The 13 Clocks"],
&["The day came slow, till ", "five o'clock", ". Then sprang before the hills. Like hindered rubies, or the light. A sudden musket spills", "Emily Dickinson", "The Day Came Slow, Till Five O' Clock"],
&["There are worse things than having behaved foolishly in public. There are worse things than these miniature betrayals, committed or endured or suspected; there are worse things than not being able to sleep for thinking about them. It is ", "5 a.m.", " All the worse things come stalking in and stand icily about the bed looking worse and worse and worse.", "Fleur Adcock", "Things"],
&["What causes young people to \"come out,\" but the noble ambition of matrimony? What sends them trooping to watering-places? What keeps them dancing till ", "five o'clock", " in the morning through a whole mortal season?", "William Makepeace Thackeray", "Vanity Fair"],
]);
    minutes.insert("05:01", &[
&["\"Oh yes. His clocks were set at ", "one minute past five", ", four minutes past five and seven minutes past five. That was the combination number of a safe, 515457. The safe was concealed behind a reproduction of the Mona Lisa. Inside the safe,\" continued Poirot, with distaste, \"were the Crown Jewels of the Russian Royal Family.\"", "Agatha Christie", "The clocks"],
&["Just ", "after five o'clock", " on this chill September morning, the fishmonger's cart, containing Kirsten and Emilia and such possessions as they have been able to assemble in the time allowed to them, is driven out of the gates of Rosenborg?", "Rose Tremain", "Music and Silence"],
]);
    minutes.insert("05:02", &[
&["It was ", "5:02 a.m.", ", December 14. In another fifty-eight minutes he would set sail for America. He did not want to leave his bride; he did not want to go.", "Brenda Joyce", "The Prize"],
]);
    minutes.insert("05:03", &[
&["It was ", "5:03 a.m.", " It didn't matter. She wasn't going to get back to sleep. She threw off her covers and, swearing at herself, Caleb and Mr. Griffin, she headed into the shower.", "Heather Graham", "Unhallowed ground"],
]);
    minutes.insert("05:04", &[
&["\"Oh yes. His clocks were set at one minute past five, ", "four minutes past five", " and seven minutes past five. That was the combination number of a safe, 515457. The safe was concealed behind a reproduction of the Mona Lisa. Inside the safe,\" continued Poirot, with distaste, \"were the Crown Jewels of the Russian Royal Family.\"", "Agatha Christie", "The clocks"],
&["", "5.04 a.m.", " on the substandard clock radio. Because why do people always say the day starts now? Really it starts in the middle of the night at a fraction of a second past midnight.", "Ali Smith", "The Accidental"],
&["Oh yes. His clocks were set at one minute past five, ", "four minutes past five", " and seven minutes past five. That was the combination number of a safe, 515457. The safe was concealed behind a reproduction of the Mona Lisa. Inside the safe, continued Poirot, with distaste, \"were the Crown Jewels of the Russian Royal Family.\"", "Agatha Christie", "The Clocks"],
]);
    minutes.insert(
        "05:05",
        &[&[
            "The baby, a boy, is born at ",
            "five past five",
            " in the morning.",
            "Jhumpa Lahiri",
            "The Namesake",
        ]],
    );
    minutes.insert("05:06", &[
&["", "5:06 a.m.", " I wake up strangely energized, my stomach growling. Upstairs, the overstocked fridge offers me its bounty of sympathy food.", "Jonathon Tropper", "This is Where I Leave you"],
]);
    minutes.insert("05:07", &[
&["\"Oh yes. His clocks were set at one minute past five, four minutes past five and ", "seven minutes past five", ". That was the combination number of a safe, 515457. The safe was concealed behind a reproduction of the Mona Lisa. Inside the safe,\" continued Poirot, with distaste, \"were the Crown Jewels of the Russian Royal Family.\"", "Agatha Christie", "The clocks"],
]);
    minutes.insert(
        "05:08",
        &[&[
            "Ambrose and I will marry at Fort McHenry at ",
            "5:08",
            " EDST this coming Saturday, Rosh Hashanah!",
            "John Barth",
            "Letters",
        ]],
    );
    minutes.insert("05:09", &[
&["The primal flush of triumph which had saturated the American's humor on this signal success, proved but fictive and transitory when inquiry of the station attendants educed the information that the two earliest trains to be obtained were the ", "5:09", " to Dunkerque and the 5:37 for Ostend.", "Louis Joseph Vance", "The Black Bag"],
]);
    minutes.insert("05:10", &[
&["\"Oh, my husband, I have done the deed which will relieve you of the wife whom you hate! I have taken the poison--all of it that was left in the paper packet, which was the first that I found. If this is not enough to kill me, I have more left in the bottle. ", "Ten minutes past five", ". \"You have just gone, after giving me my composing draught. My courage failed me at the sight of you. I thought to myself, 'If he look at me kindly, I will confess what I have done, and let him save my life.' You never looked at me at all. You only looked at the medicine. I let you go without saying a word.", "Wilkie Collins", "The Law and the Lady"],
&["I settled into a daily routine. Wake up at 4:40am, shower, get on the train north by ", "ten after five", ".", "Tina Fey", "Bossypants"],
]);
    minutes.insert(
        "05:11",
        &[&[
            "Today was Tuesday, the fifteenth of August; the sun had risen at ",
            "eleven minutes past five",
            " this morning and would set at two minutes before seven this evening.",
            "Donald E Westlake",
            "The Hot Rock",
        ]],
    );
    minutes.insert("05:12", &[
&["At ", "twelve minutes and six seconds past five o'clock", " on the morning of April 18th, 1906, the San francisco peninsula began to shiver in the grip of an earthquake which, when its ultimate consequences are considered, was the most disastrous in the recorded history of the North American continent.", "Herbert Asbury", "Slummer's Paradise"],
]);
    minutes.insert("05:13", &[
&["Lying on my side in bed, I stared at my alarm clock until it became a blemish, its red hue glowing like a welcome sign beckoning me into the depths of hell's crimson-colored cavities. ", "5:13 am.", " To describe this Monday as a blue Monday was an understatement.", "Nakia D Johnson", "Uptempo"],
]);
    minutes.insert("05:14", &[
&["The time was ", "5.14am", ", a very strange time indeed for the sheriff to have seen what he claimed he saw as he made his early-morning rounds, first patrolling back and forth along the deserted, snowbound streets of Kingdom City before extending his vigilance northward, along County Road.", "Thomas H Cook", "Into the Web"],
]);
    minutes.insert("05:15", &[
&["By the first week of May, Ralph was waking up to birdsong at ", "5:15 a.m.", " He tried earplugs for a few nights, although he doubted from the outset that they would work. It wasn’t the newly returned birds that were waking him up, nor the occasional delivery-truck backfire out on Harris Avenue. He had always been the sort of guy who could sleep in the middle of a brass marching bad, and he didn’t think that had changed. What had changed was inside his head.", "Stephen King", "Insomnia"],
&["Weird conversation with Brown, a tired & confused old man who's been jerked out of bed at ", "5:15", ".", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:16", &[
&["", "5:16", " - Mank on phone to Secretary of State Brown: 'Mr Brown, we're profoundly disturbed about this situation in the 21st. We can't get a single result out of there.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["She could go back to sleep. But typical and ironic, she is completely awake. It is completely light outside now; you can see for miles. Except there is nothing to see here; trees and fields and that kind of thing. ", "5:16 a.m", " on the substandard clock radio. She is really awake.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("05:17", &[
&["", "5:16", " - Mank on phone to Secretary of State Brown: 'Mr Brown, we're profoundly disturbed about this situation in the 21st. We can't get a single result out of there.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["She could go back to sleep. But typical and ironic, she is completely awake. It is completely light outside now; you can see for miles. Except there is nothing to see here; trees and fields and that kind of thing. ", "5:16 a.m", " on the substandard clock radio. She is really awake.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("05:18", &[
&["", "5:16", " - Mank on phone to Secretary of State Brown: 'Mr Brown, we're profoundly disturbed about this situation in the 21st. We can't get a single result out of there.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["She could go back to sleep. But typical and ironic, she is completely awake. It is completely light outside now; you can see for miles. Except there is nothing to see here; trees and fields and that kind of thing. ", "5:16 a.m", " on the substandard clock radio. She is really awake.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("05:19", &[
&["", "5:16", " - Mank on phone to Secretary of State Brown: 'Mr Brown, we're profoundly disturbed about this situation in the 21st. We can't get a single result out of there.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["She could go back to sleep. But typical and ironic, she is completely awake. It is completely light outside now; you can see for miles. Except there is nothing to see here; trees and fields and that kind of thing. ", "5:16 a.m", " on the substandard clock radio. She is really awake.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("05:20", &[
&["He saw on the floor his cigarette reduced to a long thin cylinder of ash: it had smoked itself. It was ", "five twenty", ", dawn was breaking behind the shed of empty barrels, the thermometer pointed to 210 degrees.", "Primo Levi", "The Periodic Table"],
]);
    minutes.insert("05:21", &[
&["He saw on the floor his cigarette reduced to a long thin cylinder of ash: it had smoked itself. It was ", "five twenty", ", dawn was breaking behind the shed of empty barrels, the thermometer pointed to 210 degrees.", "Primo Levi", "The Periodic Table"],
]);
    minutes.insert("05:22", &[
&["He saw on the floor his cigarette reduced to a long thin cylinder of ash: it had smoked itself. It was ", "five twenty", ", dawn was breaking behind the shed of empty barrels, the thermometer pointed to 210 degrees.", "Primo Levi", "The Periodic Table"],
]);
    minutes.insert("05:23", &[
&["If I could count precisely to sixty between two passing orange minutes on her digital clock, starting at ", "5.23am", " and ending exactly as it melted into 5:24, then when she woke she would love me and not say this had been a terrible mistake.", "Arthur Phillips", "The Tragedy of Arthur"],
]);
    minutes.insert("05:24", &[
&["If I could count precisely to sixty between two passing orange minutes on her digital clock, starting at 523am. and ending exactly as it melted into ", "5:24", ", then when she woke she would love me and not say this had been a terrible mistake.", "Arthur Phillips", "The Tragedy of Arthur"],
]);
    minutes.insert(
        "05:25",
        &[&[
            "George's train home from New Street leaves at ",
            "5.25",
            ". On the return journey, there are rarely schoolboys.",
            "Julian Barnes",
            "Arthur and George",
        ]],
    );
    minutes.insert("05:26", &[
&["I think this is actually bump number 1,970. And the boy keeps plugging away at the same speed. There isn’t a sound from them. Not a moan. Poor them. Poor me. I look at the clock. ", "05:26", ".", "Hallgrímur Helgason", "101 Reykjavik"],
]);
    minutes.insert("05:27", &[
&["I think this is actually bump number 1,970. And the boy keeps plugging away at the same speed. There isn’t a sound from them. Not a moan. Poor them. Poor me. I look at the clock. ", "05:26", ".", "Hallgrímur Helgason", "101 Reykjavik"],
]);
    minutes.insert("05:28", &[
&["I pulled into the Aoyama supermarket parking garage at ", "five-twenty-eight", ". The sky to the east was getting light. I entered the store carrying my bag. Almost no one was in the place. A young clerk in a striped uniform sat reading a magazine; a woman of indeterminate age was buying a cartload of cans and instant food. I turned past the liquor display and went straight to the snack bar.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("05:29", &[
&["I pulled into the Aoyama supermarket parking garage at ", "five-twenty-eight", ". The sky to the east was getting light. I entered the store carrying my bag. Almost no one was in the place. A young clerk in a striped uniform sat reading a magazine; a woman of indeterminate age was buying a cartload of cans and instant food. I turned past the liquor display and went straight to the snack bar.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("05:30", &[
&["Gideon has been most unlike Gideon. As Walter Eastman is preoccupied himself, he has not had time, or more to the point, inclination, to notice aberrant behaviour. For instance, it is ", "half-past five", " in the summer morning. Young Chase's narrow bachelor bed has evidently been slept in, for it is rumpled in that barely disturbed way which can never be counterfeited. His jug's empty and there's grey water in the basin, cleanly boy. The window is open, admitting the salubrious sea-breeze. He doesn't smoke anyway. What an innocent room it is.", "Timothy Mo", "An Insular Possession"],
&["It was by this time ", "half-past five", ", and the sun was on the point of rising; but I found the kitchen still dark and silent. … The stillness of early morning slumbered everywhere .. the carriage horses stamped from time to time in their closed stables: all else was still.", "Charlotte Brontë", "Jane Eyre"],
&["On the day they were going to kill him, Santiago Nasar got up at ", "five-thirty", " in the morning to wait for the boat the bishop was coming on.", "Gabriel García Márquez", "Chronicle of a Death Foretold"],
]);
    minutes.insert("05:31", &[
&["", "5:31", " - Mank on phone to lawyer: 'Jesus, I think we gotta go in there and get those ballots! Impound 'em! Every damn one!'", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:32", &[
&["", "5:31", " - Mank on phone to lawyer: 'Jesus, I think we gotta go in there and get those ballots! Impound 'em! Every damn one!'", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:33", &[
&["", "5:31", " - Mank on phone to lawyer: 'Jesus, I think we gotta go in there and get those ballots! Impound 'em! Every damn one!'", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:34", &[
&["I asked \"What time is sunrise?”' A second's silence while the crestfallen Bush absorbed his rebuke, and then another voice answered: ‘", "Five-thirty-four", ", sir.'", "C.S. Forester", "The Commodore"],
]);
    minutes.insert("05:35", &[
&["", "5:35", " - All phones ringing now, the swing shift has shot the gap - now the others are waking up.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["I squinted at the clock. 'It says ", "twenty-five before six", ",' I said and rolled away from him.", "Luke Rhinehart", "the dice man"],
]);
    minutes.insert("05:36", &[
&["", "5:35", " - All phones ringing now, the swing shift has shot the gap - now the others are waking up.", "Hunter S Thompson", "Fear and Loathing: On the Campaign Trail '72"],
&["I squinted at the clock. 'It says ", "twenty-five before six", ",' I said and rolled away from him.", "Luke Rhinehart", "the dice man"],
]);
    minutes.insert(
        "05:37",
        &[&[
            "Richard glanced at the clock on the microwave - ",
            "5:37",
            " - almost twelve hours, almost one half-day since he'd dialed 911.",
            "AM Homes",
            "This Book Will Save Your Life",
        ]],
    );
    minutes.insert("05:38", &[
&["Kovac,’ said Johnny sleepily. It was very rare for the quantum computer and not Sol to wake him up. ‘What’s going on? What time is it?’ ‘Good morning, Johnny,’ said the ship. ‘It is ", "5.38 a.m.", "’ ‘What?’ said Johnny. ‘It’s Saturday.’ ‘I told you he wouldn’t like it,’ said Sol, presumably to Kovac. ‘It’s hardly a matter of likes or dislikes,’ said the computer. ‘I have information I deem important enough to pass on at the earliest opportunity – whatever time it is.’", "Keith Mansfield", "Johnny Mackintosh: Battle for Earth"],
]);
    minutes.insert("05:39", &[
&["Kovac,’ said Johnny sleepily. It was very rare for the quantum computer and not Sol to wake him up. ‘What’s going on? What time is it?’ ‘Good morning, Johnny,’ said the ship. ‘It is ", "5.38 a.m.", "’ ‘What?’ said Johnny. ‘It’s Saturday.’ ‘I told you he wouldn’t like it,’ said Sol, presumably to Kovac. ‘It’s hardly a matter of likes or dislikes,’ said the computer. ‘I have information I deem important enough to pass on at the earliest opportunity – whatever time it is.’", "Keith Mansfield", "Johnny Mackintosh: Battle for Earth"],
]);
    minutes.insert("05:40", &[
&["", "Twenty minutes to six", ". 'Rob's boys were already on the platform, barrows ready. The only thing that ever dared to be late around here was the train. Rob's boys were in fact Bill Bing, thirty, sucking a Woodbine, and Arthur, sixty, half dead.", "Bruce Robinson", "The Peculiar Memories of Thomas Penman"],
]);
    minutes.insert("05:41", &[
&["", "Twenty minutes to six", ". 'Rob's boys were already on the platform, barrows ready. The only thing that ever dared to be late around here was the train. Rob's boys were in fact Bill Bing, thirty, sucking a Woodbine, and Arthur, sixty, half dead.", "Bruce Robinson", "The Peculiar Memories of Thomas Penman"],
]);
    minutes.insert("05:42", &[
&["", "Twenty minutes to six", ". 'Rob's boys were already on the platform, barrows ready. The only thing that ever dared to be late around here was the train. Rob's boys were in fact Bill Bing, thirty, sucking a Woodbine, and Arthur, sixty, half dead.", "Bruce Robinson", "The Peculiar Memories of Thomas Penman"],
]);
    minutes.insert("05:43", &[
&["", "5.43", " - Mank on phone to 'Mary' in Washington; 'It now appears quite clear that we'll lead the state - without the 21st.'", "Hunter S. Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:44", &[
&["", "5.43", " - Mank on phone to 'Mary' in Washington; 'It now appears quite clear that we'll lead the state - without the 21st.'", "Hunter S. Thompson", "Fear and Loathing: On the Campaign Trail '72"],
]);
    minutes.insert("05:45", &[
&["At ", "5:45", " a power-transformer on a pole beside the abandoned Tracker Brothers’ Truck Depot exploded in a flash of purple light, spraying twisted chunks of metal onto the shingled roof.", "Stephen King", "IT"],
]);
    minutes.insert("05:46", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:47", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:48", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:49", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:50", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:51", &[
&["Herbert could feel nothing. He wrote a legal-sounding phrase to the effect that the sentence had been carried out at ", "5.46am", ", adding, 'without a snag'. The burial party had cursed him quietly as they'd hacked at the thick roots and tight soil.", "William Brodrick", "A Whispered Name"],
]);
    minutes.insert("05:52", &[
&["At ", "5.52am", " paramedics from the St. Petersburg Fire Department and SunStar Medic One ambulance service responded to a medical emergency call at 12201 Ninth Street North, St. Petersburg, apartment 2210.", "Mark Fuhrman", "Silent Witness"],
]);
    minutes.insert("05:53", &[
&["At ", "5.52am", " paramedics from the St. Petersburg Fire Department and SunStar Medic One ambulance service responded to a medical emergency call at 12201 Ninth Street North, St. Petersburg, apartment 2210.", "Mark Fuhrman", "Silent Witness"],
]);
    minutes.insert("05:54", &[
&["At ", "5.52am", " paramedics from the St. Petersburg Fire Department and SunStar Medic One ambulance service responded to a medical emergency call at 12201 Ninth Street North, St. Petersburg, apartment 2210.", "Mark Fuhrman", "Silent Witness"],
]);
    minutes.insert("05:55", &[
&["It was ", "5.55am", " and raining hard when I pedalled up to the bike stand just outside the forecourt of the station and dashed inside. I raced past the bookstall, where all the placards of the Yorkshire Post (a morning paper) read 'York Horror', but also 'Terrific February Gales at Coast'.", "Andrew Martin", "The Lost Luggage Porter"],
]);
    minutes.insert("05:56", &[
&["It was ", "5.55am", " and raining hard when I pedalled up to the bike stand just outside the forecourt of the station and dashed inside. I raced past the bookstall, where all the placards of the Yorkshire Post (a morning paper) read 'York Horror', but also 'Terrific February Gales at Coast'.", "Andrew Martin", "The Lost Luggage Porter"],
]);
    minutes.insert("05:57", &[
&["It was ", "5.55am", " and raining hard when I pedalled up to the bike stand just outside the forecourt of the station and dashed inside. I raced past the bookstall, where all the placards of the Yorkshire Post (a morning paper) read 'York Horror', but also 'Terrific February Gales at Coast'.", "Andrew Martin", "The Lost Luggage Porter"],
]);
    minutes.insert(
        "05:58",
        &[&[
            "Annika Giannini woke with a start. She saw that it was ",
            "5.58 a.m.",
            "",
            "Stieg Larsson",
            "The Girl who Kicked the Hornet's Nest",
        ]],
    );
    minutes.insert(
        "05:59",
        &[&[
            "Annika Giannini woke with a start. She saw that it was ",
            "5.58 a.m.",
            "",
            "Stieg Larsson",
            "The Girl who Kicked the Hornet's Nest",
        ]],
    );
    minutes.insert("06:00", &[
&["‘What’s the time?’ I ask, and telling him so that he knows, ‘My mother likes “peace and quiet” to sleep late on Saturday mornings.’ ‘She does, does she? It’s ", "six o’clock", ". I couldn’t sleep,’ he says wearily, like an afterthought, as if it’s what he expects. ‘Why are you up so early?’ ‘I woke up and needed my panda. I can’t find him.’ ‘Where do you think he can be?’ His face changes and he smiles again, bending down to look under the table and behind the curtain. But he isn’t clowning or teasing. He’s in earnest.", "Patsy Hickman", "The Saints"],
&["But every morning, even if there's been a nighttime session and he has only slept two hours, he gets up ", "at six", " and reads his paper while he drinks a strong cup of coffee. In this way Papa constructs himself every day.", "Muriel Barbery", "The Elegance of the Hedgehog"],
&["I had risen half-an-hour before her entrance, and had washed my face, and put on my clothes by the light of a half-moon just setting, whose rays streamed through the narrow window near my crib. I was to leave Gateshead that day by a coach which passed the lodge gates ", "at six a.m.", "", "Charlotte Brontë", "Jane Eyre"],
&["Lying awake in my attic room, i hear a clock strike ", "six", " downstairs. It was fairly light and people were beginning to walk up and down the stairs...- i heard the clock strike eight downstairs before i rose and got dressed... I looked up - the clock tower of our saviour's showed ten.", "Knut Hamsun", "Hunger"],
&["On the 15th of September 1840, about ", "six o'clock", " in the morning, the Ville-de-Montereau, ready to depart, pouring out great whirls of smoke by the quai Saint-Bernard.", "Gustave Flaubert", "L'Education sentimentale"],
&["Rise from bed ............... . ", "6.00 A.M.", "", "F. Scott Fitzgerald", "The Great Gatsby"],
&["The ball went on for a long time, until ", "six", " in the morning; all were exhausted and wishing they had been in bed for at least three hours; but to leave early was like proclaiming the party a failure and offending the host and hostess who had taken such a lot of trouble, poor dears.", "Giuseppe Tomasi di Lampedusa", "The Leopard"],
]);
    minutes.insert("06:01", &[
&["‘What’s the time?’ I ask, and telling him so that he knows, ‘My mother likes “peace and quiet” to sleep late on Saturday mornings.’ ‘She does, does she? It’s ", "six o’clock", ". I couldn’t sleep,’ he says wearily, like an afterthought, as if it’s what he expects. ‘Why are you up so early?’ ‘I woke up and needed my panda. I can’t find him.’ ‘Where do you think he can be?’ His face changes and he smiles again, bending down to look under the table and behind the curtain. But he isn’t clowning or teasing. He’s in earnest.", "Patsy Hickman", "The Saints"],
&["But every morning, even if there's been a nighttime session and he has only slept two hours, he gets up ", "at six", " and reads his paper while he drinks a strong cup of coffee. In this way Papa constructs himself every day.", "Muriel Barbery", "The Elegance of the Hedgehog"],
&["I had risen half-an-hour before her entrance, and had washed my face, and put on my clothes by the light of a half-moon just setting, whose rays streamed through the narrow window near my crib. I was to leave Gateshead that day by a coach which passed the lodge gates ", "at six a.m.", "", "Charlotte Brontë", "Jane Eyre"],
&["Lying awake in my attic room, i hear a clock strike ", "six", " downstairs. It was fairly light and people were beginning to walk up and down the stairs...- i heard the clock strike eight downstairs before i rose and got dressed... I looked up - the clock tower of our saviour's showed ten.", "Knut Hamsun", "Hunger"],
&["On the 15th of September 1840, about ", "six o'clock", " in the morning, the Ville-de-Montereau, ready to depart, pouring out great whirls of smoke by the quai Saint-Bernard.", "Gustave Flaubert", "L'Education sentimentale"],
&["Rise from bed ............... . ", "6.00 A.M.", "", "F. Scott Fitzgerald", "The Great Gatsby"],
&["The ball went on for a long time, until ", "six", " in the morning; all were exhausted and wishing they had been in bed for at least three hours; but to leave early was like proclaiming the party a failure and offending the host and hostess who had taken such a lot of trouble, poor dears.", "Giuseppe Tomasi di Lampedusa", "The Leopard"],
]);
    minutes.insert("06:02", &[
&["Bimingham New Street 5.25. Walsall 5.55. This train does not stop at Birchills, for reasons George has never been able to ascertain. Then it is Bloxwich ", "6.02", ", Wyrley & Churchbridge 6.09. At 6.10 he nods to Mr Merriman the stationmaster.", "Julian Barnes", "Arthur and George"],
]);
    minutes.insert("06:03", &[
&["Bimingham New Street 5.25. Walsall 5.55. This train does not stop at Birchills, for reasons George has never been able to ascertain. Then it is Bloxwich ", "6.02", ", Wyrley & Churchbridge 6.09. At 6.10 he nods to Mr Merriman the stationmaster.", "Julian Barnes", "Arthur and George"],
]);
    minutes.insert("06:04", &[
&["Bimingham New Street 5.25. Walsall 5.55. This train does not stop at Birchills, for reasons George has never been able to ascertain. Then it is Bloxwich ", "6.02", ", Wyrley & Churchbridge 6.09. At 6.10 he nods to Mr Merriman the stationmaster.", "Julian Barnes", "Arthur and George"],
]);
    minutes.insert(
        "06:05",
        &[&[
            "A second man went in and found the shop empty, as he thought, at ",
            "five minutes past six",
            ". That puts the time at between 5:30 and 6:05.",
            "Agatha Christie",
            "The ABC Murders",
        ]],
    );
    minutes.insert("06:06", &[
&["At ", "6:06", ", every toilet on Merit Street suddenly exploded in a geyser of shit and raw sewage as some unimaginable reversal took place in the pipes which fed the holding tanks of the new waste-treatment plant in the Barrens.", "Stephen King", "IT"],
]);
    minutes.insert("06:07", &[
&["At ", "6:06", ", every toilet on Merit Street suddenly exploded in a geyser of shit and raw sewage as some unimaginable reversal took place in the pipes which fed the holding tanks of the new waste-treatment plant in the Barrens.", "Stephen King", "IT"],
]);
    minutes.insert("06:08", &[
&["At ", "six oh-eight a.m.", " two men wearing ragged trench coats approached the Casino. The shorter of the men burst into flames.", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("06:09", &[
&["At ", "six oh-eight a.m.", " two men wearing ragged trench coats approached the Casino. The shorter of the men burst into flames.", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("06:10", &[
&["The bus left the station at ", "ten past six", " - and she sat proud, like an accustomed traveller, apart from her father, John Henry, and Berenice. But after a while a serious doubt came in her, which even the answers of the bus-driver could not quite satisfy.", "Carson McCullers", "The Member of the Wedding"],
]);
    minutes.insert("06:11", &[
&["The bus left the station at ", "ten past six", " - and she sat proud, like an accustomed traveller, apart from her father, John Henry, and Berenice. But after a while a serious doubt came in her, which even the answers of the bus-driver could not quite satisfy.", "Carson McCullers", "The Member of the Wedding"],
]);
    minutes.insert("06:12", &[
&["The bus left the station at ", "ten past six", " - and she sat proud, like an accustomed traveller, apart from her father, John Henry, and Berenice. But after a while a serious doubt came in her, which even the answers of the bus-driver could not quite satisfy.", "Carson McCullers", "The Member of the Wedding"],
]);
    minutes.insert("06:13", &[
&["It's ", "06:13", " .........Ma says I ought to be wrapped up in Rug already, Old Nick might possibly come.", "Emma Donoghue", "Room"],
]);
    minutes.insert("06:14", &[
&["It's ", "06:13", " .........Ma says I ought to be wrapped up in Rug already, Old Nick might possibly come.", "Emma Donoghue", "Room"],
]);
    minutes.insert("06:15", &[
&["Dumbbell exercise and wall-scaling ..... . ", "6.15", "-6.30", "F. Scott Fitzgerald", "The Great Gatsby"],
&["Father expected his shaving-water to be ready at a ", "quarter past six", ". Just seven minutes late, Dorothy took the can upstairs and knocked at her father's door.", "George Orwell", "A Clergyman's Daughter"],
&["It was ", "6.15 am.", " Just starting to get light. A small knot of older teenagers were leaning against a nearby wall. They looked as though they had been out all night.Two of the guys stared at us. Their eyes hard and threatening.", "Sophie McKenzie", "Girl Missing"],
&["It was ", "6.15 am.", " Just starting to get light. A small knot of older teenagers were leaning against a nearby wall. They looked as though they had been out all night.Two of the guys stared at us. Their eyes hard and threatening.", "Sophie McKenzie", "Girl Missing"],
]);
    minutes.insert("06:16", &[
&["Dumbbell exercise and wall-scaling ..... . ", "6.15", "-6.30", "F. Scott Fitzgerald", "The Great Gatsby"],
&["Father expected his shaving-water to be ready at a ", "quarter past six", ". Just seven minutes late, Dorothy took the can upstairs and knocked at her father's door.", "George Orwell", "A Clergyman's Daughter"],
&["It was ", "6.15 am.", " Just starting to get light. A small knot of older teenagers were leaning against a nearby wall. They looked as though they had been out all night.Two of the guys stared at us. Their eyes hard and threatening.", "Sophie McKenzie", "Girl Missing"],
&["It was ", "6.15 am.", " Just starting to get light. A small knot of older teenagers were leaning against a nearby wall. They looked as though they had been out all night.Two of the guys stared at us. Their eyes hard and threatening.", "Sophie McKenzie", "Girl Missing"],
]);
    minutes.insert("06:17", &[
&["Dizzy, come on.' He turned slowly, coaxing the animal down on to the pillow. The clock read ", "six-seventeen", ". A second cat, Miles, purred on contentedly from the patch in the covers where Resnick's legs had made a deep V.", "John Harvey", "Lonely Hearts"],
]);
    minutes.insert("06:18", &[
&["Dizzy, come on.' He turned slowly, coaxing the animal down on to the pillow. The clock read ", "six-seventeen", ". A second cat, Miles, purred on contentedly from the patch in the covers where Resnick's legs had made a deep V.", "John Harvey", "Lonely Hearts"],
]);
    minutes.insert(
        "06:19",
        &[&[
            "",
            "6.19 am",
            ", 8th June 2004, the jet of your pupil set in the gold of your eye.",
            "Carol Ann Duffy",
            "Venus",
        ]],
    );
    minutes.insert("06:20", &[
&["It was ", "6:20 a.m.", ", and my parents and I were standing, stunned and haf-awake, in the parking lot of a Howard Johnson's in Iowa.", "Austin Grossman", "Soon I Will Be Invincible"],
]);
    minutes.insert("06:21", &[
&["It was ", "6:20 a.m.", ", and my parents and I were standing, stunned and haf-awake, in the parking lot of a Howard Johnson's in Iowa.", "Austin Grossman", "Soon I Will Be Invincible"],
]);
    minutes.insert("06:22", &[
&["It was ", "6:20 a.m.", ", and my parents and I were standing, stunned and haf-awake, in the parking lot of a Howard Johnson's in Iowa.", "Austin Grossman", "Soon I Will Be Invincible"],
]);
    minutes.insert("06:23", &[
&["It was ", "6:20 a.m.", ", and my parents and I were standing, stunned and haf-awake, in the parking lot of a Howard Johnson's in Iowa.", "Austin Grossman", "Soon I Will Be Invincible"],
]);
    minutes.insert("06:24", &[
&["It was ", "6:20 a.m.", ", and my parents and I were standing, stunned and haf-awake, in the parking lot of a Howard Johnson's in Iowa.", "Austin Grossman", "Soon I Will Be Invincible"],
]);
    minutes.insert("06:25", &[
&["Simon is happy to travel scum class when he's on his own and even sometimes deliberately aims for the ", "6.25", ". But today the .25 is delayed to 6.44.", "Mark Lawson", "The Deaths"],
&["Still, it's your consciousness that's created it. Not somethin' just anyone could do. Others could be wanderin' around forever in who-knows-what contradictory chaos of a world. You're different. You seem t'be the immortal type.\" \"When's the turnover into that world going to take place?\" asked the chubby girl. The Professor looked at his watch. I looked at my watch. ", "Six-twenty-five", ". Well past daybreak. Morning papers delivered. \"According t'my estimates, in another twenty-nine hours and thirty-five minutes,\" said the Professor. \"Plus or minus forty-five minutes. I set it at twelve noon for easy reference. Noon tomorrow.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("06:26", &[
&["Simon is happy to travel scum class when he's on his own and even sometimes deliberately aims for the ", "6.25", ". But today the .25 is delayed to 6.44.", "Mark Lawson", "The Deaths"],
&["Still, it's your consciousness that's created it. Not somethin' just anyone could do. Others could be wanderin' around forever in who-knows-what contradictory chaos of a world. You're different. You seem t'be the immortal type.\" \"When's the turnover into that world going to take place?\" asked the chubby girl. The Professor looked at his watch. I looked at my watch. ", "Six-twenty-five", ". Well past daybreak. Morning papers delivered. \"According t'my estimates, in another twenty-nine hours and thirty-five minutes,\" said the Professor. \"Plus or minus forty-five minutes. I set it at twelve noon for easy reference. Noon tomorrow.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("06:27", &[
&["", "06:27", ":52 by the chip in her optic nerve; Case had been following her progress through Villa Straylight for over an hour, letting the endorphin analogue she'd taken blot out his hangover.", "William Gibson", "Neuromancer"],
&["Early in the morning, late in the century, Cricklewood Broadway. At ", "0627 hours", " on January 1, 1975, Alfred Archibald Jones was dressed in corduroy and sat in a fume-filled Cavalier Musketeer Estate, facedown on the steering wheel, hoping the judgment would not be too heavy upon him.", "Zadie Smith", "White Teeth"],
]);
    minutes.insert("06:28", &[
&["", "06:27", ":52 by the chip in her optic nerve; Case had been following her progress through Villa Straylight for over an hour, letting the endorphin analogue she'd taken blot out his hangover.", "William Gibson", "Neuromancer"],
&["Early in the morning, late in the century, Cricklewood Broadway. At ", "0627 hours", " on January 1, 1975, Alfred Archibald Jones was dressed in corduroy and sat in a fume-filled Cavalier Musketeer Estate, facedown on the steering wheel, hoping the judgment would not be too heavy upon him.", "Zadie Smith", "White Teeth"],
]);
    minutes.insert("06:29", &[
&["I sat up. There was a rug over me. I threw that off and got my feet on the floor. I scowled at a clock. The clock said ", "a minute short of six-thirty", ".", "Raymond Chandler", "The Big Sleep"],
]);
    minutes.insert("06:30", &[
&["Inside now MJ ordered. She pushed the three of us into the hotel room, thern shut the soor. I glanced at the clock by the bed. ", "6.30 am.", " Why were they waking Mum and Dad up this early?", "Sophie McKenzie", "Girl Missing"],
&["Daniel and the FBI men listened to the sounds of his mother waking up his father. Daniel still held the door-knob. He was ready to close the door the second he was told to.\"What time is it?\" said his father in a drugged voice. \"Oh my God, it's ", "six-thirty", ",\" his mother said.", "E.L. Doctorow", "The Book of Daniel"],
&["It was ", "six-thirty", ". When the baby's cry came, they could not pick it out, and Sam, eagerly thrusting his face amongst their ears, said, \"Listen, there, there, that's the new baby.\" He was red with delight and success.", "Christina Stead", "The Man Who Loved Children"],
&["It was very cold sitting in the truck and after a while he got out and walked around and flailed at himself with his arms and stamped his boots. Then he got back in the truck. The bar clock said ", "six-thirty", "...By eight-thirty he’d decided that it that was it would take to make the cab arrive then that’s what he would do and he started the engine.", "Cormac McCarthy", "Cities of the Plain"],
&["Nervously she jumped up and listened; the house itself was as still as ever; the footsteps had retreated. Through her wide-open window the brilliant rays of the morning sun were flooding her room with light. She looked up at the clock; it was ", "half-past six", "—too early for any of the household to be already astir.", "Baroness Orczy", "The Scarlet Pimpernel"],
&["", "Six-thirty", " was clearly a preposterous time and he, the client, obviously hadn't meant it seriously. A civilised six-thirty for twelve noon was almost certainly what he had in mind, and if he wanted to cut up rough about it, Dirk would have no option but to start handing out some serious statistics. Nobody got murdered before lunch. But nobody. People weren't up to it. You needed a good lunch to get both the blood-sugar and blood-lust levels up. Dirk had the figures to prove it.", "Douglas Adams", "The Long Dark Tea-time of the Soul"],
&["Sometimes they were hooded carts, sometimes they were just open carts, with planks for seats, on which sat twelve cloaked and bonneted women, six a side, squeezed together, for the interminable journey. As late as 1914 I knew the carrier of Croydon-cum-Clopton, twelve miles from Cambridge; his cart started at ", "6.30", " in the morning and got back at about ten at night. Though he was not old, he could neither read nor write; but he took commissions all along the road - a packet of needles for Mrs. This, and a new teapot for Mrs. That - and delivered them all correctly on the way back.", "Gwen Raverat", "Period Piece"],
]);
    minutes.insert("06:31", &[
&["Inside now MJ ordered. She pushed the three of us into the hotel room, thern shut the soor. I glanced at the clock by the bed. ", "6.30 am.", " Why were they waking Mum and Dad up this early?", "Sophie McKenzie", "Girl Missing"],
&["Daniel and the FBI men listened to the sounds of his mother waking up his father. Daniel still held the door-knob. He was ready to close the door the second he was told to.\"What time is it?\" said his father in a drugged voice. \"Oh my God, it's ", "six-thirty", ",\" his mother said.", "E.L. Doctorow", "The Book of Daniel"],
&["It was ", "six-thirty", ". When the baby's cry came, they could not pick it out, and Sam, eagerly thrusting his face amongst their ears, said, \"Listen, there, there, that's the new baby.\" He was red with delight and success.", "Christina Stead", "The Man Who Loved Children"],
&["It was very cold sitting in the truck and after a while he got out and walked around and flailed at himself with his arms and stamped his boots. Then he got back in the truck. The bar clock said ", "six-thirty", "...By eight-thirty he’d decided that it that was it would take to make the cab arrive then that’s what he would do and he started the engine.", "Cormac McCarthy", "Cities of the Plain"],
&["Nervously she jumped up and listened; the house itself was as still as ever; the footsteps had retreated. Through her wide-open window the brilliant rays of the morning sun were flooding her room with light. She looked up at the clock; it was ", "half-past six", "—too early for any of the household to be already astir.", "Baroness Orczy", "The Scarlet Pimpernel"],
&["", "Six-thirty", " was clearly a preposterous time and he, the client, obviously hadn't meant it seriously. A civilised six-thirty for twelve noon was almost certainly what he had in mind, and if he wanted to cut up rough about it, Dirk would have no option but to start handing out some serious statistics. Nobody got murdered before lunch. But nobody. People weren't up to it. You needed a good lunch to get both the blood-sugar and blood-lust levels up. Dirk had the figures to prove it.", "Douglas Adams", "The Long Dark Tea-time of the Soul"],
&["Sometimes they were hooded carts, sometimes they were just open carts, with planks for seats, on which sat twelve cloaked and bonneted women, six a side, squeezed together, for the interminable journey. As late as 1914 I knew the carrier of Croydon-cum-Clopton, twelve miles from Cambridge; his cart started at ", "6.30", " in the morning and got back at about ten at night. Though he was not old, he could neither read nor write; but he took commissions all along the road - a packet of needles for Mrs. This, and a new teapot for Mrs. That - and delivered them all correctly on the way back.", "Gwen Raverat", "Period Piece"],
]);
    minutes.insert("06:32", &[
&["The familiar radium numerals on my left wrist confirmed the clock tower. It was ", "twenty-eight minutes to seven", ". I seemed to be filling a set of loud maroon pajamas which were certainly not mine. My vis-a-vis was wearing a little number in yellow.", "Dana Chambers", "Too Like the Lightning"],
]);
    minutes.insert("06:33", &[
&["Woke ", "6.33 a.m.", " Last session with Anderson. He made it plain he's seen enough of me, and from now on I'm better alone. To sleep 8:00? (These count-downs terrify me.) He paused, then added: Goodbye, Eniwetok.", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("06:34", &[
&["Woke ", "6.33 a.m.", " Last session with Anderson. He made it plain he's seen enough of me, and from now on I'm better alone. To sleep 8:00? (These count-downs terrify me.) He paused, then added: Goodbye, Eniwetok.", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("06:35", &[
&["My watch lay on the dressing-table close by; glancing at it, I saw that the time was ", "twenty-five minutes to seven", ". I had been told that the family breakfasted at nine, so I had nearly two-and-a-half hours of leisure. Of course, I would go out, and enjoy the freshness of the morning.", "J.S. Fletcher", "Ravensdene Court"],
]);
    minutes.insert("06:36", &[
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke 9:05. To sleep ", "6:36", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("06:37", &[
&["The dashboard clock said ", "6.37am", " Town frowned, and checked his wristwatch, which blinked that it was 1.58pm. Great, he thought. I was either up on that tree for eight hours, or for minus a minute.", "Neil Gaiman", "American Gods"],
]);
    minutes.insert(
        "06:38",
        &[&[
            "The clock on the dashboard said it was ",
            "6.38am.",
            " He left the keys in the car, and walked toward the tree.",
            "Neil Gaiman",
            "American Gods",
        ]],
    );
    minutes.insert(
        "06:39",
        &[&[
            "The clock on the dashboard said it was ",
            "6.38am.",
            " He left the keys in the car, and walked toward the tree.",
            "Neil Gaiman",
            "American Gods",
        ]],
    );
    minutes.insert("06:40", &[
&["At eleven o'clock the phone rang, and still the figure did not respond, any more than it has responded when the phone had rung at twenty-five to seven in the morning, and again at ", "twenty to seven", "", "Douglas Adams", "The Long Dark Tea-Time of the Soul"],
]);
    minutes.insert("06:41", &[
&["At eleven o'clock the phone rang, and still the figure did not respond, any more than it has responded when the phone had rung at twenty-five to seven in the morning, and again at ", "twenty to seven", "", "Douglas Adams", "The Long Dark Tea-Time of the Soul"],
]);
    minutes.insert("06:42", &[
&["At eleven o'clock the phone rang, and still the figure did not respond, any more than it has responded when the phone had rung at twenty-five to seven in the morning, and again at ", "twenty to seven", "", "Douglas Adams", "The Long Dark Tea-Time of the Soul"],
]);
    minutes.insert("06:43", &[
&["To London on the ", "6.43am.", " Jessica is back from her holiday. Things are looking up, she called me Chris, instead of Minister, when we talked on the phone this afternoon.", "Chris Mullin", "A View From the Foothills"],
]);
    minutes.insert("06:44", &[
&["Simon is happy to travel scum class when he's on his own and even sometimes deliberately aims for the 6.25. But today the .25 is delayed to ", "6.44", ".", "Mark Lawson", "The Deaths"],
]);
    minutes.insert("06:45", &[
&["As the clock pointed to a ", "quarter to seven", ", the dog woke and shook himself. After waiting in vain for the footman, who was accustomed to let him out, the animal wandered restlessly from one closed door to another on the ground floor; and, returning to his mat in great perplexity, appealed to the sleeping family, with a long and melancholy howl.'", "Wilkie Collins", "No Name"],
&["He was still hurriedly thinking all this through, unable to decide to get out of the bed, when the clock struck ", "quarter to seven", ". There was a cautious knock at the door near his head. \"Gregor\", somebody called - it was his mother - \"it's quarter to seven. Didn't you want to go somewhere?\"", "Franz Kafka", "Metamorphosis"],
]);
    minutes.insert("06:46", &[
&["At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth.", "John Buchan", "The Thirty-Nine Steps"],
&["Then I hung about in the hall waiting for the milkman. That was the worst part of the business, for I was fairly choking to get out of doors. Six-thirty passed, then six-forty, but still he did not come. The fool had chosen this day of all days to be late. At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth. He jumped a bit at the sight of me.", "John Buchan", "The Thirty-Nine Steps"],
]);
    minutes.insert("06:47", &[
&["At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth.", "John Buchan", "The Thirty-Nine Steps"],
&["Then I hung about in the hall waiting for the milkman. That was the worst part of the business, for I was fairly choking to get out of doors. Six-thirty passed, then six-forty, but still he did not come. The fool had chosen this day of all days to be late. At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth. He jumped a bit at the sight of me.", "John Buchan", "The Thirty-Nine Steps"],
]);
    minutes.insert("06:48", &[
&["At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth.", "John Buchan", "The Thirty-Nine Steps"],
&["Then I hung about in the hall waiting for the milkman. That was the worst part of the business, for I was fairly choking to get out of doors. Six-thirty passed, then six-forty, but still he did not come. The fool had chosen this day of all days to be late. At ", "one minute after the quarter to seven", " I heard the rattle of the cans outside. I opened the front door, and there was my man, singling out my cans from a bunch he carried and whistling through his teeth. He jumped a bit at the sight of me.", "John Buchan", "The Thirty-Nine Steps"],
]);
    minutes.insert(
        "06:49",
        &[&[
            "Night ends, ",
            "6:49",
            ". Meet in the coffee shop at 7:30; press conference at 10:00.",
            "Hunter S. Thompson",
            "Fear and Loathing: On the Campaign Trail '72",
        ]],
    );
    minutes.insert("06:50", &[
&["Will, my fiancé, was coming from Boston on the ", "six-fifty", " train - the dawn train, the only train that still stopped in the small Ohio city where I lived.", "Mary Robison", "Pretty Ice"],
]);
    minutes.insert("06:51", &[
&["Will, my fiancé, was coming from Boston on the ", "six-fifty", " train - the dawn train, the only train that still stopped in the small Ohio city where I lived.", "Mary Robison", "Pretty Ice"],
]);
    minutes.insert("06:52", &[
&["Will, my fiancé, was coming from Boston on the ", "six-fifty", " train - the dawn train, the only train that still stopped in the small Ohio city where I lived.", "Mary Robison", "Pretty Ice"],
]);
    minutes.insert("06:53", &[
&["Will, my fiancé, was coming from Boston on the ", "six-fifty", " train - the dawn train, the only train that still stopped in the small Ohio city where I lived.", "Mary Robison", "Pretty Ice"],
]);
    minutes.insert("06:54", &[
&["Will, my fiancé, was coming from Boston on the ", "six-fifty", " train - the dawn train, the only train that still stopped in the small Ohio city where I lived.", "Mary Robison", "Pretty Ice"],
]);
    minutes.insert("06:55", &[
&["At ", "6:55 am", " Lisa parked and took the lift from the frozen underground car park up to level 1 of Green Oaks Shopping Centre.", "Catherine O'Flynn", "What was Lost"],
]);
    minutes.insert("06:56", &[
&["At ", "6:55 am", " Lisa parked and took the lift from the frozen underground car park up to level 1 of Green Oaks Shopping Centre.", "Catherine O'Flynn", "What was Lost"],
]);
    minutes.insert("06:57", &[
&["At ", "6:55 am", " Lisa parked and took the lift from the frozen underground car park up to level 1 of Green Oaks Shopping Centre.", "Catherine O'Flynn", "What was Lost"],
]);
    minutes.insert("06:58", &[
&["At ", "6:55 am", " Lisa parked and took the lift from the frozen underground car park up to level 1 of Green Oaks Shopping Centre.", "Catherine O'Flynn", "What was Lost"],
]);
    minutes.insert("06:59", &[
&["It was ", "6.59 a.m.", " on Maundy Thursday as Blomkvist and Berger let themselves into the \"Millennium\" offices.", "Stieg Larsson", "The Girl who Played with Fire"],
]);
    minutes.insert("07:00", &[
&["\"", "Seven o'clock", ", already\", he said to himself when the clock struck again, \"seven o'clock, and there's still a fog like this.\"", "Franz Kafka", "Metamorphosis"],
&["At ", "seven o’clock", " in the morning, Rubashov was awakened by a bugle, but he did not get up. Soon he heard sounds in the corridor. He imagined that someone was to be tortured, and he dreaded hearing the first screams of pain. When the footsteps reached his own section, he saw through the eye hole that guards were serving breakfast. Rubashov did not receive any breakfast because he had reported himself ill. He began to pace up and down the cell, six and a half steps to the window, six and a half steps back.", "Arthur Koestler", "Darkness at Noon"],
&["I had left directions that I was to be called ", "at seven", "; for it was plain that I must see Wemmick before seeing any one else, and equally plain that this was a case in which his Walworth sentiments, only, could be taken. It was a relief to get out of the room where the night had been so miserable, and I needed no second knocking at the door to startle me from my uneasy bed.", "Charles Dickens", "Great Expectations"],
&["She locked herself in, made no reply to my bonjour through the door; she was up at ", "seven o'clock", ", the samovar was taken in to her from the kitchen.", "Fyodor Dostoyevsky", "Crime and Punishment"],
]);
    minutes.insert("07:01", &[
&["\"", "Seven o'clock", ", already\", he said to himself when the clock struck again, \"seven o'clock, and there's still a fog like this.\"", "Franz Kafka", "Metamorphosis"],
&["At ", "seven o’clock", " in the morning, Rubashov was awakened by a bugle, but he did not get up. Soon he heard sounds in the corridor. He imagined that someone was to be tortured, and he dreaded hearing the first screams of pain. When the footsteps reached his own section, he saw through the eye hole that guards were serving breakfast. Rubashov did not receive any breakfast because he had reported himself ill. He began to pace up and down the cell, six and a half steps to the window, six and a half steps back.", "Arthur Koestler", "Darkness at Noon"],
&["I had left directions that I was to be called ", "at seven", "; for it was plain that I must see Wemmick before seeing any one else, and equally plain that this was a case in which his Walworth sentiments, only, could be taken. It was a relief to get out of the room where the night had been so miserable, and I needed no second knocking at the door to startle me from my uneasy bed.", "Charles Dickens", "Great Expectations"],
&["She locked herself in, made no reply to my bonjour through the door; she was up at ", "seven o'clock", ", the samovar was taken in to her from the kitchen.", "Fyodor Dostoyevsky", "Crime and Punishment"],
]);
    minutes.insert(
        "07:02",
        &[&[
            "",
            "07:02",
            ":18 One and a half hours. 'Case,' she said, 'I wanna favour.'",
            "William Gibson",
            "Neuromancer",
        ]],
    );
    minutes.insert(
        "07:03",
        &[&[
            "",
            "7:03am",
            " General Tanz woke up as though aroused by a mental alarm-clock.",
            "Hans Hellmut Kirst",
            "The Night of the Generals",
        ]],
    );
    minutes.insert("07:04", &[
&["Sunday evening at almost the same hour (to be precise, at about ", "7:04 p.m.", ") she rings the front door bell at the home of Walter Moeding, Crime Commissioner, who is at that moment engaged, for professional rather than private reasons, in disguising himself as a sheikh.", "Heinrich Böll", "The Lost Honour of Katharina Blum"],
]);
    minutes.insert("07:05", &[
&["He really couldn't believe that the old woman who'd phoned him last night would show up this morning, as she'd said she would. He decided he'd wait until ", "five minutes after seven o'clock", ", and then he'd call in, take the day off, and make every effort in the book to locate someone reliable.", "Raymond Carver", "Where I'm Calling From"],
&["Outside my window the sky hung low and gray. It looked like snow, which added to my malaise. The clock read ", "five after seven", ". I punched the remote control and watched the morning news as I lay in bed.", "Haruki Murakami", "Dance Dance Dance"],
&["Ryan missed the dawn. He boarded a TWA 747 that left Dulles on time, at ", "7:05 A.M.", " The sky was overcast, and when the aircraft burst through the cloud layer into sunlight, Ryan did something he had never done before. For the first time in his life, Jack Ryan fell asleep on an airplane.", "Tom Clancy", "The Hunt for Red October"],
]);
    minutes.insert("07:06", &[
&["So far so good. There followed a little passage of time when we stood by the duty desk, drinking coffee and studiously not mentioning what we were all thinking and hoping: that Percy was late, that maybe Percy wasn't going to show up at all. Considering the hostile reviews he'd gotten on the way he'd handled the electrocution, that seemed at least possible. But Percy subscribed to that old axiom about how you should get right back on the horse that had thrown you, because here he came through the door at ", "six minutes past seven", ", resplendent in his blue uniform with his sidearm on one hip and his hickory stick in its ridiculous custom-made holster on the other.", "Stephen King", "The Green Mile"],
&["Percy subscribed to that old axiom about how you should get right back on the horse that had thrown you, because here he came through the door at ", "six minutes past seven", ", resplendent in his blue uniform with his sidearm on one hip and his hickory stick in its ridiculous custom-made holster on the other.", "Stephen King", "The Green Mile"],
]);
    minutes.insert("07:07", &[
&["So far so good. There followed a little passage of time when we stood by the duty desk, drinking coffee and studiously not mentioning what we were all thinking and hoping: that Percy was late, that maybe Percy wasn't going to show up at all. Considering the hostile reviews he'd gotten on the way he'd handled the electrocution, that seemed at least possible. But Percy subscribed to that old axiom about how you should get right back on the horse that had thrown you, because here he came through the door at ", "six minutes past seven", ", resplendent in his blue uniform with his sidearm on one hip and his hickory stick in its ridiculous custom-made holster on the other.", "Stephen King", "The Green Mile"],
&["Percy subscribed to that old axiom about how you should get right back on the horse that had thrown you, because here he came through the door at ", "six minutes past seven", ", resplendent in his blue uniform with his sidearm on one hip and his hickory stick in its ridiculous custom-made holster on the other.", "Stephen King", "The Green Mile"],
]);
    minutes.insert(
        "07:08",
        &[&[
            "Reacher had no watch but he figured when he saw Gregory it must have been ",
            "between eight and nine minutes after seven o'clock",
            ".",
            "Lee Child",
            "The Hard Way",
        ]],
    );
    minutes.insert("07:09", &[
&["In the living room the voice-clock sang, Tick-tock, seven o'clock, time to get up, time to get up, seven o 'clock! as if it were afraid that nobody would. The morning house lay empty. The clock ticked on, repeating and repeating its sounds into the emptiness. ", "Seven-nine", ", breakfast time, seven-nine!", "Ray Bradbury", "There Will Come Soft Rains"],
&["", "Seven-nine", ", breakfast time, seven-nine!", "Ray Bradbury", "There Will Come Soft Rains"],
]);
    minutes.insert("07:10", &[
&["A search in Bradshaw informed me that a train left St Pancras at ", "7.10", ", which would land me at any Galloway station in the late afternoon.", "John Buchan", "The Thirty-Nine Steps"],
&["There were many others waiting to execute the same operation, so she would have to move fast, elbow her way to the front so that she emerged first. The time was ", "7:10", " in the morning. The manoeuvre would start at 7:12. She looked apprehensively at the giant clock at the railway station.", "Mini Nair", "The Fourth Passenger"],
]);
    minutes.insert("07:11", &[
&["A search in Bradshaw informed me that a train left St Pancras at ", "7.10", ", which would land me at any Galloway station in the late afternoon.", "John Buchan", "The Thirty-Nine Steps"],
&["There were many others waiting to execute the same operation, so she would have to move fast, elbow her way to the front so that she emerged first. The time was ", "7:10", " in the morning. The manoeuvre would start at 7:12. She looked apprehensively at the giant clock at the railway station.", "Mini Nair", "The Fourth Passenger"],
]);
    minutes.insert("07:12", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. 'Never meet people at 7:45 or 6:30, Jasper, but pick times like ", "7:12", " and 8:03!'", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("07:13", &[
&["It was all the more surprising and indeed alarming a little later, said Austerlitz, when I looked out of the corridor window of my carriage just before the train left at ", "seven-thirteen", ", to find it dawning upon me with perfect certainty that I had seen the pattern of glass and steel roof above the platforms before.", "WG Sebald", "Austerlitz"],
]);
    minutes.insert("07:14", &[
&["At ", "7.14", " Harry knew he was alive. He knew that because the pain could be felt in every nerve fibre.", "Jo Nesbo", "The Redeemer"],
]);
    minutes.insert("07:15", &[
&["At ", "7:15 A.M.", ", January 25th, we started flying northwestward under McTighe's pilotage with ten men, seven dogs, a sledge, a fuel and food supply, and other items including the plane's wireless outfit.", "H.P. Lovecraft", "At the Mountains of Madness"],
&["Gough again knocked on Mr and Mrs Kent's bedroom door. This time it was opened - Mary Kent had got out of bed and put on her dressing gown, having just checked her husband's watch: it was ", "7.15", ". A confused conversation ensued, in which each woman seemed to assume Saville was with the other.", "Kate Summerscale", "The Suspicions of Mr Whicher"],
&["Gough again knocked on Mr and Mrs Kent's bedroom door. This time it was opened - Mary Kent had got out of bed and put on her dressing gown, having just checked her husband's watch: it was ", "7.15", ". A confused conversation ensued, in which each woman seemed to assume Saville was with the other.", "Kate Summerscale", "The Suspicions of Mr Whicher"],
&["It was early in April in the year ’83 that I woke one morning to find Sherlock Holmes standing, fully dressed, by the side of my bed. He was a late riser, as a rule, and as the clock on the mantelpiece showed me that it was only a ", "quarter-past seven", ", I blinked up at him in some surprise, and perhaps just a little resentment, for I was myself regular in my habits.", "Arthur Conan Doyle", "The Adventure of the Speckled Band"],
]);
    minutes.insert("07:16", &[
&["At ", "7:15 A.M.", ", January 25th, we started flying northwestward under McTighe's pilotage with ten men, seven dogs, a sledge, a fuel and food supply, and other items including the plane's wireless outfit.", "H.P. Lovecraft", "At the Mountains of Madness"],
&["Gough again knocked on Mr and Mrs Kent's bedroom door. This time it was opened - Mary Kent had got out of bed and put on her dressing gown, having just checked her husband's watch: it was ", "7.15", ". A confused conversation ensued, in which each woman seemed to assume Saville was with the other.", "Kate Summerscale", "The Suspicions of Mr Whicher"],
&["Gough again knocked on Mr and Mrs Kent's bedroom door. This time it was opened - Mary Kent had got out of bed and put on her dressing gown, having just checked her husband's watch: it was ", "7.15", ". A confused conversation ensued, in which each woman seemed to assume Saville was with the other.", "Kate Summerscale", "The Suspicions of Mr Whicher"],
&["It was early in April in the year ’83 that I woke one morning to find Sherlock Holmes standing, fully dressed, by the side of my bed. He was a late riser, as a rule, and as the clock on the mantelpiece showed me that it was only a ", "quarter-past seven", ", I blinked up at him in some surprise, and perhaps just a little resentment, for I was myself regular in my habits.", "Arthur Conan Doyle", "The Adventure of the Speckled Band"],
]);
    minutes.insert("07:17", &[
&["As of ", "7.17am", " local time on 30 June 1908, Padzhitnoff had been working for nearly a year as a contract employee of the Okhrana, receiving five hundred rubles a month, a sum which hovered at the exorbitant end of spy-budget outlays for those years.", "Thomas Pynchon", "Against the Day"],
]);
    minutes.insert("07:18", &[
&["As of ", "7.17am", " local time on 30 June 1908, Padzhitnoff had been working for nearly a year as a contract employee of the Okhrana, receiving five hundred rubles a month, a sum which hovered at the exorbitant end of spy-budget outlays for those years.", "Thomas Pynchon", "Against the Day"],
]);
    minutes.insert("07:19", &[
&["I opened the sunroof and turned up the CD player volume to combat fatigue, and at ", "7.19am", " on Saturday, with the caffeine still running all around my brain, Jackson Browne and I pulled into Moree.", "Graeme Simsion", "The Rosie Project"],
]);
    minutes.insert("07:20", &[
&["And this was my timetable when I lived at home with Father and I thought that Mother was dead from a heart attack (this was the timetable for a Monday and also it is an approximation). ", "7.20 a.m.", " Wake up", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["He who had been a boy very credulous of life was no longer greatly interested in the possible and improbable adventures of each new day. He escaped from reality till the alarm-clock rang, at ", "seven-twenty", ".", "Sinclair Lewis", "Babbitt"],
]);
    minutes.insert("07:21", &[
&["And this was my timetable when I lived at home with Father and I thought that Mother was dead from a heart attack (this was the timetable for a Monday and also it is an approximation). ", "7.20 a.m.", " Wake up", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["He who had been a boy very credulous of life was no longer greatly interested in the possible and improbable adventures of each new day. He escaped from reality till the alarm-clock rang, at ", "seven-twenty", ".", "Sinclair Lewis", "Babbitt"],
]);
    minutes.insert("07:22", &[
&["And this was my timetable when I lived at home with Father and I thought that Mother was dead from a heart attack (this was the timetable for a Monday and also it is an approximation). ", "7.20 a.m.", " Wake up", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["He who had been a boy very credulous of life was no longer greatly interested in the possible and improbable adventures of each new day. He escaped from reality till the alarm-clock rang, at ", "seven-twenty", ".", "Sinclair Lewis", "Babbitt"],
]);
    minutes.insert("07:23", &[
&["And this was my timetable when I lived at home with Father and I thought that Mother was dead from a heart attack (this was the timetable for a Monday and also it is an approximation). ", "7.20 a.m.", " Wake up", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["He who had been a boy very credulous of life was no longer greatly interested in the possible and improbable adventures of each new day. He escaped from reality till the alarm-clock rang, at ", "seven-twenty", ".", "Sinclair Lewis", "Babbitt"],
]);
    minutes.insert("07:24", &[
&["And this was my timetable when I lived at home with Father and I thought that Mother was dead from a heart attack (this was the timetable for a Monday and also it is an approximation). ", "7.20 a.m.", " Wake up", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["He who had been a boy very credulous of life was no longer greatly interested in the possible and improbable adventures of each new day. He escaped from reality till the alarm-clock rang, at ", "seven-twenty", ".", "Sinclair Lewis", "Babbitt"],
]);
    minutes.insert(
        "07:25",
        &[&[
            "",
            "7.25 a.m.",
            " clean teeth and wash face",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "07:26",
        &[&[
            "",
            "7.25 a.m.",
            " clean teeth and wash face",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "07:27",
        &[&[
            "His appointment with the doctor was for 8.45. It was ",
            "7.27",
            ".",
            "Henning Mankell",
            "The Return of the Dancing Master",
        ]],
    );
    minutes.insert(
        "07:28",
        &[&[
            "His appointment with the doctor was for 8.45. It was ",
            "7.27",
            ".",
            "Henning Mankell",
            "The Return of the Dancing Master",
        ]],
    );
    minutes.insert(
        "07:29",
        &[&[
            "At ",
            "7.29",
            " in the morning of 1 July, the cinematographer finds himself filming silence itself.",
            "Elizabeth Speller",
            "At Break of Day",
        ]],
    );
    minutes.insert("07:30", &[
&["At ", "half-past seven", " the next morning he rang the bell of 21 Blenheim Avenue.", "William Trevor", "After Rain"],
&["Precisely at ", "half past seven", " the station-master came into the traffic office. He weighed almost sixteen stone, but women always said that he was incredibly light on his feet when he danced.", "Bohumil Hrabal", "Closely Observed Trains"],
]);
    minutes.insert("07:31", &[
&["At ", "half-past seven", " the next morning he rang the bell of 21 Blenheim Avenue.", "William Trevor", "After Rain"],
&["Precisely at ", "half past seven", " the station-master came into the traffic office. He weighed almost sixteen stone, but women always said that he was incredibly light on his feet when he danced.", "Bohumil Hrabal", "Closely Observed Trains"],
]);
    minutes.insert(
        "07:32",
        &[&[
            "At ",
            "7:32",
            ", he suffered a fatal stroke.",
            "Stephen King",
            "IT",
        ]],
    );
    minutes.insert(
        "07:33",
        &[&[
            "At ",
            "7:32",
            ", he suffered a fatal stroke.",
            "Stephen King",
            "IT",
        ]],
    );
    minutes.insert("07:34", &[
&["", "7:34.", " Monday morning, Blackeberg. The burglar alarm at the ICA grocery store on Arvid Morne's way is set off.", "John Ajvide Lindqvist", "Let The Right One In"],
]);
    minutes.insert(
        "07:35",
        &[
            &[
                "At ",
                "7:35am",
                " Ishigami left his apartment as he did every weekday morning.",
                "Higashino, Keigo",
                "The Devotion of Duspect X",
            ],
            &[
                "I looked at my watch. ",
                "Seven thirty-five",
                ".",
                "Kathy Reichs",
                "Bare Bones",
            ],
        ],
    );
    minutes.insert(
        "07:36",
        &[&[
            "",
            "7:36",
            ", sunrise. The hospital blinds were much better, darker than her own.",
            "John Ajvide Lindqvist",
            "Let The Right One In",
        ]],
    );
    minutes.insert(
        "07:37",
        &[&[
            "",
            "7:36",
            ", sunrise. The hospital blinds were much better, darker than her own.",
            "John Ajvide Lindqvist",
            "Let The Right One In",
        ]],
    );
    minutes.insert(
        "07:38",
        &[&[
            "",
            "7:36",
            ", sunrise. The hospital blinds were much better, darker than her own.",
            "John Ajvide Lindqvist",
            "Let The Right One In",
        ]],
    );
    minutes.insert("07:39", &[
&["Now, at the station, do you recall speaking to Mr Joseph Markew?' 'Yes, indeed. I was standing on the platform waiting for my usual train - the ", "7.39", " - when he accosted me.'", "Julian Barnes", "Arthur & George"],
]);
    minutes.insert(
        "07:40",
        &[&[
            "",
            "7.40 a.m.",
            " Have breakfast.",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "07:41",
        &[&[
            "",
            "7.40 a.m.",
            " Have breakfast.",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert("07:42", &[
&["", "Seven forty-two", " am., Mr Gasparian: I curse you. I curse your arms so they will wither and die and fall off your body...", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("07:43", &[
&["", "Seven forty-two", " am., Mr Gasparian: I curse you. I curse your arms so they will wither and die and fall off your body...", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("07:44", &[
&["And there I was, complaining that all this was just inconvenient, Anna castigates herself. The Goth was obviously right. What does it matter, really, if I'm a bit late for work? She voices her thoughts: \"It's not exactly how you'd choose to go, is it? You'd rather die flying a kite with your grandchildren, or at a great party or something. Not on the ", "seven forty-four", ".\"", "Sarah Rayner", "One moment, one morning"],
&["The Goth was obviously right. What does it matter, really, if I'm a bit late for work? She voices her thoughts: \"It's not exactly how you'd choose to go, is it? You'd rather die flying a kite with your grandchildren, or at a great party or something. Not on the ", "seven forty-four", ".\"", "Sarah Rayner", "One Moment, One Morning"],
]);
    minutes.insert("07:45", &[
&["Mr Green left for work at a ", "quarter to eight", ", as he did every morning. He walked down his front steps carrying his empty-looking leatherette briefcase with the noisy silver clasps, opened his car door, and ducked his head to climb into the driver's seat.", "Suzanne Berne", "A Crime in The Neighborhood"],
&["Mr Green left for work at a ", "quarter to eight", ", as he did every morning. He walked down his front steps carrying his empty-looking leatherette briefcase with the noisy silver clasps, opened his car door, and ducked his head to climb into the driver's seat.", "Suzanne Berne", "A crime in the neighborhood"],
]);
    minutes.insert("07:46", &[
&["He awoke with a start. The clock on his bedside table said ", "7.46 a.m.", " He cursed, jumped out of bed and dressed. He stuffed his toothbrush and toothpaste in his jacket pocket, and parked outside the station just before 8 a.m. In reception, Ebba beckoned to him.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("07:47", &[
&["He awoke with a start. The clock on his bedside table said ", "7.46 a.m.", " He cursed, jumped out of bed and dressed. He stuffed his toothbrush and toothpaste in his jacket pocket, and parked outside the station just before 8 a.m. In reception, Ebba beckoned to him.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("07:48", &[
&["He awoke with a start. The clock on his bedside table said ", "7.46 a.m.", " He cursed, jumped out of bed and dressed. He stuffed his toothbrush and toothpaste in his jacket pocket, and parked outside the station just before 8 a.m. In reception, Ebba beckoned to him.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("07:49", &[
&["He awoke with a start. The clock on his bedside table said ", "7.46 a.m.", " He cursed, jumped out of bed and dressed. He stuffed his toothbrush and toothpaste in his jacket pocket, and parked outside the station just before 8 a.m. In reception, Ebba beckoned to him.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("07:50", &[
&["At about ", "ten minutes to eight", ", Jim had squared the part of the work he had been doing - the window - so he decided not to start on the door or the skirting until after breakfast.", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("07:51", &[
&["Vimes fished out the Gooseberry as a red-hot cabbage smacked into the road behind him. \"Good morning!\" he said brightly to the surprised imp. \"What is the time, please?\" \"Er...", "nine minutes to eight", ", Insert Name Here,\" said the imp.", "Terry Pratchett", "Thud!"],
]);
    minutes.insert("07:52", &[
&["Vimes fished out the Gooseberry as a red-hot cabbage smacked into the road behind him. \"Good morning!\" he said brightly to the surprised imp. \"What is the time, please?\" \"Er...", "nine minutes to eight", ", Insert Name Here,\" said the imp.", "Terry Pratchett", "Thud!"],
]);
    minutes.insert(
        "07:53",
        &[&[
            "\"What time is it?\" \"",
            "Seven to eight",
            ". Won't be long now ...\"",
            "Robert Goddard",
            "Never go back",
        ]],
    );
    minutes.insert(
        "07:54",
        &[&[
            "\"What time is it?\" \"",
            "Seven to eight",
            ". Won't be long now ...\"",
            "Robert Goddard",
            "Never go back",
        ]],
    );
    minutes.insert(
        "07:55",
        &[&[
            "at ",
            "7.55",
            " this morning the circus ran away to join me.",
            "Roger McGough",
            "Tightrope, from Selected Poems 1967-1987",
        ]],
    );
    minutes.insert("07:56", &[
&["I sit by the window, crunching toast, sipping coffee, and leafing through the paper in a leisurely way. At last, after devouring three slices, two cups of coffee, and all the Saturday sections, I stretch my arms in a big yawn and glance at the clock. I don't believe it. It's only ", "seven fifty-six", ".", "Sophie Kinsella", "The Undomestic Goddess"],
&["The Castle Gate - only the Castle Gate - and it was ", "four minutes to eight", ".", "Thomas Mann", "Buddenbrooks"],
]);
    minutes.insert("07:57", &[
&["I sit by the window, crunching toast, sipping coffee, and leafing through the paper in a leisurely way. At last, after devouring three slices, two cups of coffee, and all the Saturday sections, I stretch my arms in a big yawn and glance at the clock. I don't believe it. It's only ", "seven fifty-six", ".", "Sophie Kinsella", "The Undomestic Goddess"],
&["The Castle Gate - only the Castle Gate - and it was ", "four minutes to eight", ".", "Thomas Mann", "Buddenbrooks"],
]);
    minutes.insert("07:58", &[
&["I sit by the window, crunching toast, sipping coffee, and leafing through the paper in a leisurely way. At last, after devouring three slices, two cups of coffee, and all the Saturday sections, I stretch my arms in a big yawn and glance at the clock. I don't believe it. It's only ", "seven fifty-six", ".", "Sophie Kinsella", "The Undomestic Goddess"],
&["The Castle Gate - only the Castle Gate - and it was ", "four minutes to eight", ".", "Thomas Mann", "Buddenbrooks"],
]);
    minutes.insert(
        "07:59",
        &[&[
            "I'd spent fifty two days in 1958, but here it was ",
            "7.59",
            " in the morning.",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert("08:00", &[
&["\"I'm not crying,\" Maria said when Carter called from the desert at ", "8 a.m.", " \"I'm perfectly alright\". \"You don't sound perfectly alright", "Joan Didion", "Play it as is Lays"],
&["", "8.00 a.m.", " Put school clothes on", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "8 o'clock", " on Thursday morning Arthur didn't feel very good.", "Douglas Adams", "Hitch-Hikers Guide to the Galaxy"],
&["At ", "eight o'clock", " on Thursday morning Arthur didn't feel very good. He woke up blearily, got up, wandered blearily round his room, opened a window, saw a bulldozer, found his slippers and stomped off to the bathroom to wash.", "Douglas Adams", "Hitch-hikers guide to the galaxy"],
&["", "At eight o’clock", ", a shaft of daylight came to wake us. The thousand facets of the lava on the rock face picked it up as it passed, scattering like a shower of sparks.", "Jules Verne", "Journey to the Centre of the Earth"],
&["But for now it was still ", "eight o'clock", ", and as I walked along the avenue under that brilliant blue sky, I was happy, my friends, as happy as any man who had ever lived.", "Paul Auster", "Brooklyn Follies"],
&["By ", "eight o'clock", " Stillman would come out, always in his long brown overcoat, carrying a large, old-fashioned carpet bag. For two weeks this routine did not vary. The old man would wander through the streets of the neighbourhood, advancing slowly, sometimes by the merest of increments, pausing, moving on again, pausing once more, as though each step had to be weighed and measured before it could take its place among the sum total of steps.", "Paul Auster", "City of Glass"],
&["Dressed in sweater, anorak and long johns, he lay in bed, hemmed in on three sides by chunky wooden beams, and ate all the salted snacks in the minibar, and then all the sugary snacks, and when he was woken by reception ", "at eight", " the following morning to be told that everyone was waiting for him downstairs, the wrapper of a Mars bar was still folded in his fist.", "Ian McEwan", "Solar"],
&["I hear noise at the ward door, off up the hall out of my sight. That ward door starts opening ", "at eight", " and opens and closes a thousand times a day, kashash, click.", "Ken Kesey", "One Flew Over the Cuckoo's Nest"],
&["It was dated from Rosings, at ", "eight o'clock", " in the morning, and was as follows: - \"Be not alarmed, madam, on receiving this letter, by the apprehension of its containing any repetition of those sentiments or renewal of those offerings which were last night so disgusting to you.", "Jane Austen", "Pride and Prejudice"],
&["Mr. Pumblechook and I breakfasted at ", "eight o'clock", " in the parlour behind the shop, while the shopman took his mug of tea and hunch of bread-and-butter on a sack of peas in the front premises.", "Charles Dickens", "Great Expectations"],
&["Mrs. Rochester! She did not exist: she would not be born till to-morrow, some time after ", "eight o'clock a.m.", "; and I would wait to be assured she had come into the world alive, before I assigned to her all that property.", "Charlotte Brontë", "Jane Eyre"],
&["So here I'll watch the night and wait To see the morning shine, When he will hear the stroke of ", "eight", " And not the stroke of nine;", "A E Housman", "A shropshire Lad"],
&["Someone must have been telling lies about Joseph K. for without having done anything wrong he was arrested one fine morning. His landlady's cook, who always brought him breakfast at ", "eight o'clock", ", failed to appear on this occasion.", "Franz Kafka", "The Trial"],
&["The next morning I woke up at ", "oh eight oh oh hours", ", my brothers, and as I still felt shagged and fagged and fashed and bashed and as my glazzies were stuck together real horrorshow with sleepglue, I thought I would not go to school .", "Anthony Burgess", "A Clockwork Orange"],
&["Three days after the quarrel, Prince Stepan Arkadyevitch Oblonsky--Stiva, as he was called in the fashionable world-- woke up at his usual hour, that is, at ", "eight o'clock", " in the morning, not in his wife's bedroom, but on the leather-covered sofa in his study.", "Leo Tolstoy", "Anna Karenina"],
&["Through the curtained windows of the furnished apartment which Mrs. Horace Hignett had rented for her stay in New York rays of golden sunlight peeped in like the foremost spies of some advancing army. It was a fine summer morning. The hands of the Dutch clock in the hall pointed to thirteen minutes past nine; those of the ormolu clock in the sitting-room to eleven minutes past ten; those of the carriage clock on the bookshelf to fourteen minutes to six. In other words, it was ", "exactly eight", "; and Mrs. Hignett acknowledged the fact by moving her head on the pillow, opening her eyes, and sitting up in bed. She always woke at eight precisely.", "P.G. Wodehouse", "Three Men and a Maid"],
&["When he opened the windows in the morning, the sky was as overcast as it had been, but the air seemed fresher, and regret set in. Had giving notice not been impetuous and wrongheaded, the result of an inconsequential indisposition? If he had held off a bit, if he had not been so quick to lose heart, if he had instead tried to adjust to the air or wait for the weather to improve, he would now have been free of stress and strain and looking forward to a morning on the beach like the one the day before. Too late. He must go on wanting what he had wanted yesterday. He dressed and rode down to the ground floor ", "at eight", " for breakfast.", "Thomas Mann", "Death in Venice"],
]);
    minutes.insert(
        "08:01",
        &[&[
            "",
            "Eight-one",
            ", tick-tock, eight-one o'clock, off to school, off to work, run, run, eight-one!",
            "Ray Bradbury",
            "There Will Come Soft Rains",
        ]],
    );
    minutes.insert("08:02", &[
&["... bingeley ... ", "Eight oh two", " eh em, Death of Corporal Littlebottombottom ... Eight oh three eh em ... Death of Sergeant Detritus ... Eight oh threethreethree eh em and seven seconds seconds ... Death of Constable Visit ... Eight oh three eh em and nineninenine seconds ... Death of death of death of ...", "Terry Pratchett", "Jingo"],
]);
    minutes.insert("08:03", &[
&["... bingeley ... Eight oh two eh em, Death of Corporal Littlebottombottom ... ", "Eight oh three", " eh em ... Death of Sergeant Detritus ... Eight oh threethreethree eh em and seven seconds seconds ... Death of Constable Visit ... Eight oh three eh em and nineninenine seconds ... Death of death of death of ...", "Terry Pratchett", "Jingo"],
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. 'Never meet people at 7:45 or 6:30, Jasper, but pick times like 7:12 and ", "8:03", "!'", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("08:04", &[
&["... every clerk had his particular schedule of hours, which coincided with a single pair of tram runs coming from the city: A had to come in at 8, B at ", "8:04", ", C at 8:08 and so on, and the same for quitting times, in such a manner that never would two colleagues have the opportunity to travel in the same tramcar.", "Primo Levi", "The Periodic Table"],
]);
    minutes.insert(
        "08:05",
        &[&[
            "",
            "8.05 a.m.",
            " Pack school bag",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "08:06",
        &[&[
            "",
            "8.05 a.m.",
            " Pack school bag",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "08:07",
        &[&[
            "",
            "8.05 a.m.",
            " Pack school bag",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert("08:08", &[
&["... every clerk had his particular schedule of hours, which coincided with a single pair of tram runs coming from the city: A had to come in at 8, B at 8:04, C at ", "8:08", " and so on, and the same for quitting times, in such a manner that never would two colleagues have the opportunity to travel in the same tramcar.", "Primo Levi", "The Periodic Table"],
]);
    minutes.insert(
        "08:09",
        &[&[
            "He followed the squeals down a hallway. A wall clock read ",
            "8:09",
            " - 10:09 Dallas time.",
            "James Ellroy",
            "American Tabloid",
        ]],
    );
    minutes.insert("08:10", &[
&["", "8.10 a.m.", " Read book or watch video", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Amory rushed into the house and the rest followed with a limp mass that they laid on the sofa in the shoddy little front parlor. Sloane, with his shoulder punctured, was on another lounge. He was half delirious, and kept calling something about a chemistry lecture at ", "8:10", ".", "F. Scott Fitzgerald", "This Side of Paradise"],
&["Cell count down to 400,000. Woke ", "8:10", ". To sleep 7:15. (Appear to have lost my watch without realising it, had to drive into town to buy another.)", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("08:11", &[
&["'Care for a turn on the engine?' he called to the doxies, and pointed up at the footplate. They laughed but voted not to, climbing up with their bathtub into one of the rattlers instead. They both had very fetching hats, with one flower apiece, but the prettiness of their faces made you think it was more. For some reason they both wore white rosettes pinned to their dresses. I looked again at the clock: ", "eight-eleven", ".", "Andrew Martin", "The Blackpool Highflyer"],
]);
    minutes.insert("08:12", &[
&["At ", "8:12 a.m.", ", just before the moment of pff, all the business of the cellars was being transacted - garbage transferred from small cans into large ones; early wide-awake grandmas, rocky with insomnia, dumped wash into the big tubs; boys in swimming trunks rolled baby carriages out into the cool morning.", "Grace Paley", "In Time Which Made A Monkey Of Us All"],
]);
    minutes.insert("08:13", &[
&["At ", "8:13 a.m.", " the alarm clock in the laboratory gave the ringing word. Eddie touched a button in the substructure of an ordinary glass coffeepot, from whose spout two tubes proceeded into the wall.", "Grace Paley", "In Time Which Made A Monkey Of Us All"],
]);
    minutes.insert("08:14", &[
&["At ", "8:13 a.m.", " the alarm clock in the laboratory gave the ringing word. Eddie touched a button in the substructure of an ordinary glass coffeepot, from whose spout two tubes proceeded into the wall.", "Grace Paley", "In Time Which Made A Monkey Of Us All"],
]);
    minutes.insert("08:15", &[
&["It was in the winter when this happened, very near the shortest day, and a week of fog into the bargain, so the fact that it was still very dark when George woke in the morning was no guide to him as to the time. He reached up, and hauled down his watch. It was a ", "quarter-past eight", ".", "Jerome K Jerome", "Three Men in a Boat"],
&["You scrutinized your wrist: \"It's ", "eight fifteen", ". (And here time forked.) I'll turn it on.\" The screen In its blank broth evolved a lifelike blur, And music welled.", "Vladimir Nabokov", "Pale Fire"],
]);
    minutes.insert("08:16", &[
&["I walk through the fruit trees toward a huge, square, brown patch of earth with vegetation growing in serried rows. These must be the vegetables. I prod one of them cautiously with my foot. It could be a cabbage or a lettuce. Or the leaves of something growing underground, maybe. To be honest, it could be an alien. I have no idea. I sit down on a mossy wooden bench and look at a nearby bush covered in white flowers. Mm. Pretty. Now what? What do people do in their gardens? I feel I should have something to read. Or someone to call. My fingers are itching to move. I look at my watch. Still only ", "eight sixteen", ". Oh God.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("08:17", &[
&["Breakfast over, my uncle drew from his pocket a small notebook, intended for scientific observations. He consulted his instruments, and recorded:
“Monday, July 1.
“Chronometer, ", "8.17 a.m.", "; barometer, 297 in.; thermometer, 6° (43° F.). Direction, E.S.E.”
This last observation applied to the dark gallery, and was indicated by the compass.", "Jules Verne", "A Journey to the Centre of the Earth"],
&["Come on, I can't give up yet. I'll just sit here for a bit and enjoy the peace. I lean back and watch a little speckled bird pecking the ground nearby for a while. Then I look at my watch again: ", "eight seventeen", ". I can't do this.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("08:18", &[
&["Breakfast over, my uncle drew from his pocket a small notebook, intended for scientific observations. He consulted his instruments, and recorded:
“Monday, July 1.
“Chronometer, ", "8.17 a.m.", "; barometer, 297 in.; thermometer, 6° (43° F.). Direction, E.S.E.”
This last observation applied to the dark gallery, and was indicated by the compass.", "Jules Verne", "A Journey to the Centre of the Earth"],
&["Come on, I can't give up yet. I'll just sit here for a bit and enjoy the peace. I lean back and watch a little speckled bird pecking the ground nearby for a while. Then I look at my watch again: ", "eight seventeen", ". I can't do this.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("08:19", &[
&["I had arranged to meet the Occupational Health Officer at 10:30. I took the train from Watford Junction at ", "8.19", " and arrived at London Euston seven minutes late, at 8.49.", "Jonathan Coe", "The Terrible Privacy of Maxwell Sim"],
]);
    minutes.insert("08:20", &[
&["When the typewriters happen to pause (", "8:20", " and other mythical hours), and there are no flights of American bombers in the sky, and the motor traffic's not too heavy in Oxford Street, you can hear winter birds cheeping outside, busy at the feeders the girls have put up.", "Thomas Pynchon", "Gravity's Rainbow"],
]);
    minutes.insert("08:21", &[
&["When the typewriters happen to pause (", "8:20", " and other mythical hours), and there are no flights of American bombers in the sky, and the motor traffic's not too heavy in Oxford Street, you can hear winter birds cheeping outside, busy at the feeders the girls have put up.", "Thomas Pynchon", "Gravity's Rainbow"],
]);
    minutes.insert("08:22", &[
&["When the typewriters happen to pause (", "8:20", " and other mythical hours), and there are no flights of American bombers in the sky, and the motor traffic's not too heavy in Oxford Street, you can hear winter birds cheeping outside, busy at the feeders the girls have put up.", "Thomas Pynchon", "Gravity's Rainbow"],
]);
    minutes.insert("08:23", &[
&["And then Wedderburn looked at his watch. \"", "Twenty-three minutes past eight", ". I am going up by the quarter to twelve train, so that there is plenty of time. I think I shall wear my alpaca jacket - it is quite warm enough - and my grey felt hat and brown shoes. I suppose”", "HG Wells", "The Flowering of The Strange Orchid"],
&["At ", "8:23", " there seemed every chance of a lasting alliance starting between Florin and Guilder. At 8:24 the two nations were very close to war.", "William Goldman", "The Princess Bride"],
]);
    minutes.insert("08:24", &[
&["At 8:23 there seemed every chance of a lasting alliance starting between Florin and Guilder. At ", "8:24", " the two nations were very close to war.", "William Goldman", "The Princess Bride"],
]);
    minutes.insert("08:25", &[
&["At 8:23 there seemed every chance of a lasting alliance starting between Florin and Guilder. At ", "8:24", " the two nations were very close to war.", "William Goldman", "The Princess Bride"],
]);
    minutes.insert("08:26", &[
&["It exploded much later than intended, probably a good twelve hours later, at ", "twenty-six minutes past eight", " on Monday morning. Several defunct wristwatches, the property of victims, confirmed the time. As with its predecessors over the last few months, there had been no warning.", "John Le Carre", "The Little Drummer Girl"],
]);
    minutes.insert(
        "08:27",
        &[&[
            "The lecture was to be given tomorrow, and it was now ",
            "almost eight-thirty",
            ".",
            "John Kennedy Toole",
            "A Confederacy of Dunces",
        ]],
    );
    minutes.insert("08:28", &[
&["And at ", "8.28", " on the following morning, with a novel chilliness about the upper lip, and a vast excess of strength and spirits, I was sitting in a third-class carriage, bound for Germany, and dressed as a young sea-man, in a pea-jacket, peaked cap, and comforter.", "Erskine Childers", "The Riddle of the Sands"],
]);
    minutes.insert("08:29", &[
&["At ", "8.29", " I punched the front doorbell in Elgin Crescent. It was opened by a small oriental woman in a white apron. She showed me into a large, empty sitting room with an open fire and a couple of huge oil paintings.", "Sebastian Faulks", "Engleby"],
]);
    minutes.insert("08:30", &[
&["At ", "half past eight", ", Mr. Dursley picked up his briefcase, pecked Mrs. Dursley on the cheek, and tried to kiss Dudley good-bye but missed, because Dudley was now having a tantrum and throwing his cereal at the walls.", "JK Rowling", "Harry Potter and the Philosopher's Stone"],
&["It is around ", "8:30", ". Sunshine comes through the windows at right. As the curtain rises, the family has just finished breakfast.", "Eugene O'Neill", "Long Day's Journey Into Night"],
&["On July 25th, ", "8:30 a.m.", " the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at 7:30 that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The lecture was to be given tomorrow, and it was now almost ", "eight-thirty", ".", "John Kennedy Toole", "A Confederacy of Dunces"],
&["When he woke, at ", "eight-thirty", ", he was alone in the bedroom. He put on his dressing gown and put in his hearing aid and went into the living room.", "David Lodge", "Deaf Sentence"],
]);
    minutes.insert("08:31", &[
&["At ", "half past eight", ", Mr. Dursley picked up his briefcase, pecked Mrs. Dursley on the cheek, and tried to kiss Dudley good-bye but missed, because Dudley was now having a tantrum and throwing his cereal at the walls.", "JK Rowling", "Harry Potter and the Philosopher's Stone"],
&["It is around ", "8:30", ". Sunshine comes through the windows at right. As the curtain rises, the family has just finished breakfast.", "Eugene O'Neill", "Long Day's Journey Into Night"],
&["On July 25th, ", "8:30 a.m.", " the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at 7:30 that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The lecture was to be given tomorrow, and it was now almost ", "eight-thirty", ".", "John Kennedy Toole", "A Confederacy of Dunces"],
&["When he woke, at ", "eight-thirty", ", he was alone in the bedroom. He put on his dressing gown and put in his hearing aid and went into the living room.", "David Lodge", "Deaf Sentence"],
]);
    minutes.insert("08:32", &[
&["'Does anybody know the time a little more exactly is what I'm wondering, Don, since Day doesn't.' Gately checks his cheap digital, head still hung over the sofa's arm. 'I got 0", "832", ":14, 15, 16, Randy.' ''ks a lot, D.G. man.'", "David Foster Wallace", "Infinite Jest"],
&["", "8.32 a.m.", " Catch bus to school", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("08:33", &[
&["'Does anybody know the time a little more exactly is what I'm wondering, Don, since Day doesn't.' Gately checks his cheap digital, head still hung over the sofa's arm. 'I got 0", "832", ":14, 15, 16, Randy.' ''ks a lot, D.G. man.'", "David Foster Wallace", "Infinite Jest"],
&["", "8.32 a.m.", " Catch bus to school", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("08:34", &[
&["'Does anybody know the time a little more exactly is what I'm wondering, Don, since Day doesn't.' Gately checks his cheap digital, head still hung over the sofa's arm. 'I got 0", "832", ":14, 15, 16, Randy.' ''ks a lot, D.G. man.'", "David Foster Wallace", "Infinite Jest"],
&["", "8.32 a.m.", " Catch bus to school", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("08:35", &[
&["It was ", "thirty-five minutes past eight", " by the big clock of the central building when Mathieu crossed the yard towards the office which he occupied as chief designer. For eight years he had been employed at the works where, after a brilliant and special course of study, he had made his beginning as assistant draughtsman when but nineteen years old, receiving at that time a salary of one hundred francs a month.", "Emile Zola", "Fruitfulness"],
&["Old gummy granny (thrusts a dagger towards Stephen's hand) Remove him, acushla. At ", "8.35 a.m.", " you will be in heaven and Ireland will be free (she prays) O good God take him!", "James Joyce", "Ulysses"],
]);
    minutes.insert("08:36", &[
&["It was ", "thirty-five minutes past eight", " by the big clock of the central building when Mathieu crossed the yard towards the office which he occupied as chief designer. For eight years he had been employed at the works where, after a brilliant and special course of study, he had made his beginning as assistant draughtsman when but nineteen years old, receiving at that time a salary of one hundred francs a month.", "Emile Zola", "Fruitfulness"],
&["Old gummy granny (thrusts a dagger towards Stephen's hand) Remove him, acushla. At ", "8.35 a.m.", " you will be in heaven and Ireland will be free (she prays) O good God take him!", "James Joyce", "Ulysses"],
]);
    minutes.insert("08:37", &[
&["", "Eight thirty-seven", " am., Patrice Lane, Biohazard: The dog's clean. The Good Samaritan was a woman with an accent of some sort. Why haven't you called me?", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("08:38", &[
&["", "Eight thirty-seven", " am., Patrice Lane, Biohazard: The dog's clean. The Good Samaritan was a woman with an accent of some sort. Why haven't you called me?", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert(
        "08:39",
        &[&[
            "Doug McGuire noticed the early hour, ",
            "8:39 A.M.",
            " on the one wall clock that gave Daylight Savings Time for the East Coast.",
            "Winn Schwartau",
            "Terminal Compromise",
        ]],
    );
    minutes.insert("08:40", &[
&["At this moment the clock indicated ", "8.40", ". 'Five minutes more,' said Andrew Stuart. The five friends looked at each other. One may surmise that their heart-beats were slightly accelereted, for, even for bold gamblers, the stake was a large one.'", "Jules Verne", "Around the world in eighty days"],
&["It was when I stood before her, avoiding her eyes, that I took note of the surrounding objects in detail, and saw that her watch had stopped at ", "twenty minutes to nine", ", and that a clock in the room had stopped at twenty minutes to nine.", "Charles Dickens", "Great Expectations"],
]);
    minutes.insert("08:41", &[
&["By ", "forty-one minutes past eight", " we are five hundred yards from the water’s edge, and between our road and the foot of the mountain we descry the piled-up remains of a ruined tower.", "Félicien de Saulcy", "Narrative of a Journey round the Dead Sea and in the Bible lands in 1850 and 1851"],
]);
    minutes.insert("08:42", &[
&["By ", "forty-one minutes past eight", " we are five hundred yards from the water’s edge, and between our road and the foot of the mountain we descry the piled-up remains of a ruined tower.", "Félicien de Saulcy", "Narrative of a Journey round the Dead Sea and in the Bible lands in 1850 and 1851"],
]);
    minutes.insert("08:43", &[
&["\"You understand this tape recorder is on?\" \"Uh huh\" \"And it's Wednesday, May 15, at ", "eight forty-three", " in the mornin'.\" \"If you say so\"", "John Grisham", "A Time to Kill"],
&["", "8.43 a.m.", " Go past tropical fish shop", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("08:44", &[
&["Several soldiers - some with their uniforms unbuttoned - were looking over a motorcycle, arguing about it. The sergeant looked at his watch; it was ", "eight forty-four", ". They had to wait until nine. Hladik, feeling more insignificant than ill-fortuned, sat down on a pile of firewood.", "Jorge Luis Borges", "The Secret Miracle"],
]);
    minutes.insert("08:45", &[
&["He paid the waitress and left the café. It was ", "8:45", ". The sun pressed against the inside of a thin layer of cloud. He unbuttoned his jacket as he hurried down Queensway. His mind, unleashed, sprang forwards.", "Rupert Thomson", "Dreams of leaving"],
]);
    minutes.insert("08:46", &[
&["He paid the waitress and left the café. It was ", "8:45", ". The sun pressed against the inside of a thin layer of cloud. He unbuttoned his jacket as he hurried down Queensway. His mind, unleashed, sprang forwards.", "Rupert Thomson", "Dreams of leaving"],
]);
    minutes.insert(
        "08:47",
        &[
            &[
                "\"Just on my way to the cottage. It's, er, ..",
                "8.47",
                ". Bit misty on the roads.....\"",
                "Douglas Adams",
                "Dirk Gently's Holistic Detective Agency",
            ],
            &[
                "",
                "8.47",
                ". Bit misty on the roads",
                "Douglas Adams",
                "Dirk Gently's Holistic Detective Agency",
            ],
        ],
    );
    minutes.insert(
        "08:48",
        &[
            &[
                "\"Just on my way to the cottage. It's, er, ..",
                "8.47",
                ". Bit misty on the roads.....\"",
                "Douglas Adams",
                "Dirk Gently's Holistic Detective Agency",
            ],
            &[
                "",
                "8.47",
                ". Bit misty on the roads",
                "Douglas Adams",
                "Dirk Gently's Holistic Detective Agency",
            ],
        ],
    );
    minutes.insert("08:49", &[
&["I had arranged to meet the Occupational Health Officer at 10:30. I took the train from Watford Junction at 8.19 and arrived at London Euston seven minutes late, at ", "8.49", ".", "Jonathan Coe", "The Terrible Privacy of Maxwell Sim"],
]);
    minutes.insert("08:50", &[
&["At ", "ten to nine", " the clerks began to arrive.When they had hung up their coats and hates they came to the fireplace and stood warming themselves. If there was no fire, they stood there all the same", "V.S. Pritchett", "The Chestnut Tree"],
&["It was ", "8:50", " in the morning and Bernie and I were alone on an Astoria side street, not far from a sandwich shop that sold a sopressatta sub called \"The Bypass\". I used to eat that sandwich weekly, wash it down with espresso soda, smoke a cigarette, go for a jog. Now I was too near the joke to order the sandwich, and my son's preschool in the throes of doctrinal schism.", "Sam Lipsyte", "The Ask"],
&["Punctually at ", "ten minutes to nine", ", a quarter hour after early mass, the boy stood in his Sunday uniform outside his father's door.", "Joseph Roth", "The Radetzky March"],
]);
    minutes.insert(
        "08:51",
        &[&[
            "",
            "8.51 a.m.",
            " Arrive at school",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-time",
        ]],
    );
    minutes.insert(
        "08:52",
        &[&[
            "Message one. Tuesday, ",
            "8.52am.",
            " Is anybody there? Hello?",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert(
        "08:53",
        &[&[
            "Message one. Tuesday, ",
            "8.52am.",
            " Is anybody there? Hello?",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert("08:54", &[
&["It was Mrs. Poppets that woke me up next morning. She said: “Do you know that it’s ", "nearly nine o’clock", ", sir?” “Nine o’ what?” I cried, starting up. “Nine o’clock,” she replied, through the keyhole. “I thought you was a- oversleeping yourselves.”", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert("08:55", &[
&["At ", "five minutes to nine", ", Jacques, in his gray butler's livery, came down the stairs and said, \"Young master, your Herr Papá is coming.\"", "Joseph Roth", "The Radetzky March"],
&["George pulled out his watch and looked at it: it was ", "five minutes to nine", "!", "Jerome K Jerome", "Three Men in a Boat"],
]);
    minutes.insert(
        "08:56",
        &[&[
            "It was ",
            "nearly nine o'clock",
            " and the sun was fiercer every minute.'",
            "George Orwell",
            "Burmese Days",
        ]],
    );
    minutes.insert("08:57", &[
&["You'll have to hurry. Many a long year before that, in one of the bygone centuries, a worthy citizen of Wrychester, Martin by name, had left a sum of money to the Dean and Chapter of the Cathedral on condition that as long as ever the Cathedral stood, they should cause to be rung a bell from its smaller bell-tower for ", "three minutes before nine", " o'clock every morning, all the year round.", "JS Fletcher", "The Paradise Mystery"],
]);
    minutes.insert("08:58", &[
&["It was ", "two minutes of nine", " now - two minutes before the bombs were set to explode - and three or four people were gathered in front of the bank waiting for it to open.", "Jim Thompson", "The Getaway"],
]);
    minutes.insert("08:59", &[
&["She had been lying in bed reading about Sophie and Alberto's conversation on Marx and had fallen asleep. The reading lamp by the bed had been on all night. The green glowing digits on her desk alarm clock showed ", "8:59", ".", "Jostein Gaarder", "Sophie's World"],
]);
    minutes.insert("09:00", &[
&["'I could never get all the way down there before ", "nine o'clock", ".'", "John Kennedy Toole", "A Confederacy of Dunces"],
&["'Look. Ignatius. I'm beat. I've been on the road since ", "nine o'clock", " yesterday morning.'", "John Kennedy Toole", "A Confederacy of Dunces"],
&["On the third morning after their arrival, just as all the clocks in the city were striking ", "nine", " individually, and somewhere about nine hundred and ninety-nine collectively, Sam was taking the air in George Yard, when a queer sort of fresh painted vehicle drove up, out of which there jumped with great agility, throwing the reins to a stout man who sat beside him, a queer sort of gentleman, who seemed made for the vehicle, and the vehicle for him.", "Charles Dickens", "The Pickwick Papers"],
&["14 June ", "9:00 am", " woke up", "Steve Toltz", "A Fraction of the Whole"],
&["", "9.00 a.m.", " School assembly", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["A fly buzzed, the wall clock began to strike. After the ", "nine", " golden strokes faded, the district captain began. \"How is Herr Colonel Marek?\" \"Thank you, Papá, he's fine.\" \"Still weak in geometry?\" \"Thank you, Papá, a little better.\" \"Read any books?\" \"Yessir, Papá.\"", "Joseph Roth", "The Radetzky March"],
&["As ", "nine o' clock", " was left behind, the preposterousness of the delay overwhelmed me, and I went in a kind of temper to the owner and said that I thought he should sign on another cook and weigh spars and be off.", "John Hershey", "A Single Pebble"],
&["At ", "nine o'clock", ", one morning late in July, Gatsby's gorgeous car lurched up the rocky drive to my door and gave out a burst of melody from its three-noted horn", "F. Scott Fitzgerald", "The Great Gatsby"],
&["He was at breakfast ", "at nine", ", and for the twentieth time consulted his \"Bradshaw,\" to see at what earliest hour Dr. Grantly could arrive from Barchester.", "Anthony Trollope", "The Warden"],
&["He won't stand beating. Now, if you only kept on good terms with him, he'd do almost anything you liked with the clock. For instance, suppose it were ", "nine o'clock", " in the morning, just time to begin lessons: you'd only have to whisper a hint to Time, and round goes the clock in a twinkling! Half-past one, time for dinner!", "Lewis Carroll", "Alice's Adventures in Wonderland"],
&["It was around ", "nine o'clock", " that I crossed the border into Cornwall. This was at least three hours before the rain began and the clouds were still all of a brilliant white. In fact, many of the sights that greeted me this morning were among the most charming I have so far encountered. It was unfortunate, then, that I could not for much of the time give to them the attention they warranted; for one may as well declare it, one was in a condition of some preoccupation with the thought that - barring some unseen complication - one would be meeting Miss Kenton again before the day's end.", "Kazuo Ishiguro", "The Remains of the Day"],
&["Opening his window, Aschenbach thought he could smell the foul stench of the lagoon. A sudden despondency came over him. He considered leaving then and there. Once, years before, after weeks of a beautiful spring, he had been visited by this sort of weather and it so affected his health he had been obliged to flee. Was not the same listless fever setting in? The pressure in the temples, the heavy eyelids? Changing hotels again would be a nuisance, but if the wind failed to shift he could not possibly remain here. To be on the safe side, he did not unpack everything. ", "At nine", " he went to breakfast in the specially designated buffet between the lobby and the dining room.", "Thomas Mann", "Death in Venice"],
&["Sometimes what I wouldn't give to have us sitting in a bar again at ", "9.00am", " telling lies to one another, far from God.", "Denis Johnson", "Jesus' Son"],
&["The clock struck ", "nine", " when I did send the nurse; In half an hour she promised to return. Perchance she cannot meet him: that's not so.", "Shakespeare", "Romeo and Juliet"],
&["To where Saint Mary Woolnoth kept the hours With a dead sound on the final stroke of ", "nine", ".", "T S Eliot", "The Waste Land"],
&["Unreal City, Under the brown fog of a winter dawn, A crowd flowed over London Bridge, so many, I had not thought death had undone so many. Sighs, short and infrequent, were exhaled, And each man fixed his eyes before his feet. Flowed up the hill and down King William Street, To where Saint Mary Woolnoth kept the hours With a dead sound on the final stroke of ", "nine", ".", "T S Eliot", "The Waste Land"],
]);
    minutes.insert(
        "09:01",
        &[&[
            "",
            "9:01am",
            " lay in bed, staring at ceiling.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:02",
        &[&[
            "",
            "9:02am",
            " lay in bed, staring at ceiling.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert("09:03", &[
&["", "9:03am", " lay in bed, staring at ceiling.", "Steve Toltz", "A Fraction of the Whole"],
&["This isn't a very good start to the new school year.\" I stared at her. What was she talking about? Why was she looking at her watch? I wasn't late. Okay, the school bell had rung as I was crossing the playground, but you always get five minutes to get to your classroom. \"It's ", "three minutes past nine", ",\" Miss Beckworth announced. \"You're late.\"", "Jacqueline Wilson", "The Lottie Project"],
]);
    minutes.insert(
        "09:04",
        &[
            &[
                "",
                "9:04am",
                " lay in bed, staring at ceiling",
                "Steve Toltz",
                "A Fraction of the Whole",
            ],
            &[
                "In the light of a narrow-beam lantern, Pierce checked his watch. It was ",
                "9.04",
                ".",
                "Michael Crichton",
                "The Great Train Robbery",
            ],
        ],
    );
    minutes.insert("09:05", &[
&["", "9:05am", " lay in bed, staring at ceiling", "Steve Toltz", "A Fraction of the Whole"],
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke ", "9:05", ". To sleep 6:36.", "JG Ballard", "The Voices of Time"],
&["The tour of the office doesn't take that long. In fact, we're pretty much done by ", "9:05 a.m.", " … and even though it's just a room with a window and a pin board and two doors and two desks... I can't help feeling a buzz as I lead them around. It's mine. My space. My company.", "Sophie Kinsella", "Twenties Girl"],
&["The tour of the office doesn't take that long. In fact, we're pretty much done by ", "9:05 a.m.", " Ed looks at everything twice and says it's all great, and gives me a list of contacts who might be helpful, then has to leave for his own office.", "Sophie Kinsella", "Twenties Girl"],
]);
    minutes.insert(
        "09:06",
        &[&[
            "",
            "9:06am",
            " lay in bed, staring at ceiling",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert("09:07", &[
&["", "9:07am", " lay in bed, staring at ceiling", "Steve Toltz", "A Fraction of the Whole"],
&["It was a sparkling morning, ", "9:07", " by the clock when Mrs. Flett stepped aboard the Imperial Limited at the Tyndall station, certain that her life was ruined, but managing, through an effort of will, to hold herself erect and to affect an air of preoccupation and liveliness.", "Carol Shields", "The Stone Diaries"],
]);
    minutes.insert(
        "09:08",
        &[&[
            "",
            "9.08am",
            " rolled over onto left side.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:09",
        &[&[
            "",
            "9.09am",
            " lay in bed, staring at wall.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:10",
        &[&[
            "",
            "9.10am",
            " lay in bed, staring at wall.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:11",
        &[&[
            "",
            "9:11am",
            " lay in bed, staring at wall",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:12",
        &[&[
            "",
            "9.12am",
            " lay in bed, staring at wall.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert("09:13", &[
&["", "9:13am", " lay in bed, staring at wall", "Steve Toltz", "A Fraction of the Whole"],
&["She tucked the phone in the crook of her neck and thumbed hurriedly through her pink messages. Dr. Provetto, at ", "9:13 A.M.", "", "Lisas Scottoline", "Mistaken Identity"],
]);
    minutes.insert(
        "09:14",
        &[&[
            "",
            "9.14am",
            " lay in bed, staring at wall.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert("09:15", &[
&["\"Great!\" Jones commented. \"I've never seen it do that before. That's all right. Okay.\" Jones pulled a handful of pencils from his back pocket. \"Now, I got the contact first at 0", "915", " or so, and the bearing was about two-six-nine.\"", "Tom Clancy", "The Hunt for Red October"],
&["", "9:15am", " doubled over pillow, sat up to see out window", "Steve Toltz", "A Fraction of the Whole"],
&["", "9.15 a.m.", " First morning class", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Miss Pettigrew pushed open the door of the employment agency and went in as the clock struck a ", "quarter past nine", ".", "Winifred Watson", "Miss Pettigrew Lives For a Day"],
]);
    minutes.insert(
        "09:16",
        &[&[
            "",
            "9.16am",
            " sat in bed, staring out window.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:17",
        &[&[
            "",
            "9.17am",
            " sat in bed, staring out window.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:18",
        &[&[
            "",
            "9.18am",
            " sat in bed, staring out window.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert(
        "09:19",
        &[&[
            "",
            "9.19am",
            " sat in bed, staring out window.",
            "Steve Toltz",
            "A Fraction of the Whole",
        ]],
    );
    minutes.insert("09:20", &[
&["I'll compromise by saying that I left home at eight and spent an hour travelling to a nine o'clock appointment. Twenty minutes later is ", "nine-twenty", ".", "Susanna Kaysen", "Girl, Interrupted"],
&["At ", "twenty minutes past nine", ", the Duke of Dunstable, who had dined off a tray in his room, was still there, waiting for his coffee and liqueur.", "P.G. Wodehouse", "Uncle Fred in the Springtime"],
&["The following morning at ", "9.20", " Mr Cribbage straightened his greasy old tie, combed his Hitler moustache and arranged the few strands of his hair across his bald patch.", "Louis de Bernieres", "Red Dog"],
]);
    minutes.insert(
        "09:21",
        &[&[
            "It was ",
            "nine twenty-one",
            ". With one minute to go, there was no sign of Herbert's mother.",
            "Dan Rhodes",
            "This is Life",
        ]],
    );
    minutes.insert("09:22", &[
&["No more throwing stones at him, and I'll see you back here exactly one week from now. She looked at her watch. 'At ", "nine twenty-two", " next Wednesday.'", "Dan Rhodes", "This is Life"],
]);
    minutes.insert(
        "09:23",
        &[&[
            "",
            "9.23",
            ". What possessed me to buy this comb?",
            "James Joyce",
            "Ulysses",
        ]],
    );
    minutes.insert(
        "09:24",
        &[&[
            "",
            "9.24",
            " I'm swelled after that cabbage. A speck of dust on the patent leather of her boot.",
            "James Joyce",
            "Ulysses",
        ]],
    );
    minutes.insert("09:25", &[
&["A man I would cross the street to avoid at nine o'clock - by ", "nine twenty-five", " I wanted to fuck him until he wept. My legs trembled with it. My voice floated out of my mouth when I opened it to speak. The glass wall of the meeting room was huge and suddenly too transparent.", "Anne Enright", "The Forgotten Waltz"],
]);
    minutes.insert("09:26", &[
&["A man I would cross the street to avoid at nine o'clock - by ", "nine twenty-five", " I wanted to fuck him until he wept. My legs trembled with it. My voice floated out of my mouth when I opened it to speak. The glass wall of the meeting room was huge and suddenly too transparent.", "Anne Enright", "The Forgotten Waltz"],
]);
    minutes.insert("09:27", &[
&["From twenty minutes past nine until ", "twenty-seven minutes past nine", ", from twenty-five minutes past eleven until twenty-eight minutes past eleven, from ten minutes to three until two minutes to three the heroes of the school met in a large familiarity whose Olympian laughter awed the fearful small boy that flitted uneasily past and chilled the slouching senior that rashly paused to examine the notices in assertion of an unearned right.", "Compton Mackenzie", "Sinister Street"],
]);
    minutes.insert("09:28", &[
&["\"This clock right?\" he asked the butler in the hall. \"Yes, sir.\" The clock showed ", "twenty-eight minutes past nine", ". \"The clocks here have to be right, sir,\" the butler added with pride and a respectful humour, on the stairs.", "Arnold Bennett", "Lord Raingo"],
&["He entered No. 10 for the first time, he who had sat on the Government benches for eight years and who had known the Prime Minister from youth up. \"This clock right?\" he asked the butler in the hall. \"Yes, sir.\" The clock showed ", "twenty-eight minutes past nine", ". \"The clocks here have to be right, sir,\" the butler added with pride and a respectful humour, on the stairs.", "Arnold Bennett", "Lord Raingo"],
]);
    minutes.insert("09:29", &[
&["\"This clock right?\" he asked the butler in the hall. \"Yes, sir.\" The clock showed ", "twenty-eight minutes past nine", ". \"The clocks here have to be right, sir,\" the butler added with pride and a respectful humour, on the stairs.", "Arnold Bennett", "Lord Raingo"],
&["He entered No. 10 for the first time, he who had sat on the Government benches for eight years and who had known the Prime Minister from youth up. \"This clock right?\" he asked the butler in the hall. \"Yes, sir.\" The clock showed ", "twenty-eight minutes past nine", ". \"The clocks here have to be right, sir,\" the butler added with pride and a respectful humour, on the stairs.", "Arnold Bennett", "Lord Raingo"],
]);
    minutes.insert("09:30", &[
&["he looked at his watch; it was ", "half-past nine", "", "Ambrose Bierce", "A watcher by the dead"],
&["It was ", "nine-thirty", ". In another ten minutes she would turn off the heat; then it would take a while for the water to cool. In the meantime there was nothing to do but wait. “Have you thought it through April?” Never undertake to do a thing until you’ve –“ But she needed no more advice and no more instruction. She was calm and quiet now with knowing what she had always known, what neither her parents not Aunt Claire not Frank nor anyone else had ever had to teach her: that if you wanted to do something absolutely honest, something true, it always turned out to be a thing that had to be done alone.", "Richard Yates", "Revolutionary Road"],
&["The body came in at ", "nine-thirty", " this morning. One of Holding's men went to the house and collected it. There was nothing particularly unusual about the death. The man had had a fear of hospitals and had died at home, being cared for more than adequately by his devoted wife.", "Jackie Kay", "Trumpet"],
&["Up the welcomingly warm morning hill we trudge, side by each, bound finally for the Hall of Fame. It's ", "9.30", ", and time is in fact a-wastin'.", "Richard Ford", "Independence Day"],
]);
    minutes.insert("09:31", &[
&["he looked at his watch; it was ", "half-past nine", "", "Ambrose Bierce", "A watcher by the dead"],
&["It was ", "nine-thirty", ". In another ten minutes she would turn off the heat; then it would take a while for the water to cool. In the meantime there was nothing to do but wait. “Have you thought it through April?” Never undertake to do a thing until you’ve –“ But she needed no more advice and no more instruction. She was calm and quiet now with knowing what she had always known, what neither her parents not Aunt Claire not Frank nor anyone else had ever had to teach her: that if you wanted to do something absolutely honest, something true, it always turned out to be a thing that had to be done alone.", "Richard Yates", "Revolutionary Road"],
&["The body came in at ", "nine-thirty", " this morning. One of Holding's men went to the house and collected it. There was nothing particularly unusual about the death. The man had had a fear of hospitals and had died at home, being cared for more than adequately by his devoted wife.", "Jackie Kay", "Trumpet"],
&["Up the welcomingly warm morning hill we trudge, side by each, bound finally for the Hall of Fame. It's ", "9.30", ", and time is in fact a-wastin'.", "Richard Ford", "Independence Day"],
]);
    minutes.insert("09:32", &[
&["He said he couldn't say for certain of course, but that he rather thought he was. Anyhow, if he wasn't the 11.5 for Kingston, he said he was pretty confident he was the ", "9.32", " for Virginia Water, or the 10 a.m. express for the Isle of Wight, or somewhere in that direction, and we should all know when we got there.", "Jerome K Jerome", "Three Men in a Boat"],
&["Sandy barely made the ", "nine-thirty-two", " and found a seat in no-smoking. She'd been looking forward to this visit with Lisbeth. They hadn't seen each other in months, not since January, when Sandy had returned from Jamaica. And on that day Sandy was sporting a full-blown herpes virus on her lower lip.", "Judy Blume", "Wifey"],
]);
    minutes.insert("09:33", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at twenty-one minutes past three, the half-ebb at a quarter past seven, low water at ", "thirty-three minutes past nine", ", and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("09:34", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at twenty-one minutes past three, the half-ebb at a quarter past seven, low water at ", "thirty-three minutes past nine", ", and half flood at thirty-nine minutes past twelve.", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("09:35", &[
&["", "Nine-thirty-five", ". He really must be gone. The bird is no longer feeding but sitting at the apex of a curl of razor wire.", "Aminatta Forna", "The Memory of Love"],
&["", "Nine-thirty-five", ". He really must be gone. The bird is no longer feeding but sitting at the apex of a curl of razor wire.", "Aminatta Forna", "The Memory of Love"],
]);
    minutes.insert("09:36", &[
&["I grab a pen and the pad of paper by the phone and start scribbling a list for the day. I have an image of myself moving smoothly from task to task, brush in one hand, duster in the other, bringing order to everything. Like Mary Poppins. 9:30-", "9:36", " Make Geigers' bed 9:36-9:42 Take laundry out of machine and put in dryer 9:42-10:00 Clean bathrooms I get to the end and read it over with a fresh surge of optimism. At this rate I should be done easily by lunchtime. 9:36 Fuck. I cannot make this bed. Why won't this sheet lie flat? 9:42 And why do they make mattresses so heavy?", "Sophie Kinsella", "The Undomestic Goddess"],
&["Monday February 6th. '", "9.36am.", " Oh god, Oh god. Maybe he's fallen in love in New York and stayed there'.", "Helen Fielding", "Bridget Jones Diary"],
]);
    minutes.insert("09:37", &[
&["It comprised all that was required of the servant, from eight in the morning, exactly at which hour Phileas Fogg rose, till half-past eleven, when he left the house for the Reform Club - all the details of service, the tea and toast at twenty-three minutes past eight, the shaving-water at ", "thirty-seven minutes past nine", ", and the toilet at twenty minutes before ten.", "Jules Verne", "Around the World in 80 days"],
]);
    minutes.insert("09:38", &[
&["It comprised all that was required of the servant, from eight in the morning, exactly at which hour Phileas Fogg rose, till half-past eleven, when he left the house for the Reform Club - all the details of service, the tea and toast at twenty-three minutes past eight, the shaving-water at ", "thirty-seven minutes past nine", ", and the toilet at twenty minutes before ten.", "Jules Verne", "Around the World in 80 days"],
]);
    minutes.insert("09:39", &[
&["It comprised all that was required of the servant, from eight in the morning, exactly at which hour Phileas Fogg rose, till half-past eleven, when he left the house for the Reform Club - all the details of service, the tea and toast at twenty-three minutes past eight, the shaving-water at ", "thirty-seven minutes past nine", ", and the toilet at twenty minutes before ten.", "Jules Verne", "Around the World in 80 days"],
]);
    minutes.insert("09:40", &[
&["It comprised all that was required of the servant, from eight in the morning, exactly at which hour Phileas Fogg rose, till half-past eleven, when he left the house for the Reform Club—all the details of service, the tea and toast at twenty-three minutes past eight, the shaving-water at thirty-seven minutes past nine, and the toilet at ", "twenty minutes before ten", ".", "Jules Verne", "Around the World in 80 days"],
&["Must have the phone disconnected. Some contractor keeps calling me up about payment for 50 bags of cement he claims I collected ten days ago. Says he helped me load them onto a truck himself. I did drive Whitby's pick-up into town but only to get some lead screening. What does he think I'd do with all that cement? Just the sort of irritating thing you don't expect to hang over your final exit. (Moral: don't try too hard to forget Eniwetok.) Woke ", "9:40", ". To sleep 4:15.", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("09:41", &[
&["It comprised all that was required of the servant, from eight in the morning, exactly at which hour Phileas Fogg rose, till half-past eleven, when he left the house for the Reform Club—all the details of service, the tea and toast at twenty-three minutes past eight, the shaving-water at thirty-seven minutes past nine, and the toilet at ", "twenty minutes before ten", ".", "Jules Verne", "Around the World in 80 days"],
&["Must have the phone disconnected. Some contractor keeps calling me up about payment for 50 bags of cement he claims I collected ten days ago. Says he helped me load them onto a truck himself. I did drive Whitby's pick-up into town but only to get some lead screening. What does he think I'd do with all that cement? Just the sort of irritating thing you don't expect to hang over your final exit. (Moral: don't try too hard to forget Eniwetok.) Woke ", "9:40", ". To sleep 4:15.", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("09:42", &[
&["I grab a pen and the pad of paper by the phone and start scribbling a list for the day. I have an image of myself moving smoothly from task to task, brush in one hand, duster in the other, bringing order to everything. Like Mary Poppins. 9:30-9:36 Make Geigers' bed 9:36-", "9:42", " Take laundry out of machine and put in dryer 9:42-10:00 Clean bathrooms I get to the end and read it over with a fresh surge of optimism. At this rate I should be done easily by lunchtime. 9:36 Fuck. I cannot make this bed. Why won't this sheet lie flat? 9:42 And why do they make mattresses so heavy?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("09:43", &[
&["I grab a pen and the pad of paper by the phone and start scribbling a list for the day. I have an image of myself moving smoothly from task to task, brush in one hand, duster in the other, bringing order to everything. Like Mary Poppins. 9:30-9:36 Make Geigers' bed 9:36-", "9:42", " Take laundry out of machine and put in dryer 9:42-10:00 Clean bathrooms I get to the end and read it over with a fresh surge of optimism. At this rate I should be done easily by lunchtime. 9:36 Fuck. I cannot make this bed. Why won't this sheet lie flat? 9:42 And why do they make mattresses so heavy?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("09:44", &[
&["I grab a pen and the pad of paper by the phone and start scribbling a list for the day. I have an image of myself moving smoothly from task to task, brush in one hand, duster in the other, bringing order to everything. Like Mary Poppins. 9:30-9:36 Make Geigers' bed 9:36-", "9:42", " Take laundry out of machine and put in dryer 9:42-10:00 Clean bathrooms I get to the end and read it over with a fresh surge of optimism. At this rate I should be done easily by lunchtime. 9:36 Fuck. I cannot make this bed. Why won't this sheet lie flat? 9:42 And why do they make mattresses so heavy?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert(
        "09:45",
        &[&[
            "9.15, 9.30, ",
            "9.45",
            ", 10! Bond felt the excitement ball up inside him like cat's fur.",
            "Ian Fleming",
            "On Her Majesty's Secret Service",
        ]],
    );
    minutes.insert(
        "09:46",
        &[&[
            "9.15, 9.30, ",
            "9.45",
            ", 10! Bond felt the excitement ball up inside him like cat's fur.",
            "Ian Fleming",
            "On Her Majesty's Secret Service",
        ]],
    );
    minutes.insert(
        "09:47",
        &[&[
            "Monday February 6th. '",
            "9.47am.",
            " Or gone to Las Vegas and got married'.",
            "Helen Fielding",
            "Bridget Jones Diary",
        ]],
    );
    minutes.insert(
        "09:48",
        &[&[
            "Monday February 6th. '",
            "9.47am.",
            " Or gone to Las Vegas and got married'.",
            "Helen Fielding",
            "Bridget Jones Diary",
        ]],
    );
    minutes.insert(
        "09:49",
        &[&[
            "Monday February 6th. '",
            "9.47am.",
            " Or gone to Las Vegas and got married'.",
            "Helen Fielding",
            "Bridget Jones Diary",
        ]],
    );
    minutes.insert("09:50", &[
&["", "9.50am.", " Hmmm. Think will go inspect make-up in case he does come in", "Helen Fielding", "Bridget Jones Diary"],
&["", "Ten minutes to ten.", " \"I had just time to hide the bottle (after the nurse had left me) when you came into my room.\"", "Wilkie Collins", "The Law and the Lady"],
]);
    minutes.insert("09:51", &[
&["", "9.50am.", " Hmmm. Think will go inspect make-up in case he does come in", "Helen Fielding", "Bridget Jones Diary"],
&["", "Ten minutes to ten.", " \"I had just time to hide the bottle (after the nurse had left me) when you came into my room.\"", "Wilkie Collins", "The Law and the Lady"],
]);
    minutes.insert("09:52", &[
&["\"She caught the ", "9:52", " to Victoria. I kept well clear of her on the train and picked her up as she went through the barrier. Then she took a taxi to Hammersmith.\" \"A taxi?\" Smiley interjected. \"She must be out of her mind.\"", "John le Carre", "Call for the Dead"],
]);
    minutes.insert("09:53", &[
&["Miss Pettigrew went to the bus-stop to await a bus. She could not afford the fare, but she could still less afford to lose a possible situation by being late. The bus deposited her about five minutes' walk from Onslow Mansions, an at ", "seven minutes to ten", " precisely she was outside her destination.", "Winifred Watson", "Miss Pettigrew Lives for a Day"],
]);
    minutes.insert("09:54", &[
&["", "9:54", " This is sheer torture. My arms have never ached so much in my entire life. The blankets weigh a ton, and the sheets won't go straight and I have no idea how to do the wretched corners. How do chambermaids do it?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("09:55", &[
&["At ", "five to ten", " I'm ready in the hall. Nathaniel's mother's house is nearby but apparently tricky to find, so the plan is to meet here and he'll walk me over. I check my reflection in the hall mirror and wince. The streak of bleach in my hair is as obvious as ever. Am I really going out in public like this?", "Sophie Kinsella", "The Undomestic Goddess"],
&["Good-morning, Lucien, good-morning, said Albert; \"your punctuality really alarms me. What do I say? punctuality! You, whom I expected last, you arrive at ", "five minutes to ten", ", when the time fixed was half-past! Has the ministry resigned?\"", "Alexandre Dumas", "The Count of Monte Cristo"],
]);
    minutes.insert("09:56", &[
&["At ", "five to ten", " I'm ready in the hall. Nathaniel's mother's house is nearby but apparently tricky to find, so the plan is to meet here and he'll walk me over. I check my reflection in the hall mirror and wince. The streak of bleach in my hair is as obvious as ever. Am I really going out in public like this?", "Sophie Kinsella", "The Undomestic Goddess"],
&["Good-morning, Lucien, good-morning, said Albert; \"your punctuality really alarms me. What do I say? punctuality! You, whom I expected last, you arrive at ", "five minutes to ten", ", when the time fixed was half-past! Has the ministry resigned?\"", "Alexandre Dumas", "The Count of Monte Cristo"],
]);
    minutes.insert("09:57", &[
&["At ", "five to ten", " I'm ready in the hall. Nathaniel's mother's house is nearby but apparently tricky to find, so the plan is to meet here and he'll walk me over. I check my reflection in the hall mirror and wince. The streak of bleach in my hair is as obvious as ever. Am I really going out in public like this?", "Sophie Kinsella", "The Undomestic Goddess"],
&["Good-morning, Lucien, good-morning, said Albert; \"your punctuality really alarms me. What do I say? punctuality! You, whom I expected last, you arrive at ", "five minutes to ten", ", when the time fixed was half-past! Has the ministry resigned?\"", "Alexandre Dumas", "The Count of Monte Cristo"],
]);
    minutes.insert("09:58", &[
&["I didn't sleep too long, because I think it was only ", "around ten o'clock", " when I woke up. I felt pretty hungry as soon as I had a cigarette. The last time I'd eaten was those two hamburgers I had with Brossard and Ackley when we went in to Agerstown to the movies. That was a long time ago. It seemed like fifty years ago.", "J.D. Salinger", "Catcher in the Rye"],
]);
    minutes.insert("09:59", &[
&["", "One minute to ten.", " With a heavy heart Bert watched the clock. His legs were still aching very badly. He could not see the hands of the clock moving, but they were creeping on all the same.", "Robert Tressell", "The Ragged Trouserred Philanthropists"],
]);
    minutes.insert("10:00", &[
&["––In assaying to put on his regimental coat and waistcoat, my uncle Toby found the same objection in his wig, ––so that went off too: ––So that with one thing and what with another, as always falls out when a man is in the most haste, ––'twas ", "ten o'clock", ", which was half an hour later than his usual time before my uncle Toby sallied out.", "Laurence Sterne", "Tristram Shandy"],
&["’Tis but ", "an hour ago since it was nine", ", And after one hour more ‘twill be eleven.", "William Shakespeare", "As You Like It"],
&["For some seconds the light went on becoming brighter and brighter, and she saw everything more and more clearly and the clock ticked louder and louder until there was a terrific explosion right in her ear. Orlando leapt as if she had been violently struck on the head. ", "Ten", " times she was struck. In fact it was ten o'clock in the morning. It was the eleventh of October. It was 1928. It was the present moment.", "Virginia Woolf", "Orlando"],
&["The trial was irretrievably over; everything that could be said had been said, but he had never doubted that he would lose. The written verdict was handed down at ", "10:00", " on Friday morning, and all that remained was a summing up from the reporters waiting in the corridor outside the district court.", "Stieg Larsson", "The Girl with the Dragon Tattoo"],
&["According to military records no US bombers or any other kind of aircraft were flying over that region at the time, that is around ", "10 am", " on November 7,1944.", "Haruki Murakami", "Kafka on the shore"],
&["At about ", "ten o'clock", " in the morning the sun threw a bright dust-laden bar through one of the side windows, and in and out of the beam flies shot like rushing stars.", "John Steinbeck", "Of Mice And Men"],
&["I went to bed and the next thing I knew I was awake again and it was getting on for ", "ten o' clock", " in the morning. Ring, ring, said the telephone, ring, ring.", "Russell Hoban", "The Medusa Frequency"],
&["If Wednesday should ever come! It did come, and exactly when it might be reasonably looked for. It came - it was fine - and Catherine trod on air. By ", "ten o'clock", ", the chaise and four conveyed the two from the abbey; and, after an agreeable drive of almost twenty miles, they entered Woodston, a large and populous village, in a situation not unpleasant.", "Jane Austen", "Northanger Abbey"],
&["King Richard: Well, but what's o'clock? Buckingham: Upon the stroke of ", "ten", ".", "William Shakespeare", "Richard III"],
&["Monday 30 March 1668 Up betimes, and so to the office, there to do business till about ", "10 o’clock", "", "Samuel Pepys", "The Diary of Samuel Pepys"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At ", "10 o'clock", " she is lowered into her cool grave, at 7:30 that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The pundit sighed. 'Only a fool like me would leave his door open when a riot can occur at any moment, and only a fool like me would say yes to you,' he said. 'What time?' Just his head was sticking out of the partially opened door. The money from blessing the ice-cream factory must have dulled his desire for work, I thought. 'Ten.' '", "Ten-thirty", ".' Without another word, he closed the door.", "Akhil Sharma", "An Obedient Father"],
&["The Saturday immediately preceding the examinations was a very busy day for Kennedy. At ", "ten o' clock", " he was entering Willey's room; the latter had given him a key and left the room vacant by previous arrangement - in fact he had taken Olivia on another house hunting trip.", "Barry Unsworth", "The Greeks have a word for it"],
&["The summer holidays were near at hand when I made up my mind to break out of the weariness of school-life for one day at least. With Leo Dillon and a boy named Mahoney I planned a day's mitching. Each of us saved up sixpence. We were to meet ", "at ten", " in the morning on the Canal Bridge.", "James Joyce", "Dubliners"],
&["The written verdict was handed down at ", "10:00", " on Friday morning, and allthat remained was a summing up from the reporters waiting in the corridor outside the district court.", "Stieg Larsson", "The girl with the dragon tattoo"],
]);
    minutes.insert("10:01", &[
&["At ", "about ten o'clock", " in the morning the sun threw a bright dust-laden bar through one of the side windows, and in and out of the beam flies shot like rushing stars.'", "John Steinbeck", "Of Mice And Men"],
]);
    minutes.insert("10:02", &[
&["It was ", "two minutes after ten", "; she was not satisfied with her clothes, her face, her apartment. She heated the coffee again and sat down in the chair by the window. Can't do anything more now, she thought, no sense trying to improve anything the last minute.", "Shirley Jackson", "The Daemon Lover"],
]);
    minutes.insert("10:03", &[
&["It's ", "10.03", " according to his watch, and he is travelling down through the Scottish highlands to Inverness, tired and ever-so-slightly anxious in case he falls asleep between now and when the train reaches the station, and misses his cue to say to Alice, Drew and Aleesha: 'OK, this is Inverness, let's move it.'", "Michel Faber", "“Vanilla-Bright like Eminem” from The Farenheit Twins"],
&["The date was the 14th of May and the clock on his desk said the time was twenty three minutes past ten, so he tapped in the numbers 10.23. At least, that's what he meant to do. In fact he typed in the numbers ", "10.03", ".", "Andrew Norriss", "Ctrl-Z"],
]);
    minutes.insert("10:04", &[
&["It's ", "10.03", " according to his watch, and he is travelling down through the Scottish highlands to Inverness, tired and ever-so-slightly anxious in case he falls asleep between now and when the train reaches the station, and misses his cue to say to Alice, Drew and Aleesha: 'OK, this is Inverness, let's move it.'", "Michel Faber", "“Vanilla-Bright like Eminem” from The Farenheit Twins"],
&["The date was the 14th of May and the clock on his desk said the time was twenty three minutes past ten, so he tapped in the numbers 10.23. At least, that's what he meant to do. In fact he typed in the numbers ", "10.03", ".", "Andrew Norriss", "Ctrl-Z"],
]);
    minutes.insert("10:05", &[
&["We both watch as a pair of swans sail regally under the little bridge. Then I glance at my watch. It's already ", "five past ten", ". “We should get going,” I say with a little start. Your mother will be waiting.” “There's no rush,” Nathaniel calls as I hasten down the other side of the bridge. “We've got all day.” He lopes down the bridge. “It's OK. You can slow down.” I try to match his relaxed pace. But I'm not used to this easy rhythm. I'm used to striding along crowded pavements, fighting my way, pushing and elbowing.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("10:06", &[
&["We both watch as a pair of swans sail regally under the little bridge. Then I glance at my watch. It's already ", "five past ten", ". “We should get going,” I say with a little start. Your mother will be waiting.” “There's no rush,” Nathaniel calls as I hasten down the other side of the bridge. “We've got all day.” He lopes down the bridge. “It's OK. You can slow down.” I try to match his relaxed pace. But I'm not used to this easy rhythm. I'm used to striding along crowded pavements, fighting my way, pushing and elbowing.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("10:07", &[
&["", "10.07 am", ": In a meeting with Rod, Momo and Guy. We are rehearsing the final for the third time, with Rod and Guy taking the parts of the clients, when Rod's secretary, Lorraine, bursts in.", "Allison Pearson", "I Don't Know How She Does It"],
]);
    minutes.insert("10:08", &[
&["", "10.07 am", ": In a meeting with Rod, Momo and Guy. We are rehearsing the final for the third time, with Rod and Guy taking the parts of the clients, when Rod's secretary, Lorraine, bursts in.", "Allison Pearson", "I Don't Know How She Does It"],
]);
    minutes.insert(
        "10:09",
        &[&[
            "He followed the squeals down a hallway. A wall clock read 8:09-",
            "10:09",
            " Dallas time.",
            "James Ellroy",
            "American Tabloid",
        ]],
    );
    minutes.insert("10:10", &[
&["", "10:10", " Shot is fired.", "John Dickson Carr", "The Hollow Man"],
&["Saturday morning was bright and sunny, and at ", "ten minutes past 10", " Donald arrived at the Embankment entrance of Charing Cross Underground Station, carrying a small suitcase full of clothes suitable for outdoor sports and pastimes.", "AG Macdonell", "England, Their England"],
]);
    minutes.insert("10:11", &[
&["Through the curtained windows of the furnished apartment which Mrs. Horace Hignett had rented for her stay in New York rays of golden sunlight peeped in like the foremost spies of some advancing army. It was a fine summer morning. The hands of the Dutch clock in the hall pointed to thirteen minutes past nine; those of the ormolu clock in the sitting-room to ", "eleven minutes past ten", "; those of the carriage clock on the bookshelf to fourteen minutes to six. In other words, it was exactly eight; and Mrs. Hignett acknowledged the fact by moving her head on the pillow, opening her eyes, and sitting up in bed. She always woke at eight precisely.", "P.G. Wodehouse", "Three Men and a Maid"],
]);
    minutes.insert("10:12", &[
&["“I'll take the coffee tray out,” I suggest humbly. As I pick it up I glance again at my watch. ", "Ten twelve", ". I wonder if they've started the meeting.", "Sophie Kinsella", "The Undomestic Goddess"],
&["He stood up once, early on, to lock his office door, and then he was reading the last page, and it was exactly ", "10:12 a.m.", ", and the sun beating on his office windows was a different sun from the one he'd always known.", "Jonathan Franzen", "Freedom"],
]);
    minutes.insert("10:13", &[
&["\"By the bye,\" said the first, \"I was able this morning to telegraph the very words of the order to my cousin at seventeen minutes past ten.\" \"And I sent it to the Daily Telegraph at ", "thirteen minutes past ten", ".\" \"Bravo, Mr. Blount!\" \"Very good, M. Jolivet.\"", "Jules Verne", "Michel Strogoff"],
]);
    minutes.insert("10:14", &[
&["“Okay. ", "Ten fourteen", ": Mrs. Narada reports that her cat has been attacked by a large dog. Now I send all the boys out looking, but they don't find anything until eleven. Then one of them calls in that a big dog has just bitten holes in the tires on his golf cart and run off. Eleven thirty: Dr. Epstein makes his first lost-nap call: dog howling. Eleven thirty-five: Mrs. Norcross is putting the kids out on the deck for some burgers when a big dog jumps over the rail, eats the burgers, growls at the kids, runs off. First mention of lawsuit.”", "Christopher Moore", "Coyote Blue"],
]);
    minutes.insert("10:15", &[
&["At ", "10.15", " Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked she asked Linda at twenty-five past eleven what time it was. Linda looked at her watch and replied that it was a quarter to twelve.", "Agatha Christie", "Evil under the sun"],
]);
    minutes.insert("10:16", &[
&["", "10:16", " At last. Forty minutes of hard work and I have made precisely one bed. I'm way behind. But never mind. Just keep moving. Laundry next.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("10:17", &[
&["\"By the bye,\" said the first, \"I was able this morning to telegraph the very words of the order to my cousin at ", "seventeen minutes past ten", ".\" \"And I sent it to the Daily Telegraph at thirteen minutes past ten.\"n \"Bravo, Mr. Blount!\" \"Very good, M. Jolivet.\" \"I will try and match that!\"", "Jules Verne", "Michel Strogoff"],
]);
    minutes.insert(
        "10:18",
        &[&[
            "I know that it was ",
            "10:18",
            " when I got home because I look at my watch a lot.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert(
        "10:19",
        &[&[
            "I know that it was ",
            "10:18",
            " when I got home because I look at my watch a lot.",
            "Jonathan Safran Foer",
            "Extremely Loud and Incredibly Close",
        ]],
    );
    minutes.insert("10:20", &[
&["How much is the clock fast now? His mother straightened the battered alarm clock that was lying on its side in the middle of the mantelpiece until its dial showed a quarter to twelve and then laid it once more on its side. An hour and twenty-five minutes, she said. The right time now is ", "twenty past ten", ".", "James Joyce", "A Portrait of the Artist as a Young Man"],
]);
    minutes.insert("10:21", &[
&["Liz Headleand stares into the mirror, as though entranced. She does not see herself or the objects on her dressing-table. The clock abruptly jerks to ", "10.21", ".", "Margaret Drabble", "The Radiant Way"],
]);
    minutes.insert("10:22", &[
&["I listened to them, and listened to them again, and then before I had time to figure out what to do, or even what to think or feel, the phone started ringing. It was ", "10:22", ":27. I looked at the caller ID and saw that it was him.", "Jonathan Safran Foer", "Extremely Loud and Incredibly Close"],
]);
    minutes.insert("10:23", &[
&["The date was the 14th of May and the clock on his desk said the time was ", "twenty three minutes past ten", ", so he tapped in the numbers 10.23. At least, that's what he meant to do. In fact he typed in the numbers 10.03", "Andrew Norriss", "Ctrl-Z"],
]);
    minutes.insert("10:24", &[
&["The date was the 14th of May and the clock on his desk said the time was ", "twenty three minutes past ten", ", so he tapped in the numbers 10.23. At least, that's what he meant to do. In fact he typed in the numbers 10.03", "Andrew Norriss", "Ctrl-Z"],
]);
    minutes.insert("10:25", &[
&["", "10:25", ": Phone call from Lüding, very worked up, urging me to return at once and get in touch with Alois, who was equally worked up.", "Heinrich Böll", "The Lost Honour of Katharina Blum"],
&["One meal is enough now, topped up with a glucose shot. Sleep is still 'black', completely unrefreshing. Last night I took a 16 mm. film of the first three hours, screened it this morning at the lab. The first true-horror movie. I looked like a half-animated corpse. Woke ", "10:25", ". To sleep 3:45.", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("10:26", &[
&["", "10:26", " No. Please, no. I can hardly bear to look. It's a total disaster. Everything in the washing machine has gone pink. Every single thing. What happened? With trembling fingers I pick out a damp cashmere cardigan. It was a cream when I put it in. It's now a sickly shade of candy floss. I knew K3 was bad news. I knew it -", "Sophie Kinsella", "The Undomestic Goddess"],
&["In the exact centre of my visual field was the alarm clock, hands pointing to ", "ten-twenty-six", ". An alarm clock I received as a memento of somebody's wedding.", "Haruki Murakami", "Hard-boiled Wonderland and The End of the World"],
]);
    minutes.insert("10:27", &[
&["Mr. Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses, and biscuits.", "AG Macdonell", "England, Their England"],
&["She is on holiday in Norfolk. The substandard clock radio says ", "10:27 a.m.", " The noise is Katrina the Cleaner thumping the hoover against the skirting boards and the bedroom doors. Her hand is asleep. It is still hooked through the handstrap of the camera. She unhooks it and shakes it to get the blood back into it. She puts her feet on top of her trainers and slides them across the substandard carpet. It has the bare naked feet of who knows how many hundreds of dead or old people on it.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("10:28", &[
&["Mr. Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses, and biscuits.", "AG Macdonell", "England, Their England"],
&["She is on holiday in Norfolk. The substandard clock radio says ", "10:27 a.m.", " The noise is Katrina the Cleaner thumping the hoover against the skirting boards and the bedroom doors. Her hand is asleep. It is still hooked through the handstrap of the camera. She unhooks it and shakes it to get the blood back into it. She puts her feet on top of her trainers and slides them across the substandard carpet. It has the bare naked feet of who knows how many hundreds of dead or old people on it.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("10:29", &[
&["Mr. Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses, and biscuits.", "AG Macdonell", "England, Their England"],
&["She is on holiday in Norfolk. The substandard clock radio says ", "10:27 a.m.", " The noise is Katrina the Cleaner thumping the hoover against the skirting boards and the bedroom doors. Her hand is asleep. It is still hooked through the handstrap of the camera. She unhooks it and shakes it to get the blood back into it. She puts her feet on top of her trainers and slides them across the substandard carpet. It has the bare naked feet of who knows how many hundreds of dead or old people on it.", "Ali Smith", "The Accidental"],
]);
    minutes.insert("10:30", &[
&["", "10.30 a.m.", " Break", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["according to the clock on the wall, it is barely ", "ten thirty", ".", "Michael Cunningham", "The Hours"],
&["At ", "ten-thirty", " I'm cleaned up, shaved and dressed in my Easter best - a two-piece seersucker Palm Beach I've had since college.", "Richard Ford", "The Sportswriter"],
]);
    minutes.insert(
        "10:31",
        &[&[
            "\"If you please. You went to bed at what time, Madame?\" \"",
            "Just after half past ten.",
            "\"",
            "Agatha Christie",
            "Death on the Nile",
        ]],
    );
    minutes.insert(
        "10:32",
        &[&[
            "\"If you please. You went to bed at what time, Madame?\" \"",
            "Just after half past ten.",
            "\"",
            "Agatha Christie",
            "Death on the Nile",
        ]],
    );
    minutes.insert(
        "10:33",
        &[&[
            "\"If you please. You went to bed at what time, Madame?\" \"",
            "Just after half past ten.",
            "\"",
            "Agatha Christie",
            "Death on the Nile",
        ]],
    );
    minutes.insert(
        "10:34",
        &[&[
            "\"If you please. You went to bed at what time, Madame?\" \"",
            "Just after half past ten.",
            "\"",
            "Agatha Christie",
            "Death on the Nile",
        ]],
    );
    minutes.insert("10:35", &[
&["", "Five-and-twenty to eleven", ". A horrible hour - a macabre hour, for it is not only the hour of pleasure ended, it is the hour when pleasure itself has been found wanting.", "Patrick Hamilton", "Rope"],
]);
    minutes.insert("10:36", &[
&["\"Strand post mark and dispatched ", "ten thirty-six", "\" said Holmes reading it over and over. \"Mr Overton was evidently considerably excited when he sent it over and somewhat incoherent in consequence.\"", "Arthur Conan Doyle", "The Adventure of the Missing Three-Quarter"],
]);
    minutes.insert("10:37", &[
&["I quite agree with you,' said Mr Murbles. 'It is a most awkward situation. Lady Dormer died at precisely ", "10.37 a.m.", " on November 11th.'", "Dorothy L. Sayers", "The Unpleasantness at the Bellona Club"],
]);
    minutes.insert("10:38", &[
&["There must be a solution, there must be. Frantically I scan the cans of products stacked on the shelves. Stain Away. Vanish. There has to be a remedy. ... I just need to think. ... ", "10:38", " OK, I have the answer. It may not totally work—but it's my best shot.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("10:39", &[
&["There must be a solution, there must be. Frantically I scan the cans of products stacked on the shelves. Stain Away. Vanish. There has to be a remedy. ... I just need to think. ... ", "10:38", " OK, I have the answer. It may not totally work—but it's my best shot.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert(
        "10:40",
        &[&[
            "",
            "10:40",
            ": Call from Katharina asking me whether I had really said what was in the News.",
            "Heinrich Böll",
            "The Lost Honour of Katharina Blum",
        ]],
    );
    minutes.insert(
        "10:41",
        &[&[
            "",
            "10:40",
            ": Call from Katharina asking me whether I had really said what was in the News.",
            "Heinrich Böll",
            "The Lost Honour of Katharina Blum",
        ]],
    );
    minutes.insert(
        "10:42",
        &[&[
            "",
            "10:40",
            ": Call from Katharina asking me whether I had really said what was in the News.",
            "Heinrich Böll",
            "The Lost Honour of Katharina Blum",
        ]],
    );
    minutes.insert("10:43", &[
&["24 January, ", "10.43 a.m", ": one month and two days later I wonder if I should worry about the fact that my darling boyfriend bought me a birthday present that has the potential to cause instant death...", "Jane Costello", "The Wish List"],
]);
    minutes.insert("10:44", &[
&["24 January, ", "10.43 a.m", ": one month and two days later I wonder if I should worry about the fact that my darling boyfriend bought me a birthday present that has the potential to cause instant death...", "Jane Costello", "The Wish List"],
]);
    minutes.insert("10:45", &[
&["If this is so, we have now to determine what Barker and Mrs. Douglas, presuming they are not the actual murderers, would have been doing from ", "quarter to eleven", ", when the sound of the shot brought them down, until quarter past eleven, when they rang for the bell and summoned the servants'.", "Arthur Conan Doyle", "The Valley of Fear"],
&["They reached King's Cross at a ", "quarter to eleven", ". Mr Weasley dashed across the road to get trolleys for their trunks and they all hurried into the station.", "J.K.Rowling", "Harry Potter and the Chamber of Secrets"],
]);
    minutes.insert("10:46", &[
&["If this is so, we have now to determine what Barker and Mrs. Douglas, presuming they are not the actual murderers, would have been doing from ", "quarter to eleven", ", when the sound of the shot brought them down, until quarter past eleven, when they rang for the bell and summoned the servants'.", "Arthur Conan Doyle", "The Valley of Fear"],
&["They reached King's Cross at a ", "quarter to eleven", ". Mr Weasley dashed across the road to get trolleys for their trunks and they all hurried into the station.", "J.K.Rowling", "Harry Potter and the Chamber of Secrets"],
]);
    minutes.insert(
        "10:47",
        &[&[
            "He whistles in the shower. It is ",
            "10.47",
            " and he is ready for the off.",
            "Jackie Kay",
            "Trumpet",
        ]],
    );
    minutes.insert("10:48", &[
&["At ", "10.48am", ", I closed my folder but didn't bother putting it back in my bag, so you knew I was on my way to a committee or meeting room nearby. Before I stood up, I folded my paper napkin and put it and the spoon into my coffee cup, a neat sort of person, you thought.", "Louise Doughty", "Apple Tree Yard"],
]);
    minutes.insert("10:49", &[
&["By ", "forty-nine minutes past ten", ", we fall in again with a fine portion of the ancient road, which the modern track constantly follows, and descend by some steep windings, hewn in the side of a precipitous cliff, to the place where the Ouad-el-Haoud commences.", "Félicien de Saulcy", "Narrative of a Journey round the Dead Sea and in the Bible lands in 1850 and 1851"],
]);
    minutes.insert("10:50", &[
&["", "10.50 a.m.", " Art class with Mrs Peters", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["As he walked back to the flight office, airmen were forming a line to await the arrival of the NAAFI van with morning tea and cakes. Lambert looked at his watch; it was ", "ten to eleven", ".", "Len Deighton", "Bomber"],
]);
    minutes.insert("10:51", &[
&["", "10.50 a.m.", " Art class with Mrs Peters", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["As he walked back to the flight office, airmen were forming a line to await the arrival of the NAAFI van with morning tea and cakes. Lambert looked at his watch; it was ", "ten to eleven", ".", "Len Deighton", "Bomber"],
]);
    minutes.insert("10:52", &[
&["", "10.50 a.m.", " Art class with Mrs Peters", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["As he walked back to the flight office, airmen were forming a line to await the arrival of the NAAFI van with morning tea and cakes. Lambert looked at his watch; it was ", "ten to eleven", ".", "Len Deighton", "Bomber"],
]);
    minutes.insert("10:53", &[
&["He begins to make a record of our observations.'", "10.53 hrs", ",' he writes, as we crouch at the top of the stairs, listening to his mother in the hall below.", "Michael Frayn", "Spies"],
&["I gaze and gaze again at that face, which seems to me both strange and familiar, said Austerlitz, I run the tape back repeatedly, looking at the time indicator in the top left-hand corner of the screen, where the figures covering part of her forehead show the minutes and seconds, from ", "10:53", " to 10:57, while the hundredths of a second flash by so fast that you cannot read and capture them.", "W.G. Sebald", "Austerlitz"],
]);
    minutes.insert("10:54", &[
&["He begins to make a record of our observations.'", "10.53 hrs", ",' he writes, as we crouch at the top of the stairs, listening to his mother in the hall below.", "Michael Frayn", "Spies"],
&["I gaze and gaze again at that face, which seems to me both strange and familiar, said Austerlitz, I run the tape back repeatedly, looking at the time indicator in the top left-hand corner of the screen, where the figures covering part of her forehead show the minutes and seconds, from ", "10:53", " to 10:57, while the hundredths of a second flash by so fast that you cannot read and capture them.", "W.G. Sebald", "Austerlitz"],
]);
    minutes.insert(
        "10:55",
        &[&[
            "The clock was still saying ",
            "five minutes to eleven",
            " when Pooh and Piglet set out on their way half an hour later.",
            "AA Milne",
            "The House at Pooh Corner",
        ]],
    );
    minutes.insert(
        "10:56",
        &[&[
            "The clock was still saying ",
            "five minutes to eleven",
            " when Pooh and Piglet set out on their way half an hour later.",
            "AA Milne",
            "The House at Pooh Corner",
        ]],
    );
    minutes.insert("10:57", &[
&["I run the tape back repeatedly, looking at the time indicator in the top left-hand corner of the screen, where the figures covering part of her forehead show the minutes and seconds, from 10.53 to ", "10.57", ".", "W. G. Sebald", "Austerlitz"],
]);
    minutes.insert(
        "10:58",
        &[&[
            "One day Joe was sitting in the office waiting for his 11 o'clock appointment, and at ",
            "10:58",
            " this black gal came in.",
            "Helen DeWitt",
            "Lightning Rods",
        ]],
    );
    minutes.insert("10:59", &[
&["Harry grunted in his sleep and his face slid down the window an inch or so, making his glasses still more lopsided, but he did not wake up. An alarm clock, repaired by Harry several years ago, ticked loudly on the sill, showing ", "one minute to eleven", ".", "J. K. Rowling", "Harry Potter and the Half-Blood Prince"],
]);
    minutes.insert("11:00", &[
&["'Who can - what can -” asked Mrs Dalloway (thinking it was outrageous to be interrupted at ", "eleven o'clock", " on the morning of the day she was giving a party), hearing a step on the stairs.", "Virginia Woolf", "Mrs Dalloway"],
&["\"By ", "11 o'clock", " I have finished the first chapter of Mr Y. The winter sun is peeping meekly through the thin curtains and I decide to get up\"", "Scarlett Thomas", "The End of Mr Y"],
&["He was rather a long time, and I began to feel muffled, weighed down by thick stuffs and silence. I thought: He'll never come back; and when he did his figure seemed to come at me from very far away, dream-like and dwindled, making his way back along a tunnel...I dare say it was champagne ", "at eleven", " in the morning.", "Rosamond Lehmann", "The Weather in the Streets"],
&["As her husband had told him, she was still in bed although it was past ", "11 o'clock", ". Her normally mobile face was encased in clay, rigid and menacing as an Aztec mask.", "Evelyn Waugh", "Scoop"],
&["As they looked the whole world became perfectly silent, and a flight of gulls crossed the sky, first one gull leading, then another, and in this extraordinary silence and peace, in this pallor, in this purity, bells struck ", "eleven", " times the sound fading up there among the gulls.", "Virginia Woolf", "Mrs. Dalloway"],
&["At ", "eleven o'clock", " in the morning, large flakes had appeared from a colourless sky and invaded the fields, gardens and lawns of Romerike like an armada from outer space.", "Jo Nesbo", "The Snowman"],
&["At ", "eleven o'clock", " the phone rang, and still the figure did not respond, any more than it had responded when the phone had rung at twenty-five to seven, and again for ten minutes continuously starting at five to seven...", "Douglas Adams", "The Long Dark Tea-Time of the Soul"],
&["Big Ben was striking as she stepped out into the street. It was ", "eleven o'clock", " and the unused hour was fresh as if issued to children on a beach.", "Virginia Woolf", "Mrs Dalloway"],
&["It was about ", "eleven o'clock", " in the morning, mid October, with the sun not shining and a look of hard wet rain in the clearness of the foothills. I was wearing my powder-blue suit, with dark blue shirt, tie and display handkerchief, black brogues, black wool socks with dark blue clocks on them. I was neat, clean, shaved and sober, and I didn't care who knew it. I was everything the well-dressed private detective ought to be. I was calling on four millon dollars.", "Raymond Chandler", "The big sleep"],
&["My sister is terrified that I might write and tell all the family secrets. Why do I feel like a rebel, like an iconoclast? I am only trying to do a writing class, what is wrong with that? I keep telling myself that once in the car I will be fine, I can listen to Radio Four Woman’s Hour and that will take me till ", "eleven", " o’clock when the class starts.", "Patsy Hickman", "The Saints"],
&["ON the morning following the events just narrated, Mrs. Arlington was seated at breakfast in a sweet little parlour of the splendid mansion which the Earl of Warrington had taken and fitted up for her in Dover Street, Piccadilly. It was about ", "eleven o'clock", "; and the Enchantress was attired in a delicious deshabillé. With her little feet upon an ottoman near the fender, and her fine form reclining in a luxurious large arm-chair, she divided her attention between her chocolate and the columns of the Morning Herald. She invariably prolonged the morning's repast as much as possible, limply because it served to wile away the time until the hour for dressing arrived.", "G.W.M. Reynolds", "The Mysteries of London"],
&["Quiet as I am, I become at ", "Eleven o'Clock", " in the Morning on every day of the week save Sunday a raving, ranting maniac -- a dangerous lunatic, panting with insane desires to do, not only myself but other people, a mischief, and possessed less by hallucination than by rabies.", "George Augustus Sala", "Twice Around the Clock"],
&["Though perhaps' – but here the bracket clock whirred and then hectically struck ", "eleven", ", its weights spooling downwards at the sudden expense of energy. She had to sit for a moment, when the echo had vanished, to repossess her thoughts.", "Alan Hollinghurst", "The Stranger's Child"],
&["We got to Waterloo ", "at eleven", ", and asked where the eleven-five started from. Of course nobody knew; nobody at Waterloo ever does know where a train is going to start from, or where a train when it does start is going to, or anything about it.", "Jerome K Jerome", "Three Men in a Boat"],
&["We got to Waterloo ", "at eleven", ", and asked where the eleven-five started from.Of course nobody knew; nobody at Waterloo ever does know where a train is going to start from, or where a train when it does start is going to, or anything about it.", "Jerome K Jerome", "Three Men in a Boat"],
&["We passed a few sad hours until ", "eleven o'clock", ", when the trial was to commence. My father and the rest of the family being obliged to attend as witnesses, I accompanied them to the court. During the whole of this wretched mockery of justice I suffered living torture.", "Mary Shelley", "Frankenstein"],
]);
    minutes.insert("11:01", &[
&["O'Neil rises and takes the tray. He has finished the tea, but the muffins are still here in a wicker basket covered with a blue napkin. The clock above the stove says that it is ", "just past eleven", ", and guests will be arriving at the house now.", "Justin Cronin", "Mary and O'Neil"],
]);
    minutes.insert("11:02", &[
&["On August 9th, three days later, at ", "11.02am", ", another B−29 dropped the second bomb on the industrial section of the city of Nagasaki, totally destroying 1 1/2 square miles of the city, killing 39,000 persons and injuring 25,000 more.", "The Manhattan Engineer District", "The Atomic Bombings of Hiroshima and Nagasaki"],
]);
    minutes.insert("11:03", &[
&["\"What makes you think it's for real?\" \"Just a hunch, really. He sounded for real. Sometimes you can just tell about people\"-he smiled-\"even if you're a dull old WASP.\" \"I think it's a setup.\" \"Why?\" \"I just do. Why would someone from the government want to help you?\" \"Good question. Guess I'll find out.\" She went back into the kitchen.\"What time are you meeting him?\" she called out. \"", "Eleven oh-three", ",\" he said. \"That made me think he's for real. Military and intelligence types set precise appointment times to eliminate confusion and ambiguity. Nothing ambiguous about eleven oh-three.\"", "Christopher Buckley", "Little Green Men"],
&["On the fourth, at ", "11.03am", ", the editor of the Yidische Zaitung put in a call to him; Doctor Yarmolinsky did not answer. He was found in his room, his face already a little dark, nearly nude beneath a large, anachronistic cape.", "Jorge Luis Borges", "Death and the Compass"],
]);
    minutes.insert(
        "11:04",
        &[&[
            "As her husband had told him, she was still in bed although it was ",
            "past 11 o'clock",
            ". Her normally mobile face was encased in clay, rigid and menacing as an Aztec mask.",
            "Evelyn Waugh",
            "Scoop",
        ]],
    );
    minutes.insert("11:05", &[
&["July 3: 5 3/4 hours. Little done today. Deepening lethargy, dragged myself over to the lab, nearly left the road twice. Concentrated enough to feed the zoo and get the log up to date. Read through the operating manuals Whitby left for the last time, decided on a delivery rate of 40 rontgens/min., target distance of 530 cm. Everything is ready now. Woke ", "11:05", ". To sleep 3:15.", "JG Ballard", "The Voices of Time"],
&["Sansom arrived in a Town Car at ", "five past eleven", ". Local plates, which meant he had ridden up most of the way on the train. Less convenient for him, but a smaller carbon footprint than driving all the way, or flying. Every detail mattered, in a campaign.", "Lee Child", "Gone Tomorrow"],
]);
    minutes.insert("11:06", &[
&["", "11:06", " And ... oh. The ironing. What am I going to do about that?", "Sophie Kinsella", "The Undomestic Goddess"],
&["Despite the remaking of the BookWorld, some books remained tantalisingly out of reach [...] It was entirely possible that they didn't know there was a BookWorld, and still they thought they were real. A fantastic notion, until you consider that up until ", "11.06am", " on 12 April 1948, everyone else had thought the same.", "Jasper Fforde", "One of Our Thursdays is Missing"],
]);
    minutes.insert("11:07", &[
&["At exactly ", "seven minutes past eleven", " by the ship's clock the Adventurer gave a prolonged screech and, moorings cast off, edged her way out of the basin and dipped her nose in the laughing waters of the bay, embarked at last on a voyage that was destined to fully vindicate her new name.", "Ralph Henry Barbour", "The Adventure Club Afloat"],
]);
    minutes.insert("11:08", &[
&["The bursar was standing in the hall with his arms folded across his chest and when he caught sight of the fat young man he looked significantly at the clock. It was ", "eight minutes past eleven", ".", "James Joyce", "Stephen Hero"],
]);
    minutes.insert("11:09", &[
&["The first time I saw them it was ", "around eleven", ", eleven-fifteen, a Saturday morning, I was about two thirds through my route when I turned onto their block and noticed a '56 Ford sedan pulled up in the yard with a big open U-Haul behind.", "Raymond Carver", "Where I'm Calling From"],
]);
    minutes.insert("11:10", &[
&["", "Ten minutes after eleven", " in Archie McCue's room on the third floor of the extension to the Robert Matthews' soaring sixties' tower - The Queen's Tower, although no queen was ever likely to live in it.", "Kate Atkinson", "Emotionally Weird"],
]);
    minutes.insert("11:11", &[
&["", "Ten minutes after eleven", " in Archie McCue's room on the third floor of the extension to the Robert Matthews' soaring sixties' tower - The Queen's Tower, although no queen was ever likely to live in it.", "Kate Atkinson", "Emotionally Weird"],
]);
    minutes.insert("11:12", &[
&["", "11:12", " I have a solution, via the local paper. A girl from the village will collect it, iron it all overnight at £3 a shirt, and sew on Eddie's button.", "Sophie Kinsella", "The Undomestic Goddess"],
&["I squinted down the street at the bank clock: ", "11:12", ", 87 degrees. \"It's only a block and a half and it's not that hot, Daddy. The walk will do you good.\" This conversation made me breathless, as if I were wearing a girdle with tight stays.", "Jane Smiley", "A Thousand Acres"],
]);
    minutes.insert("11:13", &[
&["", "11:12", " I have a solution, via the local paper. A girl from the village will collect it, iron it all overnight at £3 a shirt, and sew on Eddie's button.", "Sophie Kinsella", "The Undomestic Goddess"],
&["I squinted down the street at the bank clock: ", "11:12", ", 87 degrees. \"It's only a block and a half and it's not that hot, Daddy. The walk will do you good.\" This conversation made me breathless, as if I were wearing a girdle with tight stays.", "Jane Smiley", "A Thousand Acres"],
]);
    minutes.insert("11:14", &[
&["The report was dated Sunday, 25 September, 1966, at ", "11.14am.", " The text was laconic. Call from Hrk Vanger; stating that his brother's daughter (?) Harriett Ulrika Vanger, born 15 June 1960 (age 1960) has been missing from her home on Hedley Island since Saturday afternoon.", "Stieg Larsson", "The Girl With The Dragon Tattoo"],
]);
    minutes.insert("11:15", &[
&["\"Have you a couple of days to spare? Have just been wired for from the west of England in connection with Boscombe Valley tragedy. Shall be glad if you will come with me. Air and scenery perfect. Leave Paddington by the ", "11:15", ".\"", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
&["The first time I saw them it was around eleven, ", "eleven-fifteen", ", a Saturday morning, I was about two thirds through my route when I turned onto their block and noticed a '56 Ford sedan pulled up in the yard with a big open U-Haul behind. There are only three houses on Pine, and theirs was the last house,the others being the Murchisons, who'd been in Arcata a little less than a year, and the Grants, who'd been here about two years. Murchison worked at Simpson Redwood, and Gene Grant was a cook on the morning shift at Denny's. Those two, then a vacant lot, then the house on the end that used to belong to the Coles.", "Raymond Carver", "Where I'm Calling From"],
]);
    minutes.insert("11:16", &[
&["\"Have you a couple of days to spare? Have just been wired for from the west of England in connection with Boscombe Valley tragedy. Shall be glad if you will come with me. Air and scenery perfect. Leave Paddington by the ", "11:15", ".\"", "Sir Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
&["The first time I saw them it was around eleven, ", "eleven-fifteen", ", a Saturday morning, I was about two thirds through my route when I turned onto their block and noticed a '56 Ford sedan pulled up in the yard with a big open U-Haul behind. There are only three houses on Pine, and theirs was the last house,the others being the Murchisons, who'd been in Arcata a little less than a year, and the Grants, who'd been here about two years. Murchison worked at Simpson Redwood, and Gene Grant was a cook on the morning shift at Denny's. Those two, then a vacant lot, then the house on the end that used to belong to the Coles.", "Raymond Carver", "Where I'm Calling From"],
]);
    minutes.insert("11:17", &[
&["Mrs. Mooney glanced instinctively at the little gilt clock on the mantelpiece as soon as she had become aware through her revery that the bells of George's Church had stopped ringing. It was ", "seventeen minutes past eleven", ": she would have lots of time to have the matter out with Mr. Doran and then catch short twelve at Marlborough Street. She was sure she would win.", "James Joyce", "Dubliners"],
]);
    minutes.insert(
        "11:18",
        &[&[
            "It is ",
            "11.18",
            ". A row of bungalows in a round with a clump of larch tree in the middle.",
            "Jackie Kay",
            "Trumpet",
        ]],
    );
    minutes.insert("11:19", &[
&["A whistle cut sharply across his words. Peter got onto his knees to look out the window, and Miss Fuller glared at him. Polly looked down at her watch: ", "11:19", ". The train. But the stationmaster had said it was always late.", "Connie Willis", "Blackout"],
]);
    minutes.insert("11:20", &[
&["OFFICER'S NOTES Disruption alert logged ", "11h20", " from Stones' Pool Hall (Premises ID 33CBD-Long181). Officer and Aito /379 responded. On arrival found subject shouting threats and acting in aggressive manner. A scan of the subject's SIM ID register revealed that the subject has recent priors including previous public disruptions and a juvenile record.", "Lauren Beukes", "Moxyland"],
&["Sweeney pointed to the clock above the bar, held in the massive and indifferent jaws of a stuffed alligator head. The time was ", "11.20", ".", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("11:21", &[
&["OFFICER'S NOTES Disruption alert logged ", "11h20", " from Stones' Pool Hall (Premises ID 33CBD-Long181). Officer and Aito /379 responded. On arrival found subject shouting threats and acting in aggressive manner. A scan of the subject's SIM ID register revealed that the subject has recent priors including previous public disruptions and a juvenile record.", "Lauren Beukes", "Moxyland"],
&["Sweeney pointed to the clock above the bar, held in the massive and indifferent jaws of a stuffed alligator head. The time was ", "11.20", ".", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("11:22", &[
&["OFFICER'S NOTES Disruption alert logged ", "11h20", " from Stones' Pool Hall (Premises ID 33CBD-Long181). Officer and Aito /379 responded. On arrival found subject shouting threats and acting in aggressive manner. A scan of the subject's SIM ID register revealed that the subject has recent priors including previous public disruptions and a juvenile record.", "Lauren Beukes", "Moxyland"],
&["Sweeney pointed to the clock above the bar, held in the massive and indifferent jaws of a stuffed alligator head. The time was ", "11.20", ".", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("11:23", &[
&["OFFICER'S NOTES Disruption alert logged ", "11h20", " from Stones' Pool Hall (Premises ID 33CBD-Long181). Officer and Aito /379 responded. On arrival found subject shouting threats and acting in aggressive manner. A scan of the subject's SIM ID register revealed that the subject has recent priors including previous public disruptions and a juvenile record.", "Lauren Beukes", "Moxyland"],
&["Sweeney pointed to the clock above the bar, held in the massive and indifferent jaws of a stuffed alligator head. The time was ", "11.20", ".", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("11:24", &[
&["OFFICER'S NOTES Disruption alert logged ", "11h20", " from Stones' Pool Hall (Premises ID 33CBD-Long181). Officer and Aito /379 responded. On arrival found subject shouting threats and acting in aggressive manner. A scan of the subject's SIM ID register revealed that the subject has recent priors including previous public disruptions and a juvenile record.", "Lauren Beukes", "Moxyland"],
&["Sweeney pointed to the clock above the bar, held in the massive and indifferent jaws of a stuffed alligator head. The time was ", "11.20", ".", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("11:25", &[
&["At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at ", "twenty-five past eleven", " what time it was. Linda looked at her watch and replied that it was a quarter to twelve.", "Agatha Christie", "Evil under the Sun"],
&["When, at about ", "11.25am", ", Katharina Blum was finally taken from her apartment for questioning, it was decided not to handcuff her at all.", "Heinrich Böll", "The Lost Honour of Katharina Blum"],
]);
    minutes.insert("11:26", &[
&["At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at ", "twenty-five past eleven", " what time it was. Linda looked at her watch and replied that it was a quarter to twelve.", "Agatha Christie", "Evil under the Sun"],
&["When, at about ", "11.25am", ", Katharina Blum was finally taken from her apartment for questioning, it was decided not to handcuff her at all.", "Heinrich Böll", "The Lost Honour of Katharina Blum"],
]);
    minutes.insert(
        "11:27",
        &[&[
            "It's from one of the more recent plates the tree has scanned: ",
            "11.27",
            " in the morning of 4 April 1175",
            "Chris James",
            "The Second Internet Cafe, Part 2: The Cascade Annihilator",
        ]],
    );
    minutes.insert("11:28", &[
&["From twenty minutes past nine until twenty-seven minutes past nine, from twenty-five minutes past eleven until ", "twenty-eight minutes past eleven", ", from ten minutes to three until two minutes to three the heroes of the school met in a large familiarity whose Olympian laughter awed the fearful small boy that flitted uneasily past and chilled the slouching senior that rashly paused to examine the notices in assertion of an unearned right.", "Compton Mackenzie", "Sinister Street"],
]);
    minutes.insert("11:29", &[
&["You are four minutes too slow. No matter; it's enough to mention the error. Now from this moment, ", "twenty-nine minutes after eleven, a.m.", ", this Wednesday, 2nd October, you are in my service.", "Jules Verne", "Around the World in Eighty Days"],
]);
    minutes.insert("11:30", &[
&["'It is now ", "11.30", ". The door to this room is shut, and will remain shut, barring emergencies, until 12.00. I am authorised to inform you that we are now under battle orders.", "Charles Stross", "Singularity Sky"],
&["\"O, Frank - I made a mistake! - I thought that church with the spire was All Saints', and I was at the door at ", "half-past eleven", " to a minute as you said...\"", "Thomas Hardy", "Far from the madding crowd"],
&["\"Thank-you,\" said C.B. quietly; but as he hung up his face was grim. In a few minutes he would have to break it to John that, although they had braved such dredful perils dring the earlier part of the night they had, after all, failed to save Christina. Beddows had abjured Satan at a little after ", "half-past eleven", ". By about eighteen minutes the Canon had beaten them to it again.\"", "Dennis Wheatley", "To the Devil a Daughter"],
&["This time it was Kumiko. The wall clock said ", "11.30", ".", "Haruki Murakami", "The Wind-up Bird Chronicle"],
]);
    minutes.insert("11:31", &[
&["Albatross 8 passed over Pamlico Sound at ", "1131", " local time. Its on-board programming was designed to trace thermal receptors over the entire visible horizon, interrogating everything in sight and locking on any signature that fit its acquisition parameters.", "Tom Clancy", "The Hunt for Red October"],
]);
    minutes.insert("11:32", &[
&["And after that, not forgetting, there was the Flemish armada, all scattered, and all officially drowned, there and then, on a lovely morning, after the universal flood, at about ", "eleven thirty two", " was it? Off the coast of Cominghome...", "James Joyce", "Finnegans Wake"],
]);
    minutes.insert("11:33", &[
&["And after that, not forgetting, there was the Flemish armada, all scattered, and all officially drowned, there and then, on a lovely morning, after the universal flood, at about ", "eleven thirty two", " was it? Off the coast of Cominghome...", "James Joyce", "Finnegans Wake"],
]);
    minutes.insert("11:34", &[
&["Christmas Eve 1995. ", "11.34am", ". The first time, Almasa says it slowly and softly, as if she is really looking for an answer, \"Are you talking to me?\" She peers into the small, grimy mirror in a train toilet.", "Adnan Mahmutovic", "How to Fare Well and Stay Fair"],
]);
    minutes.insert("11:35", &[
&["At ", "11.35", " the Colonel came out; he looked hot and angry as he strode towards the lift. There goes a hanging judge, thought Wormold.", "Graham Greene", "Our Man in Havana"],
]);
    minutes.insert("11:36", &[
&["I ran up the stairs, away from the heat and the noise, the mess and the confusion. I saw the clock radio by my bed. ", "Eleven thirty-six", ".", "Nicci French", "Losing You"],
]);
    minutes.insert("11:37", &[
&["I ran up the stairs, away from the heat and the noise, the mess and the confusion. I saw the clock radio by my bed. ", "Eleven thirty-six", ".", "Nicci French", "Losing You"],
]);
    minutes.insert("11:38", &[
&["At ", "11:38", ", she left her desk and walked to the side door of the auditorium, arriving ten minutes before noon.", "Dave Eggers", "The Circle"],
]);
    minutes.insert("11:39", &[
&["At ", "11:38", ", she left her desk and walked to the side door of the auditorium, arriving ten minutes before noon.", "Dave Eggers", "The Circle"],
]);
    minutes.insert("11:40", &[
&["Did escape occur to him? … But the door was locked, and the window heavily barred with iron rods. He sat down again, and drew his journal from his pocket. On the line where these words were written, \"21st December, Saturday, Liverpool,\" he added, \"80th day, ", "11.40am", ",\" and waited.", "Jules Verne", "Around the World in Eighty Days"],
&["During the sessions at Ito he read the Lotus Sutra on mornings of play, and he now seemed to be bringing himself to order through silent meditation. Then, quickly, there came a rap of stone on board. It was ", "twenty minutes before noon", ".", "Yusunari Kawabata", "The Master of Go"],
]);
    minutes.insert("11:41", &[
&["Spagnola took a deep breath and started into the log again. \"", "Eleven forty-one", ": large dog craps in Dr. Yamata's Aston Martin. Twelve oh-three: dog eats two, count 'em, two of Mrs. Wittingham's Siamese cats. She just lost her husband last week; this sort of put her over the edge. We had to call Dr. Yamata in off the putting green to give her a sedative. The personal-injury lawyer in the unit next to hers was home for lunch and he came over to help. He was talking class action then, and we didn't even know who owned the dog yet.\"", "Christopher Moore", "Coyote Blue"],
]);
    minutes.insert("11:42", &[
&["", "11:42", " I'm doing fine. I'm doing well. I've got the Hoover on, I'm cruising along nicely- What was that? What just went up the Hoover? Why is it making that grinding noise? Have I broken it?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("11:43", &[
&["", "11:42", " I'm doing fine. I'm doing well. I've got the Hoover on, I'm cruising along nicely- What was that? What just went up the Hoover? Why is it making that grinding noise? Have I broken it?", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("11:44", &[
&["At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at twenty-five past eleven what time it was. Linda looked at her watch and replied that it was a ", "quarter to twelve", ".", "Agatha Christie", "Evil under the Sun"],
]);
    minutes.insert("11:45", &[
&["\"...I waited till a ", "quarter to twelve", ", and found then that I was in All Souls'. But I wasn't much frightened, for I thought it could be tomorrow as well.\"", "Thomas Hardy", "Far from the madding crowd"],
&["\"I will tell you the time,\" said Septimus, very slowly, very drowsily, smiling mysteriously. As he sat smiling at the dead man in the grey suit the quarter struck, the ", "quarter to twelve", ".", "Virginia Woolf", "Mrs. Dalloway"],
&["As he sat smiling, the quarter struck - the ", "quarter to twelve", ".", "Virginia Woolf", "Mrs Dalloway"],
&["I arrived at St. Gatien from Nice on Tuesday, the 14th of August. I was arrested at ", "11.45am", " on Thursday, the 16th by an agent de police and an inspector in plain clothes and taken to the Commissariat.", "Eric Ambler", "Epitaph for a Spy"],
&["She tucked the phone in the crook of her neck and thumbed hurriedly through her pink messages. .... Dr. Provetto, at ", "11:45 A.M.", "", "Lisa Scottoline", "Mistaken Identity"],
]);
    minutes.insert("11:46", &[
&["At 10.15 Arlena departed from her rondezvous, a minute or two later Patrick Redfern came down and registered surprise, annoyance, etc. Christine's task was easy enough. Keeping her own watch concealed she asked Linda at twenty-five past eleven what time it was. Linda looked at her watch and replied that it was a ", "quarter to twelve", ".", "Agatha Christie", "Evil under the Sun"],
]);
    minutes.insert("11:47", &[
&["It was a vast plain with no one on it, neither living on the earth nor dead beneath it; and I walked a long time beneath a colourless sky, which didn't let me judge the time (my watch, set like all military watches to Berlin time, hadn't stood up to the swim and showed an eternal ", "thirteen minutes to noon", ").", "Jonathan Littell", "The Kindly Ones"],
]);
    minutes.insert(
        "11:48",
        &[&[
            "At 11:38, she left her desk and walked to the side door of the auditorium, arriving ",
            "ten minutes before noon",
            ".",
            "Dave Eggers",
            "The Circle",
        ]],
    );
    minutes.insert(
        "11:49",
        &[&[
            "At 11:38, she left her desk and walked to the side door of the auditorium, arriving ",
            "ten minutes before noon",
            ".",
            "Dave Eggers",
            "The Circle",
        ]],
    );
    minutes.insert("11:50", &[
&["The man who gave them to him handed him a ten-shilling note and promised him another if it were delivered at exactly ", "ten minutes to twelve", ".", "Agatha Christie", "The Adventure of Johnnie Waverley: A Hercule Poirot Story"],
]);
    minutes.insert("11:51", &[
&["The next day, at ", "nine minutes to twelve", " o'clock noon, the last clock ran down and stopped. It was then placed in the town museum, as a collector's item, or museum piece, with proper ceremonies, addresses, and the like.", "James Thurber", "Lanterns & Lances"],
]);
    minutes.insert("11:52", &[
&["At any rate, we whirled into the station with many more, just as the great clock pointed to ", "eight minutes to twelve", " o'clock. \"Thank God! We are in time,\" said the young man, \"and thank you, too, my friend, and your good horse...\"", "Anna Sewell", "Black Beauty"],
]);
    minutes.insert("11:53", &[
&["At any rate, we whirled into the station with many more, just as the great clock pointed to ", "eight minutes to twelve", " o'clock. \"Thank God! We are in time,\" said the young man, \"and thank you, too, my friend, and your good horse...\"", "Anna Sewell", "Black Beauty"],
]);
    minutes.insert(
        "11:54",
        &[&[
            "He swilled off the remains of [his beer] and looked at the clock. It was ",
            "six minutes to twelve",
            ".",
            "Patrick Hamilton",
            "Hangover Square",
        ]],
    );
    minutes.insert("11:55", &[
&["He was tearing off on his bicycle to one of the jobs about ", "five minutes to twelve", " to see if he could catch anyone leaving off for dinner before the proper time.", "Robert Tressell", "The Ragged Trousered Philanthropists"],
&["It was ", "11:55 a.m.", " on April 30.", "Bernstein & Woodward", "All the President's Men"],
&["What time did you arrive at the site? It was ", "11:55", ". I remember since I happened to glance at my watch when we got there. We rode our bicycles to the bottom of the hill, as far as we could go, then climbed the rest of the way on foot.", "Haruki Murakami", "Kafka on the Shore"],
]);
    minutes.insert("11:56", &[
&["A few minutes' light ", "around noon", " is all that you need to discover the error, and re-set the clock – provide that you bother to go up and make the observation.", "Neal Stephenson", "Odalisque: The Baroque Cycle #3"],
&["I wondered what the time is?' said the latter after a pause'. 'I don't know exactly', replied Easton, 'but it ", "can't be far-off twelve", ".'", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("11:57", &[
&["I wondered what the time is?' said the latter after a pause'. 'I don't know exactly', replied Easton, 'but it ", "can't be far-off twelve", ".'", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("11:58", &[
&["And when you go down the steps, it's always ", "11.58", " on the morning of September ninth, 1958.", "Stephen King", "11/22/63"],
&["", "Two minutes before the clock struck noon", ", the savage baron was on the platform to inspect the preparation for the frightful ceremony of mid-day. The block was laid forth-the hideous minister of vengeance, masked and in black, with the flaming glaive in his hand, was ready. The baron tried the edge of the blade with his finger, and asked the dreadful swordsman if his hand was sure? A nod was the reply of the man of blood. The weeping garrison and domestics shuddered and shrank from him. There was not one there but loved and pitied the gentle lady.", "William Makepeace Thackeray", "Burlesques"],
]);
    minutes.insert(
        "11:59",
        &[&[
            "There is a big grandfather clock there, and as the hands drew ",
            "near to twelve",
            " I don't mind confessing I was as nervous as a cat.",
            "Agatha Christie",
            "The Adventure of Johnnie Waverley: A Hercule Poirot Story",
        ]],
    );
    minutes.insert("12:00", &[
&["'It will soon be an hour and a half,' said the girl who kept the records. The ", "noon", "day siren blew. 'Exactly a minute,' she said, looking at the stopwatch of which she was so proud.", "Yasunari Kawabata", "The Master of Go"],
&["'There's nobody here!' I insisted. 'It was yourself, Mrs. Linton: you knew it a while since.' 'Myself!' she gasped, 'and the clock is striking ", "twelve", "!", "Emily Brontë", "Wuthering Heights"],
&["A cheap little clock on the wall struck ", "twelve", " hurriedly, and served to begin the conversation.", "Fyodor Dostoyevsky", "The Brothers Karamazov"],
&["He had saved [the republic] and it was now in the present, alive now and everywhere in the present, and the hovering faces brightened and blurred about him, became the sound of a canal in the morning, the look of some roofs in the ", "noon", " sun, and the fragrance of a certain evening flower. Here he was, home at last. Behind him were the mountains and the Sleeping Woman in the sky, and before him, like smoky flames in the sunset, the whole beautiful beloved city.", "Nick Joaquin", "The Woman Who Had Two Navels"],
&["It was precisely ", "twelve o'clock", "; twelve by Big Ben; whose stroke was wafted over the northern part of London; blent with that of other clocks, mixed in a thin ethereal way with the clouds and wisps of smoke and died up there among the seagulls, twelve o'clock struck as Clarissa Dalloway laid her green dress on her bed and the Warren Smiths walked down Harley Street. Twelve was the hour of their appointment.", "Virginia Woolf", "Mrs Dalloway"],
&["It was precisely ", "twelve o'clock", "; twelve by Big Ben; whose stroke was wafted over the northern part of London; blent with that of other clocks, mixed in a thin ethereal way wth the clouds and wisps of smoke and died up there among the seagulls - twelve o'clock struck as Clarissa Dalloway laid her green dress on the bed, and the Warren Smiths walked down Harley Street.", "Virginia Woolf", "Mrs Dalloway"],
&["It was precisely ", "twelve o’clock", "; twelve by Big Ben; whose stroke was wafted over the northern part of London; blent with that of other clocks, mixed in a thin ethereal way with the clouds and wisps of smoke and died up there among the seagulls—twelve o’clock struck as Clarissa Dalloway laid her green dress on her bed, and the Warren Smiths walked down Harley Street", "Virginia Woolf", "Mrs Dalloway"],
&["It was precisely ", "twelve o’clock", "; twelve by Big Ben; whose stroke was wafted over the northern part of London; blent with that of other clocks, mixed in a thin ethereal way with the clouds and wisps of smoke, and died up there among the seagulls.", "Virginia Woolf", "Mrs Dalloway"],
&["", "Noon", " found him momentarily alone, while the family prepared lunch in the kitchen. The cracks in the ceiling widened into gaps. The locked wheels of his bed sank into new fault lines opening in the oak floor beneath the rug. At any moment the floor was going to give.", "Paul Harding", "tinkers"],
&["On Friday ", "noon", ", July the twentieth, 1714, the finest bridge in all Peru broke and precipitated five travellers into the gulf below.", "Thornton Wilder", "The Bridge of San Luis Rey"],
&["Roaring ", "noon", ". In a well-fanned Forty-second Street cellar I met Gatsby for lunch.", "F. Scott Fitzgerald", "The Great Gatsby"],
&["The Birds begun at Four o'clock — Their period for Dawn — A Music numerous as space — But neighboring as ", "Noon", " —", "Emily Dickinson", "The Birds begun at Four o'clock"],
&["The Oxen Christmas Eve, and ", "twelve", " of the clock. \"Now they are all on their knees,\" An elder said as we sat in a flock By the embers in hearthside ease. We pictured the meek mild creatures where They dwelt in their strawy pen, Nor did it occur to one of us there To doubt they were kneeling then. So fair a fancy few would weave In these years! Yet, I feel, If someone said on Christmas Eve, \"Come; see the oxen kneel \"In the lonely barton by yonder coomb Our childhood used to know,\" I should go with him in the gloom, Hoping it might be so.", "Thomas Hardy", "The Oxen"],
&["Then came the stroke of ", "noon", ", and all these working and professional people dispersed like a trampled anthill into all the streets and directions. The white bridge was swarming with nimble dots. And when you considered that each dot had a mouth with which it was now planning to eat lunch, you couldn't help bursting into laughter.", "Robert Walser", "The Tanners"],
]);
    minutes.insert("12:01", &[
&["And on all sides there were the clocks. Conrad noticed them immediately, at every street corner, over every archway, three quarters of the way up the sides of buildings, covering every conceivable angle of approach. Most of them were too high off the ground to be reached by anything less than a fireman's ladder and still retained their hands. All registered the same time: ", "12:01", ". Conrad looked at his wristwatch, noted that it was just 2:45. ‘‘They were driven by a master dock’’ Stacey told him. ‘‘When that stopped, they all ceased at the same moment. One minute after midnight, thirty-seven years ago.’’", "J.G. Ballard", "Chronopolis"],
&["It was the twelfth of December, the twelfth month. A was twelve. The electric clock/radio by his bedside table said ", "12:01", ".", "Jonathan Trigell", "Boy A"],
&["It was the twelfth of December, the twelfth month. A was twelve. The electric clock/radio by his bedside table said ", "12:01", ". A was waiting for it to read 12:12, he hoped there would be some sense of cosmic rightness when it did.", "Jonathan Trigell", "Boy A"],
]);
    minutes.insert("12:02", &[
&["It had struck ", "twelve o'clock two minutes and a quarter", ". The Baron's footman hastily seized a large goblet, and gasped with terror as he filled it with hot, spiced wine. \"Tis past the hour, 'tis past,\" he groaned in anguish, \"and surely I shall now get the red hot poker the Baron hath so often promised me, oh! Woe is me! Would that I had prepared the Baron's lunch before!\"", "Lewis Carroll", "Crundle Castle"],
]);
    minutes.insert("12:03", &[
&["At ", "12.03", " the sun has already punched its ticket. Sinking, it stains the cobbles and stucco of the platz in a violin-coloured throb of light that you would have to be a stone not to find poignant.", "Michael Chabon", "The Yiddish Policemen's Union"],
]);
    minutes.insert("12:04", &[
&["Though by then it was by Tina's own desk clock ", "12.04pm", " I was always touched when, out of a morning's worth of repetition, secretaries continued to answer with good mornings for an hour or so into the afternoon, just as people often date things with the previous year well into February; sometimes they caught their mistake and went into a \"This is not my day\" or \"Where is my head?\" escape routine; but in a way they were right, since the true tone of afternoons does not take over in offices until nearly two.", "Nicholson Baker", "The Mezzanine"],
]);
    minutes.insert("12:05", &[
&["A few minutes' light ", "around noon", " is all that you need to discover the error, and re-set the clock – provide that you bother to go up and make the observation.", "Neal Stephenson", "Odalisque: The Baroque Cycle #3"],
]);
    minutes.insert("12:06", &[
&["A few minutes' light ", "around noon", " is all that you need to discover the error, and re-set the clock – provide that you bother to go up and make the observation.", "Neal Stephenson", "Odalisque: The Baroque Cycle #3"],
]);
    minutes.insert("12:07", &[
&["On a Monday Simon Hirsch was going to break his leg at ", "seven minutes after 12", ", noon, and as soon as Satan told us the day before, Seppi went to betting with me that it would not happen, and soon they got excited and went to betting with me themselves.", "Mark Twain", "The Chronicle of Young Satan"],
]);
    minutes.insert(
        "12:08",
        &[&[
            "When a clock struck noon in Washington, D.C., the time was ",
            "12:08",
            " in Philadephia, 12:12 in new York, and 12:24 in Boston.",
            "Matthew Goodman",
            "Eighty Days",
        ]],
    );
    minutes.insert(
        "12:09",
        &[&[
            "When a clock struck noon in Washington, D.C., the time was ",
            "12:08",
            " in Philadephia, 12:12 in new York, and 12:24 in Boston.",
            "Matthew Goodman",
            "Eighty Days",
        ]],
    );
    minutes.insert("12:10", &[
&["Madame Dumas arrived at ", "noon, and ten minutes later", " Trause handed her his ATM card and instructed her to go to the neighborhood Citibank near Sheridan Square and transfer forty thousand dollars from his savings account to his checking account.", "Paul Auster", "Oracle Night"],
&["They paid for only one room and kept Einstein with them because they were not going to need privacy for lovemaking. Exhausted, Travis barely managed to kiss Nora before falling into a deep sleep. He dreamed of things with yellow eyes, misshapen heads, and crocodile mouths full of sharks’ teeth. He woke five hours later, at ", "twelve-ten", " Thursday afternoon.", "Dean Koontz", "Watchers"],
]);
    minutes.insert("12:11", &[
&["At ", "12:11", " there was a knock on the door. It was Terry, A could tell. He hadn't known Terry long, but there was something calmer, more patient, that separated Terry's knocks from the rest of the staff. He knocked from genuine politeness, not formality. \"Come in,\" A said, although the lock was on the other side. Terry did. \"It's your mother,\" he said. \"There's no easy way to say this.\" Though he had just used the easiest, because A now knew the rest. A’s face froze, as it tried to catch up, as it tried to register the news. Then it crumpled, and while he considered this fresh blow, the tears came.", "Jonathan Trigell", "Boy A"],
]);
    minutes.insert("12:12", &[
&["It was the twelfth of December, the twelfth month. A was twelve. The electric clock/radio by his bedside table said 12:01. A was waiting for it to read ", "12:12", ", he hoped there would be some sense of cosmic rightness when it did.", "Jonathan Trigell", "Boy A"],
]);
    minutes.insert("12:13", &[
&["It was the twelfth of December, the twelfth month. A was twelve. The electric clock/radio by his bedside table said 12:01. A was waiting for it to read ", "12:12", ", he hoped there would be some sense of cosmic rightness when it did.", "Jonathan Trigell", "Boy A"],
]);
    minutes.insert(
        "12:14",
        &[&[
            "She left London on the ",
            "twelve-fourteen",
            " from Paddington, arriving at Bristol (where she had to change) at two-fifty.",
            "Agatha Christie",
            "The Plymouth Express",
        ]],
    );
    minutes.insert("12:15", &[
&["Very well, dear,' she said. 'I caught the 10.20 to Eastnor, which isn't a bad train, if you ever want to go down there. I arrived at a ", "quarter past twelve", ", and went straight up to the house--you've never seen the house, of course? It's quite charming--and told the butler that I wanted to see Mr Ford on business. I had taken the precaution to find out that he was not there. He is at Droitwich.'", "P.G. Wodehouse", "The Little Nugget"],
&["What shall I think of that's liberating and refreshing? I'm in the mood when I open my window at night and look at the stars. Unfortunately it's ", "12.15", " on a grey dull day, the aeroplanes are active", "Virginia Woolf", "A Writer's Diary: Being Extracts from the Diary of Virgina Woolf"],
]);
    minutes.insert("12:16", &[
&["Very well, dear,' she said. 'I caught the 10.20 to Eastnor, which isn't a bad train, if you ever want to go down there. I arrived at a ", "quarter past twelve", ", and went straight up to the house--you've never seen the house, of course? It's quite charming--and told the butler that I wanted to see Mr Ford on business. I had taken the precaution to find out that he was not there. He is at Droitwich.'", "P.G. Wodehouse", "The Little Nugget"],
&["What shall I think of that's liberating and refreshing? I'm in the mood when I open my window at night and look at the stars. Unfortunately it's ", "12.15", " on a grey dull day, the aeroplanes are active", "Virginia Woolf", "A Writer's Diary: Being Extracts from the Diary of Virgina Woolf"],
]);
    minutes.insert("12:17", &[
&["Kava ordered two glasses of coffee for himself and his beloved and some cake. When the pair left, exactly ", "seventeen minutes after twelve", ", the club began to buzz with excitement.", "Isaac Bashevis Singer", "Vanvild Kava"],
]);
    minutes.insert("12:18", &[
&["Kava ordered two glasses of coffee for himself and his beloved and some cake. When the pair left, exactly ", "seventeen minutes after twelve", ", the club began to buzz with excitement.", "Isaac Bashevis Singer", "Vanvild Kava"],
]);
    minutes.insert("12:19", &[
&["Kava ordered two glasses of coffee for himself and his beloved and some cake. When the pair left, exactly ", "seventeen minutes after twelve", ", the club began to buzz with excitement.", "Isaac Bashevis Singer", "Vanvild Kava"],
]);
    minutes.insert("12:20", &[
&["By ", "twelve-twenty", " in the afternoon, Vince was seated in a rattan chair with comfortable yellow and green cushions at a table by the windows in that same restaurant. He’d spotted Haines on entering. The doctor was at another window table, three away from Vince, half-screened by a potted palm. Haines was eating shrimp and drinking margaritas with a stunning blonde. She was wearing white slacks and a gaily striped tube-top, and half the men in the place were staring at her.", "Dean Koontz", "Watchers"],
&["It is ", "12:20", " in New York a Friday three days after Bastille day, yes it is 1959 and I go get a shoeshine because I will get off the 4:19 in Easthampton at 7:15 and then go straight to dinner and I don’t know the people who will feed me", "Frank O'Hara", "The Day Lady Died"],
]);
    minutes.insert(
        "12:21",
        &[&[
            "Jake think of something. PLEASE! ",
            "Twelve twenty-one",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert("12:22", &[
&["By ", "twenty-two minutes past twelve", " we leave, much too soon for our desires, this delightful spot, where the pilgrims are in the habit of bathing who come to visit the Jordan.", "Félicien de Saulcy", "Narrative of a Journey round the Dead Sea and in the Bible lands in 1850 and 1851"],
]);
    minutes.insert("12:23", &[
&["By ", "twenty-two minutes past twelve", " we leave, much too soon for our desires, this delightful spot, where the pilgrims are in the habit of bathing who come to visit the Jordan.", "Félicien de Saulcy", "Narrative of a Journey round the Dead Sea and in the Bible lands in 1850 and 1851"],
]);
    minutes.insert("12:24", &[
&["", "12:24", " My legs are in total agony. I've been kneeling on hard tiles, cleaning the bath, for what seems like hours. There are little ridges where the tiles have dug into my knees, and I'm boiling hot and the cleaning chemicals are making me cough. All I want is a rest. But I can't stop for a moment. I am so behind ...", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert(
        "12:25",
        &[&[
            "Boys, do it now. God's time is ",
            "12.25",
            ".",
            "James Joyce",
            "Ulysses",
        ]],
    );
    minutes.insert(
        "12:26",
        &[&[
            "12.25pm. ",
            "26.",
            " 27. Every time Billy saved a shot he looked heartbroken",
            "Barry Hines",
            "A Kestrel For a Knave",
        ]],
    );
    minutes.insert(
        "12:27",
        &[&[
            "12.25pm. 26. ",
            "27.",
            " Every time Billy saved a shot he looked heartbroken",
            "Barry Hines",
            "A Kestrel For a Knave",
        ]],
    );
    minutes.insert(
        "12:28",
        &[&[
            "The DRINK CHEER-UP COFFEE wall clock read ",
            "12.28",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert(
        "12:29",
        &[&[
            "The DRINK CHEER-UP COFFEE wall clock read ",
            "12.28",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert("12:30", &[
&["\"You'll never believe this but (in Spain) they are two hours late for ever meal - two hours Fanny - (can we lunch at ", "half-past twelve", " today?)\"", "Nancy Mitford", "Love in a Cold Climate"],
&["", "12.30 p.m.", " Lunch", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "half past twelve", ", when Catherine’s anxious attention to the weather was over and she could no longer claim any merit from its amendment, the sky began voluntarily to clear. A gleam of sunshine took her quite by surprise; she looked round; the clouds were parting, and she instantly returned to the window to watch over and encourage the happy appearance. Ten minutes more made it certain that a bright afternoon would succeed, and justified the opinion of Mrs. Allen, who had “always thought it would clear up.”", "Jane Austen", "Northanger Abbey"],
&["Tuesday, ", "12.30pm", "… Baker, California… Into the Ballantine Ale now, zombie drunk and nervous. I recognize this feeling: three or four days of booze, drugs, sun, no sleep and burned out adrenalin reserves – a giddy, quavering sort of high that means the crash is coming. But when? How much longer?", "Hunter S. Thompson", "Fear and Loathing in Las Vegas"],
]);
    minutes.insert("12:31", &[
&["\"You'll never believe this but (in Spain) they are two hours late for ever meal - two hours Fanny - (can we lunch at ", "half-past twelve", " today?)\"", "Nancy Mitford", "Love in a Cold Climate"],
&["", "12.30 p.m.", " Lunch", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "half past twelve", ", when Catherine’s anxious attention to the weather was over and she could no longer claim any merit from its amendment, the sky began voluntarily to clear. A gleam of sunshine took her quite by surprise; she looked round; the clouds were parting, and she instantly returned to the window to watch over and encourage the happy appearance. Ten minutes more made it certain that a bright afternoon would succeed, and justified the opinion of Mrs. Allen, who had “always thought it would clear up.”", "Jane Austen", "Northanger Abbey"],
&["Tuesday, ", "12.30pm", "… Baker, California… Into the Ballantine Ale now, zombie drunk and nervous. I recognize this feeling: three or four days of booze, drugs, sun, no sleep and burned out adrenalin reserves – a giddy, quavering sort of high that means the crash is coming. But when? How much longer?", "Hunter S. Thompson", "Fear and Loathing in Las Vegas"],
]);
    minutes.insert("12:32", &[
&["12:30 What is wrong with this bleach bottle? Which way is the nozzle pointing, anyway? I'm turning it round in confusion, peering at the arrows on the plastic ... Why won't anything come out? OK, I'm going to squeeze it really, really hard- That nearly got my eye. ", "12:32", " FUCK. What has it done to my HAIR?", "Sophie Kinsella", "The Undomestic Goddess"],
&["A chutney-biting brigadier named Boyd-Boyd fixed an appointment on the 'phone with Oxted, at Hornborough Station, for the ", "twelve thirty-two", ". He was to deliver the goods.", "Stacy Aumonier", "Extremely Entertaining Short Stories"],
]);
    minutes.insert(
        "12:33",
        &[&[
            "It's ",
            "12.33",
            " now and I could do it, the station is just down that side road there.",
            "Dorothy L. Sayers",
            "Five Red Herrings",
        ]],
    );
    minutes.insert(
        "12:34",
        &[&[
            "It's ",
            "12.33",
            " now and I could do it, the station is just down that side road there.",
            "Dorothy L. Sayers",
            "Five Red Herrings",
        ]],
    );
    minutes.insert("12:35", &[
&["As surely as Apthorpe was marked for early promotion, Trimmer was marked for ignominy. That morning he had appeared at the precise time stated in orders. Everyone else had been waiting five minutes and Colour Sergeant Cork called out the marker just as Trimmer appeared. So it was ", "twelve-thirty-five", " when they were dismissed.", "Evelyn Waugh", "Men At Arms"],
]);
    minutes.insert("12:36", &[
&["As surely as Apthorpe was marked for early promotion, Trimmer was marked for ignominy. That morning he had appeared at the precise time stated in orders. Everyone else had been waiting five minutes and Colour Sergeant Cork called out the marker just as Trimmer appeared. So it was ", "twelve-thirty-five", " when they were dismissed.", "Evelyn Waugh", "Men At Arms"],
]);
    minutes.insert("12:37", &[
&["As surely as Apthorpe was marked for early promotion, Trimmer was marked for ignominy. That morning he had appeared at the precise time stated in orders. Everyone else had been waiting five minutes and Colour Sergeant Cork called out the marker just as Trimmer appeared. So it was ", "twelve-thirty-five", " when they were dismissed.", "Evelyn Waugh", "Men At Arms"],
]);
    minutes.insert("12:38", &[
&["As surely as Apthorpe was marked for early promotion, Trimmer was marked for ignominy. That morning he had appeared at the precise time stated in orders. Everyone else had been waiting five minutes and Colour Sergeant Cork called out the marker just as Trimmer appeared. So it was ", "twelve-thirty-five", " when they were dismissed.", "Evelyn Waugh", "Men At Arms"],
]);
    minutes.insert("12:39", &[
&["Next, he remembered that the morrow of Christmas would be the twenty-seventh day of the moon, and that consequently high water would be at twenty-one minutes past three, the half-ebb at a quarter past seven, low water at thirty-three minutes past nine, and half flood at ", "thirty-nine minutes past twelve", ".", "Victor Hugo", "The Toilers of the Sea"],
]);
    minutes.insert("12:40", &[
&["A little ormolu clock in the outer corridor indicated ", "twenty minutes to one", ". The car was due at one-fifteen. Thirty-five minutes: oh, to escape for only that brief period!", "Stacy Aumonier", "Extremely Entertaining Short Stories (The Octave of Jealousy)"],
]);
    minutes.insert("12:41", &[
&["A little ormolu clock in the outer corridor indicated ", "twenty minutes to one", ". The car was due at one-fifteen. Thirty-five minutes: oh, to escape for only that brief period!", "Stacy Aumonier", "Extremely Entertaining Short Stories (The Octave of Jealousy)"],
]);
    minutes.insert("12:42", &[
&["The butt had been growing warm in her fingers; now the glowing end stung her skin. She crushed the cigarette out and stood, brushing ash from her black skirt. It was ", "eighteen minutes to one", ". She went to the house phone and called his room. The telephone rang and rang, but there was no answer.", "Herman Wouk", "Marjorie Morningstar"],
]);
    minutes.insert(
        "12:43",
        &[&[
            "Died five minutes ago, you say? he asked. His eye went to the watch on his wrist. ",
            "Twelve-forty-three",
            ", he wrote on the blotter.",
            "Agatha Christie",
            "A Pocket Full of Rye",
        ]],
    );
    minutes.insert("12:44", &[
&["It is ", "around quarter to one", ". No sunlight comes into the room now through the windows at right. Outside the day is fine but increasingly sultry, with a faint haziness in the air which softens the glare of the sun.", "Eugene O'Neil", "Long Day's Journey Into Night"],
]);
    minutes.insert("12:45", &[
&["The boy handed in a dispatch. The Professor closed the door again, and after looking at the direction, opened it and read aloud. \"Look out for D. He has just now, ", "12:45", ", come from Carfax hurriedly and hastened towards the South. He seems to be going the round and may want to see you: Mina\"", "Bram Stoker", "Dracula"],
]);
    minutes.insert("12:46", &[
&["It is ", "around quarter to one", ". No sunlight comes into the room now through the windows at right. Outside the day is fine but increasingly sultry, with a faint haziness in the air which softens the glare of the sun.", "Eugene O'Neil", "Long Day's Journey Into Night"],
]);
    minutes.insert("12:47", &[
&["It is ", "around quarter to one", ". No sunlight comes into the room now through the windows at right. Outside the day is fine but increasingly sultry, with a faint haziness in the air which softens the glare of the sun.", "Eugene O'Neil", "Long Day's Journey Into Night"],
]);
    minutes.insert("12:48", &[
&["It is ", "around quarter to one", ". No sunlight comes into the room now through the windows at right. Outside the day is fine but increasingly sultry, with a faint haziness in the air which softens the glare of the sun.", "Eugene O'Neil", "Long Day's Journey Into Night"],
]);
    minutes.insert(
        "12:49",
        &[&[
            "The first victim of the Krefeld raid died at ",
            "12:49 hours",
            " Double British Summer Time at B Flight, but it wasn't due to carelessness.",
            "Len Deighton",
            "Bomber",
        ]],
    );
    minutes.insert("12:50", &[
&["So presently Bert was sent up to the top of the house to look at a church clock which was visible therefrom, and when he came down he reported that it was ", "ten minutes to one", ".", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("12:51", &[
&["So presently Bert was sent up to the top of the house to look at a church clock which was visible therefrom, and when he came down he reported that it was ", "ten minutes to one", ".", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert(
        "12:52",
        &[&[
            "The nightclub stood on the junction, flamboyant, still. It was ",
            "12.52",
            ".",
            "Rupert Thomson",
            "Dreams of Leaving",
        ]],
    );
    minutes.insert(
        "12:53",
        &[&[
            "Aboot twelve miles. We ought tae pass her at Pinmore. She's due there at ",
            "12:53",
            ".",
            "Dorothy L. Sayers",
            "Five Red Herrings",
        ]],
    );
    minutes.insert("12:54", &[
&["I listen to the different boats' horns, hoping to learn what kind of boat I'm hearing and what the signal means: is the boat leaving or entering the harbor; is it the ferry, or a whale-watching boat, or a fishing boat? At 5:33 pm there is a blast of two deep, resonant notes a major third apart. On another day there is the same blast at ", "12:54 pm.", " On another, exactly 8:00 am.", "Lydia Davis", "Varieties of Disturbance"],
]);
    minutes.insert(
        "12:55",
        &[&[
            "The inspector glanced at the clock. ",
            "Five to one",
            ". A busy morning.",
            "Ngaio Marsh",
            "A Man Lay Dead",
        ]],
    );
    minutes.insert(
        "12:56",
        &[&[
            "The inspector glanced at the clock. ",
            "Five to one",
            ". A busy morning.",
            "Ngaio Marsh",
            "A Man Lay Dead",
        ]],
    );
    minutes.insert(
        "12:57",
        &[&[
            "The inspector glanced at the clock. ",
            "Five to one",
            ". A busy morning.",
            "Ngaio Marsh",
            "A Man Lay Dead",
        ]],
    );
    minutes.insert(
        "12:58",
        &[&[
            "The watch on my wrist showed ",
            "12.58pm",
            " I'd have time to hit the morgue.",
            "Ilona Andrews",
            "Magic Bites",
        ]],
    );
    minutes.insert(
        "12:59",
        &[&[
            "And I had been looking at my watch since the train had started at ",
            "12.59pm",
            "",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-Time",
        ]],
    );
    minutes.insert("13:00", &[
&["\"I think,\" he said, with a triumphant smile, \"that I may safely expect to find the person I seek in the dining-room, fair lady.\" \"There may be more than one.\" \"Whoever is there, as the ", "clock strikes one", ", will be shadowed by one of my men; of these, one, or perhaps two, or even three, will leave for France to-morrow. One of these will be the `Scarlet Pimpernel.'\"", "Baroness Orczy", "The Scarlet Pimpernel"],
&["\"", "One o'clock", " pee em! Hello, Insert Name Here!\" Said by the Disorganizer", "Terry Pratchett", "Jingo"],
&["“Czarina Catherine reported entering Galatz at ", "one o'clock", " today.”", "Bram Stoker", "Dracula"],
&["", "1.00 p.m.", " First afternoon class", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["After ", "1 o'clock", " checks, Gretta always goes out for a smoke.", "Susanna Kaysen", "Girl, Interrupted"],
&["Gottfried Rembke arrived at ", "1pm", " precisely. The moment he walked into the restaurant, handed his coat to the waiter, they knew it was him. The solid, stocky body, the gleaming pate, the open expression, the vigorous handshake: everything about him radiated ease and enthusiasm", "Michel Houellebecq", "Platform"],
&["I got to Schmidt's early, feeling horribly nervous. At ", "one o'clock", " sharp: Toni. She was looking at the menu she knew well - Schmorbraten? Schnitzel? - when he loomed over her. I had seen him come in. She looked up, through him, at me. 'Traitor.' Jamie, hovering, looking very big, said her pet name, a German diminutive chosen by her. Toni addressed the air. 'If he does not leave at once I shall tell the waiter that I am not sharing my table with this gentleman.' Jamie heard, said her name again, turned to go, I rose to go with him. Toni - with that concentration of will - said, 'YOU are lunching with me.'", "Sybille Bedford", "Jigsaw"],
&["It was a bright cold day in April, and the clocks were striking ", "thirteen", ".", "George Orwell", "Nineteen Eighty-Four"],
&["It was ", "one o'clock", ". I bought some apples and a small pork pie and drove across the bridge to the other side of the riverbank in the direction of Orford Ness.", "Roma Tearne", "The Swimmer"],
&["Many moons passed by. Did Baboon ever fly? Did he ever get to the sun? I’ve just heard today That he’s well on his way! He’ll be passing through Acton ", "at one", ".", "Spike Milligan", "Silly Old Baboon"],
&["That day it was ", "one o'clock", " before John and Roger rowed across and went up to Dixon's farm for the milk and a new supply of eggs and butter.", "Arthur Ransome", "Swallows and Amazons"],
&["The day-room floor gets cleared of tables and at ", "one o'clock", " the doctor comes out of his office down the hall, nods once at the nurse as he goes past where he's watching out of her window, sits in his chair just to the left of the door.", "Ken Kesey", "One Flew Over the Cuckoo's Nest"],
]);
    minutes.insert("13:01", &[
&["There's five fathoms out there, he said. It'll be swept up that way when the tide comes in ", "about one", ". It's nine days today.", "James Joyce", "Ulysses"],
]);
    minutes.insert(
        "13:02",
        &[&[
            "At ",
            "about one o'clock",
            " the overseer arrived and told them he had no jobs for them",
            "George Orwell",
            "A Clergyman's Daughter",
        ]],
    );
    minutes.insert("13:03", &[
&["It was ", "a little after one o'clock", " when I got there, time for lunch, so I had it. The food was awful. But it would go on the expense account, and after I'd eaten I got out my notebook and put it down. Lunch $1.50. Taxi $1.00.", "Kenneth Fearing", "The Big Clock"],
]);
    minutes.insert("13:04", &[
&["\"Jesus Christ!\" he gasped. \"It's ", "four minutes past one", "!\" Linden frantically seized hold of a pair of steps and began wandering about the room with them.", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("13:05", &[
&["“Samantha?” I can hear Trish approaching. “Um ... hold on!” I hurry to the door, trying to block her view. “It's already ", "five past one", ",” I can hear her saying a little sharply. “And I did ask, most clearly for ...” Her voice trails off into silence as she reaches the kitchen door, and her whole face sags in astonishment. I turn and follow her gaze as she surveys the endless plates of sandwiches. “My goodness!” At last Trish finds her voice. “This is ... this is very impressive!”", "Sophie Kinsella", "The Undomestic Goddess"],
&["At ", "five past one", " Alleyn opened the outer door, knocked his pipe out on the edge of the stone step,and remained staring out on to the drive.", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert(
        "13:06",
        &[&[
            "And then at precisely ",
            "13 hours and 6 minutes",
            " - confusion broke out in the rectangle.",
            "Yevgeny Zamyatin",
            "We",
        ]],
    );
    minutes.insert(
        "13:07",
        &[&[
            "And then at precisely ",
            "13 hours and 6 minutes",
            " - confusion broke out in the rectangle.",
            "Yevgeny Zamyatin",
            "We",
        ]],
    );
    minutes.insert(
        "13:08",
        &[&[
            "And then at precisely ",
            "13 hours and 6 minutes",
            " - confusion broke out in the rectangle.",
            "Yevgeny Zamyatin",
            "We",
        ]],
    );
    minutes.insert("13:09", &[
&["At ", "nine minutes past one", ", a pair of horses approached (not from the city, from which direction Krieger had expected her to come, but from the Desert, which lay, vast and largely uncharted, out to the West and South-West of the city.)", "Clive Barker", "Tortured Souls: The Legend of Primordium"],
]);
    minutes.insert(
        "13:10",
        &[&[
            "\"It was ",
            "ten minutes past one",
            ".” “You are sure of that?\"",
            "Agatha Christie",
            "Death on the Nile",
        ]],
    );
    minutes.insert("13:11", &[
&["I pursued my inquiries at the other stations along the line an' I found there was a gentleman wi' a bicycle tuk the ", "1.11", " train at Girvan.", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("13:12", &[
&["I pursued my inquiries at the other stations along the line an' I found there was a gentleman wi' a bicycle tuk the ", "1.11", " train at Girvan.", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("13:13", &[
&["\"There it is! There it is!\" shouted the Professor. \"Now for the centre of the globe!\" he added in Danish. I looked at Hans. \"Forüt!\" was his tranquil answer. \"Forward!\" replied my uncle. It was ", "thirteen minutes past one", ".", "Jules Verne", "Journey to the Centre of the Earth"],
]);
    minutes.insert("13:14", &[
&["\"There it is! There it is!\" shouted the Professor. \"Now for the centre of the globe!\" he added in Danish. I looked at Hans. \"Forüt!\" was his tranquil answer. \"Forward!\" replied my uncle. It was ", "thirteen minutes past one", ".", "Jules Verne", "Journey to the Centre of the Earth"],
]);
    minutes.insert("13:15", &[
&["‘Monsieur has well slept this morning,’ he said, smiling. ‘What o’clock is it, Victor?’ asked Dorian Gray, sleepily. ‘", "One hour and a quarter", ", monsieur.’", "Oscar Wilde", "The Picture of Dorian Gray"],
&["\"Where are the ladies and Gentlemen?\" asked Aleyn. \"Sir, in the garding\", said Bunce. \"What time's lunch?\" \"", "One-fifteen", "\".", "Ngaio Marsh", "A Man Lay Dead"],
&["The clock caught Miss LaFosse´s eye. ´Good heavens!´ she gasped. ´Look at the time. ", "Quarter-past one", ". You must be starved.' She turned impetuously to Miss Pettigrew.", "Winifred Watson", "Miss Pettigrew lives for a Day"],
]);
    minutes.insert(
        "13:16",
        &[&[
            "And the first stop had been at ",
            "1.16pm",
            " which was 17 minutes later.",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-Time",
        ]],
    );
    minutes.insert("13:17", &[
&["", "One seventeen", " and four seconds. That shorter guy’s really got it made, and gets on a scooter, and that taller one, he goes in. One seventeen and forty seconds. That girl there, she’s got a green ribbon in her hair. Too bad that bus just cut her from view.", "Wislawa Szymborska", "The Terrorist, He Watches"],
]);
    minutes.insert("13:18", &[
&["", "One eighteen", " exactly. Was she stupid enough to head inside? Or wasn't she? We'll know before long, When the dead are carried out.", "Wislawa Szymborska", "The Terrorist, He Watches"],
]);
    minutes.insert("13:19", &[
&["", "One eighteen", " exactly. Was she stupid enough to head inside? Or wasn't she? We'll know before long, When the dead are carried out.", "Wislawa Szymborska", "The Terrorist, He Watches"],
]);
    minutes.insert("13:20", &[
&["Kamarov, signal to Purga: 'Diving at—,'\" he checked his watch, \"'—", "1320 hours", ". Exercise OCTOBER FROST begins as scheduled. You are released to other assigned duties. We will return as scheduled.\" Kamarov worked the trigger on the blinker light to transmit the message. The Purga responded at once, and Ramius read the flashing signal unaided: \"IF THE WHALES DON'T EAT YOU. GOOD LUCK TO RED OCTOBER!\"", "Tom Clancy", "The Hunt for Red October"],
&["The time is coming for action. Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
&["Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("13:21", &[
&["Kamarov, signal to Purga: 'Diving at—,'\" he checked his watch, \"'—", "1320 hours", ". Exercise OCTOBER FROST begins as scheduled. You are released to other assigned duties. We will return as scheduled.\" Kamarov worked the trigger on the blinker light to transmit the message. The Purga responded at once, and Ramius read the flashing signal unaided: \"IF THE WHALES DON'T EAT YOU. GOOD LUCK TO RED OCTOBER!\"", "Tom Clancy", "The Hunt for Red October"],
&["The time is coming for action. Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
&["Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("13:22", &[
&["Kamarov, signal to Purga: 'Diving at—,'\" he checked his watch, \"'—", "1320 hours", ". Exercise OCTOBER FROST begins as scheduled. You are released to other assigned duties. We will return as scheduled.\" Kamarov worked the trigger on the blinker light to transmit the message. The Purga responded at once, and Ramius read the flashing signal unaided: \"IF THE WHALES DON'T EAT YOU. GOOD LUCK TO RED OCTOBER!\"", "Tom Clancy", "The Hunt for Red October"],
&["The time is coming for action. Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
&["Today this Vampire is limit to the powers of man, and till sunset he may not change. It will take him time to arrive here, see it is ", "twenty minutes past one", ", and there are yet some times before he can hither come, be he never so quick.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("13:23", &[
&["And when we got to Swindon Mother had keys to the house and we went in and she said, \"Hello?\" but there was no one there because it was ", "1.23pm", ".", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
&["The clock marked ", "twenty-three minutes past one", ". He was suddenly full of agitation, yet hopeful. She had come! Who could tell what she would say? She might offer the most natural explanation of her late arrival. Félicie entered the room, her hair in disorder, her eyes shining, her cheeks white, her bruised lips a vivid red; she was tired, indifferent, mute, happy and lovely, seeming to guard beneath her cloak, which she held wrapped about her with both hands, some remnant of warmth and voluptuous pleasure.", "Anatole France", "A Mummer's Tale"],
]);
    minutes.insert(
        "13:24",
        &[&[
            "Littell checked his watch - ",
            "1:24 p.m",
            " - Littell grabbed the phone by the bed.",
            "James Ellroy",
            "The Cold Six Thousand",
        ]],
    );
    minutes.insert("13:25", &[
&["I'd really have liked to, I told her, if it weren't for the things I had in the drier. I cast an eye at my watch. ", "One-twenty-five", ". The drier had already stopped.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("13:26", &[
&["Raymond came back with Masson ", "around one-thirty", ". His arm was bandaged up and he had an adhesive plaster on the corner of his mouth. The doctor had told him it was nothing, but Raymond looked pretty grim. Masson tried to make him laugh. But he still wouldn't say anything.", "Albert Camus", "The Stranger"],
]);
    minutes.insert("13:27", &[
&["Raymond came back with Masson ", "around one-thirty", ". His arm was bandaged up and he had an adhesive plaster on the corner of his mouth. The doctor had told him it was nothing, but Raymond looked pretty grim. Masson tried to make him laugh. But he still wouldn't say anything.", "Albert Camus", "The Stranger"],
]);
    minutes.insert("13:28", &[
&["Raymond came back with Masson ", "around one-thirty", ". His arm was bandaged up and he had an adhesive plaster on the corner of his mouth. The doctor had told him it was nothing, but Raymond looked pretty grim. Masson tried to make him laugh. But he still wouldn't say anything.", "Albert Camus", "The Stranger"],
]);
    minutes.insert("13:29", &[
&["Raymond came back with Masson ", "around one-thirty", ". His arm was bandaged up and he had an adhesive plaster on the corner of his mouth. The doctor had told him it was nothing, but Raymond looked pretty grim. Masson tried to make him laugh. But he still wouldn't say anything.", "Albert Camus", "The Stranger"],
]);
    minutes.insert("13:30", &[
&["Lupin not having come down, I went up again at ", "half-past one", ", and said we dined at two; he said he \"would be there.\"", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["She was a sticker. A clock away in the town struck ", "half past one", ".", "Graham Greene", "Brighton Rock"],
&["Shredding and slicing, dividing and subdividing, the clocks of Harley Street nibbled at the June day, counselled submission, upheld authority, and pointed out in chorus the supreme advantages of a sense of proportion, until the mound of time was so far diminished that a commercial clock, suspended above a shop in Oxford Street, announced, genially and fraternally, as if it were a pleasure to Messrs Rigby and Lowndes to give the information gratis, that it was ", "half-past one", ".", "Virginia Woolf", "Mrs dalloway"],
]);
    minutes.insert("13:31", &[
&["Lupin not having come down, I went up again at ", "half-past one", ", and said we dined at two; he said he \"would be there.\"", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["She was a sticker. A clock away in the town struck ", "half past one", ".", "Graham Greene", "Brighton Rock"],
&["Shredding and slicing, dividing and subdividing, the clocks of Harley Street nibbled at the June day, counselled submission, upheld authority, and pointed out in chorus the supreme advantages of a sense of proportion, until the mound of time was so far diminished that a commercial clock, suspended above a shop in Oxford Street, announced, genially and fraternally, as if it were a pleasure to Messrs Rigby and Lowndes to give the information gratis, that it was ", "half-past one", ".", "Virginia Woolf", "Mrs dalloway"],
]);
    minutes.insert("13:32", &[
&["At the third stroke it will be ", "one ... thirty-two", " ... and twenty seconds. 'Beep ... beep ... beep.' Ford Prefect suppressed a little giggle of evil satisfaction, realized that he had no reason to suppress it, and laughed out loud, a wicked laugh.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("13:33", &[
&["He waited for the green light to show and then opened the door again on to the now empty cargo hold.'... ", "one ... thirty-three", " ... and fifty seconds.' Very nice.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("13:34", &[
&["'At the third stroke it will be ...' He tiptoed out and returned to the control cabin. '... ", "one ... thirty-four", " and twenty seconds.' The voice sounded as clear as if he was hearing it over a phone in London, which he wasn't, not by a long way.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
&["He then went and had a last thorough examination of the emergency suspended animation chamber, which was where he particularly wanted it to be heard. 'At the third stroke it will be ", "one ... thirty ... four", " ... precisely.'", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("13:35", &[
&["'At the third stroke it will be ...' He tiptoed out and returned to the control cabin. '... ", "one ... thirty-four", " and twenty seconds.' The voice sounded as clear as if he was hearing it over a phone in London, which he wasn't, not by a long way.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
&["He then went and had a last thorough examination of the emergency suspended animation chamber, which was where he particularly wanted it to be heard. 'At the third stroke it will be ", "one ... thirty ... four", " ... precisely.'", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("13:36", &[
&["'At the third stroke it will be ...' He tiptoed out and returned to the control cabin. '... ", "one ... thirty-four", " and twenty seconds.' The voice sounded as clear as if he was hearing it over a phone in London, which he wasn't, not by a long way.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
&["He then went and had a last thorough examination of the emergency suspended animation chamber, which was where he particularly wanted it to be heard. 'At the third stroke it will be ", "one ... thirty ... four", " ... precisely.'", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("13:37", &[
&["He had not dared to sleep in his rented car—you didn't sleep in your car when you worked for Jesus Castro—and he was beginning to hallucinate. Still, he was on the job, and he scribbled in his notebook:\" ", "1.37pm", " Subject appears to be getting laid.\"", "William Monahan", "Light House"],
]);
    minutes.insert("13:38", &[
&["He had not dared to sleep in his rented car—you didn't sleep in your car when you worked for Jesus Castro—and he was beginning to hallucinate. Still, he was on the job, and he scribbled in his notebook:\" ", "1.37pm", " Subject appears to be getting laid.\"", "William Monahan", "Light House"],
]);
    minutes.insert("13:39", &[
&["And it was now ", "1.39pm", " which was 23 minutes after the stop, which mean that we would be at the sea if the train didn't go in a big curve. But I didn't know if it went in a big curve.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert("13:40", &[
&["And it was now ", "1.39pm", " which was 23 minutes after the stop, which mean that we would be at the sea if the train didn't go in a big curve. But I didn't know if it went in a big curve.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert("13:41", &[
&["And it was now ", "1.39pm", " which was 23 minutes after the stop, which mean that we would be at the sea if the train didn't go in a big curve. But I didn't know if it went in a big curve.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
]);
    minutes.insert(
        "13:42",
        &[&[
            "The last note was recorded at ",
            "1.42pm",
            ": G.M. on site at H-by; will take over the matter.",
            "Stieg Larsson",
            "The Girl with the Dragon Tattoo",
        ]],
    );
    minutes.insert(
        "13:43",
        &[&[
            "The last note was recorded at ",
            "1.42pm",
            ": G.M. on site at H-by; will take over the matter.",
            "Stieg Larsson",
            "The Girl with the Dragon Tattoo",
        ]],
    );
    minutes.insert(
        "13:44",
        &[&[
            "By good luck, the next train was due at ",
            "forty-four minutes past one",
            ", and arrived at Yateland (the next station) ten minutes afterward.",
            "Wilkie Collins",
            "Mr. Policeman and the Cook",
        ]],
    );
    minutes.insert("13:45", &[
&["That period which is always so dangerous, when the wicket is bad, the ten minutes before lunch, proved fatal to two more of the enemy. The last man had just gone to the wickets, with the score at a hundred and thirty-one, when a ", "quarter to two", " arrived, and with it the luncheon interval.", "PG Wodehouse", "Mike"],
&["The blow fell at precisely ", "one forty-five", " (summer-time). Benson, my Aunt Agatha's butler, was offering me the fried potatoes at the moment, and such was my emotion that I lofted six of them on the sideboard with the spoon.", "P.G. Wodehouse", "Sir Roderick Comes to Lunch"],
]);
    minutes.insert("13:46", &[
&["That period which is always so dangerous, when the wicket is bad, the ten minutes before lunch, proved fatal to two more of the enemy. The last man had just gone to the wickets, with the score at a hundred and thirty-one, when a ", "quarter to two", " arrived, and with it the luncheon interval.", "PG Wodehouse", "Mike"],
&["The blow fell at precisely ", "one forty-five", " (summer-time). Benson, my Aunt Agatha's butler, was offering me the fried potatoes at the moment, and such was my emotion that I lofted six of them on the sideboard with the spoon.", "P.G. Wodehouse", "Sir Roderick Comes to Lunch"],
]);
    minutes.insert("13:47", &[
&["Poppy was sprawled on Brianne's bed, applying black mascara to her stubby lashes. Brianne was sitting at her desk, trying to complete an essay before the 2pm deadline. It was ", "1.47pm.", "", "Sue Townsend", "The Woman Who Went To Bed For A Year"],
]);
    minutes.insert("13:48", &[
&["It was ", "twelve minutes to two", " in the afternoon when Claude Moreau and his most-trusted field officer, Jacques Bergeron, arrived at the Georges Cinq station of the Paris Metro. They walked, separately, to the rear of the platform, each carrying a handheld radio, the frequencies calibrated to each other.", "Robert Ludlum", "The Apocalypse Watch"],
]);
    minutes.insert(
        "13:49",
        &[&[
            "The bookstall clerk had seen the passenger in grey pass the bookstall at ",
            "1.49",
            ", in the direction of the exit.",
            "Dorothy L. Sayers",
            "Five Red Herrings",
        ]],
    );
    minutes.insert("13:50", &[
&["Rahel's toy wristwatch had the time painted on it. ", "Ten to two", ". One of her ambitions was to own a watch on which she could change the time whenever she wanted to (which according to her was what Time was meant for in the first place).", "Arundhati Roy", "The God of Small Things"],
&["The best train of the day was the ", "one-fifty", " from Paddington which reached Polgarwith just after seven o'clock.", "Agatha Christie", "The Cornish Mystery"],
]);
    minutes.insert("13:51", &[
&["Rahel's toy wristwatch had the time painted on it. ", "Ten to two", ". One of her ambitions was to own a watch on which she could change the time whenever she wanted to (which according to her was what Time was meant for in the first place).", "Arundhati Roy", "The God of Small Things"],
&["The best train of the day was the ", "one-fifty", " from Paddington which reached Polgarwith just after seven o'clock.", "Agatha Christie", "The Cornish Mystery"],
]);
    minutes.insert("13:52", &[
&["Rahel's toy wristwatch had the time painted on it. ", "Ten to two", ". One of her ambitions was to own a watch on which she could change the time whenever she wanted to (which according to her was what Time was meant for in the first place).", "Arundhati Roy", "The God of Small Things"],
&["The best train of the day was the ", "one-fifty", " from Paddington which reached Polgarwith just after seven o'clock.", "Agatha Christie", "The Cornish Mystery"],
]);
    minutes.insert("13:53", &[
&["Rahel's toy wristwatch had the time painted on it. ", "Ten to two", ". One of her ambitions was to own a watch on which she could change the time whenever she wanted to (which according to her was what Time was meant for in the first place).", "Arundhati Roy", "The God of Small Things"],
&["The best train of the day was the ", "one-fifty", " from Paddington which reached Polgarwith just after seven o'clock.", "Agatha Christie", "The Cornish Mystery"],
]);
    minutes.insert("13:54", &[
&["Rahel's toy wristwatch had the time painted on it. ", "Ten to two", ". One of her ambitions was to own a watch on which she could change the time whenever she wanted to (which according to her was what Time was meant for in the first place).", "Arundhati Roy", "The God of Small Things"],
&["The best train of the day was the ", "one-fifty", " from Paddington which reached Polgarwith just after seven o'clock.", "Agatha Christie", "The Cornish Mystery"],
]);
    minutes.insert("13:55", &[
&["If I was punctual in quitting Mlle. Reuter's domicile, I was at least equally punctual in arriving there; I came the next day at ", "five minutes before two", ", and on reaching the schoolroom door, before I opened it, I heard a rapid, gabbling sound, which warned me that the \"priere du midi\" was not yet concluded.", "Charlotte Brontë", "The Professor"],
]);
    minutes.insert("13:56", &[
&["If I was punctual in quitting Mlle. Reuter's domicile, I was at least equally punctual in arriving there; I came the next day at ", "five minutes before two", ", and on reaching the schoolroom door, before I opened it, I heard a rapid, gabbling sound, which warned me that the \"priere du midi\" was not yet concluded.", "Charlotte Brontë", "The Professor"],
]);
    minutes.insert(
        "13:57",
        &[&[
            "I looked for a clock. It was ",
            "three minutes to two",
            ". \"I hope you can catch him, then. Thank you. I really appreciate it.\"",
            "C.E. Murphy",
            "Urban Shaman",
        ]],
    );
    minutes.insert("13:58", &[
&["It was ", "almost two o’clock", ", but nothing moved, Stari Teočak was silent and so empty it seemed abandoned, and yet Tijmen constantly felt he was being observed by invisible eyes.", "Arnold Jansen op de Haar", "King of Tuzla"],
]);
    minutes.insert("13:59", &[
&["For twenty minutes he sat and watched as the gap between the ship and Epun closed, as the ship's computer teased and kneaded the numbers that would bring it into a loop around the little moon, and close the loop and keep it there, orbiting in perpetual obscurity. '", "One ... fifty-nine …", "'\"", "Douglas Adams", "So Long, and Thanks for All the Fish"],
]);
    minutes.insert("14:00", &[
&["'She could have fired the jig, and he could have kept on picking up his packages at the old time, ", "two o'clock", ". As it was, he had almost been arrested.'", "John Kennedy Toole", "A Confederacy of Dunces"],
&["\"The old people's home is at Marengo, fifty miles from Algiers. I'll catch the ", "two o'clock", " bus and get there in the afternoon.\".... \"I caught the two o'clock bus. It was very hot.\"", "Albert Camus", "The Outsider"],
&["At approximately ", "1400 hours", " a pair of enemy Skyhawks came flying in at deck level out of nowhere.", "David Mitchell", "Black Swan Green"],
&["At ", "two o'clock", " Gatsby put on his bathing suit and left word with the butler that if any one phoned word was to be brought to him at the pool.", "F. Scott Fitzgerald", "The Great Gatsby"],
&["", "At two", ", the snowplows were in action in Lillestrom.", "Jo Nesbo", "The Snowman"],
&["I caught the ", "two o'clock", " bus. It was very hot. I ate at Céleste's restaurant as usual. They all felt very sorry for me and Céleste told me, 'There's no one like a mother'.", "Albert Camus", "The Outsider"],
&["The Home for Aged Persons is at Marengo, some fifty miles from Algiers. With the ", "two o'clock", " bus I should get there well before nightfall. Then I can spend the night there, keeping the usual vigil beside the body, and be back here by tomorrow evening.", "Albert Camus", "The Outsider"],
&["When Salander woke up it was ", "2.00", " on Saturday afternoon and a doctor was poking at her.", "Stieg Larsson", "The Girl Who Kicked the Hornets' Nest"],
]);
    minutes.insert("14:01", &[
&["At ", "about two o' clock", " the owners young wife came, carrying a handleless cup and a pot with a quilted cover, to where I was still lying disconsolate", "John Hershey", "A Single Pebble"],
&["The next day was Saturday and, now that Moon was done, I decided to bring the job to its end. So I sent word that I shouldn't be able to umpire for the team at Steeple Sinderby and, after working through the morning, came down ", "about two o'clock", ".", "JL Carr", "A Month in the Country"],
]);
    minutes.insert("14:02", &[
&["\"I'm not dead. How did that happen?\" He was right. It was ", "14.02", " and twenty-six seconds. Destiny had not been fulfilled. We all looked at each other, confused.", "Jasper Fforde", "The Woman Who Died A Lot"],
]);
    minutes.insert("14:03", &[
&["\"I'm not dead. How did that happen?\" He was right. It was ", "14.02", " and twenty-six seconds. Destiny had not been fulfilled. We all looked at each other, confused.", "Jasper Fforde", "The Woman Who Died A Lot"],
]);
    minutes.insert("14:04", &[
&["", "2.04pm.", " Once again, the Quartermaster-General's office came on the line asking for Colonel Finckh, and once again Finckh heard the quiet, unemotional, unfamiliar voice", "Hans Hellmut Kirst", "The Night of the Generals"],
]);
    minutes.insert("14:05", &[
&["...and at ", "five past two", " on 17 September of that same unforgettable year 1916, I was in the Muryovo hospital yard, standing on trampled withered grass, flattened by the September rain.", "Mikhail Bulgakov", "A Country Doctor's Notebook"],
]);
    minutes.insert(
        "14:06",
        &[&[
            "A man driving a tractor saw her, four hundred yards from her house, ",
            "six minutes past two",
            " in the afternoon.",
            "Hilary Mantel",
            "A Change of Climate",
        ]],
    );
    minutes.insert(
        "14:07",
        &[&[
            "A man driving a tractor saw her, four hundred yards from her house, ",
            "six minutes past two",
            " in the afternoon.",
            "Hilary Mantel",
            "A Change of Climate",
        ]],
    );
    minutes.insert(
        "14:08",
        &[&[
            "A man driving a tractor saw her, four hundred yards from her house, ",
            "six minutes past two",
            " in the afternoon.",
            "Hilary Mantel",
            "A Change of Climate",
        ]],
    );
    minutes.insert(
        "14:09",
        &[&[
            "A man driving a tractor saw her, four hundred yards from her house, ",
            "six minutes past two",
            " in the afternoon.",
            "Hilary Mantel",
            "A Change of Climate",
        ]],
    );
    minutes.insert("14:10", &[
&["Mrs Eunice Harris pulls back the sleeve of her good coat and checks her good watch. \"Indeed yes. Half twelve,\" and waves a hand at the Town Hall clock as if it was hers. \"Always ", "ten past two", ". Someone put a nail in the time years back.\"", "Vanessa Gebbie", "The Coward's Tale"],
]);
    minutes.insert("14:11", &[
&["Mrs Eunice Harris pulls back the sleeve of her good coat and checks her good watch. \"Indeed yes. Half twelve,\" and waves a hand at the Town Hall clock as if it was hers. \"Always ", "ten past two", ". Someone put a nail in the time years back.\"", "Vanessa Gebbie", "The Coward's Tale"],
]);
    minutes.insert("14:12", &[
&["Mrs Eunice Harris pulls back the sleeve of her good coat and checks her good watch. \"Indeed yes. Half twelve,\" and waves a hand at the Town Hall clock as if it was hers. \"Always ", "ten past two", ". Someone put a nail in the time years back.\"", "Vanessa Gebbie", "The Coward's Tale"],
]);
    minutes.insert(
        "14:13",
        &[&[
            "At the third stroke, it will be ",
            "two ... thirteen",
            " ... and fifty seconds.'",
            "Douglas Adams",
            "So Long, and Thanks for All the Fish",
        ]],
    );
    minutes.insert(
        "14:14",
        &[&[
            "At the third stroke, it will be ",
            "two ... thirteen",
            " ... and fifty seconds.'",
            "Douglas Adams",
            "So Long, and Thanks for All the Fish",
        ]],
    );
    minutes.insert("14:15", &[
&["I had a date with her next day at ", "2.15 P.M.", " In my own rooms, but it was less successful, she seemed to have grown less juvenile, more of a woman overnight.", "Vladimir Nabokov", "Lolita"],
&["", "2.15 p.m.", " Second afternoon class", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["I had a date with her next day at ", "2.15pm", " in my own rooms, but it was less successful, she seemed to have grown less juvenile, more of a woman overnight. A cold I caught from her led me to cancel a fourth assignment, nor was I sorry to break an emotional series that threatened to burden me with heart-rending fantasies and peter out in dull disappointment. So let her remain, sleek, slender Monique, as she was for a minute or two", "Vladimir Nabokov", "Lolita"],
]);
    minutes.insert("14:16", &[
&["Oh, good evening. I think you were on the barrier when I came in at ", "2.16", " this afternoon. Now, do you know that you let me get past without giving up my ticket? Yes, yes he-he! I really think you ought to be more careful", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("14:17", &[
&["Oh, good evening. I think you were on the barrier when I came in at ", "2.16", " this afternoon. Now, do you know that you let me get past without giving up my ticket? Yes, yes he-he! I really think you ought to be more careful", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("14:18", &[
&["Oh, good evening. I think you were on the barrier when I came in at ", "2.16", " this afternoon. Now, do you know that you let me get past without giving up my ticket? Yes, yes he-he! I really think you ought to be more careful", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("14:19", &[
&["", "2:19", ": Duane Hinton walks out. He walks through the backyard. He lugs some clothes. He wore said clothes last night. He walks to the fence. He feeds the incinerator. He lights a match.", "James Ellroy", "The Cold Six Thousand"],
]);
    minutes.insert("14:20", &[
&["The having originated a precaution which was already in course of execution, was a great relief to Miss Pross. The necessity of composing her appearance so that it should attract no special notice in the streets, was another relief. She looked at her watch, and it was ", "twenty minutes past two", ". She had no time to lose, but must get ready at once.", "Charles Dickens", "A tale of two cities"],
&["Inevitable, implacable, the rainstorm wept itself out. She saw Tom look at his watch. 'What time is it?' '", "Twenty past two", ". Want to go back to the hotel for a while?' 'All right.' They walked out of the gardens and down the rue de Vaugirard. This holiday, unlike those holidays long ago, would not end with her sleeping at home. Two nights from now I will be high over the Atlantic Ocean and on Saturday I will be walking around in the Other Place. I am going to America. I am starting my life over again. But as she said these words to herself, she found it hard to imagine what the new life would be like. And, again, she was afraid.", "Brian Moore", "The Doctor's Wife"],
&["She looked at her watch and it was ", "twenty minutes past two", ". She had no time to lose but must get ready at once.", "Charles Dickens", "A Tale of Two Cities"],
&["The watch found at the Weir was challenged by the jeweller as one he had wound and set for Edwin Drood, at ", "twenty minutes past two", " on that same afternoon; and it had run down, before being cast into the water; and it was the jeweller's positive opinion that it had never been re-wound.", "Charles Dickens", "The Mystery of Edwin Drood"],
]);
    minutes.insert("14:21", &[
&["The having originated a precaution which was already in course of execution, was a great relief to Miss Pross. The necessity of composing her appearance so that it should attract no special notice in the streets, was another relief. She looked at her watch, and it was ", "twenty minutes past two", ". She had no time to lose, but must get ready at once.", "Charles Dickens", "A tale of two cities"],
&["Inevitable, implacable, the rainstorm wept itself out. She saw Tom look at his watch. 'What time is it?' '", "Twenty past two", ". Want to go back to the hotel for a while?' 'All right.' They walked out of the gardens and down the rue de Vaugirard. This holiday, unlike those holidays long ago, would not end with her sleeping at home. Two nights from now I will be high over the Atlantic Ocean and on Saturday I will be walking around in the Other Place. I am going to America. I am starting my life over again. But as she said these words to herself, she found it hard to imagine what the new life would be like. And, again, she was afraid.", "Brian Moore", "The Doctor's Wife"],
&["She looked at her watch and it was ", "twenty minutes past two", ". She had no time to lose but must get ready at once.", "Charles Dickens", "A Tale of Two Cities"],
&["The watch found at the Weir was challenged by the jeweller as one he had wound and set for Edwin Drood, at ", "twenty minutes past two", " on that same afternoon; and it had run down, before being cast into the water; and it was the jeweller's positive opinion that it had never been re-wound.", "Charles Dickens", "The Mystery of Edwin Drood"],
]);
    minutes.insert("14:22", &[
&["Garth here. Sunday afternoon. Sorry to miss you, but I'll leave a brief message on your tape. ", "Two-twenty-two", " or there-aboutish. Great party.", "Carol Shields", "Larry's Party"],
]);
    minutes.insert("14:23", &[
&["Garth here. Sunday afternoon. Sorry to miss you, but I'll leave a brief message on your tape. ", "Two-twenty-two", " or there-aboutish. Great party.", "Carol Shields", "Larry's Party"],
]);
    minutes.insert("14:24", &[
&["Garth here. Sunday afternoon. Sorry to miss you, but I'll leave a brief message on your tape. ", "Two-twenty-two", " or there-aboutish. Great party.", "Carol Shields", "Larry's Party"],
]);
    minutes.insert("14:25", &[
&["Gary shut himself inside his office and flipped through the messages. Caroline had called at 1:35, 1:40, 1:50, 1:55, and 2:10; it was now ", "2:25", ". He pumped his fist in triumph. Finally, finally, some evidence of desperation.", "Jonathan Franzen", "The Corrections"],
]);
    minutes.insert("14:26", &[
&["Gary shut himself inside his office and flipped through the messages. Caroline had called at 1:35, 1:40, 1:50, 1:55, and 2:10; it was now ", "2:25", ". He pumped his fist in triumph. Finally, finally, some evidence of desperation.", "Jonathan Franzen", "The Corrections"],
]);
    minutes.insert("14:27", &[
&["Gary shut himself inside his office and flipped through the messages. Caroline had called at 1:35, 1:40, 1:50, 1:55, and 2:10; it was now ", "2:25", ". He pumped his fist in triumph. Finally, finally, some evidence of desperation.", "Jonathan Franzen", "The Corrections"],
]);
    minutes.insert("14:28", &[
&["It happened to be the case that the sixty-based system coincided with our our current method of keeping time... Apparently they wanted us to know that that something might happen at ", "28 minutes and 57 seconds after 2pm", " on a day yet to be specified.", "Don DeLillo", "Ratner's Star"],
]);
    minutes.insert("14:29", &[
&["It happened to be the case that the sixty-based system coincided with our our current method of keeping time... Apparently they wanted us to know that that something might happen at ", "28 minutes and 57 seconds after 2pm", " on a day yet to be specified.", "Don DeLillo", "Ratner's Star"],
]);
    minutes.insert("14:30", &[
&["Ach! It's ", "2:30", ". Look how the time is flying. And it's still so much to do today.. It's dishes to clean, dinner to defrost, and my pills I haven't yet counted. I don't get it... Why didn't the Jews at least try to resist? It wasn't so easy like you think. Everybody was so starving and frightened, and tired they couldn't believe even what's in front of their eyes.", "Art Spiegelman", "Maus"],
&["At ", "2.30pm", " on the 13th inst. began to shadow Sir Bobadil the Ostrich, whom I suspect of being the criminal. Shadowing successful. Didn't lose sight of him once.", "Eric Linklater", "The Wind on the Moon"],
&["At ", "half past two", " the same afternoon the boy and the elderly man are standing in the room directly above the Inner Office and Waiting-Room.", "John Berger", "Corker's Freedom"],
&["It was ", "half-past two", " in the afternoon. The sun hung in the faded blue sky like a burning mirror, and away beyond the paddocks the blue mountains quivered and leapt like sea. Sid wouldn't be back until half-past ten. He had ridden over to the township with four of the boys to help hunt down the young fellow who'd murdered Mr. Williamson. Such a dreadful thing!", "Katherine Mansfield", "Millie"],
&["It was ", "half-past two", " o'clock when the knock came. I took my courage a deux mains and waited. In a few minutes Mary opened the door, and announced \"Dr. Van Helsing\".", "Bram Stoker", "Dracula"],
&["May 14th 1800. Wm and John set off into Yorkshire after dinner at ", "1/2 past 2 o'clock", ", cold pork in their pockets. I left them at the turning of the Low-wood bay under the trees. My heart was so full that I could barely speak to W. when I gave him a farewell kiss.", "Dorothy Wordsworth", "The Journals of Dorothy Wordsworth"],
]);
    minutes.insert("14:31", &[
&["Ach! It's ", "2:30", ". Look how the time is flying. And it's still so much to do today.. It's dishes to clean, dinner to defrost, and my pills I haven't yet counted. I don't get it... Why didn't the Jews at least try to resist? It wasn't so easy like you think. Everybody was so starving and frightened, and tired they couldn't believe even what's in front of their eyes.", "Art Spiegelman", "Maus"],
&["At ", "2.30pm", " on the 13th inst. began to shadow Sir Bobadil the Ostrich, whom I suspect of being the criminal. Shadowing successful. Didn't lose sight of him once.", "Eric Linklater", "The Wind on the Moon"],
&["At ", "half past two", " the same afternoon the boy and the elderly man are standing in the room directly above the Inner Office and Waiting-Room.", "John Berger", "Corker's Freedom"],
&["It was ", "half-past two", " in the afternoon. The sun hung in the faded blue sky like a burning mirror, and away beyond the paddocks the blue mountains quivered and leapt like sea. Sid wouldn't be back until half-past ten. He had ridden over to the township with four of the boys to help hunt down the young fellow who'd murdered Mr. Williamson. Such a dreadful thing!", "Katherine Mansfield", "Millie"],
&["It was ", "half-past two", " o'clock when the knock came. I took my courage a deux mains and waited. In a few minutes Mary opened the door, and announced \"Dr. Van Helsing\".", "Bram Stoker", "Dracula"],
&["May 14th 1800. Wm and John set off into Yorkshire after dinner at ", "1/2 past 2 o'clock", ", cold pork in their pockets. I left them at the turning of the Low-wood bay under the trees. My heart was so full that I could barely speak to W. when I gave him a farewell kiss.", "Dorothy Wordsworth", "The Journals of Dorothy Wordsworth"],
]);
    minutes.insert("14:32", &[
&["Like ", "2.32 p.m.", ", Beecher and Avalon, L3 R2 (which meant left three blocks, right two) 2:35 p.m., and you wondered how you could pick up one box, then drive 5 blocks in 3 minutes and be finished cleaning out another box.", "Charles Bukowski", "Post Office"],
]);
    minutes.insert("14:33", &[
&["Like ", "2.32 p.m.", ", Beecher and Avalon, L3 R2 (which meant left three blocks, right two) 2:35 p.m., and you wondered how you could pick up one box, then drive 5 blocks in 3 minutes and be finished cleaning out another box.", "Charles Bukowski", "Post Office"],
]);
    minutes.insert("14:34", &[
&["Like ", "2.32 p.m.", ", Beecher and Avalon, L3 R2 (which meant left three blocks, right two) 2:35 p.m., and you wondered how you could pick up one box, then drive 5 blocks in 3 minutes and be finished cleaning out another box.", "Charles Bukowski", "Post Office"],
]);
    minutes.insert("14:35", &[
&["Like ", "2.32 p.m.", ", Beecher and Avalon, L3 R2 (which meant left three blocks, right two) 2:35 p.m., and you wondered how you could pick up one box, then drive 5 blocks in 3 minutes and be finished cleaning out another box.", "Charles Bukowski", "Post Office"],
]);
    minutes.insert(
        "14:36",
        &[&[
            "I look at my watch. ",
            "Two thirty-six",
            ". All I've got left today is take in the laundry and fix dinner.",
            "Haruki Murakami",
            "The Elephant Vanishes",
        ]],
    );
    minutes.insert(
        "14:37",
        &[&[
            "I look at my watch. ",
            "Two thirty-six",
            ". All I've got left today is take in the laundry and fix dinner.",
            "Haruki Murakami",
            "The Elephant Vanishes",
        ]],
    );
    minutes.insert(
        "14:38",
        &[&[
            "I look at my watch. ",
            "Two thirty-six",
            ". All I've got left today is take in the laundry and fix dinner.",
            "Haruki Murakami",
            "The Elephant Vanishes",
        ]],
    );
    minutes.insert("14:39", &[
&["Noo, there's a report come in fra' the station-master at Pinwherry that there was a gentleman tuk the ", "2.39", " at Pinwherry.", "Dorothy L. Sayers", "Five Red Herrings"],
]);
    minutes.insert("14:40", &[
&["If a girl looks swell when she meets you, who gives a damn when she's late? 'We better hurry', I said. 'The show starts at ", "two-forty", ".'", "J.D. Salinger", "The Catcher in the Rye"],
&["Members of Big Side marked Michael and Alan as the two most promising three-quarters for Middle Side next year, and when the bell sounded at ", "twenty minutes to three", ", the members of Big Side would walk with Michael and Alan towards the changing room and encourage them by flattery and genial ragging.", "Compton Mackenzie", "Sinister Street"],
]);
    minutes.insert("14:41", &[
&["At ", "2.41", ", when the afternoon fast train to London was pulling out of Larborough prompt to the minute, Miss Pym sat under the cedar on the lawn wondering whether she was a fool, and not caring much anyhow.", "Josephine Tey", "Miss Pym Disposes"],
]);
    minutes.insert("14:42", &[
&["At ", "2.41", ", when the afternoon fast train to London was pulling out of Larborough prompt to the minute, Miss Pym sat under the cedar on the lawn wondering whether she was a fool, and not caring much anyhow.", "Josephine Tey", "Miss Pym Disposes"],
]);
    minutes.insert("14:43", &[
&["Jacobson died at ", "2.43pm", " the next day after slashing his wrists with a razor blade in the second cubicle from the left in the men's washroom on the third floor.", "JG Ballard", "Now: Zero"],
]);
    minutes.insert("14:44", &[
&["Jacobson died at ", "2.43pm", " the next day after slashing his wrists with a razor blade in the second cubicle from the left in the men's washroom on the third floor.", "JG Ballard", "Now: Zero"],
]);
    minutes.insert("14:45", &[
&["He never came down till a ", "quarter to three", ".", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["Pull the other one, and tell it to the marines, and don't make me laugh, and fuck off out of it, and all that, but the fact remained that it was still only ", "two forty-five", "'.", "Martin Amis", "The Pregnant Window"],
&["What time is it?' 'Look for yourself,' the old woman says to me. I look, and I see the clock has no hands. 'There are no hands,' I say. The old woman looks at the clock face and says to me, 'It's a ", "quarter to three", "'.", "Daniil Ivanovich Kharms", "The Old Woman"],
]);
    minutes.insert("14:46", &[
&["He never came down till a ", "quarter to three", ".", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["Pull the other one, and tell it to the marines, and don't make me laugh, and fuck off out of it, and all that, but the fact remained that it was still only ", "two forty-five", "'.", "Martin Amis", "The Pregnant Window"],
&["What time is it?' 'Look for yourself,' the old woman says to me. I look, and I see the clock has no hands. 'There are no hands,' I say. The old woman looks at the clock face and says to me, 'It's a ", "quarter to three", "'.", "Daniil Ivanovich Kharms", "The Old Woman"],
]);
    minutes.insert("14:47", &[
&["He never came down till a ", "quarter to three", ".", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["Pull the other one, and tell it to the marines, and don't make me laugh, and fuck off out of it, and all that, but the fact remained that it was still only ", "two forty-five", "'.", "Martin Amis", "The Pregnant Window"],
&["What time is it?' 'Look for yourself,' the old woman says to me. I look, and I see the clock has no hands. 'There are no hands,' I say. The old woman looks at the clock face and says to me, 'It's a ", "quarter to three", "'.", "Daniil Ivanovich Kharms", "The Old Woman"],
]);
    minutes.insert("14:48", &[
&["He never came down till a ", "quarter to three", ".", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["Pull the other one, and tell it to the marines, and don't make me laugh, and fuck off out of it, and all that, but the fact remained that it was still only ", "two forty-five", "'.", "Martin Amis", "The Pregnant Window"],
&["What time is it?' 'Look for yourself,' the old woman says to me. I look, and I see the clock has no hands. 'There are no hands,' I say. The old woman looks at the clock face and says to me, 'It's a ", "quarter to three", "'.", "Daniil Ivanovich Kharms", "The Old Woman"],
]);
    minutes.insert("14:49", &[
&["He never came down till a ", "quarter to three", ".", "George and Weedon Grossmith", "The Diary of a Nobody"],
&["Pull the other one, and tell it to the marines, and don't make me laugh, and fuck off out of it, and all that, but the fact remained that it was still only ", "two forty-five", "'.", "Martin Amis", "The Pregnant Window"],
&["What time is it?' 'Look for yourself,' the old woman says to me. I look, and I see the clock has no hands. 'There are no hands,' I say. The old woman looks at the clock face and says to me, 'It's a ", "quarter to three", "'.", "Daniil Ivanovich Kharms", "The Old Woman"],
]);
    minutes.insert(
        "14:50",
        &[&[
            "Stands the Church clock at ",
            "ten to three",
            "? And is there honey still for tea?",
            "Rupert Brooke",
            "The Old Vicarage, Grantchester",
        ]],
    );
    minutes.insert(
        "14:51",
        &[&[
            "Stands the Church clock at ",
            "ten to three",
            "? And is there honey still for tea?",
            "Rupert Brooke",
            "The Old Vicarage, Grantchester",
        ]],
    );
    minutes.insert(
        "14:52",
        &[&[
            "Stands the Church clock at ",
            "ten to three",
            "? And is there honey still for tea?",
            "Rupert Brooke",
            "The Old Vicarage, Grantchester",
        ]],
    );
    minutes.insert(
        "14:53",
        &[&[
            "Stands the Church clock at ",
            "ten to three",
            "? And is there honey still for tea?",
            "Rupert Brooke",
            "The Old Vicarage, Grantchester",
        ]],
    );
    minutes.insert("14:54", &[
&["In the end, it was the Sunday afternoons he couldn’t cope with, and that terrible listlessness that starts to set in ", "about 2.55", ", when you know you’ve had all the baths you can usefully have that day, that however hard you stare at any given paragraph in the newspaper you will never actually read it, or use the revolutionary new pruning technique it describes, and that as you stare at the clock the hands will move relentlessly on to four o’clock, and you will enter the long dark teatime of the soul.", "Douglas Adams", "Life, the universe and everything"],
]);
    minutes.insert("14:55", &[
&["The superior, the very reverend John Conmee SJ reset his smooth watch in his interior pocket as he came down the presbytery steps. ", "Five to three", ". Just nice time to walk to Artane.", "James Joyce", "Ulysses"],
]);
    minutes.insert("14:56", &[
&["", "2.56 P.M.", " Helen is alone now. Her face is out of frame, and through the viewfinder I see only a segment of the pillow, an area of crumpled sheet and the upper section of her chest and shoulders.", "JG Ballard", "The 60 Minute Zoom"],
]);
    minutes.insert("14:57", &[
&["", "2.56 P.M.", " Helen is alone now. Her face is out of frame, and through the viewfinder I see only a segment of the pillow, an area of crumpled sheet and the upper section of her chest and shoulders.", "JG Ballard", "The 60 Minute Zoom"],
]);
    minutes.insert("14:58", &[
&["From twenty minutes past nine until twenty-seven minutes past nine, from twenty-five minutes past eleven until twenty-eight minutes past eleven, from ten minutes to three until ", "two minutes to three", " the heroes of the school met in a large familiarity whose Olympian laughter awed the fearful small boy that flitted uneasily past and chilled the slouching senior that rashly paused to examine the notices in assertion of an unearned right.", "Compton Mackenzie", "Sinister Street"],
&["We betted that it would happen on the morrow; they took us up and gave us the odds of two to one; we betted that it would happen in the afternoon; we got odds of four to one on that; we betted that it would happen at ", "two minutes to three", "; they willingly granted us the odds of ten to one on that.", "Mark Twain", "The Chronicle of Young Satan"],
]);
    minutes.insert("14:59", &[
&["From twenty minutes past nine until twenty-seven minutes past nine, from twenty-five minutes past eleven until twenty-eight minutes past eleven, from ten minutes to three until ", "two minutes to three", " the heroes of the school met in a large familiarity whose Olympian laughter awed the fearful small boy that flitted uneasily past and chilled the slouching senior that rashly paused to examine the notices in assertion of an unearned right.", "Compton Mackenzie", "Sinister Street"],
&["We betted that it would happen on the morrow; they took us up and gave us the odds of two to one; we betted that it would happen in the afternoon; we got odds of four to one on that; we betted that it would happen at ", "two minutes to three", "; they willingly granted us the odds of ten to one on that.", "Mark Twain", "The Chronicle of Young Satan"],
]);
    minutes.insert("15:00", &[
&["'I gotta get uptown by ", "three o'clock", ".'", "John Kennedy Toole", "A Confederacy of Dunces"],
&["\"Remember,\" they shouted, \"battle at ", "three o'clock", " sharp. There's no time to lose.\"", "Arthur Ransome", "Swallows and Amazons"],
&["And the sound of the bell flooded the room with its melancholy wave; which receded, and gathered itself together to fall once more, when she heard, distractedly, something fumbling, something scratching at the door. Who at this hour? ", "Three", ", good Heavens! Three already!", "Virginia Woolf", "Mrs Dalloway"],
&["At ", "three o'clock", " on the afternoon of that same day, he called on her. She held out her two hands, smiling in her usual charming, friendly way; and for a few seconds they looked deep into each other's eyes.", "Guy de Maupassant", "Bel-Ami"],
&["At ", "three o’clock", " precisely I was at Baker Street, but Holmes had not yet returned.", "Sir Arthur Conan Doyle", "A Scandal in Bohemia"],
&["", "At three", " on the Wednesday afternoon, that bit of the painting was completed.", "Wilkie Collins", "The Moonstone"],
&["Ditched by the woman I loved, I exalted my suffering into a sign of greatness (lying collapsed on a bed ", "at three", " in the afternoon), and hence protected myself from experiencing my grief as the outcome of what was at best a mundane romantic break-up. Chloe's departure may have killed me, but it had at least left me in glorious possession of the moral high ground. I was a martyr.", "Alain de Botton", "Essays in Love"],
&["He walks into the Hospital for Broken Things at ", "three o'clock", " on Monday afternoon. That was the arrangement. If he came in after six o'clock, he was to head straight for the house in Sunset Park.", "Paul Auster", "Sunset Park"],
&["I had a ", "three o’clock", " class in psychology, the first meeting of the semester, and I suspected I was going to miss it. I was right. Victoria made a real ritual of the whole thing, clothes coming off with the masturbatory dalliance of a strip show, the covers rolling back periodically to show this patch of flesh or that, strategically revealed.", "T.C. Boyle", "Achates McNeil"],
&["It was ", "three o'clock", " in the beautiful breezy autumn day when Mr. Casaubon drove off to his Rectory at Lowick, only five miles from Tipton; and Dorothea, who had on her bonnet and shawl, hurried along the shrubbery and across the park that she might wander through the bordering wood with no other visible companionship than that of Monk, the Great St. Bernard dog, who always took care of the young ladies in their walks", "George Eliot", "Middlemarch"],
&["Ladies bathed before noon, after their ", "three-o’clock", " naps, and by nightfall were like soft teacakes with frostings of sweat and sweet talcum.", "Harper Lee", "To kill a mockingbird"],
&["M. Madeleine usually came at ", "three o'clock", ", and as punctuality was kindness, he was punctual.", "Victor Hugo", "Les Miserables"],
&["On Wednesday at ", "three o'clock", ", Monsieur and Madame Bovary, seated in their dog-cart, set out for Vaubyessard, with a great trunk strapped on behind and a bonnet-box in front of the apron. Besides these Charles held a bandbox between his knees.", "Gustave Flaubert", "Madame Bovary"],
&["The scent and smoke and sweat of a casino are nauseating ", "at three", " in the morning. Then the soul-erosion produced by high gambling - a compost of greed and fear and nervous tension - becomes unbearable and the senses awake and revolt from it.", "Ian Fleming", "Casino Royale"],
&["", "Three o'clock", " is always too late or too early for anything you want to do.", "Jean-Paul Sartre", "Nausea"],
&["", "Three o'clock", " is the perfect time in Cham, because anything is possible. You can still ski, but also respectably start drinking, the shops have just reopened, the sun is still up. Three o'clock is never too late or too early.", "Jonathan Trigell", "Cham"],
&["Today was the day Alex had appointed for her 'punishment'. I became increasingly nervous as the hour of ", "three o'clock", " approached. I was alone in the house, and paced restlessly from room to room, glancing at the clocks in each of them.", "David Lodge", "Deaf Sentence"],
]);
    minutes.insert("15:01", &[
&["The sun was now setting. It was ", "about three", " in the afternoon when Alisande had begun to tell me who the cowboys were; so she had made pretty good progress with it - for her. She would arrive some time or other, no doubt, but she was not a person who could be hurried.", "Mark Twain", "A Connecticut Yankee in King Arthur's Court"],
]);
    minutes.insert("15:02", &[
&["The sun was now setting. It was ", "about three", " in the afternoon when Alisande had begun to tell me who the cowboys were; so she had made pretty good progress with it - for her. She would arrive some time or other, no doubt, but she was not a person who could be hurried.", "Mark Twain", "A Connecticut Yankee in King Arthur's Court"],
]);
    minutes.insert("15:03", &[
&["I check Shingi's mobile phone - it says it's ", "3.03pm.", " I get out of bed, open my suitcase to take clean socks out and the smell of Mother hit my nose and make me feel dizzy.", "Brian Chikwava", "Harare North"],
]);
    minutes.insert(
        "15:04",
        &[&[
            "Woken at ",
            "1504",
            " by Michelangelo hammering away with his chisel.",
            "Hallgrímur Helgason",
            "101 Reykjavik",
        ]],
    );
    minutes.insert("15:05", &[
&["Ultimately, at ", "five minutes past three", " that afternoon, Smith admitted the falsity of the Fort Scott tale. \"That was only something Dick told his family. So he could stay out overnight. Do some drinking.\"", "Truman Capote", "In Cold Blood"],
]);
    minutes.insert("15:06", &[
&["Ultimately, at ", "five minutes past three", " that afternoon, Smith admitted the falsity of the Fort Scott tale. \"That was only something Dick told his family. So he could stay out overnight. Do some drinking.\"", "Truman Capote", "In Cold Blood"],
]);
    minutes.insert("15:07", &[
&["The next day was grey, threatening rain. He was there at ", "seven minutes past three", ". The clock on the church over the way pointed to it. They had arranged to be there at three fifteen. Therefore, if she had been there when he came, she would have been eight minutes before her time.", "Patrick Hamilton", "Twenty Thousand Streets Under The Sky"],
]);
    minutes.insert("15:08", &[
&["A private wireless telegraph which would transmit by dot and dash system the result of a national equine handicap (flat or steeplechase) of 1 or more miles and furlongs won by an outsider at odds of 50 to 1 at ", "3 hr 8 m p.m.", " at Ascot (Greenwich time), the message being received and available for betting purposes in Dublin at 2.59 p.m.", "James Joyce", "Ulysses"],
]);
    minutes.insert(
        "15:09",
        &[&[
            "On the next day he boarded the London train which reaches Hull at ",
            "3.09",
            ". At Paragon Station he soon singled out Beamish from Merriman's description.",
            "Freeman Wills Crofts",
            "The Pit-Prop Syndicate",
        ]],
    );
    minutes.insert(
        "15:10",
        &[&[
            "This time it was only the simple fact that the hands chanced to point to ",
            "3.10pm",
            ", the precise moment at which all the clocks of London had stopped.",
            "M.P. Shiel",
            "The Purple Cloud",
        ]],
    );
    minutes.insert(
        "15:11",
        &[&[
            "This time it was only the simple fact that the hands chanced to point to ",
            "3.10pm",
            ", the precise moment at which all the clocks of London had stopped.",
            "M.P. Shiel",
            "The Purple Cloud",
        ]],
    );
    minutes.insert(
        "15:12",
        &[&[
            "This time it was only the simple fact that the hands chanced to point to ",
            "3.10pm",
            ", the precise moment at which all the clocks of London had stopped.",
            "M.P. Shiel",
            "The Purple Cloud",
        ]],
    );
    minutes.insert("15:13", &[
&["The lift moved. It was ", "thirteen minutes past three", ". The bell gave out its ping. Two men stepped out of the lift, Alan Norman and another man. Tony Blair walked into the office.", "Simon Kearns", "Virtual Assassin"],
]);
    minutes.insert("15:14", &[
&["A signal sounded. \"There's the ", "3.14", " up,\" said Perks. \"You lie low till she's through, and then we'll go up along to my place, and see if there's any of them strawberries ripe what I told you about.\"", "Edith Nesbit", "The Railway Children"],
&["I shall be back at exactly ", "THREE fourteen", ", for our hour of revery together, real sweet revery darling", "Jack Kerouac", "On the Road"],
]);
    minutes.insert("15:15", &[
&["Gordon was alone. He wandered back to the door. The strawberry-nosed man glanced over his shoulder, caught Gordon's eye, and moved off, foiled. He had been on the point of slipping Edgar Wallace into his pocket. The clock over the Prince of Wales struck a ", "quarter past three", ".", "George Orwell", "Keep the Aspidistra Flying"],
&["I got out my old clothes. I put wool socks over my regular socks and took my time lacing up the boots. I made a couple of tuna sandwiches and some double-decker peanut-butter crackers. I filled my canteen and attached the hunting knife and the canteen to my belt. As I was going out the door, I decided to leave a note. So I wrote: \"Feeling better and going to Birch Creek. Back soon. R. ", "3:15", ".\" That was about four hours from now.", "Raymond Carver", "Where I'm Calling From"],
&["July 3: 5 3/4 hours. Little done today. Deepening lethargy, dragged myself over to the lab, nearly left the road twice. Concentrated enough to feed the zoo and get the log up to date. Read through the operating manuals Whitby left for the last time, decided on a delivery rate of 40 rontgens/min., target distance of 530 cm. Everything is ready now. Woke 11:05. To sleep ", "3:15", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert(
        "15:16",
        &[&[
            "The Nimrod rendezvoused with the light aircraft at ",
            "1516",
            " GMT.",
            "Iain Banks",
            "The Crow Road",
        ]],
    );
    minutes.insert(
        "15:17",
        &[&[
            "The Nimrod rendezvoused with the light aircraft at ",
            "1516",
            " GMT.",
            "Iain Banks",
            "The Crow Road",
        ]],
    );
    minutes.insert(
        "15:18",
        &[&[
            "The Nimrod rendezvoused with the light aircraft at ",
            "1516",
            " GMT.",
            "Iain Banks",
            "The Crow Road",
        ]],
    );
    minutes.insert(
        "15:19",
        &[&[
            "The Nimrod rendezvoused with the light aircraft at ",
            "1516",
            " GMT.",
            "Iain Banks",
            "The Crow Road",
        ]],
    );
    minutes.insert("15:20", &[
&["At ", "twenty minutes past three", " on Monday, 26 January 1948, in Tokyo, and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking …", "David Peace", "Occupied City"],
]);
    minutes.insert("15:21", &[
&["At ", "twenty minutes past three", " on Monday, 26 January 1948, in Tokyo, and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking …", "David Peace", "Occupied City"],
]);
    minutes.insert("15:22", &[
&["At ", "twenty minutes past three", " on Monday, 26 January 1948, in Tokyo, and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking and I am drinking …", "David Peace", "Occupied City"],
]);
    minutes.insert("15:23", &[
&["", "Three twenty-three", "! Is that all? Doesn't time - no, I've already said that, thought that. I sit and watch the seconds change on the watch. I used to have a limited edition Rolex worth the price of a new car but I lost it.", "Iain Banks", "Espedair Street"],
&["", "Three twenty-three", "! Is that all? Doesn't time - no, I've already said that, thought that. I sit and watch the seconds change on the watch. I used to have a limited edition Rolex worth the price of a new car but I lost it. It was present from...Christine? No, Inez. She got fed up with me always having to ask other people what the time was; embarrassed on my behalf.", "Iain Banks", "Espedair Street"],
]);
    minutes.insert("15:24", &[
&["", "Three twenty-three", "! Is that all? Doesn't time - no, I've already said that, thought that. I sit and watch the seconds change on the watch. I used to have a limited edition Rolex worth the price of a new car but I lost it.", "Iain Banks", "Espedair Street"],
&["", "Three twenty-three", "! Is that all? Doesn't time - no, I've already said that, thought that. I sit and watch the seconds change on the watch. I used to have a limited edition Rolex worth the price of a new car but I lost it. It was present from...Christine? No, Inez. She got fed up with me always having to ask other people what the time was; embarrassed on my behalf.", "Iain Banks", "Espedair Street"],
]);
    minutes.insert("15:25", &[
&["\"Hmm, let's see. It's a three-line rail-fence, a, d, g...d-a-r-l...Got it: 'Darling Hepzibah'—Hepzibah? What kind of name is that?—'Will meet you Reading Sunday ", "15.25", " train Didcot-Reading.' Reading you all right, you idiots.\"", "Tom McCarthy", "C"],
]);
    minutes.insert("15:26", &[
&["\"Hmm, let's see. It's a three-line rail-fence, a, d, g...d-a-r-l...Got it: 'Darling Hepzibah'—Hepzibah? What kind of name is that?—'Will meet you Reading Sunday ", "15.25", " train Didcot-Reading.' Reading you all right, you idiots.\"", "Tom McCarthy", "C"],
]);
    minutes.insert(
        "15:27",
        &[&[
            "And she rang the Reverend Peters and he came into school at ",
            "3.27pm",
            " and he said, 'So, young man, are we ready to roll?'",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert(
        "15:28",
        &[&[
            "And she rang the Reverend Peters and he came into school at ",
            "3.27pm",
            " and he said, 'So, young man, are we ready to roll?'",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert("15:29", &[
&["\"Good heavens!\" she said, \"it's ", "nearly half-past three", ". I must fly. Don't forget about the funeral service,\" she added, as she put on her coat. \"The tapers, the black coffin in the middle of the aisle, the nuns in their white-winged coifs, the gloomy chanting, and the poor cowering creature without any teeth, her face all caved in like an old woman's, wondering whether she wasn't really and in fact dead - wondering whether she wasn't already in hell. Goodbye.\"", "Aldous Huxley", "Nuns at Luncheon"],
]);
    minutes.insert("15:30", &[
&["\"Before I am rrroasting the alarm-clock, I am setting it to go off, not at nine o'clock the next morning, but at ", "half-past thrrree", " the next afternoon. Vhich means half-past thrrree this afternoon. And that\", she said, glancing at her wrist-watch, \"is in prrree-cisely seven minutes' time!\"", "Roald Dahl", "The Witches"],
&["", "3.30 p.m.", " Catch school bus home", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["I must have completed my packing with time to spare, for when the knock came on my door at ", "half past three", " precisely, I had been sitting in my chair waiting for a good while. I opened the door to a young Chinese man, perhaps not even twenty, dressed in a gown, his hat in his hand.", "Kazuo Ishiguro", "When We Were Orphans"],
]);
    minutes.insert("15:31", &[
&["\"Before I am rrroasting the alarm-clock, I am setting it to go off, not at nine o'clock the next morning, but at ", "half-past thrrree", " the next afternoon. Vhich means half-past thrrree this afternoon. And that\", she said, glancing at her wrist-watch, \"is in prrree-cisely seven minutes' time!\"", "Roald Dahl", "The Witches"],
&["", "3.30 p.m.", " Catch school bus home", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["I must have completed my packing with time to spare, for when the knock came on my door at ", "half past three", " precisely, I had been sitting in my chair waiting for a good while. I opened the door to a young Chinese man, perhaps not even twenty, dressed in a gown, his hat in his hand.", "Kazuo Ishiguro", "When We Were Orphans"],
]);
    minutes.insert("15:32", &[
&["At ", "3:32", " precisely, I noticed Kaitlyn striding confidently past the Wok House. She saw me the moment I raised my hand, flashed her very white and newly straightened teeth at me, and headed over.", "John Green", "The Fault in Our Stars"],
]);
    minutes.insert(
        "15:33",
        &[&[
            "I picked up my briefcase, glancing at my watch again as I did so. ",
            "Three thirty-three",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert(
        "15:34",
        &[&[
            "I picked up my briefcase, glancing at my watch again as I did so. ",
            "Three thirty-three",
            ".",
            "Stephen King",
            "11/22/63",
        ]],
    );
    minutes.insert("15:35", &[
&["By ", "three-thirty-five", " business really winds down. I have already sold my ladderback chairs and my Scottish cardigans. I'm not even sure now why I've sold all these things, except perhaps so as not to be left out of this giant insult to one's life that is a yard sale, this general project of getting rid quick.", "Lorrie Moore", "Anagrams"],
&["If Me flashed a little crazy after a restless night of smoking & prowling the darkened house with owl-eyes alert to suspicious noises outside & on the roof, it didn’t inevitably mean she’d still be in such a state when the schoolbus deposited Wolfie back home at ", "3:35 P.M.", "", "Joyce Carol Oates", "I Am No One You Know: Stories"],
]);
    minutes.insert("15:36", &[
&["By ", "three-thirty-five", " business really winds down. I have already sold my ladderback chairs and my Scottish cardigans. I'm not even sure now why I've sold all these things, except perhaps so as not to be left out of this giant insult to one's life that is a yard sale, this general project of getting rid quick.", "Lorrie Moore", "Anagrams"],
&["If Me flashed a little crazy after a restless night of smoking & prowling the darkened house with owl-eyes alert to suspicious noises outside & on the roof, it didn’t inevitably mean she’d still be in such a state when the schoolbus deposited Wolfie back home at ", "3:35 P.M.", "", "Joyce Carol Oates", "I Am No One You Know: Stories"],
]);
    minutes.insert("15:37", &[
&["The explosion was now officially designated an \"Act of God\". But, thought Dirk, what god? And why? What god would be hanging around Terminal Two of Heathrow Airport trying to catch the ", "15.37", " flight to Oslo?", "Douglas Adams", "The Long Dark Tea Time of the Soul"],
]);
    minutes.insert("15:38", &[
&["The explosion was now officially designated an \"Act of God\". But, thought Dirk, what god? And why? What god would be hanging around Terminal Two of Heathrow Airport trying to catch the ", "15.37", " flight to Oslo?", "Douglas Adams", "The Long Dark Tea Time of the Soul"],
]);
    minutes.insert("15:39", &[
&["I lived two lives in late 1965 and early 1963, one in Dallas and one in Jodie. They came together at ", "three thirty-nine", " in the afternoon of April 10.", "Stephen King", "11/22/63"],
]);
    minutes.insert("15:40", &[
&["At ", "three-forty", ", Cliff called to report that Dilworth and his lady friend were sitting on the deck of the Amazing Grace, eating fruit and sipping wine, reminiscing a lot, laughing a little. “From what we can pick up with directional microphones and from what we can see, I’d say they don’t have any intention of going anywhere. Except maybe to bed. They sure do seem to be a randy old pair.” “Stay with them,” Lem said. “I don’t trust him.”", "Dean Koontz", "Watchers"],
]);
    minutes.insert("15:41", &[
&["At ", "15:41", " GMT, the Cessna's engine began to cut out and the plane - presumably out of fuel - began to lose altitude", "Iain Banks", "The Crow Road"],
]);
    minutes.insert("15:42", &[
&["At ", "15:41", " GMT, the Cessna's engine began to cut out and the plane - presumably out of fuel - began to lose altitude", "Iain Banks", "The Crow Road"],
]);
    minutes.insert("15:43", &[
&["At ", "15:41", " GMT, the Cessna's engine began to cut out and the plane - presumably out of fuel - began to lose altitude", "Iain Banks", "The Crow Road"],
]);
    minutes.insert("15:44", &[
&["The armed response team hastily assembled from Strängnäs arrived at Bjurman's summer cabin at ", "3.44 p.m.", "", "Stieg Larsson", "The Girl who Played with Fire"],
]);
    minutes.insert("15:45", &[
&["I opened my notebook, flipped almost to the end before I found a blank page, and wrote \"October 5th, ", "3.45pm", ", Dunning to Longview Cem, puts flowers on parents' (?) graves. Rain.\" I had what I wanted.", "Stephen King", "11/22/63"],
&["One meal is enough now, topped up with a glucose shot. Sleep is still 'black', completely unrefreshing. Last night I took a 16 mm. film of the first three hours, screened it this morning at the lab. The first true-horror movie. I looked like a half-animated corpse. Woke 10:25. To sleep ", "3:45", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("15:46", &[
&["I opened my notebook, flipped almost to the end before I found a blank page, and wrote \"October 5th, ", "3.45pm", ", Dunning to Longview Cem, puts flowers on parents' (?) graves. Rain.\" I had what I wanted.", "Stephen King", "11/22/63"],
&["One meal is enough now, topped up with a glucose shot. Sleep is still 'black', completely unrefreshing. Last night I took a 16 mm. film of the first three hours, screened it this morning at the lab. The first true-horror movie. I looked like a half-animated corpse. Woke 10:25. To sleep ", "3:45", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("15:47", &[
&["I opened my notebook, flipped almost to the end before I found a blank page, and wrote \"October 5th, ", "3.45pm", ", Dunning to Longview Cem, puts flowers on parents' (?) graves. Rain.\" I had what I wanted.", "Stephen King", "11/22/63"],
&["One meal is enough now, topped up with a glucose shot. Sleep is still 'black', completely unrefreshing. Last night I took a 16 mm. film of the first three hours, screened it this morning at the lab. The first true-horror movie. I looked like a half-animated corpse. Woke 10:25. To sleep ", "3:45", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("15:48", &[
&["I opened my notebook, flipped almost to the end before I found a blank page, and wrote \"October 5th, ", "3.45pm", ", Dunning to Longview Cem, puts flowers on parents' (?) graves. Rain.\" I had what I wanted.", "Stephen King", "11/22/63"],
&["One meal is enough now, topped up with a glucose shot. Sleep is still 'black', completely unrefreshing. Last night I took a 16 mm. film of the first three hours, screened it this morning at the lab. The first true-horror movie. I looked like a half-animated corpse. Woke 10:25. To sleep ", "3:45", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("15:49", &[
&["", "3.49 p.m.", " Get off school bus at home", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["But there were more bad things than good things. And one of them was that Mother didn't get back from work til 5.30 pm so I had to go to Father's house between ", "3.49 pm", " and 5.30 pm because I wasn't allowed to be on my own and Mother said I didn't have a choice so I pushed the bed against the door in case Father tried to come in.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert(
        "15:50",
        &[&[
            "",
            "3.50 p.m.",
            " Have juice and snack",
            "Mark Haddon",
            "The Curious Incident Of The Dog In The Night-Time",
        ]],
    );
    minutes.insert("15:51", &[
&["Date of the telegram, Rome, November 24, ten minutes before twenty-three o'clock. The telegram seems to say, \"The Sovereigns and the Royal Children expect themselves at Rome tomorrow at ", "fifty-one minutes after fifteen o'clock", ".\"", "Mark Twain", "Italian Without a Master"],
]);
    minutes.insert("15:52", &[
&["Date of the telegram, Rome, November 24, ten minutes before twenty-three o'clock. The telegram seems to say, \"The Sovereigns and the Royal Children expect themselves at Rome tomorrow at ", "fifty-one minutes after fifteen o'clock", ".\"", "Mark Twain", "Italian Without a Master"],
]);
    minutes.insert("15:53", &[
&["It was like the clouds lifting away from the sun. Jodie glanced at Reacher. He glanced at the clock. ", "Seven minutes to four", ". Less than three hours to go.", "Lee Child", "Tripwire"],
]);
    minutes.insert("15:54", &[
&["It was like the clouds lifting away from the sun. Jodie glanced at Reacher. He glanced at the clock. ", "Seven minutes to four", ". Less than three hours to go.", "Lee Child", "Tripwire"],
]);
    minutes.insert(
        "15:55",
        &[&[
            "",
            "3.55 p.m.",
            " Give Toby food and water",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-time",
        ]],
    );
    minutes.insert(
        "15:56",
        &[&[
            "",
            "Four minutes to four",
            ". Newman sighed again, lost in thought.",
            "Lee Child",
            "Tripwire",
        ]],
    );
    minutes.insert("15:57", &[
&["It was ", "close upon four", " before the door opened, and a drunken-looking groom, ill-kempt and side-whiskered with an inflamed face and disreputable clothes, walked into the room. Accustomed as I was to my friend's amazing powers in the use of disguises, I had to look three times before I was certain that it was indeed he.", "Arthur Conan Doyle", "The Adventures of Sherlock Holmes"],
]);
    minutes.insert("15:58", &[
&["", "Towards four o'clock", " the condition of the English army was serious. The Prince of Orange was in command of the centre, Hill of the right wing, Picton of the left wing. The Prince of Orange, desperate and intrepid, shouted to the Hollando-Belgians: \"Nassau! Brunswick! Never retreat!\"", "Victor Hugo", "Les Miserables"],
]);
    minutes.insert("15:59", &[
&["He looked at his watch: it was ", "nearly 4", ". He helped Delphine to her feet and led her down a passage to a rear door that gave on to the hospital garden.", "William Boyd", "The Blue Afternoon"],
]);
    minutes.insert("16:00", &[
&["... when they all sat down to table at ", "four o'clock", ", about three hours after his arrival, he had secured his lady, engaged her mother's consent, and was not only in the rapturous profession of the lover, but, in the reality of reason and truth, one of the happiest of men.", "Jane Austen", "Sense and Sensibility"],
&["\"What else can I answer, When the lights come on ", "at four", " At the end of another year\"", "Philip Larkin", "“Toads Revisited” - The Whitsun Weddings"],
&["", "4.00 p.m.", " Take Toby out of his cage", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["As he turned off towards the fishing village of Cellardyke, the familiar pips announced the ", "four o'clock", " news. The comforting voice of the newsreader began the bulletin. 'The convicted serial killer and former TV chat show host Jacko Vance has begun his appeal against conviction.", "Val McDermid", "The Last Temptation"],
&["Charmian woke ", "at four", " and sensed the emptiness of the house.", "Muriel Spark", "Memento Mori"],
&["Djerzinski arrived punctually ", "at four", " o’clock. Desplechin had asked to see him. The case was intriguing. Certainly, it was common for a researcher to take a year’s sabbatical to work in Norway or Japan, or one of those sinister countries where middle aged people committed suicide en masse.", "Michel Houellebecq", "Atomised"],
&["", "Four o'clock", ": wedge-shaped gardens lie Under a cavernous, a wind-picked sky.", "Philip Larkin", "Sad Steps"],
&["", "Four o'clock", ": when time in the city quivers on its axis - the day not yet spent, the wheels of evening just beginning to turn. The handover hour, was how Marius liked to think of it.", "Howard Jacobson", "The Act of Love"],
&["", "Four o’clock", " has just struck. Good! Arrangement, revision, reading from four to five. Short snooze of restoration for myself, from five to six. Affair of agent and sealed letter from seven to eight. At eight, en route.", "Walter Hartwright VII", "The Woman in White - The Story Continued"],
&["", "Four o’clock", " has just struck. Good! Arrangement, revision, reading from four to five. Short snooze of restoration for myself, from five to six. Affair of agent and sealed letter from seven to eight. At eight, en route.", "Wilkie Collins", "The Woman in White - The Story Continued"],
&["He played for twenty-two days, just as he said he would. Every day at ", "four o'clock", " in the afternoon, regardless of how much fighting was going on around him.", "Steven Galloway", "The Cellist Of Sarajevo"],
&["Her eyes caught the kryptonite glow of the digital clock on the front of the microwave. Honest and true, the numbers spelled out the time although she, for a moment, found its calculation to be somehow erroneous. It was ", "four o’clock", " in the afternoon.", "Susan May Gudge", "Blood Bride"],
&["I doubt whether anyone was commissioned to send the news along the actual telegraph, and yet Mrs. Proudie knew it before ", "four o'clock", ". But she did not know it quite accurately.'Bishop', she said, standing at her husband's study door. 'They have committed that man to gaol. There was no help for them unless they had forsworn themselves.'", "Anthony Trollope", "The Last Chronicle of Barset"],
&["I only found out much later that those flowers were called ", "Four O’clock", ", and were not magic at all. The magic was in the seed, waiting to be watered and cared for, the real magic was life.", "Susan May Gudge", "Ghost Generations"],
&["In the end, it was the Sunday afternoons he couldn’t cope with, and that terrible listlessness that starts to set in about 2.55, when you know you’ve had all the baths you can usefully have that day, that however hard you stare at any given paragraph in the newspaper you will never actually read it, or use the revolutionary new pruning technique it describes, and that as you stare at the clock the hands will move relentlessly on to ", "four o’clock", ", and you will enter the long dark teatime of the soul.", "Douglas Adams", "Life, the universe and everything"],
&["In the four thousand rooms of the Centre the four thousand electric clocks simultaneously ", "struck four", ". Discarnate voices called from the trumpet mouths. \"Main Day-shift off duty. Second Day-shift take over. Main Day-shift off …\"", "Aldous Huxley", "Brave New World"],
&["It was my turn to cook the evening meal so I didn't linger in the common room. It was exactly ", "4 o'clock", " as I made my way out of the building, and doors opened behind and before me, discharging salvos of vocal babble and the noise of chair-legs scraping on wooden floors.", "David Lodge", "Deaf Sentence"],
&["Miss Douce took Boylan's coin, struck boldly the cashregister. It clanged. Clock clacked. Fair one of Egypt teased and sorted in the till and hummed and handed coins in change. Look to the west. A clack. For me. —What time is that? asked Blazes Boylan. ", "Four", "? O'clock. Lenehan, small eyes ahunger on her humming, bust ahumming, tugged Blazes Boylan's elbowsleeve. —Let's hear the time, he said.", "James Joyce", "Ulysses"],
&["The horrifying R.N. wipes Gately's face off as best she can with her hand and says she'll try to fit him in for a sponge bath before she goes off shift at ", "1600h.", ", at which Gately goes rigid with dread.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("16:01", &[
&["Light is coming in through the curtains. Suddenly the digits on the clock radio look like a year. ", "1601", ". I woke up a bit early, don't have to be born for another 400 years.", "Hallgrímur Helgason", "101 Reykjavik"],
]);
    minutes.insert("16:02", &[
&["I'd just looked up at the clock, to make sure time wasn't getting away from me, when I heard the shot. It was ", "two minutes after four", ". I didn't know what to do.", "Carolyn G Hart", "Southern Ghost"],
]);
    minutes.insert(
        "16:03",
        &[&[
            "She read the page carefully and then said, '",
            "16.03",
            " - cat goes to the toilet in front garden.'",
            "Catherine O'Flynn",
            "What was Lost",
        ]],
    );
    minutes.insert("16:04", &[
&["", "A little after four o'clock", ", Pippa meandered over to Dot's house carrying a bottle of wine she had been keeping in reserve and wondering if she could possibly be pregnant in spite of the vestigial coil still lodged in her uterus like astronaut litter abandoned on the moon.", "Rebecca Miller", "The Private Lives of Pippa Lee"],
]);
    minutes.insert("16:05", &[
&["I had met Irwin on the steps of the Widener Library. I was standing at the top of the long flight, overlooking the red brick buildings that walled the snow-filled quad and preparing to catch the trolley back to the asylum, when a tall young man with a rather ugly and bespectacled, but intelligent face, came up and said, 'Could you please tell me the time?' I glanced at my watch. '", "Five past four", ".'", "Sylvia Plath", "The Bell Jar"],
&["I was standing at the top of the long flight, overlooking the red brick buildings that walled the snow-filled quad and preparing to catch the trolley back to the asylum, when a tall young man with a rather ugly and bespectacled, but intelligent face, came up and said, 'Could you please tell me the time?' I glanced at my watch. '", "Five past four", ".'", "Sylvia Plath", "The Bell Jar"],
&["IT was exactly ", "five minutes past four", " as Mr. Robert Audley stepped out upon the platform at Shoreditch, and waited placidly … it took a long while to make matters agreeable to all claimants, and even the barrister's seraphic indifference to mundane affairs nearly gave way.", "Mary Elizabeth Braddon", "Lady Audley's Secret"],
]);
    minutes.insert("16:06", &[
&["At ", "six minutes after four", ", Benny's Cadillac pulled up in front of Mr. Botelia's store, and Benny's mother stepped out of the car with Penelope, who was gnawing on the tip of an ice cream cone.", "Joanna Scott", "Follow Me: A Novel"],
]);
    minutes.insert("16:07", &[
&["But he released him immediately because the ladder slipped from under his feet and for an instant he was suspended in air and then he realised that he had died without Communion, without time to repent of anything or to say goodbye to anyone, at ", "seven minutes after four", " on Pentecost Sunday.", "Gabriel García Márquez", "Love in the Time of Cholera"],
]);
    minutes.insert(
        "16:08",
        &[&[
            "It was ",
            "eight minutes after four",
            ". I still don't have a plan. Maybe the guys in the Nova, maybe they had a plan.",
            "Robert Crais",
            "The Monkey's Raincoat",
        ]],
    );
    minutes.insert("16:09", &[
&["I have to hang up now, Rosemary said. \"I just wanted to know if there was any improvement.\" \"No, there isn't. It was nice of you to call.\" She hung up. It was ", "nine minutes after four", ".", "Ira Levin", "Rosemary's Baby"],
]);
    minutes.insert("16:10", &[
&["", "1610h.", " E.T.A Weight room. Freestyle circuits. The clank and click of various resistance systems.", "David Foster Wallace", "Infinite Jest"],
&["She looks at the clock. She's in the kitchen. A minute left. She waits. It's ", "ten-past four", ". She picks up the eclair. She licks the cream out of it. She watches herself.It's fuckin' stupid. But. She bites into the chocolate, and the pastry that's been softened by the cream. Jack's not home yet. Leannes's at work. Paula will be leaving, herself, in a bit. She's a year off the drink. Exactly a year. She looks at the clock. A year and a minute.", "Roddy Doyle", "Paula Spencer"],
]);
    minutes.insert("16:11", &[
&["", "4:11 P.M.", " Thurs. A Huey helicopter flies east overhead as the last of the U.S. Marines make ready to leave the beach; a buzzard dangles in the thermals closer over the town.", "Denis Johnson", "Seek"],
]);
    minutes.insert("16:12", &[
&["At precisely ", "twelve minutes after four", " a body of cavalry rode into the square, four abreast, clearing a way for the funeral cortege.", "J Sydney Jones", "The Empty Mirror"],
]);
    minutes.insert("16:13", &[
&["But at precisely ", "4.13pm", ", the fifty thousand spectators saw the totally unexpected happen, before their very eyes. From the most crowded section of the southern grandstand, an apparition suddenly emerged.", "Mario Vargas Llosa", "Aunt Julia and the Scriptwriter"],
]);
    minutes.insert("16:14", &[
&["Then at ", "4.14pm", " on March 12 I moved behind zinc-zirconium-not-to-be-revealed-compounds protecting me in this hill, and God have mercy but the struggle is just exchanged for the next one, which is exhausting me further as I say, to separate the true from the false.", "Denis Johnson", "Already Dead"],
]);
    minutes.insert("16:15", &[
&["I remember the dread with which I at ", "quarter past four", "/ Let go with a bang behind me our house front door", "John Betjeman", "False Security"],
&["It is only a ", "quarter past four", ", (shewing his watch) and you are not now in Bath. No theatre, no rooms to prepare for. Half an hour at Northanger must be enough.", "Jane Austen", "Northanger Abbey"],
&["Must have the phone disconnected. Some contractor keeps calling me up about payment for 50 bags of cement he claims I collected ten days ago. Says he helped me load them onto a truck himself. I did drive Whitby's pick-up into town but only to get some lead screening. What does he think I'd do with all that cement? Just the sort of irritating thing you don't expect to hang over your final exit. (Moral: don't try too hard to forget Eniwetok.) Woke 9:40. To sleep ", "4:15", ".", "JG Ballard", "The Voices of Time"],
&["On the tenth day of October at ", "quarter past four", " in the afternoon with a dry hot wind blowing through the passed Maria found herself in Baker. She had never meant to go as far as Baker, had started out that day as every day, her only destination the freeway. But she had driven out of San Bernadino and up the Barstow and instead of turning back at Barstow (she had been out that far before but never that late in the day, it was past time to navigate back, she was out too far too late, the rhythm was lost ) she kept driving.", "Joan Didion", "Play it as it Lays"],
&["The sun had begun to sink in the west, and the shadow of an oak branch had crept across my knees. My watch said it was ", "4.15", ".", "Haruki Murakami", "The Wind-up Bird Chronicle"],
]);
    minutes.insert("16:16", &[
&["", "4.16pm", " The terrace outside the bar is packed, and Igor feels proud of his ability to plan things, because even though he's never been to Cannes before, he had foreseen precisely this situation and reserved a table.", "Paulo Coelho", "The Winner Stands Alone"],
]);
    minutes.insert("16:17", &[
&["Apparently the great Percy has no sense of humour, for at ", "four-seventeen", " he got tired of it, and hit Skinner crisply in the right eyeball, blacking the same as per illustration.", "P.G. Wodehouse", "A Prefect's Uncle"],
&["In the next instant she was running toward her house, unmindful of the bags she had dropped, seeing only the police cars, knowing as she glanced down at her watch and saw that it was ", "seventeen minutes after four", ", that for her time had stopped.", "Joy Fielding", "Life Penalty"],
]);
    minutes.insert(
        "16:18",
        &[&[
            "",
            "4.18 p.m.",
            " Put Toby into his cage",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-time",
        ]],
    );
    minutes.insert("16:19", &[
&["Jessica [", "4:19 PM", "] Don't tease me like that. I haven't been to a play in years. Charles [4:19 PM] Then it'll be my treat. You and the hubby can have big fun on me.", "Eric Jerome Dickey", "The Other Woman"],
]);
    minutes.insert("16:20", &[
&["", "4.20 p.m.", " Watch television or a video", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "twenty minutes past four", " - or, to put it another, blunter way, an hour and twenty minutes past what seemed to be all reasonable hope - the unmarried bride, her head down, a parent stationed on either side of her, was helped out of the building...", "J. D. Salinger", "Raise High the Roof Beam, Carpenters"],
]);
    minutes.insert("16:21", &[
&["", "4.21pm", " As they started on, Doug picked up a twig and after rubbing it off, started to move one end of it inside his mouth. \"What are you doing?\" Bob asked. \"Brushing my teeth, nature style,\" Doug answered. Bob grunted, smiling slightly. \"I'll use my toothbrush,\" he said.", "Richard Matheson", "Hunted Past Reason"],
]);
    minutes.insert("16:22", &[
&["Monday, ", "4.22pm", " Washington, D.C. Paul Hood took his daily late-afternoon look at the list of names on his computer monitor.", "Tom Clancy, Steve Pieczenik, and Jeff Rovin", "Balance of Power: Op-Center 05"],
]);
    minutes.insert("16:23", &[
&["They were hurrying west, trying to reach the river before sunset. The warming-related 'adjustments' to Earth's orbit had shortened the winter days, so that now, in January, sunset was taking place at ", "4.23", ".", "Jennifer Egan", "A Visit from the Goon Squad"],
]);
    minutes.insert("16:24", &[
&["Mike winked at Ashley and continued with the remaining greetings and hugs and handshakes. The time was ", "4:24", ". Six hours to go. The minutes seemed to just melt away.", "Travis Thrasher", "Teardrop"],
]);
    minutes.insert("16:25", &[
&["As I dressed I glanced at my watch. It was no wonder that no one was stirring. It was ", "twenty-five minutes past four", ". I had hardly finished when Holmes returned with the news that the boy was putting in the horse.", "Arthur Conan Doyle", "The Man with the Twisted Lip"],
]);
    minutes.insert("16:26", &[
&["It seemed all wrong to have thought of such a thing. She thought, \"I don't know him. Nor does he know me. Nor ever shall we.” She put her bare hand in the sun, where the wind would weather it. It was ", "twenty-six minutes after four", ".", "Charlotte Armstrong", "Catch-As-Catch-Can"],
]);
    minutes.insert("16:27", &[
&["It seemed all wrong to have thought of such a thing. She thought, \"I don't know him. Nor does he know me. Nor ever shall we.” She put her bare hand in the sun, where the wind would weather it. It was ", "twenty-six minutes after four", ".", "Charlotte Armstrong", "Catch-As-Catch-Can"],
]);
    minutes.insert("16:28", &[
&["Same day: ", "4.28pm", "- Right turn at the second bus stop after the gas station. I stopped the car at the first ward post office and inquired at the corner tobacconists. Mr. M's house was the one to the right of the post office, visible diagonally in front of me.", "Kobo Abe", "The Ruined Map: A Novel"],
]);
    minutes.insert("16:29", &[
&["October 21, 2007, ", "4:29 pm.", " The phone was red. And what William hated most about it, besides the fact that it was inconveniently mounted on a wall in a tight corner (and at a strange angle), was that when it rang it was so gratingly loud that you could actually see the cherry receiver quavering as you picked it up.", "Mark Lavorato", "Believing Cedric"],
]);
    minutes.insert("16:30", &[
&["At ", "four-thirty", " that afternoon in late January when I stepped into the parlour with Boo, my dog, Hutch was in his favourite armchair, scowling at the television, which he had muted.", "Dean Koontz", "Odd Hours"],
&["I leave the office at ", "four thirty", ", head up to Xclusive where I work out on free weights for an hour, then taxi across the park to Gio's in the Pierre Room for a facial, a manicure and, if time permits, a pedicure.", "Bret Easton Ellis", "American Psycho"],
&["She hung up on me at first, then asked me whether I made a point of behaving like a 'small-time suburban punk' with women I had slept with. But after apologies, insults, laughter, and tears, Romeo and Juliet were to be seen together later that afternoon, mushily holding hands in the dark at a ", "four-thirty", " screening of L ove and Death at the National Film Theatre. Happy endings – for now at least.", "Alain de Botton", "Essays in Love"],
]);
    minutes.insert("16:31", &[
&["From: Renee Greene – August 5, 2011 – ", "4:31 PM", " To: Shelley Manning Subject: Re: All Access What should I be worried about, then? JUST KIDDING. You're right. Well, I gotta run, my groupie friend. I actually have REAL work to do. I'll talk to you tonight.", "Lisa Becker", "Click: An Online Love Story"],
]);
    minutes.insert("16:32", &[
&["", "4.32pm.", " Now the eight Marines next to us leave their emplacement and file quickly past, the last saying, \"Go! Go! Go!\" They break into a run.", "Denis Johnson", "Seek"],
]);
    minutes.insert("16:33", &[
&["At ", "4.33pm", ", a short bald man puffing on a cigar arrived at the library. He approached a huge cabinet storing thousands of alphabetically arranged cards and slid a drawer out. The tips of his fingers were bandaged.", "José Latour", "Havana World Series"],
]);
    minutes.insert("16:34", &[
&["A bedroom stocked with all the ordinary, usual things. There was a wardrobe in the corner. A bedside table with a collection of water glasses of varying ages and an alarm clock with red digital numbers- ", "4.34 p.m.", "", "Steven Hall", "The Raw Shark Texts"],
]);
    minutes.insert("16:35", &[
&["The Voice shut itself off with a click, and then reopened conversation by announcing the arrival at Platform 9 of the ", "4.35", " from Birmingham and Wolverhampton.", "Agatha Christie", "4.50 from Paddington"],
]);
    minutes.insert("16:36", &[
&["The Voice shut itself off with a click, and then reopened conversation by announcing the arrival at Platform 9 of the ", "4.35", " from Birmingham and Wolverhampton.", "Agatha Christie", "4.50 from Paddington"],
]);
    minutes.insert("16:37", &[
&["She should have been home by now. ", "1637.", " Yes. It's as if I had the date of a year on my arm. Every day is a piece of world history.", "Hallgrímur Helgason", "101 Reykjavik"],
]);
    minutes.insert("16:38", &[
&["She should have been home by now. ", "1637.", " Yes. It's as if I had the date of a year on my arm. Every day is a piece of world history.", "Hallgrímur Helgason", "101 Reykjavik"],
]);
    minutes.insert("16:39", &[
&["Harlem enjoys lazy Sabbath mornings, although the pace picks up again in the afternoon, after church. My watch read ", "4:39 p.m.", ", and I realized that I hadn't eaten all day. I bought two slices of pizza from a sidewalk vendor on 122nd and Lenox Avenue and washed it down with a grape Snapple.", "Teddy Hayes", "Blood Red Blues"],
]);
    minutes.insert("16:40", &[
&["", "Four forty", " P.M. Besta sang another hymn. Everyone knew something was wrong. How long did they wait? The mayor was going crazy inside, as was the mayor's wife, as was their daughter. Seiji could barely contain his rage. He was turning as red as his red tuxedo.", "Tip \"T.I.\" Harris with David Ritz", "Trouble & Triumph: A Novel of Power & Beauty"],
]);
    minutes.insert("16:41", &[
&["", "Four forty", " P.M. Besta sang another hymn. Everyone knew something was wrong. How long did they wait? The mayor was going crazy inside, as was the mayor's wife, as was their daughter. Seiji could barely contain his rage. He was turning as red as his red tuxedo.", "Tip \"T.I.\" Harris with David Ritz", "Trouble & Triumph: A Novel of Power & Beauty"],
]);
    minutes.insert("16:42", &[
&["I'm always happy when I reach the finish line of a long-distance race, but this time it really struck me hard. I pumped my right fist into the air. The time was ", "4:42pm.", " Eleven hours and forty-two minutes since the start of the race.", "Haruki Murakami", "What I Talk About When I Talk About Running"],
]);
    minutes.insert("16:43", &[
&["I'm always happy when I reach the finish line of a long-distance race, but this time it really struck me hard. I pumped my right fist into the air. The time was ", "4:42pm.", " Eleven hours and forty-two minutes since the start of the race.", "Haruki Murakami", "What I Talk About When I Talk About Running"],
]);
    minutes.insert("16:44", &[
&["I'm always happy when I reach the finish line of a long-distance race, but this time it really struck me hard. I pumped my right fist into the air. The time was ", "4:42pm.", " Eleven hours and forty-two minutes since the start of the race.", "Haruki Murakami", "What I Talk About When I Talk About Running"],
]);
    minutes.insert("16:45", &[
&["At ", "four-forty-five", " Miss Haddon went to tea with the Principal, who explained why she desired all the pupils to learn the same duet. It was part of her new co-ordinative system.", "EM Forster", "Co-ordination"],
&["The next day Bill took only ten minutes of the twenty-minute break allotted for the afternoon and left at ", "fifteen minutes before five", ". He parked the car in the lot just as Arlene hopped down from the bus.", "Raymond Carver", "Where I'm Calling From"],
]);
    minutes.insert("16:46", &[
&["At ", "4:46", " an obese, middle-aged man shuffled in. Wearing a starched guayabera and dark green pants, Ureña asked for a book on confectionery, then took a seat at the end of the same reading room. Evelina and Leticia exchanged astonished glances. It definitely was one of those days.", "José Latour", "The Havana World Series"],
]);
    minutes.insert("16:47", &[
&["But maybe it was more than that, maybe Affenlight had erred badly somehow, because here it was 4:49 by his watch, ", "4:47", " by the wall clock, and Owen had not yet come.", "Chad Harbach", "The Art of Fielding"],
]);
    minutes.insert("16:48", &[
&["Thinking about the card warms me to the idea of walking under the arched doorway of the Newtons' home, but when I arrive at their house, the plan seems ridiculous. What am I doing? It's ", "4:48 a.m.", ", and I'm parked outside their darkened house.", "Dave Eggers", "What is the What"],
]);
    minutes.insert("16:49", &[
&["", "4:49 p.m.", ", a bald-headed man wearing khakis and ankle-high deck shoes came out through the front door of the purple house on 21st Avenue East. The detectives had nicknamed him the General.", "Ridley Pearson", "Beyond Recognition"],
]);
    minutes.insert("16:50", &[
&["\"The train standing at Platform 3,\" the Voice told her, \"is the ", "4.50", " for Brackhampton, Milchester, Waverton, Carvil Junction, Roxeter and stations to Chadmouth. Passengers for Brackhampton and Milchester travel at the rear of the train. Passengers for Vanequay change at Roxeter.\" The voice shut itself off with a click,", "Agatha Christie", "4.50 from Paddington"],
&["They had all frozen at the same time, on a snowy night, seven years before, and after that it was always ", "ten minutes to five", " in the castle.", "James Thurber", "The 13 Clocks"],
&["When the clock said ", "ten minutes to five", ", she began to listen, and a few moments later, punctually as always, she heard the tires on the gravel outside, and the car door slamming, the footsteps passing the window, the key turning in the lock. She laid aside her sewing, stood up, and went forward to kiss him as he came in.", "Roald Dahl", "Lamb to the Slaughter"],
]);
    minutes.insert("16:51", &[
&["", "Nine minutes to five.", " If this wasn't some new ordeal, intended to fray her nerves to shreds, if this important person really did exist, if he'd actually set up this appointment, and if, moreover, he arrived on time, then there were nine minutes left.", "Norman Manea and Linda Coverdale", "Compulsory Happiness"],
]);
    minutes.insert("16:52", &[
&["The corrida was to begin at five o'clock. The five-footed beasts make a point of arriving at the latest at eight or seven minutes to: ritual again. At ", "eight minutes to five", ", there they were. The urchins gave them a tap on the shoulder: another bit of ritual.", "Henry De Montherlant", "Chaos and Night"],
]);
    minutes.insert("16:53", &[
&["It was so quiet in the post office that Trinidad could hear the soft tick of the clock's second hand every time it moved. It was now ", "seven minutes before five", ".", "Rita Leganski", "The Silence of Bonaventure Arrow"],
]);
    minutes.insert("16:54", &[
&["At ", "six minutes before five", " o'clock, Daisy Robinson, about to reach her own apartment door, paused to look and to listen. Something was out of order. Tess Rogan's door was standing wide open and, from within, Daisy could hear something being broken.", "Charlotte Armstrong", "The Seventeen Widows of Sans Souci"],
&["It was ", "1654", " local time when the Red October broke the surface of the Atlantic Ocean for the first time, forty-seven miles southeast of Norfolk. There was no other ship in sight.", "Tom Clancy", "The Hunt for Red October"],
]);
    minutes.insert("16:55", &[
&["About ", "five minutes to five", ", just as they were all putting their things away for the night, Nimrod suddenly appeared in the house. He had come hoping to find some of them ready dressed to go home before the proper time.", "Robert Tressell", "The Ragged Trousered Philanthropists"],
]);
    minutes.insert("16:56", &[
&["And when that final Friday came, when my packing was mostly done, she sat with my dad and me on the living-room couch at ", "4:56 P.M.", " and patiently awaited the arrival of the Good-bye to Miles Cavalry.", "John Green", "Looking for Alaska"],
]);
    minutes.insert("16:57", &[
&["It was ", "nearly five", " in the evening when the cook came aboard. He did not have the cabbages.", "John Hershey", "A Single Pebble"],
&["Then at ", "three minutes to five", " — Pendel had somehow never doubted that Osnard would be punctual — along comes a brown Ford hatchback with an Avis sticker on the back window and pulls into the space reserved for customers.", "John Le Carré", "The Tailor of Panama"],
]);
    minutes.insert("16:58", &[
&["I was told that in his vest pocket he kept a chronometer instead of a watch. If someone asked him what time it was, he would say, \"", "A minute and twenty-one seconds to five", ".\"", "Isaac Bashevis Singer", "The Collected Stories"],
]);
    minutes.insert("16:59", &[
&["The rain stopped ", "around 5 p.m.", " and a few of those people who were out and about expressed mild surprise when the rainbow failed to fade.", "Nicholas Royle", "Mortality -- 'The Rainbow'"],
]);
    minutes.insert("17:00", &[
&["", "5.00 p.m.", " Read a book", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["About ", "five", ", the Abbot, a young Manchester terrier, began chirruping. He stood on the body of his owner, Flora, with his forepaws on the sill of the balcony, stared through the green rattan blinds, and trembled. He could see the farmer in the field, and Edward asleep on the next balcony.", "Christina Stead", "The People With the Dogs"],
&["At ", "five o'clock", " that afternoon, while Barbara waited in a taxi, Harold went into the convent in Auteuil and explained to the nun who sat in the concierge's glass cage that Mme. Straus-Muguet was expecting them. He assumed that men were not permitted any further, and that they would all three go out for tea.", "William Maxwell", "The Chateau"],
&["At ", "five o’clock", " adieux were waved, and the ponderous liner edged away from the long pier, slowly turned its nose seaward, discarded its tug, and headed for the widening water spaces that led to old world wonders. By night the outer harbour was cleared, and late passengers watched the stars twinkling above an unpolluted ocean.", "H. P. Lovecraft", "The Horror at Red Hook"],
&["But I took the mixture at ", "five o'clock", " in the afternoon. I run my tongue over my dry mouth. I feel dizzy. I know this dizziness: it's because I haven't had a cigarette for hours.", "Scarlett Thomas", "The End of Mr Y"],
&["ERE THE HALF-HOUR ended, ", "five o'clock", " struck; school was dismissed, and all were gone into the refectory to tea. I now ventured to descend; it was deep dusk; I retired into a corner and sat down on the floor.", "Charlotte Brontë", "Jane Eyre"],
&["From ", "five o'clock", " to eight is on certain occasions a little eternity; but on such an occasion as this the interval could be only an eternity of pleasure.", "Henry James", "The Portrait of a Lady"],
&["He found it harder to concentrate on drills that afternoon and when he left the building at ", "five o'clock", ", he was still so worried that he walked straight into someone just outside the door.", "JK Rowling", "Harry Potter and the Philosopher's Stone"],
&["She had not seen her yet, as Osmond had given her to understand that it was too soon to begin. She drove at ", "five o'clock", " to a high floor in a narrow street in the quarter of the Piazza Navona, and was admitted by the portress of the convent, a genial and obsequious person. Isabel had been at this institution before; she had come with Pansy to see the sisters.", "Henry James", "The Portrait of a Lady"],
&["Until ", "five o'clock", " there was no sign of life from the room. Then he rang for his servant and ordered a cold bath.", "Sandor Marai", "Embers"],
&["We motored, I remember, leaving London in the morning in a heavy shower of rain, coming to Manderley about ", "five o'clock", ", in time for tea. I can see myself now, unsuitably dressed as usual, although a bride of seven weeks, in a tan-coloured stockinette frock, a small fur known as a stone marten round my neck, and over all a shapeless mackintosh, far too big for me and dragging to my ankles.", "Daphne du Maurier", "Rebecca"],
]);
    minutes.insert("17:01", &[
&["", "One minute after five.", " The seated guests were told that the ceremony would begin shortly. A little more patience was required.", "Tip \"T.I.\" Harris with David Ritz", "Trouble & Triumph: A Novel of Power & Beauty"],
]);
    minutes.insert("17:02", &[
&["She stood up, shook her hair into place, smoothed her skirt and turned on the light. It was ", "two minutes past five", ". She would have thought it midnight or five in the morning.", "Jane Smiley", "Duplicate Keys"],
]);
    minutes.insert("17:03", &[
&["\"Good evening, Mrs. Scheindlin,\" the man said before departing. \"Good evening, Chris. Say hello to the wife for me.\" \"I sure will. Thanks. Bye,\" he said, waving to Elliot, who returned the goodbye. It was ", "5:03", " when Elliot rested the handset in its cradle.", "José Latour", "Comrades in Miami: A Novel"],
]);
    minutes.insert("17:04", &[
&["Frank Wamsley spotted his cousin Barbara and her husband and waved to them. Just ahead, he saw Marvin and his two friends. Suddenly the whole bridge convulsed. The time was ", "5:04 P.M.", " Steel screamed.", "John A. Keel", "The Mothman Prophecies"],
]);
    minutes.insert("17:05", &[
&["At approximately ", "5:05 p.m.", " Joe became aware of a man standing close to the table, about two metres away, talking in Mandarin into a mobile phone. He was a middle-aged Han wearing cheap leather slip-on shoes, high-waisted black trousers and a white short-sleeved shirt.", "Charles Cumming", "Typhoon"],
]);
    minutes.insert("17:06", &[
&["The rain stopped ", "around 5 p.m.", " and a few of those people who were out and about expressed mild surprise when the rainbow failed to fade.", "Nicholas Royle", "Mortality -- 'The Rainbow'"],
]);
    minutes.insert("17:07", &[
&["The rain stopped ", "around 5 p.m.", " and a few of those people who were out and about expressed mild surprise when the rainbow failed to fade.", "Nicholas Royle", "Mortality -- 'The Rainbow'"],
]);
    minutes.insert("17:08", &[
&["The rain stopped ", "around 5 p.m.", " and a few of those people who were out and about expressed mild surprise when the rainbow failed to fade.", "Nicholas Royle", "Mortality -- 'The Rainbow'"],
]);
    minutes.insert("17:09", &[
&["The rain stopped ", "around 5 p.m.", " and a few of those people who were out and about expressed mild surprise when the rainbow failed to fade.", "Nicholas Royle", "Mortality -- 'The Rainbow'"],
]);
    minutes.insert("17:10", &[
&["", "Five ten P.M.", " A ground-to-ground cruise missile, launched from a tractor installed in the backyard of Leonard Sudavico's former home by Rashan and a crew of technicians from Afghanistan, exploded onto the Paul Clay estate in the exact spot where the life-size mermaid had once swum in the waterfall.", "Tip \"T.I.\" Harris with David Ritz", "Trouble & Triumph: A Novel of Power & Beauty"],
&["Hours later, at ", "ten minutes past five", ", Saturday afternoon, Nora and Travis and Jim Keene crowded in front of the mattress on which Einstein lay. The dog had just taken a few more ounces of water. He looked at them with interest, too. Travis tried to decide if those large brown eyes still had the strange depth, uncanny alertness, and undoglike awareness that he had seen in them so many times before.", "Dean Koontz", "Watchers"],
]);
    minutes.insert("17:11", &[
&["", "Five ten P.M.", " A ground-to-ground cruise missile, launched from a tractor installed in the backyard of Leonard Sudavico's former home by Rashan and a crew of technicians from Afghanistan, exploded onto the Paul Clay estate in the exact spot where the life-size mermaid had once swum in the waterfall.", "Tip \"T.I.\" Harris with David Ritz", "Trouble & Triumph: A Novel of Power & Beauty"],
&["Hours later, at ", "ten minutes past five", ", Saturday afternoon, Nora and Travis and Jim Keene crowded in front of the mattress on which Einstein lay. The dog had just taken a few more ounces of water. He looked at them with interest, too. Travis tried to decide if those large brown eyes still had the strange depth, uncanny alertness, and undoglike awareness that he had seen in them so many times before.", "Dean Koontz", "Watchers"],
]);
    minutes.insert("17:12", &[
&["\"Well, here we are,\" said Colonel Julyan, \"and it's exactly ", "twelve minutes past five", ". We shall catch them in the middle of their tea. Better wait for a bit\" Maxim lit a cigarette, and then stretched out his hand to me. He did not speak.", "Daphne du Maurier", "Rebecca"],
]);
    minutes.insert("17:13", &[
&["\"Well, here we are,\" said Colonel Julyan, \"and it's exactly ", "twelve minutes past five", ". We shall catch them in the middle of their tea. Better wait for a bit\" Maxim lit a cigarette, and then stretched out his hand to me. He did not speak.", "Daphne du Maurier", "Rebecca"],
]);
    minutes.insert(
        "17:14",
        &[&[
            "\"Do you know what time it is, Atticus?\" she said. \"Exactly ",
            "fourteen minutes past five",
            ". The alarm clock's set for five-thirty. I want you to know that.\"",
            "Harper Lee",
            "To Kill a Mockingbird",
        ]],
    );
    minutes.insert("17:15", &[
&["When August Bach emerged from the gloomy chill of the air-conditioned Divisional Fighter Control bunker it was ", "17:15 hrs", " CET. The day had ripened into one of those mellow summer afternoons when the air is warm and sweet like soft toffee", "Len Deighton", "Bomber"],
]);
    minutes.insert("17:16", &[
&["When August Bach emerged from the gloomy chill of the air-conditioned Divisional Fighter Control bunker it was ", "17:15 hrs", " CET. The day had ripened into one of those mellow summer afternoons when the air is warm and sweet like soft toffee", "Len Deighton", "Bomber"],
]);
    minutes.insert("17:17", &[
&["When August Bach emerged from the gloomy chill of the air-conditioned Divisional Fighter Control bunker it was ", "17:15 hrs", " CET. The day had ripened into one of those mellow summer afternoons when the air is warm and sweet like soft toffee", "Len Deighton", "Bomber"],
]);
    minutes.insert("17:18", &[
&["Lupin rose, without breaking his contemptuous silence, and took the sheet of paper. I remembered soon after that, at this moment, I happened to look at the clock. It was ", "eighteen minutes past five", ".", "Maurice LeBlanc", "The Confessions of Arsène Lupin"],
]);
    minutes.insert("17:19", &[
&["The call came at ", "5.19 p.m.", " The line was surprisingly clear. A man introduced himself as Major Liepa from the Riga police. Wallander made notes as he listened, occasionally answering a question.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("17:20", &[
&["The Meeting was listed as starting at 1730, and it was only around ", "1720", ", and Hal thought the voices might signify some sort of pre-Meeting orientation for people who've come for the first time, sort of tentatively, just to scout the whole enterprise out, so he doesn't knock.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("17:21", &[
&["The Meeting was listed as starting at 1730, and it was only ", "around 1720", ", and Hal thought the voices might signify some sort of pre-Meeting orientation for people who've come for the first time, sort of tentatively, just to scout the whole enterprise out, so he doesn't knock.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("17:22", &[
&["The Meeting was listed as starting at 1730, and it was only ", "around 1720", ", and Hal thought the voices might signify some sort of pre-Meeting orientation for people who've come for the first time, sort of tentatively, just to scout the whole enterprise out, so he doesn't knock.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("17:23", &[
&["\"I was wondering if we could meet for a drink.\" \"What for?\" \"Just for a chat. Do you know the Royal batsman, near Central Station? We could meet tomorrow at five?\" \"", "Five twenty-three", ",\" I said, to exert some control over the situation.", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("17:24", &[
&["\"I was wondering if we could meet for a drink.\" \"What for?\" \"Just for a chat. Do you know the Royal batsman, near Central Station? We could meet tomorrow at five?\" \"", "Five twenty-three", ",\" I said, to exert some control over the situation.", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("17:25", &[
&["It was ", "five-twenty-five", " when I pulled up in front of the library. Still early for our date, so I got out of the car and took a stroll down the misty streets. In a coffee shop, watched a golf match on television, then I went to an entertainment center and played a video game. The object of the game was to wipe out tanks invading from across the river. I was winning at first, but as the game went on, the enemy tanks bred like lemmings, crushing me by sheer number and destroying my base. An on-screen nuclear blast took care of everything, followed by the message game over insert coin.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Now said Handsley, when Angela had poured out the last cup, \"it's ", "twenty-five minutes past five", ", At half-past the Murder game is on\"", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("17:26", &[
&["It was ", "five-twenty-five", " when I pulled up in front of the library. Still early for our date, so I got out of the car and took a stroll down the misty streets. In a coffee shop, watched a golf match on television, then I went to an entertainment center and played a video game. The object of the game was to wipe out tanks invading from across the river. I was winning at first, but as the game went on, the enemy tanks bred like lemmings, crushing me by sheer number and destroying my base. An on-screen nuclear blast took care of everything, followed by the message game over insert coin.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Now said Handsley, when Angela had poured out the last cup, \"it's ", "twenty-five minutes past five", ", At half-past the Murder game is on\"", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("17:27", &[
&["It was ", "five-twenty-five", " when I pulled up in front of the library. Still early for our date, so I got out of the car and took a stroll down the misty streets. In a coffee shop, watched a golf match on television, then I went to an entertainment center and played a video game. The object of the game was to wipe out tanks invading from across the river. I was winning at first, but as the game went on, the enemy tanks bred like lemmings, crushing me by sheer number and destroying my base. An on-screen nuclear blast took care of everything, followed by the message game over insert coin.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Now said Handsley, when Angela had poured out the last cup, \"it's ", "twenty-five minutes past five", ", At half-past the Murder game is on\"", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("17:28", &[
&["It was ", "five-twenty-five", " when I pulled up in front of the library. Still early for our date, so I got out of the car and took a stroll down the misty streets. In a coffee shop, watched a golf match on television, then I went to an entertainment center and played a video game. The object of the game was to wipe out tanks invading from across the river. I was winning at first, but as the game went on, the enemy tanks bred like lemmings, crushing me by sheer number and destroying my base. An on-screen nuclear blast took care of everything, followed by the message game over insert coin.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Now said Handsley, when Angela had poured out the last cup, \"it's ", "twenty-five minutes past five", ", At half-past the Murder game is on\"", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("17:29", &[
&["It was ", "five-twenty-five", " when I pulled up in front of the library. Still early for our date, so I got out of the car and took a stroll down the misty streets. In a coffee shop, watched a golf match on television, then I went to an entertainment center and played a video game. The object of the game was to wipe out tanks invading from across the river. I was winning at first, but as the game went on, the enemy tanks bred like lemmings, crushing me by sheer number and destroying my base. An on-screen nuclear blast took care of everything, followed by the message game over insert coin.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["Now said Handsley, when Angela had poured out the last cup, \"it's ", "twenty-five minutes past five", ", At half-past the Murder game is on\"", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("17:30", &[
&["He went up to his coachman, who was dozing on the box in the shadow, already lengthening, of a thick lime-tree; he admired the shifting clouds of midges circling over the hot horses, and, waking the coachman, he jumped into the carriage, and told him to drive to Bryansky’s. It was only after driving nearly five miles that he had sufficiently recovered himself to look at his watch, and realise that it was ", "half-past five", ", and he was late.", "Leo Tolstoy", "Anna Karenina"],
&["It was ", "half-past five", " before Holmes returned. He was bright, eager, and in excellent spirits, a mood which in his case alternated with fits of the blackest depression.", "Arthur Conan Doyle", "The Sign Of Four"],
]);
    minutes.insert("17:31", &[
&["He went up to his coachman, who was dozing on the box in the shadow, already lengthening, of a thick lime-tree; he admired the shifting clouds of midges circling over the hot horses, and, waking the coachman, he jumped into the carriage, and told him to drive to Bryansky’s. It was only after driving nearly five miles that he had sufficiently recovered himself to look at his watch, and realise that it was ", "half-past five", ", and he was late.", "Leo Tolstoy", "Anna Karenina"],
&["It was ", "half-past five", " before Holmes returned. He was bright, eager, and in excellent spirits, a mood which in his case alternated with fits of the blackest depression.", "Arthur Conan Doyle", "The Sign Of Four"],
]);
    minutes.insert("17:32", &[
&["He went up to his coachman, who was dozing on the box in the shadow, already lengthening, of a thick lime-tree; he admired the shifting clouds of midges circling over the hot horses, and, waking the coachman, he jumped into the carriage, and told him to drive to Bryansky’s. It was only after driving nearly five miles that he had sufficiently recovered himself to look at his watch, and realise that it was ", "half-past five", ", and he was late.", "Leo Tolstoy", "Anna Karenina"],
&["It was ", "half-past five", " before Holmes returned. He was bright, eager, and in excellent spirits, a mood which in his case alternated with fits of the blackest depression.", "Arthur Conan Doyle", "The Sign Of Four"],
]);
    minutes.insert("17:33", &[
&["At ", "5:33 p.m.", " there is a blast of two deep, resonant notes a major third apart. On another day there is the same blast at 12:54 p.m. On another, exactly 8:00 a.m.", "Lydia Davis", "Varieties of Disturbance"],
]);
    minutes.insert("17:34", &[
&["At ", "5:33 p.m.", " there is a blast of two deep, resonant notes a major third apart. On another day there is the same blast at 12:54 p.m. On another, exactly 8:00 a.m.", "Lydia Davis", "Varieties of Disturbance"],
]);
    minutes.insert("17:35", &[
&["At ", "5:33 p.m.", " there is a blast of two deep, resonant notes a major third apart. On another day there is the same blast at 12:54 p.m. On another, exactly 8:00 a.m.", "Lydia Davis", "Varieties of Disturbance"],
]);
    minutes.insert("17:36", &[
&["At ", "5:33 p.m.", " there is a blast of two deep, resonant notes a major third apart. On another day there is the same blast at 12:54 p.m. On another, exactly 8:00 a.m.", "Lydia Davis", "Varieties of Disturbance"],
]);
    minutes.insert("17:37", &[
&["Look, Lucille, said Joe when Lucille strolled into the office at ", "5:37", ". \"I don't know what you said to this gal, but it seems to have had exactly the opposite of the desired effect. She's got some bee in her bonnet about Harvard Law School.\"", "Helen DeWitt", "Lightning Rods"],
]);
    minutes.insert("17:38", &[
&["Look, Lucille, said Joe when Lucille strolled into the office at ", "5:37", ". \"I don't know what you said to this gal, but it seems to have had exactly the opposite of the desired effect. She's got some bee in her bonnet about Harvard Law School.\"", "Helen DeWitt", "Lightning Rods"],
]);
    minutes.insert("17:39", &[
&["Look, Lucille, said Joe when Lucille strolled into the office at ", "5:37", ". \"I don't know what you said to this gal, but it seems to have had exactly the opposite of the desired effect. She's got some bee in her bonnet about Harvard Law School.\"", "Helen DeWitt", "Lightning Rods"],
]);
    minutes.insert("17:40", &[
&["Hey, young man, what time is it? 'What?' I said, is it 5:30 yet? 'Er, ", "5:40", ".' Heavens, they'll be starving. But then that's a good thing. Let them.'", "Zhu Wen", "I Love Dollars"],
&["It's ", "five-forty", " now. The party's at six. By about ten past, the eleventh floor should be clearing. Arnold is a very popular partner; no one's going to miss his farewell speech if they can help it. Plus, at Carter Spink parties, the speeches always happen early on, so people can get back to work if they need to. And while everyone's listening I'll slip down to Arnold's office. It should work. It has to work. As I stare at my own bizarre reflection, I feel a grim resolve hardening inside me. He's not going to get away with everyone thinking he's a cheery, harmless old teddy bear. He's not going to get away with it.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("17:41", &[
&["Hey, young man, what time is it? 'What?' I said, is it 5:30 yet? 'Er, ", "5:40", ".' Heavens, they'll be starving. But then that's a good thing. Let them.'", "Zhu Wen", "I Love Dollars"],
&["It's ", "five-forty", " now. The party's at six. By about ten past, the eleventh floor should be clearing. Arnold is a very popular partner; no one's going to miss his farewell speech if they can help it. Plus, at Carter Spink parties, the speeches always happen early on, so people can get back to work if they need to. And while everyone's listening I'll slip down to Arnold's office. It should work. It has to work. As I stare at my own bizarre reflection, I feel a grim resolve hardening inside me. He's not going to get away with everyone thinking he's a cheery, harmless old teddy bear. He's not going to get away with it.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("17:42", &[
&["Janice is not waiting for him in the lounge or beside the pool when at last ", "around 5.45", " they come home from playing the par-5 eighteenth. Instead one of the girls in their green and white uniforms comes over and tells him that his wife wants him to call home.", "John Updike", "Rabbit Is Rich"],
]);
    minutes.insert("17:43", &[
&["Janice is not waiting for him in the lounge or beside the pool when at last ", "around 5.45", " they come home from playing the par-5 eighteenth. Instead one of the girls in their green and white uniforms comes over and tells him that his wife wants him to call home.", "John Updike", "Rabbit Is Rich"],
]);
    minutes.insert("17:44", &[
&["Janice is not waiting for him in the lounge or beside the pool when at last ", "around 5.45", " they come home from playing the par-5 eighteenth. Instead one of the girls in their green and white uniforms comes over and tells him that his wife wants him to call home.", "John Updike", "Rabbit Is Rich"],
]);
    minutes.insert("17:45", &[
&["Janice is not waiting for him in the lounge or beside the pool when at last around ", "5.45", " they come home from playing the par-5 eighteenth. Instead one of the girls in their green and white uniforms comes over and tells him that his wife wants him to call home.", "John Updike", "Rabbit Is Rich"],
]);
    minutes.insert("17:46", &[
&["Through the curtained windows of the furnished apartment which Mrs. Horace Hignett had rented for her stay in New York rays of golden sunlight peeped in like the foremost spies of some advancing army. It was a fine summer morning. The hands of the Dutch clock in the hall pointed to thirteen minutes past nine; those of the ormolu clock in the sitting-room to eleven minutes past ten; those of the carriage clock on the bookshelf to ", "fourteen minutes to six", ". In other words, it was exactly eight; and Mrs. Hignett acknowledged the fact by moving her head on the pillow, opening her eyes, and sitting up in bed. She always woke at eight precisely.", "P.G. Wodehouse", "Three Men and a Maid"],
]);
    minutes.insert("17:47", &[
&["Through the curtained windows of the furnished apartment which Mrs. Horace Hignett had rented for her stay in New York rays of golden sunlight peeped in like the foremost spies of some advancing army. It was a fine summer morning. The hands of the Dutch clock in the hall pointed to thirteen minutes past nine; those of the ormolu clock in the sitting-room to eleven minutes past ten; those of the carriage clock on the bookshelf to ", "fourteen minutes to six", ". In other words, it was exactly eight; and Mrs. Hignett acknowledged the fact by moving her head on the pillow, opening her eyes, and sitting up in bed. She always woke at eight precisely.", "P.G. Wodehouse", "Three Men and a Maid"],
]);
    minutes.insert("17:48", &[
&["Father came home at ", "5:48 p.m.", " I heard him come through the front door. Then he came into the living room. He was wearing a lime green and sky blue check shirt and there was a double knot on one of his shoes but not on the other.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("17:49", &[
&["Father came home at ", "5:48 p.m.", " I heard him come through the front door. Then he came into the living room. He was wearing a lime green and sky blue check shirt and there was a double knot on one of his shoes but not on the other.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert(
        "17:50",
        &[&[
            "\"What time is it Jack?\" \"",
            "Ten to six",
            "\"\"Ten more minutes then.\" I shuffle the cards. \"Time for a quick game of rummy?\"",
            "Malorie Blackman",
            "Noughts and Crosses",
        ]],
    );
    minutes.insert(
        "17:51",
        &[&[
            "\"What time is it Jack?\" \"",
            "Ten to six",
            "\"\"Ten more minutes then.\" I shuffle the cards. \"Time for a quick game of rummy?\"",
            "Malorie Blackman",
            "Noughts and Crosses",
        ]],
    );
    minutes.insert(
        "17:52",
        &[&[
            "\"What time is it Jack?\" \"",
            "Ten to six",
            "\"\"Ten more minutes then.\" I shuffle the cards. \"Time for a quick game of rummy?\"",
            "Malorie Blackman",
            "Noughts and Crosses",
        ]],
    );
    minutes.insert("17:53", &[
&["\"That boy will be spoiled, as sure as I go on springs; he's made such a lot of. Have you been regulated?\" \"I should think I have!\" exclaimed I, in indignant recollection of my education. \"All right; keep your temper. What time are you?\" \"", "Seven minutes to six", ".\"", "Talbot Baines Reed", "The Adventures of a Three-Guinea Watch"],
]);
    minutes.insert("17:54", &[
&["It was ", "5:54 pm", " when Father came back into the living room. He said, 'What is this?\" but he said it very quietly and I didn't realise that he was angry because he wasn't shouting.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("17:55", &[
&["The wind moaned and sang dismally, catching the ears and lifting the shabby coat-tails of Mr Mortimer Jenkyn, 'Photographic Artist', as he stood outside and put the shutters up with this own cold hands in despair of further trade. It was ", "five minutes to six", ".", "Algernon Blackwood", "The Deferred Appointment"],
]);
    minutes.insert("17:56", &[
&["The wind moaned and sang dismally, catching the ears and lifting the shabby coat-tails of Mr Mortimer Jenkyn, 'Photographic Artist', as he stood outside and put the shutters up with this own cold hands in despair of further trade. It was ", "five minutes to six", ".", "Algernon Blackwood", "The Deferred Appointment"],
]);
    minutes.insert("17:57", &[
&["When he arrived it was ", "nearly six o'clock", ", and the sun was setting full and warm, and the red light streamed in through the window and gave more colour to the pale cheeks.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("17:58", &[
&["It was ", "nearly six o'clock", " in the evening, and the absurd bell in the six-foot tin steeple of the church went clank-clank, clank- clank! as old Mattu pulled the rope within.'", "George Orwell", "Burmese Days"],
]);
    minutes.insert("17:59", &[
&["When he arrived it was ", "nearly six o'clock", ", and the sun was setting full and warm, and the red light streamed in through the window and gave more colour to the pale cheeks.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("18:00", &[
&["", "6.00 p.m.", " Have tea", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Although it was only ", "six o'clock", ", the night was already dark. The fog, made thicker by its proximity to the Seine, blurred every detail with its ragged veils, punctured at various distances by the reddish glow of lanterns and bars of light escaping from illuminated windows.", "Théophile Gautier", "Le Club des Hachichins"],
&["Did you go down to the farm while I was away?' 'No,' I said 'but I saw Ted.' 'Did he have a message for me ?' she asked. 'He said today was no good as he was going to Norwich. But Friday at ", "six o'clock", ", same as usual.' 'Are you sure he said six o'clock?' she asked, puzzled. 'Quite sure.'", "L.P. Hartley", "The Go-Between"],
&["King Richard: What is o'clock? Catesby: It is ", "six o'clock", ", full supper time. King Richard: I will not sup tonight. Give me some ink and paper.", "William Shakespeare", "Richard III"],
&["Leon waited all day for ", "six o'clock", " to arrive; when he got to the inn, he found no one there but Monsieur Binet, already at the table.", "Gustave Flaubert", "Madame Bovary"],
&["Oh oh oh. ", "Six o'clock", " and the master not home yet.", "Thornton Wilder", "The skin of our teeth"],
&["The newspaper snaked through the door and there was suddenly a ", "six o'clock", " feeling in the house", "Muriel Spark", "The Prime of Miss Jean Brodie"],
&["The winter evening settles down With smell of steaks in passageways. ", "Six o'clock", ".", "T S Eliot", "Preludes"],
&["When the bells of Calvary Church struck ", "six", ", she saw Mr and Mrs Biggs hurrying down the front stoop, rushing off to the shops before they closed.", "Jed Rubenfeld", "The Interpretation Of Murder"],
]);
    minutes.insert("18:01", &[
&["", "6.00 p.m.", " Have tea", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Although it was only ", "six o'clock", ", the night was already dark. The fog, made thicker by its proximity to the Seine, blurred every detail with its ragged veils, punctured at various distances by the reddish glow of lanterns and bars of light escaping from illuminated windows.", "Théophile Gautier", "Le Club des Hachichins"],
&["Did you go down to the farm while I was away?' 'No,' I said 'but I saw Ted.' 'Did he have a message for me ?' she asked. 'He said today was no good as he was going to Norwich. But Friday at ", "six o'clock", ", same as usual.' 'Are you sure he said six o'clock?' she asked, puzzled. 'Quite sure.'", "L.P. Hartley", "The Go-Between"],
&["King Richard: What is o'clock? Catesby: It is ", "six o'clock", ", full supper time. King Richard: I will not sup tonight. Give me some ink and paper.", "William Shakespeare", "Richard III"],
&["Leon waited all day for ", "six o'clock", " to arrive; when he got to the inn, he found no one there but Monsieur Binet, already at the table.", "Gustave Flaubert", "Madame Bovary"],
&["Oh oh oh. ", "Six o'clock", " and the master not home yet.", "Thornton Wilder", "The skin of our teeth"],
&["The newspaper snaked through the door and there was suddenly a ", "six o'clock", " feeling in the house", "Muriel Spark", "The Prime of Miss Jean Brodie"],
&["The winter evening settles down With smell of steaks in passageways. ", "Six o'clock", ".", "T S Eliot", "Preludes"],
&["When the bells of Calvary Church struck ", "six", ", she saw Mr and Mrs Biggs hurrying down the front stoop, rushing off to the shops before they closed.", "Jed Rubenfeld", "The Interpretation Of Murder"],
]);
    minutes.insert("18:02", &[
&["", "6.00 p.m.", " Have tea", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Although it was only ", "six o'clock", ", the night was already dark. The fog, made thicker by its proximity to the Seine, blurred every detail with its ragged veils, punctured at various distances by the reddish glow of lanterns and bars of light escaping from illuminated windows.", "Théophile Gautier", "Le Club des Hachichins"],
&["Did you go down to the farm while I was away?' 'No,' I said 'but I saw Ted.' 'Did he have a message for me ?' she asked. 'He said today was no good as he was going to Norwich. But Friday at ", "six o'clock", ", same as usual.' 'Are you sure he said six o'clock?' she asked, puzzled. 'Quite sure.'", "L.P. Hartley", "The Go-Between"],
&["King Richard: What is o'clock? Catesby: It is ", "six o'clock", ", full supper time. King Richard: I will not sup tonight. Give me some ink and paper.", "William Shakespeare", "Richard III"],
&["Leon waited all day for ", "six o'clock", " to arrive; when he got to the inn, he found no one there but Monsieur Binet, already at the table.", "Gustave Flaubert", "Madame Bovary"],
&["Oh oh oh. ", "Six o'clock", " and the master not home yet.", "Thornton Wilder", "The skin of our teeth"],
&["The newspaper snaked through the door and there was suddenly a ", "six o'clock", " feeling in the house", "Muriel Spark", "The Prime of Miss Jean Brodie"],
&["The winter evening settles down With smell of steaks in passageways. ", "Six o'clock", ".", "T S Eliot", "Preludes"],
&["When the bells of Calvary Church struck ", "six", ", she saw Mr and Mrs Biggs hurrying down the front stoop, rushing off to the shops before they closed.", "Jed Rubenfeld", "The Interpretation Of Murder"],
]);
    minutes.insert("18:03", &[
&["Above it all rose the Houses of Parliament, with the hands of the clock stopped at ", "three minutes past six", ". It was difficult to believe that all that meant nothing any more, that now it was just a pretentious confection that could decay in peace.", "John Wyndham", "The Day of the Triffids"],
]);
    minutes.insert("18:04", &[
&["\"We will make record of it, my Rosannah; every year, as this dear hour chimes from the clock, we will celebrate it with thanksgivings, all the years of our life.\" \"We will, we will, Alonzo!\" \"", "Four minutes after six", ", in the evening, my Rosannah...”", "Mark Twain", "The Loves of Alonzo Fitz Clarence and Rosannah Ethelton"],
]);
    minutes.insert(
        "18:05",
        &[&[
            "At about ",
            "five past six",
            " Piers came in carrying an evening paper and a few books.",
            "Barbara Pym",
            "A Glass of Blessings",
        ]],
    );
    minutes.insert(
        "18:06",
        &[&[
            "At about ",
            "five past six",
            " Piers came in carrying an evening paper and a few books.",
            "Barbara Pym",
            "A Glass of Blessings",
        ]],
    );
    minutes.insert(
        "18:07",
        &[&[
            "At about ",
            "five past six",
            " Piers came in carrying an evening paper and a few books.",
            "Barbara Pym",
            "A Glass of Blessings",
        ]],
    );
    minutes.insert(
        "18:08",
        &[&[
            "",
            "6:08 p.m.",
            " The code-word \"Valkyrie\" reached von Seydlitz Gabler's headquarters",
            "Hans Hellmut Kirst",
            "The Night of the Generals",
        ]],
    );
    minutes.insert(
        "18:09",
        &[&[
            "",
            "6:08 p.m.",
            " The code-word \"Valkyrie\" reached von Seydlitz Gabler's headquarters",
            "Hans Hellmut Kirst",
            "The Night of the Generals",
        ]],
    );
    minutes.insert("18:10", &[
&["'Let me see now. You had a drink at the Continental at ", "six ten", ".' 'Yes.' 'And at six forty-five you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
]);
    minutes.insert("18:11", &[
&["'Let me see now. You had a drink at the Continental at ", "six ten", ".' 'Yes.' 'And at six forty-five you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
]);
    minutes.insert("18:12", &[
&["'Let me see now. You had a drink at the Continental at ", "six ten", ".' 'Yes.' 'And at six forty-five you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
]);
    minutes.insert("18:13", &[
&["'Let me see now. You had a drink at the Continental at ", "six ten", ".' 'Yes.' 'And at six forty-five you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
]);
    minutes.insert("18:14", &[
&["'Let me see now. You had a drink at the Continental at ", "six ten", ".' 'Yes.' 'And at six forty-five you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
]);
    minutes.insert(
        "18:15",
        &[
            &[
                "'",
                "Quarter past six",
                ",' said Tony. 'He's bound to have told her by now.'",
                "Evelyn Waugh",
                "A Handful of Dust",
            ],
            &[
                "At a ",
                "quarter past six",
                " he was through with them.",
                "Penelope Lively",
                "The Photograph",
            ],
            &[
                "I checked the time on the corner of my screen. ",
                "6.15 pm.",
                " I was never going to finisah my essay in forty-five minutes",
                "Sophie McKenzie",
                "Girl Missing",
            ],
        ],
    );
    minutes.insert(
        "18:16",
        &[
            &[
                "'",
                "Quarter past six",
                ",' said Tony. 'He's bound to have told her by now.'",
                "Evelyn Waugh",
                "A Handful of Dust",
            ],
            &[
                "At a ",
                "quarter past six",
                " he was through with them.",
                "Penelope Lively",
                "The Photograph",
            ],
            &[
                "I checked the time on the corner of my screen. ",
                "6.15 pm.",
                " I was never going to finisah my essay in forty-five minutes",
                "Sophie McKenzie",
                "Girl Missing",
            ],
        ],
    );
    minutes.insert(
        "18:17",
        &[
            &[
                "'",
                "Quarter past six",
                ",' said Tony. 'He's bound to have told her by now.'",
                "Evelyn Waugh",
                "A Handful of Dust",
            ],
            &[
                "At a ",
                "quarter past six",
                " he was through with them.",
                "Penelope Lively",
                "The Photograph",
            ],
            &[
                "I checked the time on the corner of my screen. ",
                "6.15 pm.",
                " I was never going to finisah my essay in forty-five minutes",
                "Sophie McKenzie",
                "Girl Missing",
            ],
        ],
    );
    minutes.insert(
        "18:18",
        &[
            &[
                "'",
                "Quarter past six",
                ",' said Tony. 'He's bound to have told her by now.'",
                "Evelyn Waugh",
                "A Handful of Dust",
            ],
            &[
                "At a ",
                "quarter past six",
                " he was through with them.",
                "Penelope Lively",
                "The Photograph",
            ],
            &[
                "I checked the time on the corner of my screen. ",
                "6.15 pm.",
                " I was never going to finisah my essay in forty-five minutes",
                "Sophie McKenzie",
                "Girl Missing",
            ],
        ],
    );
    minutes.insert(
        "18:19",
        &[
            &[
                "'",
                "Quarter past six",
                ",' said Tony. 'He's bound to have told her by now.'",
                "Evelyn Waugh",
                "A Handful of Dust",
            ],
            &[
                "At a ",
                "quarter past six",
                " he was through with them.",
                "Penelope Lively",
                "The Photograph",
            ],
            &[
                "I checked the time on the corner of my screen. ",
                "6.15 pm.",
                " I was never going to finisah my essay in forty-five minutes",
                "Sophie McKenzie",
                "Girl Missing",
            ],
        ],
    );
    minutes.insert(
        "18:20",
        &[&[
            "By the time Elliot's mother arrived at ",
            "twenty past six",
            ", Mrs Sen always made sure all evidence of her chopping was disposed of.",
            "Jhumpa Lahiri",
            "Interpreter of Maladies",
        ]],
    );
    minutes.insert("18:21", &[
&["5.20pm - ", "6.21pm", ": Miss Pettigrew found herself wafted into the passage. She was past remonstrance now, past bewilderment, surprise, expostulation. Her eyes shone. Her face glowed. Her spirits soared. Everything was happening too quickly. She couldn't keep up with things, but, by golly, she could enjoy them.", "Winifred Watson", "Miss Pettigrew Lives for a Day"],
]);
    minutes.insert(
        "18:22",
        &[&[
            "Clock overturned when he fell forward. That'll give us the time of the crime. ",
            "Twenty-two minutes past six",
            ".",
            "Agatha Christie",
            "The Murder at the Vicarage",
        ]],
    );
    minutes.insert(
        "18:23",
        &[&[
            "Clock overturned when he fell forward. That'll give us the time of the crime. ",
            "Twenty-two minutes past six",
            ".",
            "Agatha Christie",
            "The Murder at the Vicarage",
        ]],
    );
    minutes.insert(
        "18:24",
        &[&[
            "Clock overturned when he fell forward. That'll give us the time of the crime. ",
            "Twenty-two minutes past six",
            ".",
            "Agatha Christie",
            "The Murder at the Vicarage",
        ]],
    );
    minutes.insert("18:25", &[
&["At ", "twenty-five past six", " I go into the bathroom and have a wash, then while the Old Lady's busy in the kitchen helping Chris with the washing up I get my coat and nip out down the stairs.", "Stan Barstow", "A Kind of Loving"],
&["I have this moment, while writing, had a wire from Jonathan saying that he leaves by the ", "6.25", " tonight from Launceston and will be here at 10.18, so that I shall have no fear tonight.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("18:26", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:27", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:28", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:29", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:30", &[
&["At ", "six-thirty", " I left the bar and walked outside. It was getting dark and the big Avenida looked cool and graceful. On the other side were homes that once looked out on the beach. Now they looked out on hotels and most of them had retreated behind tall hedges and walls that cut them off from the street.", "Hunter S. Thompson", "The Rum Diary"],
&["", "6.30 p.m.", " Watch television or a video", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["As I was turning away, grieved to be parting from him, a thought started up in me and I turned back. 'Shall I take one more message for you?' 'That's good of you' he said, 'but do you want to?' 'Yes, just this once.' It could do no harm, I thought; and I should be far away when the message takes effect, and I wanted to say something to show we were friends. 'Well,' he said, once more across the gap, 'say tomorrow's no good, I'm going to Norwich, but Friday at ", "half-past six", ", same as usual.'", "L.P. Hartley", "The Go-Between"],
&["At five o'clock the two ladies retired to dress, and at ", "half past six", " Elizabeth was summoned to dinner.", "Jane Austen", "Pride and Prejudice"],
&["It is ", "six thirty", ". Now the dark night and the deafening racket of the crickets again engulf the garden and the veranda, all around the house", "Alain Robbe-Grillet", "Jealousy"],
&["To a casual visitor it might have seemed that Mr Penicuik, who owned the house, had fallen upon evil days; but two of the three gentlemen assembled in the Saloon at ", "half-past six", " on a wintry evening of late February were in no danger of falling into this error.", "Georgette Heyer", "Cotillion"],
]);
    minutes.insert(
        "18:31",
        &[&[
            "I had been delayed at a case and it was ",
            "a little after half past six",
            " when I found myself at Baker Street once more",
            "Arthur Conan Doyle",
            "The Adventure of The Blue Carbuncle",
        ]],
    );
    minutes.insert("18:32", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:33", &[
&["Every evening, Michel took the train home, changed at Esbly and usually arrived in Crécy on the ", "6.33pm", " train where Annabelle would be waiting at the station.", "Michel Houellebecq", "Atomised"],
]);
    minutes.insert("18:34", &[
&["It is ", "around half past six", " in the evening. Dusk is gathering in the living room, an early dusk due to the fog which has rolled in from the Sound and is like a white curtain drawn down outside the windows.", "Eugene O'Neill", "Long Day's Journey Into Night"],
]);
    minutes.insert("18:35", &[
&["And then it was ", "6.35 pm", " and I heard Father come home in his van and I moved the bed up against the door so he couldn't get in and he came into the house and he and Mother shouted at each other.", "Mark Haddon", "The Curious Incident Of The Dog In The Night-Time"],
]);
    minutes.insert("18:36", &[
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke 9:05. To sleep ", "6:36", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("18:37", &[
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke 9:05. To sleep ", "6:36", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("18:38", &[
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke 9:05. To sleep ", "6:36", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("18:39", &[
&["Kaldren pursues me like luminescent shadow. He has chalked up on the gateway '96,688,365,498,702'. Should confuse the mail man. Woke 9:05. To sleep ", "6:36", ".", "JG Ballard", "The Voices of Time"],
]);
    minutes.insert("18:40", &[
&["Amy: What's that? I thought I saw someone pass the window. What time is it? Charles: Nearly ", "twenty to seven", ".", "TS Eliot", "The Family Reunion"],
&["Having to change 'buses, I allowed plenty of time — in fact, too much; for we arrived at ", "twenty minutes to seven", ", and Franching, so the servant said, had only just gone up to dress.", "George and Weedon Grossmith", "Diary of a Nobody"],
]);
    minutes.insert("18:41", &[
&["He made it to Grand Central well in advance. Stillman's train was not due to arrive until ", "six forty-one", ", but Quinn wanted time to study the geography of the place, to make sure that Stillman would not be able to slip away from him.", "Paul Auster", "The New York Trilogy"],
]);
    minutes.insert("18:42", &[
&["He made it to Grand Central well in advance. Stillman's train was not due to arrive until ", "six forty-one", ", but Quinn wanted time to study the geography of the place, to make sure that Stillman would not be able to slip away from him.", "Paul Auster", "The New York Trilogy"],
]);
    minutes.insert("18:43", &[
&["He made it to Grand Central well in advance. Stillman's train was not due to arrive until ", "six forty-one", ", but Quinn wanted time to study the geography of the place, to make sure that Stillman would not be able to slip away from him.", "Paul Auster", "The New York Trilogy"],
]);
    minutes.insert("18:44", &[
&["He made it to Grand Central well in advance. Stillman's train was not due to arrive until ", "six forty-one", ", but Quinn wanted time to study the geography of the place, to make sure that Stillman would not be able to slip away from him.", "Paul Auster", "The New York Trilogy"],
]);
    minutes.insert("18:45", &[
&["'Let me see now. You had a drink at the Continental at six ten.' 'Yes.' 'And at ", "six forty-five", " you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
&["\"", "Six forty-five", ",\" called Louie. \"Did you hear, Ming,\" he asked, \"did you hear?\" \"Yes, Taddy, I heard.\" \"What is it?' asked Tommy. \"The new baby, listen, the new baby.\"", "Christina Stead", "The Man Who Loved Children"],
&["It was a ", "quarter to seven", " when I let myself into the office and clicked the light on and picked a piece of paper off the floor. It was a notice from the Green Feather Messenger Service ...", "Raymond Chandler", "The High Window"],
]);
    minutes.insert("18:46", &[
&["'Let me see now. You had a drink at the Continental at six ten.' 'Yes.' 'And at ", "six forty-five", " you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
&["\"", "Six forty-five", ",\" called Louie. \"Did you hear, Ming,\" he asked, \"did you hear?\" \"Yes, Taddy, I heard.\" \"What is it?' asked Tommy. \"The new baby, listen, the new baby.\"", "Christina Stead", "The Man Who Loved Children"],
&["It was a ", "quarter to seven", " when I let myself into the office and clicked the light on and picked a piece of paper off the floor. It was a notice from the Green Feather Messenger Service ...", "Raymond Chandler", "The High Window"],
]);
    minutes.insert("18:47", &[
&["'Let me see now. You had a drink at the Continental at six ten.' 'Yes.' 'And at ", "six forty-five", " you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
&["\"", "Six forty-five", ",\" called Louie. \"Did you hear, Ming,\" he asked, \"did you hear?\" \"Yes, Taddy, I heard.\" \"What is it?' asked Tommy. \"The new baby, listen, the new baby.\"", "Christina Stead", "The Man Who Loved Children"],
&["It was a ", "quarter to seven", " when I let myself into the office and clicked the light on and picked a piece of paper off the floor. It was a notice from the Green Feather Messenger Service ...", "Raymond Chandler", "The High Window"],
]);
    minutes.insert("18:48", &[
&["'Let me see now. You had a drink at the Continental at six ten.' 'Yes.' 'And at ", "six forty-five", " you were talking to another journalist at the door of the Majestic?' 'Yes, Wilkins. I told you all this, Vigot, before. That night.'", "Graham Greene", "The Quiet American"],
&["\"", "Six forty-five", ",\" called Louie. \"Did you hear, Ming,\" he asked, \"did you hear?\" \"Yes, Taddy, I heard.\" \"What is it?' asked Tommy. \"The new baby, listen, the new baby.\"", "Christina Stead", "The Man Who Loved Children"],
&["It was a ", "quarter to seven", " when I let myself into the office and clicked the light on and picked a piece of paper off the floor. It was a notice from the Green Feather Messenger Service ...", "Raymond Chandler", "The High Window"],
]);
    minutes.insert(
        "18:49",
        &[&[
            "",
            "6:49 p.m.",
            " Lieutenant-General Tanz escorted by a motorized unit, drove to Corps headquarters",
            "Hans Hellmut Kirst",
            "The Night of the Generals",
        ]],
    );
    minutes.insert("18:50", &[
&["At ", "ten minutes to seven", " Dulcie was ready. She looked at herself in the wrinkly mirror. The reflection was satisfactory. The dark blue dress, fitting without a wrinkle, the hat with its jaunty black feather, the but-slightly-soiled gloves--all representing self- denial, even of food itself--were vastly becoming. Dulcie forgot everything else for a moment except that she was beautiful, and that life was about to lift a corner of its mysterious veil for her to observe its wonders. No gentleman had ever asked her out before. Now she was going for a brief moment into the glitter and exalted show.", "O. Henry", "The Four Million"],
&["It was time to go see the Lady. When we arrived at her house at ", "ten minutes before seven", " o'clock, Damaronde answered the door.", "Robert R. McCammon", "Boy's Life"],
&["It was time to go see the Lady. When we arrived at her house at ", "ten minutes before seven", " o'clock, Damaronde answered the door.", "Robert R. McCammon", "Boy's Life"],
]);
    minutes.insert(
        "18:51",
        &[&[
            "The square of light in the kitchen doorway had faded to thin purple; his watch said ",
            "6:51",
            ".",
            "Stephen King",
            "Salem's Lot",
        ]],
    );
    minutes.insert(
        "18:52",
        &[&[
            "The square of light in the kitchen doorway had faded to thin purple; his watch said ",
            "6:51",
            ".",
            "Stephen King",
            "Salem's Lot",
        ]],
    );
    minutes.insert("18:53", &[
&["It was ", "near on seven o'clock", " when I got to Mr. and Mrs. Fleming's house on 6th Street, where I was renting a room. It was late September, and though there was some sun left, I didn't want to visit a dead man's place with night coming on.", "Edward P Jones", "All Aunt Hagar's Children"],
]);
    minutes.insert("18:54", &[
&["It was ", "near on seven o'clock", " when I got to Mr. and Mrs. Fleming's house on 6th Street, where I was renting a room. It was late September, and though there was some sun left, I didn't want to visit a dead man's place with night coming on.", "Edward P Jones", "All Aunt Hagar's Children"],
]);
    minutes.insert("18:55", &[
&["... You had no reason to think the times important. Indeed how suspicious it would be if you had been completely accurate. ''Haven't I been?'' Not quite. It was ", "five to seven", " that you talked to Wilkins. ''Another ten minutes.\"", "Graham Greene", "The Quiet American"],
&["The play was set to begin at seven o'clock and finish before sunset. It was ", "6:55", ". Beyond the flats we could hear the hockey field filling up. the low rumble got steadily louder - voices, footsteps, the creaking of bleachers, the slamming of car doors in the parking lot.", "Jeffrey Eugenides", "Middlesex"],
]);
    minutes.insert("18:56", &[
&["Then it was ", "6.56", ". A black Rover - a Rover 90, registration PYX 520 - turned into the street that ran down the left-hand side of The Bunker. It parked. The door on the driver's side opened. A man got out.", "Rupert Thomson", "Dreams of leaving"],
]);
    minutes.insert("18:57", &[
&["\"I feel a little awkward,\" Kay Randall said on the phone, \"asking a man to do these errands ... but that's my problem, not yours. Just bring the supplies and try to be at the church meeting room ", "a few minutes before seven", ".\"", "Max Apple", "Bridging"],
&["Folded in this triple melody, the audience sat gazing; and beheld gently and approvingly without interrogation, for it seemed inevitable, a box tree in a green tub take the place of the ladies’ dressing-room; while on what seemed to be a wall, was hung a great clock face; the hands pointing to ", "three minutes to the hour; which was seven", ".'", "Virginia Woolf", "Between the Acts"],
]);
    minutes.insert("18:58", &[
&["\"Walk fast,\" says Perry, \"it's ", "two minutes to seven", ", and I got to be home by—' \"Oh, shut up,\" says I. \"I had an appointment as chief performer at an inquest at seven, and I'm not kicking about not keeping it.\"", "O. Henry", "Roads of Destiny"],
]);
    minutes.insert("18:59", &[
&["", "About seven o’clock", " in the evening she had died, and her frantic husband had made a frightful scene in his efforts to kill West, whom he wildly blamed for not saving her life. Friends had held him when he drew a stiletto, but West departed amidst his inhuman shrieks, curses, and oaths of vengeance.", "H. P. Lovecraft", "Herbert West - Reanimator"],
]);
    minutes.insert("19:00", &[
&["… in a word, seen always at the same evening hour, isolated from all its possible surroundings, detached and solitary against its shadowy background, the bare minimum of scenery necessary .. to the drama of my undressing, as though all Combray had consisted of but two floors joined by a slender staircase, and as though there had been no time there but ", "seven o'clock", " at night.", "Marcel Proust", "In Search of Lost Time: Swann's Way"],
&["The town clock struck ", "seven", ". The echoes of the great chime wandered in the unlit halls of the library. An autumn leaf, very crisp, fell somewhere in the dark. But it was only the page of a book, turning.", "Ray Bradbury", "Something Wicked This Way Comes"],
&["", "7.00 p.m.", " Do maths practice", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["By ", "seven o'clock", " the orchestra has arrived--no thin five-piece affair but a whole pitful of oboes and trombones and saxophones and viols and cornets and piccolos and low and high drums.", "F. Scott Fitzgerald", "The Great Gatsby"],
&["Edward had been allowed to see me only from ", "seven", " till nine-thirty pm, always inside the confines of my home and under the supervision of my dad's unfailingly crabby glare.", "Stephenie Meyer", "New Moon"],
&["It was ", "seven o'clock", " and by this time she was not very far from Raveloe, but she was not familiar enough with those monotonous lanes to know how near she was to her journey's end. She needed comfort, and she knew but one comforter - the familiar demon in her bosom; but she hesitated a moment, after drawing out the black remnant, before she raised it to her lips.", "George Eliot", "Silas Marner"],
&["It was ", "seven o'clock", " when we got into the coupé with him and started for Long Island. [...] So we drove on toward death through the cooling twilight.", "F. Scott Fitzgerald", "The Great Gatsby"],
]);
    minutes.insert(
        "19:01",
        &[&[
            "He waited until nearly eight, because ",
            "around seven",
            " there were always more people coming in and out of the house than at other times.",
            "Patricia Highsmith",
            "The Talented Mr Ripley",
        ]],
    );
    minutes.insert("19:02", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:03", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:04", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:05", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:06", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:07", &[
&["Twas ", "about seven o'clock", " at night, And the wind it blew with all its might, And the rain came pouring down, And the dark clouds seem'd to frown,", "William McGonagall", "The Tay Bridge Disaster"],
]);
    minutes.insert("19:08", &[
&["It was ", "eight minutes past seven", " and still no girl. I waited impatiently. I watched another crowd surge through the barriers and move quickly down the steps. My eyes were alert for the faintest recognition.", "James Furner", "The Girl from East Berlin"],
]);
    minutes.insert("19:09", &[
&["It was ", "eight minutes past seven", " and still no girl. I waited impatiently. I watched another crowd surge through the barriers and move quickly down the steps. My eyes were alert for the faintest recognition.", "James Furner", "The Girl from East Berlin"],
]);
    minutes.insert("19:10", &[
&["He had already got to the point where, by rocking more strongly, he maintained his equilibrium with difficulty, and very soon he would finally have to make a final decision, for ", "in five minutes it would be a quarter past seven", ". Then there was a ring at the door of the apartment. “That’s someone from the office,” he told himself, and he almost froze, while his small limbs only danced around all the faster. For one moment everything remained still. “They aren’t opening,” Gregor said to himself, caught up in some absurd hope.", "Franz Kafka", "Metamorphosis"],
&["The party was to begin at seven. The invitations gave the hour as six-thirty because the family knew everyone would come a little late, so as not to be the first to arrive. At ", "seven-ten", " not a soul had come; somewhat acrimoniously, the family discussed the advantages and disadvantages of tardiness", "Jorge Luis Borges", "The Elderly Lady"],
]);
    minutes.insert("19:11", &[
&["Good, you said. Run, or you won't get a seat. See you soon. Your voice was reassuring. ", "19:11", ":00, the clock said. I put the phone back on its hook and I ran. The seat I got, almost the last one in the carriage, was opposite a girl who started coughing as soon as there weren't any other free seats I could move to. She looked pale and the cough rattled deep in her chest as she punched numbers into her mobile. Hi, she said (cough). I'm on the train. No, I've got a cold. A cold (cough). Yeah, really bad. Yeah, awful actually. Hello? (cough) Hello?", "Ali Smith", "The Whole Story and Other Stories"],
]);
    minutes.insert("19:12", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at 7:45 or 6:30, Jasper, but pick times like ", "7:12", " and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("19:13", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at 7:45 or 6:30, Jasper, but pick times like ", "7:12", " and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("19:14", &[
&["If he'd lived in New York and worked in an office, he might have thrived as the typical, over-martini'd, cheating husband, leaving every night on the ", "7:14", " to White Plains, a smudge of lipstick high on his neck, and a tide of lies to see him through to the next day.", "Andrew O'Hagan", "The Life and Opinions of Maf the Dog, and of his friend Marilyn Monroe"],
]);
    minutes.insert("19:15", &[
&["Cell count down to 400,000. Woke 8:10. To sleep ", "7:15", ". (Appear to have lost my watch without realising it, had to drive into town to buy another.)", "JG Ballard", "The Voices of Time"],
&["Nick had a large wild plan of his own for the night, but for now he let Leo take charge: they were going to go back to Notting Hill and catch the ", "seven fifteen", " screening of Scarface at the Gate.", "Alan Hollinghurst", "The Line of Beauty"],
&["The party was to begin at seven. The invitations gave the hour as six-thirty because the famly knew everyone would come a little late, so as not to be the first to arrive. .. By ", "seven-fifteen", " not another soul could squeeze into the house.", "Jorge Luis Borges", "The elderly lady"],
]);
    minutes.insert(
        "19:16",
        &[&[
            "“",
            "Sixteen past seven PM",
            "? That's when he came into the store or when he left after the fact?”",
            "Patricia Cornwell",
            "The Last Precinct",
        ]],
    );
    minutes.insert("19:17", &[
&["Colonel Putnis knocked on his door at ", "7.17 p.m.", " The car was waiting in front of the hotel, and they drove through the dark streets to police headquarters. It had grown much colder during the evening, and the city was almost deserted.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("19:18", &[
&["Colonel Putnis knocked on his door at ", "7.17 p.m.", " The car was waiting in front of the hotel, and they drove through the dark streets to police headquarters. It had grown much colder during the evening, and the city was almost deserted.", "Henning Mankell", "The Dogs of Riga"],
]);
    minutes.insert("19:19", &[
&["And it was me who spent about three hours this afternoon arguing one single contract. The term was best endeavors. The other side wanted to use reasonable efforts. In the end we won the point- but I can't feel my usual triumph. All I know is, it's ", "seven-nineteen", ", and in eleven minutes I'm supposed to be halfway across town, sitting down to dinner at Maxim's with my mother and brother Daniel. I'll have to cancel. My own birthday dinner.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("19:20", &[
&["The clock read ", "seven-twenty", ", but I felt no hunger. You'd think I might have wanted to eat something after the day I'd had, but I cringed at the very thought of food. I was short of sleep, my gut was slashed, and my apartment was gutted. There was no room for appetite.", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
&["The pause, we finally concluded, was to allow the other important people to catch up, those who had arrived at 7:10 waiting for those who had arrived at ", "7:20", ".", "C Northcote Parkinson", "Parkinson's Law or the Pursuit of Progress"],
]);
    minutes.insert("19:21", &[
&["Gripping her gym bag in her right hand, Aomame, like Buzzcut, was waiting for something to happen. The clock display changed to ", "7:21", ", then 7:22, then 7:23.", "Haruki Murakami", "1Q84"],
]);
    minutes.insert("19:22", &[
&["Gripping her gym bag in her right hand, Aomame, like Buzzcut, was waiting for something to happen. The clock display changed to 7:21, then ", "7:22", ", then 7:23.", "Haruki Murakami", "1Q84"],
]);
    minutes.insert("19:23", &[
&["Gripping her gym bag in her right hand, Aomame, like Buzzcut, was waiting for something to happen. The clock display changed to 7:21, then 7:22, then ", "7:23", ".", "Haruki Murakami", "1Q84"],
]);
    minutes.insert("19:24", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was ", "almost twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:25", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was almost ", "twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:26", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was almost ", "twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:27", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was almost ", "twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:28", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was almost ", "twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:29", &[
&["He picked up his hat and coat and Clarice said hello to him and he said hello and looked at the clock and it was almost ", "twenty-five after seven", ".", "James Thurber", "The Evening's at Seven"],
]);
    minutes.insert("19:30", &[
&["But now he was close - here was the house, here were the gates. Somewhere a clock beat a single chime. 'What, is it really ", "half-past seven", "? That's impossible, it must be fast!’", "Fyodor Dostoyevsky", "Crime and Punishment"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at ", "7:30", " that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The clock showed ", "half-past seven", ". This was the twilight time. He would be there now. I pictured him in his old navy-blue sweater and peaked cap, walking soft-footed up the track towards the wood. He told me he wore the sweater because navy-blue barely showed up in the dark, black was even better, he said. The peaked cap was important too, he explained, because the peak casts a shadow over one's face.", "Roald Dahl", "Danny, the Champion of the World"],
&["The telephone call came at ", "7.30", " on the evening of March 18th, a Saturday, the eve of the noisy, colourful festival that the town held in honour of Saint Joseph the carpenter -", "Leonardo Sciascia", "A Simple Story"],
]);
    minutes.insert("19:31", &[
&["But now he was close - here was the house, here were the gates. Somewhere a clock beat a single chime. 'What, is it really ", "half-past seven", "? That's impossible, it must be fast!’", "Fyodor Dostoyevsky", "Crime and Punishment"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at ", "7:30", " that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The clock showed ", "half-past seven", ". This was the twilight time. He would be there now. I pictured him in his old navy-blue sweater and peaked cap, walking soft-footed up the track towards the wood. He told me he wore the sweater because navy-blue barely showed up in the dark, black was even better, he said. The peaked cap was important too, he explained, because the peak casts a shadow over one's face.", "Roald Dahl", "Danny, the Champion of the World"],
&["The telephone call came at ", "7.30", " on the evening of March 18th, a Saturday, the eve of the noisy, colourful festival that the town held in honour of Saint Joseph the carpenter -", "Leonardo Sciascia", "A Simple Story"],
]);
    minutes.insert("19:32", &[
&["But now he was close - here was the house, here were the gates. Somewhere a clock beat a single chime. 'What, is it really ", "half-past seven", "? That's impossible, it must be fast!’", "Fyodor Dostoyevsky", "Crime and Punishment"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at ", "7:30", " that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The clock showed ", "half-past seven", ". This was the twilight time. He would be there now. I pictured him in his old navy-blue sweater and peaked cap, walking soft-footed up the track towards the wood. He told me he wore the sweater because navy-blue barely showed up in the dark, black was even better, he said. The peaked cap was important too, he explained, because the peak casts a shadow over one's face.", "Roald Dahl", "Danny, the Champion of the World"],
&["The telephone call came at ", "7.30", " on the evening of March 18th, a Saturday, the eve of the noisy, colourful festival that the town held in honour of Saint Joseph the carpenter -", "Leonardo Sciascia", "A Simple Story"],
]);
    minutes.insert("19:33", &[
&["But now he was close - here was the house, here were the gates. Somewhere a clock beat a single chime. 'What, is it really ", "half-past seven", "? That's impossible, it must be fast!’", "Fyodor Dostoyevsky", "Crime and Punishment"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at ", "7:30", " that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The clock showed ", "half-past seven", ". This was the twilight time. He would be there now. I pictured him in his old navy-blue sweater and peaked cap, walking soft-footed up the track towards the wood. He told me he wore the sweater because navy-blue barely showed up in the dark, black was even better, he said. The peaked cap was important too, he explained, because the peak casts a shadow over one's face.", "Roald Dahl", "Danny, the Champion of the World"],
&["The telephone call came at ", "7.30", " on the evening of March 18th, a Saturday, the eve of the noisy, colourful festival that the town held in honour of Saint Joseph the carpenter -", "Leonardo Sciascia", "A Simple Story"],
]);
    minutes.insert("19:34", &[
&["But now he was close - here was the house, here were the gates. Somewhere a clock beat a single chime. 'What, is it really ", "half-past seven", "? That's impossible, it must be fast!’", "Fyodor Dostoyevsky", "Crime and Punishment"],
&["On July 25th, 8:30 a.m. the bitch Novaya dies whelping. At 10 o'clock she is lowered into her cool grave, at ", "7:30", " that same evening we see our first floes and greet them wishing they were the last.", "Christoph Ransmayr", "The Terrors of Ice and Darkness"],
&["The clock showed ", "half-past seven", ". This was the twilight time. He would be there now. I pictured him in his old navy-blue sweater and peaked cap, walking soft-footed up the track towards the wood. He told me he wore the sweater because navy-blue barely showed up in the dark, black was even better, he said. The peaked cap was important too, he explained, because the peak casts a shadow over one's face.", "Roald Dahl", "Danny, the Champion of the World"],
&["The telephone call came at ", "7.30", " on the evening of March 18th, a Saturday, the eve of the noisy, colourful festival that the town held in honour of Saint Joseph the carpenter -", "Leonardo Sciascia", "A Simple Story"],
]);
    minutes.insert(
        "19:35",
        &[&[
            "",
            "7.35",
            "-40. Yseut arrives at 'M. and S.', puts through phone call.",
            "Edmund Crispin",
            "The Case of the Gilded Fly",
        ]],
    );
    minutes.insert(
        "19:36",
        &[&[
            "",
            "7.35",
            "-40. Yseut arrives at 'M. and S.', puts through phone call.",
            "Edmund Crispin",
            "The Case of the Gilded Fly",
        ]],
    );
    minutes.insert(
        "19:37",
        &[&[
            "",
            "7.35",
            "-40. Yseut arrives at 'M. and S.', puts through phone call.",
            "Edmund Crispin",
            "The Case of the Gilded Fly",
        ]],
    );
    minutes.insert(
        "19:38",
        &[&[
            "",
            "7.35",
            "-40. Yseut arrives at 'M. and S.', puts through phone call.",
            "Edmund Crispin",
            "The Case of the Gilded Fly",
        ]],
    );
    minutes.insert(
        "19:39",
        &[&[
            "",
            "7.35",
            "-40. Yseut arrives at 'M. and S.', puts through phone call.",
            "Edmund Crispin",
            "The Case of the Gilded Fly",
        ]],
    );
    minutes.insert("19:40", &[
&["She arrives at ", "7.40", ", ten minutes late, but the children, Jimmy and Bitsy, are still eating supper and their parents are not ready to go yet. From other rooms come the sound of a baby screaming, water running, a television musical (no words: probably a dance number - patterns of gliding figures come to mind).", "Robert Coover", "The Babysitter"],
]);
    minutes.insert("19:41", &[
&["She arrives at ", "7.40", ", ten minutes late, but the children, Jimmy and Bitsy, are still eating supper and their parents are not ready to go yet. From other rooms come the sound of a baby screaming, water running, a television musical (no words: probably a dance number - patterns of gliding figures come to mind).", "Robert Coover", "The Babysitter"],
]);
    minutes.insert("19:42", &[
&["I glance at my watch as we speed along the Strand. ", "Seven forty-two", ". I'm starting to feel quite excited. The street outside is still bright and warm and tourists are walking along in T-shirts and shorts, pointing at the High Court. It must have been a gorgeous summer's day. Inside the air-conditioned Carter Spink building you have no idea what the weather in the real world is doing.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("19:43", &[
&["I glance at my watch as we speed along the Strand. ", "Seven forty-two", ". I'm starting to feel quite excited. The street outside is still bright and warm and tourists are walking along in T-shirts and shorts, pointing at the High Court. It must have been a gorgeous summer's day. Inside the air-conditioned Carter Spink building you have no idea what the weather in the real world is doing.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("19:44", &[
&["I glance at my watch as we speed along the Strand. ", "Seven forty-two", ". I'm starting to feel quite excited. The street outside is still bright and warm and tourists are walking along in T-shirts and shorts, pointing at the High Court. It must have been a gorgeous summer's day. Inside the air-conditioned Carter Spink building you have no idea what the weather in the real world is doing.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("19:45", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at ", "7:45", " or 6:30, Jasper, but pick times like 7:12 and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
&["He tells his old friend the train times and they settle on the ", "19.45", " arriving at 23.27. 'I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("19:46", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at ", "7:45", " or 6:30, Jasper, but pick times like 7:12 and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
&["He tells his old friend the train times and they settle on the ", "19.45", " arriving at 23.27. 'I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("19:47", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at ", "7:45", " or 6:30, Jasper, but pick times like 7:12 and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
&["He tells his old friend the train times and they settle on the ", "19.45", " arriving at 23.27. 'I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("19:48", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. \"Never meet people at ", "7:45", " or 6:30, Jasper, but pick times like 7:12 and 8:03!\"", "Steve Toltz", "A Fraction of the Whole"],
&["He tells his old friend the train times and they settle on the ", "19.45", " arriving at 23.27. 'I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("19:49", &[
&["There's a big, old-fashioned clock in the surgery. Just as Dr. Wellesley went out I heard the Moot Hall clock chime half-past seven, and then the chimes of St. Hathelswide's Church. I noticed that our clock was a couple of minutes slow, and I put it right.\" When did you next see Dr. Wellesley?\" \"At just ", "eleven minutes to eight", ".\" \"Where?\" \"In the surgery.\" \"He came back there?\" \"Yes.\" \"How do you fix that precise time--eleven minutes to eight?\" \"Because he'd arranged to see a patient in Meadow Gate at ten minutes to eight. I glanced at the clock as he came in, saw what time it was, and reminded him of the appointment.\"", "J. S. Fletcher", "In the Mayor's Parlour"],
]);
    minutes.insert("19:50", &[
&["At ", "ten to eight", ", he strolled downstairs, to make sure that Signora Buffi was not pottering around in the hall and that her door was not open, and to make sure there really was no one in Freddie's car", "Patricia Highsmith", "The Talented Mr Ripley"],
]);
    minutes.insert("19:51", &[
&["At ", "ten to eight", ", he strolled downstairs, to make sure that Signora Buffi was not pottering around in the hall and that her door was not open, and to make sure there really was no one in Freddie's car", "Patricia Highsmith", "The Talented Mr Ripley"],
]);
    minutes.insert("19:52", &[
&["He waited until ", "nearly eight", ", because around seven there were always more people coming in and out of the house than at other times. At ten to eight, he strolled downstairs, to make sure that Signora Buffi was not pottering around in the hall and that her door was not open, and to make sure there really was no one in Freddie's car, though he had gone down in the middle of the afternoon to look at the car and see if it was Freddie's.", "Patricia Highsmith", "The Talented Mr Ripley"],
]);
    minutes.insert("19:53", &[
&["Wednesday, 11 th December 1963. ", "7.53 p.m.", " \"Help me. You've got to help me.\" The woman's voice quavered on the edge of tears. The duty constable who had picked up the phone heard a hiccuping gulp, as if the caller was struggling to speak.", "Val McDermid", "A Place of Execution"],
]);
    minutes.insert("19:54", &[
&["The body was found at ", "six minutes to eight", ". Doctor Young arrived some thirty minutes later. Just let me get that clear - I've a filthy memory.", "Ngaio Marsh", "A Man Lay Dead"],
]);
    minutes.insert("19:55", &[
&["Flora drew her coat round her, and looked up into the darkening vault of the sky. Then she glanced at her watch. It was ", "five to eight", ".", "Stella Gibbons", "Cold Comfort Farm"],
]);
    minutes.insert("19:56", &[
&["I remember the cigarette in his hard face, against the now limitless storm cloud. Bernardo cried to him unexpectedly: 'What time is it, Ireno?' Without consulting the sky, without stopping, he replied: 'It's ", "four minutes to eight", ", young Bernardo Juan Franciso.' His voice was shrill, mocking.", "Jorge Luis Borges", "Funes the Memorious-Labyrinths"],
]);
    minutes.insert("19:57", &[
&["At ", "three minutes till eight", ", Laszlo and His Yankee Hussars set up onstage. While the band played their Sousa medley, Carter thoroughly checked his kit, stuffing his pockets with scarves, examining the seals on decks of cards. He glanced toward his levitation device. \"Good luck, Carter.\" The voice was quiet.", "Glen David Gold", "Carter Beats The Devil"],
]);
    minutes.insert(
        "19:58",
        &[&[
            "Robert Langdon stole an anxious glance at his wristwatch: ",
            "7.58pm.",
            " The smiling face of Mickey Mouse did little to cheer him up.",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert("19:59", &[
&["Kuniang made her appearance in my study ", "just before eight o' clock", ", arrayed in what had once ben a \"party frock\".", "Daniel Vare", "The Maker of Heavenly Trousers"],
&["Quickly, quickly. ", "A minute to eight.", " My hot water bottle was ready, and I filled a glass with water from the tap. Time was of the essence.", "Diane Setterfield", "The Thirteenth Tale"],
]);
    minutes.insert("20:00", &[
&["'TIS ", "eight o'clock", ",--a clear March night, The moon is up,--the sky is blue, The owlet, in the moonlight air, Shouts from nobody knows where; He lengthens out his lonely shout, Halloo! halloo! a long halloo!", "William Wordsworth", "The Idiot Boy"],
&["\"I trace the words, I'll arrive to collect you for drinks ", "at eight", " on Saturday.\"", "Chuck Palahniuk", "Tell-All"],
&["", "8.00 p.m.", " Have a bath", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Arthur thought he could even bear to listen to the album of bagpipe music he had won. It was ", "eight o'clock", " and he decided he would make himself, force himself, to listen to the whole record before he phoned her.", "Douglas Adams", "So Long, and Thanks for All the Fish"],
&["At ", "eight o'clock", " that evening, a Saturday, Pamela Chamcha stood with Jumpy Joshi - who had refused to let her go unaccompanied - next to the Photo-Me machine in a corner of the main concourse of Euston station, feeling ridiculously conspiratorial.", "Salman Rushdie", "Satanic Verses"],
&["Freud had me knock on Jung's door, to no avail. They waited until ", "eight", ", then set off for Brill's without him.", "Jed Rubenfeld", "The Interpretation Of Murder"],
&["I have been drunk just twice in my life, and the second time was that afternoon; so everything that happened has a dim, hazy cast over it. although until after ", "eight o'clock", " the apartment was full of cheerful sun.", "F. Scott Fitzgerald", "The Great Gatsby"],
&["It's the twenty-third of June nineteen seventy-five, and it is ", "eight o'clock", " in the evening, seated at his jigsaw puzzle, Bartlebooth has just died.", "Georges Perec", "Life: A User's Manual"],
&["She looked at her watch- it was ", "eight o'clock", "", "F. Scott Fitzgerald", "The Beautiful and Damned"],
&["That day he forgot to go to dinner; he noticed the fact ", "at eight", " in the evening, and as it was too late to go to the Rue St Jaques, he ate a lump of bread.", "Victor Hugo", "Les Miserables"],
&["The clock struck ", "eight", ". Had it been ten, Elinor would have been convinced that at that moment she heard a carriage driving up to the house; and so strong was the persuasion that she did, in spite of the almost impossibility of their being already come, that she moved into the adjoining dressing-closet and opened a window-shutter, to be satisfied of the truth. She instantly saw that her ears had not deceived her.", "Jane Austen", "Sense and Sensibility"],
]);
    minutes.insert(
        "20:01",
        &[&[
            "It was only ",
            "a little after eight o'clock",
            ", so all the shows were about silliness or murder.",
            "Kurt Vonnegut",
            "Slaughterhouse 5",
        ]],
    );
    minutes.insert("20:02", &[
&["\"Yes, I must go to the railway station, and if he's not there, then go there and catch him.\" Anna looked at the railway timetable in the newspapers. An evening train went at ", "two minutes past eight", ". \"Yes, I shall be in time.\"", "Leo Tolstoy", "Anna Karenina"],
]);
    minutes.insert("20:03", &[
&["He taught me that if I had to meet someone for an appointment, I must refuse to follow the 'stupid human habit' of arbitrarily choosing a time based on fifteen-minute intervals. 'Never meet people at 7:45 or 6:30, Jasper, but pick times like 7:12 and ", "8:03", "!'", "Steve Toltz", "A Fraction of the Whole"],
]);
    minutes.insert("20:04", &[
&["The earth seems to cast its darkness upward into the air. The farm country is somber at night. He is grateful when the lights of Lankaster merge with his dim beams. He stops at a diner who's clock says ", "8.04", ".", "John Updike", "Rabbit, Run"],
]);
    minutes.insert("20:05", &[
&["December 23rd At ", "8.05 pm", " Prof. Preobrazhensky commenced the first operation of its kind to be performed in Europe: removal under anaesthesia of a dog's testicles and their replacement by implanted human testes, with appendages and seminal ducts, taken from a 28-year-old human male", "Mikhail Bulgakov", "The Heart of a Dog"],
&["Ransom took out his watch, which he had adapted, on purpose, several hours before, to Boston time, and saw that the minutes had sped with increasing velocity during this interview, and that it now marked ", "five minutes past eight", ".", "Henry James", "The Bostonians"],
]);
    minutes.insert("20:06", &[
&["I have been drunk just twice in my life, and the second time was that afternoon; so everything that happened has a dim, hazy cast over it, although until ", "after eight o’clock", " the apartment was full of cheerful sun.", "F. Scott Fitzgerald", "The Great Gatsby"],
]);
    minutes.insert("20:07", &[
&["And I could hear that there were fewer people in the little station when the train wasn't there, so I opened my eyes and I looked at my watch and it said ", "8:07 pm", " and I had been sitting on the bench for approximately 5 hours but it hadn't seemed like approximately 5 hours, except that my bottom hurt and I was hungry and thirsty.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["Bennie pulled the transcripts for that night. The first call had come in at ", "8:07", ", with a positive ID.", "Lisa Scottoline", "Mistaken Identity"],
]);
    minutes.insert("20:08", &[
&["And I could hear that there were fewer people in the little station when the train wasn't there, so I opened my eyes and I looked at my watch and it said ", "8:07 pm", " and I had been sitting on the bench for approximately 5 hours but it hadn't seemed like approximately 5 hours, except that my bottom hurt and I was hungry and thirsty.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["Bennie pulled the transcripts for that night. The first call had come in at ", "8:07", ", with a positive ID.", "Lisa Scottoline", "Mistaken Identity"],
]);
    minutes.insert("20:09", &[
&["And I could hear that there were fewer people in the little station when the train wasn't there, so I opened my eyes and I looked at my watch and it said ", "8:07 pm", " and I had been sitting on the bench for approximately 5 hours but it hadn't seemed like approximately 5 hours, except that my bottom hurt and I was hungry and thirsty.", "Mark Haddon", "The Curious Incident of the Dog in the Night-Time"],
&["Bennie pulled the transcripts for that night. The first call had come in at ", "8:07", ", with a positive ID.", "Lisa Scottoline", "Mistaken Identity"],
]);
    minutes.insert("20:10", &[
&["At ", "2010h.", " on 1 April Y.D.A.U., the medical attache is still watching the unlabelled entertainment cartridge.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("20:11", &[
&["At ", "2010h.", " on 1 April Y.D.A.U., the medical attache is still watching the unlabelled entertainment cartridge.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("20:12", &[
&["At ", "2010h.", " on 1 April Y.D.A.U., the medical attache is still watching the unlabelled entertainment cartridge.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("20:13", &[
&["At ", "2010h.", " on 1 April Y.D.A.U., the medical attache is still watching the unlabelled entertainment cartridge.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("20:14", &[
&["When a call came through to Dilworth’s home number at ", "fourteen minutes past eight", " o’clock, Olbier and Jones reacted with far more excitement than the situation warranted because they were desperate for action.", "Dean Koontz", "Watchers"],
]);
    minutes.insert("20:15", &[
&["", "8:15 p.m.", " Cannot locate operating instructions (for video)", "Helen Fielding", "Bridget Jones's Diary"],
&["", "8.15 p.m.", " Get changed into pyjamas", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Natsha: I was looking to see if there wasn't a fire. It's Shrovetide, and the servant is simply beside herself; I must look out that something doesn't happen. When I came through the dining-room yesterday midnight, there was a candle burning. I couldn't get her to tell me who had lighted it. [Puts down her candle] What's the time? Andrey: [Looks at his watch] A ", "quarter past eight", ". Natasha: And Olga and Irina aren't in yet. The poor things are still at work. Olga at the teachers' council, Irina at the telegraph office...[sighs] I said to your sister this morning, \"Irina, darling, you must take care of yourself.\" But she pays no attention. Did you say it was a quarter past eight?", "Anton Chekhov", "The Three Sisters"],
]);
    minutes.insert("20:16", &[
&["He kissed her hand and after a while went to get two more drinks. When he got back, it was ", "sixteen minutes past eight", ", and Lois was humming softly along with the jukebox", "Jonathan Coe", "The Rotters' Club"],
]);
    minutes.insert("20:17", &[
&["", "20.17", " A red warning light failed to go on in the Drive Room, beginning a chain of events which would lead, in a further twenty-three minutes, to the total annihilation of the entire crew of Red Dwarf.", "Grant Naylor", "Red Dwarf"],
]);
    minutes.insert("20:18", &[
&["", "2018 hrs", " Katya has arrived at the Odessa Hotel. Barley and Katya are talking in the canteen. Wicklow and one irregular observing. More.", "John le Carre", "The Russia House"],
]);
    minutes.insert("20:19", &[
&["", "2018 hrs", " Katya has arrived at the Odessa Hotel. Barley and Katya are talking in the canteen. Wicklow and one irregular observing. More.", "John le Carre", "The Russia House"],
]);
    minutes.insert("20:20", &[
&["", "8.20 p.m.", " Play computer games", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "20.20", " all ships had completed oiling. Hove to, they had had the utmost difficulty in keeping position in that great wind; but they were infinitely safer than in the open sea", "Alistair MacLean", "H.M.S. Ulysses"],
&["Knowing that the dinner was only for us six, we never dreamed it would be a full dress affair. I had no appetite. It was quite ", "twenty minutes past eight", " before we sat down to dinner.", "George and Weedon Grossmith", "Diary of a Nobody"],
]);
    minutes.insert("20:21", &[
&["At ", "8.21", ", after a knock at the door, a constable said a military police vehicle had just driven into the courtyard, the driver asking for \"Mr.\" Murray.", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert("20:22", &[
&["At ", "8.21", ", after a knock at the door, a constable said a military police vehicle had just driven into the courtyard, the driver asking for \"Mr.\" Murray.", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert("20:23", &[
&["", "20.23.", " In a few minutes she would go down.She could have borrowed some mascara from her daughter Sally, but it was too late. She could have rung her mother in Northam, but it was too late. Seven minutes of solitude she had, and then she would descend.", "Margaret Drabble", "The Radiant Way"],
]);
    minutes.insert(
        "20:24",
        &[&[
            "Peach checked his watch. ",
            "8.24.",
            " If he wasn't in a taxi in twenty minutes he'd be done for.",
            "Rupert Thomson",
            "Dreams of Leaving",
        ]],
    );
    minutes.insert("20:25", &[
&["She sat down in her usual seat and smiled at her husband as he sank into his own chair opposite her. She was saved. It was only ", "five and twenty past eight", ".", "Agatha Christie", "The Listerdale Mystery"],
]);
    minutes.insert("20:26", &[
&["She sat down in her usual seat and smiled at her husband as he sank into his own chair opposite her. She was saved. It was only ", "five and twenty past eight", ".", "Agatha Christie", "The Listerdale Mystery"],
]);
    minutes.insert("20:27", &[
&["At ", "seven-and-twenty minutes past eight", " Mrs Lofthouse was seated at Aurora's piano, in the first agonies of a prelude in six flats; a prelude which demanded such extraordinary uses of the left hand across the right, and the right over the left, and such exercise of the thumbs in all positions", "Mary Elizabeth Braddon", "Aurora Floyd"],
]);
    minutes.insert("20:28", &[
&["At ", "seven-and-twenty minutes past eight", " Mrs Lofthouse was seated at Aurora's piano, in the first agonies of a prelude in six flats; a prelude which demanded such extraordinary uses of the left hand across the right, and the right over the left, and such exercise of the thumbs in all positions", "Mary Elizabeth Braddon", "Aurora Floyd"],
]);
    minutes.insert("20:29", &[
&["\"", "Twenty-nine and a half minutes past eight", ", sir.\" And then, from habit, he glanced at the clock in the tower, and made further oration. \"By George! that clock's half an hour fast! First time in ten years I've known it to be off. This watch of mine never varies a--\" But the citizen was talking to vacancy. He turned and saw his hearer, a fast receding black shadow, flying in the direction of a house with three lighted upper windows.", "O. Henry", "The Four Million"],
]);
    minutes.insert("20:30", &[
&["Alix took up a piece of needlework and began to stitch. Gerald read a few pages of his book. Then he glanced up at the clock and tossed the book away. \"", "Half-past eight", ". Time to go down to the cellar and start work.\"", "Agatha Christie", "The Listerdale Mystery"],
&["The bicycles go by in twos and threes - there's a dance on in Billy Brennan's barn tonight, and there's the half-talk code of mysteries and the wink-and-elbow language of delight. ", "Half-past eight", " and there is not a spot upon a mile of road, no shadow thrown that might turn out a man or woman,", "Patrick Kavanagh", "Inniskeen Road: July Evening"],
]);
    minutes.insert("20:31", &[
&["Alix took up a piece of needlework and began to stitch. Gerald read a few pages of his book. Then he glanced up at the clock and tossed the book away. \"", "Half-past eight", ". Time to go down to the cellar and start work.\"", "Agatha Christie", "The Listerdale Mystery"],
&["The bicycles go by in twos and threes - there's a dance on in Billy Brennan's barn tonight, and there's the half-talk code of mysteries and the wink-and-elbow language of delight. ", "Half-past eight", " and there is not a spot upon a mile of road, no shadow thrown that might turn out a man or woman,", "Patrick Kavanagh", "Inniskeen Road: July Evening"],
]);
    minutes.insert("20:32", &[
&["At the station he captured Miss Lantry out of the gadding mob at ", "eight thirty-two", ". \"We mustn't keep mamma and the others waiting,\" said she. \"To Wallack's Theatre as fast as you can drive!\"", "O. Henry", "The Four Million"],
]);
    minutes.insert("20:33", &[
&["", "20.33", " Navigation officer Henri DuBois knocked his black cona coffee with four sugars over his computer console keyboard. As he mopped up the coffee, he noticed three red warning blips on his monitor screen, which he wrongly assumed were the result of his spillage.", "Grant Naylor", "Red Dwarf"],
]);
    minutes.insert("20:34", &[
&["", "20.33", " Navigation officer Henri DuBois knocked his black cona coffee with four sugars over his computer console keyboard. As he mopped up the coffee, he noticed three red warning blips on his monitor screen, which he wrongly assumed were the result of his spillage.", "Grant Naylor", "Red Dwarf"],
]);
    minutes.insert("20:35", &[
&["", "8:35pm.", " Found operating instructions under Hello.", "Helen Fielding", "Bridget Jones's Diary"],
&["Left Munich at ", "8.35 p.m.", " on 1st May, arriving at Vienna early the next morning", "Bram Stoker", "Dracula"],
&["She paused reflectively. He was keenly interested now, not a doubt of it. The murderer is bound to have an interest in murder. She had gambled on that, and succeeded. She stole a glance at the clock. It was ", "five and twenty to nine", ".", "Agatha Christie", "The Listerdale mystery"],
]);
    minutes.insert(
        "20:36",
        &[&[
            "",
            "20.36",
            " Rimmer stood in the main wash-room on the stasis deck and combed his hair.",
            "Grant Naylor",
            "Red Dwarf",
        ]],
    );
    minutes.insert(
        "20:37",
        &[&[
            "",
            "20.36",
            " Rimmer stood in the main wash-room on the stasis deck and combed his hair.",
            "Grant Naylor",
            "Red Dwarf",
        ]],
    );
    minutes.insert(
        "20:38",
        &[&[
            "",
            "20.36",
            " Rimmer stood in the main wash-room on the stasis deck and combed his hair.",
            "Grant Naylor",
            "Red Dwarf",
        ]],
    );
    minutes.insert(
        "20:39",
        &[&[
            "",
            "20.36",
            " Rimmer stood in the main wash-room on the stasis deck and combed his hair.",
            "Grant Naylor",
            "Red Dwarf",
        ]],
    );
    minutes.insert("20:40", &[
&["It was when I stood before her, avoiding her eyes, that I took note of the surrounding objects in detail, and saw that her watch had stopped at ", "twenty minutes to nine", ", and that a clock in the room had stopped at twenty minutes to nine.", "Charles Dickens", "Great Expectations"],
&["The letter had been brought in at ", "twenty minutes to nine", ".", "Agatha Christie", "The Murder of Roger Ackroyd"],
]);
    minutes.insert("20:41", &[
&["It was when I stood before her, avoiding her eyes, that I took note of the surrounding objects in detail, and saw that her watch had stopped at ", "twenty minutes to nine", ", and that a clock in the room had stopped at twenty minutes to nine.", "Charles Dickens", "Great Expectations"],
&["The letter had been brought in at ", "twenty minutes to nine", ".", "Agatha Christie", "The Murder of Roger Ackroyd"],
]);
    minutes.insert("20:42", &[
&["The hand at this moment pointed to ", "8.42", ". The players took up their cards, but their eyes were constantly on the clock. One may safely say that, however secure they might feel, never had minutes seemed so long to them.", "Jules Verne", "Around the World in Eighty Days"],
]);
    minutes.insert("20:43", &[
&["'", "8.43", ",' said Thomas Flanagan, as he cut the cards placed before him by Gauthier Ralph. There was a moment's pause, during which the spacious room was perfectly silent.", "Jules Verne", "Around the world in eighty days"],
]);
    minutes.insert("20:44", &[
&["The clock's pendulum beat every second with mathematical regularity, and each player could count every sixtieth of a minute as it struck his ear.\"", "8.44!", "\" said John Sullivan, in a voice that betrayed his emotion.Only one minute more and the wager would be won.", "Jules Verne", "Around the World in Eighty Days"],
]);
    minutes.insert("20:45", &[
&["'It's not impossible,' Phileas said quietly.'I bet you 20,000 pounds I could do it. If I leave this evening on the ", "8.45", " train to Dover, I can be back here at the Reform Club by 8.45 on Saturday 21 December. I'll get my passport stamped at every place i stop to prove I've been around the world.'", "Jules Verne", "Around the World in Eighty Days"],
&["Beaver arrived at ", "quarter to nine", " in a state of high self-approval; he had refused two invitations for dinner while dressing that evening; he had cashed a cheque for ten pounds at his club; he had booked a Divan table at Espinosa's.", "Evelyn Waugh", "A Handful of Dust"],
]);
    minutes.insert(
        "20:46",
        &[&[
            "At the tone, the time will be ",
            "eight forty six",
            ", exactly. One cubic mile of seawater contains about 50 pounds of gold.",
            "Tom Lichtenberg",
            "Macedonia",
        ]],
    );
    minutes.insert(
        "20:47",
        &[&[
            "At the tone, the time will be ",
            "eight forty six",
            ", exactly. One cubic mile of seawater contains about 50 pounds of gold.",
            "Tom Lichtenberg",
            "Macedonia",
        ]],
    );
    minutes.insert(
        "20:48",
        &[&[
            "At the tone, the time will be ",
            "eight forty six",
            ", exactly. One cubic mile of seawater contains about 50 pounds of gold.",
            "Tom Lichtenberg",
            "Macedonia",
        ]],
    );
    minutes.insert("20:49", &[
&["", "8.49", ". I took the phone, cleared my throat, and dialled the keep, the packs stronghold on the outskirts of Atlanta. Just keep it professional. Less pathetic that way.", "Ilona Andrews", "Magic Bleeds"],
]);
    minutes.insert("20:50", &[
&["", "8.50pm.", " Ah Diagram \"Buttons for IMC functions\". But what are IMC functions?", "Helen Fielding", "Bridget Jones's Diary"],
&["all the clocks in London were striking ", "ten minutes before nine", ".", "Jules Verne", "Around the world in eighty days"],
&["He glanced at the bracket-clock on the mantelpiece, but as this had stopped, drew out his watch. 'It is already too late,' he said. 'It wants only ", "ten minutes to nine", ".' 'Good God!' she exclaimed, turning quite pale. 'What am I to do?'", "Georgette Heyer", "The Reluctant Widow"],
&["He was, yes, always home from work by ", "2050", " on Thursdays.", "David Foster Wallace", "Infinite Jest"],
&["What did it mean by beginning to tick so loudly all of a sudden? Its face indicated ", "ten minutes to nine", ". Mrs Verloc cared nothing for time, and the ticking went on.", "Joseph Conrad", "The Secret Agent"],
]);
    minutes.insert("20:51", &[
&["", "8.50pm.", " Ah Diagram \"Buttons for IMC functions\". But what are IMC functions?", "Helen Fielding", "Bridget Jones's Diary"],
&["all the clocks in London were striking ", "ten minutes before nine", ".", "Jules Verne", "Around the world in eighty days"],
&["He glanced at the bracket-clock on the mantelpiece, but as this had stopped, drew out his watch. 'It is already too late,' he said. 'It wants only ", "ten minutes to nine", ".' 'Good God!' she exclaimed, turning quite pale. 'What am I to do?'", "Georgette Heyer", "The Reluctant Widow"],
&["He was, yes, always home from work by ", "2050", " on Thursdays.", "David Foster Wallace", "Infinite Jest"],
&["What did it mean by beginning to tick so loudly all of a sudden? Its face indicated ", "ten minutes to nine", ". Mrs Verloc cared nothing for time, and the ticking went on.", "Joseph Conrad", "The Secret Agent"],
]);
    minutes.insert("20:52", &[
&["", "8.50pm.", " Ah Diagram \"Buttons for IMC functions\". But what are IMC functions?", "Helen Fielding", "Bridget Jones's Diary"],
&["all the clocks in London were striking ", "ten minutes before nine", ".", "Jules Verne", "Around the world in eighty days"],
&["He glanced at the bracket-clock on the mantelpiece, but as this had stopped, drew out his watch. 'It is already too late,' he said. 'It wants only ", "ten minutes to nine", ".' 'Good God!' she exclaimed, turning quite pale. 'What am I to do?'", "Georgette Heyer", "The Reluctant Widow"],
&["He was, yes, always home from work by ", "2050", " on Thursdays.", "David Foster Wallace", "Infinite Jest"],
&["What did it mean by beginning to tick so loudly all of a sudden? Its face indicated ", "ten minutes to nine", ". Mrs Verloc cared nothing for time, and the ticking went on.", "Joseph Conrad", "The Secret Agent"],
]);
    minutes.insert("20:53", &[
&["Only ", "eight fifty-three", ". The partners' decision meeting starts in seven minutes. I'm not sure I can bear this.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("20:54", &[
&["Only ", "eight fifty-three", ". The partners' decision meeting starts in seven minutes. I'm not sure I can bear this.", "Sophie Kinsella", "The Undomestic Goddess"],
]);
    minutes.insert("20:55", &[
&["And the past. The clock on the dash said ", "8:55pm.", " And the last pink shard of the sun was reaching up into the night sky, desperately trying to hold on for just one more minute.", "Chris Cole", "Such Great Heights"],
]);
    minutes.insert(
        "20:56",
        &[&[
            "“No. 7 berth—a second-class. The gentleman has not yet come, and it is ",
            "four minutes to nine",
            ".\"",
            "Agatha Christie",
            "Murder on the Orient Express",
        ]],
    );
    minutes.insert("20:57", &[
&["\"Wait,\" he said solemnly, \"till the clock strikes. I have wealth and power and knowledge above most men, but when the clock strikes I am afraid. Stay by me until then. This woman shall be yours. You have the word of the hereditary Prince of Valleluna. On the day of your marriage I will give you $100,000 and a palace on the Hudson. But there must be no clocks in that palace--they measure our follies and limit our pleasures. Do you agree to that?\" \"Of course,\" said the young man, cheerfully, \"they're a nuisance, anyway--always ticking and striking and getting you late for dinner.\" He glanced again at the clock in the tower. The hands stood at ", "three minutes to nine", ".", "O. Henry", "The Four Million"],
]);
    minutes.insert(
        "20:58",
        &[&[
            "\"What time is it?\" she asked, quiet, definite, hopeless. \"",
            "Two minutes to nine",
            ",\" he replied, telling the truth with a struggle.",
            "D H Lawrence",
            "Sons and Lovers",
        ]],
    );
    minutes.insert(
        "20:59",
        &[&[
            "\"What time is it?\" she asked, quiet, definite, hopeless. \"",
            "Two minutes to nine",
            ",\" he replied, telling the truth with a struggle.",
            "D H Lawrence",
            "Sons and Lovers",
        ]],
    );
    minutes.insert("21:00", &[
&["", "9.00 p.m.", " Watch television or a video", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["At ", "2100", " at night it's cold out.", "David Foster Wallace", "Infinite Jest"],
&["It was ", "nine o'clock", " at night upon the second of August—the most terrible August in the history of the world. One might have thought already that God's curse hung heavy over a degenerate world, for there was an awesome hush and a feeling of vague expectancy in the sultry and stagnant air", "Arthur Conan Doyle", "His Last Bow An Epilogue of Sherlock Holmes"],
&["On the evening before K.'s thirty-first birthday - it was about ", "nine o'clock", ", when there is a lull in the streets - two gentlemen came to his apartment.", "Franz Kafka", "The Trial"],
&["On the stroke of ", "nine o’clock", " Mr. and Mrs. De Voted took their places on either side of the drawing-room fire, in attitudes of gracefully combined hospitality and unconcern, Vivian De Voted wearing a black beard and black velvet jacket buttoned over his Bohemian bosom, his lady in a flowing purple gown embroidered in divers appropriate places with pomegranates and their leaves.", "Katherine Mansfield", "A Marriage of Passion"],
&["Shortly after ", "nine o'clock", " that evening, Weyrother drove with his plans to Kutuzov's quarters where the council of war was to be held. All the commanders of columns were summoned to the commander in chief's and with the exception of Prince Bagration, who declined to come, were all there at the appointed time.", "Leo Tolstoy", "War and Peace"],
&["Standing in the chrome-and-tile desolation of the Polar-Shtern Kafeteria at ", "nine o'clock", " on a Friday night, in a snowstorm, he's the loneliest Jew in the Sitka District.", "Michael Chabon", "The Yiddish Policemen's Union"],
&["That night ", "at nine", " the President addressed the nation.", "Bernstein & Woodward", "All the President's Men"],
&["Then he put on a grey jacket and left the flat to make his way to Praca da Alegria. It was already ", "nine o'clock", ", Pereira maintains.", "Antonio Tabucchi", "Pereira Maintains"],
&["This time, the putting on of her best hat at ", "nine o'clock", " at night with the idea of sallying forth from the castle, down the long drive and then northwards along the acacia avenue, had been enough to send her to her own doorway as though she suspected someone might be there, someone who was listening to her thoughts.", "Mervyn Peake", "Titus Groan"],
]);
    minutes.insert(
        "21:01",
        &[&[
            "On the evening before K.'s thirty-first birthday - it was ",
            "about nine o'clock",
            ", when there is a lull in the streets - two gentlemen came to his apartment.",
            "Franz Kafka",
            "The Trial",
        ]],
    );
    minutes.insert("21:02", &[
&["The good Brants did not allow the boys to play out ", "after nine", " in summer evenings; they were sent to bed at that hour; Eddie honorably remained, but Georgie usually slipped out of the window toward ten, and enjoyed himself until midnight.", "Mark Twain", "Edward Mills and George Benton: A Tale"],
]);
    minutes.insert("21:03", &[
&["Billy Weaver had travelled down from London on the slow afternoon train, with a change at Swindon on the way, and by the time he got to Bath it was ", "about nine o’clock", " in the evening and the moon was coming up out of a clear starry sky over the houses opposite the station entrance. But the air was deadly cold and the wind was like a flat blade of ice on his cheeks.", "Roald Dahl", "“The Landlady”"],
]);
    minutes.insert("21:04", &[
&["At ", "9.04pm", " trilobites swim onto the scene, followed more or less immediately by the shapely creatures of the Burgess Shale.", "Bill Bryson", "A Short History of Nearly Everything"],
]);
    minutes.insert("21:05", &[
&["", "Nine-five", ". A voice spoke from the study ceiling: \"Mrs. McClellan, which poem would you like this evening?\". The house was silent. The voice said at last, \"Since you express no preference, I shall select a poem at random.\"", "Ray Bradbury", "There will come soft rains"],
]);
    minutes.insert("21:06", &[
&["", "Nine-five", ". A voice spoke from the study ceiling: \"Mrs. McClellan, which poem would you like this evening?\". The house was silent. The voice said at last, \"Since you express no preference, I shall select a poem at random.\"", "Ray Bradbury", "There will come soft rains"],
]);
    minutes.insert("21:07", &[
&["", "Nine-five", ". A voice spoke from the study ceiling: \"Mrs. McClellan, which poem would you like this evening?\". The house was silent. The voice said at last, \"Since you express no preference, I shall select a poem at random.\"", "Ray Bradbury", "There will come soft rains"],
]);
    minutes.insert("21:08", &[
&["", "Nine-five", ". A voice spoke from the study ceiling: \"Mrs. McClellan, which poem would you like this evening?\". The house was silent. The voice said at last, \"Since you express no preference, I shall select a poem at random.\"", "Ray Bradbury", "There will come soft rains"],
]);
    minutes.insert(
        "21:09",
        &[&[
            "",
            "9.09",
            ". Too late to turn around and go back. Too late, too dangerous.",
            "Rupert Thomson",
            "Dreams of Leaving",
        ]],
    );
    minutes.insert(
        "21:10",
        &[&[
            "",
            "9.09",
            ". Too late to turn around and go back. Too late, too dangerous.",
            "Rupert Thomson",
            "Dreams of Leaving",
        ]],
    );
    minutes.insert("21:11", &[
&["Every few seconds the house changed character, at one time menacing and sinister, and again the innocent abode of law-abiding citizens about to be attacked by my private army. The luminous watch said ", "9.11", ".", "Len Deighton", "The Ipcress File"],
]);
    minutes.insert("21:12", &[
&["The crime was reported to us (with almost indecent alacrity, Rog) at ", "21.12", ", by Susan Trott - of Black Grouse Cottage - who had been, I quote: 'out looking for hedgehogs when I was horrified to notice the postbox door had fallen off and was just lying there, on the ground'.", "Nicola Barker", "Burley Cross Postbox Theft"],
]);
    minutes.insert("21:13", &[
&["The crime was reported to us (with almost indecent alacrity, Rog) at ", "21.12", ", by Susan Trott - of Black Grouse Cottage - who had been, I quote: 'out looking for hedgehogs when I was horrified to notice the postbox door had fallen off and was just lying there, on the ground'.", "Nicola Barker", "Burley Cross Postbox Theft"],
]);
    minutes.insert("21:14", &[
&["The crime was reported to us (with almost indecent alacrity, Rog) at ", "21.12", ", by Susan Trott - of Black Grouse Cottage - who had been, I quote: 'out looking for hedgehogs when I was horrified to notice the postbox door had fallen off and was just lying there, on the ground'.", "Nicola Barker", "Burley Cross Postbox Theft"],
]);
    minutes.insert("21:15", &[
&["", "9.15", ". Did Roberts pay you yet?", "James Joyce", "Ulysses"],
&["What are we going to do? Should we try to walk to Clapham High Street? But it's bloody miles away. I glance at my watch and am shocked to see that it's ", "nine-fifteen", ". We've spent over an hour faffing about and we haven't even had a drink. And it's all my fault. I can't even organize one simple evening without its going catastrophically wrong.", "Sophie Kinsella", "Can You Keep a Secret?"],
]);
    minutes.insert("21:16", &[
&["", "9.15", ". Did Roberts pay you yet?", "James Joyce", "Ulysses"],
&["What are we going to do? Should we try to walk to Clapham High Street? But it's bloody miles away. I glance at my watch and am shocked to see that it's ", "nine-fifteen", ". We've spent over an hour faffing about and we haven't even had a drink. And it's all my fault. I can't even organize one simple evening without its going catastrophically wrong.", "Sophie Kinsella", "Can You Keep a Secret?"],
]);
    minutes.insert("21:17", &[
&["", "21:17", ", Sunday Evening, Angbyplan. A man is observed outside the hair salon. He presses his face and hands against the glass, and appears extremely intoxicated.", "John Ajvide Lindqvist", "Let The Right One In"],
]);
    minutes.insert("21:18", &[
&["The same thing would hold true if there were someone in her apartment. In that case he would just say that he had been passing by, recognized her charming house, and thought to drop in. It was ", "eighteen minutes after nine", " when Mr. Martin turned into Twelfth Street.", "James Thurber", "The Catbird Seat"],
]);
    minutes.insert("21:19", &[
&["The same thing would hold true if there were someone in her apartment. In that case he would just say that he had been passing by, recognized her charming house, and thought to drop in. It was ", "eighteen minutes after nine", " when Mr. Martin turned into Twelfth Street.", "James Thurber", "The Catbird Seat"],
]);
    minutes.insert(
        "21:20",
        &[&[
            "",
            "9.20 p.m.",
            " Have juice and a snack",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-time",
        ]],
    );
    minutes.insert(
        "21:21",
        &[&[
            "",
            "9.20 p.m.",
            " Have juice and a snack",
            "Mark Haddon",
            "The Curious Incident of the Dog in the Night-time",
        ]],
    );
    minutes.insert("21:22", &[
&["Fifteen minutes later (", "21.22 hrs", "), Miss Squire arrives in Skipton where she is booked into a local B&B. This B&B is located directly across the road from Mhairi Callaghan's Feathercuts.", "Nicola Barker", "Burley Cross Postbox Theft"],
]);
    minutes.insert("21:23", &[
&["My father met me at the station, the dog jumped up to meet me, missed, and nearly fell in front of the ", "9.23pm", " Birmingham express.", "Sue Townsend", "The Secret Diary of Adrian Mole Aged 13 3/4"],
]);
    minutes.insert("21:24", &[
&["My father met me at the station, the dog jumped up to meet me, missed, and nearly fell in front of the ", "9.23pm", " Birmingham express.", "Sue Townsend", "The Secret Diary of Adrian Mole Aged 13 3/4"],
]);
    minutes.insert("21:25", &[
&["", "9:25 p.m.", " Aargh. Suddenly main menu is on TV saying Press 6. Realize was using telly remote control by mistake. Now News has come on", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("21:26", &[
&["", "9:25 p.m.", " Aargh. Suddenly main menu is on TV saying Press 6. Realize was using telly remote control by mistake. Now News has come on", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("21:27", &[
&["", "9:25 p.m.", " Aargh. Suddenly main menu is on TV saying Press 6. Realize was using telly remote control by mistake. Now News has come on", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert(
        "21:28",
        &[&[
            "From that moment on--",
            "9:28",
            " in the evening, June 18, 1941--everything was different.",
            "Jonathan Safran Foer",
            "Everything is Illuminated",
        ]],
    );
    minutes.insert(
        "21:29",
        &[&[
            "From that moment on--",
            "9:28",
            " in the evening, June 18, 1941--everything was different.",
            "Jonathan Safran Foer",
            "Everything is Illuminated",
        ]],
    );
    minutes.insert("21:30", &[
&["", "9.30 p.m.", " Go to bed", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
&["Forty-eight years old, profoundly asleep at ", "nine thirty", " on a Friday night - this is modern professional life.", "Ian McEwan", "Saturday"],
&["It's ", "9:30 p.m.", " already. I've gotta head uptown for my appointment with Pavel. Pavel is my shrink. He sees patients at night. He's a Czech Jew, a survivor of Terezin and Auswitz. I see him once a week.", "Art Spiegelman", "Maus"],
&["The light in Mr. Green's kitchen snapped off at ", "nine-thirty", ", followed by the light in his bedroom at his usual ten o'clock. His house was the first on the street to go dark.", "Suzanne Berne", "A crime in the neighborhood"],
]);
    minutes.insert("21:31", &[
&["I took some juice out of the refrigerator and sat down at the kitchen table with it. On the table was a note from my girlfriend: \"Gone out to eat. Back by 9:30.\" The digital clock on the table read 9:30. I watched it flip over to ", "9:31", ", then to 9:32.", "Haruki Murakami", "A Wild Sheep Chase"],
]);
    minutes.insert("21:32", &[
&["I took some juice out of the refrigerator and sat down at the kitchen table with it. On the table was a note from my girlfriend: \"Gone out to eat. Back by 9:30.\" The digital clock on the table read 9:30. I watched it flip over to 9:31, then to ", "9:32", ".", "Haruki Murakami", "A Wild Sheep Chase"],
]);
    minutes.insert("21:33", &[
&["I took some juice out of the refrigerator and sat down at the kitchen table with it. On the table was a note from my girlfriend: \"Gone out to eat. Back by 9:30.\" The digital clock on the table read 9:30. I watched it flip over to 9:31, then to ", "9:32", ".", "Haruki Murakami", "A Wild Sheep Chase"],
]);
    minutes.insert("21:34", &[
&["Thanks; expect me ", "9.34 p.m.", " 26th'; which produced, three hours later, a reply: 'Delighted; please bring a No. 3 Rippingille stove' - a perplexing and ominous direction, which somehow chilled me in spite of its subject matter.", "Erskine Childers", "The Riddle of the Sands"],
]);
    minutes.insert(
        "21:35",
        &[&[
            "The Sergeant jotted it down on a piece of paper. 'That checks up with his own story: ",
            "9.35 p.m.",
            " Budd leaves; the North dame arrives.'",
            "Georgette Heyer",
            "A Blunt Instrument",
        ]],
    );
    minutes.insert("21:36", &[
&["My backpack was already packed, and I'd already gotten the other supplies together, like the altimeter and the granola bars and the Swiss army knife I'd dug up in Central Park, so there was nothing else to do. Mom tucked me in at ", "9:36", ".", "Jonathan Safran Foer", "Extremely Loud and Incredibly Close"],
]);
    minutes.insert("21:37", &[
&["My backpack was already packed, and I'd already gotten the other supplies together, like the altimeter and the granola bars and the Swiss army knife I'd dug up in Central Park, so there was nothing else to do. Mom tucked me in at ", "9:36", ".", "Jonathan Safran Foer", "Extremely Loud and Incredibly Close"],
]);
    minutes.insert("21:38", &[
&["At ", "nine thirty-eight", " the waiter came back and offered us a second helping of cheese,salami and sardines, and Mr Yoshogi who had been converting sterling into yen looked extremely puzzled and said he had no idea that British Honduras had so large an export trade", "David Footman", "Pig and Pepper"],
]);
    minutes.insert("21:39", &[
&["At ", "nine thirty-eight", " the waiter came back and offered us a second helping of cheese,salami and sardines, and Mr Yoshogi who had been converting sterling into yen looked extremely puzzled and said he had no idea that British Honduras had so large an export trade", "David Footman", "Pig and Pepper"],
]);
    minutes.insert("21:40", &[
&["At ", "nine thirty-eight", " the waiter came back and offered us a second helping of cheese,salami and sardines, and Mr Yoshogi who had been converting sterling into yen looked extremely puzzled and said he had no idea that British Honduras had so large an export trade", "David Footman", "Pig and Pepper"],
]);
    minutes.insert("21:41", &[
&["At ", "nine thirty-eight", " the waiter came back and offered us a second helping of cheese,salami and sardines, and Mr Yoshogi who had been converting sterling into yen looked extremely puzzled and said he had no idea that British Honduras had so large an export trade", "David Footman", "Pig and Pepper"],
]);
    minutes.insert(
        "21:42",
        &[&[
            "Langdon looked at his Mickey Mouse watch. ",
            "9:42 P.M.",
            "",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert(
        "21:43",
        &[&[
            "Langdon looked at his Mickey Mouse watch. ",
            "9:42 P.M.",
            "",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert(
        "21:44",
        &[&[
            "Langdon looked at his Mickey Mouse watch. ",
            "9:42 P.M.",
            "",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert("21:45", &[
&["But for some unfathomable reason-birth, death, the end of the universe and all things available to man-Cody Menhoff's was closed at ", "9:45 PM", " on a Thursday...", "T. C. Boyle", "Riven Rock"],
]);
    minutes.insert("21:46", &[
&["But for some unfathomable reason-birth, death, the end of the universe and all things available to man-Cody Menhoff's was closed at ", "9:45 PM", " on a Thursday...", "T. C. Boyle", "Riven Rock"],
]);
    minutes.insert(
        "21:47",
        &[&[
            "For Hunter, who was trained to note times exactly, the final emergency started at ",
            "thirteen minutes to ten",
            ".",
            "CJ Driver",
            "Elegy for a Revolutionary",
        ]],
    );
    minutes.insert(
        "21:48",
        &[&[
            "For Hunter, who was trained to note times exactly, the final emergency started at ",
            "thirteen minutes to ten",
            ".",
            "CJ Driver",
            "Elegy for a Revolutionary",
        ]],
    );
    minutes.insert(
        "21:49",
        &[&[
            "For Hunter, who was trained to note times exactly, the final emergency started at ",
            "thirteen minutes to ten",
            ".",
            "CJ Driver",
            "Elegy for a Revolutionary",
        ]],
    );
    minutes.insert(
        "21:50",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:51",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:52",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:53",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:54",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:55",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert(
        "21:56",
        &[&[
            "I passed out on to the road and saw by the lighted dial of a clock that it was ",
            "ten minutes to ten",
            ". In front of me was a large building which displayed the magical name.",
            "James Joyce",
            "Dubliners",
        ]],
    );
    minutes.insert("21:57", &[
&["Second to last, the inset clock blinks from ", "21:57", " to 21:58. Napier's eyes sink, newborn sunshine slants through ancient oaks and on a lost river. Look, Joe, herons", "David Mitchell", "Cloud Atlas"],
&["The waiting man pulled out a handsome watch, the lids of it set with small diamonds. \"", "Three minutes to ten", ",\" he announced.", "O. Henry", "The Four Million"],
]);
    minutes.insert("21:58", &[
&["Second to last, the inset clock blinks from 21:57 to ", "21:58", ". Napier's eyes sink, newborn sunshine slants through ancient oaks and on a lost river. Look, Joe, herons", "David Mitchell", "Cloud Atlas"],
]);
    minutes.insert("21:59", &[
&["The first night, as soon as the corporal had conducted my uncle Toby upstairs, which was ", "about 10", " - Mrs. Wadman threw herself into her arm chair", "Laurence Sterne", "The Life and Opinions of Tristram Shandy, Gentleman"],
]);
    minutes.insert("22:00", &[
&["By ", "ten", ", Quoyle was drunk. The crowd was enormous, crushed together so densely that Nutbeem could not force his way down the hall or to the door and urinated on the remaining potato chips in the blue barrel, setting a popular example.", "E. Annie Proulx", "The Shipping News"],
&["Her body asserted itself with a restless movement of the knee, and she stood up. '", "Ten o'clock", ",' she remarked, apparently finding the time on the ceiling. 'Time for this good girl to go to bed.'", "F. Scott Fitzgerald", "The Great Gatsby"],
&["I could not doubt that this was the BLACK SPOT; and taking it up, I found writ", "ten", " on the other side, in a very good, clear hand, this short message: \"You have till ten tonight.\"", "Robert Louis Stevenson", "Treasure Island"],
&["I went back into the library, limp and exhausted. In a few minutes the telephone began ringing again. I did not do anything. I let it ring. I went and sat down at Maxim's feet. It went on ringing. I did not move. Presently it stopped, as though cut suddenly in exasperation. The clock on the mantelpiece struck ", "ten o'clock", ". Maxim put his arms round me and lifted me against him. We began to kiss one another, feverishly, desperately, like guilty lovers who have not kissed before.", "Daphne du Maurier", "Rebecca"],
&["No one wanted to go to bed when at ", "ten o'clock", " Mrs. March put by the last finished job, and said, \"Come girls.\" Beth went to the piano and played the father's favorite hymn.", "Louisa May Alcott", "Little Women"],
&["The grandfather clock in the State Room strikes ", "ten", " times.", "David Mitchell", "The Thousand Autumns of Jacob de Zoet"],
&["The light in Mr. Green's kitchen snapped off at nine-thirty, followed by the light in his bedroom at his usual ", "ten o'clock", ". His house was the first on the street to go dark.", "Suzanne Berne", "A crime in the neighborhood"],
&["They were alone then, and theoretically free to do whatever they wanted, but they went on eating the dinner they had no appetite for. Florence set down her knife and reached for Edward's hand and squeezed. From downstairs they heard the wireless, the chimes of Big Ben at the start of the ", "ten o'clock", " news.", "Ian McEwan", "On Chesil Beach"],
&["We let our upstairs room to a certain Mr. Goudsmit, a divorced man in his thirties, who appeared to have nothing to do on this particular evening; we simply could not get rid of him without being rude; he hung about until ", "ten o'clock", ".", "Anne Frank", "Anne Frank: The Diary of a Young Girl"],
]);
    minutes.insert("22:01", &[
&["By ", "ten", ", Quoyle was drunk. The crowd was enormous, crushed together so densely that Nutbeem could not force his way down the hall or to the door and urinated on the remaining potato chips in the blue barrel, setting a popular example.", "E. Annie Proulx", "The Shipping News"],
&["Her body asserted itself with a restless movement of the knee, and she stood up. '", "Ten o'clock", ",' she remarked, apparently finding the time on the ceiling. 'Time for this good girl to go to bed.'", "F. Scott Fitzgerald", "The Great Gatsby"],
&["I could not doubt that this was the BLACK SPOT; and taking it up, I found written on the other side, in a very good, clear hand, this short message: \"You have till ", "ten", " tonight.\"", "Robert Louis Stevenson", "Treasure Island"],
&["I went back into the library, limp and exhausted. In a few minutes the telephone began ringing again. I did not do anything. I let it ring. I went and sat down at Maxim's feet. It went on ringing. I did not move. Presently it stopped, as though cut suddenly in exasperation. The clock on the mantelpiece struck ", "ten o'clock", ". Maxim put his arms round me and lifted me against him. We began to kiss one another, feverishly, desperately, like guilty lovers who have not kissed before.", "Daphne du Maurier", "Rebecca"],
&["No one wanted to go to bed when at ", "ten o'clock", " Mrs. March put by the last finished job, and said, \"Come girls.\" Beth went to the piano and played the father's favorite hymn.", "Louisa May Alcott", "Little Women"],
&["The grandfather clock in the State Room strikes ", "ten", " times.", "David Mitchell", "The Thousand Autumns of Jacob de Zoet"],
&["The light in Mr. Green's kitchen snapped off at nine-thirty, followed by the light in his bedroom at his usual ", "ten o'clock", ". His house was the first on the street to go dark.", "Suzanne Berne", "A crime in the neighborhood"],
&["They were alone then, and theoretically free to do whatever they wanted, but they went on eating the dinner they had no appetite for. Florence set down her knife and reached for Edward's hand and squeezed. From downstairs they heard the wireless, the chimes of Big Ben at the start of the ", "ten o'clock", " news.", "Ian McEwan", "On Chesil Beach"],
&["We let our upstairs room to a certain Mr. Goudsmit, a divorced man in his thirties, who appeared to have nothing to do on this particular evening; we simply could not get rid of him without being rude; he hung about until ", "ten o'clock", ".", "Anne Frank", "Anne Frank: The Diary of a Young Girl"],
]);
    minutes.insert(
        "22:02",
        &[&[
            "It was now ",
            "10.02pm.",
            " He has less than two hours.",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert(
        "22:03",
        &[&[
            "It was now ",
            "10.02pm.",
            " He has less than two hours.",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert(
        "22:04",
        &[&[
            "It was now ",
            "10.02pm.",
            " He has less than two hours.",
            "Dan Brown",
            "The Lost Symbol",
        ]],
    );
    minutes.insert("22:05", &[
&["The A-B elevator was our elevator, the elevator on which the paramedics came up at 9:20 p.m., the elevator on which they took John (and me) downstairs to the ambulance at ", "10:05 p.m.", "", "Joan Didion", "The Year of Magical Thinking"],
]);
    minutes.insert("22:06", &[
&["Of course, they had good reason to be fussy on such a night. And then it was long ", "after ten o'clock", " and yet there was no sign of Gabriel and his wife. Besides they were dreadfully afraid that Freddy Malins might turn up screwed.", "James Joyce", "The Dead"],
]);
    minutes.insert("22:07", &[
&["Of course, they had good reason to be fussy on such a night. And then it was long ", "after ten o'clock", " and yet there was no sign of Gabriel and his wife. Besides they were dreadfully afraid that Freddy Malins might turn up screwed.", "James Joyce", "The Dead"],
]);
    minutes.insert(
        "22:08",
        &[&[
            "\"My watch is always a little fast,\" I said. \"What time do you make it now?\" \"",
            "Ten eight",
            ".\" \"Ten eighteen by mine. You see.\"",
            "Graham Greene",
            "The Quiet American",
        ]],
    );
    minutes.insert(
        "22:09",
        &[&[
            "\"My watch is always a little fast,\" I said. \"What time do you make it now?\" \"",
            "Ten eight",
            ".\" \"Ten eighteen by mine. You see.\"",
            "Graham Greene",
            "The Quiet American",
        ]],
    );
    minutes.insert("22:10", &[
&["That was the past, and now I had just died on the narrow couch of a Paris lodging house, and my wife was crouching on the floor, crying bitterly. The white light before my left eye was growing dim, but I remembered the room perfectly. On the left there was a chest of drawers, on the right a mantelpiece surmounted by a damaged clock without a pendulum, the hands of which marked ", "ten minutes past ten", ". The window overlooked the Rue Dauphine, a long, dark street. All Paris seemed to pass below, and the noise was so great that the window shook.", "Emile Zola", "The Death of Olivier Becaille"],
&["", "10.10pm.", " When you turn your recorder on you must adjust clock and the calendar.......Press red and nothing happens. Press numbers and nothing happens. Wish stupid video had never been invented.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:11", &[
&["Therefore a sergeant called Trifonov had been on post all day or all week and then he had left at ", "eleven minutes past ten", " in the evening.", "Lee Child", "The Enemy"],
]);
    minutes.insert("22:12", &[
&["The Chinese women scuttled at an amazing rate, given their size and the bags' size. It was c. ", "2212", ":30-40h., smack in the middle of the former Interval of Issues Resolution.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:13", &[
&["The Chinese women scuttled at an amazing rate, given their size and the bags' size. It was c. ", "2212", ":30-40h., smack in the middle of the former Interval of Issues Resolution.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:14", &[
&["The shopping bags looked heavy and impressive, their weight making the Chinese women lean in slightly towards each other. Call it ", "2214", ":10h.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert(
        "22:15",
        &[&[
            "",
            "10:15 p.m.",
            " Aargh Newsnight on in 15 minutes",
            "Helen Fielding",
            "Bridget Jones's Diary",
        ]],
    );
    minutes.insert(
        "22:16",
        &[&[
            "",
            "10:15 p.m.",
            " Aargh Newsnight on in 15 minutes",
            "Helen Fielding",
            "Bridget Jones's Diary",
        ]],
    );
    minutes.insert(
        "22:17",
        &[&[
            "",
            "10:17 p. m.",
            " Casette will not go in",
            "Helen Fielding",
            "Bridget Jones's Diary",
        ]],
    );
    minutes.insert("22:18", &[
&["\"My watch is always a little fast,\" I said. \"What time do you make it now?\" \"Ten eight.\" \"", "Ten eighteen", " by mine. You see.\"", "Graham Greene", "The Quiet American"],
&["", "10:18pm.", " Ah. Thelma and Louise is in there", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:19", &[
&["\"My watch is always a little fast,\" I said. \"What time do you make it now?\" \"Ten eight.\" \"", "Ten eighteen", " by mine. You see.\"", "Graham Greene", "The Quiet American"],
&["", "10:18pm.", " Ah. Thelma and Louise is in there", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:20", &[
&["At ", "10:20", " she returned with a shopping bag from the supermarket. In the bag were three scrub brushes, one box of paperclips and a well-chilled six-pack of canned beer. So I had another beer. \"It was about sheep,\" I said. \"Didn't I tell you?\" she said.", "Haruki Murakami", "A Wild Sheep Chase"],
]);
    minutes.insert("22:21", &[
&["", "10:21pm.", " Frenziedly press all buttons. Cassette comes out and goes back in again", "Helen Fielding", "Bridget Jones's Diary"],
&["", "10:21pm.", " Thelma and Louise will not come out", "Helen Fielding", "Bridget Jones's Diary"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually, and went in with Green and ducked Green and went back outside to # 3's lawn and put the thing in a pocket and went in and put it down the garbage disposal in the kitchen sink of the kitchen, but still felt largely impotent and unresolved.", "David Foster Wallace", "Infinite Jest"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually...", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:22", &[
&["", "10:21pm.", " Frenziedly press all buttons. Cassette comes out and goes back in again", "Helen Fielding", "Bridget Jones's Diary"],
&["", "10:21pm.", " Thelma and Louise will not come out", "Helen Fielding", "Bridget Jones's Diary"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually, and went in with Green and ducked Green and went back outside to # 3's lawn and put the thing in a pocket and went in and put it down the garbage disposal in the kitchen sink of the kitchen, but still felt largely impotent and unresolved.", "David Foster Wallace", "Infinite Jest"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually...", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:23", &[
&["", "10:21pm.", " Frenziedly press all buttons. Cassette comes out and goes back in again", "Helen Fielding", "Bridget Jones's Diary"],
&["", "10:21pm.", " Thelma and Louise will not come out", "Helen Fielding", "Bridget Jones's Diary"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually, and went in with Green and ducked Green and went back outside to # 3's lawn and put the thing in a pocket and went in and put it down the garbage disposal in the kitchen sink of the kitchen, but still felt largely impotent and unresolved.", "David Foster Wallace", "Infinite Jest"],
&["On a Saturday c. ", "2221h", "., Lenz found a miniature bird that had fallen out of some nest and was sitting bald and pencil-necked on the lawn of Unit #3 flapping ineffectually...", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:24", &[
&["Thanks to ten minutes or so of balmy weather, by ", "10:24", " the Earth is covered in the great carboniferous forests whose residues give us all our coal, and the first winged insects are evident.", "Bill Bryson", "A Short History of Nearly Everything"],
]);
    minutes.insert("22:25", &[
&["", "10:25pm.", " Got new cassette in now. Right. Turn to \"Recording.................. Aargh Newsnight is starting", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:26", &[
&["As always, consciousness returned to me progressively from the edges of my field of vision. The first things to claim recognition were the bathroom door emerging from the far right and a lamp from the far left, from which my awareness gradually drifted inward like ice flowing together toward the middle of a lake. In the exact center of my visual field was the alarm clock, hands pointing to ", "ten-twenty-six", ".", "Haruki Murakami", "Hard Boiled Wonderland and the End of the World"],
]);
    minutes.insert("22:27", &[
&["Mr Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses and biscuits.", "AG Macdonell", "England, Their England"],
]);
    minutes.insert("22:28", &[
&["Mr Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses and biscuits.", "AG Macdonell", "England, Their England"],
]);
    minutes.insert("22:29", &[
&["Mr Harcourt woke up with mysterious suddenness at ", "twenty-seven minutes past 10", ", and, by a curious coincidence, it was at that very instant that the butler came in with two footmen laden with trays of whisky, brandy, syphons, glasses and biscuits.", "AG Macdonell", "England, Their England"],
]);
    minutes.insert("22:30", &[
&["She looked at the clock; it was ", "ten thirty", ". If she could get there quickly on the subway, then she could be at his house in less than an hour, maybe a bit longer if the late trains did not come so often.", "Colm Toibin", "Brooklyn"],
&["The time was ", "ten-thirty", " but it could have been three in the morning, because along its borders, West Berlin goes to bed with the dark", "John Le Carre", "Smiley's People"],
]);
    minutes.insert("22:31", &[
&["", "10.31pm.", " Ok OK. Calm. Penny Husbands-Bosworth, so asbestos leukaemia item is not on yet.", "Helen Fielding", "Bridget Jones's Diary"],
&["And, later on, at ", "10.31 pm", ", I went out onto the balcony to find out whether I could see any stars, but there weren't any because of all the clouds and what is called Light Pollution which is light from streetlights and car headlights and floodlights and lights in buildings reflecting off tiny particles in the atmosphere and getting in the way of light from the stars. So I went back inside.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("22:32", &[
&["", "10.31pm.", " Ok OK. Calm. Penny Husbands-Bosworth, so asbestos leukaemia item is not on yet.", "Helen Fielding", "Bridget Jones's Diary"],
&["And, later on, at ", "10.31 pm", ", I went out onto the balcony to find out whether I could see any stars, but there weren't any because of all the clouds and what is called Light Pollution which is light from streetlights and car headlights and floodlights and lights in buildings reflecting off tiny particles in the atmosphere and getting in the way of light from the stars. So I went back inside.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("22:33", &[
&["", "10:33 p.m.", " Yessss, yessss. RECORDING CURRENT PROGRAMME. Have done it. Aargh. All going mad. Cassette has started rewinding and now stopped and ejected. Why? Shit. Shit. Realize in excitement have sat on remote control.", "Helen Fielding", "Bridget Jones's Diary"],
&["", "10:33pm.", " Yessss, yessss. RECORDING CURRENT PROGRAMME. Have done it. Aargh. All going mad. Cassette has started rewinding and now stopped and ejected. Why? Shit. Shit. Realize in excitement have sat on remote control.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:34", &[
&["", "10:33 p.m.", " Yessss, yessss. RECORDING CURRENT PROGRAMME. Have done it. Aargh. All going mad. Cassette has started rewinding and now stopped and ejected. Why? Shit. Shit. Realize in excitement have sat on remote control.", "Helen Fielding", "Bridget Jones's Diary"],
&["", "10:33pm.", " Yessss, yessss. RECORDING CURRENT PROGRAMME. Have done it. Aargh. All going mad. Cassette has started rewinding and now stopped and ejected. Why? Shit. Shit. Realize in excitement have sat on remote control.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:35", &[
&["", "10:35 p.m.", " Frantic now. Have rung Sahzzer, Rebecca, Simon, Magda. Nobody knows how to programme their videos. Only person I know who knows how to do it is Daniel.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:36", &[
&["", "10:35 p.m.", " Frantic now. Have rung Sahzzer, Rebecca, Simon, Magda. Nobody knows how to programme their videos. Only person I know who knows how to do it is Daniel.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:37", &[
&["", "10:35 p.m.", " Frantic now. Have rung Sahzzer, Rebecca, Simon, Magda. Nobody knows how to programme their videos. Only person I know who knows how to do it is Daniel.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:38", &[
&["", "10:35 p.m.", " Frantic now. Have rung Sahzzer, Rebecca, Simon, Magda. Nobody knows how to programme their videos. Only person I know who knows how to do it is Daniel.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:39", &[
&["", "10:35 p.m.", " Frantic now. Have rung Sahzzer, Rebecca, Simon, Magda. Nobody knows how to programme their videos. Only person I know who knows how to do it is Daniel.", "Helen Fielding", "Bridget Jones's Diary"],
]);
    minutes.insert("22:40", &[
&["The station clock told him the time: ", "twenty to eleven", ". He went to the booking office and asked the clerk in a polite tone when was the next train to Paris. 'In twelve minutes.'", "Georges Simenon", "The Man Who Watched the Trains Go By"],
]);
    minutes.insert("22:41", &[
&["He climbed into the front seat and started the car. It started with a merry powerful hum, ready to go. \"There, the bastards\", said Julian, and smashed the clock with the bottom of the bottle, to give them an approximate time. It was ", "10:41", "", "John O'Hara", "Appointment in Samarra"],
]);
    minutes.insert("22:42", &[
&["He climbed into the front seat and started the car. It started with a merry powerful hum, ready to go. \"There, the bastards\", said Julian, and smashed the clock with the bottom of the bottle, to give them an approximate time. It was ", "10:41", "", "John O'Hara", "Appointment in Samarra"],
]);
    minutes.insert("22:43", &[
&["He climbed into the front seat and started the car. It started with a merry powerful hum, ready to go. \"There, the bastards\", said Julian, and smashed the clock with the bottom of the bottle, to give them an approximate time. It was ", "10:41", "", "John O'Hara", "Appointment in Samarra"],
]);
    minutes.insert("22:44", &[
&["Alec pricked up his ears. \"When was that?\" \"Oh, yesterday evening.\" \"What time?\" \"", "About a quarter to eleven.", " I was playing bridge.\"", "Carola Dunn", "Dead in the Water"],
]);
    minutes.insert("22:45", &[
&["", "10.45pm.", " Oh God Daniel fell about laughing when I said I could not programme video. Said he would do it for me. Still at least I have done best for Mum. It is exciting and historic when one's friends are on TV.", "Helen Fielding", "Bridget Jones's Diary"],
&["So the Lackadaisical Broadcasting Co. bids you farewell with the message that if you aren't grateful to be living in a world where so many things to be grateful for are yours as a matter of course. Why it is now five seconds until ", "fifteen minutes before eleven", " o'clock and you are just an old Trojan Horse.", "Ogden Nash", "Good Intentions"],
]);
    minutes.insert("22:46", &[
&["The 'night train' tallied to perfection, for high tide in the creek would be, as Davies estimated, between 10.30 and 11.00 p.m.on the night of the 25th; and the time-table showed that the only night train arriving at Norden was one from the south at ", "10.46 p.m.", "", "Erskine Childers", "The Riddle of the Sands"],
]);
    minutes.insert("22:47", &[
&["The 'night train' tallied to perfection, for high tide in the creek would be, as Davies estimated, between 10.30 and 11.00 p.m.on the night of the 25th; and the time-table showed that the only night train arriving at Norden was one from the south at ", "10.46 p.m.", "", "Erskine Childers", "The Riddle of the Sands"],
]);
    minutes.insert("22:48", &[
&["\"Oh! I don't know about that,\" said Mr. Satterthwaite, warming to his subject. \"I was down there for a bit last summer. I found it quite convenient for town. Of course the trains only go every hour. 48 minutes past the hour from Waterloo-up to ", "10.48", ".\"", "Agatha Christie", "The Sign in the Sky"],
]);
    minutes.insert("22:49", &[
&["It's ", "well after 2245h", ". The dog's leash slides hissing to the end of the Day-Glo line and stops the dog a couple of paces from the inside of the gate, where Lenz is standing, inclined in the slight forward way of somebody who's talking baby-talk to a dog.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("22:50", &[
&["", "10.50 P. M.", " This diary-keeping of mine is, I fancy, the outcome of that scientific habit of mind about which I wrote this morning. I like to register impressions while they are fresh.", "Sir Arthur Conan Doyle", "The Parasite"],
&["Saturday night. And I said, 'It's a hundred this year, ain't anybody noticed?'\"Jack said, 'What's a hundred?' I said, 'Pub is. Coach is. Look at the clock.' Jack said, ‘It's ", "ten to eleven", "’.", "Graham Swift", "Last Orders"],
&["So think yourself lucky while you're awake and remember a happy crew. Think of Hamburg on the Magic Night. ", "22.50", " and they went out neatly, just as they should - you couldn't fault Parks, he was always on his route.", "A. L. Kennedy", "Day"],
]);
    minutes.insert("22:51", &[
&["", "10.50 P. M.", " This diary-keeping of mine is, I fancy, the outcome of that scientific habit of mind about which I wrote this morning. I like to register impressions while they are fresh.", "Sir Arthur Conan Doyle", "The Parasite"],
&["Saturday night. And I said, 'It's a hundred this year, ain't anybody noticed?'\"Jack said, 'What's a hundred?' I said, 'Pub is. Coach is. Look at the clock.' Jack said, ‘It's ", "ten to eleven", "’.", "Graham Swift", "Last Orders"],
&["So think yourself lucky while you're awake and remember a happy crew. Think of Hamburg on the Magic Night. ", "22.50", " and they went out neatly, just as they should - you couldn't fault Parks, he was always on his route.", "A. L. Kennedy", "Day"],
]);
    minutes.insert("22:52", &[
&["", "10.50 P. M.", " This diary-keeping of mine is, I fancy, the outcome of that scientific habit of mind about which I wrote this morning. I like to register impressions while they are fresh.", "Sir Arthur Conan Doyle", "The Parasite"],
&["Saturday night. And I said, 'It's a hundred this year, ain't anybody noticed?'\"Jack said, 'What's a hundred?' I said, 'Pub is. Coach is. Look at the clock.' Jack said, ‘It's ", "ten to eleven", "’.", "Graham Swift", "Last Orders"],
&["So think yourself lucky while you're awake and remember a happy crew. Think of Hamburg on the Magic Night. ", "22.50", " and they went out neatly, just as they should - you couldn't fault Parks, he was always on his route.", "A. L. Kennedy", "Day"],
]);
    minutes.insert("22:53", &[
&["", "10.50 P. M.", " This diary-keeping of mine is, I fancy, the outcome of that scientific habit of mind about which I wrote this morning. I like to register impressions while they are fresh.", "Sir Arthur Conan Doyle", "The Parasite"],
&["Saturday night. And I said, 'It's a hundred this year, ain't anybody noticed?'\"Jack said, 'What's a hundred?' I said, 'Pub is. Coach is. Look at the clock.' Jack said, ‘It's ", "ten to eleven", "’.", "Graham Swift", "Last Orders"],
&["So think yourself lucky while you're awake and remember a happy crew. Think of Hamburg on the Magic Night. ", "22.50", " and they went out neatly, just as they should - you couldn't fault Parks, he was always on his route.", "A. L. Kennedy", "Day"],
]);
    minutes.insert("22:54", &[
&["", "10.50 P. M.", " This diary-keeping of mine is, I fancy, the outcome of that scientific habit of mind about which I wrote this morning. I like to register impressions while they are fresh.", "Sir Arthur Conan Doyle", "The Parasite"],
&["Saturday night. And I said, 'It's a hundred this year, ain't anybody noticed?'\"Jack said, 'What's a hundred?' I said, 'Pub is. Coach is. Look at the clock.' Jack said, ‘It's ", "ten to eleven", "’.", "Graham Swift", "Last Orders"],
&["So think yourself lucky while you're awake and remember a happy crew. Think of Hamburg on the Magic Night. ", "22.50", " and they went out neatly, just as they should - you couldn't fault Parks, he was always on his route.", "A. L. Kennedy", "Day"],
]);
    minutes.insert("22:55", &[
&["\"It is eleven o'clock! ", "Eleven o'clock, all but five minutes!", "\" \"But which eleven o'clock?\" \"The eleven o'clock that is to decide life or death!...He told me so just before he went....He is terrible....He is quite mad: he tore off his mask and his yellow eyes shot flames!...\"", "Gaston Leroux", "The Phantom of the Opera"],
]);
    minutes.insert("22:56", &[
&["\"It is eleven o'clock! ", "Eleven o'clock, all but five minutes!", "\" \"But which eleven o'clock?\" \"The eleven o'clock that is to decide life or death!...He told me so just before he went....He is terrible....He is quite mad: he tore off his mask and his yellow eyes shot flames!...\"", "Gaston Leroux", "The Phantom of the Opera"],
]);
    minutes.insert("22:57", &[
&["\"It is eleven o'clock! ", "Eleven o'clock, all but five minutes!", "\" \"But which eleven o'clock?\" \"The eleven o'clock that is to decide life or death!...He told me so just before he went....He is terrible....He is quite mad: he tore off his mask and his yellow eyes shot flames!...\"", "Gaston Leroux", "The Phantom of the Opera"],
]);
    minutes.insert("22:58", &[
&["Then it grew dark; she would have had them to bed, but they begged sadly to be allowed to stay up; and, ", "just about eleven o’clock", ", the door-latch was raised quietly, and in stepped the master.", "Emily Brontë", "Wuthering Heights"],
]);
    minutes.insert("22:59", &[
&["They parked the car outside Lowther's at precisely ", "one minute to eleven", ". People were leaving, not all of them happy at having their evening curtailed. But the grumbling was muted, and even then it only started once they were safely on the street.", "Ian Rankin", "The Complaints"],
]);
    minutes.insert("23:00", &[
&["'He will be here ", "at eleven", " exactly, sir.' At the bar, naked couples had begun dancing.", "John le Carre", "Smiley's People"],
&["At ", "eleven o'clock", " that night, having secured a bed at one of the hotels and telegraphed his address to his father immediately on his arrival, he walked out into the streets of Sandbourne.", "Thomas Hardy", "Tess of the d'Urbervilles"],
&["At ", "eleven o'clock", ", I rang the bell for Betteredge, and told Mr. Blake that he might at last prepare himself for bed.", "Wilkie Collins", "The Moonstone"],
&["He says, \"They've killed Jan. Clear out.\" \"The suitcase?\" I ask. \"Take it away again. We want nothing to do with it now. Catch the ", "eleven o'clock", " express.\" \"But it doesn't stop here....\" \"It will. Go to track six. Opposite the freight station. You have three minutes.\" \"But...\" \"Move, or I'll have to arrest you.\"", "Italo Calvino", "If on a winter's night a traveller"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The train arrived in New York ", "at eleven", " that night.", "William Gaddis", "The Recognitions"],
&["They didn't even sit down to eat until ", "2300h", ".", "David Foster Wallace", "Infinite Jest"],
&["When they reached the top of the Astronomy Tower at ", "eleven o'clock", ", they found a perfect night for stargazing, cloudless and still.", "JK Rowling", "Harry Potter and the Order of the Phoenix"],
]);
    minutes.insert("23:01", &[
&["'He will be here ", "at eleven", " exactly, sir.' At the bar, naked couples had begun dancing.", "John le Carre", "Smiley's People"],
&["At ", "eleven o'clock", " that night, having secured a bed at one of the hotels and telegraphed his address to his father immediately on his arrival, he walked out into the streets of Sandbourne.", "Thomas Hardy", "Tess of the d'Urbervilles"],
&["At ", "eleven o'clock", ", I rang the bell for Betteredge, and told Mr. Blake that he might at last prepare himself for bed.", "Wilkie Collins", "The Moonstone"],
&["He says, \"They've killed Jan. Clear out.\" \"The suitcase?\" I ask. \"Take it away again. We want nothing to do with it now. Catch the ", "eleven o'clock", " express.\" \"But it doesn't stop here....\" \"It will. Go to track six. Opposite the freight station. You have three minutes.\" \"But...\" \"Move, or I'll have to arrest you.\"", "Italo Calvino", "If on a winter's night a traveller"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The train arrived in New York ", "at eleven", " that night.", "William Gaddis", "The Recognitions"],
&["They didn't even sit down to eat until ", "2300h", ".", "David Foster Wallace", "Infinite Jest"],
&["When they reached the top of the Astronomy Tower at ", "eleven o'clock", ", they found a perfect night for stargazing, cloudless and still.", "JK Rowling", "Harry Potter and the Order of the Phoenix"],
]);
    minutes.insert("23:02", &[
&["'He will be here ", "at eleven", " exactly, sir.' At the bar, naked couples had begun dancing.", "John le Carre", "Smiley's People"],
&["At ", "eleven o'clock", " that night, having secured a bed at one of the hotels and telegraphed his address to his father immediately on his arrival, he walked out into the streets of Sandbourne.", "Thomas Hardy", "Tess of the d'Urbervilles"],
&["At ", "eleven o'clock", ", I rang the bell for Betteredge, and told Mr. Blake that he might at last prepare himself for bed.", "Wilkie Collins", "The Moonstone"],
&["He says, \"They've killed Jan. Clear out.\" \"The suitcase?\" I ask. \"Take it away again. We want nothing to do with it now. Catch the ", "eleven o'clock", " express.\" \"But it doesn't stop here....\" \"It will. Go to track six. Opposite the freight station. You have three minutes.\" \"But...\" \"Move, or I'll have to arrest you.\"", "Italo Calvino", "If on a winter's night a traveller"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The clock struck ", "eleven", ". I looked at Adele, whose head leant against my shoulder; her eyes were waxing heavy, so I took her up in my arms and carried her off to bed. It was near one before the gentlemen and ladies sought their chambers.", "Charlotte Brontë", "Jane Eyre"],
&["The train arrived in New York ", "at eleven", " that night.", "William Gaddis", "The Recognitions"],
&["They didn't even sit down to eat until ", "2300h", ".", "David Foster Wallace", "Infinite Jest"],
&["When they reached the top of the Astronomy Tower at ", "eleven o'clock", ", they found a perfect night for stargazing, cloudless and still.", "JK Rowling", "Harry Potter and the Order of the Phoenix"],
]);
    minutes.insert("23:03", &[
&["\"What makes you think it's for real?\" \"Just a hunch, really. He sounded for real. Sometimes you can just tell about people\"-he smiled-\"even if you're a dull old WASP.\" \"I think it's a setup.\" \"Why?\" \"I just do. Why would someone from the government want to help you?\" \"Good question. Guess I'll find out.\" She went back into the kitchen.\"What time are you meeting him?\" she called out. \"", "Eleven oh-three", ",\" he said. \"That made me think he's for real. Military and intelligence types set precise appointment times to eliminate confusion and ambiguity. Nothing ambiguous about eleven oh-three.\"", "Christopher Buckley", "Little Green Men"],
]);
    minutes.insert("23:04", &[
&["\"What makes you think it's for real?\" \"Just a hunch, really. He sounded for real. Sometimes you can just tell about people\"-he smiled-\"even if you're a dull old WASP.\" \"I think it's a setup.\" \"Why?\" \"I just do. Why would someone from the government want to help you?\" \"Good question. Guess I'll find out.\" She went back into the kitchen.\"What time are you meeting him?\" she called out. \"", "Eleven oh-three", ",\" he said. \"That made me think he's for real. Military and intelligence types set precise appointment times to eliminate confusion and ambiguity. Nothing ambiguous about eleven oh-three.\"", "Christopher Buckley", "Little Green Men"],
]);
    minutes.insert("23:05", &[
&["It was ", "11.05", ", five minutes later than my habitual bedtime. I felt. I felt guilty at being still up, but the past kept pricking at me and I knew that all the events of those nineteen days in July were astir within me, like the loosening phlegm in an attack of bronchitis", "L.P.Hartley", "The Go-Between"],
&["It was ", "five minutes past eleven", " when I made my last entry. I remember winding up my watch and noting the time. So I have wasted some five hours of the little span still left to us. Who would have believed it possible? But I feel very much fresher, and ready for my fate--or try to persuade myself that I am. And yet, the fitter a man is, and the higher his tide of life, the more must he shrink from death. How wise and how merciful is that provision of nature by which his earthly anchor is usually loosened by many little imperceptible tugs, until his consciousness has drifted out of its untenable earthly harbor into the great sea beyond!", "Sir Arthur Conan Doyle", "The Poison Belt"],
&["My watch says ", "11:05", ". But whether AM or PM I don't know.", "Stanley Donwood", "Household Worms"],
]);
    minutes.insert("23:06", &[
&["It was ", "11.05", ", five minutes later than my habitual bedtime. I felt. I felt guilty at being still up, but the past kept pricking at me and I knew that all the events of those nineteen days in July were astir within me, like the loosening phlegm in an attack of bronchitis", "L.P.Hartley", "The Go-Between"],
&["It was ", "five minutes past eleven", " when I made my last entry. I remember winding up my watch and noting the time. So I have wasted some five hours of the little span still left to us. Who would have believed it possible? But I feel very much fresher, and ready for my fate--or try to persuade myself that I am. And yet, the fitter a man is, and the higher his tide of life, the more must he shrink from death. How wise and how merciful is that provision of nature by which his earthly anchor is usually loosened by many little imperceptible tugs, until his consciousness has drifted out of its untenable earthly harbor into the great sea beyond!", "Sir Arthur Conan Doyle", "The Poison Belt"],
&["My watch says ", "11:05", ". But whether AM or PM I don't know.", "Stanley Donwood", "Household Worms"],
]);
    minutes.insert("23:07", &[
&["At ", "11.07 pm", ", Samuel \"Gunner\" Wilson was moving at 645 miles per hour over the Mojave Desert. Up ahead in the moonlinght, he saw the twin lead jets, their afterburners glowing angrily in the night sky.", "Michael Crichton", "The Andromeda Strain"],
]);
    minutes.insert("23:08", &[
&["At ", "11.07 pm", ", Samuel \"Gunner\" Wilson was moving at 645 miles per hour over the Mojave Desert. Up ahead in the moonlinght, he saw the twin lead jets, their afterburners glowing angrily in the night sky.", "Michael Crichton", "The Andromeda Strain"],
]);
    minutes.insert("23:09", &[
&["At ", "11.07 pm", ", Samuel \"Gunner\" Wilson was moving at 645 miles per hour over the Mojave Desert. Up ahead in the moonlinght, he saw the twin lead jets, their afterburners glowing angrily in the night sky.", "Michael Crichton", "The Andromeda Strain"],
]);
    minutes.insert("23:10", &[
&["Another Christmas day is nearly over. It's ", "ten past eleven", ". Richard declined with thanks my offer to make up a bed for him here in my study, and has driven off back to Cambridge, so I am able to make some notes on the day before going to bed myself.", "David Lodge", "Deaf Sentence"],
&["He had not the strength to help himself, and at ", "ten minutes past eleven", " no one could have helped him, no one in the world", "John O'Hara", "Appointment in Samarra"],
]);
    minutes.insert("23:11", &[
&["Life changes fast Life changes in an instant You sit down to dinner and life as you know it ends. The Question of self-pity. Those were the first words I wrote after it happened. The computer dating on the Microsoft Word file (\"Notes on change.doc\") reads \"May 20, 2004, ", "11:11 p.m.", ",\" but that would have been a case of my opening the file and reflexively pressing save when I closed it. I had made no changes to that file since I wrote the words, in January 2004, a day or two after the fact. For a long time I wrote nothing else. Life changes in the instant. The ordinary instant.", "Joan Didion", "The Year of Magical Thinking"],
]);
    minutes.insert(
        "23:12",
        &[&[
            "There was a confirmatory identification done by undercover officer 6475 at ",
            "23:12",
            " hours at the corner of 147th and Amsterdam.",
            "Sergio De La Pava",
            "A Naked Singularity",
        ]],
    );
    minutes.insert(
        "23:13",
        &[&[
            "There was a confirmatory identification done by undercover officer 6475 at ",
            "23:12",
            " hours at the corner of 147th and Amsterdam.",
            "Sergio De La Pava",
            "A Naked Singularity",
        ]],
    );
    minutes.insert(
        "23:14",
        &[&[
            "There was a confirmatory identification done by undercover officer 6475 at ",
            "23:12",
            " hours at the corner of 147th and Amsterdam.",
            "Sergio De La Pava",
            "A Naked Singularity",
        ]],
    );
    minutes.insert("23:15", &[
&["", "11.15pm.", " Humph. Mum just rang \"Sorry, darling. It isn't Newsnigtht, it's Breakfast News tomorrow. Could you set it for seven o'clock tomorrow morning, BBC1?\"", "Helen Fielding", "Bridget Jones's Diary"],
&["On arriving home at a ", "quarter-past eleven", ", we found a hansom cab, which had been waiting for me for two hours with a letter. Sarah said she did not know what to do, as we had not left the address where we had gone.", "George and Weedon Grossmith", "Dairy of a Nobody"],
]);
    minutes.insert("23:16", &[
&["But I couldn't get out of the house straight away because he would see me, so I would have to wait until he was asleep. The time was ", "11.16 pm.", " I tried doubling 2s again, but I couldn't get past 2(15) which was 32,768. So I groaned to make the time pass quicker and not think.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("23:17", &[
&["But I couldn't get out of the house straight away because he would see me, so I would have to wait until he was asleep. The time was ", "11.16 pm.", " I tried doubling 2s again, but I couldn't get past 2(15) which was 32,768. So I groaned to make the time pass quicker and not think.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert(
        "23:18",
        &[&[
            "It is ",
            "11.18",
            ". A row of bungalows in a round with a clump of larch tree in the middle.",
            "Jackie Kay",
            "Trumpet",
        ]],
    );
    minutes.insert("23:19", &[
&["A whistle cut sharply across his words. Peter got onto his knees to look out the window, and Miss Fuller glared at him. Polly looked down at her watch: ", "11:19", ". The train. But the stationmaster had said it was always late.", "Connie Willis", "Blackout"],
]);
    minutes.insert("23:20", &[
&["From Balboa Island, he drove south to Laguna Beach. At ", "eleven-twenty", ", he parked his van across the street from the Hudston house.", "Dean Koontz", "Watchers"],
&["Harvey looked at the clock, which marked ", "twenty past eleven", ". \"Then I'll sleep here till three and catch the four o'clock freight. They let us men from the Fleet ride free as a rule.\"", "Rudyard Kipling", "Captains Courageous"],
]);
    minutes.insert("23:21", &[
&["From Balboa Island, he drove south to Laguna Beach. At ", "eleven-twenty", ", he parked his van across the street from the Hudston house.", "Dean Koontz", "Watchers"],
&["Harvey looked at the clock, which marked ", "twenty past eleven", ". \"Then I'll sleep here till three and catch the four o'clock freight. They let us men from the Fleet ride free as a rule.\"", "Rudyard Kipling", "Captains Courageous"],
]);
    minutes.insert("23:22", &[
&["At ", "11.22", " he handed his ticket to a yawning guard and walked down a long flight of wooden steps to the car-park. A breeze lifted and dropped the leaves of a tree, and he thought of the girl with the blonde hair. His bicycle lay where he had left it.", "Robert Thomson", "Dreams of leaving"],
]);
    minutes.insert("23:23", &[
&["At ", "11.22", " he handed his ticket to a yawning guard and walked down a long flight of wooden steps to the car-park. A breeze lifted and dropped the leaves of a tree, and he thought of the girl with the blonde hair. His bicycle lay where he had left it.", "Robert Thomson", "Dreams of leaving"],
]);
    minutes.insert("23:24", &[
&["At ", "11.22", " he handed his ticket to a yawning guard and walked down a long flight of wooden steps to the car-park. A breeze lifted and dropped the leaves of a tree, and he thought of the girl with the blonde hair. His bicycle lay where he had left it.", "Robert Thomson", "Dreams of leaving"],
]);
    minutes.insert("23:25", &[
&["\"OK, Estelle, I willl be at Nice Airport at ", "11.25 p.m.", " on Saturday, BA: Could you send the driver?\"", "Justin Cartwright", "Other People's Money"],
&["To test the intensity of the light whose nature and cause he could not determine, he took out his watch to see if he could make out the figures on the dial. They were plainly visible, and the hands indicated the hour of ", "eleven o'clock and twenty-five minutes", ". At that moment the mysterious illumination suddenly flared to an intense, an almost blinding splendor…", "Ambrose Bierce", "A Wireless Message"],
]);
    minutes.insert("23:26", &[
&["Los Angeles. ", "11:26 p.m.", " In a dark red room- the color of the walls is close to that of raw liver- is a tall woman dressed cartoonishly in too-tight silk shorts, her breasts pulled up and pressed forward by the yellow blouse tied beneath them.", "Neil Gaiman", "American Gods"],
]);
    minutes.insert("23:27", &[
&["He tells his old friend the train times and they settle on the 19.45 arriving at ", "23.27", ". \"I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("23:28", &[
&["He tells his old friend the train times and they settle on the 19.45 arriving at ", "23.27", ". \"I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("23:29", &[
&["He tells his old friend the train times and they settle on the 19.45 arriving at ", "23.27", ". \"I'll book us into the ultra-luxurious Francis Drake Lodge. Running water in several rooms. Have you got a mobile?\"", "Justin Cartwright", "Other People's Money"],
]);
    minutes.insert("23:30", &[
&["He loaded the player and turned on the viewer, his knees popping again as he squatted to set the cue to ", "2330", ".", "David Foster Wallace", "Infinite Jest"],
&["He would catch the night bus for Casablanca, the one that left the beach at ", "half past eleven", ".", "Paul Bowles", "'The Dismissal', in Midnight Mass & Other Stories"],
&["The Picton boat was due to leave at ", "half-past eleven", ". It was a beautiful night, mild, starry, only when they got out of the cab and started to walk down the Old Wharf that jutted out into the harbour, a faint wind blowing off the water ruffled under Fenella's hat, and she put up her hand to keep it on.", "Katherine Mansfield", "“The Voyage”"],
&["The ship's clock in the bar says ", "half past eleven", ". Half past eleven is opening time. The hands of the clock have stayed still at half past eleven for fifty years.", "Dylan Thomas", "Under Milk Wood"],
]);
    minutes.insert("23:31", &[
&["It is ", "twenty-nine minutes to midnight", ". Dr Narlikar's Nursing Home is running on a skeleton staff; there are many absentees, many employees who have preferred to celebrate the imminent birth of the nation, and will not assist tonight at the births of children.", "Salman Rushdie", "Midnight's Children"],
]);
    minutes.insert("23:32", &[
&["\"This is the evening. This is the night. It is New Year´s Eve. ", "In about twenty-eight minutes it will be midnight.", " I still have twenty-eight minutes left. I have to recollect my thoughts. At twelve o´clock, I should be done thinking.\" He looked at his father. \"Help those that are depressed and consider themselves lost in this world,\" he thought. \"Old fart.\"", "Gerard Reve", "The Evenings"],
&["And then it started to rain and I got wet and I started shivering because I was cold. And then it was ", "11.32 pm", " and I heard voices of people walking along the street. And a voice said, 'I don't care whether you thought it was funny or not,' and it was a lady's voice.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("23:33", &[
&["We are on Colaba Causeway now, just for a moment, to reveal that at ", "twenty-seven minutes to midnight", ", the police are hunting for a dangerous criminal. His name: Joseph D'Costa. The orderly is absent, has been absent for several days, from his work at the Nursing Home, from his room near the slaughterhouse, and from the life of a distraught virginal Mary", "Salman Rushdie", "Midnight's Children"],
]);
    minutes.insert("23:34", &[
&["", "Eleven thirty-four", ". We stand on the sidewalk in front of Jean's apartment on the Upper East Side. Her doorman eyes us warily and fills me with a nameless dread, his gaze piercing me from the lobby. A curtain of stars, miles of them, are scattered, glowing, across the sky and their multitude humbles me, which I have a hard time tolerating. She shrugs and nods after I say something about forms of anxiety. It's as if her mind is having a hard time communicating with her mouth, as if she is searching for a rational analysis of who I am, which is, of course, an impossibility: there ... is ... no ... key.", "Bret Easton Ellis", "American Psycho"],
&["Reacher retrieved his G36 from under the saloon bar window at ", "eleven thirty-four", " precisely and set out to walk back on the road, which he figured would make the return trip faster.", "Lee Child", "The Hard Way"],
]);
    minutes.insert("23:35", &[
&["Then at ", "eleven thirty-five", " the door at the rear of the hall opened and a police sergeant and three constables entered, ushered by Bagot.", "Michael Innes", "Hamlet, Revenge!"],
]);
    minutes.insert("23:36", &[
&["Then Green knocks at the front door at ", "2336", " - Gately has to Log the exact time and then it's his call whether to unlock the door.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("23:37", &[
&["Then Green knocks at the front door at ", "2336", " - Gately has to Log the exact time and then it's his call whether to unlock the door.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("23:38", &[
&["Then Green knocks at the front door at ", "2336", " - Gately has to Log the exact time and then it's his call whether to unlock the door.", "David Foster Wallace", "Infinite Jest"],
]);
    minutes.insert("23:39", &[
&["There's a whisper down the line at ", "11.39", " When the Night Mail's ready to depart, Saying \"Skimble where is Skimble has he gone to hunt the thimble? We must find him or the train can't start.\"", "T S Eliot", "Old Possum's Book of Practical Cats"],
]);
    minutes.insert("23:40", &[
&["We all have the maps and appliances of various kinds that can be had. Professor Van Helsing and I are to leave by the ", "11:40", " train tonight for Veresti, where we are to get a carriage to drive to the Borgo Pass. We are bringing a good deal of ready money, as we are to buy a carriage and horses.", "Bram Stoker", "Dracula"],
]);
    minutes.insert("23:41", &[
&["In a little while his mind cleared, but his head ached, arms ached, body ached. The phosphorescent figures on his watch attracted his attention. He peered at them. The time was ", "11:41", ". I remember...what do I remember?", "James Clavell", "Noble House"],
]);
    minutes.insert("23:42", &[
&["At ", "11.42", " then the signal's nearly due And the passengers are frantic to a man- Then Skimble will appear and he'll saunter to the rear:", "TS Eliot", "Old Possum's Book of Practical Cats"],
]);
    minutes.insert("23:43", &[
&["The clock told him it was ", "eleven forty-three", " and in that moment, in a flash of illumination, Mungo understood what the numbers at the end of Moscow Centre's messages were", "Ruth Rendell", "Talking to Strange Men"],
]);
    minutes.insert("23:44", &[
&["'At ", "eleven forty-four", " last night somebody stabbed this girl in the neck with a kitchen knife and immediately thereafter plunged the same knife through her skull, where it remained.’", "Ben Elton", "Dead Famous"],
]);
    minutes.insert("23:45", &[
&["The church clocks chimed ", "three quarters past eleven", ", as two figures emerged on London Bridge. One, which advanced with a swift and rapid step, was that of a woman who looked eagerly about her as though in quest of some expected object; the other figure was that of a man...", "Charles Dickens", "Oliver Twist"],
&["We struck the tow-path at length, and that made us happy because prior to this we had not been sure whether we were walking towards the river or away from it, and when you are tired and want to go to bed, uncertainties like that worry you. We passed Shiplake as the clock was striking the ", "quarter to twelve", " and then George said thoughtfully: 'You don't happen to remember which of the islands it was, do you?'", "Jerome K. Jerome", "Three Men in a Boat"],
]);
    minutes.insert("23:46", &[
&["In the Kismet Lounge, Mr. Early sees suddenly to his horror it's ", "11:46 p.m.", " He's been in this place far longer than he'd planned, and he's had more to drink than he'd planned. Shame! What if, back at the E-Z, his little girl is crying piteously for him?", "Joyce Carol Oates", "Doll: A Romance of the Mississippi"],
]);
    minutes.insert("23:47", &[
&["If he had glanced at his watch, he would have seen that it was ", "thirteen minutes to midnight", ". And if he had been interested in what was going on, he would have heard the voices and bawling of terrified men.", "Rudolf Jašík", "Dead Soldiers Don't Sing"],
]);
    minutes.insert("23:48", &[
&["Littell arranged a private charter.He told the pilot to fly balls-to-the-wall.The little two-seater rattled and shook-Kemper couldn't believe it. It was ", "11.48pm.", " They were thirty-six hours short of GO.", "James Ellroy", "American Tabloid"],
]);
    minutes.insert(
        "23:49",
        &[&[
            "Tom shrugged. He pushed his pinkish ruffled sleeve back, and saw that it was ",
            "eleven minutes to midnight",
            ". Tom finished his coffee.",
            "The Patricia Highsmith",
            "The Boy Who Followed Ripley",
        ]],
    );
    minutes.insert("23:50", &[
&["At ", "11.50pm", ", I got up extremely quietly, took my things from under the bed, and opened the door one millimeter at a time, so it wouldn't make any noise.", "Jonathan Safran Foer", "Extremely Loud and Incredibly Close"],
]);
    minutes.insert("23:51", &[
&["\"Due at Waterloo at ", "eleven-fifty-one", ",\" panted Smith.\"That gives us thirty-nine minutes to get to the other side of the river and reach his hotel.\"", "Sax Rohmer", "The Mystery of Dr Fu Manchu"],
]);
    minutes.insert("23:52", &[
&["It was ", "eight minutes to midnight", ". Just nice time, I said to myself. Indoors, everything was quiet and in darkness. Splendid. I went to the bar and fetched a tumbler, a siphon of soda and a bottle of Glen Grant, took a weak drink and a pill, and settled down in the public dining-room to wait the remaining two minutes.", "Kingsley Amis", "The Green Man"],
]);
    minutes.insert("23:53", &[
&["It was ", "7 minutes to midnight", ". The dog was lying on the grass in the middle of the lawn in front of Mrs. Shears' house.", "Mark Haddon", "The Curious Incident of the Dog in the Night-time"],
]);
    minutes.insert("23:54", &[
&["His watch read ", "11.54pm", " Eastern Standard Time. Already it was nearly 6.00am in Rome. He had left a city frozen by a harsh January storm, after a bleak, wet Christmas season.", "Greg Tobin", "Conclave"],
]);
    minutes.insert("23:55", &[
&["\"I am going to lock you in. It is-\" he consulted his watch, \"", "five minutes to midnight", ". Miss Granger, three turns should do it. Good luck.\"", "J. K. Rowling", "Harry Potter and the Prisoner of Azkaban"],
&["I looked at my watch. It wanted ", "five minutes to twelve", ", when the premonitory symptoms of the working of the laudanum first showed themselves to me. At this time, no unpractised eyes would have detected any change in him. But, as the minutes of the new morning wore away, the swiftly-subtle progress of the influence began to show itself more plainly. The sublime intoxication of opium gleamed in his eyes; the dew of a stealthy perspiration began to glisten on his face. In five minutes more, the talk which he still kept up with me, failed in coherence.", "Wilkie Collins", "The Moonstone"],
]);
    minutes.insert("23:56", &[
&["The human race is at the end of the line, the doomsday clock ticks on. It's stopped for a decade at ", "four minutes to midnight", ", but there the hands still stand. Any minute now they'll begin to move again.", "Fay Weldon", "Wicked Women"],
]);
    minutes.insert("23:57", &[
&["Wells looked out at the street. \"What time is it?\" he said. Chigurh raised his wrist and looked at his watch. \"", "Eleven fifty-seven", "\" he said. Wells nodded. By the old woman's calendar I've got three more minutes.", "Cormac McCarthy", "No Country for Old Men"],
]);
    minutes.insert("23:58", &[
&["Humans emerge ", "one minute and seventeen seconds before midnight", ". The whole of our recorded history, on this scale, would be no more than a few seconds, a single human lifetime barely an instant.", "Bill Bryson", "A Short History of Nearly Everything"],
]);
    minutes.insert("23:59", &[
&["At ", "a minute to midnight", ", Roquenton was holding his wife's hand and giving her some last words of advice. On the stroke of midnight, she felt her companion's hand melt away inside her own.", "Marcel Aymé", "The Man Who Walked Through Walls"],
&["Chigurgh rose and picked up the empty casing off the rug and blew into it and put it in his pocket and looked at his watch. The ", "new day was still a minute away", ".", "Cormac McCarthy", "No Country for Old Men"],
]);

    let minute = minutes.get(time.as_str()).unwrap();
    let mut rng = thread_rng();
    let random = rng.choose(&minute).unwrap();

    Minute {
        start: random[0],
        time: random[1],
        end: random[2],
        author: random[3],
        title: random[4],
    }
}
