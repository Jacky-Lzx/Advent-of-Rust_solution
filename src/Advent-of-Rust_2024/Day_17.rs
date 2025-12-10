// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

pub trait NewTrait {
    fn anonymize_email(&self) -> String;
}

// Your Solution here ...
impl NewTrait for String {
    fn anonymize_email(&self) -> String {
        if self.contains('@') {
            let mut it = self.split('@');
            if let (Some(local), Some(domain)) = (it.next(), it.next()) {
                let local: String = local
                    .chars()
                    .enumerate()
                    .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                    .collect();
                format!("{}@{}", local, domain)
            } else {
                self.chars()
                    .enumerate()
                    .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                    .collect()
            }
        } else {
            self.chars()
                .enumerate()
                .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                .collect()
        }
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}
