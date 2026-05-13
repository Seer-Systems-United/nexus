use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::poll::question::Question;

use super::page::parse_page;

pub fn parse_pages(pages: &[String]) -> Vec<Question> {
    let mut parsed_questions: Vec<(usize, Question)> = pages
        .par_iter()
        .enumerate()
        .filter_map(|(index, page)| parse_page(page).map(|question| (index, question)))
        .collect();
    parsed_questions.sort_by_key(|(index, _)| *index);

    let mut questions: Vec<Question> = Vec::with_capacity(parsed_questions.len());

    for (_, question) in parsed_questions {
        if let Some(existing) = questions
            .iter_mut()
            .find(|existing| existing.text == question.text)
        {
            existing.responses.extend(question.responses);
        } else {
            questions.push(question);
        }
    }

    questions
}
