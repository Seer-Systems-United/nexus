//! # Term aliases
//!
//! Maps common variations to canonical term forms.
//! Handles acronyms, plurals, and synonyms.

use super::term;
use crate::topics::nlp::models::Term;

pub fn term_for_intent(intent: &str) -> Option<Term> {
    match intent {
        "approval" => Some(term("approval", "approval")),
        "attention-focus" => Some(term("attention-focus", "focus")),
        "awareness" => Some(term("awareness", "awareness")),
        "favorability" => Some(term("favorability", "favorability")),
        "future-effect" => Some(term("future-effect", "future effect")),
        "impact" => Some(term("impact", "impact")),
        "direction" => Some(term("direction", "direction")),
        "local-condition" => Some(term("local-condition", "local condition")),
        "support-oppose" => Some(term("support-oppose", "support")),
        "safety-effect" => Some(term("safety-effect", "safety effect")),
        "success-failure" => Some(term("success-failure", "success failure")),
        "worth-it" => Some(term("worth-it", "worth it")),
        "vote-choice" => Some(term("vote-choice", "vote choice")),
        "party-advantage" => Some(term("party-advantage", "party advantage")),
        _ => None,
    }
}

pub fn aliased_term(token: &str) -> Option<Term> {
    let term = match token {
        "ai" => term("ai", "AI"),
        "covid19" | "covid" | "coronavirus" => term("covid", "COVID"),
        "sport" | "sports" => term("sports", "sports"),
        "price" | "prices" | "pricing" | "cost" | "costs" => term("price", "price"),
        "subscription" | "subscriptions" | "subscribers" => term("subscription", "subscription"),
        "economic" | "economics" => term("economy", "economy"),
        "policy" | "policies" => term("policy", "policy"),
        "ukrainian" => term("ukraine", "Ukraine"),
        "israeli" => term("israel", "Israel"),
        "republicans" => term("republican", "Republican"),
        "democrats" => term("democrat", "Democrat"),
        "palestinian" | "palestinians" => term("palestine", "Palestine"),
        "immigration" | "immigrant" | "immigrants" | "migrant" | "migrants" | "border" => {
            term("immigration", "immigration")
        }
        "wager" | "wagers" | "wagering" | "gamble" | "gambles" | "gambling" | "bet" | "bets"
        | "betting" => term("betting", "betting"),
        "regulate" | "regulates" | "regulated" | "regulating" | "regulation" | "regulations" => {
            term("regulation", "regulation")
        }
        "strike" | "strikes" | "airstrike" | "airstrikes" | "troop" | "troops" | "war" | "wars"
        | "military" | "conflict" | "conflicts" => term("military-conflict", "military conflict"),
        _ => return None,
    };

    Some(term)
}

pub fn display_label(token: &str) -> String {
    match token {
        "policies" => "policy".to_string(),
        "companies" => "company".to_string(),
        "countries" => "country".to_string(),
        other if other.len() > 4 && other.ends_with('s') && !other.ends_with("ss") => {
            other[..other.len() - 1].to_string()
        }
        other => other.to_string(),
    }
}
