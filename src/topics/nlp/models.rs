use std::collections::{HashMap, HashSet};

pub const MIN_TERMS: usize = 2;
pub const MAX_LABEL_TERMS: usize = 5;

#[derive(Debug, Clone)]
pub struct Term {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct CandidateTerms {
    pub terms: Vec<Term>,
    pub features: HashSet<String>,
    pub signature: String,
    pub intent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub index: usize,
    pub terms: CandidateTerms,
}

#[derive(Debug, Clone)]
pub struct ClusterTerm {
    pub label: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub candidates: Vec<Candidate>,
    pub term_counts: HashMap<String, ClusterTerm>,
}

impl Cluster {
    pub fn new(candidate: Candidate) -> Self {
        let mut cluster = Self {
            candidates: Vec::new(),
            term_counts: HashMap::new(),
        };
        cluster.add(candidate);
        cluster
    }

    pub fn add(&mut self, candidate: Candidate) {
        for term in &candidate.terms.terms {
            self.term_counts
                .entry(term.key.clone())
                .and_modify(|existing| existing.count += 1)
                .or_insert(ClusterTerm {
                    label: term.label.clone(),
                    count: 1,
                });
        }
        self.candidates.push(candidate);
    }
}
